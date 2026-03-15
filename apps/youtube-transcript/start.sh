#!/bin/bash
# 一次性启动 YouTube Transcript 服务

set -e

# 颜色输出
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting YouTube Transcript services...${NC}"

# 进入项目根目录
cd "$(dirname "$0")"

# 后台启动 Rust API
echo -e "${GREEN}→ Starting Rust API on port 8080...${NC}"
cd api
cargo run &
API_PID=$!
cd ..

# 等待 API 启动
sleep 3

# 启动 Next.js 前端
echo -e "${GREEN}→ Starting Next.js on port 3000...${NC}"
cd web
pnpm dev

# 当前端退出时，也停止 API
kill $API_PID 2>/dev/null || true
