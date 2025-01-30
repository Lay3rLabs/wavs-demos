// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Test} from "forge-std/Test.sol";
import {NFTWithTrigger} from "../src/NFTWithTrigger.sol";
import {ISimpleTrigger} from "../src/interfaces/ISimpleTrigger.sol";

contract NFTWithTriggerTest is Test {
    NFTWithTrigger public nft;
    address public owner;
    address public serviceProvider;
    address public user;

    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri
    );

    function setUp() public {
        owner = makeAddr("owner");
        serviceProvider = makeAddr("serviceProvider");
        user = makeAddr("user");

        vm.prank(owner);
        nft = new NFTWithTrigger();
    }

    function test_InitializeCorrectly() public {
        // Check initial state
        assertFalse(nft.initialized());
        assertEq(nft.serviceProvider(), address(0));

        // Initialize contract
        vm.prank(owner);
        nft.initialize(serviceProvider);

        // Verify state after initialization
        assertTrue(nft.initialized());
        assertEq(nft.serviceProvider(), serviceProvider);
        assertTrue(nft.hasRole(nft.SERVICE_PROVIDER_ROLE(), serviceProvider));
    }

    function test_InitializeRevertsIfAlreadyInitialized() public {
        vm.prank(owner);
        nft.initialize(serviceProvider);

        vm.prank(owner);
        vm.expectRevert("Already initialized");
        nft.initialize(serviceProvider);
    }

    function test_InitializeRevertsWithZeroAddress() public {
        vm.prank(owner);
        vm.expectRevert("Invalid service provider");
        nft.initialize(address(0));
    }

    function test_AddTriggerAndHandlePayload() public {
        // Initialize contract
        vm.prank(owner);
        nft.initialize(serviceProvider);

        // Add trigger
        bytes memory triggerData = "test data";
        vm.deal(user, 1 ether);
        vm.prank(user);
        nft.addTrigger{value: 0.1 ether}(triggerData);

        // Get triggerId
        ISimpleTrigger.TriggerId triggerId = nft.getTriggerIdAtIndex(user, 0);

        // Prepare mint data
        string memory dataUri = "ipfs://test";
        bytes memory mintData = abi.encode(user, triggerId, dataUri);
        bytes memory signature = ""; // Empty signature for this test

        // Handle payload as service provider
        vm.prank(serviceProvider);
        vm.expectEmit(true, true, true, true);
        emit NFTMinted(user, 0, dataUri);
        nft.handleAddPayload(mintData, signature);

        // Verify NFT was minted
        assertEq(nft.ownerOf(0), user);
        assertEq(nft.tokenURI(0), dataUri);
    }

    function test_HandlePayloadRevertsForNonServiceProvider() public {
        vm.prank(owner);
        nft.initialize(serviceProvider);

        bytes memory mintData = "";
        bytes memory signature = "";

        vm.prank(user);
        vm.expectRevert("Only service provider");
        nft.handleAddPayload(mintData, signature);
    }

    function test_AddTriggerRequiresExactPayment() public {
        vm.deal(user, 1 ether);
        vm.prank(user);
        vm.expectRevert("Payment must be exactly 0.1 ETH");
        nft.addTrigger{value: 0.05 ether}("");
    }
}
