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

async function createProxyFetch(): Promise<
  ((input: RequestInfo | URL, init?: RequestInit) => Promise<Response>) | undefined
> {
  const proxyUrl =
    process.env.HTTPS_PROXY ||
    process.env.https_proxy ||
    process.env.HTTP_PROXY ||
    process.env.http_proxy;
  const url = proxyUrl?.trim();
  if (!url || (!url.startsWith("http://") && !url.startsWith("https://")))
    return undefined;

  const { fetch: undiciFetch, ProxyAgent } = await import("undici");
  const agent = new ProxyAgent(url);
  const proxyFetch = async (input: RequestInfo | URL, init?: RequestInit) => {
    const isRequest = input && typeof input === "object" && "url" in input && typeof (input as Request).url === "string";
    const url = typeof input === "string" ? input : input instanceof URL ? input.href : (input as Request).url;
    const base = isRequest ? { method: (input as Request).method, headers: (input as Request).headers, body: (input as Request).body } : {};
    const opts = { ...base, ...init, dispatcher: agent };
    const res = await undiciFetch(url, opts as Parameters<typeof undiciFetch>[1]);
    return res as unknown as Response;
  };
  return proxyFetch as (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;
}

function decodeHtmlEntities(text: string): string {
  return text
    .replace(/&#39;/g, "'")
    .replace(/&quot;/g, '"')
    .replace(/&amp;/g, "&")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&nbsp;/g, " ")
    .replace(/&#(\d+);/g, (_, num) => String.fromCharCode(Number.parseInt(num, 10)));
}

interface ParsedSegment {
  startMs: number;
  durationMs: number;
  text: string;
}

function parsePTagFormat(xml: string): ParsedSegment[] {
  const segments: ParsedSegment[] = [];
  const pTagRegex = /<p t="(\d+)" d="(\d+)"[^>]*>([\s\S]*?)<\/p>/g;
  let match = pTagRegex.exec(xml);
  while (match !== null) {
    const [, startMsStr, durationMsStr, rawText] = match;
    if (startMsStr && durationMsStr && rawText) {
      const text = decodeHtmlEntities(rawText.replace(/<[^>]+>/g, "")).trim();
      if (text) {
        segments.push({
          durationMs: Number.parseInt(durationMsStr, 10),
          startMs: Number.parseInt(startMsStr, 10),
          text,
        });
      }
    }
    match = pTagRegex.exec(xml);
  }
  return segments;
}

function parseTextTagFormat(xml: string): ParsedSegment[] {
  const segments: ParsedSegment[] = [];
  const textTagRegex = /<text start="([^"]+)" dur="([^"]+)"[^>]*>([\s\S]*?)<\/text>/g;
  let match = textTagRegex.exec(xml);
  while (match !== null) {
    const [, startStr, durStr, rawText] = match;
    if (startStr && durStr && rawText) {
      const text = decodeHtmlEntities(rawText.replace(/<[^>]+>/g, "")).trim();
      if (text) {
        segments.push({
          durationMs: Math.round(Number.parseFloat(durStr) * 1000),
          startMs: Math.round(Number.parseFloat(startStr) * 1000),
          text,
        });
      }
    }
    match = textTagRegex.exec(xml);
  }
  return segments;
}

function parseTimedTextXml(xml: string): ParsedSegment[] {
  const pSegments = parsePTagFormat(xml);
  if (pSegments.length > 0) return pSegments;
  return parseTextTagFormat(xml);
}

type CaptionTrack = { base_url?: string; language_code?: string; kind?: string };

function getCaptionTracks(info: unknown): CaptionTrack[] | undefined {
  const o = info as Record<string, unknown>;
  const captions = o?.captions as { caption_tracks?: CaptionTrack[] } | undefined;
  if (captions?.caption_tracks?.length) return captions.caption_tracks;
  const direct = o?.caption_tracks as CaptionTrack[] | undefined;
  if (Array.isArray(direct) && direct.length) return direct;
  return undefined;
}

async function getTranscriptViaCaptionTracks(
  videoId: string,
  youtube: Awaited<ReturnType<typeof Innertube.create>>,
  fetchFn: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
): Promise<TranscriptResult> {
  const info = await youtube.getBasicInfo(videoId);
  const tracks = getCaptionTracks(info);
  if (!tracks?.length) {
    throw new Error("No caption tracks found for this video");
  }
  const preferred =
    tracks.find((t) => t.language_code === "en" && t.kind !== "asr") ||
    tracks.find((t) => t.language_code?.startsWith("en")) ||
    tracks[0];
  const baseUrl = preferred?.base_url;
  if (!baseUrl) {
    throw new Error("No valid caption track URL found");
  }
  const res = await fetchFn(baseUrl, {
    headers: {
      "Accept-Language": "en-US,en;q=0.9",
      "User-Agent":
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    },
  });
  if (!res.ok) {
    throw new Error(`Failed to fetch caption XML: ${res.status}`);
  }
  const xml = await res.text();
  const segments = parseTimedTextXml(xml);
  if (segments.length === 0) {
    throw new Error("Failed to parse any transcript segments from caption XML");
  }
  const items: TranscriptItem[] = segments.map((s) => ({
    text: s.text,
    duration: s.durationMs / 1000,
    offset: s.startMs / 1000,
  }));
  return { videoId, items };
}

/**
 * 仅通过 getBasicInfo + 字幕轨 + timedtext XML 获取字幕。
 * 不调用 getInfo/getTranscript，避免 youtubei.js 解析错误和 get_transcript 400。
 */
export async function getTranscript(videoId: string): Promise<TranscriptResult> {
  try {
    const proxyFetch = await createProxyFetch();
    const youtube = await Innertube.create({
      cache: new UniversalCache(false),
      ...(proxyFetch && { fetch: proxyFetch }),
    });

    const fetchFn = proxyFetch ?? ((input: RequestInfo | URL, init?: RequestInit) => fetch(input, init));

    return getTranscriptViaCaptionTracks(videoId, youtube, fetchFn);
  } catch (error) {
    const message = error instanceof Error ? error.message : "Unknown error";
    const err = new Error(`Failed to fetch transcript: ${message}`);
    if (error instanceof Error && error.cause) err.cause = error.cause;
    throw err;
  }
}
