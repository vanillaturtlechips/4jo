-- db/init.sql (v2.0)
CREATE TABLE IF NOT EXISTS logs (
    id SERIAL PRIMARY KEY,
    url TEXT NOT NULL,
    result TEXT NOT NULL,
    model_version VARCHAR(50), -- 어떤 모델이 분석했는지 기록
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 인덱스 추가 (대규모 트래픽 대비)
CREATE INDEX IF NOT EXISTS idx_url ON logs(url);