# backend-ai/Dockerfile
FROM python:3.11-slim

WORKDIR /app

# 필요한 패키지 설치 (FastAPI, Google Generative AI 등)
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "main.py"]