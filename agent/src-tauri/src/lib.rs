use tauri::Emitter;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::sync::mpsc::channel;
use std::time::Duration;
use rusqlite::Connection;
use std::path::PathBuf;
use regex::Regex;
use dotenv::dotenv;

#[derive(Clone, serde::Serialize)]
struct ShortsData {
    url: String,
    title: String,
}

// YouTube 전체 데이터 구조
#[derive(serde::Serialize, Debug)]
struct YouTubeData {
    video_id: String,
    url: String,
    metadata: VideoMetadata,
    comments: Vec<Comment>,
    captions: Vec<Caption>,
}

#[derive(serde::Serialize, Debug)]
struct VideoMetadata {
    title: String,
    description: String,
    duration: String,
    view_count: String,
    like_count: String,
    published_at: String,
}

#[derive(serde::Serialize, Debug, serde::Deserialize)]
struct Comment {
    text: String,
    author: String,
    like_count: u64,
    published_at: String,
}

#[derive(serde::Serialize, Debug, serde::Deserialize)]
struct Caption {
    start: f64,
    duration: f64,
    text: String,
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
    
    let current_time = *last_visit_time;
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

// URL에서 Video ID 추출
fn extract_video_id(url: &str) -> Option<String> {
    let re = Regex::new(r"youtube\.com/shorts/([a-zA-Z0-9_-]+)").ok()?;
    re.captures(url)?.get(1).map(|m| m.as_str().to_string())
}

// YouTube Data API로 메타데이터 + 댓글 가져오기
fn fetch_youtube_data(video_id: &str, api_key: &str) -> Option<YouTubeData> {
    let client = reqwest::blocking::Client::new();
    
    println!("📡 YouTube API 호출 중...");
    
    // 1. 비디오 메타데이터
    let metadata_url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet,statistics,contentDetails&id={}&key={}",
        video_id, api_key
    );
    
    let metadata_resp = client.get(&metadata_url).send().ok()?;
    
    if !metadata_resp.status().is_success() {
        println!("❌ API 오류: {}", metadata_resp.status());
        return None;
    }
    
    let metadata_json: serde_json::Value = metadata_resp.json().ok()?;
    
    if metadata_json["items"].as_array()?.is_empty() {
        println!("⚠️ 영상 정보를 찾을 수 없습니다.");
        return None;
    }
    
    let item = metadata_json["items"].get(0)?;
    let snippet = &item["snippet"];
    let statistics = &item["statistics"];
    let content_details = &item["contentDetails"];
    
    let metadata = VideoMetadata {
        title: snippet["title"].as_str()?.to_string(),
        description: snippet["description"].as_str().unwrap_or("").to_string(),
        duration: content_details["duration"].as_str()?.to_string(),
        view_count: statistics["viewCount"].as_str().unwrap_or("0").to_string(),
        like_count: statistics["likeCount"].as_str().unwrap_or("0").to_string(),
        published_at: snippet["publishedAt"].as_str()?.to_string(),
    };
    
    println!("✅ 메타데이터 수집 완료");
    println!("   제목: {}", metadata.title);
    println!("   조회수: {} | 좋아요: {}", metadata.view_count, metadata.like_count);
    
    // 2. 댓글 상위 10개
    println!("📝 댓글 수집 중...");
    
    let comments_url = format!(
        "https://www.googleapis.com/youtube/v3/commentThreads?part=snippet&videoId={}&order=relevance&maxResults=10&key={}",
        video_id, api_key
    );
    
    let comments_resp = client.get(&comments_url).send().ok()?;
    let comments_json: serde_json::Value = comments_resp.json().ok()?;
    
    let mut comments = Vec::new();
    if let Some(items) = comments_json["items"].as_array() {
        for item in items {
            let comment_snippet = &item["snippet"]["topLevelComment"]["snippet"];
            if let (Some(text), Some(author), Some(like_count), Some(published_at)) = (
                comment_snippet["textDisplay"].as_str(),
                comment_snippet["authorDisplayName"].as_str(),
                comment_snippet["likeCount"].as_u64(),
                comment_snippet["publishedAt"].as_str(),
            ) {
                comments.push(Comment {
                    text: text.to_string(),
                    author: author.to_string(),
                    like_count,
                    published_at: published_at.to_string(),
                });
            }
        }
    }
    
