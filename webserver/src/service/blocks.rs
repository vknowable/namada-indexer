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
        height: i32,
    ) -> Result<Block, BlocksError> {
        let block = self
            .blocks_repo
            .get_block(height)
            .await
            .map_err(BlocksError::Database)?;

        Ok(block.into())
    }

    pub async fn get_block_range(
        &self,
        tip: i32,
        length: i32,
    ) -> Result<Vec<Block>, BlocksError> {
        let blocks = self
            .blocks_repo
            .get_block_range(tip, length)
            .await
            .map_err(BlocksError::Database)?
            .into_iter().map(Block::from).collect();

        Ok(blocks)
    }

    pub async fn get_latest_height(
        &self,
    ) -> Result<i32, BlocksError> {
        self.blocks_repo
            .get_latest_height()
            .await
            .map_err(BlocksError::Database)
    }
}
