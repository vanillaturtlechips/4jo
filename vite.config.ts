import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  plugins: [
    react(),
    tailwindcss(), // 2. 여기에 추가합니다.
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: true, // 리눅스 환경에서 접속 안정성을 위해 추가
  },
});