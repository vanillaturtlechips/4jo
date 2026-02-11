#!/usr/bin/env python3
"""
YouTube 자막 추출 헬퍼 스크립트
Rust에서 호출하거나 독립 실행 가능
"""

import sys
import json
import yt_dlp

def extract_captions(video_id):
    """
    YouTube 영상의 자막을 추출하여 JSON으로 반환
    
    Args:
        video_id: YouTube 영상 ID (예: 39KRfXSrKxo)
    
    Returns:
        list: [{"start": 0.0, "duration": 2.5, "text": "..."}, ...]
    """
    url = f"https://www.youtube.com/shorts/{video_id}"
    
    ydl_opts = {
        'skip_download': True,
        'writesubtitles': True,
        'writeautomaticsub': True,
        'subtitleslangs': ['ko', 'en'],  # 한국어 우선, 없으면 영어
        'quiet': True,
        'no_warnings': True,
    }
    
    try:
        with yt_dlp.YoutubeDL(ydl_opts) as ydl:
            info = ydl.extract_info(url, download=False)
            
            # 자막 데이터 추출
            captions = []
            
            # 자동 생성 자막 (automatic_captions)
            if 'automatic_captions' in info:
                for lang, subtitle_list in info['automatic_captions'].items():
                    if lang in ['ko', 'en']:
                        # JSON 형식 자막 찾기
                        for subtitle in subtitle_list:
                            if subtitle.get('ext') == 'json3':
                                # 자막 데이터 다운로드
                                subtitle_data = ydl.urlopen(subtitle['url']).read()
                                subtitle_json = json.loads(subtitle_data)
                                
                                # 이벤트 파싱
                                for event in subtitle_json.get('events', []):
                                    if 'segs' in event:
                                        text = ''.join(seg.get('utf8', '') for seg in event['segs'])
                                        if text.strip():
                                            captions.append({
                                                'start': event.get('tStartMs', 0) / 1000.0,
                                                'duration': event.get('dDurationMs', 0) / 1000.0,
                                                'text': text.strip()
                                            })
                                break
                        break
            
            # 수동 자막이 있다면 우선 사용
            if 'subtitles' in info and not captions:
                for lang, subtitle_list in info['subtitles'].items():
                    if lang in ['ko', 'en']:
                        for subtitle in subtitle_list:
                            if subtitle.get('ext') == 'json3':
                                subtitle_data = ydl.urlopen(subtitle['url']).read()
                                subtitle_json = json.loads(subtitle_data)
                                
                                for event in subtitle_json.get('events', []):
                                    if 'segs' in event:
                                        text = ''.join(seg.get('utf8', '') for seg in event['segs'])
                                        if text.strip():
                                            captions.append({
                                                'start': event.get('tStartMs', 0) / 1000.0,
                                                'duration': event.get('dDurationMs', 0) / 1000.0,
                                                'text': text.strip()
                                            })
                                break
                        break
            
            return captions
    
    except Exception as e:
        print(f"❌ 자막 추출 오류: {e}", file=sys.stderr)
        return []

def main():
    """CLI 인터페이스"""
    if len(sys.argv) != 2:
        print("사용법: python extract_captions.py VIDEO_ID")
        print("예시: python extract_captions.py 39KRfXSrKxo")
        sys.exit(1)
    
    video_id = sys.argv[1]
    captions = extract_captions(video_id)
    
    # JSON으로 출력 (Rust가 파싱 가능)
    print(json.dumps(captions, ensure_ascii=False, indent=2))

if __name__ == '__main__':
    main()