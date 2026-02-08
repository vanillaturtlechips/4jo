use tauri::Emitter;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::sync::mpsc::channel;
use std::time::Duration;
use rusqlite::Connection;
use std::path::PathBuf;

#[derive(Clone, serde::Serialize)]
struct ShortsData {
    url: String,
    title: String,
}

fn get_chrome_history_path() -> PathBuf {
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        PathBuf::from(format!("{}/.config/google-chrome/Default/History", home))
    }
    
    #[cfg(target_os = "windows")]
    {
        let username = std::env::var("USERNAME").expect("USERNAME not set");
        PathBuf::from(format!(
            r"C:\Users\{}\AppData\Local\Google\Chrome\User Data\Default\History",
            username
        ))
    }
    
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").expect("HOME not set");
        PathBuf::from(format!(
            "{}/Library/Application Support/Google/Chrome/Default/History",
            home
        ))
    }
}

fn get_latest_shorts_url(last_visit_time: &mut i64) -> Option<String> {
    let history_path = get_chrome_history_path();
    let temp_path = std::env::temp_dir().join("chrome_history_temp");
    
    std::fs::copy(&history_path, &temp_path).ok()?;
    
    let conn = Connection::open(temp_path).ok()?;
    
    let mut stmt = conn.prepare(
        "SELECT url, last_visit_time FROM urls 
         WHERE url LIKE '%youtube.com/shorts/%'
         AND last_visit_time > ?
         ORDER BY last_visit_time DESC LIMIT 1"
    ).ok()?;
    
    let current_time = *last_visit_time;  // 복사
    let result = stmt.query_row([current_time], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    }).ok();
    
    if let Some((url, visit_time)) = result {
        *last_visit_time = visit_time;
        Some(url)
    } else {
        None
    }
}

fn get_youtube_title(url: &str) -> String {
    let oembed_url = format!("https://www.youtube.com/oembed?url={}&format=json", url);
    
    if let Ok(resp) = reqwest::blocking::get(&oembed_url) {
        if let Ok(json) = resp.json::<serde_json::Value>() {
            if let Some(title) = json["title"].as_str() {
                return title.to_string();
            }
        }
    }
    
    "제목 가져오기 실패".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();
            
            std::thread::spawn(move || {
                let (tx, rx) = channel();
                
                let mut debouncer = new_debouncer(Duration::from_secs(2), tx)
                    .expect("Failed to create debouncer");
                
                let chrome_dir = get_chrome_history_path().parent().unwrap().to_path_buf();
                
                debouncer.watcher().watch(&chrome_dir, RecursiveMode::NonRecursive)
                    .expect("Failed to watch");
                
                println!("👀 Watching Chrome History...");
                
                // 현재 시간으로 초기화 (과거 기록 무시)
                let mut last_visit_time: i64 = chrono::Utc::now().timestamp_micros();
                
                loop {
                    match rx.recv() {
                        Ok(Ok(_events)) => {
                            if let Some(url) = get_latest_shorts_url(&mut last_visit_time) {
                                println!("🎬 New: {}", url);
                                
                                let title = get_youtube_title(&url);
                                println!("📝 Title: {}", title);
                                
                                handle.emit("sidecar-data", ShortsData {
                                    url: url.clone(),
                                    title: title.clone(),
                                }).ok();
                            }
                        }
                        Err(e) => println!("Watch error: {:?}", e),
                        _ => {}
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}