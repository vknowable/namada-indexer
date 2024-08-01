use crate::appstate::AppState;
use crate::error::blocks::BlocksError;
use crate::repository::blocks::{BlocksRepository, BlocksRepositoryTrait};
use crate::response::blocks::Block;

#[derive(Clone)]
pub struct BlocksService {
    blocks_repo: BlocksRepository,
}

impl BlocksService {
    pub fn new(app_state: AppState) -> Self {
        Self {
            blocks_repo: BlocksRepository::new(app_state.clone()),
        }
    }

    pub async fn get_block(
        &self,
        height: u64,
    ) -> Result<Block, BlocksError> {
        let block = self
            .blocks_repo
            .get_block(height as i32)
            .await
            .map_err(BlocksError::Database)?;

        Ok(block.into())
    }

    pub async fn get_block_range(
        &self,
        tip: u64,
        length: u64,
    ) -> Result<Vec<Block>, BlocksError> {
        let blocks = self
            .blocks_repo
            .get_block_range(tip as i32, length as i32)
            .await
            .map_err(BlocksError::Database)?
            .into_iter().map(Block::from).collect();

        Ok(blocks)
    }
}
