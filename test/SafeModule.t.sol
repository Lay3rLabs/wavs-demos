// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/SafeModule.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

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
        bytes memory payload = abi.encode(address(token), 0, transferData);

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

        bytes memory payload = abi.encode(alice, 0.5 ether, "");

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
        bytes memory payload = abi.encode(address(token), 0, transferData);

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

        bytes memory payload1 = abi.encode(address(token), 0, data1);
        bytes memory payload2 = abi.encode(address(token), 0, data2);

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

    receive() external payable {}
}
