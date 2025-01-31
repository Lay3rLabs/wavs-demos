// SPDX-License-Identifier: LGPL-3.0
pragma solidity ^0.8.22;

import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {LMSRMarketMaker} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMaker.sol";
import {LMSRMarketMakerFactory} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMakerFactory.sol";
import {Whitelist} from "@lay3rlabs/conditional-tokens-market-makers/Whitelist.sol";
import {ConditionalTokens} from "@lay3rlabs/conditional-tokens-contracts/ConditionalTokens.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";
import {Clones} from "@openzeppelin/contracts/proxy/Clones.sol";
import {ERC20Mintable} from "./ERC20Mintable.sol";
import {console} from "forge-std/console.sol";

contract PredictionMarketFactory is LMSRMarketMakerFactory {
    address public oracle = msg.sender;

    function createConditionalTokenAndLMSRMarketMaker(
        string memory uri,
        bytes32 questionId,
        address collateralTokenAddress,
        uint64 fee,
        uint256 funding
    )
        external
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

    /**
     * @dev Handle the AVS oracle resolution event. This should close the market and payout the corresponding outcome tokens based on the result.
     */
    function resolveMarket(
        LMSRMarketMaker lmsrMarketMaker,
        ConditionalTokens conditionalTokens,
        bool result
    ) external {
        require(msg.sender == oracle, "Only the oracle can call this function");

        // pause the market maker, which this factory owns
        lmsrMarketMaker.pause();

        uint256[] memory payouts = new uint256[](2);
        // the first outcome slot is NO
        payouts[0] = result ? 0 : 1e18;
        // the second outcome slot is YES
        payouts[1] = result ? 1e18 : 0;

        // resolve the token
        conditionalTokens.reportPayouts(bytes32(0), payouts);
    }
}
