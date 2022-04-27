// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import "./interfaces.sol";

contract MockAaveV2StablecoinCellar is Ownable {
    /**
     * @notice Whether or not the contract is shutdown in case of an emergency.
     */
    bool public isShutdown;

    event mockAccrueFees();
    event mockTransferFees();
    event mockSetFeesDistributor(bytes32 newFeesDistributor);
    event mockEnterPosition();
    event mockRebalance(address[9] route, uint256[3][4] swapParams, uint256 minAssetsOut);
    event mockReinvest(uint256 minAssetsOut);
    event mockClaimAndUnstake();
    event mockSweep(address token, address to);
    event mockSetLiquidityLimit(uint256 limit);
    event mockSetDepositLimit(uint256 limit);
    event mockSetTrust(address position, bool trust);
    event mockSetShutdown(bool shutdown, bool existPosition);

    constructor() {}

    /**
     * @notice Take platform fees and performance fees off of cellar's active assets.
     */
    function accrueFees() external {
        emit mockAccrueFees();
    }

    /**
     * @notice Transfer accrued fees to the Sommelier Chain to distribute.
     */
    function transferFees() external onlyOwner {
        emit mockTransferFees();
    }

    /**
     * @notice Update the address of the fee distributor on the Sommelier Chain. IMPORTANT: Ensure
     *         that the address is formatted in the specific way that the Gravity contract expects
     *         it to be.
     */
    function setFeesDistributor(bytes32 newFeesDistributor) external onlyOwner {
        emit mockSetFeesDistributor(newFeesDistributor);
    }

    /**
     * @notice Enters into the current Aave stablecoin strategy.
     */
    function enterPosition() external onlyOwner  {
        emit mockEnterPosition();
    }

    /**
     * @notice Rebalances current assets into a new asset position.
     * @param route array of [initial token, pool, token, pool, token, ...] that specifies the swap route
     * @param swapParams multidimensional array of [i, j, swap type] where i and j are the correct
                         values for the n'th pool in `_route` and swap type should be 1 for a
                         stableswap `exchange`, 2 for stableswap `exchange_underlying`, 3 for a
                         cryptoswap `exchange`, 4 for a cryptoswap `exchange_underlying` and 5 for
                         Polygon factory metapools `exchange_underlying`
     * @param minAssetsOut minimum amount of assets received from swap
     */
    function rebalance(
        address[9] memory route,
        uint256[3][4] memory swapParams,
        uint256 minAssetsOut
    ) external onlyOwner  {
        emit mockRebalance(route, swapParams, minAssetsOut);
    }

    /**
     * @notice Reinvest rewards back into cellar's current position.
     * @dev Must be called within 2 day unstake period 10 days after `claimAndUnstake` was run.
     * @param minAssetsOut minimum amount of assets received after swapping AAVE to the current asset
     */
    function reinvest(uint256 minAssetsOut) external onlyOwner {
        emit mockReinvest(minAssetsOut);
    }

    /**
     * @notice Claim rewards from Aave and begin cooldown period to unstake them.
     * @return claimed amount of rewards claimed from Aave
     */
    function claimAndUnstake() external onlyOwner returns (uint256 claimed) {
        emit mockClaimAndUnstake();
        return 100;
    }

    /**
     * @notice Sweep tokens sent here that are not managed by the cellar.
     * @dev This may be used in case the wrong tokens are accidentally sent to this contract.
     * @param token address of token to transfer out of this cellar
     * @param to address to transfer sweeped tokens to
     */
    function sweep(address token, address to) external onlyOwner {
        emit mockSweep(token, to);
    }

    /**
     * @notice Sets the maximum liquidity that cellar can manage. Careful to use the same decimals as the
     *         current asset.
     */
    function setLiquidityLimit(uint256 limit) external onlyOwner {
        emit mockSetLiquidityLimit(limit);
    }

    /**
     * @notice Sets the per-wallet deposit limit. Careful to use the same decimals as the current asset.
     */
    function setDepositLimit(uint256 limit) external onlyOwner {
        emit mockSetDepositLimit(limit);
    }

    /**
     * @notice Trust or distrust an asset position on Aave (eg. FRAX, UST, FEI).
     */
    function setTrust(address position, bool trust) external onlyOwner {
        emit mockSetTrust(position, trust);
    }

    /**
     * @notice Stop or start the contract. Used in an emergency or if the cellar has been retired.
     */
    function setShutdown(bool shutdown, bool exitPosition) external onlyOwner {
        isShutdown = shutdown;

        emit mockSetShutdown(shutdown, exitPosition);
    }
}
