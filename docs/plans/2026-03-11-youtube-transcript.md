# YouTube 视频转录工具实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**目标:** 构建一个免费的 YouTube 视频转录工具，支持 AI 摘要、翻译和导出功能

**架构:** Monorepo (pnpm + Turbo)，Next.js App Router + API Routes，使用 youtube-transcript (优先) 和 Groq API (备用)，极简 UI 设计，双语支持

**技术栈:** Next.js 15, React 19, TypeScript, TailwindCSS, shadcn/ui, next-intl, Vitest, Playwright

**Monorepo 结构:**
```
ai-tools/
├── apps/
│   ├── youtube-transcript/    # YouTube 转录工具
│   └── (future apps)          # 未来应用
├── packages/
│   ├── ui/                    # 共享 UI 组件
│   ├── config/                # 共享配置 (ESLint, TypeScript, Tailwind)
│   ├── api-clients/           # 共享 API 客户端
│   └── utils/                 # 共享工具函数
├── package.json
├── pnpm-workspace.yaml
└── turbo.json
```

---

## 前置准备

### Task 0: Monorepo 项目初始化

**Files:**
- Create: `package.json` (根目录)
- Create: `pnpm-workspace.yaml`
- Create: `turbo.json`
- Create: `.gitignore`
- Create: `.npmrc`
- Create: `apps/.gitkeep`
- Create: `packages/.gitkeep`

**Step 1: 创建根 package.json**

```bash
cat > package.json << 'EOF'
{
  "name": "ai-tools",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "turbo run dev",
    "build": "turbo run build",
    "start": "turbo run start",
    "lint": "turbo run lint",
    "test": "turbo run test",
    "test:e2e": "turbo run test:e2e",
    "clean": "turbo run clean && rm -rf node_modules"
  },
  "devDependencies": {
    "turbo": "^2.0.0",
    "typescript": "^5.0.0",
    "prettier": "^3.0.0",
    "eslint": "^9.0.0"
  },
  "packageManager": "pnpm@9.0.0",
  "engines": {
    "node": ">=18.0.0",
    "pnpm": ">=9.0.0"
  }
}
EOF
```

**Step 2: 创建 pnpm workspace 配置**

```bash
cat > pnpm-workspace.yaml << 'EOF'
packages:
  - 'apps/*'
  - 'packages/*'
EOF
```

**Step 3: 创建 Turbo 配置**

```bash
cat > turbo.json << 'EOF'
{
  "$schema": "https://turbo.build/schema.json",
  "globalDependencies": ["**/.env.*local"],
  "pipeline": {
    "build": {
      "dependsOn": ["^build"],
      "outputs": [".next/**", "!.next/cache/**", "dist/**"]
    },
    "dev": {
      "cache": false,
      "persistent": true
    },
    "start": {
      "cache": false,
      "persistent": true
    },
    "lint": {
      "dependsOn": ["^lint"]
    },
    "test": {
      "dependsOn": ["^build"],
      "outputs": ["coverage/**"]
    },
    "test:e2e": {
      "cache": false,
      "dependsOn": ["^build"]
    },
    "clean": {
      "cache": false
    }
  }
}
EOF
```

**Step 4: 创建 .gitignore**

```bash
cat > .gitignore << 'EOF'
# Dependencies
node_modules
.pnpm-store

# Build outputs
.next
out
dist
build
turbo

# Environment files
.env
.env*.local
.env.local

# Debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*

# IDE
.vscode/*
!.vscode/settings.json
!.vscode/extensions.json
.idea
*.swp
*.swo

# OS
.DS_Store
*.pem

# Testing
coverage
.nyc_output
playwright-report
test-results

# Vercel
.vercel

# Turbo
.turbo
EOF
```

**Step 5: 创建 .npmrc**

```bash
cat > .npmrc << 'EOF'
shamefully-hoist=true
strict-peer-dependencies=false
EOF
```

**Step 6: 创建目录结构**

```bash
mkdir -p apps packages
touch apps/.gitkeep packages/.gitkeep
```

**Step 7: 提交 monorepo 基础结构**

```bash
git add .
git commit -m "chore: initialize monorepo with pnpm and turbo"
```

---

## 共享包

### Task 1: 创建共享配置包

**Files:**
- Create: `packages/config/package.json`
- Create: `packages/config/index.ts`
- Create: `packages/config/tsconfig.json`
- Create: `packages/config/eslint-config-next.js`
- Create: `packages/config/tailwind.config.ts`
- Create: `packages/config/prettier.config.js`

**Step 1: 创建 config 包的 package.json**

```bash
mkdir -p packages/config
cat > packages/config/package.json << 'EOF'
{
  "name": "@ai-tools/config",
  "version": "0.1.0",
  "private": true,
  "main": "index.ts",
  "files": [
    "*.js",
    "*.ts"
  ]
}
EOF
```

**Step 2: 创建 TypeScript 配置导出**

```bash
cat > packages/config/tsconfig.json << 'EOF'
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@ai-tools/ui": ["../../packages/ui/src"],
      "@ai-tools/utils": ["../../packages/utils/src"],
      "@ai-tools/api-clients": ["../../packages/api-clients/src"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
EOF
```

**Step 3: 创建 Tailwind 配置**

```bash
cat > packages/config/tailwind.config.ts << 'EOF'
import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
};
export default config;
EOF
```

**Step 4: 创建 Prettier 配置**

```bash
cat > packages/config/prettier.config.js << 'EOF'
module.exports = {
  semi: false,
  singleQuote: true,
  trailingComma: 'es5',
  tabWidth: 2,
  printWidth: 100,
};
EOF
```

**Step 5: 创建 index.ts 导出所有配置**

```bash
cat > packages/config/index.ts << 'EOF'
export * from './tailwind.config';
EOF
```

**Step 6: 提交 config 包**

```bash
git add .
git commit -m "chore: add shared config package"
```

---

### Task 2: 创建共享工具包

**Files:**
- Create: `packages/utils/package.json`
- Create: `packages/utils/src/index.ts`
- Create: `packages/utils/src/youtube.ts`
- Create: `packages/utils/src/format.ts`

