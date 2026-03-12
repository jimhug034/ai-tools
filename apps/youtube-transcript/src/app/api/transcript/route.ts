import { NextRequest, NextResponse } from "next/server";
import { getTranscript } from "@ai-tools/api-clients";
import { extractVideoId } from "@ai-tools/utils";

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

    const result = await getTranscript(videoId);

    return NextResponse.json(result);
  } catch (error) {
    console.error("Transcript error:", error);

    const proxyEnv =
      process.env.HTTPS_PROXY ||
      process.env.HTTP_PROXY ||
      process.env.https_proxy ||
      process.env.http_proxy;
    if (!proxyEnv?.trim()) {
      console.warn(
        "[YouTube] 未检测到 HTTPS_PROXY/HTTP_PROXY。若无法直连 YouTube，请在 .env.local 中设置 HTTPS_PROXY=http://127.0.0.1:7890 并重启。"
      );
    }

    const message = error instanceof Error ? error.message : "Unknown error";
    const rawCause = error instanceof Error ? error.cause : undefined;
    const causeStr =
      typeof rawCause === "object" && rawCause !== null
        ? String(
            "message" in rawCause
              ? (rawCause as { message?: string }).message
              : "code" in rawCause
                ? (rawCause as { code?: string }).code
                : ""
          )
        : "";
    const isNetworkError =
      message.includes("fetch failed") ||
      message.includes("ECONNREFUSED") ||
      message.includes("ETIMEDOUT") ||
      message.includes("ENOTFOUND");
    const isConnectTimeout =
      causeStr.includes("ConnectTimeoutError") ||
      causeStr.includes("UND_ERR_CONNECT_TIMEOUT");

    const proxySet = !!(
      process.env.HTTPS_PROXY ||
      process.env.HTTP_PROXY ||
      process.env.https_proxy ||
      process.env.http_proxy
    );
    const hint = isConnectTimeout
      ? proxySet
        ? "连接超时：当前已配置代理但仍超时，请确认代理软件已开启且端口正确（如 Clash 常用 7890）。"
        : "连接超时：本机无法直连 YouTube。请在项目 .env.local 中添加 HTTPS_PROXY=http://127.0.0.1:7890（端口改为你的代理端口），保存后重启开发服务。"
      : isNetworkError
        ? proxySet
          ? "网络错误：请确认代理已开启且能访问 YouTube。"
          : "网络错误：请在 .env.local 中设置 HTTPS_PROXY=http://127.0.0.1:7890 并重启开发服务。"
        : undefined;

    return NextResponse.json(
      {
        error: "Failed to fetch transcript",
        details: message,
        hint,
      },
      { status: 500 }
    );
  }
}
