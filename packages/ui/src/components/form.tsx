import * as React from "react";
import * as LabelPrimitive from "@radix-ui/react-label";
import { cn } from "../lib/utils";

const Label = React.forwardRef<
  React.ComponentRef<typeof LabelPrimitive.Root>,
  React.ComponentPropsWithoutRef<typeof LabelPrimitive.Root>
>(({ className, ...props }, ref) => (
  <LabelPrimitive.Root
    ref={ref}
    className={cn(
      "text-xs font-semibold leading-none text-text-secondary peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
      className,
    )}
    {...props}
  />
));
Label.displayName = LabelPrimitive.Root.displayName;

interface FormFieldProps extends React.HTMLAttributes<HTMLDivElement> {
  /** Optional error message displayed below the input */
  error?: string;
}

const FormField = React.forwardRef<HTMLDivElement, FormFieldProps>(
  ({ className, error, children, ...props }, ref) => (
    <div
      ref={ref}
      className={cn("space-y-1.5", className)}
      {...props}
    >
      {children}
      {error && (
        <p className="text-xs text-error" role="alert">
          {error}
        </p>
      )}
    </div>
  ),
);
FormField.displayName = "FormField";

const FormDescription = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, ...props }, ref) => (
  <p
    ref={ref}
    className={cn("text-xs text-text-tertiary", className)}
    {...props}
  />
));
FormDescription.displayName = "FormDescription";

const FormMessage = React.forwardRef<
  HTMLParagraphElement,
  React.HTMLAttributes<HTMLParagraphElement>
>(({ className, children, ...props }, ref) => {
  if (!children) return null;

  return (
    <p
      ref={ref}
      className={cn("text-xs font-medium text-error", className)}
      role="alert"
      {...props}
    >
      {children}
    </p>
  );
});
FormMessage.displayName = "FormMessage";

interface FormSectionProps extends React.HTMLAttributes<HTMLFieldSetElement> {
  /** Section title */
  title?: string;
  /** Section description */
  description?: string;
}

const FormSection = React.forwardRef<HTMLFieldSetElement, FormSectionProps>(
  ({ className, title, description, children, ...props }, ref) => (
    <fieldset
      ref={ref}
      className={cn("space-y-4", className)}
      {...props}
    >
      {(title || description) && (
        <div className="space-y-1">
          {title && (
            <legend className="text-base font-semibold text-text-primary font-display">
              {title}
            </legend>
          )}
          {description && (
            <p className="text-sm text-text-secondary">{description}</p>
          )}
        </div>
      )}
      {children}
    </fieldset>
  ),
);
FormSection.displayName = "FormSection";

const FormActions = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      "flex items-center justify-end gap-3 pt-4 border-t border-border",
      className,
    )}
    {...props}
  />
));
FormActions.displayName = "FormActions";

export {
  Label,
  FormField,
  FormDescription,
  FormMessage,
  FormSection,
  FormActions,
};
