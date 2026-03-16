# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目结构

这是一个基于 pnpm workspaces 和 Turborepo 的 monorepo，包含一个 YouTube 字幕工具应用。

```
ai-tools/
├── apps/
│   └── youtube-transcript/
│       ├── web/         # Next.js 15 前端 (React 19)
│       ├── api/         # Rust 后端服务 (Axum)
│       └── start.sh     # 一键启动脚本
└── packages/
    ├── ui/             # 共享 UI 组件 (shadcn/ui + Radix UI)
    ├── config/         # 共享 TypeScript/Tailwind 配置
    ├── api-clients/    # API 客户端 (Groq, YouTube)
    └── utils/          # 共享工具函数
```

## 常用命令

### 开发

```bash
# 一键启动所有服务 (Rust API + Next.js)
pnpm dev:yt

# 或直接运行脚本
./apps/youtube-transcript/start.sh

# 单独启动 Next.js 前端
pnpm dev

# 单独启动 Rust API
pnpm dev:api
```

### 构建/测试

```bash
# 构建所有
pnpm build

# 运行测试
pnpm test
pnpm test:e2e

# Lint 和 Format
pnpm lint          # 检查
pnpm lint:fix      # 自动修复
pnpm format        # 检查格式
pnpm format:fix    # 自动格式化
```

### Rust 相关

```bash
# 在 apps/youtube-transcript/api 目录下
cargo run          # 开发运行
cargo test         # 运行测试
cargo clippy       # Rust lint
```

## 技术栈版本

- **Node.js**: >=24 (见 .node-version)
- **pnpm**: >=10
- **Next.js**: 15.5.x
- **React**: 19.x
- **Rust**: 1.85+

## 代码规范

- **Linter**: oxlint (配置见 .oxlintrc.json)
- **Formatter**: oxfmt (配置见 .oxfmtrc.json)
- **Git Hooks**: lefthook (自动运行 format、lint、rust-test、rust-clippy)

### 格式化规则

- 行宽: 100 字符
- 缩进: 2 空格
- 引号: 双引号
- 分号: 无
- 尾随逗号: ES5

## 架构说明

### YouTube Transcript 应用

#### Rust API (`apps/youtube-transcript/api/`)

纯 Rust 实现，使用 Axum 框架，不依赖 yt-dlp：

- `main.rs` - API 路由和服务入口
- `innertube.rs` - YouTube InnerTube API 客户端
- `downloader.rs` - 视频下载器
- `extractor.rs` - 字幕提取器
- `converter.rs` - 字幕格式转换 (SRT, VTT, ASS, TXT, LRC)
- `types.rs` - 数据类型定义

API 端点：

- `POST /transcript` - 获取字幕 JSON
- `GET /format` - 获取指定格式字幕
- `GET /info` - 视频信息
- `GET /health` - 健康检查

#### Next.js 前端 (`apps/youtube-transcript/web/`)

- `src/app/` - App Router 结构
- `src/i18n.ts` - 国际化配置
- `src/instrumentation.ts` - 请求 instrumentation (支持 HTTPS_PROXY)

### 共享包

- `@ai-tools/ui` - Radix UI 组件 + lucide-react 图标
- `@ai-tools/config` - TypeScript 和 Tailwind 基础配置
- `@ai-tools/api-clients` - Groq SDK, youtubei.js 客户端
- `@ai-tools/utils` - 工具函数 (clsx, tailwind-merge)

## 环境变量

前端需要在 `apps/youtube-transcript/web/.env.local` 配置：

```bash
GROQ_API_KEY=your_key_here
HTTPS_PROXY=http://127.0.0.1:7890  # 可选，用于访问 YouTube
```

## 工作区命令

使用 turbo filter 针对特定包：

```bash
pnpm --filter @ai-tools/youtube-transcript-web dev
pnpm --filter @ai-tools/ui lint
```
