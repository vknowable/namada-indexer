use anyhow::Context;
use chrono::NaiveDateTime;
use diesel::upsert::excluded;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use orm::crawler_state::{BlockStateInsertDb, CrawlerNameDb};
use orm::schema::{crawler_state, blocks};
use orm::blocks::BlockInsertDb;
use shared::crawler_state::{BlockCrawlerState, CrawlerName};
use shared::block::BlockWithSignatures;

pub fn insert_blocks(
    transaction_conn: &mut PgConnection,
    blocks: Vec<BlockWithSignatures>,
) -> anyhow::Result<()> {
    diesel::insert_into(blocks::table)
        .values::<&Vec<BlockInsertDb>>(
            &blocks.into_iter()
                .map(BlockInsertDb::from)
                .collect::<Vec<_>>(),
        )
        .execute(transaction_conn)
        .context("Failed to insert blocks in db")?;

    anyhow::Ok(())
}

pub fn insert_crawler_state(
    transaction_conn: &mut PgConnection,
    crawler_state: BlockCrawlerState,
) -> anyhow::Result<()> {
    diesel::insert_into(crawler_state::table)
        .values::<&BlockStateInsertDb>(
            &(CrawlerName::Blocks, crawler_state).into(),
        )
        .on_conflict(crawler_state::name)
        .do_update()
        .set((
            crawler_state::timestamp.eq(excluded(crawler_state::timestamp)),
            crawler_state::last_processed_block
                .eq(excluded(crawler_state::last_processed_block)),
        ))
        .execute(transaction_conn)
        .context("Failed to update crawler state in db")?;

    anyhow::Ok(())
}

pub fn update_crawler_timestamp(
    transaction_conn: &mut PgConnection,
    timestamp: NaiveDateTime,
) -> anyhow::Result<()> {
    diesel::update(crawler_state::table)
        .filter(
            crawler_state::name
                .eq(CrawlerNameDb::from(CrawlerName::Transactions)),
        )
        .set(crawler_state::timestamp.eq(timestamp))
        .execute(transaction_conn)
        .context("Failed to update crawler timestamp in db")?;

    anyhow::Ok(())
}
