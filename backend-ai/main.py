import os
import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import google.generativeai as genai
import psycopg2
from datetime import datetime

app = FastAPI()

# 1. Gemini 2.0 설정 (2026년 최신 모델)
genai.configure(api_key=os.getenv("GEMINI_API_KEY"))
model = genai.GenerativeModel('gemini-2.0-flash')

class AnalyzeRequest(BaseModel):
    url: str

@app.post("/analyze")
async def analyze_v2(request: AnalyzeRequest):
    try:
        # [PROMPT 2.0] 특정 출력 형식을 강제하고 불필요한 설명을 제거함
        prompt = (
            f"당신은 보안 전문가입니다. 다음 유튜브 URL의 메타데이터를 확인하여 "
            f"반드시 '이 유튜브 쇼츠의 제목은 [실제제목]입니다'라는 형식으로만 한 문장으로 답변하세요. "
            f"부가적인 설명은 절대 하지 마세요. URL: {request.url}"
        )
        
        response = model.generate_content(prompt)
        analysis_text = response.text.strip()

        # [IDC LOGGING 2.0] 분석 버전과 시각을 포함하여 DB 저장
        try:
            conn = psycopg2.connect(os.getenv("DB_URL"))
            cur = conn.cursor()
            cur.execute(
                "INSERT INTO logs (url, result, model_version, created_at) VALUES (%s, %s, %s, %s)",
                (request.url, analysis_text, "Gemini-2.0-Flash", datetime.now())
            )
            conn.commit()
            cur.close()
            conn.close()
        except Exception as db_err:
            print(f"IDC DB 저장 에러 (시연 영향 없음): {db_err}")

        # 에이전트로 정제된 결과 반환
        return {
            "version": "2.0",
            "status": "analyzed",
            "message": analysis_text
        }

    except Exception as e:
        print(f"AI Server Error: {str(e)}")
        raise HTTPException(status_code=500, detail="AI 분석 서버 내부 오류")

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8080)