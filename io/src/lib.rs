#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo};

#[derive(TypeInfo, Encode, Decode)]
pub enum ContractAction {
    IncreaseCounter,
    DecreaseCounter,
}

#[derive(TypeInfo, Encode, Decode)]
pub enum ContractEvent {
    Success,
    Failure,
}

#[derive(Default, TypeInfo, Encode, Decode)]
pub struct ContractState {
    pub owner: ActorId,
    pub counter: u64,
    pub value: u128,
    pub last_call_by: ActorId,
    pub last_block_number: u32,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<ContractAction, ContractEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<ContractState>;
}