**Step 1: 创建 utils 包结构**

```bash
mkdir -p packages/utils/src
cat > packages/utils/package.json << 'EOF'
{
  "name": "@ai-tools/utils",
  "version": "0.1.0",
  "private": true,
  "main": "./src/index.ts",
  "types": "./src/index.ts",
  "scripts": {
    "lint": "eslint src/",
    "test": "vitest"
  },
  "devDependencies": {
    "vitest": "^2.0.0",
    "@ai-tools/config": "workspace:*"
  }
}
EOF
```

**Step 2: 创建 YouTube 工具函数**

```bash
cat > packages/utils/src/youtube.ts << 'EOF'
export function extractVideoId(url: string): string | null {
  const patterns = [
    /(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([^&\n?#]+)/,
    /^([a-zA-Z0-9_-]{11})$/
  ];

  for (const pattern of patterns) {
    const match = url.match(pattern);
    if (match && match[1]) {
      return match[1];
    }
  }

  return null;
}

export function buildYouTubeUrl(videoId: string): string {
  return `https://www.youtube.com/watch?v=${videoId}`;
}

export function buildYouTubeEmbedUrl(videoId: string): string {
  return `https://www.youtube.com/embed/${videoId}`;
}

export function buildThumbnailUrl(videoId: string): string {
  return `https://img.youtube.com/vi/${videoId}/maxresdefault.jpg`;
}
EOF
```

**Step 3: 创建格式化工具函数**

```bash
cat > packages/utils/src/format.ts << 'EOF'
export function formatTimestamp(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
  return `${minutes}:${secs.toString().padStart(2, '0')}`;
}

export function formatSrtTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const ms = Math.floor((seconds % 1) * 1000);

  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')},${ms.toString().padStart(3, '0')}`;
}

export function formatVttTime(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  const ms = Math.floor((seconds % 1) * 1000);

  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}.${ms.toString().padStart(3, '0')}`;
}
EOF
```

**Step 4: 创建通用工具函数**

```bash
cat > packages/utils/src/common.ts << 'EOF'
import {type ClassValue, clsx} from 'clsx';
import {twMerge} from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function sleep(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export function truncate(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text;
  return text.slice(0, maxLength) + '...';
}
EOF
```

**Step 5: 创建导出文件**

```bash
cat > packages/utils/src/index.ts << 'EOF'
export * from './youtube';
export * from './format';
export * from './common';
EOF
```

**Step 6: 创建测试文件**

```bash
mkdir -p packages/utils/src/__tests__
cat > packages/utils/src/__tests__/youtube.test.ts << 'EOF'
import {describe, it, expect} from 'vitest';
import {extractVideoId} from '../youtube';

describe('extractVideoId', () => {
  it('should extract video ID from standard YouTube URL', () => {
    expect(extractVideoId('https://www.youtube.com/watch?v=dQw4w9WgXcQ')).toBe('dQw4w9WgXcQ');
  });

  it('should extract video ID from short URL', () => {
    expect(extractVideoId('https://youtu.be/dQw4w9WgXcQ')).toBe('dQw4w9WgXcQ');
  });

  it('should extract video ID from embed URL', () => {
    expect(extractVideoId('https://www.youtube.com/embed/dQw4w9WgXcQ')).toBe('dQw4w9WgXcQ');
  });

  it('should return null for invalid URL', () => {
    expect(extractVideoId('https://example.com')).toBeNull();
  });

  it('should accept raw video ID', () => {
    expect(extractVideoId('dQw4w9WgXcQ')).toBe('dQw4w9WgXcQ');
  });
});
EOF
```

**Step 7: 添加依赖**

```bash
cd packages/utils && pnpm add clsx tailwind-merge && cd ../..
```

**Step 8: 更新 utils package.json 添加依赖**

```bash
cat > packages/utils/package.json << 'EOF'
{
  "name": "@ai-tools/utils",
  "version": "0.1.0",
  "private": true,
  "main": "./src/index.ts",
  "types": "./src/index.ts",
  "scripts": {
    "lint": "eslint src/",
    "test": "vitest"
  },
  "dependencies": {
    "clsx": "^2.1.0",
    "tailwind-merge": "^2.5.0"
  },
  "devDependencies": {
    "vitest": "^2.0.0",
    "@ai-tools/config": "workspace:*"
  }
}
EOF
```

**Step 9: 提交 utils 包**

```bash
git add .
git commit -m "chore: add shared utils package"
```

---

### Task 3: 创建共享 UI 组件包

**Files:**
- Create: `packages/ui/package.json`
- Create: `packages/ui/src/index.ts`
- Create: `packages/ui/src/components/button.tsx`
- Create: `packages/ui/src/components/input.tsx`
- Create: `packages/ui/src/components/card.tsx`
- Create: `packages/ui/src/components/textarea.tsx`
- Create: `packages/ui/src/components/select.tsx`

**Step 1: 创建 UI 包结构**

```bash
mkdir -p packages/ui/src/components
cat > packages/ui/package.json << 'EOF'
{
  "name": "@ai-tools/ui",
  "version": "0.1.0",
  "private": true,
  "main": "./src/index.ts",
  "types": "./src/index.ts",
  "scripts": {
    "lint": "eslint src/",
    "test": "vitest"
  },
  "dependencies": {
    "react": "^19.0.0",
    "@ai-tools/utils": "workspace:*",
    "class-variance-authority": "^0.7.0"
  },
  "devDependencies": {
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "vitest": "^2.0.0",
    "@ai-tools/config": "workspace:*"
  }
}
EOF
```

**Step 2: 创建 Button 组件**

