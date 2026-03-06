import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";
import { cn } from "../lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg font-medium transition-all duration-150 ease-out focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-40 active:scale-[0.97] [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  {
    variants: {
      variant: {
        primary:
          "bg-primary text-white shadow-sm hover:bg-primary-dark",
        secondary:
          "border border-secondary text-secondary bg-transparent hover:bg-secondary/5",
        ghost:
          "text-text-secondary hover:bg-surface-alt hover:text-text-primary",
        danger:
          "bg-error text-white shadow-sm hover:bg-error/90",
        success:
          "bg-success text-white shadow-sm hover:bg-success/90",
        outline:
          "border border-border-strong bg-surface text-text-primary hover:bg-surface-alt",
      },
      size: {
        lg: "h-12 px-6 text-base font-semibold",
        default: "h-10 px-4 text-sm",
        sm: "h-8 px-3 text-[13px]",
        icon: "h-9 w-9",
      },
    },
    defaultVariants: {
      variant: "primary",
      size: "default",
    },
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        ref={ref}
        {...props}
      />
    );
  }
);
Button.displayName = "Button";

export { Button, buttonVariants };
