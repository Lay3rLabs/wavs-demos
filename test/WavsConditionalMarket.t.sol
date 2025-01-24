// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {WavsConditionalMarket} from "../src/WavsConditionalMarket.sol";
import {console} from "forge-std/console.sol";
import {IConditionalTokens} from "../src/interfaces/IConditionalTokens.sol";
import {IWavsSDK} from "../src/interfaces/IWavsSDK.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {MockERC20} from "./mocks/MockERC20.sol";
import {MockConditionalTokens} from "./mocks/MockConditionalTokens.sol";

contract WavsConditionalMarketTest is Test {
    WavsConditionalMarket public market;
    MockERC20 public collateral;
    MockConditionalTokens public conditionalTokens;

    address public constant STAKE_REGISTRY = address(0x1);
    bytes32 public constant QUESTION_ID = keccak256("Who will win?");
    uint256 public constant OUTCOME_SLOT_COUNT = 2;

    event MarketCreated(
        bytes32 indexed questionId,
        bytes32 indexed conditionId
    );
    event MarketResolved(
        bytes32 indexed questionId,
        bytes32 indexed conditionId,
        uint256 outcome
    );

    function setUp() public {
        collateral = new MockERC20("Test Token", "TEST");
        conditionalTokens = new MockConditionalTokens();
        market = new WavsConditionalMarket(
            STAKE_REGISTRY,
            address(conditionalTokens)
        );
    }

    function test_CreateMarket() public {
        bytes32 expectedConditionId = conditionalTokens.getConditionId(
            address(market),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        vm.expectEmit(true, true, false, true);
        emit MarketCreated(QUESTION_ID, expectedConditionId);

        market.createMarket(
            IERC20(address(collateral)),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );
    }

    function test_ResolveMarket() public {
        // First create the market
        market.createMarket(
            IERC20(address(collateral)),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        bytes32 conditionId = conditionalTokens.getConditionId(
            address(market),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        // Create resolution payload
        WavsConditionalMarket.MarketResolutionPayload
            memory payload = WavsConditionalMarket.MarketResolutionPayload({
                questionId: QUESTION_ID,
                conditionId: conditionId,
                outcome: 1
            });

        bytes memory encodedPayload = abi.encode(payload);

        // Add debug logging
        console.log("Question ID:", uint256(QUESTION_ID));
        console.log("Condition ID:", uint256(conditionId));
        console.log("Outcome:", payload.outcome);

        // Create signed payload with valid signature
        bytes32 messageHash = keccak256(encodedPayload);
        bytes32 ethSignedMessageHash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash)
        );

        console.log("Message Hash:", uint256(messageHash));
        console.log("Eth Signed Message Hash:", uint256(ethSignedMessageHash));

        // Create a valid signature
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, ethSignedMessageHash);
        bytes memory signature = abi.encodePacked(r, s, v);

        IWavsSDK.SignedPayload memory signedPayload = IWavsSDK.SignedPayload({
            data: encodedPayload,
            signature: signature
        });

        // Get the signer's address
        address signer = vm.addr(1);
        console.log("Signer address:", signer);

        // Mock the stake registry to authorize our signer
        vm.mockCall(
            STAKE_REGISTRY,
            abi.encodeWithSignature("isStaker(address)", signer),
            abi.encode(true)
        );

        vm.prank(signer);
        vm.expectEmit(true, true, false, true);
        emit MarketResolved(QUESTION_ID, conditionId, 1);

        // Add try-catch to see revert reason
        try market.addPayload(signedPayload) {
            console.log("addPayload succeeded");
        } catch Error(string memory reason) {
            console.log("addPayload failed with reason:", reason);
        } catch (bytes memory) {
            console.log("addPayload failed with low-level error");
        }

        assertTrue(market.marketResolved(QUESTION_ID));
        assertEq(market.marketOutcome(QUESTION_ID), 1);
    }

    function testFail_ResolveMarketTwice() public {
        // First create and resolve the market
        test_ResolveMarket();

        // Try to resolve again with the same payload
        bytes32 conditionId = conditionalTokens.getConditionId(
            address(market),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        WavsConditionalMarket.MarketResolutionPayload
            memory payload = WavsConditionalMarket.MarketResolutionPayload({
                questionId: QUESTION_ID,
                conditionId: conditionId,
                outcome: 1
            });

        bytes memory encodedPayload = abi.encode(payload);
        bytes memory signature = new bytes(65);
        IWavsSDK.SignedPayload memory signedPayload = IWavsSDK.SignedPayload({
            data: encodedPayload,
            signature: signature
        });

        vm.prank(address(0x123));
        market.addPayload(signedPayload); // Should revert with MarketAlreadyResolved
    }

    function testFail_InvalidOutcome() public {
        // Create market
        market.createMarket(
            IERC20(address(collateral)),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        bytes32 conditionId = conditionalTokens.getConditionId(
            address(market),
            QUESTION_ID,
            OUTCOME_SLOT_COUNT
        );

        // Try to resolve with invalid outcome (2)
        WavsConditionalMarket.MarketResolutionPayload
            memory payload = WavsConditionalMarket.MarketResolutionPayload({
                questionId: QUESTION_ID,
                conditionId: conditionId,
                outcome: 2 // Invalid outcome
            });

        bytes memory encodedPayload = abi.encode(payload);
        bytes memory signature = new bytes(65);
        IWavsSDK.SignedPayload memory signedPayload = IWavsSDK.SignedPayload({
            data: encodedPayload,
            signature: signature
        });

        vm.prank(address(0x123));
        market.addPayload(signedPayload); // Should revert with InvalidOutcome
    }
}
