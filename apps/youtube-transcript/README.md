# YouTube Video Transcript Tool

A free YouTube video transcript tool with AI-powered summaries and translations.

## Features

- 📹 Get instant transcripts from YouTube videos
- 🤖 AI-powered video summaries
- 🌐 Multi-language translation support
- 📥 Export transcripts as TXT or SRT files
- 🌍 Bilingual UI (English/Chinese)
- 💰 100% free, no registration required

## Development

```bash
# Install dependencies
pnpm install

# Run development server
pnpm --filter @ai-tools/youtube-transcript dev

# Run tests
pnpm --filter @ai-tools/youtube-transcript test
pnpm --filter @ai-tools/youtube-transcript test:e2e

# Build
pnpm --filter @ai-tools/youtube-transcript build
```

## Environment Variables

Copy `.env.local.example` to `.env.local`:

```bash
GROQ_API_KEY=your_groq_api_key_here
```

### 使用代理访问 YouTube

若本机无法直连 YouTube（例如未开系统代理或需指定代理），可让服务端请求走代理：

- **方式一**：在 `.env.local` 中设置（需带协议）：
  ```bash
  HTTPS_PROXY=http://127.0.0.1:7890
  ```
  其中 `7890` 替换为你本地代理端口（Clash / V2Ray 等常用 7890、7891、1087 等）。

- **方式二**：启动时传入环境变量：
  ```bash
  HTTPS_PROXY=http://127.0.0.1:7890 pnpm --filter @ai-tools/youtube-transcript dev
  ```

设置后重启开发服务器，获取字幕的请求会经该代理访问 YouTube。