```bash
cat > packages/ui/src/components/button.tsx << 'EOF'
import * as React from 'react';
import {cva, type VariantProps} from 'class-variance-authority';
import {cn} from '@ai-tools/utils';

const buttonVariants = cva(
  'inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'bg-primary text-primary-foreground hover:bg-primary/90',
        destructive: 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
        outline: 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
        secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80',
        ghost: 'hover:bg-accent hover:text-accent-foreground',
        link: 'text-primary underline-offset-4 hover:underline',
      },
      size: {
        default: 'h-10 px-4 py-2',
        sm: 'h-9 rounded-md px-3',
        lg: 'h-11 rounded-md px-8',
        icon: 'h-10 w-10',
      },
    },
    defaultVariants: {
      variant: 'default',
      size: 'default',
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({className, variant, size, ...props}, ref) => {
    return (
      <button
        className={cn(buttonVariants({variant, size, className}))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = 'Button';

export {Button, buttonVariants};
EOF
```

**Step 3: 创建 Input 组件**

```bash
cat > packages/ui/src/components/input.tsx << 'EOF'
import * as React from 'react';
import {cn} from '@ai-tools/utils';

export interface InputProps
  extends React.InputHTMLAttributes<HTMLInputElement> {}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  ({className, type, ...props}, ref) => {
    return (
      <input
        type={type}
        className={cn(
          'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        ref={ref}
        {...props}
      />
    );
  }
);
Input.displayName = 'Input';

export {Input};
EOF
```

**Step 4: 创建 Card 组件**

```bash
cat > packages/ui/src/components/card.tsx << 'EOF'
import * as React from 'react';
import {cn} from '@ai-tools/utils';

const Card = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({className, ...props}, ref) => (
  <div
    ref={ref}
    className={cn(
      'rounded-lg border bg-card text-card-foreground shadow-sm',
      className
    )}
    {...props}
  />
));
Card.displayName = 'Card';

const CardHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({className, ...props}, ref) => (
  <div
    ref={ref}
    className={cn('flex flex-col space-y-1.5 p-6', className)}
    {...props}
  />
));
CardHeader.displayName = 'CardHeader';

const CardTitle = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({className, ...props}, ref) => (
  <h3
    ref={ref}
    className={cn(
      'text-2xl font-semibold leading-none tracking-tight',
      className
    )}
    {...props}
  />
));
CardTitle.displayName = 'CardTitle';

const CardDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({className, ...props}, ref) => (
  <p
    ref={ref}
    className={cn('text-sm text-muted-foreground', className)}
    {...props}
  />
));
CardDescription.displayName = 'CardDescription';

const CardContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({className, ...props}, ref) => (
  <div ref={ref} className={cn('p-6 pt-0', className)} {...props} />
));
CardContent.displayName = 'CardContent';

const CardFooter = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({className, ...props}, ref) => (
  <div
    ref={ref}
    className={cn('flex items-center p-6 pt-0', className)}
    {...props}
  />
));
CardFooter.displayName = 'CardFooter';

export {Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent};
EOF
```

**Step 5: 创建 Textarea 组件**

```bash
cat > packages/ui/src/components/textarea.tsx << 'EOF'
import * as React from 'react';
import {cn} from '@ai-tools/utils';

export interface TextareaProps
  extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  ({className, ...props}, ref) => {
    return (
      <textarea
        className={cn(
          'flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        ref={ref}
        {...props}
      />
    );
  }
);
Textarea.displayName = 'Textarea';

export {Textarea};
EOF
```

**Step 6: 创建 Select 组件**

```bash
cat > packages/ui/src/components/select.tsx << 'EOF'
'use client';

import * as React from 'react';
import {cn} from '@ai-tools/utils';

export interface SelectProps
  extends React.SelectHTMLAttributes<HTMLSelectElement> {}

const Select = React.forwardRef<HTMLSelectElement, SelectProps>(
  ({className, children, ...props}, ref) => {
    return (
      <select
        className={cn(
          'flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50',
          className
        )}
        ref={ref}
        {...props}
      >
        {children}
      </select>
    );
  }
);
Select.displayName = 'Select';

export {Select};
EOF
```

**Step 7: 创建导出文件**

```bash
cat > packages/ui/src/index.ts << 'EOF'
export {Button, buttonVariants} from './components/button';
export type {ButtonProps} from './components/button';

export {Input} from './components/input';
export type {InputProps} from './components/input';

export {Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent} from './components/card';

export {Textarea} from './components/textarea';
export type {TextareaProps} from './components/textarea';

export {Select} from './components/select';
export type {SelectProps} from './components/select';
EOF
```

**Step 8: 提交 UI 包**

```bash
git add .
git commit -m "chore: add shared UI components package"
```

---

### Task 4: 创建 API 客户端包

**Files:**
- Create: `packages/api-clients/package.json`
- Create: `packages/api-clients/src/index.ts`
- Create: `packages/api-clients/src/youtube.ts`
- Create: `packages/api-clients/src/groq.ts`

**Step 1: 创建 API 客户端包结构**

```bash
mkdir -p packages/api-clients/src
cat > packages/api-clients/package.json << 'EOF'
{
  "name": "@ai-tools/api-clients",
  "version": "0.1.0",
  "private": true,
  "main": "./src/index.ts",
  "types": "./src/index.ts",
  "scripts": {
    "lint": "eslint src/",
    "test": "vitest"
  },
  "dependencies": {
    "youtube-transcript": "^1.2.0",
    "groq-sdk": "^0.7.0"
  },
  "devDependencies": {
    "@types/youtube-transcript": "^1.0.0",
    "vitest": "^2.0.0",
    "@ai-tools/config": "workspace:*"
  }
}
EOF
```

**Step 2: 创建 YouTube 客户端**

```bash
cat > packages/api-clients/src/youtube.ts << 'EOF'
import {YoutubeTranscript} from 'youtube-transcript';

export interface TranscriptItem {
  text: string;
  duration: number;
  offset: number;
  lang?: string;
}

export interface TranscriptResult {
  videoId: string;
  items: TranscriptItem[];
  language?: string;
}

export async function getTranscript(videoId: string): Promise<TranscriptResult> {
  try {
    const transcript = await YoutubeTranscript.fetchTranscript(videoId);

    return {
      videoId,
      items: transcript.map(item => ({
        text: item.text,
        duration: item.duration,
        offset: item.offset,
        lang: item.lang
      }))
    };
  } catch (error) {
    throw new Error(`Failed to fetch transcript: ${error instanceof Error ? error.message : 'Unknown error'}`);
  }
}
EOF
```

