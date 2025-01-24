// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {WavsSDK} from "./WavsSDK.sol";
import {IWavsSDK} from "./interfaces/IWavsSDK.sol";
import {IConditionalTokens} from "./interfaces/IConditionalTokens.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract WavsConditionalMarket is WavsSDK {
    // State variables
    IConditionalTokens public immutable conditionalTokens;
    mapping(bytes32 => bool) public marketResolved;
    mapping(bytes32 => uint256) public marketOutcome;

    // Events
    event MarketCreated(
        bytes32 indexed questionId,
        bytes32 indexed conditionId
    );
    event MarketResolved(
        bytes32 indexed questionId,
        bytes32 indexed conditionId,
        uint256 outcome
    );

    // Errors
    error MarketAlreadyResolved();
    error InvalidOutcome();
    error InvalidCollateralToken();

    struct MarketResolutionPayload {
        bytes32 questionId;
        bytes32 conditionId;
        uint256 outcome;
    }

    constructor(
        address _stakeRegistry,
        address _conditionalTokens
    ) WavsSDK(_stakeRegistry) {
        conditionalTokens = IConditionalTokens(_conditionalTokens);
    }

    /**
     * @notice Creates a new conditional tokens market
     * @param collateralToken The ERC20 token used as collateral
     * @param questionId Unique identifier for the market question
     * @param outcomeSlotCount Number of possible outcomes (typically 2 for binary markets)
     */
    function createMarket(
        IERC20 collateralToken,
        bytes32 questionId,
        uint outcomeSlotCount
    ) external {
        // Prepare condition
        bytes32 conditionId = conditionalTokens.getConditionId(
            address(this),
            questionId,
            outcomeSlotCount
        );

        // Create the condition in the Conditional Tokens contract
        conditionalTokens.prepareCondition(
            address(this),
            questionId,
            outcomeSlotCount
        );

        emit MarketCreated(questionId, conditionId);
    }

    /**
     * @notice Handles the resolution payload from WAVS
     * @param signedPayload The signed payload containing market resolution data
     */
    function _handleAddPayload(
        IWavsSDK.SignedPayload calldata signedPayload
    ) internal override {
        MarketResolutionPayload memory resolution = abi.decode(
            signedPayload.data,
            (MarketResolutionPayload)
        );

        if (marketResolved[resolution.questionId]) {
            revert MarketAlreadyResolved();
        }

        if (resolution.outcome > 1) {
            // Assuming binary markets (0 or 1)
            revert InvalidOutcome();
        }

        // Store the outcome
        marketResolved[resolution.questionId] = true;
        marketOutcome[resolution.questionId] = resolution.outcome;

        // Report the outcome to Conditional Tokens contract
        conditionalTokens.reportPayouts(
            resolution.questionId,
            _generatePayoutNumerators(resolution.outcome)
        );

        emit MarketResolved(
            resolution.questionId,
            resolution.conditionId,
            resolution.outcome
        );
    }

    /**
     * @notice Generates payout numerators for binary markets
     * @param outcome The winning outcome (0 or 1)
     */
    function _generatePayoutNumerators(
        uint256 outcome
    ) internal pure returns (uint[] memory) {
        uint[] memory payoutNumerators = new uint[](2);
        payoutNumerators[outcome] = 1;
        return payoutNumerators;
    }
}
