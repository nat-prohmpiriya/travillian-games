use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ==================== Enums ====================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    GoldPurchase,
    Subscription,
    GoldSpend,
    GoldRefund,
    GoldGift,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "subscription_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionType {
    TravianPlus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "gold_feature", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum GoldFeature {
    FinishNow,
    NpcMerchant,
    ProductionBonus,
    BookOfWisdom,
    Artwork,
    Ointment,
    PlusSubscription,
    HeroSlot,
}

// ==================== Database Models ====================

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct GoldPackage {
    pub id: Uuid,
    pub name: String,
    pub gold_amount: i32,
    pub price_cents: i32,
    pub currency: String,
    pub stripe_price_id: Option<String>,
    pub is_active: bool,
    pub bonus_percent: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub gold_amount: i32,
    pub amount_cents: Option<i32>,
    pub currency: Option<String>,
    pub stripe_session_id: Option<String>,
    pub stripe_payment_intent_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub gold_package_id: Option<Uuid>,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct UserSubscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub subscription_type: SubscriptionType,
    pub stripe_subscription_id: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub auto_renew: bool,
    pub is_active: bool,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct GoldUsage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feature: GoldFeature,
    pub gold_spent: i32,
    pub target_type: Option<String>,
    pub target_id: Option<Uuid>,
    pub effect_data: Option<serde_json::Value>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct GoldFeatureCost {
    pub feature: GoldFeature,
    pub base_cost: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct SubscriptionPrice {
    pub id: Uuid,
    pub subscription_type: SubscriptionType,
    pub duration_days: i32,
    pub gold_cost: i32,
    pub stripe_price_id: Option<String>,
    pub is_active: bool,
}

// ==================== Request DTOs ====================

#[derive(Debug, Deserialize)]
pub struct PurchaseGoldRequest {
    pub package_id: Uuid,
    pub success_url: String,
    pub cancel_url: String,
}

#[derive(Debug, Deserialize)]
pub struct BuySubscriptionRequest {
    pub duration_days: i32,
}

#[derive(Debug, Deserialize)]
pub struct UseFinishNowRequest {
    pub target_type: String, // "building" or "troop_queue"
    pub target_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UseNpcMerchantRequest {
    pub village_id: Uuid,
    pub wood: i32,
    pub clay: i32,
    pub iron: i32,
    pub crop: i32,
}

#[derive(Debug, Deserialize)]
pub struct UseProductionBonusRequest {
    pub village_id: Uuid,
    pub resource_type: String, // "wood", "clay", "iron", "crop"
}

#[derive(Debug, Deserialize)]
pub struct UseBookOfWisdomRequest {
    pub village_id: Uuid,
}

// ==================== Response DTOs ====================

#[derive(Debug, Clone, Serialize)]
pub struct GoldBalanceResponse {
    pub gold_balance: i32,
    pub has_plus: bool,
    pub plus_expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckoutResponse {
    pub checkout_url: String,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub gold_amount: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FeatureCostResponse {
    pub feature: GoldFeature,
    pub cost: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UseFeatureResponse {
    pub success: bool,
    pub gold_spent: i32,
    pub new_balance: i32,
    pub message: String,
}

impl From<Transaction> for TransactionResponse {
    fn from(t: Transaction) -> Self {
        Self {
            id: t.id,
            transaction_type: t.transaction_type,
            status: t.status,
            gold_amount: t.gold_amount,
            description: t.description,
            created_at: t.created_at,
        }
    }
}
