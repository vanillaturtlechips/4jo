# backend-ai/main.py
from fastapi import FastAPI
import os
import psycopg2 # DB 연결용

app = FastAPI()

@app.post("/analyze")
async def analyze(data: dict):
    url = data.get("url")
    
    # 1. 여기서 제미니 API 호출 로직이 들어갑니다.
    # 2. 분석 결과를 DB에 저장합니다.
    conn = psycopg2.connect(os.getenv("DB_URL"))
    cur = conn.cursor()
    cur.execute("INSERT INTO logs (url, result) VALUES (%s, %s)", (url, "AI 분석 완료"))
    conn.commit()
    
    return {"status": "success", "url": url, "analysis": "위험(AI 합성 의심)"}