import { NextRequest, NextResponse } from "next/server";
import { translateText } from "@ai-tools/api-clients";

export async function POST(request: NextRequest) {
  try {
    const { text, targetLang } = await request.json();

    if (!text || typeof text !== "string") {
      return NextResponse.json({ error: "Text is required and must be a string" }, { status: 400 });
    }

    if (!targetLang || typeof targetLang !== "string") {
      return NextResponse.json({ error: "targetLang is required" }, { status: 400 });
    }

    const translation = await translateText(text, targetLang);

    return NextResponse.json({ translation });
  } catch (error) {
    console.error("Translation error:", error);

    return NextResponse.json(
      {
        error: "Failed to translate",
        details: error instanceof Error ? error.message : "Unknown error",
      },
      { status: 500 }
    );
  }
}

export const runtime = "edge";
