use tauri::{Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
struct AnalysisResponse {
    version: String,
    status: String,
    result: String,
}

#[derive(Serialize, Clone)]
struct FinalPayload {
    url: String,
    analysis: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            
            // 사이드카 실행 (플랫폼 접미사는 Tauri가 자동으로 붙여줍니다)
            let sidecar = app.shell().sidecar("scanner").expect("사이드카 파일을 찾을 수 없습니다.");
            let (mut rx, _child) = sidecar.spawn().expect("사이드카 실행에 실패했습니다.");

            tauri::async_runtime::spawn(async move {
                println!("📡 [Rust Agent] 사이드카 모니터링 시작...");

                while let Some(event) = rx.recv().await {
                    match event {
                        // Stdout과 Stderr를 모두 처리하여 누락을 방지합니다.
                        CommandEvent::Stdout(line_bytes) | CommandEvent::Stderr(line_bytes) => {
                            let full_line = String::from_utf8_lossy(&line_bytes).trim().to_string();
                            
                            // 사이드카에서 오는 날것의 로그를 출력 (디버깅 핵심)
                            println!("📢 [Sidecar Raw]: {}", full_line);

                            // "https://"가 포함된 라인에서 URL만 추출합니다.
                            if let Some(url_index) = full_line.find("https://") {
                                let url = full_line[url_index..].trim().to_string();
                                println!("🚀 [Rust Agent] URL 감지 성공: {}", url);

                                let client = reqwest::Client::new();
                                // 게이트웨이를 통해 분석 요청
                                let res = client.post("http://localhost/api/analyze")
                                    .json(&serde_json::json!({ "url": url }))
                                    .send()
                                    .await;

                                match res {
                                    Ok(response) => {
                                        if let Ok(data) = response.json::<AnalysisResponse>().await {
                                            println!("✅ [Rust Agent] 분석 완료: {}", data.result);
                                            
                                            // React로 데이터 전송
                                            handle.emit("sidecar-data", FinalPayload {
                                                url: url.clone(),
                                                analysis: data.result,
                                            }).unwrap();
                                        }
                                    }
                                    Err(e) => {
                                        println!("❌ [Rust Agent] 서버 통신 에러: {}", e);
                                        handle.emit("sidecar-data", FinalPayload {
                                            url: url.clone(),
                                            analysis: format!("서버 연결 오류: {}", e),
                                        }).unwrap();
                                    }
                                }
                            }
                        }
                        CommandEvent::Terminated(payload) => {
                            println!("⚠️ [Rust Agent] 사이드카가 종료되었습니다: {:?}", payload.code);
                        }
                        _ => {}
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}