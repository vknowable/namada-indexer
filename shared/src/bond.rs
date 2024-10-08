use fake::Fake;

use crate::balance::Amount;
use crate::block::Epoch;
use crate::id::Id;

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct BondAddresses {
    pub source: Id,
    pub target: Id,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bond {
    pub source: Id,
    pub target: Id,
    pub amount: Amount,
    pub start: Epoch,
}

impl Bond {
    pub fn fake(validator_address: Id) -> Self {
        let source_address =
            namada_core::address::gen_established_address("namada-indexer");

        Self {
            source: Id::Account(source_address.to_string()),
            target: validator_address,
            amount: Amount::fake(),
            start: (1..1000).fake::<u32>(),
        }
    }
}

pub type Bonds = Vec<Bond>;
