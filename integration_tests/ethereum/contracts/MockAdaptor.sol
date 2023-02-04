// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {ERC20, Owned} from "./interfaces.sol";
import {Cellar} from "./MockCellar.sol";

contract Adaptor is Owned {
    event ClaimCompAndSwap(ERC20 assetOut, Cellar.Exchange exchange, bytes params, uint64 slippage);
    event SwapAndRepay(ERC20 tokenIn, ERC20 tokenToRepay, uint256 amountIn, Cellar.Exchange exchange, bytes params);
    event BorrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow);

    constructor() Owned(msg.sender) {}

    function borrowFromAave(address debtTokenToBorrow, uint256 amountToBorrow) external onlyOwner {
        emit BorrowFromAave(debtTokenToBorrow, amountToBorrow);
    }

    // Mocks the function parameters of the same name from CompoundCTokenAdaptor.sol
    function claimCompAndSwap(
        ERC20 assetOut,
        Cellar.Exchange exchange,
        bytes memory params,
        uint64 slippage
    ) external onlyOwner {
        if (exchange == Cellar.Exchange.UNIV2) {
            address[] memory path = abi.decode(params, (address[]));
        }
        if (exchange == Cellar.Exchange.UNIV3) {
            (address[] memory path, uint24[] memory poolFees) = abi.decode(params, (address[], uint24[]));
        }
        emit ClaimCompAndSwap(assetOut, exchange, params, slippage);
    }

    // Mocks the function parameters of the same name from AaveDebtTokenAdaptor.sol
    function swapAndRepay(
        ERC20 tokenIn,
        ERC20 tokenToRepay,
        uint256 amountIn,
        Cellar.Exchange exchange,
        bytes memory params
    ) external onlyOwner {
        if (exchange == Cellar.Exchange.UNIV2) {
            (address[] memory path, uint256 amount, uint256 amountOutMin) = abi.decode(
                params,
                (address[], uint256, uint256)
            );
        }
        if (exchange == Cellar.Exchange.UNIV3) {
            (address[] memory path, uint24[] memory poolFees, uint256 amount, uint256 amountOutMin) = abi.decode(
                params,
                (address[], uint24[], uint256, uint256)
            );
        }
        emit SwapAndRepay(tokenIn, tokenToRepay, amountIn, exchange, params);
    }
}
