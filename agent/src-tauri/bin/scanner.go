package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	fmt.Println("Silver Guardian Go Sidecar Started!")
	fmt.Println("--------------------------------------------------")
	fmt.Println("시연 모드: 유튜브 쇼츠 URL을 복사해서 여기에 붙여넣고 Enter를 누르세요.")
	fmt.Println("예시: https://www.youtube.com/shorts/08jxS9WUOz0")
	fmt.Println("--------------------------------------------------")

	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print("URL 입력 대기 중 > ")
		if scanner.Scan() {
			input := strings.TrimSpace(scanner.Text())
			
			if input == "" {
				continue
			}

			// 유튜브 주소인지 확인 후 Rust 에이전트로 전송
			if strings.Contains(input, "youtube.com") || strings.Contains(input, "youtu.be") {
				// Rust가 인식할 수 있게 "DETECTED: " 접두사를 붙여 출력합니다.
				fmt.Printf("DETECTED: %s\n", input)
			} else {
				fmt.Println("⚠️ 올바른 유튜브 URL을 입력해주세요.")
			}
		}
	}
}