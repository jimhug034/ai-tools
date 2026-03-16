"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Sparkles, Languages, Copy, Check, Loader2 } from "lucide-react";
import { Button } from "@ai-tools/ui";
import { Input } from "@ai-tools/ui";
import { Card, CardContent, CardHeader, CardTitle } from "@ai-tools/ui";
import { Textarea } from "@ai-tools/ui";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@ai-tools/ui";
import { Skeleton } from "@ai-tools/ui";
import { extractVideoId, formatTimestamp, formatSrtTime } from "@ai-tools/utils";

export interface TranscriptItem {
  text: string;
  duration: number;
  offset: number;
}

export default function HomePage() {
  const t = useTranslations();
  const [url, setUrl] = useState("");
  const [error, setError] = useState("");
  const [errorHint, setErrorHint] = useState("");
  const [loading, setLoading] = useState(false);
  const [transcript, setTranscript] = useState<TranscriptItem[]>([]);
  const [summary, setSummary] = useState("");
  const [translation, setTranslation] = useState("");
  const [targetLang, setTargetLang] = useState("zh");
  const [copiedId, setCopiedId] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const videoId = extractVideoId(url);

    if (!videoId) {
      setError(t("input.invalid"));
      return;
    }

    setError("");
    setErrorHint("");
    setLoading(true);
    setTranscript([]);
    setSummary("");
    setTranslation("");

    try {
      const response = await fetch("/api/transcript", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ url }),
      });

      const data = await response.json();

      if (!response.ok) {
        setError(data.error || t("transcript.error"));
        setErrorHint(data.hint ?? "");
        setLoading(false);
        return;
      }

      setTranscript(data.items);
    } catch (err) {
      setError(err instanceof Error ? err.message : t("transcript.error"));
      setErrorHint("");
    } finally {
      setLoading(false);
    }
  };

  const handleGenerateSummary = async () => {
    const transcriptText = transcript.map((item) => item.text).join(" ");
    const response = await fetch("/api/summarize", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ transcript: transcriptText }),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.error || t("summary.error"));
    }

    setSummary(data.summary);
  };

  const handleTranslate = async () => {
    const transcriptText = transcript.map((item) => item.text).join(" ");
    const response = await fetch("/api/translate", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ text: transcriptText, targetLang }),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.error || t("translation.error"));
    }

    setTranslation(data.translation);
  };

  const handleCopy = async (id: string, text: string) => {
    await navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleExportTxt = () => {
    const text = transcript
      .map((item) => `[${formatTimestamp(item.offset)}] ${item.text}`)
      .join("\n");
    const blob = new Blob([text], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "transcript.txt";
    a.click();
    URL.revokeObjectURL(url);
  };

  const handleExportSrt = () => {
    let srtIndex = 1;
    const srt = transcript
      .map((item) => {
        const startTime = formatSrtTime(item.offset);
        const endTime = formatSrtTime(item.offset + item.duration);
        return `${srtIndex++}\n${startTime} --> ${endTime}\n${item.text}\n`;
      })
      .join("\n");

    const blob = new Blob([srt], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "transcript.srt";
    a.click();
    URL.revokeObjectURL(url);
  };

  const transcriptText = transcript.map((item) => item.text).join(" ");

  return (
    <div className="space-y-8">
      <div className="space-y-2">
        <h1 className="text-4xl font-semibold tracking-tight">{t("header.title")}</h1>
        <p className="text-sm text-neutral-500 dark:text-neutral-400 max-w-lg">
          {t("meta.description")}
        </p>
      </div>

      <form onSubmit={handleSubmit} className="space-y-3">
        <div className="flex gap-2">
          <Input
            type="url"
            placeholder={t("input.placeholder")}
            value={url}
            onChange={(e) => {
              setUrl(e.target.value);
              setError("");
              setErrorHint("");
            }}
            disabled={loading}
            className="flex-1"
          />
          <Button type="submit" disabled={loading || !url}>
            {loading ? <Loader2 className="h-4 w-4 animate-spin" /> : t("input.button")}
          </Button>
        </div>
        {error && (
          <div className="space-y-1">
            <p className="text-red-500 text-sm" role="alert">
              {error}
            </p>
            {errorHint && <p className="text-neutral-500 text-xs">{errorHint}</p>}
          </div>
        )}
      </form>

      {loading && (
        <Card>
          <CardContent className="pt-6">
            <div className="space-y-3">
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-5/6" />
              <Skeleton className="h-4 w-4/6" />
              <Skeleton className="h-4 w-full" />
              <Skeleton className="h-4 w-3/4" />
            </div>
          </CardContent>
        </Card>
      )}

      {transcript.length > 0 && (
        <Card>
          <CardHeader className="pb-3">
            <div className="flex items-center justify-between">
              <CardTitle className="text-lg">
                {t("transcript.title")}{" "}
                <span className="text-neutral-500">({transcript.length})</span>
              </CardTitle>
              <div className="flex gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handleCopy("transcript", transcriptText)}
                >
                  {copiedId === "transcript" ? (
                    <Check className="h-4 w-4 text-green-500" />
                  ) : (
                    <Copy className="h-4 w-4" />
                  )}
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportTxt}>
                  TXT
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportSrt}>
                  SRT
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="space-y-2 max-h-[500px] overflow-y-auto">
              {transcript.map((item, index) => (
                <div
                  key={index}
                  className="flex gap-3 p-2 rounded hover:bg-muted/50 transition-colors"
                >
                  <span className="text-xs text-muted-foreground font-mono shrink-0">
                    {formatTimestamp(item.offset)}
                  </span>
                  <span className="text-sm">{item.text}</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {transcript.length > 0 && (
        <div className="space-y-4">
          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-lg">{t("summary.title")}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              {!summary ? (
                <Button onClick={handleGenerateSummary} className="w-full">
                  <Sparkles className="h-4 w-4 mr-2" />
                  {t("summary.button")}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={summary}
                    onChange={(e) => setSummary(e.target.value)}
                    rows={6}
                    className="resize-none"
                  />
                  <div className="flex justify-end">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleCopy("summary", summary)}
                    >
                      {copiedId === "summary" ? (
                        <Check className="h-4 w-4 text-green-500" />
                      ) : (
                        <Copy className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-3">
              <CardTitle className="text-lg">{t("translation.title")}</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3">
              <Select value={targetLang} onValueChange={(value) => setTargetLang(value as string)}>
                <SelectTrigger>
                  <SelectValue placeholder={t("translation.selectLanguage")} />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="en">{t("translation.languages.en")}</SelectItem>
                  <SelectItem value="zh">{t("translation.languages.zh")}</SelectItem>
                  <SelectItem value="es">{t("translation.languages.es")}</SelectItem>
                  <SelectItem value="fr">{t("translation.languages.fr")}</SelectItem>
                  <SelectItem value="de">{t("translation.languages.de")}</SelectItem>
                  <SelectItem value="ja">{t("translation.languages.ja")}</SelectItem>
                  <SelectItem value="ko">{t("translation.languages.ko")}</SelectItem>
                </SelectContent>
              </Select>
              {!translation ? (
                <Button onClick={handleTranslate} className="w-full">
                  <Languages className="h-4 w-4 mr-2" />
                  {t("translation.button")}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={translation}
                    onChange={(e) => setTranslation(e.target.value)}
                    rows={6}
                    className="resize-none"
                  />
                  <div className="flex justify-end">
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleCopy("translation", translation)}
                    >
                      {copiedId === "translation" ? (
                        <Check className="h-4 w-4 text-green-500" />
                      ) : (
                        <Copy className="h-4 w-4" />
                      )}
                    </Button>
                  </div>
                </>
              )}
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
