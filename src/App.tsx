import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

interface LogEntry {
  id: string;
  time: string;
  url: string;
  severity: "info" | "warning";
}

function App() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isRunning, setIsRunning] = useState(true);

  useEffect(() => {
    const unlisten = listen<string>("sidecar-data", (event) => {
      const newEntry: LogEntry = {
        id: Math.random().toString(36).substr(2, 9),
        time: new Date().toLocaleTimeString([], { hour12: false }),
        url: event.payload,
        severity: event.payload.includes("shorts") ? "warning" : "info",
      };
      setLogs((prev) => [newEntry, ...prev].slice(0, 5));
    });

    return () => { unlisten.then((f) => f()); };
  }, []);

  return (
    <div className="min-h-screen bg-slate-50 flex flex-col items-center p-6 text-slate-900">
      {/* 상단 헤더 섹션 */}
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
          <span className="text-xs font-bold text-slate-600">실시간 엔진 가동 중</span>
        </div>
      </header>

      {/* 메인 상태 카드 */}
      <main className="w-full max-w-2xl space-y-6">
        <section className="bg-white p-10 rounded-[2rem] shadow-xl shadow-slate-200/60 border border-white text-center relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-400 to-indigo-500"></div>
          <div className="text-6xl mb-6">✨</div>
          <h2 className="text-3xl font-extrabold text-slate-800 mb-2">부모님 PC가 안전합니다</h2>
          <p className="text-slate-400 font-medium">유튜브 쇼츠 탐지 엔진이 위협을 감시하고 있습니다.</p>
        </section>

        {/* 로그 리포트 카드 */}
        <section className="bg-white rounded-[2rem] shadow-lg border border-slate-100 overflow-hidden">
          <div className="px-8 py-5 border-b border-slate-50 flex justify-between items-center bg-slate-50/30">
            <h3 className="text-xs font-black text-slate-400 uppercase tracking-widest">실시간 분석 리포트</h3>
            <span className="bg-slate-100 text-slate-500 px-2 py-0.5 rounded text-[10px] font-bold">V0.1-ALPHA</span>
          </div>
          
          <div className="divide-y divide-slate-50">
            {logs.length > 0 ? (
              logs.map((log) => (
                <div key={log.id} className="px-8 py-5 flex items-center justify-between hover:bg-slate-50/80 transition-all group">
                  <div className="flex flex-col gap-1">
                    <span className="text-[10px] font-bold text-blue-500 font-mono">{log.time}</span>
                    <span className="text-sm font-semibold text-slate-700 truncate max-w-[300px]">
                      {log.url}
                    </span>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className={`px-3 py-1 rounded-lg text-[10px] font-black ${
                      log.severity === "warning" ? "bg-amber-100 text-amber-600" : "bg-blue-100 text-blue-600"
                    }`}>
                      {log.severity === "warning" ? "주의 대상" : "정상 분석"}
                    </span>
                  </div>
                </div>
              ))
            ) : (
              <div className="py-20 text-center">
                <div className="animate-bounce mb-4 text-2xl">⏳</div>
                <p className="text-slate-300 font-medium italic">유튜브 접속 신호를 기다리는 중...</p>
              </div>
            )}
          </div>
        </section>
      </main>

      <footer className="mt-auto py-8 text-slate-300 text-[10px] font-bold tracking-widest uppercase">
        © 2026 Silver Guardian Project | Cloud Engineering Bootcamp
      </footer>
    </div>
  );
}

export default App;