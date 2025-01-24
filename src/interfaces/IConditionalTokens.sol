// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IConditionalTokens {
    function prepareCondition(
        address oracle,
        bytes32 questionId,
        uint outcomeSlotCount
    ) external;

    function reportPayouts(
        bytes32 questionId,
        uint[] calldata payouts
    ) external;

    function getConditionId(
        address oracle,
        bytes32 questionId,
        uint outcomeSlotCount
    ) external pure returns (bytes32);

    function redeemPositions(
        address collateralToken,
        bytes32 parentCollectionId,
        bytes32 conditionId,
        uint[] calldata indexSets
    ) external;
}
