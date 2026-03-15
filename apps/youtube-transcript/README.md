# YouTube Video Transcript Tool

A free YouTube video transcript tool with AI-powered summaries and translations.

## Features

- 📹 Get instant transcripts from YouTube videos
- 🎬 Support for manual and auto-generated captions
- 📝 Export transcripts in multiple formats (SRT, VTT, ASS, TXT, LRC)
- 🤖 AI-powered video summaries
- 🌐 Multi-language translation support
- 🌍 Bilingual UI (English/Chinese)
- 💰 100% free, no registration required
- 🦀 Pure Rust implementation - no yt-dlp dependency

## Project Structure

```
youtube-transcript/
├── web/              # Next.js frontend
├── api/              # Rust backend service
└── docker/           # Docker configurations
```

## Development

### 一键启动所有服务

```bash
# 从项目根目录
pnpm dev:yt

# 或直接运行脚本
cd apps/youtube-transcript
./start.sh
```

这将同时启动：

- Rust API 服务 → `http://localhost:8080`
- Next.js 前端 → `http://localhost:3000`

### 单独启动服务

#### 启动 Rust API

```bash
pnpm dev:api
# 或
cd apps/youtube-transcript/api && cargo run
```

#### 启动 Next.js 前端

```bash
pnpm dev
# 或
pnpm --filter @ai-tools/youtube-transcript-web dev
```

#### API 端点

| 端点          | 方法 | 描述                                          |
| ------------- | ---- | --------------------------------------------- |
| `/transcript` | POST | 获取视频字幕（JSON 格式）                     |
| `/format`     | GET  | 获取指定格式的字幕（srt, vtt, ass, txt, lrc） |
| `/info`       | GET  | 获取视频信息和可用字幕列表                    |
| `/formats`    | GET  | 获取支持的格式列表                            |
| `/health`     | GET  | 健康检查                                      |

### 2. 启动 Next.js 开发服务器

```bash
# Install dependencies
pnpm install

# Run development server
pnpm --filter @ai-tools/youtube-transcript-web dev

# Run tests
pnpm --filter @ai-tools/youtube-transcript-web test
pnpm --filter @ai-tools/youtube-transcript-web test:e2e

# Build
pnpm --filter @ai-tools/youtube-transcript-web build
```

## Environment Variables

Copy `.env.local.example` to `web/.env.local`:

```bash
GROQ_API_KEY=your_groq_api_key_here
```

### 使用代理访问 YouTube

若本机无法直连 YouTube（例如未开系统代理或需指定代理），可让服务端请求走代理：

- **方式一**：在 `web/.env.local` 中设置（需带协议）：

  ```bash
  HTTPS_PROXY=http://127.0.0.1:7890
  ```

  其中 `7890` 替换为你本地代理端口（Clash / V2Ray 等常用 7890、7891、1087 等）。

- **方式二**：启动时传入环境变量：
  ```bash
  HTTPS_PROXY=http://127.0.0.1:7890 pnpm --filter @ai-tools/youtube-transcript-web dev
  ```

设置后重启开发服务器，获取字幕的请求会经该代理访问 YouTube。
