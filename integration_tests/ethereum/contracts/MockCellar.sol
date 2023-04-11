// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {Address, ERC20, Owned} from "./interfaces.sol";
import {Adaptor} from "./MockAdaptor.sol";

/**
 * @title Sommelier Cellar
 * @notice A composable ERC4626 that can use a set of other ERC4626 or ERC20 positions to earn yield.
 * @author Brian Le
 */
contract Cellar is Owned {
    using Address for address;

    constructor() Owned(msg.sender) {}

    // =========================================== POSITION LOGIC ===========================================

    /**
     * @notice Emitted on rebalancing positions.
     * @param fromPosition the address of the position rebalanced from
     * @param toPosition the address of the position rebalanced to
     * @param assetsFrom the amount of assets withdrawn from the position rebalanced from
     */
    event Rebalance(address indexed fromPosition, address indexed toPosition, uint256 assetsFrom, Exchange exchange);

    /**
     * @notice Emitted when trust for a position is changed.
     * @param position address of position that trust was changed for
     * @param isTrusted whether the position is trusted
     */
    event TrustChanged(address position, bool isTrusted);

    enum Exchange {
        UNIV2,
        UNIV3
    }

    /**
     * @notice Value specifying the interface a position uses.
     * @param ERC20 an ERC20 token
     * @param ERC4626 an ERC4626 vault
     * @param Cellar a cellar
     */
    enum PositionType {
        ERC20,
        ERC4626,
        Cellar
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

     /**
     * @notice Trust a position to be used by the cellar.
     * @param position address of position to trust
     * @param positionType value specifying the interface the position uses
     */
    function trustPosition(address position, PositionType positionType) external onlyOwner {
        emit TrustChanged(position, true);
    }

    /************* CELLAR V2 **************/
    struct AdaptorCall {
        address adaptor;
        bytes[] callData;
    }

    event CallOnAdaptor(AdaptorCall[] data);

    function callOnAdaptor(AdaptorCall[] memory data) external onlyOwner {
        for (uint8 i = 0; i < data.length; ++i) {
            address adaptor = data[i].adaptor;
            for (uint8 j = 0; j < data[i].callData.length; j++) {
                adaptor.functionDelegateCall(data[i].callData[j]);
            }
        }

        emit CallOnAdaptor(data);
    }
}
