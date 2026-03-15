import { Innertube, UniversalCache } from "youtubei.js";

export interface TranscriptItem {
  text: string;
  duration: number;
  offset: number;
  lang?: string;
}

export interface TranscriptResult {
  videoId: string;
  items: TranscriptItem[];
}

/**
 * 备用方案：通过 youtubei.js 获取字幕
 * 注意：此方法可能不稳定，推荐使用 Rust + yt-dlp 服务
 *
 * @deprecated 使用 Rust 服务代替
 */
export async function getTranscript(videoId: string): Promise<TranscriptResult> {
  throw new Error(
    "Direct YouTube API access is deprecated. Please use the Rust transcript service instead. Run: pnpm dev:rust"
  );
}
