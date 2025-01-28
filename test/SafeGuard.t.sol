// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "forge-std/Test.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/libraries/SafeStorage.sol";
import "../src/SafeGuard.sol";
import "@gnosis.pm/safe-contracts/contracts/base/GuardManager.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "forge-std/console.sol";

// Mock stake registry without inheritance
contract MockStakeRegistry {
    bytes4 constant _ERC1271_MAGIC_VALUE = 0x1626ba7e;

    function operatorRegistered(address) external pure returns (bool) {
        return true;
    }

    function isValidSignature(
        bytes32,
        bytes memory
    ) external pure returns (bytes4) {
        return _ERC1271_MAGIC_VALUE;
    }
}

contract SafeGuardTest is Test {
    Safe public safe;
    SafeGuard public guard;

    address public owner;
    address public serviceProvider;
    uint256 public ownerKey;

    event WavsTriggerEvent(bytes);
    event ValidationRequired(
        bytes32 indexed txHash,
        address indexed to,
        uint256 value,
        bytes data,
        Enum.Operation operation,
        address initiator,
        uint256 estimatedProcessingTime
    );
    event ValidationStatusUpdated(
        bytes32 indexed txHash,
        SafeGuard.ValidationStatus status,
        string message
    );

    function setUp() public {
        // Create accounts
        (owner, ownerKey) = makeAddrAndKey("owner");
        serviceProvider = makeAddr("serviceProvider");

        // Deploy Safe singleton and factory
        Safe safeSingleton = new Safe();
        SafeProxyFactory factory = new SafeProxyFactory();

        // Deploy Safe
        address[] memory owners = new address[](1);
        owners[0] = owner;
        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector,
            owners,
            1, // threshold
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
                    factory.createProxyWithNonce(
                        address(safeSingleton),
                        initializer,
                        0
                    )
                )
            )
        );

        // Deploy guard and initialize
        guard = new SafeGuard(address(safe));
        guard.initialize(serviceProvider);

        // Set guard in Safe
        vm.startPrank(owner);
        bytes memory guardData = abi.encodeWithSelector(
            GuardManager.setGuard.selector,
            address(guard)
        );

        bytes32 txHash = safe.getTransactionHash(
            address(safe),
            0,
            guardData,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            safe.nonce()
        );

        bytes memory signature = _signTransaction(txHash, ownerKey);

        safe.execTransaction(
            address(safe),
            0,
            guardData,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );

        vm.stopPrank();
    }

    function testGuardSetup() public {
        assertEq(guard.safe(), address(safe));
        assertEq(guard.serviceProvider(), serviceProvider);

        // Verify guard is set in Safe
        bytes32 slot = keccak256("guard_manager.guard.address");
        address storedGuard = address(
            uint160(uint256(vm.load(address(safe), slot)))
        );
        assertEq(storedGuard, address(guard));
    }

    function testAsyncValidationFlow() public {
        // Prepare transaction
        address to = address(0x123);
        uint256 value = 1 ether;
        bytes memory data = "";
        Enum.Operation operation = Enum.Operation.Call;

        // Start recording logs BEFORE the transaction
        vm.recordLogs();

        // Execute transaction - should revert with AsyncValidationRequired
        vm.startPrank(owner);
        bytes memory signature = _signTransaction(
            safe.getTransactionHash(
                to,
                value,
                data,
                operation,
                0,
                0,
                0,
                address(0),
                payable(address(0)),
                safe.nonce()
            ),
            ownerKey
        );

        vm.expectRevert(SafeGuard.AsyncValidationRequired.selector);
        safe.execTransaction(
            to,
            value,
            data,
            operation,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );
        vm.stopPrank();

        // Get the actual transaction hash from the emitted event
        Vm.Log[] memory entries = vm.getRecordedLogs();

        // The ValidationRequired event should be the second event (index 1)
        // First event (index 0) is the WavsTriggerEvent
        bytes32 actualTxHash = entries[1].topics[1];

        // Rest of the test remains the same...
        (
            SafeGuard.ValidationStatus status,
            string memory message,
            uint256 remainingTime
        ) = guard.getTransactionStatus(actualTxHash);

        assertEq(uint(status), uint(SafeGuard.ValidationStatus.Pending));
        assertEq(message, "Validation in progress");
        assertTrue(remainingTime > 0);

        // Submit validation through service provider with correct hash
        bytes memory validationData = abi.encode(
            actualTxHash,
            true,
            "Approved"
        );

        vm.prank(serviceProvider);
        guard.handleAddPayload(validationData, "");

        // Verify approved status
        (status, message, remainingTime) = guard.getTransactionStatus(
            actualTxHash
        );
        assertEq(uint(status), uint(SafeGuard.ValidationStatus.Approved));
        assertEq(message, "Approved");
    }

    function testTransactionExpiration() public {
        // Prepare transaction
        address to = address(0x123);
        uint256 value = 1 ether;
        bytes memory data = "";
        Enum.Operation operation = Enum.Operation.Call;

        // Start recording logs BEFORE the transaction
        vm.recordLogs();

        vm.startPrank(owner);
        bytes memory signature = _signTransaction(
            safe.getTransactionHash(
                to,
                value,
                data,
                operation,
                0,
                0,
                0,
                address(0),
                payable(address(0)),
                safe.nonce()
            ),
            ownerKey
        );

        vm.expectRevert(SafeGuard.AsyncValidationRequired.selector);
        safe.execTransaction(
            to,
            value,
            data,
            operation,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );
        vm.stopPrank();

        // Get the actual transaction hash from the emitted event
        Vm.Log[] memory entries = vm.getRecordedLogs();
        bytes32 actualTxHash = entries[1].topics[1];

        // Move time forward past expiration
        vm.warp(block.timestamp + 3 minutes);

        // Try to validate expired transaction
        bytes memory validationData = abi.encode(
            actualTxHash,
            true,
            "Approved"
        );

        vm.expectRevert(SafeGuard.TransactionExpired.selector);
        vm.prank(serviceProvider);
        guard.handleAddPayload(validationData, "");

        // Verify expired status
        (
            SafeGuard.ValidationStatus status,
            string memory message,
            uint256 remainingTime
        ) = guard.getTransactionStatus(actualTxHash);

        assertEq(uint(status), uint(SafeGuard.ValidationStatus.Expired));
        assertEq(remainingTime, 0);
    }

    // Helper functions
    function _signTransaction(
        bytes32 txHash,
        uint256 privateKey
    ) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, txHash);
        return abi.encodePacked(r, s, v);
    }
}
