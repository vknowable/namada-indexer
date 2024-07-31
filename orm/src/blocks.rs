use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
// use shared::transaction::{
//     InnerTransaction, TransactionExitStatus, TransactionKind,
//     WrapperTransaction,
// };
use shared::block::Block;

// use crate::schema::{inner_transactions, wrapper_transactions};
use crate::schema::blocks;

// #[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
// #[ExistingTypePath = "crate::schema::sql_types::TransactionKind"]
// pub enum TransactionKindDb {
//     TransparentTransfer,
//     ShieldedTransfer,
//     ShieldingTransfer,
//     UnshieldingTransfer,
//     Bond,
//     Redelegation,
//     Unbond,
//     Withdraw,
//     ClaimRewards,
//     VoteProposal,
//     InitProposal,
//     ChangeMetadata,
//     ChangeCommission,
//     RevealPk,
//     Unknown,
// }

// impl From<TransactionKind> for TransactionKindDb {
//     fn from(value: TransactionKind) -> Self {
//         match value {
//             TransactionKind::TransparentTransfer(_) => {
//                 TransactionKindDb::TransparentTransfer
//             }
//             TransactionKind::ShieldedTransfer(_) => {
//                 TransactionKindDb::ShieldedTransfer
//             }
//             TransactionKind::Bond(_) => TransactionKindDb::Bond,
//             TransactionKind::Redelegation(_) => TransactionKindDb::Redelegation,
//             TransactionKind::Unbond(_) => TransactionKindDb::Unbond,
//             TransactionKind::Withdraw(_) => TransactionKindDb::Withdraw,
//             TransactionKind::ClaimRewards(_) => TransactionKindDb::ClaimRewards,
//             TransactionKind::ProposalVote(_) => TransactionKindDb::VoteProposal,
//             TransactionKind::InitProposal(_) => TransactionKindDb::InitProposal,
//             TransactionKind::MetadataChange(_) => {
//                 TransactionKindDb::ChangeMetadata
//             }
//             TransactionKind::CommissionChange(_) => {
//                 TransactionKindDb::ChangeCommission
//             }
//             TransactionKind::RevealPk(_) => TransactionKindDb::RevealPk,
//             TransactionKind::Unknown => TransactionKindDb::Unknown,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, diesel_derive_enum::DbEnum)]
// #[ExistingTypePath = "crate::schema::sql_types::TransactionResult"]
// pub enum TransactionResultDb {
//     Applied,
//     Rejected,
// }

// impl From<TransactionExitStatus> for TransactionResultDb {
//     fn from(value: TransactionExitStatus) -> Self {
//         match value {
//             TransactionExitStatus::Applied => TransactionResultDb::Applied,
//             TransactionExitStatus::Rejected => TransactionResultDb::Rejected,
//         }
//     }
// }

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
    pub fn from(block: Block) -> Self {
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
            wrapper_txs: inner_tx_hashes,
            inner_txs: wrapper_tx_hashes,
            signatures: vec![],
        }
    }
}

// #[derive(Serialize, Queryable, Selectable, Insertable, Clone)]
// #[diesel(table_name = wrapper_transactions)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct WrapperTransactionInsertDb {
//     pub id: String,
//     pub fee_payer: String,
//     pub fee_token: String,
//     pub gas_limit: String,
//     pub block_height: i32,
//     pub exit_code: TransactionResultDb,
//     pub atomic: bool,
// }

// pub type WrapperTransactionDb = WrapperTransactionInsertDb;

// impl WrapperTransactionInsertDb {
//     pub fn from(tx: WrapperTransaction) -> Self {
//         Self {
//             id: tx.tx_id.to_string(),
//             fee_payer: tx.fee.gas_payer.to_string(),
//             fee_token: tx.fee.gas_token.to_string(),
//             gas_limit: tx.fee.gas,
//             block_height: tx.block_height as i32,
//             exit_code: TransactionResultDb::from(tx.exit_code),
//             atomic: tx.atomic,
//         }
//     }
// }