**Step 3: 创建 Groq 客户端**

```bash
cat > packages/api-clients/src/groq.ts << 'EOF'
import Groq from 'groq-sdk';

const groq = new Groq({
  apiKey: process.env.GROQ_API_KEY || '',
});

export async function generateSummary(transcript: string): Promise<string> {
  const response = await groq.chat.completions.create({
    messages: [
      {
        role: 'system',
        content: 'You are a helpful assistant that summarizes video transcripts. Provide a concise summary in 3-5 bullet points.'
      },
      {
        role: 'user',
        content: `Please summarize this video transcript:\n\n${transcript}`
      }
    ],
    model: 'llama-3.3-70b-versatile',
    temperature: 0.5,
    max_tokens: 500,
  });

  return response.choices[0]?.message?.content || '';
}

export async function translateText(text: string, targetLang: string): Promise<string> {
  const langNames: Record<string, string> = {
    en: 'English',
    zh: 'Chinese',
    es: 'Spanish',
    fr: 'French',
    de: 'German',
    ja: 'Japanese',
    ko: 'Korean'
  };

  const response = await groq.chat.completions.create({
    messages: [
      {
        role: 'system',
        content: `You are a professional translator. Translate the given text to ${langNames[targetLang] || targetLang}. Preserve the original formatting and structure.`
      },
      {
        role: 'user',
        content: text
      }
    ],
    model: 'llama-3.3-70b-versatile',
    temperature: 0.3,
    max_tokens: 2000,
  });

  return response.choices[0]?.message?.content || '';
}
EOF
```

**Step 4: 创建导出文件**

```bash
cat > packages/api-clients/src/index.ts << 'EOF'
export * from './youtube';
export * from './groq';
EOF
```

**Step 5: 提交 API 客户端包**

```bash
git add .
git commit -m "chore: add shared API clients package"
```

---

## YouTube 转录应用

### Task 5: 创建 YouTube 转录应用基础结构

**Files:**
- Create: `apps/youtube-transcript/package.json`
- Create: `apps/youtube-transcript/tsconfig.json`
- Create: `apps/youtube-transcript/next.config.js`
- Create: `apps/youtube-transcript/tailwind.config.ts`
- Create: `apps/youtube-transcript/postcss.config.js`
- Create: `apps/youtube-transcript/.env.local.example`

**Step 1: 创建应用 package.json**

```bash
mkdir -p apps/youtube-transcript
cat > apps/youtube-transcript/package.json << 'EOF'
{
  "name": "@ai-tools/youtube-transcript",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "next lint",
    "test": "vitest",
    "test:e2e": "playwright test"
  },
  "dependencies": {
    "next": "^15.0.0",
    "react": "^19.0.0",
    "react-dom": "^19.0.0",
    "next-intl": "^3.22.0",
    "lucide-react": "^0.460.0",
    "@ai-tools/ui": "workspace:*",
    "@ai-tools/utils": "workspace:*",
    "@ai-tools/api-clients": "workspace:*",
    "@ai-tools/config": "workspace:*"
  },
  "devDependencies": {
    "@types/node": "^20.0.0",
    "@types/react": "^19.0.0",
    "@types/react-dom": "^19.0.0",
    "typescript": "^5.0.0",
    "tailwindcss": "^3.4.0",
    "postcss": "^8.4.0",
    "autoprefixer": "^10.4.0",
    "tailwindcss-animate": "^1.0.7",
    "eslint": "^9.0.0",
    "eslint-config-next": "^15.0.0",
    "vitest": "^2.0.0",
    "@vitejs/plugin-react": "^4.3.0",
    "@playwright/test": "^1.48.0"
  }
}
EOF
```

**Step 2: 创建应用 tsconfig.json**

```bash
cat > apps/youtube-transcript/tsconfig.json << 'EOF'
{
  "extends": "@ai-tools/config/tsconfig",
  "compilerOptions": {
    "plugins": [
      {
        "name": "next"
      }
    ]
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
EOF
```

**Step 3: 创建 next.config.js**

```bash
cat > apps/youtube-transcript/next.config.js << 'EOF'
const createNextIntlPlugin = require('next-intl/plugin');

const withNextIntl = createNextIntlPlugin('./src/i18n.ts');

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
};

module.exports = withNextIntl(nextConfig);
EOF
```

**Step 4: 创建 tailwind.config.ts**

```bash
cat > apps/youtube-transcript/tailwind.config.ts << 'EOF'
import config from '@ai-tools/config/tailwind.config';

export default config;
EOF
```

**Step 5: 创建 postcss.config.js**

```bash
cat > apps/youtube-transcript/postcss.config.js << 'EOF'
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
EOF
```

**Step 6: 创建 .env.local.example**

```bash
cat > apps/youtube-transcript/.env.local.example << 'EOF'
# Groq API Key (免费层: https://console.groq.com/)
GROQ_API_KEY=your_groq_api_key_here

# 应用 URL
NEXT_PUBLIC_APP_URL=http://localhost:3000
EOF
```

**Step 7: 提交应用基础结构**

```bash
git add .
git commit -m "chore: add youtube-transcript app base structure"
```

---

### Task 6: 创建应用国际化配置

**Files:**
- Create: `apps/youtube-transcript/src/i18n.ts`
- Create: `apps/youtube-transcript/src/middleware.ts`
- Create: `apps/youtube-transcript/messages/en.json`
- Create: `apps/youtube-transcript/messages/zh.json`

**Step 1: 创建 i18n 配置**

```bash
mkdir -p apps/youtube-transcript/src apps/youtube-transcript/messages
cat > apps/youtube-transcript/src/i18n.ts << 'EOF'
import {notFound} from 'next/navigation';
import {getRequestConfig} from 'next-intl/server';

export const locales = ['en', 'zh'] as const;
export type Locale = (typeof locales)[number];

export default getRequestConfig(async ({locale}) => {
  if (!locales.includes(locale as Locale)) notFound();

  return {
    messages: (await import(`../../messages/${locale}.json`)).default
  };
});
EOF
```

