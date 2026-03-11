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
