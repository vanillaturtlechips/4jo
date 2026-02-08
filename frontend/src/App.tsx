import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

// 1. Rust 에이전트로부터 수신할 데이터 규격 (Rust의 FinalPayload와 일치)
interface SidecarPayload {
  url: string;
  analysis: string;
}

// 2. UI 출력을 위한 로그 엔트리 규격
interface LogEntry {
  id: string;
  time: string;
  url: string;
  analysis: string;
  severity: "info" | "warning";
}

function App() {
  const [logs, setLogs] = useState<LogEntry[]>([]);

  useEffect(() => {
    // Rust 사이드카로부터 전달되는 이벤트를 구독합니다
    const setupListener = async () => {
      // "sidecar-data" 이벤트 이름은 Rust의 handle.emit 이름과 반드시 일치해야 합니다
      const unlisten = await listen<SidecarPayload>("sidecar-data", (event) => {
        const { url, analysis } = event.payload;
        
        console.log("🚀 분석 데이터 수신:", event.payload); // 시연 중 브라우저 콘솔(F12) 확인용

        const newEntry: LogEntry = {
          id: Math.random().toString(36).substring(2, 9),
          time: new Date().toLocaleTimeString([], { hour12: false, second: "2-digit" }),
          url: url,
          analysis: analysis,
          // AI 분석 결과에 특정 단어가 포함되면 시각적 경고 표시
          severity: (analysis.includes("위험") || analysis.includes("주의") || analysis.includes("불법")) ? "warning" : "info",
        };
        
        // 최신 로그를 상단에 배치하고 최대 10개까지 유지
        setLogs((prev) => [newEntry, ...prev].slice(0, 10));
      });

      return unlisten;
    };

    const listenerPromise = setupListener();

    // 컴포넌트 언마운트 시 리스너 해제
    return () => {
      listenerPromise.then((unlisten) => unlisten());
    };
  }, []);

  return (
    <div className="min-h-screen bg-slate-50 flex flex-col items-center p-6 text-slate-900 font-sans">
      {/* 🛡️ 상단 헤더 섹션 */}
      <header className="w-full max-w-2xl flex justify-between items-center mb-10">
        <div className="flex items-center gap-3">
          <div className="bg-blue-600 p-2 rounded-xl shadow-lg shadow-blue-200">
            <span className="text-white text-xl">🛡️</span>
          </div>
          <h1 className="text-2xl font-black tracking-tighter text-slate-800">
            SILVER <span className="text-blue-600">GUARDIAN</span>
          </h1>
        </div>
        <div className="flex items-center gap-2 bg-white px-4 py-2 rounded-full shadow-sm border border-slate-200">
          <span className="relative flex h-2 w-2">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
          </span>
          <span className="text-[10px] font-black text-slate-600 uppercase">AI 분석 엔진 가동 중</span>
        </div>
      </header>

      {/* ✨ 메인 상태 카드 */}
      <main className="w-full max-w-2xl space-y-6">
        <section className="bg-white p-10 rounded-[2.5rem] shadow-xl shadow-slate-200/60 border border-white text-center relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-1.5 bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500"></div>
          <div className="text-6xl mb-6">✨</div>
          <h2 className="text-3xl font-extrabold text-slate-800 mb-2">부모님 PC 보호 모드</h2>
          <p className="text-slate-400 font-medium leading-relaxed">
            Gemini 2.0 AI가 유튜브 쇼츠의 유해성을 실시간 판별하여<br/>안전한 디지털 환경을 유지하고 있습니다.
          </p>
        </section>

        {/* 📋 로그 리포트 카드 */}
        <section className="bg-white rounded-[2.5rem] shadow-lg border border-slate-100 overflow-hidden">
          <div className="px-8 py-6 border-b border-slate-50 flex justify-between items-center bg-slate-50/50">
            <h3 className="text-xs font-black text-slate-400 uppercase tracking-widest">실시간 분석 리포트</h3>
            <span className="bg-blue-50 text-blue-600 px-2.5 py-1 rounded-md text-[10px] font-extrabold tracking-tighter border border-blue-100">
              V0.1.2-BETA
            </span>
          </div>
          
          <div className="divide-y divide-slate-50">
            {logs.length > 0 ? (
              logs.map((log) => (
                <div key={log.id} className="px-8 py-6 flex items-start justify-between hover:bg-slate-50 transition-all group animate-in fade-in slide-in-from-top-4 duration-500">
                  <div className="flex flex-col gap-1.5 overflow-hidden">
                    <span className="text-[10px] font-bold text-blue-500 font-mono tracking-tight">{log.time}</span>
                    <span className={`text-[15px] font-bold leading-tight ${
                      log.severity === "warning" ? "text-amber-600" : "text-slate-800"
                    }`}>
                      {log.analysis}
                    </span>
                    <span className="text-xs text-slate-400 truncate max-w-[400px] font-medium">
                      {log.url}
                    </span>
                  </div>
                  <div className="flex items-center pt-5 pl-4">
                    <span className={`px-3 py-1.5 rounded-xl text-[10px] font-black border whitespace-nowrap transition-colors ${
                      log.severity === "warning" 
                        ? "bg-amber-50 text-amber-600 border-amber-100" 
                        : "bg-blue-50 text-blue-600 border-blue-100"
                    }`}>
                      {log.severity === "warning" ? "보호 필요" : "정상 통과"}
                    </span>
                  </div>
                </div>
              ))
            ) : (
              <div className="py-24 text-center">
                <div className="inline-block animate-bounce mb-4 text-3xl">🛡️</div>
                <p className="text-slate-300 font-semibold italic">유튜브 활동을 감시하고 있습니다...</p>
              </div>
            )}
          </div>
        </section>
      </main>

      <footer className="mt-auto py-10 text-slate-400 text-[10px] font-bold tracking-[0.2em] uppercase">
        © 2026 Silver Guardian Project | Cloud Engineering Bootcamp
      </footer>
    </div>
  );
}

export default App;