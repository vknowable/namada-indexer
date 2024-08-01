use orm::blocks::BlocksDb;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub height: i32,
    pub epoch: i32,
    pub time: String,
    pub proposer_address: String,
    pub wrapper_txs: Vec<String>,
    pub inner_txs: Vec<String>,
    pub signatures: Vec<String>,
}

impl From<BlocksDb> for Block {
    fn from(value: BlocksDb) -> Self {
        Self {
            id: value.id,
            height: value.height,
            epoch: value.epoch,
            time: value.time,
            proposer_address: value.proposer_address,
            wrapper_txs: value.wrapper_txs,
            inner_txs: value.inner_txs,
            signatures: value.signatures,
        }
    }
}
