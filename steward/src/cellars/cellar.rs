//! Handlers for the Cellar.sol vault interface contract functions
//!
//! To learn more see https://github.com/PeggyJV/cellar-contracts/blob/main/src/base/Cellar.sol
use steward_proto::steward::cellar::Function;

use super::log_cellar_call;

const LOG_PREFIX: &str = "Cellar";

pub fn get_encoded_call(function: Function, cellar_id: String) -> Result<Vec<u8>, Error> {
    match function {
        Function::AddPosition(_) => todo!(),
        Function::PopPosition(_) => todo!(),
        Function::PushPosition(_) => todo!(),
        Function::RemovePosition(_) => todo!(),
        Function::ReplacePosition(_) => todo!(),
        Function::SetHoldingPosition(_) => todo!(),
        Function::Rebalance(_) => todo!(),
        Function::SetStrategistPayoutAddress(_) => todo!(),
        Function::SetWithdrawType(_) => todo!(),
        Function::SwapPositions(_) => todo!(),
    }
}
