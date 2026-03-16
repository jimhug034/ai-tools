import { Youtube } from "lucide-react";

const EXAMPLE_URL = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

interface EmptyStateProps {
  onFillExample: (url: string) => void;
}

export function EmptyState({ onFillExample }: EmptyStateProps) {
  return (
    <div className="text-center py-16 px-4 animate-in">
      {/* Icon with gradient background */}
      <div className="inline-flex items-center justify-center w-20 h-20 rounded-2xl bg-gradient-to-br from-neutral-100 to-neutral-200 dark:from-neutral-800 dark:to-neutral-900 mb-6 ring-1 ring-neutral-200/50 dark:ring-neutral-700/50">
        <Youtube className="w-10 h-10 text-neutral-700 dark:text-neutral-300" />
      </div>

      {/* Text content */}
      <h3 className="text-xl font-semibold mb-3 text-neutral-900 dark:text-neutral-100">
        Ready to extract subtitles
      </h3>
      <p className="text-neutral-600 dark:text-neutral-400 text-base mb-8 max-w-md mx-auto leading-relaxed">
        Paste a YouTube video URL above to instantly get the transcript, summary, and translation.
      </p>

      {/* Example button with better styling */}
      <button
        onClick={() => onFillExample(EXAMPLE_URL)}
        className="group inline-flex items-center gap-2 px-5 py-2.5 rounded-lg bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors text-sm font-medium"
      >
        <span className="text-neutral-700 dark:text-neutral-300">Try an example</span>
        <Youtube className="w-4 h-4 text-neutral-500 dark:text-neutral-400 group-hover:text-primary transition-colors" />
      </button>
    </div>
  );
}
