# 4jo

# 4jo

1. linux

```
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
    
## No rust here that

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

2. project improt

```
# pnpm이 없다면: npm install -g pnpm
pnpm create tauri-app .
# 설정 선택:
# - Identifier: com.silver.guardian
# - Frontend: React
# - Language: TypeScript
```

3. set Go sideCar

```
mkdir -p src-tauri/bin

src-tauri/bin/scanner.go

package main

import "fmt"

func main() {
    fmt.Println("Go Sidecar is running on Linux!")
}

```

4. sideCar bianry build

```
# 내 리눅스 시스템의 아키텍처 확인 (예: x86_64-unknown-linux-gnu)
target_triple=$(rustc -Vv | grep host | cut -d ' ' -f 2)

# Go 빌드 (파일명 예: scanner-x86_64-unknown-linux-gnu)
go build -o "src-tauri/bin/scanner-$target_triple" src-tauri/bin/scanner.go
```

