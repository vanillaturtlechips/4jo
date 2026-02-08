import os
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import google.generativeai as genai
import psycopg2
import yt_dlp

app = FastAPI()

# Gemini 설정
genai.configure(api_key=os.getenv("GEMINI_API_KEY"))
model = genai.GenerativeModel('gemini-2.0-flash')

class AnalyzeRequest(BaseModel):
    url: str

def get_real_video_title(url):
    ydl_opts = {'quiet': True, 'no_warnings': True, 'extract_flat': True}
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            return info.get('title', '알 수 없는 영상')
    except:
        return "제목 추출 실패"

@app.post("/analyze")
async def analyze(request: AnalyzeRequest):
    try:
        real_title = get_real_video_title(request.url)
        prompt = f"이 유튜브 영상의 실제 제목은 [{real_title}]입니다. 이 정보를 바탕으로 반드시 '이 유튜브 쇼츠의 제목은 [제목]입니다' 형식으로만 답변하세요."
        response = model.generate_content(prompt)
        analysis_text = response.text.strip()

        # DB 저장 (Optional)
        try:
            conn = psycopg2.connect(os.getenv("DB_URL"))
            cur = conn.cursor()
            cur.execute("INSERT INTO logs (url, result) VALUES (%s, %s)", (request.url, analysis_text))
            conn.commit()
            cur.close()
            conn.close()
        except: pass

        return {"version": "2.0", "status": "success", "result": analysis_text}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

# 서버를 무한 가동시키는 핵심 엔진 (들여쓰기 주의!)
if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)