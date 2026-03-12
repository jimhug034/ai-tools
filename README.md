# AI Tools Monorepo

A monorepo for AI-powered tools, built with Next.js, pnpm workspaces, and Turbo.

## Structure

```
ai-tools/
├── apps/
│   └── youtube-transcript/    # YouTube 转录工具
├── packages/
│   ├── ui/                    # 共享 UI 组件
│   ├── config/                # 共享配置
│   ├── api-clients/           # 共享 API 客户端
│   └── utils/                 # 共享工具函数
└── package.json
```

## Getting Started

```bash
# Install dependencies
pnpm install

# Run development server (all apps)
pnpm dev

# Run development server (specific app)
pnpm --filter @ai-tools/youtube-transcript dev

# Build all
pnpm build

# Run tests
pnpm test
```

## Packages

| Package                 | Description                                 |
| ----------------------- | ------------------------------------------- |
| `@ai-tools/ui`          | Shared React components                     |
| `@ai-tools/utils`       | Shared utility functions                    |
| `@ai-tools/api-clients` | Shared API clients (YouTube, Groq)          |
| `@ai-tools/config`      | Shared TypeScript, Tailwind, ESLint configs |

## Apps

| App                            | Description                                     |
| ------------------------------ | ----------------------------------------------- |
| `@ai-tools/youtube-transcript` | YouTube video transcript tool with AI summaries |
