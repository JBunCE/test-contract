#![no_std]
use contract::Contract;
use gstd::{exec, msg, ActorId};
use io::ContractState;

pub mod contract;
pub mod handler;

static mut CONTRACT_STATE: Option<ContractState> = None;
static mut CONTRACT: Option<Contract> = None;

pub fn get_contract_state() -> &'static mut ContractState {
    unsafe { CONTRACT_STATE.as_mut().unwrap_unchecked() }
}

pub fn get_contract() -> &'static mut Contract {
    unsafe { CONTRACT.get_or_insert(Default::default()) }
}

#[no_mangle]
extern "C" fn init() {
    let owner: ActorId = msg::load().expect("Unable to decode the incoming message");

    unsafe {
        CONTRACT_STATE = Some(ContractState {
            owner,
            counter: 0,
            last_call_by: owner,
            value: exec::value_available(),
            last_block_number: exec::block_height(),
        });
    }
}

#[no_mangle]
extern "C" fn state() {
    let contract_state = get_contract_state();
    let _ = msg::reply(contract_state, 0);
}