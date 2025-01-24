// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IConditionalTokens} from "../../src/interfaces/IConditionalTokens.sol";

contract MockConditionalTokens is IConditionalTokens {
    // Add events to match the real contract
    event ConditionPreparation(
        address indexed oracle,
        bytes32 indexed questionId,
        uint outcomeSlotCount
    );

    event ConditionResolution(bytes32 indexed questionId, uint[] payouts);

    function prepareCondition(
        address oracle,
        bytes32 questionId,
        uint outcomeSlotCount
    ) external override {
        emit ConditionPreparation(oracle, questionId, outcomeSlotCount);
    }

    function reportPayouts(
        bytes32 questionId,
        uint[] calldata payouts
    ) external override {
        emit ConditionResolution(questionId, payouts);
    }

    function getConditionId(
        address oracle,
        bytes32 questionId,
        uint outcomeSlotCount
    ) external pure override returns (bytes32) {
        return
            keccak256(abi.encodePacked(oracle, questionId, outcomeSlotCount));
    }

    function redeemPositions(
        address collateralToken,
        bytes32 parentCollectionId,
        bytes32 conditionId,
        uint[] calldata indexSets
    ) external override {
        // Mock implementation
    }
}
