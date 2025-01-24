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
    
    function isValidSignature(bytes32, bytes memory) external pure returns (bytes4) {
        return _ERC1271_MAGIC_VALUE;
    }
}

contract SafeGuardTest is Test {
    Safe public safe;
    SafeGuard public guard;
    MockStakeRegistry public stakeRegistry;
    
    address public owner;
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
        // Create owner account
        (owner, ownerKey) = makeAddrAndKey("owner");
        
        // Deploy mock stake registry
        stakeRegistry = new MockStakeRegistry();
        
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
            address(0), // to
            "", // data
            address(0), // fallbackHandler
            address(0), // paymentToken
            0, // payment
            payable(address(0)) // paymentReceiver
        );
        safe = Safe(payable(address(factory.createProxyWithNonce(
            address(safeSingleton),
            initializer,
            0 // salt nonce
        ))));
        
        // Deploy guard
        guard = new SafeGuard(address(safe), address(stakeRegistry));
        
        // Set guard directly through Safe transaction
        vm.startPrank(owner);
        
        // Prepare guard setup transaction
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
        
        console.log("Setting guard in setUp...");
        console.log("Safe address:", address(safe));
        console.log("Guard address:", address(guard));
        
        bool success = safe.execTransaction(
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
        
        console.log("Guard setup transaction success:", success);
        
        // Use the correct storage slot from SafeStorage
        bytes32 slot = bytes32(uint256(keccak256(abi.encode("guard_manager.guard.address"))) - 1);
        address storedGuard = address(uint160(uint256(vm.load(address(safe), slot))));
        console.log("Storage slot:", uint256(slot));
        console.log("Stored guard address:", storedGuard);
        
        vm.stopPrank();
    }

    function testGuardSetup() public {
        // Test 1: Verify the guard is properly set in the Safe's storage
        bytes32 slot = keccak256("guard_manager.guard.address");
        address storedGuard = address(uint160(uint256(vm.load(address(safe), slot))));
        assertEq(storedGuard, address(guard), "Guard not properly set in Safe storage");
        
        // Test 2: Verify the guard is configured with the correct Safe address
        assertEq(guard.safe(), address(safe), "Guard not configured with correct Safe address");
        
        // Test 3: Try to execute a transaction to verify the guard is actually active
        vm.startPrank(owner);
        bytes memory data = "";
        bytes32 txHash = safe.getTransactionHash(
            address(0x123),
            0,
            data,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            safe.nonce()
        );
        bytes memory signature = _signTransaction(txHash, ownerKey);
        
        vm.expectRevert(SafeGuard.AsyncValidationRequired.selector);
        safe.execTransaction(
            address(0x123),
            0,
            data,
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

    function testAsyncValidationFlow() public {
        (
            address to,
            uint256 value,
            bytes memory data,
            Enum.Operation operation,
            bytes32 txHash,
            bytes memory signature
        ) = _prepareTransaction();

        bytes32 expectedTxHash = _getExpectedTxHash(to, value, data, operation);
        
        console.log("Transaction hash from preparation:", uint256(txHash));
        console.log("Expected transaction hash:", uint256(expectedTxHash));
        
        // Try to execute transaction - this should trigger the guard
        vm.startPrank(owner);
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
        
        // Verify the transaction was recorded
        console.log("Verifying transaction status...");
        (
            SafeGuard.ValidationStatus status,
            string memory message,
            uint256 remainingTime
        ) = guard.getTransactionStatus(txHash);
        
        console.log("Status after execution:", uint(status));
        console.log("Message after execution:", message);
        console.log("Remaining time:", remainingTime);
        
        // Rest of the test...
    }

    // Helper functions for testAsyncValidationFlow
    function _prepareTransaction() internal returns (
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        bytes32 txHash,
        bytes memory signature
    ) {
        to = address(0x123);
        value = 1 ether;
        data = "";
        operation = Enum.Operation.Call;
        
        // Calculate guard's hash first - this is what we'll need to verify
        bytes32 guardHash = keccak256(
            abi.encodePacked(
                to,
                value,
                keccak256(data),
                uint8(operation),
                owner,
                block.timestamp
            )
        );
        
        // Get Safe's transaction hash for signing
        txHash = safe.getTransactionHash(
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
        );
        
        signature = _signTransaction(txHash, ownerKey);
        vm.deal(address(safe), 1 ether);
        
        // Return the guard's hash instead of Safe's hash
        return (to, value, data, operation, guardHash, signature);
    }

    function _getExpectedTxHash(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation
    ) internal view returns (bytes32) {
        // Match the exact encoding used in SafeGuard.checkTransaction
        return keccak256(
            abi.encodePacked(
                to,
                value,
                keccak256(data),
                uint8(operation),
                owner,
                block.timestamp
            )
        );
    }

    function _executeTransaction(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        bytes memory signature
    ) internal {
        vm.startPrank(owner);
        // This will trigger the guard's checkTransaction
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
    }

    function _verifyTransactionStatus(bytes32 txHash) internal {
        // Add debug logging
        console.log("Checking status for hash:", uint256(txHash));
        
        (
            SafeGuard.ValidationStatus status,
            string memory message,
            uint256 remainingTime
        ) = guard.getTransactionStatus(txHash);
        
        // Add debug logging
        console.log("Status:", uint(status));
        console.log("Message:", message);
        console.log("Remaining time:", remainingTime);
        
        assertEq(uint(status), uint(SafeGuard.ValidationStatus.Pending));
        assertEq(message, "Validation in progress");
        assertTrue(remainingTime > 0);
    }

    function _submitValidation(bytes32 txHash) internal {
        bytes memory validationData = abi.encode(txHash, true, "Approved");
        bytes memory validationSignature = _getValidSignature(validationData);
        
        IWavsSDK.SignedPayload memory payload = IWavsSDK.SignedPayload({
            data: validationData,
            signature: validationSignature
        });
        
        vm.expectEmit(true, true, true, true);
        emit ValidationStatusUpdated(
            txHash,
            SafeGuard.ValidationStatus.Approved,
            "Approved"
        );
        
        guard.addPayload(payload);
    }

    function testTransactionExpiration() public {
        // Similar to testAsyncValidationFlow but with time manipulation
        // ... implementation
    }

    // Helper functions
    function _setGuard(address _guard) internal {
        // Enable guard module first
        bytes memory data = abi.encodeWithSelector(
            ModuleManager.enableModule.selector,
            _guard
        );
        
        bytes32 txHash = safe.getTransactionHash(
            address(safe),
            0,
            data,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            safe.nonce()
        );
        
        bytes memory signature = _signTransaction(txHash, ownerKey);
        
        // Enable module first
        vm.prank(owner);
        safe.execTransaction(
            address(safe),
            0,
            data,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );
        
        // Then set guard
        data = abi.encodeWithSelector(
            GuardManager.setGuard.selector,
            _guard
        );
        
        txHash = safe.getTransactionHash(
            address(safe),
            0,
            data,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            safe.nonce()
        );
        
        signature = _signTransaction(txHash, ownerKey);
        
        vm.prank(owner);
        safe.execTransaction(
            address(safe),
            0,
            data,
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            signature
        );
    }

    function _signTransaction(bytes32 txHash, uint256 privateKey) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(privateKey, txHash);
        return abi.encodePacked(r, s, v);
    }

    function _getValidSignature(bytes memory data) internal pure returns (bytes memory) {
        bytes32 hash = keccak256(data);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(1, hash);
        return abi.encodePacked(r, s, v);
    }
}