// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/SafeModule.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../src/interfaces/IWavsSDK.sol";

// Mock ECDSAStakeRegistry that can toggle signature validation
contract MockECDSAStakeRegistry {
    bytes4 constant ERC1271_MAGIC_VALUE = 0x1626ba7e;
    bytes4 constant ERC1271_FAIL_VALUE = 0xffffffff;

    bool public shouldValidate = true;

    function setShouldValidate(bool _shouldValidate) external {
        shouldValidate = _shouldValidate;
    }

    function isValidSignature(bytes32, bytes memory) public view returns (bytes4) {
        return shouldValidate ? ERC1271_MAGIC_VALUE : ERC1271_FAIL_VALUE;
    }

    function operatorRegistered(address) public pure returns (bool) {
        return true;
    }
}

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
    MockECDSAStakeRegistry public stakeRegistry;

    address public owner;
    address public alice;
    address public bob;

    event ExecutionSuccess();

    function setUp() public {
        // Setup accounts
        owner = address(this);
        alice = makeAddr("alice");
        bob = makeAddr("bob");

        // Deploy Safe contracts
        masterCopy = new Safe();
        safeFactory = new SafeProxyFactory();

        // Setup Safe
        address[] memory owners = new address[](1);
        owners[0] = owner;
        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector, owners, 1, address(0), "", address(0), address(0), 0, payable(address(0))
        );

        safe = Safe(payable(address(safeFactory.createProxyWithNonce(address(masterCopy), initializer, 0))));

        // Deploy stake registry mock
        stakeRegistry = new MockECDSAStakeRegistry();

        // Deploy module
        safeModule = new SafeModule(address(safe), address(stakeRegistry));

        // Enable module
        bytes memory enableModuleData = abi.encodeWithSelector(ModuleManager.enableModule.selector, address(safeModule));

        bytes memory signature = abi.encodePacked(uint256(uint160(owner)), uint256(0), uint8(1));

        safe.execTransaction(
            address(safe), 0, enableModuleData, Enum.Operation.Call, 0, 0, 0, address(0), payable(address(0)), signature
        );

        // Setup test token
        token = new TestToken();
        token.transfer(address(safe), 100e18);
    }

    function createDummyTask() internal pure returns (IWavsSDK.Task memory) {
        return IWavsSDK.Task({dataHash: bytes32(0x0), signatureData: new bytes(65)});
    }

    // Basic setup tests
    function test_InitialSetup() public {
        assertEq(safeModule.owner(), address(this));
        assertEq(safeModule.safe(), address(safe));
        assertTrue(safe.isModuleEnabled(address(safeModule)));
        assertEq(token.balanceOf(address(safe)), 100e18);
    }

    // Helper function to create a signed payload
    function createSignedPayload(
        address target,
        uint256 value,
        bytes memory data
    ) internal pure returns (IWavsSDK.SignedPayload memory) {
        return IWavsSDK.SignedPayload({
            data: abi.encode(target, value, data),
            signature: new bytes(65)
        });
    }

    // Update tests to use new interface
    function test_ValidPayloadExecution() public {
        bytes memory transferData = abi.encodeWithSelector(IERC20.transfer.selector, alice, 50e18);
        IWavsSDK.SignedPayload memory payload = createSignedPayload(address(token), 0, transferData);

        stakeRegistry.setShouldValidate(true);
        safeModule.addPayload(payload);

        assertEq(token.balanceOf(alice), 50e18);
    }

    function testFail_InvalidPayloadExecution() public {
        bytes memory transferData = abi.encodeWithSelector(IERC20.transfer.selector, alice, 50e18);
        IWavsSDK.SignedPayload memory payload = createSignedPayload(address(token), 0, transferData);

        stakeRegistry.setShouldValidate(false);
        safeModule.addPayload(payload);
    }

    function testFail_ZeroAddressTarget() public {
        IWavsSDK.SignedPayload memory payload = createSignedPayload(address(0), 0, "");
        safeModule.addPayload(payload);
    }

    function test_ETHTransfer() public {
        vm.deal(address(safe), 1 ether);
        uint256 initialBalance = alice.balance;

        IWavsSDK.SignedPayload memory payload = createSignedPayload(alice, 0.5 ether, "");
        safeModule.addPayload(payload);

        assertEq(alice.balance, initialBalance + 0.5 ether);
        assertEq(address(safe).balance, 0.5 ether);
    }

    function test_TokenTransfer() public {
        bytes memory transferData = abi.encodeWithSelector(IERC20.transfer.selector, alice, 50e18);
        IWavsSDK.SignedPayload memory payload = createSignedPayload(address(token), 0, transferData);

        safeModule.addPayload(payload);

        assertEq(token.balanceOf(alice), 50e18);
        assertEq(token.balanceOf(address(safe)), 50e18);
    }

    function test_BatchTransactions() public {
        bytes memory data1 = abi.encodeWithSelector(IERC20.transfer.selector, alice, 25e18);
        bytes memory data2 = abi.encodeWithSelector(IERC20.transfer.selector, bob, 25e18);

        IWavsSDK.SignedPayload[] memory payloads = new IWavsSDK.SignedPayload[](2);
        payloads[0] = createSignedPayload(address(token), 0, data1);
        payloads[1] = createSignedPayload(address(token), 0, data2);

        safeModule.addPayloadMulti(payloads);

        assertEq(token.balanceOf(alice), 25e18);
        assertEq(token.balanceOf(bob), 25e18);
        assertEq(token.balanceOf(address(safe)), 50e18);
    }

    function testFail_InsufficientETHBalance() public {
        IWavsSDK.SignedPayload memory payload = createSignedPayload(alice, 1 ether, "");
        safeModule.addPayload(payload);
    }

    function testFail_InsufficientTokenBalance() public {
        bytes memory transferData = abi.encodeWithSelector(IERC20.transfer.selector, alice, 200e18);
        IWavsSDK.SignedPayload memory payload = createSignedPayload(address(token), 0, transferData);
        safeModule.addPayload(payload);
    }

    receive() external payable {}
}
