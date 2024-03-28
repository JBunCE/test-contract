use gstd::msg;
use io::ContractAction;

use crate::get_contract;

#[no_mangle]
extern "C" fn handle() {
    let action: ContractAction = msg::load()
        .expect("Unable to decode the incoming message");
    
    let contract = get_contract();

    match action {
        ContractAction::IncreaseCounter => contract.increase_counter(),
        ContractAction::DecreaseCounter => contract.decrease_counter(),
    }
}