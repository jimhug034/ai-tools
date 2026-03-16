"use client";

import { useState } from "react";
import { useTranslations } from "next-intl";
import { Copy, Check, Loader2, Download, FileText } from "lucide-react";
import { Button } from "@ai-tools/ui";
import { Input } from "@ai-tools/ui";
import { Card, CardContent, CardHeader, CardTitle } from "@ai-tools/ui";
import { Textarea } from "@ai-tools/ui";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@ai-tools/ui";
import { Skeleton } from "@ai-tools/ui";
import { extractVideoId, formatTimestamp, formatSrtTime } from "@ai-tools/utils";
import { EmptyState } from "@/components/empty-state";

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
  const [summaryError, setSummaryError] = useState("");
  const [translationError, setTranslationError] = useState("");
  const [isSummarizing, setIsSummarizing] = useState(false);
  const [isTranslating, setIsTranslating] = useState(false);

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
    setSummaryError("");
    setIsSummarizing(true);
    try {
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
    } catch (err) {
      setSummaryError(err instanceof Error ? err.message : t("summary.error"));
    } finally {
      setIsSummarizing(false);
    }
  };

  const handleTranslate = async () => {
    setTranslationError("");
    setIsTranslating(true);
    try {
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
    } catch (err) {
      setTranslationError(err instanceof Error ? err.message : t("translation.error"));
    } finally {
      setIsTranslating(false);
    }
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
    <div className="space-y-8 animate-in">
      {/* Hero Section - Centered */}
      <div className="text-center space-y-6 py-8">
        <h1 className="text-5xl font-bold tracking-tight bg-gradient-to-r from-neutral-900 to-neutral-600 dark:from-white dark:to-neutral-400 bg-clip-text text-transparent">
          {t("header.title")}
        </h1>
        <p className="text-base text-neutral-600 dark:text-neutral-400 max-w-xl mx-auto leading-relaxed">
          {t("meta.description")}
        </p>
      </div>

      {/* Input Section - Centered with beautiful styling */}
      <form onSubmit={handleSubmit} className="max-w-2xl mx-auto animate-in delay-100">
        <div className="relative group">
          <div className="absolute -inset-0.5 bg-gradient-to-r from-primary to-accent rounded-xl opacity-20 group-hover:opacity-40 transition-opacity blur"></div>
          <div className="relative flex items-center gap-2 bg-white dark:bg-neutral-900 rounded-xl p-1.5 shadow-xl shadow-neutral-200/50 dark:shadow-black/50 border border-neutral-200 dark:border-neutral-800">
            <div className="relative flex-1">
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
                className="border-0 focus-visible:ring-0 focus-visible:ring-offset-0 bg-transparent text-base h-12 px-4"
              />
            </div>
            <Button
              type="submit"
              disabled={loading || !url}
              size="lg"
              className="h-12 px-8 rounded-lg font-medium shadow-md shadow-primary/20 hover:shadow-lg hover:shadow-primary/30 transition-all"
            >
              {loading ? <Loader2 className="h-5 w-5 animate-spin" /> : t("input.button")}
            </Button>
          </div>
        </div>
        {error && (
          <div className="mt-4 space-y-1 text-center">
            <p className="text-red-500 text-sm font-medium" role="alert">
              {error}
            </p>
            {errorHint && <p className="text-neutral-500 text-xs">{errorHint}</p>}
          </div>
        )}
      </form>

      {!loading && transcript.length === 0 && !error && <EmptyState onFillExample={setUrl} />}

      {loading && (
        <Card className="overflow-hidden">
          <CardHeader className="pb-4 border-b border-neutral-100 dark:border-neutral-800">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Skeleton className="h-5 w-5 rounded" />
                <Skeleton className="h-5 w-32 rounded" />
                <Skeleton className="h-5 w-8 rounded-full" />
              </div>
              <div className="flex gap-2">
                <Skeleton className="h-8 w-8 rounded" />
                <Skeleton className="h-8 w-16 rounded" />
                <Skeleton className="h-8 w-16 rounded" />
              </div>
            </div>
          </CardHeader>
          <CardContent className="p-0">
            <div className="divide-y divide-neutral-100 dark:divide-neutral-800/50">
              {[1, 2, 3, 4, 5, 6].map((i) => (
                <div key={i} className="flex gap-3 px-5 py-3">
                  <Skeleton className="h-4 w-16 shrink-0" />
                  <Skeleton className="h-4 flex-1" />
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}

      {transcript.length > 0 && (
        <div className="space-y-4 animate-in delay-200">
          <Card className="overflow-hidden">
            <CardHeader className="pb-4 border-b border-neutral-100 dark:border-neutral-800">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <FileText className="h-5 w-5 text-primary" />
                  <CardTitle className="text-lg">{t("transcript.title")}</CardTitle>
                  <span className="text-xs font-medium px-2 py-0.5 rounded-full bg-neutral-100 dark:bg-neutral-800 text-neutral-600 dark:text-neutral-400">
                    {transcript.length}
                  </span>
                </div>
                <div className="flex gap-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => handleCopy("transcript", transcriptText)}
                    className={
                      copiedId === "transcript"
                        ? "bg-green-50 dark:bg-green-950/20 border-green-200 dark:border-green-800"
                        : ""
                    }
                  >
                    {copiedId === "transcript" ? (
                      <Check className="h-4 w-4 text-green-600 dark:text-green-400" />
                    ) : (
                      <Copy className="h-4 w-4" />
                    )}
                  </Button>
                  <Button variant="outline" size="sm" onClick={handleExportTxt} className="gap-1.5">
                    <Download className="h-3.5 w-3.5" />
                    <span>TXT</span>
                  </Button>
                  <Button variant="outline" size="sm" onClick={handleExportSrt} className="gap-1.5">
                    <Download className="h-3.5 w-3.5" />
                    <span>SRT</span>
                  </Button>
                </div>
              </div>
            </CardHeader>
            <CardContent className="p-0">
              <div className="max-h-[500px] overflow-y-auto custom-scrollbar">
                {transcript.map((item, index) => (
                  <div
                    key={index}
                    className="flex gap-3 px-5 py-3 hover:bg-neutral-50 dark:hover:bg-neutral-900/50 transition-colors border-b border-neutral-100 dark:border-neutral-800/50 last:border-0 group"
                  >
                    <span className="text-xs text-neutral-400 dark:text-neutral-500 font-mono shrink-0 tabular-nums">
                      {formatTimestamp(item.offset)}
                    </span>
                    <span className="text-sm text-neutral-700 dark:text-neutral-300 leading-relaxed">
                      {item.text}
                    </span>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </div>
      )}

      {transcript.length > 0 && (
        <div className="grid md:grid-cols-2 gap-4">
          <Card className="overflow-hidden">
            <CardHeader className="pb-4 border-b border-neutral-100 dark:border-neutral-800">
              <CardTitle className="text-lg flex items-center gap-2">
                <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-violet-500/10 to-purple-500/10 flex items-center justify-center">
                  <span className="text-lg">✨</span>
                </div>
                {t("summary.title")}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4 pt-4">
              {!summary ? (
                <>
                  <Button
                    onClick={handleGenerateSummary}
                    disabled={isSummarizing}
                    className="w-full"
                  >
                    {isSummarizing ? (
                      <>
                        <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        Generating...
                      </>
                    ) : (
                      t("summary.button")
                    )}
                  </Button>
                  {summaryError && (
                    <p className="text-red-500 text-sm text-center" role="alert">
                      {summaryError}
                    </p>
                  )}
                </>
              ) : (
                <>
                  <Textarea
                    value={summary}
                    onChange={(e) => setSummary(e.target.value)}
                    rows={8}
                    className="resize-none"
                  />
                  <div className="flex justify-between items-center">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setSummary("")}
                      className="text-neutral-500 hover:text-neutral-700 dark:text-neutral-400 dark:hover:text-neutral-200"
                    >
                      Clear
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleCopy("summary", summary)}
                      className={
                        copiedId === "summary"
                          ? "bg-green-50 dark:bg-green-950/20 border-green-200 dark:border-green-800"
                          : ""
                      }
                    >
                      {copiedId === "summary" ? (
                        <>
                          <Check className="h-4 w-4 text-green-600 dark:text-green-400" />
                          <span>Copied</span>
                        </>
                      ) : (
                        <>
                          <Copy className="h-4 w-4" />
                          <span>Copy</span>
                        </>
                      )}
                    </Button>
                  </div>
                </>
              )}
            </CardContent>
          </Card>

          <Card className="overflow-hidden">
            <CardHeader className="pb-4 border-b border-neutral-100 dark:border-neutral-800">
              <CardTitle className="text-lg flex items-center gap-2">
                <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-blue-500/10 to-sky-500/10 flex items-center justify-center">
                  <span className="text-lg">🌐</span>
                </div>
                {t("translation.title")}
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4 pt-4">
              <Select value={targetLang} onValueChange={(value) => setTargetLang(value as string)}>
                <SelectTrigger className="w-full">
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
                <>
                  <Button onClick={handleTranslate} disabled={isTranslating} className="w-full">
                    {isTranslating ? (
                      <>
                        <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                        Translating...
                      </>
                    ) : (
                      t("translation.button")
                    )}
                  </Button>
                  {translationError && (
                    <p className="text-red-500 text-sm text-center" role="alert">
                      {translationError}
                    </p>
                  )}
                </>
              ) : (
                <>
                  <Textarea
                    value={translation}
                    onChange={(e) => setTranslation(e.target.value)}
                    rows={6}
                    className="resize-none"
                  />
                  <div className="flex justify-between items-center">
                    <Button
                      variant="ghost"
                      size="sm"
                      onClick={() => setTranslation("")}
                      className="text-neutral-500 hover:text-neutral-700 dark:text-neutral-400 dark:hover:text-neutral-200"
                    >
                      Clear
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      onClick={() => handleCopy("translation", translation)}
                      className={
                        copiedId === "translation"
                          ? "bg-green-50 dark:bg-green-950/20 border-green-200 dark:border-green-800"
                          : ""
                      }
                    >
                      {copiedId === "translation" ? (
                        <>
                          <Check className="h-4 w-4 text-green-600 dark:text-green-400" />
                          <span>Copied</span>
                        </>
                      ) : (
                        <>
                          <Copy className="h-4 w-4" />
                          <span>Copy</span>
                        </>
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
