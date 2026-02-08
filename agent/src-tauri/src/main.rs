// 필요한 라이브러리 임포트
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init()) // 쉘 플러그인 초기화
        .setup(|app| {
            // 1. 사이드카(Go) 실행 파일 설정
            let sidecar_command = app.shell().sidecar("scanner")
                .expect("failed to create sidecar command");

            // 2. 프로세스 실행 및 이벤트 수신
            let (mut rx, _child) = sidecar_command
                .spawn()
                .expect("failed to spawn sidecar");

            // 3. 비동기 루프로 Go의 표준 출력(Stdout) 감시
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    match event {
                        CommandEvent::Stdout(line) => {
                            // Go에서 출력한 내용을 Rust 터미널에 출력
                            println!("🚀 [Go Sidecar]: {}", String::from_utf8_lossy(&line));
                        }
                        CommandEvent::Stderr(line) => {
                            eprintln!("⚠️ [Go Error]: {}", String::from_utf8_lossy(&line));
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