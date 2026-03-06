import * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../lib/utils";

const badgeVariants = cva(
  "inline-flex items-center rounded-md px-2 py-0.5 text-xs font-semibold transition-colors",
  {
    variants: {
      variant: {
        success: "bg-success-bg text-[#1A7F54]",
        warning: "bg-warning-bg text-[#9A6F00]",
        error: "bg-error-bg text-[#B92B2B]",
        info: "bg-info-bg text-[#2563A8]",
        neutral: "bg-surface-alt text-text-secondary",
        primary: "bg-primary-light text-primary-dark",
      },
    },
    defaultVariants: {
      variant: "neutral",
    },
  }
);

export interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

function Badge({ className, variant, ...props }: BadgeProps) {
  return (
    <div className={cn(badgeVariants({ variant }), className)} {...props} />
  );
}

export { Badge, badgeVariants };
