/**
 * 在 Node 运行时启动时，若设置了 HTTPS_PROXY/HTTP_PROXY，
 * 则让全局 fetch（含 youtube-transcript 请求 YouTube）走代理，便于在需代理才能访问 YouTube 的网络下使用。
 */
export async function register() {
  if (process.env.NEXT_RUNTIME !== "nodejs") return;

  const proxyUrl =
    process.env.HTTPS_PROXY ||
    process.env.https_proxy ||
    process.env.HTTP_PROXY ||
    process.env.http_proxy;

  if (!proxyUrl?.trim()) return;

  const { setGlobalDispatcher, ProxyAgent } = await import("undici");
  const url = proxyUrl.trim();
  if (!url.startsWith("http://") && !url.startsWith("https://")) {
    console.warn(
      "[youtube-transcript] HTTPS_PROXY/HTTP_PROXY 需包含协议，例如: http://127.0.0.1:7890"
    );
    return;
  }
  setGlobalDispatcher(new ProxyAgent(url));
}
