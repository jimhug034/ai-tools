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