    println!("✅ 댓글 {}개 수집 완료", comments.len());
    
    // 3. 자막 (Python 스크립트 호출)
    println!("🎬 자막 추출 중...");
    let captions = fetch_captions_via_python(video_id);
    
    if captions.is_empty() {
        println!("⚠️ 자막이 없거나 추출 실패");
    } else {
        println!("✅ 자막 {}개 추출 완료", captions.len());
    }
    
    Some(YouTubeData {
        video_id: video_id.to_string(),
        url: format!("https://youtube.com/shorts/{}", video_id),
        metadata,
        comments,
        captions,
    })
}

// Python 스크립트로 자막 추출 (더 정확함)
fn fetch_captions_via_python(video_id: &str) -> Vec<Caption> {
    use std::process::Command;
    
    // Python 스크립트 경로 (프로젝트 루트 또는 PATH)
    let script_path = "extract_captions.py";
    
    let output = Command::new("python3")
        .arg(script_path)
        .arg(video_id)
        .output();
    
    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                
                // JSON 파싱
                match serde_json::from_str::<Vec<Caption>>(&stdout) {
                    Ok(captions) => captions,
                    Err(e) => {
                        println!("⚠️ 자막 JSON 파싱 실패: {}", e);
                        Vec::new()
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&result.stderr);
                println!("⚠️ Python 스크립트 오류: {}", stderr);
                Vec::new()
            }
        }
        Err(e) => {
            println!("❌ Python 실행 실패: {}", e);
            println!("   extract_captions.py가 프로젝트 루트에 있는지 확인하세요.");
            Vec::new()
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();

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
                
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("🛡️  Silver Guardian - YouTube 데이터 수집기");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("👀 Chrome History 감시 시작...");
                println!("📌 YouTube API Key: .env 파일 확인");
                println!("📌 Python 스크립트: extract_captions.py\n");
                
                // 환경 변수에서 API Key 읽기
                let api_key = std::env::var("YOUTUBE_API_KEY")
                    .unwrap_or_else(|_| {
                        println!("⚠️ YOUTUBE_API_KEY가 설정되지 않았습니다.");
                        println!("   .env 파일을 생성하고 API 키를 입력하세요.\n");
                        "YOUR_API_KEY_HERE".to_string()
                    });
                
                let mut last_visit_time: i64 = chrono::Utc::now().timestamp_micros();
                
                loop {
                    match rx.recv() {
                        Ok(Ok(_events)) => {
                            if let Some(url) = get_latest_shorts_url(&mut last_visit_time) {
                                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                println!("🎬 새 영상 감지!");
                                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                println!("🔗 URL: {}\n", url);
                                
                                // Video ID 추출
                                if let Some(video_id) = extract_video_id(&url) {
                                    println!("🆔 Video ID: {}\n", video_id);
                                    
                                    // YouTube Data 수집
                                    if let Some(youtube_data) = fetch_youtube_data(&video_id, &api_key) {
                                        // JSON으로 직렬화
                                        let json = serde_json::to_string_pretty(&youtube_data)
                                            .unwrap_or_else(|_| "JSON 변환 실패".to_string());
                                        
                                        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                        println!("📦 구조화된 JSON 데이터:");
                                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                                        println!("{}", json);
                                        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
                                        
                                        // 요약 정보
                                        println!("📊 수집 요약:");
                                        println!("   - 제목: {}", youtube_data.metadata.title);
                                        println!("   - 설명글: {} 글자", youtube_data.metadata.description.len());
                                        println!("   - 댓글: {}개", youtube_data.comments.len());
                                        println!("   - 자막: {}개\n", youtube_data.captions.len());
                                        
                                        // React UI로도 전송 (기존 방식)
                                        handle.emit("sidecar-data", ShortsData {
                                            url: url.clone(),
                                            title: youtube_data.metadata.title.clone(),
                                        }).ok();
                                    } else {
                                        println!("❌ YouTube 데이터 수집 실패");
                                        println!("   - API Key가 올바른지 확인하세요.");
                                        println!("   - 할당량을 초과하지 않았는지 확인하세요.\n");
                                    }
                                } else {
                                    println!("⚠️ Video ID 추출 실패\n");
                                }
                            }
                        }
                        Err(e) => println!("❌ 파일 감시 오류: {:?}", e),
                        _ => {}
                    }
                }
            });
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}