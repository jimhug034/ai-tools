import { cn } from "../lib/utils";

const Skeleton = ({ className, ...props }: React.HTMLAttributes<HTMLDivElement>) => (
  <div
    className={cn("animate-pulse rounded-md bg-neutral-100 dark:bg-neutral-800", className)}
    {...props}
  />
);

Skeleton.displayName = "Skeleton";

export { Skeleton };
