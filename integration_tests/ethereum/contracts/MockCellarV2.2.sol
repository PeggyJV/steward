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

    event AddAdaptorToCatalogue(address adaptor);

    function addAdaptorToCatalogue(address adaptor) external onlyOwner {
        emit AddAdaptorToCatalogue(adaptor);
    }

    event AddPositionToCatalogue(uint32 position);

    function addPositionToCatalogue(uint32 position) external onlyOwner {
        emit AddPositionToCatalogue(position);
    }

    /**
     * @notice Emitted when strategist platform fee cut is changed.
     * @param oldPlatformCut value strategist platform fee cut was changed from
     * @param newPlatformCut value strategist platform fee cut was changed to
     */
    event StrategistPlatformCutChanged(uint64 oldPlatformCut, uint64 newPlatformCut);

    /**
     * @notice Attempted to change strategist fee cut with invalid value.
     */
    error Cellar__InvalidFeeCut();

    /**
     * @notice Data related to fees.
     * @param strategistPlatformCut Determines how much platform fees go to strategist.
     *                              This should be a value out of 1e18 (ie. 1e18 represents 100%, 0 represents 0%).
     * @param platformFee The percentage of total assets accrued as platform fees over a year.
                          This should be a value out of 1e18 (ie. 1e18 represents 100%, 0 represents 0%).
     * @param strategistPayoutAddress Address to send the strategists fee shares.
     */
    struct FeeData {
        uint64 strategistPlatformCut;
        uint64 platformFee;
        uint64 lastAccrual;
        address strategistPayoutAddress;
    }

    /**
     * @notice Stores all fee data for cellar.
     */
    FeeData public feeData =
        FeeData({
            strategistPlatformCut: 0.75e18,
            platformFee: 0.01e18,
            lastAccrual: 0,
            strategistPayoutAddress: address(0)
        });

    /**
     * @notice Sets the max possible fee cut for this cellar.
     */
    uint64 public constant MAX_FEE_CUT = 1e18;

    /**
     * @notice Sets the Strategists cut of platform fees
     * @param cut the platform cut for the strategist
     */
    function setStrategistPlatformCut(uint64 cut) external onlyOwner {
        if (cut > MAX_FEE_CUT) revert Cellar__InvalidFeeCut();
        emit StrategistPlatformCutChanged(feeData.strategistPlatformCut, cut);

        feeData.strategistPlatformCut = cut;
    }
}
