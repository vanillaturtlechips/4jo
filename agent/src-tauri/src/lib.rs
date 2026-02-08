use tauri::{Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};

// 도커 서버로부터 받을 응답 구조체 (AI 2.0 규격)
#[derive(Deserialize, Serialize, Clone)]
struct AnalysisResponse {
    version: String,
    status: String,
    message: String, // "이 유튜브 쇼츠의 제목은 ~입니다"
}

// 프론트엔드로 보낼 최종 이벤트 구조체
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

            // 1. Go 사이드카 실행
            let sidecar = app.shell().sidecar("scanner-x86_64-unknown-linux-gnu")
                .unwrap();
            let (mut rx, _child) = sidecar.spawn().unwrap();

            // 2. 백그라운드 스레드에서 데이터 처리
            tauri::async_runtime::spawn(async move {
                let mut reader = BufReader::new(rx);
                let mut line = String::new();

                while let Ok(n) = reader.read_line(&mut line) {
                    if n == 0 { break; }
                    let url = line.trim().to_string();

                    // 3. 로컬 도커 서버(AI 분석 서버)로 데이터 토스
                    let client = reqwest::Client::new();
                    let res = client.post("http://localhost:8080/analyze")
                        .json(&serde_json::json!({ "url": url }))
                        .send()
                        .await;

                    match res {
                        Ok(response) => {
                            if let Ok(data) = response.json::<AnalysisResponse>().await {
                                // 4. 성공 시 AI 분석 결과를 React로 전송
                                handle.emit("sidecar-data", FinalPayload {
                                    url: url.clone(),
                                    analysis: data.message, // 정제된 제목 문구
                                }).unwrap();
                            }
                        }
                        Err(e) => {
                            // 도커 서버가 꺼져있을 경우 에러 처리
                            handle.emit("sidecar-data", FinalPayload {
                                url: url.clone(),
                                analysis: format!("분석 서버 연결 실패: {}", e),
                            }).unwrap();
                        }
                    }

                    line.clear();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}