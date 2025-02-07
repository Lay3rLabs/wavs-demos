// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/SafeModule.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ISimpleTrigger} from "../src/interfaces/ISimpleTrigger.sol";

// Test ERC20 token
contract TestToken is ERC20 {
    constructor() ERC20("Test Token", "TEST") {
        _mint(msg.sender, 1000000e18);
    }
}

contract SafeModuleTest is Test {
    Safe public masterCopy;
    SafeProxyFactory public safeFactory;
    Safe public safe;
    SafeModule public safeModule;
    TestToken public token;

    address public owner;
    address public alice;
    address public bob;
    address public serviceProvider;

    event ExecutionSuccess();
    event NewTrigger(bytes triggerData);

    function setUp() public {
        // Setup accounts
        owner = address(this);
        alice = makeAddr("alice");
        bob = makeAddr("bob");
        serviceProvider = makeAddr("serviceProvider");

        // Deploy Safe contracts
        masterCopy = new Safe();
        safeFactory = new SafeProxyFactory();

        // Setup Safe
        address[] memory owners = new address[](1);
        owners[0] = owner;
        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector,
            owners,
            1,
            address(0),
            "",
            address(0),
            address(0),
            0,
            payable(address(0))
        );

        safe = Safe(
            payable(
                address(
                    safeFactory.createProxyWithNonce(
                        address(masterCopy),
                        initializer,
                        0
                    )
                )
            )
        );

        // Deploy module with new constructor
        safeModule = new SafeModule(address(safe));

        // Initialize the module
        safeModule.initialize(serviceProvider);

        // Enable module
        bytes memory enableModuleData = abi.encodeWithSelector(
            ModuleManager.enableModule.selector,
            address(safeModule)
        );

        bytes memory signature = abi.encodePacked(
            uint256(uint160(owner)),
            uint256(0),
            uint8(1)
        );

        safe.execTransaction(
            address(safe),
            0,
            enableModuleData,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );

        // Setup test token
        token = new TestToken();
        token.transfer(address(safe), 100e18);
    }

    function test_InitialSetup() public {
        assertEq(safeModule.owner(), address(this));
        assertEq(safeModule.safe(), address(safe));
        assertEq(safeModule.serviceProvider(), serviceProvider);
        assertTrue(safe.isModuleEnabled(address(safeModule)));
        assertEq(token.balanceOf(address(safe)), 100e18);
    }

    function test_ValidPayloadExecution() public {
        bytes memory transferData = abi.encodeWithSelector(
            IERC20.transfer.selector,
            alice,
            50e18
        );
        SafeModule.TransactionPayload memory txPayload = SafeModule
            .TransactionPayload({
                to: address(token),
                value: 0,
                data: transferData
            });
        bytes memory payload = abi.encode(txPayload);

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");

        assertEq(token.balanceOf(alice), 50e18);
    }

    function testFail_UnauthorizedServiceProvider() public {
        bytes memory transferData = abi.encodeWithSelector(
            IERC20.transfer.selector,
            alice,
            50e18
        );
        bytes memory payload = abi.encode(address(token), 0, transferData);

        vm.prank(alice); // Not the service provider
        safeModule.handleAddPayload(payload, "");
    }

    function testFail_ZeroAddressTarget() public {
        bytes memory payload = abi.encode(address(0), 0, "");

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");
    }

    function test_ETHTransfer() public {
        vm.deal(address(safe), 1 ether);
        uint256 initialBalance = alice.balance;

        SafeModule.TransactionPayload memory txPayload = SafeModule
            .TransactionPayload({to: alice, value: 0.5 ether, data: ""});
        bytes memory payload = abi.encode(txPayload);

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");

        assertEq(alice.balance, initialBalance + 0.5 ether);
        assertEq(address(safe).balance, 0.5 ether);
    }

    function test_TokenTransfer() public {
        bytes memory transferData = abi.encodeWithSelector(
            IERC20.transfer.selector,
            alice,
            50e18
        );
        SafeModule.TransactionPayload memory txPayload = SafeModule
            .TransactionPayload({
                to: address(token),
                value: 0,
                data: transferData
            });
        bytes memory payload = abi.encode(txPayload);

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");

        assertEq(token.balanceOf(alice), 50e18);
        assertEq(token.balanceOf(address(safe)), 50e18);
    }

    function test_BatchTransactions() public {
        bytes memory data1 = abi.encodeWithSelector(
            IERC20.transfer.selector,
            alice,
            25e18
        );
        bytes memory data2 = abi.encodeWithSelector(
            IERC20.transfer.selector,
            bob,
            25e18
        );

        SafeModule.TransactionPayload memory txPayload1 = SafeModule
            .TransactionPayload({to: address(token), value: 0, data: data1});
        SafeModule.TransactionPayload memory txPayload2 = SafeModule
            .TransactionPayload({to: address(token), value: 0, data: data2});

        bytes memory payload1 = abi.encode(txPayload1);
        bytes memory payload2 = abi.encode(txPayload2);

        vm.startPrank(serviceProvider);
        safeModule.handleAddPayload(payload1, "");
        safeModule.handleAddPayload(payload2, "");
        vm.stopPrank();

        assertEq(token.balanceOf(alice), 25e18);
        assertEq(token.balanceOf(bob), 25e18);
        assertEq(token.balanceOf(address(safe)), 50e18);
    }

    function testFail_InsufficientETHBalance() public {
        bytes memory payload = abi.encode(alice, 1 ether, "");

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");
    }

    function testFail_InsufficientTokenBalance() public {
        bytes memory transferData = abi.encodeWithSelector(
            IERC20.transfer.selector,
            alice,
            200e18
        );
        bytes memory payload = abi.encode(address(token), 0, transferData);

        vm.prank(serviceProvider);
        safeModule.handleAddPayload(payload, "");
    }

    function test_AddTrigger() public {
        bytes memory triggerData = "test trigger";

        vm.expectEmit(true, true, true, true);
        emit NewTrigger(
            abi.encode(
                ISimpleTrigger.TriggerInfo({
                    triggerId: ISimpleTrigger.TriggerId.wrap(1),
                    creator: address(this),
                    data: triggerData
                })
            )
        );

        safeModule.addTrigger{value: 0.1 ether}(triggerData);

        assertEq(address(safe).balance, 0.1 ether);

        // Verify trigger storage
        ISimpleTrigger.TriggerInfo memory info = safeModule.getTrigger(
            ISimpleTrigger.TriggerId.wrap(1)
        );
        assertEq(info.creator, address(this));
        assertEq(info.data, triggerData);
    }

    function testFail_AddTriggerIncorrectPayment() public {
        bytes memory triggerData = "test trigger";
        safeModule.addTrigger{value: 0.05 ether}(triggerData); // Should fail with incorrect payment
    }

    // Add new test for getTrigger
    function test_GetTrigger() public {
        bytes memory triggerData = "test trigger";
        safeModule.addTrigger{value: 0.1 ether}(triggerData);

        ISimpleTrigger.TriggerInfo memory info = safeModule.getTrigger(
            ISimpleTrigger.TriggerId.wrap(1)
        );
        assertEq(info.creator, address(this));
        assertEq(info.data, triggerData);
        assertEq(ISimpleTrigger.TriggerId.unwrap(info.triggerId), 1);
    }

    // Update test for triggerIdsByCreator
    function test_TriggerIdsByCreator() public {
        bytes memory triggerData1 = "test trigger 1";
        bytes memory triggerData2 = "test trigger 2";

        safeModule.addTrigger{value: 0.1 ether}(triggerData1);
        safeModule.addTrigger{value: 0.1 ether}(triggerData2);

        // Get all trigger IDs for this creator
        uint256 count = safeModule.getTriggerCount(address(this));
        assertEq(count, 2);

        // Verify each trigger ID
        ISimpleTrigger.TriggerId triggerId1 = safeModule.getTriggerIdAtIndex(
            address(this),
            0
        );
        ISimpleTrigger.TriggerId triggerId2 = safeModule.getTriggerIdAtIndex(
            address(this),
            1
        );

        assertEq(ISimpleTrigger.TriggerId.unwrap(triggerId1), 1);
        assertEq(ISimpleTrigger.TriggerId.unwrap(triggerId2), 2);
    }

    // Add a new test for getTriggerCount
    function test_GetTriggerCount() public {
        assertEq(safeModule.getTriggerCount(address(this)), 0);

        bytes memory triggerData = "test trigger";
        safeModule.addTrigger{value: 0.1 ether}(triggerData);

        assertEq(safeModule.getTriggerCount(address(this)), 1);
    }

    receive() external payable {}
}
