use axum::async_trait;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl,
    SelectableHelper,
};
use orm::blocks::BlocksDb;
use orm::crawler_state::{CrawlerNameDb, BlockCrawlerStateDb};
use orm::schema::{blocks, crawler_state};

use crate::appstate::AppState;

#[derive(Clone)]
pub struct BlocksRepository {
    pub(crate) app_state: AppState,
}

#[async_trait]
pub trait BlocksRepositoryTrait {
    fn new(app_state: AppState) -> Self;

    async fn get_block(
        &self,
        height: i32,
    ) -> Result<BlocksDb, String>;

    async fn get_block_range(
        &self,
        tip: i32,
        length: i32,
    ) -> Result<Vec<BlocksDb>, String>;

    async fn get_state(&self) -> Result<BlockCrawlerStateDb, String>;
}

#[async_trait]
impl BlocksRepositoryTrait for BlocksRepository {
    fn new(app_state: AppState) -> Self {
        Self { app_state }
    }

    async fn get_block(
        &self,
        height: i32,
    ) -> Result<BlocksDb, String> {
        let conn = self.app_state.get_db_connection().await;

        conn.interact(move |conn| {
            blocks::table
                .filter(blocks::dsl::height.eq(height))
                .select(BlocksDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
    }

    async fn get_block_range(
        &self,
        tip: i32,
        length: i32
    ) -> Result<Vec<BlocksDb>, String> {
        let conn = self.app_state.get_db_connection().await;
        let first_block = tip - length + 1;
        let last_block = tip;

        conn.interact(move |conn| {
            blocks::table
                .filter(blocks::dsl::height.ge(first_block).and(blocks::dsl::height.le(last_block)))
                .order(blocks::dsl::height.desc())
                .select(BlocksDb::as_select())
                .load::<BlocksDb>(conn)
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
    }

    async fn get_state(&self) -> Result<BlockCrawlerStateDb, String> {
        let conn = self.app_state.get_db_connection().await;

        conn.interact(move |conn| {
            crawler_state::table
                .filter(crawler_state::dsl::name.eq(CrawlerNameDb::Blocks))
                .select((
                    crawler_state::dsl::last_processed_epoch,
                    crawler_state::dsl::timestamp,
                ))
                .first(conn)
        })
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
    }
}
