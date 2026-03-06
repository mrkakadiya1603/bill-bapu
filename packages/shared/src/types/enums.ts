export const UserRole = {
  SuperAdmin: "SuperAdmin",
  Owner: "Owner",
  Manager: "Manager",
  Cashier: "Cashier",
  Waiter: "Waiter",
  Kitchen: "Kitchen",
  Viewer: "Viewer",
} as const;
export type UserRole = (typeof UserRole)[keyof typeof UserRole];

export const OrderStatus = {
  Draft: "Draft",
  Placed: "Placed",
  Confirmed: "Confirmed",
  Preparing: "Preparing",
  Ready: "Ready",
  Served: "Served",
  Completed: "Completed",
  Cancelled: "Cancelled",
} as const;
export type OrderStatus = (typeof OrderStatus)[keyof typeof OrderStatus];

export const OrderType = {
  DineIn: "DineIn",
  TakeAway: "TakeAway",
  Delivery: "Delivery",
  DriveThrough: "DriveThrough",
} as const;
export type OrderType = (typeof OrderType)[keyof typeof OrderType];

export const PaymentMode = {
  Cash: "Cash",
  Card: "Card",
  Upi: "Upi",
  Wallet: "Wallet",
  Credit: "Credit",
  Split: "Split",
} as const;
export type PaymentMode = (typeof PaymentMode)[keyof typeof PaymentMode];

export const PaymentStatus = {
  Pending: "Pending",
  Partial: "Partial",
  Paid: "Paid",
  Refunded: "Refunded",
} as const;
export type PaymentStatus =
  (typeof PaymentStatus)[keyof typeof PaymentStatus];

export const TableStatus = {
  Available: "Available",
  Occupied: "Occupied",
  Reserved: "Reserved",
  Billing: "Billing",
  Maintenance: "Maintenance",
} as const;
export type TableStatus = (typeof TableStatus)[keyof typeof TableStatus];

export const KotStatus = {
  New: "New",
  Acknowledged: "Acknowledged",
  Preparing: "Preparing",
  Ready: "Ready",
  Delivered: "Delivered",
  Cancelled: "Cancelled",
} as const;
export type KotStatus = (typeof KotStatus)[keyof typeof KotStatus];

export const SyncStatus = {
  Pending: "Pending",
  Synced: "Synced",
  Conflict: "Conflict",
  Failed: "Failed",
} as const;
export type SyncStatus = (typeof SyncStatus)[keyof typeof SyncStatus];

export const ConnectivityMode = {
  Cloud: "Cloud",
  LocalNetwork: "LocalNetwork",
  Standalone: "Standalone",
} as const;
export type ConnectivityMode =
  (typeof ConnectivityMode)[keyof typeof ConnectivityMode];
