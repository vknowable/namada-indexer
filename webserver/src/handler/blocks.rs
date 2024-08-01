use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use axum_macros::debug_handler;

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
    //TODO: actually get latest height
    let latest_height = 1;

    let block = state.blocks_service.get_block(latest_height).await?;
    Ok(Json(block))
}

#[debug_handler]
pub async fn get_block_at_height(
    _headers: HeaderMap,
    Path(height): Path<u64>,
    State(state): State<CommonState>,
) -> Result<Json<Block>, ApiError> {

    let block = state.blocks_service.get_block(height).await?;
    Ok(Json(block))
}

#[debug_handler]
pub async fn get_block_range(
    _headers: HeaderMap,
    Query(query): Query<BlockRangeQueryParams>,
    State(state): State<CommonState>,
) -> Result<Json<Vec<Block>>, ApiError> {
    //TODO: if tip not provided, use latest
    let tip = query.tip.unwrap_or(1);
    let length = query.length.unwrap_or(1);

    let earliest_block = tip - length + 1;
    if earliest_block <= 0 {
        return Err(BlocksError::InvalidRange("Range would include block height less than 1".to_string()).into())
    }

    let block_range = state.blocks_service.get_block_range(tip, length).await?;

    Ok(Json(block_range))
}
