// SPDX-License-Identifier: LGPL-3.0
pragma solidity ^0.8.22;

import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ConditionalTokens} from "@lay3rlabs/conditional-tokens-contracts/ConditionalTokens.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ISimpleTrigger} from "./interfaces/ISimpleTrigger.sol";
import {PredictionMarketFactory} from "./PredictionMarketFactory.sol";
import {LMSRMarketMaker} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMaker.sol";

// The contract responsible for triggering the oracle to resolve the market and handling the oracle output and instructing the market maker to resolve the market.
contract PredictionMarketOracleController is IServiceHandler, ISimpleTrigger {
    // The factory that handles creating and resolving the market.
    PredictionMarketFactory public factory;

    mapping(TriggerId => TriggerInfo) public triggersById;
    uint64 private _nextTriggerId; // Changed to uint64 to match TriggerId

    // The event that this contract emits to trigger the oracle AVS.
    event ResolveMarket(
        TriggerId indexed triggerId,
        address indexed creator,
        bytes data
    );

    // The data that is passed to the oracle AVS via the `data` field in the `ResolveMarket` event.
    struct TriggerInputData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    // The data that is returned from the oracle AVS.
    struct AvsOutputData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    constructor() {
        factory = new PredictionMarketFactory();
    }

    /**
     * @dev Handle the AVS oracle resolution event. This should close the market and payout the corresponding outcome tokens based on the result.
     * @param data The data returned from the oracle AVS.
     */
    function handleAddPayload(
        bytes calldata data,
        bytes calldata // signature
    ) external override {
        AvsOutputData memory returnData = abi.decode(data, (AvsOutputData));
        factory.resolveMarket(
            LMSRMarketMaker(returnData.lmsrMarketMaker),
            ConditionalTokens(returnData.conditionalTokens),
            returnData.result
        );
    }

    /**
     * @dev Trigger the oracle AVS to resolve the market.
     * @param triggerData The data to pass to the oracle AVS.
     * @return triggerId The ID of the trigger.
     */
    function addTrigger(
        TriggerInputData calldata triggerData
    ) external payable returns (TriggerId triggerId) {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        triggerId = TriggerId.wrap(uint64(_nextTriggerId++));
        bytes memory data = abi.encode(triggerData);

        TriggerInfo memory triggerInfo = TriggerInfo({
            triggerId: triggerId,
            creator: msg.sender,
            data: data
        });

        triggersById[triggerId] = triggerInfo;

        emit ResolveMarket(triggerId, msg.sender, data);
    }

    /**
     * @dev Get the trigger info for a given trigger ID.
     * @param triggerId The ID of the trigger to get the info for.
     * @return triggerInfo The trigger info.
     */
    function getTrigger(
        TriggerId triggerId
    ) external view override returns (TriggerInfo memory) {
        return triggersById[triggerId];
    }
}