**Step 2: 创建 middleware**

```bash
cat > apps/youtube-transcript/src/middleware.ts << 'EOF'
import createMiddleware from 'next-intl/middleware';
import {locales} from './i18n';

export default createMiddleware({
  locales,
  defaultLocale: 'en',
  localePrefix: 'as-needed'
});

export const config = {
  matcher: ['/((?!api|_next|_vercel|.*\\..*).*)']
};
EOF
```

**Step 3: 创建英文翻译文件**

```bash
cat > apps/youtube-transcript/messages/en.json << 'EOF'
{
  "meta": {
    "title": "YouTube Transcript Tool",
    "description": "Get instant, accurate transcripts from any YouTube video with AI-powered summaries and translations."
  },
  "header": {
    "title": "YouTube Transcript Tool",
    "languageSwitch": "Language"
  },
  "input": {
    "placeholder": "Enter YouTube video URL",
    "button": "Get Transcript",
    "validating": "Validating URL...",
    "invalid": "Please enter a valid YouTube URL"
  },
  "transcript": {
    "title": "Transcript",
    "loading": "Fetching transcript, please wait...",
    "noSubtitles": "No subtitles available for this video",
    "error": "Failed to fetch transcript",
    "copy": "Copy",
    "export": "Export"
  },
  "summary": {
    "title": "AI Summary",
    "button": "Generate Summary",
    "loading": "Generating summary...",
    "error": "Failed to generate summary",
    "copy": "Copy"
  },
  "translation": {
    "title": "Translation",
    "button": "Translate",
    "loading": "Translating...",
    "error": "Failed to translate",
    "copy": "Copy",
    "languages": {
      "en": "English",
      "zh": "Chinese",
      "es": "Spanish",
      "fr": "French",
      "de": "German",
      "ja": "Japanese",
      "ko": "Korean"
    }
  },
  "export": {
    "txt": "Export as TXT",
    "srt": "Export as SRT",
    "success": "Exported successfully",
    "copy": "Copy"
  }
}
EOF
```

**Step 4: 创建中文翻译文件**

```bash
cat > apps/youtube-transcript/messages/zh.json << 'EOF'
{
  "meta": {
    "title": "YouTube 视频转录工具",
    "description": "即时生成准确的视频转录，AI 驱动的智能摘要和翻译。"
  },
  "header": {
    "title": "YouTube 视频转录工具",
    "languageSwitch": "语言"
  },
  "input": {
    "placeholder": "请输入 YouTube 视频链接",
    "button": "获取转录",
    "validating": "验证链接中...",
    "invalid": "请输入有效的 YouTube 链接"
  },
  "transcript": {
    "title": "转录文本",
    "loading": "正在获取转录，请稍候...",
    "noSubtitles": "该视频暂无可用字幕",
    "error": "获取转录失败",
    "copy": "复制",
    "export": "导出"
  },
  "summary": {
    "title": "AI 摘要",
    "button": "生成摘要",
    "loading": "正在生成摘要...",
    "error": "生成摘要失败",
    "copy": "复制"
  },
  "translation": {
    "title": "翻译",
    "button": "翻译",
    "loading": "翻译中...",
    "error": "翻译失败",
    "copy": "复制",
    "languages": {
      "en": "英语",
      "zh": "中文",
      "es": "西班牙语",
      "fr": "法语",
      "de": "德语",
      "ja": "日语",
      "ko": "韩语"
    }
  },
  "export": {
    "txt": "导出为 TXT",
    "srt": "导出为 SRT",
    "success": "导出成功",
    "copy": "复制"
  }
}
EOF
```

**Step 5: 提交国际化配置**

```bash
git add .
git commit -m "feat: add i18n configuration"
```

---

### Task 7: 创建应用 API 路由

**Files:**
- Create: `apps/youtube-transcript/src/app/api/transcript/route.ts`
- Create: `apps/youtube-transcript/src/app/api/summarize/route.ts`
- Create: `apps/youtube-transcript/src/app/api/translate/route.ts`

**Step 1: 创建转录 API**

```bash
mkdir -p apps/youtube-transcript/src/app/api/transcript
cat > apps/youtube-transcript/src/app/api/transcript/route.ts << 'EOF'
import {NextRequest, NextResponse} from 'next/server';
import {getTranscript} from '@ai-tools/api-clients';
import {extractVideoId} from '@ai-tools/utils';

export async function POST(request: NextRequest) {
  try {
    const {url} = await request.json();

    if (!url) {
      return NextResponse.json(
        {error: 'URL is required'},
        {status: 400}
      );
    }

    const videoId = extractVideoId(url);

    if (!videoId) {
      return NextResponse.json(
        {error: 'Invalid YouTube URL'},
        {status: 400}
      );
    }

    const result = await getTranscript(videoId);

    return NextResponse.json(result);
  } catch (error) {
    console.error('Transcript error:', error);

    return NextResponse.json(
      {
        error: 'Failed to fetch transcript',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      {status: 500}
    );
  }
}

export const runtime = 'edge';
EOF
```

**Step 2: 创建摘要 API**

```bash
mkdir -p apps/youtube-transcript/src/app/api/summarize
cat > apps/youtube-transcript/src/app/api/summarize/route.ts << 'EOF'
import {NextRequest, NextResponse} from 'next/server';
import {generateSummary} from '@ai-tools/api-clients';

export async function POST(request: NextRequest) {
  try {
    const {transcript} = await request.json();

    if (!transcript || typeof transcript !== 'string') {
      return NextResponse.json(
        {error: 'Transcript is required and must be a string'},
        {status: 400}
      );
    }

    const maxLength = 12000;
    const truncatedTranscript = transcript.length > maxLength
      ? transcript.substring(0, maxLength) + '...'
      : transcript;

    const summary = await generateSummary(truncatedTranscript);

    return NextResponse.json({summary});
  } catch (error) {
    console.error('Summary error:', error);

    return NextResponse.json(
      {
        error: 'Failed to generate summary',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      {status: 500}
    );
  }
}

export const runtime = 'edge';
EOF
```

