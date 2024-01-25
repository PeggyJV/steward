use std::convert::TryInto;

use ethers::{abi::AbiEncode, types::Bytes};
use steward_abi::{
    morpho_aave_v2_a_token_adaptor_v1::MorphoAaveV2ATokenAdaptorV1Calls,
    morpho_aave_v2_debt_token_adaptor_v1::MorphoAaveV2DebtTokenAdaptorV1Calls,
    morpho_aave_v3_a_token_collateral_adaptor_v1::MorphoAaveV3ATokenCollateralAdaptorV1Calls,
    morpho_aave_v3_a_token_p2p_adaptor_v1::MorphoAaveV3ATokenP2PAdaptorV1Calls,
    morpho_aave_v3_debt_token_adaptor_v1::MorphoAaveV3DebtTokenAdaptorV1Calls,
    morpho_blue_collateral_adaptor_v1::MorphoBlueCollateralAdaptorV1Calls, morpho_blue_debt_adaptor_v1::MorphoBlueDebtAdaptorV1Calls, morpho_blue_supply_adaptor_v1::MorphoBlueSupplyAdaptorV1Calls,
};
use steward_proto::steward::{
    morpho_aave_v2_debt_token_adaptor_v1, morpho_aave_v2a_token_adaptor_v1,
    morpho_aave_v3_debt_token_adaptor_v1, morpho_aave_v3a_token_collateral_adaptor_v1,
    morpho_aave_v3a_token_p2p_adaptor_v1, morpho_blue_collateral_adaptor_v1, morpho_blue_debt_adaptor_v1, morpho_blue_supply_adaptor_v1,
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
            morpho_aave_v2a_token_adaptor_v1::Function::Claim(p) => {
                let call = steward_abi::morpho_aave_v2_a_token_adaptor_v1::ClaimCall {
                    claimable: string_to_u256(p.claimable)?,
                    proof: parse_proof_bytes(p.proof)?,
                };
                calls.push(
                    MorphoAaveV2ATokenAdaptorV1Calls::Claim(call)
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
            morpho_aave_v3a_token_collateral_adaptor_v1::Function::Claim(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_collateral_adaptor_v1::ClaimCall {
                    claimable: string_to_u256(p.claimable)?,
                    proof: parse_proof_bytes(p.proof)?,
                };
                calls.push(
                    MorphoAaveV3ATokenCollateralAdaptorV1Calls::Claim(call)
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
            morpho_aave_v3a_token_p2p_adaptor_v1::Function::Claim(p) => {
                let call = steward_abi::morpho_aave_v3_a_token_p2p_adaptor_v1::ClaimCall {
                    claimable: string_to_u256(p.claimable)?,
                    proof: parse_proof_bytes(p.proof)?,
                };
                calls.push(
                    MorphoAaveV3ATokenP2PAdaptorV1Calls::Claim(call)
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

pub(crate) fn morpho_blue_collateral_adaptor_v1_calls(
    params: steward_proto::steward::MorphoBlueCollateralAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_blue_collateral_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_blue_collateral_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoBlueCollateralAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_collateral_adaptor_v1::Function::AddCollateral(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_collateral_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };
                let call = steward_abi::morpho_blue_collateral_adaptor_v1::AddCollateralCall {
                    market, 
                    collateral_to_deposit: string_to_u256(p.collateral_to_deposit)?,
                };
                calls.push(
                    MorphoBlueCollateralAdaptorV1Calls::AddCollateral(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_collateral_adaptor_v1::Function::RemoveCollateral(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_collateral_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };
                let call =
                    steward_abi::morpho_blue_collateral_adaptor_v1::RemoveCollateralCall {
                        market,
                        collateral_amount: string_to_u256(p.collateral_amount)?, 
                    };
                calls.push(
                    MorphoBlueCollateralAdaptorV1Calls::RemoveCollateral(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_blue_debt_adaptor_v1_calls(
    params: steward_proto::steward::MorphoBlueDebtAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_blue_debt_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_blue_debt_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoBlueDebtAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_debt_adaptor_v1::Function::BorrowFromMorphoBlue(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_debt_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };  
                let call = steward_abi::morpho_blue_debt_adaptor_v1::BorrowFromMorphoBlueCall {
                    market, 
                    amount_to_borrow: string_to_u256(p.amount_to_borrow)?,
                };
                calls.push(
                    MorphoBlueDebtAdaptorV1Calls::BorrowFromMorphoBlue(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_debt_adaptor_v1::Function::RepayMorphoBlueDebt(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_debt_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };

                let call =
                    steward_abi::morpho_blue_debt_adaptor_v1::RepayMorphoBlueDebtCall {
                        market,
                        debt_token_repay_amount: string_to_u256(p.debt_token_repay_amount)?, 
                    };
                calls.push(
                    MorphoBlueDebtAdaptorV1Calls::RepayMorphoBlueDebt(call)
                        .encode()
                        .into(),
                )
            }
        }
    }

    Ok(calls)
}

pub(crate) fn morpho_blue_supply_adaptor_v1_calls(
    params: steward_proto::steward::MorphoBlueSupplyAdaptorV1Calls,
) -> Result<Vec<Bytes>, Error> {
    let mut calls = Vec::new();
    for c in params.calls {
        let function = c
            .function
            .ok_or_else(|| sp_call_error("function cannot be empty".to_string()))?;

        match function {
            morpho_blue_supply_adaptor_v1::Function::RevokeApproval(p) => {
                let call = steward_abi::morpho_blue_supply_adaptor_v1::RevokeApprovalCall {
                    asset: sp_call_parse_address(p.asset)?,
                    spender: sp_call_parse_address(p.spender)?,
                };
                calls.push(
                    MorphoBlueSupplyAdaptorV1Calls::RevokeApproval(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_supply_adaptor_v1::Function::LendToMorphoBlue(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_supply_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };

                let call =
                    steward_abi::morpho_blue_supply_adaptor_v1::LendToMorphoBlueCall {
                        market,
                        assets: string_to_u256(p.assets)?, 
                    };
                calls.push(
                    MorphoBlueSupplyAdaptorV1Calls::LendToMorphoBlue(call)
                        .encode()
                        .into(),
                )
            }
            morpho_blue_supply_adaptor_v1::Function::WithdrawFromMorphoBlue(p) => {
                let market = p.market.unwrap();
                let market = steward_abi::morpho_blue_supply_adaptor_v1::MarketParams {
                    loan_token: sp_call_parse_address(market.loan_token)?,
                    collateral_token: sp_call_parse_address(market.collateral_token)?,
                    oracle: sp_call_parse_address(market.oracle)?,
                    irm: sp_call_parse_address(market.irm)?,
                    lltv: string_to_u256(market.lltv)?,
                };  
                let call = steward_abi::morpho_blue_supply_adaptor_v1::WithdrawFromMorphoBlueCall {
                    market, 
                    assets: string_to_u256(p.assets)?,
                };
                calls.push(
                    MorphoBlueSupplyAdaptorV1Calls::WithdrawFromMorphoBlue(call)
                        .encode()
                        .into(),
                )
            } 
        }
    }

    Ok(calls)
}

fn parse_proof_bytes(proof: Vec<Vec<u8>>) -> Result<Vec<[u8; 32]>, Error> {
    proof
        .iter()
        .map(Vec::as_slice)
        .map(TryInto::try_into)
        .collect::<Result<_, std::array::TryFromSliceError>>()
        .map_err(|e| sp_call_error(format!("failed to parse proof bytes: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proof_bytes() {
        let valid_proof = vec![
            // 32 bytes
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            // 32 bytes
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        ];
        let invalid_proof = vec![
            // 31 bytes
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00,
            ],
            // 32 bytes
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        ];

        assert!(parse_proof_bytes(valid_proof).is_ok());

        let err_res = parse_proof_bytes(invalid_proof);
        assert!(err_res.is_err());
        assert_eq!(
            "SP call error: failed to parse proof bytes: could not convert slice to array",
            &err_res.unwrap_err().to_string()
        );
    }
}
