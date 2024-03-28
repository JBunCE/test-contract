use gstd::{exec, msg};
use io::ContractEvent;

use crate::get_contract_state;

// In this part if you are seing this 
// you should name the struct like you contract function
#[derive(Default)]
pub struct Contract {}

impl Contract {
    pub fn increase_counter(&mut self) {
        let contract_state = get_contract_state();

        contract_state.counter += 1;
        contract_state.last_call_by = msg::source();
        contract_state.last_block_number = exec::block_height();

        let _ = msg::reply(ContractEvent::Success, 0);
    }

    pub fn decrease_counter(&mut self) {
        let contract_state = get_contract_state();

        contract_state.counter -= 1;
        contract_state.last_call_by = msg::source();
        contract_state.last_block_number = exec::block_height();

        let _ = msg::reply(ContractEvent::Success, 0);
    }
}