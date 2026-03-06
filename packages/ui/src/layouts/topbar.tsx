import * as React from "react";
import { Cloud, Wifi, HardDrive, Bell, User } from "lucide-react";
import { cn } from "../lib/utils";

type ConnectionMode = "cloud" | "local" | "standalone";

interface TopbarProps {
  connectionMode?: ConnectionMode;
  outletName?: string;
  userName?: string;
}

const connectionConfig: Record<
  ConnectionMode,
  { icon: React.ReactNode; label: string; color: string }
> = {
  cloud: {
    icon: <Cloud size={16} />,
    label: "Cloud",
    color: "text-success",
  },
  local: {
    icon: <Wifi size={16} />,
    label: "Local WiFi",
    color: "text-info",
  },
  standalone: {
    icon: <HardDrive size={16} />,
    label: "Standalone",
    color: "text-warning",
  },
};

export function Topbar({
  connectionMode = "cloud",
  outletName = "My Restaurant",
  userName = "Admin",
}: TopbarProps) {
  const conn = connectionConfig[connectionMode];

  return (
    <header className="flex items-center justify-between h-14 px-6 bg-surface border-b border-border">
      {/* Left: outlet name */}
      <div className="flex items-center gap-3">
        <h1 className="text-base font-semibold text-text-primary">
          {outletName}
        </h1>
      </div>

      {/* Right: status + notifications + user */}
      <div className="flex items-center gap-4">
        {/* Connection status */}
        <div
          className={cn(
            "flex items-center gap-1.5 text-xs font-medium",
            conn.color
          )}
        >
          {conn.icon}
          <span>{conn.label}</span>
        </div>

        {/* Notifications */}
        <button className="relative p-2 rounded-lg text-text-secondary hover:bg-surface-alt transition-colors">
          <Bell size={18} />
        </button>

        {/* User */}
        <button className="flex items-center gap-2 px-2 py-1.5 rounded-lg text-text-secondary hover:bg-surface-alt transition-colors">
          <div className="w-7 h-7 rounded-full bg-primary/10 flex items-center justify-center">
            <User size={14} className="text-primary" />
          </div>
          <span className="text-sm font-medium text-text-primary">
            {userName}
          </span>
        </button>
      </div>
    </header>
  );
}
