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
