use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum UserRole {
    SuperAdmin,
    Owner,
    Manager,
    Cashier,
    Waiter,
    Kitchen,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum OrderStatus {
    Draft,
    Placed,
    Confirmed,
    Preparing,
    Ready,
    Served,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum OrderType {
    DineIn,
    TakeAway,
    Delivery,
    DriveThrough,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum PaymentMode {
    Cash,
    Card,
    Upi,
    Wallet,
    Credit,
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum PaymentStatus {
    Pending,
    Partial,
    Paid,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum TableStatus {
    Available,
    Occupied,
    Reserved,
    Billing,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum KotStatus {
    New,
    Acknowledged,
    Preparing,
    Ready,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum SyncStatus {
    Pending,
    Synced,
    Conflict,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[ts(export)]
pub enum ConnectivityMode {
    Cloud,
    LocalNetwork,
    Standalone,
}
