# Silver Guardian Agent(NoName yet...)

부모님 PC에서 유튜브 쇼츠 시청 기록을 실시간으로 감지하고 제목을 수집하는 Tauri 에이전트

## 지원 플랫폼

- ✅ Linux (Ubuntu/Debian)
- ✅ Windows 10/11
- ✅ macOS (Intel/Apple Silicon)

---

## 1. 사전 준비

### Linux (Ubuntu/Debian)

```bash
sudo apt update
sudo apt install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Node.js 설치 (pnpm 사용)
curl -fsSL https://fnm.vercel.app/install | bash
source ~/.bashrc
fnm install 20
fnm use 20

# pnpm 설치
npm install -g pnpm
```

### Windows

```powershell
# 1. Rust 설치
# https://rustup.rs/ 에서 다운로드 후 설치

# 2. Node.js 설치
# https://nodejs.org/ 에서 LTS 버전 다운로드

# 3. pnpm 설치
npm install -g pnpm

# 4. Visual Studio Build Tools 설치 (필수)
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# "C++ 빌드 도구" 체크하여 설치
```

### macOS

```bash
# Xcode Command Line Tools 설치
xcode-select --install

# Homebrew 설치 (없는 경우)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Rust 설치
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Node.js 설치
brew install node

# pnpm 설치
npm install -g pnpm
```

---

## 2. 프로젝트 클론 및 설정

```bash
# 저장소 클론
git clone <repo-url>
cd agent

# 의존성 설치
pnpm install

# Rust 의존성 확인
cd src-tauri
cargo check
cd ..
```

---

## 3. 개발 환경 실행

```bash
# 프로젝트 루트에서
pnpm run tauri dev
```

**동작 확인:**
1. 에이전트가 실행되면 터미널에 `👀 Watching Chrome History...` 메시지 표시
2. Chrome 브라우저에서 유튜브 쇼츠 접속
3. UI에 실시간으로 URL + 제목 표시

---

## 4. 프로덕션 빌드

### Linux

```bash
pnpm run tauri build
```

빌드 결과: `src-tauri/target/release/bundle/`

### Windows

```powershell
pnpm run tauri build
```

빌드 결과: `src-tauri\target\release\bundle\msi\`

### macOS

```bash
pnpm run tauri build
```

빌드 결과: `src-tauri/target/release/bundle/dmg/`

---

## 5. 프로젝트 구조

```
agent/
├── src/                    # React 프론트엔드
│   ├── App.tsx            # 메인 UI 컴포넌트
│   └── main.tsx
├── src-tauri/             # Rust 백엔드
│   ├── src/
│   │   ├── main.rs        # 엔트리포인트
│   │   └── lib.rs         # 파일 감시 + YouTube API
│   ├── Cargo.toml         # Rust 의존성
│   └── tauri.conf.json    # Tauri 설정
└── package.json
```

---

## 6. 주요 기능

### Chrome History 감시

- **Windows**: `C:\Users\{user}\AppData\Local\Google\Chrome\User Data\Default\History`
- **Linux**: `~/.config/google-chrome/Default/History`
- **macOS**: `~/Library/Application Support/Google/Chrome/Default/History`

### 실시간 감지

- `notify` crate 사용 (OS별 네이티브 API)
- Windows: ReadDirectoryChangesW
- Linux: inotify
- macOS: FSEvents

### YouTube 제목 가져오기

- YouTube oEmbed API 사용 (무료, 인증 불필요)
- 엔드포인트: `https://www.youtube.com/oembed?url={url}&format=json`

---

## 7. 트러블슈팅

### Chrome History 파일 접근 오류

**증상:** `Permission denied` 또는 `Database locked`

**해결:**
```bash
# Chrome 완전히 종료 후 재시도
pkill chrome  # Linux/macOS
taskkill /F /IM chrome.exe  # Windows
```

### Rust 컴파일 오류

**증상:** `cargo check` 실패

**해결:**
```bash
# Rust 업데이트
rustup update

# 의존성 재설치
cd src-tauri
rm -rf target
cargo clean
cargo check
```

### UI에 데이터 안 나타남

**확인 사항:**
1. 터미널에 `🎬 New:` 로그가 나타나는가?
2. 브라우저 개발자 도구(F12)에서 `🚀 분석 데이터 수신` 로그 확인
3. YouTube 쇼츠 URL이 맞는가? (`.../shorts/...`)

---

## 8. 개발 팁

### 로그 확인

```bash
# Rust 백엔드 로그
pnpm run tauri dev

# React 프론트엔드 로그
# 에이전트 실행 후 F12 → Console 탭
```

### 코드 수정 후

- **Rust 수정**: 자동 재컴파일 (1-2분 소요)
- **React 수정**: 핫 리로드 즉시 반영

---

## 라이선스

MIT License

## 문의

Silver Guardian Project(no name) | AWS Cloud School 2026