**Step 3: 创建翻译 API**

```bash
mkdir -p apps/youtube-transcript/src/app/api/translate
cat > apps/youtube-transcript/src/app/api/translate/route.ts << 'EOF'
import {NextRequest, NextResponse} from 'next/server';
import {translateText} from '@ai-tools/api-clients';

export async function POST(request: NextRequest) {
  try {
    const {text, targetLang} = await request.json();

    if (!text || typeof text !== 'string') {
      return NextResponse.json(
        {error: 'Text is required and must be a string'},
        {status: 400}
      );
    }

    if (!targetLang || typeof targetLang !== 'string') {
      return NextResponse.json(
        {error: 'targetLang is required'},
        {status: 400}
      );
    }

    const translation = await translateText(text, targetLang);

    return NextResponse.json({translation});
  } catch (error) {
    console.error('Translation error:', error);

    return NextResponse.json(
      {
        error: 'Failed to translate',
        details: error instanceof Error ? error.message : 'Unknown error'
      },
      {status: 500}
    );
  }
}

export const runtime = 'edge';
EOF
```

**Step 4: 提交 API 路由**

```bash
git add .
git commit -m "feat: add API routes for transcript, summary, and translation"
```

---

### Task 8: 创建应用布局和页面

**Files:**
- Create: `apps/youtube-transcript/src/app/layout.tsx`
- Create: `apps/youtube-transcript/src/app/globals.css`
- Create: `apps/youtube-transcript/src/lib/navigation.ts`
- Create: `apps/youtube-transcript/src/app/[lang]/layout.tsx`
- Create: `apps/youtube-transcript/src/app/[lang]/page.tsx`

**Step 1: 创建根布局**

```bash
cat > apps/youtube-transcript/src/app/layout.tsx << 'EOF'
import type {Metadata} from 'next';
import {NextIntlClientProvider} from 'next-intl';
import {getMessages} from 'next-intl/server';
import {notFound} from 'next/navigation';
import {locales} from '../i18n';
import './globals.css';

export const metadata: Metadata = {
  title: 'YouTube Transcript Tool',
  description: 'Get instant transcripts from YouTube videos',
};

export function generateStaticParams() {
  return locales.map((locale) => ({locale}));
}

export default async function RootLayout({
  children,
  params: {locale}
}: {
  children: React.ReactNode;
  params: {locale: string};
}) {
  if (!locales.includes(locale as any)) {
    notFound();
  }

  const messages = await getMessages();

  return (
    <html lang={locale}>
      <body>
        <NextIntlClientProvider messages={messages}>
          {children}
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
EOF
```

**Step 2: 创建全局样式**

```bash
cat > apps/youtube-transcript/src/app/globals.css << 'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 222.2 84% 4.9%;
    --card: 0 0% 100%;
    --card-foreground: 222.2 84% 4.9%;
    --popover: 0 0% 100%;
    --popover-foreground: 222.2 84% 4.9%;
    --primary: 222.2 47.4% 11.2%;
    --primary-foreground: 210 40% 98%;
    --secondary: 210 40% 96.1%;
    --secondary-foreground: 222.2 47.4% 11.2%;
    --muted: 210 40% 96.1%;
    --muted-foreground: 215.4 16.3% 46.9%;
    --accent: 210 40% 96.1%;
    --accent-foreground: 222.2 47.4% 11.2%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 210 40% 98%;
    --border: 214.3 31.8% 91.4%;
    --input: 214.3 31.8% 91.4%;
    --ring: 222.2 84% 4.9%;
    --radius: 0.5rem;
  }

  .dark {
    --background: 222.2 84% 4.9%;
    --foreground: 210 40% 98%;
    --card: 222.2 84% 4.9%;
    --card-foreground: 210 40% 98%;
    --popover: 222.2 84% 4.9%;
    --popover-foreground: 210 40% 98%;
    --primary: 210 40% 98%;
    --primary-foreground: 222.2 47.4% 11.2%;
    --secondary: 217.2 32.6% 17.5%;
    --secondary-foreground: 210 40% 98%;
    --muted: 217.2 32.6% 17.5%;
    --muted-foreground: 215 20.2% 65.1%;
    --accent: 217.2 32.6% 17.5%;
    --accent-foreground: 210 40% 98%;
    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 210 40% 98%;
    --border: 217.2 32.6% 17.5%;
    --input: 217.2 32.6% 17.5%;
    --ring: 212.7 26.8% 83.9%;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground;
  }
}
EOF
```

**Step 3: 创建导航工具**

```bash
mkdir -p apps/youtube-transcript/src/lib
cat > apps/youtube-transcript/src/lib/navigation.ts << 'EOF'
'use client';

import {createNavigation} from 'next-intl/navigation';
import {defineRouting} from 'next-intl/routing';

const routing = defineRouting({
  locales: ['en', 'zh'],
  defaultLocale: 'en',
  localePrefix: 'as-needed'
});

export const {Link, redirect, usePathname, useRouter} = createNavigation(routing);
EOF
```

**Step 4: 创建语言布局**

```bash
mkdir -p apps/youtube-transcript/src/app/\[lang\]
cat > apps/youtube-transcript/src/app/\[lang\]/layout.tsx << 'EOF'
import {Link} from '@/lib/navigation';

export default function LocaleLayout({
  children,
  params: {locale}
}: {
  children: React.ReactNode;
  params: {locale: string};
}) {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-4 py-4 flex items-center justify-between">
          <h1 className="text-xl font-semibold">
            📺 <span className="hidden sm:inline">YouTube Transcript Tool</span>
          </h1>
          <div className="flex gap-2">
            <Link
              href="/"
              locale="en"
              className={`px-3 py-1 rounded text-sm ${locale === 'en' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              EN
            </Link>
            <Link
              href="/"
              locale="zh"
              className={`px-3 py-1 rounded text-sm ${locale === 'zh' ? 'bg-primary text-primary-foreground' : 'hover:bg-muted'}`}
            >
              中
            </Link>
          </div>
        </div>
      </header>
      <main className="container mx-auto px-4 py-8">
        {children}
      </main>
      <footer className="border-t mt-12">
        <div className="container mx-auto px-4 py-6 text-center text-sm text-muted-foreground">
          <p>100% Free • No Registration Required</p>
        </div>
      </footer>
    </div>
  );
}
EOF
```

**Step 5: 创建主页面**

```bash
cat > apps/youtube-transcript/src/app/\[lang\]/page.tsx << 'EOF'
'use client';

