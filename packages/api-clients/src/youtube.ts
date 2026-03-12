import { YoutubeTranscript } from "youtube-transcript";

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

export async function getTranscript(
  videoId: string,
): Promise<TranscriptResult> {
  try {
    const transcript = await YoutubeTranscript.fetchTranscript(videoId);

    return {
      videoId,
      items: transcript.map((item) => ({
        text: item.text,
        duration: item.duration,
        offset: item.offset,
        lang: item.lang,
      })),
    };
  } catch (error) {
    throw new Error(
      `Failed to fetch transcript: ${error instanceof Error ? error.message : "Unknown error"}`,
    );
  }
}
