import * as React from "react";
import {
  LayoutDashboard,
  Receipt,
  UtensilsCrossed,
  ClipboardList,
  Grid3X3,
  Package,
  BarChart3,
  Settings,
  ChevronLeft,
  ChevronRight,
} from "lucide-react";
import { cn } from "../lib/utils";

export interface NavItem {
  label: string;
  icon: React.ReactNode;
  href: string;
  badge?: string;
}

const defaultNavItems: NavItem[] = [
  { label: "Dashboard", icon: <LayoutDashboard size={20} />, href: "/" },
  { label: "Billing", icon: <Receipt size={20} />, href: "/billing" },
  { label: "Menu", icon: <UtensilsCrossed size={20} />, href: "/menu" },
  { label: "Orders", icon: <ClipboardList size={20} />, href: "/orders" },
  { label: "Tables", icon: <Grid3X3 size={20} />, href: "/tables" },
  { label: "Inventory", icon: <Package size={20} />, href: "/inventory" },
  { label: "Reports", icon: <BarChart3 size={20} />, href: "/reports" },
  { label: "Settings", icon: <Settings size={20} />, href: "/settings" },
];

interface SidebarProps {
  items?: NavItem[];
  activePath?: string;
  collapsed?: boolean;
  onToggle?: () => void;
}

export function Sidebar({
  items = defaultNavItems,
  activePath = "/",
  collapsed = false,
  onToggle,
}: SidebarProps) {
  return (
    <aside
      className={cn(
        "flex flex-col h-screen bg-secondary text-white transition-all duration-200 ease-in-out",
        collapsed ? "w-16" : "w-60",
      )}
    >
      {/* Logo */}
      <div className="flex items-center h-14 px-4 border-b border-white/10">
        {!collapsed && (
          <span className="text-lg font-bold font-display tracking-tight">
            Restro<span className="text-primary">Sync</span>
          </span>
        )}
        {collapsed && (
          <span className="text-lg font-bold text-primary mx-auto">R</span>
        )}
      </div>

      {/* Navigation */}
      <nav className="flex-1 py-3 px-2 space-y-0.5 overflow-y-auto">
        <p
          className={cn(
            "text-[11px] font-semibold uppercase tracking-wider text-[#7B8FA3] px-2 mb-2",
            collapsed && "sr-only",
          )}
        >
          Menu
        </p>
        {items.map((item) => {
          const isActive = activePath === item.href;
          return (
            <a
              key={item.href}
              href={item.href}
              className={cn(
                "flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors",
                isActive
                  ? "bg-primary/15 text-white border-l-[3px] border-primary"
                  : "text-[#C8D6E5] hover:bg-white/6 border-l-[3px] border-transparent",
                collapsed && "justify-center px-0",
              )}
              title={collapsed ? item.label : undefined}
            >
              <span className="shrink-0">{item.icon}</span>
              {!collapsed && <span>{item.label}</span>}
              {!collapsed && item.badge && (
                <span className="ml-auto bg-primary/20 text-primary text-[11px] font-semibold px-1.5 py-0.5 rounded">
                  {item.badge}
                </span>
              )}
            </a>
          );
        })}
      </nav>

      {/* Collapse toggle */}
      <button
        onClick={onToggle}
        className="flex items-center justify-center h-10 border-t border-white/10 text-[#7B8FA3] hover:text-white transition-colors"
      >
        {collapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
      </button>
    </aside>
  );
}