import {useState} from 'react';
import {useTranslations} from 'next-intl';
import {Youtube, FileText, Sparkles, Languages, Copy, Download, Loader2} from 'lucide-react';
import {Button} from '@ai-tools/ui';
import {Input} from '@ai-tools/ui';
import {Card, CardContent, CardHeader, CardTitle} from '@ai-tools/ui';
import {Textarea} from '@ai-tools/ui';
import {Select} from '@ai-tools/ui';
import {extractVideoId, formatTimestamp, formatSrtTime} from '@ai-tools/utils';

export interface TranscriptItem {
  text: string;
  duration: number;
  offset: number;
}

export default function HomePage() {
  const t = useTranslations();
  const [url, setUrl] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);
  const [transcript, setTranscript] = useState<TranscriptItem[]>([]);
  const [summary, setSummary] = useState('');
  const [translation, setTranslation] = useState('');
  const [targetLang, setTargetLang] = useState('zh');
  const [copied, setCopied] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const videoId = extractVideoId(url);

    if (!videoId) {
      setError(t('input.invalid'));
      return;
    }

    setError('');
    setLoading(true);
    setTranscript([]);
    setSummary('');
    setTranslation('');

    try {
      const response = await fetch('/api/transcript', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({url}),
      });

      const data = await response.json();

      if (!response.ok) {
        throw new Error(data.error || t('transcript.error'));
      }

      setTranscript(data.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : t('transcript.error'));
    } finally {
      setLoading(false);
    }
  };

  const handleGenerateSummary = async () => {
    const transcriptText = transcript.map(item => item.text).join(' ');
    const response = await fetch('/api/summarize', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({transcript: transcriptText}),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.error || t('summary.error'));
    }

    setSummary(data.summary);
  };

  const handleTranslate = async () => {
    const transcriptText = transcript.map(item => item.text).join(' ');
    const response = await fetch('/api/translate', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({text: transcriptText, targetLang}),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.error || t('translation.error'));
    }

    setTranslation(data.translation);
  };

  const handleCopy = async (text: string) => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleExportTxt = () => {
    const text = transcript.map(item => `[${formatTimestamp(item.offset)}] ${item.text}`).join('\n');
    const blob = new Blob([text], {type: 'text/plain'});
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'transcript.txt';
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleExportSrt = () => {
    let srtIndex = 1;
    const srt = transcript.map((item) => {
      const startTime = formatSrtTime(item.offset);
      const endTime = formatSrtTime(item.offset + item.duration);
      return `${srtIndex++}\n${startTime} --> ${endTime}\n${item.text}\n`;
    }).join('\n');

    const blob = new Blob([srt], {type: 'text/plain'});
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'transcript.srt';
    a.click();
    URL.revokeObjectURL(url);
  };

  const transcriptText = transcript.map(item => item.text).join(' ');

  return (
    <div className="space-y-8">
      <div className="text-center space-y-4">
        <h2 className="text-3xl font-bold">{t('header.title')}</h2>
        <p className="text-muted-foreground max-w-2xl mx-auto">
          {t('meta.description')}
        </p>
      </div>

      <form onSubmit={handleSubmit} className="w-full max-w-2xl mx-auto">
        <div className="flex gap-2">
          <div className="relative flex-1">
            <Youtube className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
            <Input
              type="url"
              placeholder={t('input.placeholder')}
              value={url}
              onChange={(e) => {
                setUrl(e.target.value);
                setError('');
              }}
              className="pl-10"
              disabled={loading}
            />
          </div>
          <Button type="submit" disabled={loading || !url}>
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                {t('input.validating')}
              </>
            ) : (
              t('input.button')
            )}
          </Button>
        </div>
        {error && (
          <p className="text-destructive text-sm mt-2">{error}</p>
        )}
      </form>

      {transcript.length > 0 && (
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2">
                <FileText className="h-5 w-5" />
                {t('transcript.title')} ({transcript.length} lines)
              </CardTitle>
              <div className="flex gap-2">
                <Button variant="outline" size="sm" onClick={() => handleCopy(transcriptText)}>
                  <Copy className="h-4 w-4 mr-1" />
                  {copied ? 'Copied!' : t('transcript.copy')}
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportTxt}>
                  <Download className="h-4 w-4 mr-1" />
                  TXT
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportSrt}>
                  <Download className="h-4 w-4 mr-1" />
                  SRT
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 max-h-[500px] overflow-y-auto">
              {transcript.map((item, index) => (
                <div
                  key={index}
                  className="flex gap-3 p-2 rounded hover:bg-muted/50 transition-colors"
                >
                  <span className="text-xs text-muted-foreground font-mono shrink-0">
                    {formatTimestamp(item.offset)}
                  </span>
                  <span className="text-sm">{item.text}</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {transcript.length > 0 && (
        <div className="grid md:grid-cols-2 gap-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Sparkles className="h-5 w-5" />
                {t('summary.title')}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {!summary ? (
                <Button onClick={handleGenerateSummary} className="w-full">
                  {t('summary.button')}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={summary}
                    onChange={(e) => setSummary(e.target.value)}
                    rows={8}
                    className="resize-none"
                  />
                  <Button variant="outline" onClick={() => handleCopy(summary)}>
                    <Copy className="h-4 w-4 mr-1" />
                    {copied ? 'Copied!' : t('summary.copy')}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Languages className="h-5 w-5" />
                {t('translation.title')}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <Select value={targetLang} onChange={(e) => setTargetLang(e.target.value)}>
                <option value="en">{t('translation.languages.en')}</option>
                <option value="zh">{t('translation.languages.zh')}</option>
                <option value="es">{t('translation.languages.es')}</option>
                <option value="fr">{t('translation.languages.fr')}</option>
                <option value="de">{t('translation.languages.de')}</option>
                <option value="ja">{t('translation.languages.ja')}</option>
                <option value="ko">{t('translation.languages.ko')}</option>
              </Select>
              {!translation ? (
                <Button onClick={handleTranslate} className="w-full">
                  {t('translation.button')}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={translation}
                    onChange={(e) => setTranslation(e.target.value)}
                    rows={8}
                    className="resize-none"
                  />
                  <Button variant="outline" onClick={() => handleCopy(translation)}>
                    <Copy className="h-4 w-4 mr-1" />
                    {copied ? 'Copied!' : t('translation.copy')}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
EOF
```

**Step 6: 提交布局和页面**

```bash
git add .
git commit -m "feat: add app layout and main page"
```

---

### Task 9: 添加测试配置

**Files:**
- Create: `apps/youtube-transcript/vitest.config.ts`
- Create: `apps/youtube-transcript/playwright.config.ts`
- Create: `apps/youtube-transcript/tests/e2e/basic-flow.test.ts`

**Step 1: 创建 vitest 配置**

```bash
mkdir -p apps/youtube-transcript/tests/e2e
cat > apps/youtube-transcript/vitest.config.ts << 'EOF'
import {defineConfig} from 'vitest/config';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    environment: 'jsdom',
    globals: true,
  },
});
EOF
```

**Step 2: 创建 Playwright 配置**

```bash
cat > apps/youtube-transcript/playwright.config.ts << 'EOF'
import {defineConfig, devices} from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: {...devices['Desktop Chrome']},
    },
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
EOF
```

**Step 3: 创建 E2E 测试**

```bash
cat > apps/youtube-transcript/tests/e2e/basic-flow.test.ts << 'EOF'
import {test, expect} from '@playwright/test';

test.describe('YouTube Transcript Tool', () => {
  test.beforeEach(async ({page}) => {
    await page.goto('/');
  });

  test('should show title and language switcher', async ({page}) => {
    await expect(page.getByText('YouTube Transcript Tool')).toBeVisible();
    await expect(page.getByText('EN')).toBeVisible();
    await expect(page.getByText('中')).toBeVisible();
  });

  test('should validate invalid URL', async ({page}) => {
    await page.fill('input[type="url"]', 'https://example.com');
    await page.click('button[type="submit"]');
    await expect(page.getByText(/valid youtube url/i)).toBeVisible();
  });

  test('should switch language', async ({page}) => {
    await page.click('text=中');
    await expect(page.getByText('请输入 YouTube 视频链接')).toBeVisible();

    await page.click('text=EN');
    await expect(page.getByText(/enter youtube video url/i)).toBeVisible();
  });
});
EOF
```

**Step 4: 提交测试配置**

```bash
git add .
git commit -m "test: add test configurations for vitest and playwright"
```

---

### Task 10: 添加部署配置和文档

**Files:**
- Create: `apps/youtube-transcript/vercel.json`
- Create: `apps/youtube-transcript/README.md`
- Create: `README.md` (根目录)

**Step 1: 创建 Vercel 配置**

```bash
cat > apps/youtube-transcript/vercel.json << 'EOF'
{
  "buildCommand": "pnpm --filter @ai-tools/youtube-transcript build",
  "devCommand": "pnpm --filter @ai-tools/youtube-transcript dev",
  "installCommand": "pnpm install",
  "framework": "nextjs",
  "regions": ["sin1"],
  "env": {
    "GROQ_API_KEY": "@groq-api-key"
  }
}
EOF
```

**Step 2: 创建应用 README**

```bash
cat > apps/youtube-transcript/README.md << 'EOF'
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

\`\`\`bash
# Install dependencies
pnpm install

# Run development server
pnpm --filter @ai-tools/youtube-transcript dev

# Run tests
pnpm --filter @ai-tools/youtube-transcript test
pnpm --filter @ai-tools/youtube-transcript test:e2e

# Build
pnpm --filter @ai-tools/youtube-transcript build
\`\`\`

## Environment Variables

Copy `.env.local.example` to `.env.local`:

\`\`\`bash
GROQ_API_KEY=your_groq_api_key_here
\`\`\`
EOF
```

**Step 3: 创建根 README**

```bash
cat > README.md << 'EOF'
# AI Tools Monorepo

A monorepo for AI-powered tools, built with Next.js, pnpm workspaces, and Turbo.

## Structure

\`\`\`
ai-tools/
├── apps/
│   └── youtube-transcript/    # YouTube 转录工具
├── packages/
│   ├── ui/                    # 共享 UI 组件
│   ├── config/                # 共享配置
│   ├── api-clients/           # 共享 API 客户端
│   └── utils/                 # 共享工具函数
└── package.json
\`\`\`

## Getting Started

\`\`\`bash
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
\`\`\`

## Packages

| Package | Description |
|---------|-------------|
| `@ai-tools/ui` | Shared React components |
| `@ai-tools/utils` | Shared utility functions |
| `@ai-tools/api-clients` | Shared API clients (YouTube, Groq) |
| `@ai-tools/config` | Shared TypeScript, Tailwind, ESLint configs |

## Apps

| App | Description |
|-----|-------------|
| `@ai-tools/youtube-transcript` | YouTube video transcript tool with AI summaries |
EOF
```

**Step 4: 提交部署配置和文档**

```bash
git add .
git commit -m "docs: add deployment config and README"
```

---

## 任务清单总结

- [x] Task 0: Monorepo 项目初始化
- [x] Task 1: 创建共享配置包
- [x] Task 2: 创建共享工具包
- [x] Task 3: 创建共享 UI 组件包
- [x] Task 4: 创建 API 客户端包
- [x] Task 5: 创建 YouTube 转录应用基础结构
- [x] Task 6: 创建应用国际化配置
- [x] Task 7: 创建应用 API 路由
- [x] Task 8: 创建应用布局和页面
- [x] Task 9: 添加测试配置
- [x] Task 10: 添加部署配置和文档

---

**实现计划完成！** 🎉

下一步：选择执行方式开始实现。
