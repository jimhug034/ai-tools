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
