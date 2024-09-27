use alloy_primitives::TxHash;
use alloy_rpc_types::{Block, Header, Log};

use defi_types::{ChainParameters, GethStateUpdateVec, MempoolTx};

use crate::Message;

#[derive(Clone, Debug)]
pub struct NodeMempoolDataUpdate {
    pub tx_hash: TxHash,
    pub mempool_tx: MempoolTx,
}

pub type MessageMempoolDataUpdate = Message<NodeMempoolDataUpdate>;

#[derive(Clone, Debug)]
pub struct BlockStateUpdate {
    pub block_header: Header,
    pub state_update: GethStateUpdateVec,
}

#[derive(Clone, Debug)]
pub struct BlockLogs {
    pub block_header: Header,
    pub logs: Vec<Log>,
}

#[derive(Clone, Debug, Default)]
pub struct BlockHeader {
    pub header: Header,
    pub next_block_base_fee: u128,
    pub next_block_number: u64,
    pub next_block_timestamp: u64,
}

pub type MessageBlockHeader = Message<BlockHeader>;
pub type MessageBlock = Message<Block>;
pub type MessageBlockLogs = Message<BlockLogs>;
pub type MessageBlockStateUpdate = Message<BlockStateUpdate>;

impl BlockHeader {
    pub fn new(chain_parameters: &ChainParameters, header: Header) -> Self {
        let next_block_base_fee: u128 =
            chain_parameters.calc_next_block_base_fee(header.gas_used, header.gas_limit, header.base_fee_per_gas.unwrap_or_default());
        let next_block_number = header.number + 1;
        let next_block_timestamp = header.timestamp + 12;
        Self { header, next_block_base_fee, next_block_number, next_block_timestamp }
    }
}
