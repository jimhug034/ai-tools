"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Youtube, FileText, Sparkles, Languages, Copy, Download, Loader2 } from "lucide-react";
import { Button } from "@ai-tools/ui";
import { Input } from "@ai-tools/ui";
import { Card, CardContent, CardHeader, CardTitle } from "@ai-tools/ui";
import { Textarea } from "@ai-tools/ui";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@ai-tools/ui";
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
  const [copied, setCopied] = useState(false);

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

  const handleCopy = async (text: string) => {
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
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
      <div className="text-center space-y-4">
        <h2 className="text-3xl font-bold">{t("header.title")}</h2>
        <p className="text-muted-foreground max-w-2xl mx-auto">{t("meta.description")}</p>
      </div>

      <form onSubmit={handleSubmit} className="w-full max-w-2xl mx-auto">
        <div className="flex gap-2">
          <div className="relative flex-1">
            <Youtube className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
            <Input
              type="url"
              placeholder={t("input.placeholder")}
              value={url}
              onChange={(e) => {
                setUrl(e.target.value);
                setError("");
                setErrorHint("");
              }}
              className="pl-10"
              disabled={loading}
            />
          </div>
          <Button type="submit" disabled={loading || !url}>
            {loading ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                {t("input.validating")}
              </>
            ) : (
              t("input.button")
            )}
          </Button>
        </div>
        {error && (
          <div className="mt-2 space-y-1">
            <p className="text-destructive text-sm">{error}</p>
            {errorHint && (
              <p className="text-muted-foreground text-xs max-w-2xl">{errorHint}</p>
            )}
          </div>
        )}
      </form>

      {transcript.length > 0 && (
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2">
                <FileText className="h-5 w-5" />
                {t("transcript.title")} ({transcript.length} lines)
              </CardTitle>
              <div className="flex gap-2">
                <Button variant="outline" size="sm" onClick={() => handleCopy(transcriptText)}>
                  <Copy className="h-4 w-4 mr-1" />
                  {copied ? "Copied!" : t("transcript.copy")}
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportTxt}>
                  <Download className="h-4 w-4 mr-1" />
                  TXT
                </Button>
                <Button variant="outline" size="sm" onClick={handleExportSrt}>
                  <Download className="h-4 w-4 mr-1" />
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
        <div className="grid md:grid-cols-2 gap-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Sparkles className="h-5 w-5" />
                {t("summary.title")}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              {!summary ? (
                <Button onClick={handleGenerateSummary} className="w-full">
                  {t("summary.button")}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={summary}
                    onChange={(e) => setSummary(e.target.value)}
                    rows={8}
                    className="resize-none"
                  />
                  <Button variant="outline" onClick={() => handleCopy(summary)}>
                    <Copy className="h-4 w-4 mr-1" />
                    {copied ? "Copied!" : t("summary.copy")}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Languages className="h-5 w-5" />
                {t("translation.title")}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
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
                  {t("translation.button")}
                </Button>
              ) : (
                <>
                  <Textarea
                    value={translation}
                    onChange={(e) => setTranslation(e.target.value)}
                    rows={8}
                    className="resize-none"
                  />
                  <Button variant="outline" onClick={() => handleCopy(translation)}>
                    <Copy className="h-4 w-4 mr-1" />
                    {copied ? "Copied!" : t("translation.copy")}
                  </Button>
                </>
              )}
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
