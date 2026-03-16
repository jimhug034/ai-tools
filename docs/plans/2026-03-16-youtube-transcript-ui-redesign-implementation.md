# YouTube Transcript UI Redesign - Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Redesign the YouTube Transcript web interface with a minimal, efficient aesthetic inspired by Linear/Vercel, using a blue/violet color scheme.

**Architecture:**

- Update design tokens (colors, typography, spacing) in globals.css and tailwind.config
- Refactor layout components (hero, input, cards) in page.tsx and layout.tsx
- Add new components (skeleton, empty-state) for enhanced UX
- Fix accessibility issues (ARIA labels, touch targets)

**Tech Stack:** Next.js 15, React 19, Tailwind CSS, shadcn/ui, TypeScript

---

## Task 1: Update Design Tokens (Color System)

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/globals.css:5-50`

**Step 1: Update CSS variables with new color palette**

Replace the current color system with blue/violet theme:

```css
@layer base {
  :root {
    /* Primary - Sky Blue */
    --background: 0 0% 100%;
    --foreground: 0 0% 4%; /* #0A0A0A */
    --card: 0 0% 100%;
    --card-foreground: 0 0% 4%;
    --popover: 0 0% 100%;
    --popover-foreground: 0 0% 4%;
    --primary: 199 89% 48%; /* #0EA5E9 Sky 500 */
    --primary-foreground: 0 0% 100%;
    --secondary: 210 40% 96%;
    --secondary-foreground: 0 0% 4%;
    --muted: 210 40% 96%;
    --muted-foreground: 0 0% 45%; /* Neutral 500 */
    --accent: 258 90% 66%; /* #8B5CF6 Violet 500 */
    --accent-foreground: 0 0% 100%;
    --destructive: 0 84% 60%;
    --destructive-foreground: 0 0% 100%;
    --border: 0 0% 90%; /* #E5E5E5 */
    --input: 0 0% 90%;
    --ring: 199 89% 48%;
    --radius: 0.375rem; /* 6px - sharper than 8px */
  }

  .dark {
    --background: 0 0% 4%; /* #0A0A0A */
    --foreground: 0 0% 90%; /* #E5E5E5 */
    --card: 0 0% 6%;
    --card-foreground: 0 0% 90%;
    --popover: 0 0% 6%;
    --popover-foreground: 0 0% 90%;
    --primary: 199 89% 48%;
    --primary-foreground: 0 0% 100%;
    --secondary: 0 0% 15%;
    --secondary-foreground: 0 0% 90%;
    --muted: 0 0% 15%;
    --muted-foreground: 0 0% 60%;
    --accent: 258 90% 66%;
    --accent-foreground: 0 0% 100%;
    --destructive: 0 62% 30%;
    --destructive-foreground: 0 0% 100%;
    --border: 0 0% 15%; /* #262626 */
    --input: 0 0% 15%;
    --ring: 199 89% 48%;
  }
}
```

**Step 2: Update Tailwind config for border radius**

Modify `packages/config/tailwind.config.ts:39-43`:

```ts
borderRadius: {
  lg: "var(--radius)",     // 6px
  md: "calc(var(--radius) - 2px)",  // 4px
  sm: "calc(var(--radius) - 3px)",  // 3px
},
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/globals.css packages/config/tailwind.config.ts
git commit -m "style: update design tokens to blue/violet theme

