extern crate bls;
extern crate boolean_bitfield;
extern crate ethereum_types;
extern crate ssz;

pub mod test_utils;

pub mod attestation;
pub mod attestation_data;
pub mod beacon_block;
pub mod beacon_block_body;
pub mod beacon_state;
pub mod casper_slashing;
pub mod crosslink;
pub mod deposit;
pub mod deposit_data;
pub mod deposit_input;
pub mod eth1_data;
pub mod eth1_data_vote;
pub mod exit;
pub mod fork;
pub mod pending_attestation;
pub mod proposal_signed_data;
pub mod proposer_slashing;
pub mod shard_reassignment_record;
pub mod slashable_vote_data;
pub mod special_record;
pub mod validator;
pub mod validator_registry;
pub mod validator_registry_delta_block;

pub mod readers;

use self::ethereum_types::{H160, H256, U256};
use std::collections::HashMap;

pub use crate::attestation::Attestation;
pub use crate::attestation_data::AttestationData;
pub use crate::beacon_block::BeaconBlock;
pub use crate::beacon_block_body::BeaconBlockBody;
pub use crate::beacon_state::BeaconState;
pub use crate::casper_slashing::CasperSlashing;
pub use crate::crosslink::Crosslink;
pub use crate::deposit::Deposit;
pub use crate::deposit_data::DepositData;
pub use crate::deposit_input::DepositInput;
pub use crate::eth1_data::Eth1Data;
pub use crate::eth1_data_vote::Eth1DataVote;
pub use crate::exit::Exit;
pub use crate::fork::Fork;
pub use crate::pending_attestation::PendingAttestation;
pub use crate::proposal_signed_data::ProposalSignedData;
pub use crate::proposer_slashing::ProposerSlashing;
pub use crate::slashable_vote_data::SlashableVoteData;
pub use crate::special_record::{SpecialRecord, SpecialRecordKind};
pub use crate::validator::{StatusFlags as ValidatorStatusFlags, Validator};
pub use crate::validator_registry_delta_block::ValidatorRegistryDeltaBlock;

pub type Hash256 = H256;
pub type Address = H160;
pub type EthBalance = U256;
pub type Bitfield = boolean_bitfield::BooleanBitfield;
pub type BitfieldError = boolean_bitfield::Error;

/// Maps a (slot, shard_id) to attestation_indices.
pub type AttesterMap = HashMap<(u64, u64), Vec<usize>>;

/// Maps a slot to a block proposer.
pub type ProposerMap = HashMap<u64, usize>;

pub use bls::{AggregatePublicKey, AggregateSignature, PublicKey, Signature};
