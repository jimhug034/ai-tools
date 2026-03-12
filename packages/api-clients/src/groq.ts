import Groq from "groq-sdk";

const groq = new Groq({
  apiKey: process.env.GROQ_API_KEY || "",
});

export async function generateSummary(transcript: string): Promise<string> {
  const response = await groq.chat.completions.create({
    messages: [
      {
        role: "system",
        content:
          "You are a helpful assistant that summarizes video transcripts. Provide a concise summary in 3-5 bullet points.",
      },
      {
        role: "user",
        content: `Please summarize this video transcript:\n\n${transcript}`,
      },
    ],
    model: "llama-3.3-70b-versatile",
    temperature: 0.5,
    max_tokens: 500,
  });

  return response.choices[0]?.message?.content || "";
}

export async function translateText(text: string, targetLang: string): Promise<string> {
  const langNames: Record<string, string> = {
    en: "English",
    zh: "Chinese",
    es: "Spanish",
    fr: "French",
    de: "German",
    ja: "Japanese",
    ko: "Korean",
  };

  const response = await groq.chat.completions.create({
    messages: [
      {
        role: "system",
        content: `You are a professional translator. Translate the given text to ${langNames[targetLang] || targetLang}. Preserve the original formatting and structure.`,
      },
      {
        role: "user",
        content: text,
      },
    ],
    model: "llama-3.3-70b-versatile",
    temperature: 0.3,
    max_tokens: 2000,
  });

  return response.choices[0]?.message?.content || "";
}
