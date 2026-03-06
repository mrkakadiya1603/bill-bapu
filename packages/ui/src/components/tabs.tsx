import * as React from "react";
import { cn } from "../lib/utils";

const Tabs = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & { defaultValue?: string; value?: string; onValueChange?: (value: string) => void }
>(({ className, defaultValue, value, onValueChange, children, ...props }, ref) => {
  const [selected, setSelected] = React.useState(defaultValue ?? "");
  const current = value ?? selected;

  const handleChange = React.useCallback(
    (val: string) => {
      if (value === undefined) setSelected(val);
      onValueChange?.(val);
    },
    [value, onValueChange],
  );

  return (
    <TabsContext.Provider value={{ value: current, onValueChange: handleChange }}>
      <div ref={ref} className={cn("w-full", className)} {...props}>
        {children}
      </div>
    </TabsContext.Provider>
  );
});
Tabs.displayName = "Tabs";

interface TabsContextValue {
  value: string;
  onValueChange: (value: string) => void;
}

const TabsContext = React.createContext<TabsContextValue>({
  value: "",
  onValueChange: () => {},
});

function useTabsContext() {
  return React.useContext(TabsContext);
}

const TabsList = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    role="tablist"
    className={cn(
      "inline-flex h-10 items-center justify-center rounded-lg bg-surface-alt p-1 text-text-secondary",
      className,
    )}
    {...props}
  />
));
TabsList.displayName = "TabsList";

const TabsTrigger = React.forwardRef<
  HTMLButtonElement,
  React.ButtonHTMLAttributes<HTMLButtonElement> & { value: string }
>(({ className, value, ...props }, ref) => {
  const ctx = useTabsContext();
  const isSelected = ctx.value === value;

  return (
    <button
      ref={ref}
      role="tab"
      type="button"
      aria-selected={isSelected}
      data-state={isSelected ? "active" : "inactive"}
      className={cn(
        "inline-flex items-center justify-center whitespace-nowrap rounded-md px-3 py-1.5 text-sm font-medium ring-offset-surface transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/30 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
        isSelected
          ? "bg-surface text-text-primary shadow-sm"
          : "text-text-secondary hover:text-text-primary",
        className,
      )}
      onClick={() => ctx.onValueChange(value)}
      {...props}
    />
  );
});
TabsTrigger.displayName = "TabsTrigger";

const TabsContent = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & { value: string }
>(({ className, value, ...props }, ref) => {
  const ctx = useTabsContext();

  if (ctx.value !== value) return null;

  return (
    <div
      ref={ref}
      role="tabpanel"
      data-state={ctx.value === value ? "active" : "inactive"}
      tabIndex={0}
      className={cn(
        "mt-2 ring-offset-surface focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/30 focus-visible:ring-offset-2",
        className,
      )}
      {...props}
    />
  );
});
TabsContent.displayName = "TabsContent";

export { Tabs, TabsList, TabsTrigger, TabsContent };
