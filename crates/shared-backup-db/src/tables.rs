use std::fmt;

use crate::utils::get_db_extension;

pub enum Tables {
    /// string version is sequencer_commitment
    #[allow(dead_code)]
    SequencerCommitment,
    MempoolTxs,
}

// impl to_string for tables
impl fmt::Display for Tables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tables::SequencerCommitment => write!(f, "sequencer_commitments"),
            Tables::MempoolTxs => write!(f, "mempool_txs"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CommitmentStatus {
    Mempool,
    Mined,
    Finalized,
}

impl fmt::Display for CommitmentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommitmentStatus::Mempool => write!(f, "mempool"),
            CommitmentStatus::Mined => write!(f, "mined"),
            CommitmentStatus::Finalized => write!(f, "finalized"),
        }
    }
}

impl std::str::FromStr for CommitmentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mempool" => Ok(CommitmentStatus::Mempool),
            "mined" => Ok(CommitmentStatus::Mined),
            "finalized" => Ok(CommitmentStatus::Finalized),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DbSequencerCommitment {
    /// Hex encoded L1 transaction ID
    pub l1_tx_id: Vec<u8>,
    pub l1_start_height: u32,
    pub l1_end_height: u32,
    /// Hex encoded L1 start hash
    pub l1_start_hash: Vec<u8>,
    /// Hex encoded L1 end hash
    pub l1_end_hash: Vec<u8>,
    pub l2_start_height: u64,
    pub l2_end_height: u64,
    /// Hex encoded merkle root of soft confirmation hashes
    pub merkle_root: Vec<u8>,
    pub status: CommitmentStatus,
}

pub fn create_database() -> String {
    format!("CREATE DATABASE citrea{};", get_db_extension())
}

pub const SEQUENCER_COMMITMENT_TABLE_CREATE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS sequencer_commitments (
    id                  SERIAL PRIMARY KEY,
    l1_start_height     OID NOT NULL,
    l1_end_height       OID NOT NULL,
    l1_tx_id            BYTEA NOT NULL,
    l1_start_hash       BYTEA NOT NULL,
    l1_end_hash         BYTEA NOT NULL,
    l2_start_height     OID NOT NULL,
    l2_end_height       OID NOT NULL,
    merkle_root         BYTEA NOT NULL,
    status              VARCHAR(15) NOT NULL,

    UNIQUE (l2_start_height, l2_end_height),
    UNIQUE (l1_start_height, l1_end_height),
    UNIQUE (l1_start_hash, l1_end_hash)
);
";

pub const INDEX_L2_END_HEIGHT: &str =
    "CREATE INDEX idx_l2_end_height ON sequencer_commitments(l2_end_height);";
pub const INDEX_L1_END_HEIGHT: &str =
    "CREATE INDEX idx_l1_end_height ON sequencer_commitments(l1_end_height);";
pub const INDEX_L1_END_HASH: &str =
    "CREATE INDEX idx_l1_end_hash ON sequencer_commitments(l1_end_hash);";

// tx is rlp encoded
pub const MEMPOOL_TXS_TABLE_CREATE_QUERY: &str = "
CREATE TABLE IF NOT EXISTS mempool_txs (
    id      SERIAL PRIMARY KEY,
    tx      BYTEA NOT NULL
);";
