// SPDX-License-Identifier: LGPL-3.0
pragma solidity ^0.8.22;

import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {LMSRMarketMaker} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMaker.sol";
import {LMSRMarketMakerFactory} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMakerFactory.sol";
import {Whitelist} from "@lay3rlabs/conditional-tokens-market-makers/Whitelist.sol";
import {ConditionalTokens} from "@lay3rlabs/conditional-tokens-contracts/ConditionalTokens.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ERC20Mintable} from "./ERC20Mintable.sol";
import {ISimpleTrigger} from "./interfaces/ISimpleTrigger.sol";

contract HyperstitionMarketFactory is
    LMSRMarketMakerFactory,
    IServiceHandler,
    ISimpleTrigger
{
    mapping(TriggerId => TriggerInfo) public triggersById;
    mapping(address => TriggerId[]) public triggerIdsByCreator;
    uint64 private _nextTriggerId; // Changed to uint64 to match TriggerId

    struct TriggerData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    event ResolveMarket(
        TriggerId indexed triggerId,
        address indexed creator,
        bytes data
    );

    struct AvsOutputData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    constructor() {
        implementationMaster = new LMSRMarketMaker(address(this));
    }

    function createConditionalTokenAndLMSRMarketMaker(
        string memory uri,
        bytes32 questionId,
        address collateralTokenAddress,
        uint64 fee,
        uint256 funding
    )
        public
        returns (
            ConditionalTokens conditionalTokens,
            LMSRMarketMaker lmsrMarketMaker
        )
    {
        ERC20Mintable collateralToken = ERC20Mintable(collateralTokenAddress);

        conditionalTokens = new ConditionalTokens(uri);
        conditionalTokens.prepareCondition(address(this), questionId, 2);
        bytes32 conditionId = conditionalTokens.getConditionId(
            address(this),
            questionId,
            2
        );

        bytes32[] memory conditionIds = new bytes32[](1);
        conditionIds[0] = conditionId;

        lmsrMarketMaker = LMSRMarketMaker(
            Clones.clone(address(implementationMaster))
        );
        lmsrMarketMaker.initialize(
            conditionalTokens,
            IERC20(address(collateralToken)),
            conditionIds,
            fee,
            Whitelist(address(0))
        );

        // Transfer funding to this factory
        collateralToken.transferFrom(msg.sender, address(this), funding);

        // Approve the market maker to spend the funding from this factory
        collateralToken.approve(address(lmsrMarketMaker), funding);

        // Add funding to the market maker, which will spend the funds from this factory
        lmsrMarketMaker.changeFunding(int256(funding));

        // Resume the market maker
        lmsrMarketMaker.resume();

        //! Retain ownership of the market maker so we can pause it once the oracle has resolved the question

        emit LMSRMarketMakerCreation(
            msg.sender,
            lmsrMarketMaker,
            conditionalTokens,
            IERC20(address(collateralToken)),
            conditionIds,
            fee,
            funding
        );
    }

    address public resolvedLmsrMarketMaker;
    address public resolvedConditionalTokens;
    bool public resolvedResult;

    /**
     * @dev Handle the AVS oracle resolution event. This should close the market and payout the corresponding outcome tokens based on the result.
     */
    function handleAddPayload(
        bytes calldata data,
        bytes calldata // signature
    ) external override {
        AvsOutputData memory returnData = abi.decode(data, (AvsOutputData));

        // pause the market maker, which this factory owns
        LMSRMarketMaker(returnData.lmsrMarketMaker).pause();

        uint256[] memory payouts = new uint256[](2);
        // the first outcome slot is YES
        payouts[0] = returnData.result ? 1e18 : 0;
        // the second outcome slot is NO
        payouts[1] = returnData.result ? 0 : 1e18;

        // resolve the token
        ConditionalTokens(returnData.conditionalTokens).reportPayouts(
            bytes32(0),
            payouts
        );
    }

    function getTrigger(
        TriggerId triggerId
    ) external view override returns (TriggerInfo memory) {
        return triggersById[triggerId];
    }

    function addTrigger(
        TriggerData calldata triggerData
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
        triggerIdsByCreator[msg.sender].push(triggerId);

        emit ResolveMarket(triggerId, msg.sender, data);
    }

    // Update helper functions
    function getTriggerCount(address creator) external view returns (uint256) {
        return triggerIdsByCreator[creator].length;
    }

    function getTriggerIdAtIndex(
        address creator,
        uint256 index
    ) external view returns (TriggerId) {
        require(
            index < triggerIdsByCreator[creator].length,
            "Index out of bounds"
        );
        return triggerIdsByCreator[creator][index];
    }
}
