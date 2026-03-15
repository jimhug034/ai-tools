import { NextRequest, NextResponse } from "next/server";
import { extractVideoId } from "@ai-tools/utils";

// Rust 服务地址
const RUST_SERVICE_URL = process.env.RUST_TRANSCRIPT_SERVICE_URL || "http://localhost:8080";

// 显示代理配置信息
const proxyUrl =
  process.env.HTTPS_PROXY ||
  process.env.HTTP_PROXY ||
  process.env.https_proxy ||
  process.env.http_proxy;

if (proxyUrl) {
  console.log(`[YouTube] Proxy configured: ${proxyUrl}`);
}

export async function POST(request: NextRequest) {
  try {
    const { url } = await request.json();

    if (!url) {
      return NextResponse.json({ error: "URL is required" }, { status: 400 });
    }

    const videoId = extractVideoId(url);

    if (!videoId) {
      return NextResponse.json({ error: "Invalid YouTube URL" }, { status: 400 });
    }

    // 调用 Rust 服务获取字幕
    const response = await fetch(`${RUST_SERVICE_URL}/transcript`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ url }),
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({ error: "Unknown error" }));
      throw new Error(errorData.details || errorData.error || "Failed to fetch transcript");
    }

    const result = await response.json();
    return NextResponse.json(result);
  } catch (error) {
    console.error("Transcript error:", error);

    // 检查是否是 Rust 服务连接错误
    const isServiceError =
      error instanceof Error &&
      (error.message.includes("ECONNREFUSED") || error.message.includes("fetch failed"));

    const hint = isServiceError
      ? `无法连接到 Rust 字幕服务 (${RUST_SERVICE_URL})。请先启动服务：\ncd services/youtube-transcript-rs\ncargo run`
      : undefined;

    return NextResponse.json(
      {
        error: "Failed to fetch transcript",
        details: error instanceof Error ? error.message : "Unknown error",
        hint,
      },
      { status: 500 }
    );
  }
}
