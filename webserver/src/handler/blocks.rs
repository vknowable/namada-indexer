use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;
use validator::Validate;

use crate::dto::blocks::BlockRangeQueryParams;
use crate::error::api::ApiError;
use crate::error::blocks::BlocksError;
use crate::response::blocks::Block;
use crate::state::common::CommonState;

#[debug_handler]
pub async fn get_block_at_latest(
    _headers: HeaderMap,
    State(state): State<CommonState>,
) -> Result<Json<Block>, ApiError> {
    
    let latest_height = state.blocks_service.get_latest_height().await?;
    let block = state.blocks_service.get_block(latest_height).await?;
    Ok(Json(block))
}

#[debug_handler]
pub async fn get_block_at_height(
    _headers: HeaderMap,
    Path(height): Path<u64>,
    State(state): State<CommonState>,
) -> Result<Json<Block>, ApiError> {

    let block = state.blocks_service.get_block(height as i32).await?;
    Ok(Json(block))
}

#[debug_handler]
pub async fn get_block_range(
    _headers: HeaderMap,
    Query(query): Query<BlockRangeQueryParams>,
    State(state): State<CommonState>,
) -> Result<Json<Vec<Block>>, ApiError> {

    // validate params
    if let Err(e) = query.validate() {
        return Err(BlocksError::InvalidParams(e.to_string()).into())
    };

    // Get the latest block height if tip is not provided
    let tip = if let Some(tip) = query.tip {
        tip as i32
    } else {
        state.blocks_service.get_latest_height().await?
    };

    let length = query.length.unwrap_or(1) as i32;

    // check for range that extends below block 1
    tip.checked_sub(length).ok_or_else(|| {
        BlocksError::InvalidRange("Range would include block height less than 1".to_string())
    })?;

    let block_range = state.blocks_service.get_block_range(tip, length).await?;

    Ok(Json(block_range))
}
