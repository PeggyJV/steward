// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {Address, ERC20, Owned} from "./interfaces.sol";

/**
 * @title Sommelier Cellar
 * @notice A composable ERC4626 that can use a set of other ERC4626 or ERC20 positions to earn yield.
 * @author Brian Le
 */
contract CellarV2_2 is Owned {
    using Address for address;

    constructor() Owned(msg.sender) {}

    // =========================================== POSITION LOGIC ===========================================

    /************* CELLAR V2 **************/
    struct AdaptorCall {
        address adaptor;
        bytes[] callData;
    }

    event CallOnAdaptor(AdaptorCall[] data);
    event Multicall();

    function multicall(bytes[] calldata data) external {
        for (uint256 i = 0; i < data.length; i++) address(this).functionDelegateCall(data[i]);
    }

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
