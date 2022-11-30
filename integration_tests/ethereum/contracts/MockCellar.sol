// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {ERC20, Owned} from "./interfaces.sol";

/**
 * @title Sommelier Cellar
 * @notice A composable ERC4626 that can use a set of other ERC4626 or ERC20 positions to earn yield.
 * @author Brian Le
 */
contract Cellar is Owned {
    constructor() Owned(msg.sender) {}

    // =========================================== POSITION LOGIC ===========================================

    /**
     * @notice Emitted on rebalancing positions.
     * @param fromPosition the address of the position rebalanced from
     * @param toPosition the address of the position rebalanced to
     * @param assetsFrom the amount of assets withdrawn from the position rebalanced from
     */
    event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom, Exchange exchange);

    enum Exchange {
        UNIV2,
        UNIV3
    }

    /**
     * @notice Move assets between positions. To move assets from/to this cellar's holdings, specify
     *         the address of this cellar as the `fromPosition`/`toPosition`.
     * @param fromPosition address of the position to move assets from
     * @param toPosition address of the position to move assets to
     * @param assetsFrom amount of assets to move from the from position
     */
    function rebalance(
        address fromPosition,
        address toPosition,
        uint256 assetsFrom,
        Exchange exchange,
        bytes calldata params
    ) external onlyOwner returns (uint256 assetsTo) {
        if (exchange == Exchange.UNIV2) {
            (address[] memory path, uint256 amount, uint256 amountOutMin) = abi.decode(
                params,
                (address[], uint256, uint256)
            );
        }
        if (exchange == Exchange.UNIV3) {
            (address[] memory path, uint24[] memory poolFees, uint256 amount, uint256 amountOutMin) = abi.decode(
                params,
                (address[], uint24[], uint256, uint256)
            );
        }
        emit Rebalance(fromPosition, toPosition, assetsFrom, exchange);
    }
}
