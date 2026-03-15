# YouTube Transcript Service (Rust)

基于 Rust + yt-dlp 的 YouTube 字幕获取服务。

## 依赖

- Rust 1.85+
- yt-dlp: `brew install yt-dlp`

## 运行

```bash
# 开发模式
cargo run

# 或指定端口
PORT=3000 cargo run

# 生产构建
cargo build --release
./target/release/youtube-transcript
```

## API

### POST /transcript

获取 YouTube 视频字幕。

**请求：**
```json
{
  "url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
}
```

**响应：**
```json
{
  "videoId": "dQw4w9WgXcQ",
  "items": [
    {
      "text": "Hello world",
      "duration": 2.5,
      "offset": 0.0
    }
  ]
}
```

### GET /health

健康检查。

**响应：**
```json
{
  "status": "healthy",
  "yt-dlp": "2025.01.15"
}
```

## 代理支持

服务会自动读取环境变量：
- `HTTPS_PROXY` 或 `HTTP_PROXY` - 代理地址

```bash
HTTPS_PROXY=http://127.0.0.1:7890 cargo run
```

## 集成到 Next.js

修改 `apps/youtube-transcript/src/app/api/transcript/route.ts`：

```typescript
const response = await fetch('http://localhost:8080/transcript', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ url })
});
```
