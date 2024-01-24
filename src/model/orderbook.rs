#![allow(non_snake_case)]

use anyhow::Result;
// use modql::field::{HasFields, Fields};
use crate::model::error::Error;
use sea_query::{enum_def, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    types::{time::OffsetDateTime, BigDecimal},
};

use super::ModelManager;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AtomicSwap {
    pub ID: u64,
    pub CreatedAt: String,
    pub UpdatedAt: String,
    pub DeletedAt: String,
    pub swapStatus: u8,
    pub initatorAddress: String,
    pub redeemerAddress: String,
    pub onChainIdentifier: String,
    pub timelock: String,
    pub chain: String,
    pub asset: String,
    pub currentConfirmation: u8,
    pub minimumConfirmations: u8,
    pub amount: String,
    pub filledAmount: String,
    pub priceByOracle: u8,
    pub initiateTxHash: u8,
    pub initiateBlockNumber: u32,
    pub redeemTxHash: String,
    pub refundTxHash: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[enum_def]
pub struct Orders {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub maker: String,
    pub taker: String,
    pub order_pair: String,
    pub initiator_atomic_swap_id: i64,
    pub follower_atomic_swap_id: i64,
    pub secret_hash: String,
    pub secret: String,
    pub price: BigDecimal,
    pub status: i64,
    pub secret_nonce: i64,
    pub user_btc_wallet_address: String,
    pub random_multiplier: i64,
    pub random_score: i64,
    pub fee: i64,
}

impl Orders {
    pub async fn getOrderById(mm: &ModelManager, id: i64) -> Result<Orders> {
        let db = mm.db();

        let mut query = Query::select();
        query.columns([
            OrdersIden::Id,
            OrdersIden::CreatedAt,
            OrdersIden::UpdatedAt,
            OrdersIden::DeletedAt,
            OrdersIden::Maker,
            OrdersIden::Taker,
            OrdersIden::OrderPair,
            OrdersIden::InitiatorAtomicSwapId,
            OrdersIden::FollowerAtomicSwapId,
            OrdersIden::SecretHash,
            OrdersIden::Secret,
            OrdersIden::Price,
            OrdersIden::Status,
            OrdersIden::SecretNonce,
            OrdersIden::UserBtcWalletAddress,
            OrdersIden::RandomMultiplier,
            OrdersIden::RandomScore,
            OrdersIden::Fee,
        ]);
        query.from(OrdersIden::Table);

        let table_name = OrdersIden::Table.to_string();
        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let entity = sqlx::query_as_with::<_, Orders, _>(&sql, values)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound {
                entity: table_name,
                id,
            })?;

        Ok(entity)
    }
}
