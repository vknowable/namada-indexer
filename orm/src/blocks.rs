use diesel::{Insertable, Queryable, Selectable};
use serde::Serialize;
use shared::block::BlockWithSignatures;

use crate::schema::blocks;

#[derive(Serialize, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = blocks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockInsertDb {
    //TODO: what block info do we really need to index here? many fields in the rpc response seem not that useful
    pub id: String,
    pub height: i32,
    pub epoch: i32,
    pub time: String,
    pub proposer_address: String,
    pub wrapper_txs: Vec<String>,
    pub inner_txs: Vec<String>,
    pub signatures: Vec<String>,
}

pub type BlocksDb = BlockInsertDb;

impl BlockInsertDb {
    pub fn from(block: BlockWithSignatures) -> Self {
      let inner_tx_hashes: Vec<String> = block.inner_txs().iter()
        .map(
          |tx| tx.tx_id.clone().to_string()
        ).collect();
      
      let wrapper_tx_hashes: Vec<String> = block.wrapper_txs().iter()
        .map(
          |tx| tx.tx_id.clone().to_string()
        ).collect();

        Self {
            id: block.hash.to_string(),
            height: block.header.height as i32,
            epoch: block.epoch as i32,
            time: block.header.timestamp,
            proposer_address: block.header.proposer_address,
            wrapper_txs: wrapper_tx_hashes,
            inner_txs: inner_tx_hashes,
            signatures: block.signatures,
        }
    }
}