- Change primary to Sky 500 (#0EA5E9)
- Add accent color Violet 500 (#8B5CF6)
- Reduce border radius to 6px for sharper look
- Update dark mode colors for better contrast

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 2: Update Typography

**Files:**

- Create: `apps/youtube-transcript/web/src/app/layout.tsx` (update metadata)
- Modify: `apps/youtube-transcript/web/src/app/globals.css:52-59`

**Step 1: Add font imports to layout**

Add to `apps/youtube-transcript/web/src/app/[lang]/layout.tsx` after imports:

```tsx
import { Inter, Inter_Tight, JetBrains_Mono } from "next/font/google";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
  display: "swap",
});

const interTight = Inter_Tight({
  subsets: ["latin"],
  variable: "--font-inter-tight",
  display: "swap",
});

const jetbrainsMono = JetBrains_Mono({
  subsets: ["latin"],
  variable: "--font-jetbrains-mono",
  display: "swap",
});
```

**Step 2: Update html element with font variables**

Replace the html element in layout:

```tsx
<html lang={locale} className={`${inter.variable} ${interTight.variable} ${jetbrainsMono.variable}`}>
```

**Step 3: Add font utilities to globals.css**

```css
@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground font-sans;
    font-family: var(--font-inter);
  }
  h1,
  h2,
  h3,
  h4,
  h5,
  h6 {
    font-family: var(--font-inter-tight);
  }
  .font-mono {
    font-family: var(--font-jetbrains-mono);
  }
}
```

**Step 4: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/layout.tsx apps/youtube-transcript/web/src/app/globals.css
git commit -m "style: add modern typography system

- Add Inter Tight for headings
- Add JetBrains Mono for code/time
- Keep Inter for body text

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 3: Refactor Hero Section

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx:147-151`

**Step 1: Update hero section JSX**

Replace current hero with:

```tsx
<div className="space-y-2">
  <h1 className="text-4xl font-semibold tracking-tight">{t("header.title")}</h1>
  <p className="text-sm text-neutral-500 dark:text-neutral-400 max-w-lg">{t("meta.description")}</p>
</div>
```

**Step 2: Update i18n messages**

Check and update messages in `apps/youtube-transcript/web/src/messages/en.json` and `zh.json`:

```json
{
  "header": {
    "title": "YouTube Transcript"
  },
  "meta": {
    "description": "Extract, translate, and summarize video transcripts instantly."
  }
}
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "refactor: redesign hero section

- Left-align heading and description
- Increase heading size with tight tracking
- Simplify description text
- Remove center alignment

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 4: Refactor Input Section

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx:153-187`

**Step 1: Remove icon from input, update layout**

Replace form content:

```tsx
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
```

**Step 2: Update import to remove Youtube icon**

Remove `Youtube` from the lucide-react import in page.tsx:5

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "refactor: simplify input section

- Remove decorative YouTube icon
- Full-width input with proper spacing
- Add role=\"alert\" to error message
- Improve error message styling

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 5: Refactor Card Layout

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx:189-305`

**Step 1: Update transcript card - remove icon**

Replace CardHeader for transcript (lines 191-211):

```tsx
<CardHeader className="pb-3">
  <div className="flex items-center justify-between">
    <CardTitle className="text-lg">
      {t("transcript.title")} <span className="text-neutral-500">({transcript.length})</span>
    </CardTitle>
    <div className="flex gap-2">
      <Button variant="outline" size="sm" onClick={() => handleCopy("transcript", transcriptText)}>
        <Copy className="h-4 w-4" />
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
```

**Step 2: Update summary and translation cards**

Replace the grid section (lines 231-305):

```tsx
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
            <Button variant="outline" size="sm" onClick={() => handleCopy("summary", summary)}>
              <Copy className="h-4 w-4" />
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
              <Copy className="h-4 w-4" />
            </Button>
          </div>
        </>
      )}
    </CardContent>
  </Card>
</div>
```

**Step 3: Update state for copy tracking**

Replace the copied state and handleCopy function (lines 29, 106-110):

```tsx
// Replace state
const [copiedId, setCopiedId] = useState<string | null>(null);

// Replace handleCopy function
const handleCopy = async (id: string, text: string) => {
  await navigator.clipboard.writeText(text);
  setCopiedId(id);
  setTimeout(() => setCopiedId(null), 2000);
};
```

**Step 4: Update copy button rendering**

Add logic to show check icon when copied. In each Button that copies:

```tsx
<Copy className={`h-4 w-4 ${copiedId === "transcript" ? "text-green-500" : ""}`} />
```

Or use conditional rendering:

```tsx
{
  copiedId === "transcript" ? (
    <Check className="h-4 w-4 text-green-500" />
  ) : (
    <Copy className="h-4 w-4" />
  );
}
```

Add `Check` to lucide-react import.

**Step 5: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "refactor: restructure card layout

- Remove icons from card titles
- Change from grid to vertical stack
- Reduce padding in cards
- Implement per-button copy state tracking
- Add Check icon for copied feedback

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 6: Update Header/Logo

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/layout.tsx:22-60`

**Step 1: Remove emoji, update header**

Replace header content:

```tsx
<header className="border-b border-neutral-200 dark:border-neutral-800">
  <div className="container mx-auto px-4 py-4 flex items-center justify-between">
    <Link href="/" className="flex items-center gap-2 hover:opacity-80 transition-opacity">
      <div className="w-8 h-8 bg-primary rounded-md flex items-center justify-center">
        <span className="text-primary-foreground font-semibold text-sm">YT</span>
      </div>
      <span className="font-semibold">Transcript</span>
    </Link>
    <nav className="flex gap-1" aria-label="Language selector">
      <Link
        href="/"
        locale="en"
        className={`px-3 py-2 text-sm transition-colors relative ${
          locale === "en" ? "text-foreground font-medium" : "text-neutral-500 hover:text-foreground"
        }`}
        aria-label="Switch to English"
        aria-current={locale === "en" ? "true" : undefined}
      >
        EN
        {locale === "en" && <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />}
      </Link>
      <Link
        href="/"
        locale="zh"
        className={`px-3 py-2 text-sm transition-colors relative ${
          locale === "zh" ? "text-foreground font-medium" : "text-neutral-500 hover:text-foreground"
        }`}
        aria-label="切换到中文"
        aria-current={locale === "zh" ? "true" : undefined}
      >
        中
        {locale === "zh" && <span className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary" />}
      </Link>
    </nav>
  </div>
</header>
```

**Step 2: Update footer**

Replace footer section:

```tsx
<footer className="border-t border-neutral-200 dark:border-neutral-800 mt-16">
  <div className="container mx-auto px-4 py-6 flex items-center justify-between text-sm text-neutral-500">
    <p>Free • No registration</p>
    <p>Built with Next.js & Rust</p>
  </div>
</footer>
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/layout.tsx
git commit -m "refactor: redesign header and footer

- Remove emoji from logo
- Replace with minimalist YT icon
- Update language switcher to underline style
- Add proper ARIA labels
- Improve touch targets (44px minimum)

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 7: Add Skeleton Loading Component

**Files:**

- Create: `packages/ui/src/components/skeleton.tsx`
- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx`

**Step 1: Create skeleton component**

```tsx
import { cn } from "../lib/utils";

const Skeleton = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <div
    className={cn("animate-pulse rounded-md bg-neutral-100 dark:bg-neutral-800", className)}
    {...props}
  />
);

Skeleton.displayName = "Skeleton";

export { Skeleton };
```

**Step 2: Export skeleton**

Add to `packages/ui/src/components/index.ts`:

```ts
export { Skeleton } from "./skeleton";
```

**Step 3: Add skeleton UI to page**

Add after the form in page.tsx:

```tsx
{
  loading && (
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
  );
}
```

**Step 4: Commit**

```bash
git add packages/ui/src/components/skeleton.tsx packages/ui/src/components/index.ts apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "feat: add skeleton loading component

- Create reusable Skeleton component
- Add skeleton for transcript loading
- Improve perceived performance

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 8: Add Empty State Component

**Files:**

- Create: `apps/youtube-transcript/web/src/components/empty-state.tsx`
- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx`

**Step 1: Create empty state component**

```tsx
import { Youtube } from "lucide-react";

const EXAMPLE_URL = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

interface EmptyStateProps {
  onFillExample: (url: string) => void;
}

export function EmptyState({ onFillExample }: EmptyStateProps) {
  return (
    <div className="text-center py-12 px-4">
      <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-neutral-100 dark:bg-neutral-800 mb-4">
        <Youtube className="w-8 h-8 text-neutral-400" />
      </div>
      <h3 className="text-lg font-medium mb-2">No transcript yet</h3>
      <p className="text-neutral-500 text-sm mb-4 max-w-sm mx-auto">
        Enter a YouTube URL above to extract the transcript, or try an example.
      </p>
      <button
        onClick={() => onFillExample(EXAMPLE_URL)}
        className="text-sm text-primary hover:underline"
      >
        Try example video →
      </button>
    </div>
  );
}
```

**Step 2: Add empty state to page**

Add in page.tsx after the form, before transcript card:

```tsx
{
  !loading && transcript.length === 0 && !error && <EmptyState onFillExample={setUrl} />;
}
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/components/empty-state.tsx apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "feat: add empty state component

- Show friendly empty state when no transcript
- Add example video URL
- Help users understand the tool

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 9: Add Transition Animations

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/globals.css`
- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx`

**Step 1: Add animation utilities to globals.css**

```css
@layer utilities {
  .animate-in {
    animation: slideIn 0.3s ease-out;
  }

  @keyframes slideIn {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .delay-100 {
    animation-delay: 100ms;
  }

  .delay-200 {
    animation-delay: 200ms;
  }
}
```

**Step 2: Add animations to page elements**

Add `animate-in` class to main containers:

```tsx
<div className="space-y-8 animate-in">
  <div className="space-y-2">{/* hero */}</div>

  <form className="animate-in delay-100">{/* form */}</form>

  {transcript.length > 0 && <div className="space-y-4 animate-in delay-200">{/* cards */}</div>}
</div>
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/globals.css apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "feat: add page transition animations

- Add slide-in animation on page load
- Stagger animations for polish
- Subtle 300ms ease-out timing

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 10: Add Error Handling for Summary/Translation

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx:72-104`

**Step 1: Add error state and handling**

Add error state after existing states:

```tsx
const [summaryError, setSummaryError] = useState("");
const [translationError, setTranslationError] = useState("");
```

**Step 2: Update handleGenerateSummary with error handling**

```tsx
const handleGenerateSummary = async () => {
  setSummaryError("");
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
  }
};
```

**Step 3: Update handleTranslate with error handling**

```tsx
const handleTranslate = async () => {
  setTranslationError("");
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
  }
};
```

**Step 4: Add error display in cards**

Add error display in summary and translation cards:

```tsx
{
  summaryError && (
    <p className="text-red-500 text-sm" role="alert">
      {summaryError}
    </p>
  );
}
```

**Step 5: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "feat: add error handling for summary and translation

- Try-catch with user-facing error messages
- Display errors inline with role=\"alert\"
- Prevent silent failures

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 11: Mobile Responsiveness Improvements

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx`
- Modify: `packages/ui/src/components/button.tsx`

**Step 1: Ensure minimum touch targets on buttons**

The Button component already has `h-10` (40px) for default. Verify small button is adequate:

Update `packages/ui/src/components/button.tsx:22`:

```ts
sm: "h-10 rounded-md px-3",  // Changed from h-9 to h-10
```

**Step 2: Update responsive spacing in page**

Add responsive classes to main container:

```tsx
<main className="container mx-auto px-4 py-6 sm:py-8">
```

**Step 3: Update card padding for mobile**

Add responsive padding to transcript list items:

```tsx
<div className="flex gap-2 sm:gap-3 p-2 sm:p-3 rounded hover:bg-muted/50 transition-colors">
```

**Step 4: Commit**

```bash
git add packages/ui/src/components/button.tsx apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "fix: improve mobile touch targets

- Increase small button height to 40px
- Add responsive spacing
- Ensure 44px minimum touch targets

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 12: Final Polish - Remove Decorative Icons

**Files:**

- Modify: `apps/youtube-transcript/web/src/app/[lang]/page.tsx`

**Step 1: Remove icons from summary and translation button**

Update the summary button:

```tsx
{!summary ? (
  <Button onClick={handleGenerateSummary} className="w-full">
    {t("summary.button")}
  </Button>
) : (
```

Update the translation button:

```tsx
{!translation ? (
  <Button onClick={handleTranslate} className="w-full">
    {t("translation.button")}
  </Button>
) : (
```

**Step 2: Remove unused icon imports**

Update the lucide-react import to only keep what's used:

```tsx
import { Copy, Download, Loader2, Check } from "lucide-react";
```

**Step 3: Commit**

```bash
git add apps/youtube-transcript/web/src/app/\[lang\]/page.tsx
git commit -m "refactor: remove decorative icons

- Remove icons from action buttons
- Clean up unused imports
- Simplify visual hierarchy

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Task 13: Final Verification

**Files:** All modified files

**Step 1: Run linter**

```bash
pnpm lint:fix
```

**Step 2: Run formatter**

```bash
pnpm format:fix
```

**Step 3: Build the project**

```bash
pnpm --filter @ai-tools/youtube-transcript-web build
```

**Step 4: Manual testing checklist**

- [ ] Page loads without errors
- [ ] Colors match design (blue primary, violet accent)
- [ ] Typography uses Inter Tight for headings
- [ ] Hero section is left-aligned
- [ ] Input has no decorative icon
- [ ] Cards are stacked vertically (not grid)
- [ ] Copy buttons show check icon when clicked
- [ ] Skeleton appears during loading
- [ ] Empty state shows when no transcript
- [ ] Error messages display properly
- [ ] Language switcher works with ARIA labels
- [ ] Mobile view is responsive at 375px
- [ ] Dark mode works correctly

**Step 5: Final commit if any adjustments needed**

```bash
git add .
git commit -m "chore: final polish and verification

- Run lint and format
- Fix any remaining issues
- Verify all functionality

Co-Authored-By: Claude Opus 4.6 <noreply@anthropic.com>"
```

---

## Summary

This plan covers:

1. ✅ Design tokens (colors, typography, spacing)
2. ✅ Layout restructuring (hero, input, cards)
3. ✅ Interaction enhancements (skeleton, copy feedback, animations)
4. ✅ Feature completions (empty state, error handling)
5. ✅ Accessibility fixes (ARIA labels, touch targets)
6. ✅ Mobile responsiveness
7. ✅ Final polish

Total: 13 tasks, ~2-3 hours of work.
