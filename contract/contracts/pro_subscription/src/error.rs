use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ProSubscriptionError {
    /// Contract has already been initialized
    AlreadyInitialized = 1,
    /// Contract has not been initialized
    NotInitialized = 2,
    /// Caller is not the admin
    Unauthorized = 3,
    /// Subscription not found for this organizer
    SubscriptionNotFound = 4,
    /// Subscription has expired
    SubscriptionExpired = 5,
    /// Subscription is not active
    SubscriptionInactive = 6,
    /// Invalid subscription tier
    InvalidTier = 7,
    /// Invalid price (must be positive)
    InvalidPrice = 8,
    /// Insufficient payment amount
    InsufficientPayment = 9,
    /// Arithmetic overflow or underflow
    ArithmeticError = 10,
    /// Token transfer failed
    TransferFailed = 11,
    /// Invalid address provided
    InvalidAddress = 12,
    /// Subscription already exists and is active
    SubscriptionAlreadyActive = 13,
}
