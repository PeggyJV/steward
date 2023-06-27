use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    morpho_aave_v2_a_token_adaptor_v1::MorphoAaveV2ATokenAdaptorV1Calls,
    morpho_aave_v2_debt_token_adaptor_v1::MorphoAaveV2DebtTokenAdaptorV1Calls,
    morpho_aave_v3_a_token_collateral_adaptor_v1::MorphoAaveV3ATokenCollateralAdaptorV1Calls,
    morpho_aave_v3_a_token_p2p_adaptor_v1::MorphoAaveV3ATokenP2PAdaptorV1Calls,
    morpho_aave_v3_debt_token_adaptor_v1::MorphoAaveV3DebtTokenAdaptorV1Calls,
};
use steward_proto::steward::{
    morpho_aave_v2_debt_token_adaptor_v1, morpho_aave_v2a_token_adaptor_v1,
    morpho_aave_v3_debt_token_adaptor_v1, morpho_aave_v3a_token_collateral_adaptor_v1,
    morpho_aave_v3a_token_p2p_adaptor_v1,
};

use crate::{
    error::Error,
    utils::{sp_call_error, sp_call_parse_address, string_to_u256},
};

pub(crate) fn morpho_aave_v2_a_token_adaptor_v1_calls(
    params: steward_proto::steward::MorphoAaveV2aTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_aave_v2a_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_aave_v2_a_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoAaveV2ATokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v2a_token_adaptor_v1::Function::DepositToAaveV2Morpho(p) => {
                let call =
                    steward_abi::morpho_aave_v2_a_token_adaptor_v1::DepositToAaveV2MorphoCall {
                        a_token: sp_call_parse_address(p.a_token)?,
                        amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                    };
                calls.push(
                    MorphoAaveV2ATokenAdaptorV1Calls::DepositToAaveV2Morpho(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v2a_token_adaptor_v1::Function::WithdrawFromAaveV2Morpho(p) => {
                let call =
                    steward_abi::morpho_aave_v2_a_token_adaptor_v1::WithdrawFromAaveV2MorphoCall {
                        a_token: sp_call_parse_address(p.a_token)?,
                        amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                    };
                calls.push(
                    MorphoAaveV2ATokenAdaptorV1Calls::WithdrawFromAaveV2Morpho(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_aave_v2_debt_token_adaptor_v1_calls(
    params: steward_proto::steward::MorphoAaveV2DebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_aave_v2_debt_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_aave_v2_debt_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoAaveV2DebtTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v2_debt_token_adaptor_v1::Function::BorrowFromAaveV2Morpho(p) => {
                let call =
                    steward_abi::morpho_aave_v2_debt_token_adaptor_v1::BorrowFromAaveV2MorphoCall {
                        a_token: sp_call_parse_address(p.a_token)?,
                        amount_to_borrow: string_to_u256(p.amount_to_borrow)?,
                    };
                calls.push(
                    MorphoAaveV2DebtTokenAdaptorV1Calls::BorrowFromAaveV2Morpho(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v2_debt_token_adaptor_v1::Function::RepayAaveV2MorphoDebt(p) => {
                let call =
                    steward_abi::morpho_aave_v2_debt_token_adaptor_v1::RepayAaveV2MorphoDebtCall {
                        a_token: sp_call_parse_address(p.a_token)?,
                        amount_to_repay: string_to_u256(p.amount_to_repay)?,
                    };
                calls.push(
                    MorphoAaveV2DebtTokenAdaptorV1Calls::RepayAaveV2MorphoDebt(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_aave_v3_a_token_collateral_adaptor_v1_calls(
    params: steward_proto::steward::MorphoAaveV3aTokenCollateralAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_aave_v3a_token_collateral_adaptor_v1::Function::RevokeApproval(p) => {
                let call =
                    steward_abi::morpho_aave_v3_a_token_collateral_adaptor_v1::RevokeApprovalCall {
                        asset: sp_call_parse_address(p.asset)?,
                        spender: sp_call_parse_address(p.spender)?,
                    };
                calls.push(
                    MorphoAaveV3ATokenCollateralAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3a_token_collateral_adaptor_v1::Function::DepositToAaveV3Morpho(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_collateral_adaptor_v1::DepositToAaveV3MorphoCall {
                    token_to_deposit: sp_call_parse_address(p.token_to_deposit)?,
                    amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                };
                calls.push(
                    MorphoAaveV3ATokenCollateralAdaptorV1Calls::DepositToAaveV3Morpho(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3a_token_collateral_adaptor_v1::Function::WithdrawFromAaveV3Morpho(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_collateral_adaptor_v1::WithdrawFromAaveV3MorphoCall {
                    token_to_withdraw: sp_call_parse_address(p.token_to_withdraw)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                };
                calls.push(
                    MorphoAaveV3ATokenCollateralAdaptorV1Calls::WithdrawFromAaveV3Morpho(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_aave_v3_a_token_p2p_adaptor_v1_calls(
    params: steward_proto::steward::MorphoAaveV3aTokenP2pAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_aave_v3a_token_p2p_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_p2p_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoAaveV3ATokenP2PAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3a_token_p2p_adaptor_v1::Function::DepositToAaveV3Morpho(p) => {
                let call =
                    steward_abi::morpho_aave_v3_a_token_p2p_adaptor_v1::DepositToAaveV3MorphoCall {
                        token_to_deposit: sp_call_parse_address(p.token_to_deposit)?,
                        amount_to_deposit: string_to_u256(p.amount_to_deposit)?,
                        max_iterations: string_to_u256(p.max_iterations)?,
                    };
                calls.push(
                    MorphoAaveV3ATokenP2PAdaptorV1Calls::DepositToAaveV3Morpho(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3a_token_p2p_adaptor_v1::Function::WithdrawFromAaveV3Morpho(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_p2p_adaptor_v1::WithdrawFromAaveV3MorphoCall {
                    token_to_withdraw: sp_call_parse_address(p.token_to_withdraw)?,
                    amount_to_withdraw: string_to_u256(p.amount_to_withdraw)?,
                    max_iterations: string_to_u256(p.max_iterations)?,
                };
                calls.push(
                    MorphoAaveV3ATokenP2PAdaptorV1Calls::WithdrawFromAaveV3Morpho(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_aave_v3_debt_token_adaptor_v1_calls(
    params: steward_proto::steward::MorphoAaveV3DebtTokenAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_aave_v3_debt_token_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_aave_v3_debt_token_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoAaveV3DebtTokenAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3_debt_token_adaptor_v1::Function::BorrowFromAaveV3Morpho(p) => {
                let call =
                    steward_abi::morpho_aave_v3_debt_token_adaptor_v1::BorrowFromAaveV3MorphoCall {
                        underlying: sp_call_parse_address(p.underlying)?,
                        amount_to_borrow: string_to_u256(p.amount_to_borrow)?,
                        max_iterations: string_to_u256(p.max_iterations)?,
                    };
                calls.push(
                    MorphoAaveV3DebtTokenAdaptorV1Calls::BorrowFromAaveV3Morpho(call)
                        .encode()
                        .into(),
                )
            }
            morpho_aave_v3_debt_token_adaptor_v1::Function::RepayAaveV3MorphoDebt(p) => {
                let call =
                    steward_abi::morpho_aave_v3_debt_token_adaptor_v1::RepayAaveV3MorphoDebtCall {
                        token_to_repay: sp_call_parse_address(p.token_to_repay)?,
                        amount_to_repay: string_to_u256(p.amount_to_repay)?,
                    };
                calls.push(
                    MorphoAaveV3DebtTokenAdaptorV1Calls::RepayAaveV3MorphoDebt(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}
