// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.0;

import {ERC20, Ownable} from "./interfaces.sol";

contract MockAaveV2StablecoinCellar is Ownable {
    /**
     * @notice Whether or not the contract is shutdown in case of an emergency.
     */
    bool public isShutdown;

    event mockAccrue();
    event mockSendFees();
    event mockSetFeesDistributor(bytes32 newFeesDistributor);
    event mockEnterPosition(uint256 assets);
    event mockExitPosition(uint256 assets);
    event mockRebalance(address[9] route, uint256[3][4] swapParams, uint256 minAssetsOut);
    event mockReinvest(uint256 minAssetsOut);
    event mockClaimAndUnstake();
    event mockSweep(ERC20 token, address to);
    event mockSetLiquidityLimit(uint256 limit);
    event mockSetDepositLimit(uint256 limit);
    event mockSetTrust(ERC20 position, bool trust);
    event mockShutdownInitiated(bool emptyPosition);
    event mockShutdownLifted();

    constructor() {}

    /**
     * @notice Accrue yield, platform fees, and performance fees.
     */
    function accrue() external {
        emit mockAccrue();
    }

    /**
     * @notice Transfer accrued fees to the Sommelier Chain to distribute.
     */
    function sendFees() external onlyOwner {
        emit mockSendFees();
    }

    /**
     * @notice Set the address of the fee distributor on the Sommelier chain.
     */
    function setFeesDistributor(bytes32 newFeesDistributor) external onlyOwner {
        emit mockSetFeesDistributor(newFeesDistributor);
    }

    /**
     * @notice Pushes assets into the current Aave lending position.
     */
    function enterPosition(uint256 assets) external whenNotShutdown onlyOwner  {
        emit mockEnterPosition(assets);
    }

    /**
     * @notice Pulls assets from the current Aave lending position.
     */
    function exitPosition(uint256 assets) external whenNotShutdown onlyOwner  {
        emit mockExitPosition(assets);
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
    ) external whenNotShutdown onlyOwner  {
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
    function sweep(ERC20 token, address to) external onlyOwner {
        emit mockSweep(token, to);
    }

    /**
     * @notice Sets the maximum liquidity that cellar can manage. Careful to use the same decimals as the
     *         current asset.
     */
    function setLiquidityLimit(uint256 newLimit) external onlyOwner {
        emit mockSetLiquidityLimit(newLimit);
    }

    /**
     * @notice Sets the per-wallet deposit limit. Careful to use the same decimals as the current asset.
     */
    function setDepositLimit(uint256 newLimit) external onlyOwner {
        emit mockSetDepositLimit(newLimit);
    }

    /**
     * @notice Trust or distrust an asset position on Aave (eg. FRAX, UST, FEI).
     */
    function setTrust(ERC20 position, bool trust) external onlyOwner {
        emit mockSetTrust(position, trust);
    }

    /**
    * @notice Attempted action was prevented due to contract being shutdown.
    */
    error STATE_ContractShutdown();

    /**
     * @notice Prevent a function from being called during a shutdown.
     */
    modifier whenNotShutdown() {
        if (isShutdown) revert STATE_ContractShutdown();

        _;
    }

    /**
     * @notice Stop or start the contract. Used in an emergency or if the cellar has been retired.
     */
    function initiateShutdown(bool emptyPosition) external whenNotShutdown onlyOwner {
        isShutdown = true;

        emit mockShutdownInitiated(emptyPosition);
    }

    /**
     * @notice Restart the cellar.
     */
    function liftShutdown() external onlyOwner {
        isShutdown = false;

        emit mockShutdownLifted();
    }
}
