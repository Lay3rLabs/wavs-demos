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
        bytes32 indexed approvedHash,
        SafeGuard.ValidationStatus status
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
        guard = new SafeGuard(payable(address(safe)));
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

    function testGuardSetup() public view {
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
        // Fund the Safe with ETH
        vm.deal(address(safe), 2 ether);

        // Prepare transaction parameters
        (
            address to,
            uint256 value,
            bytes memory data,
            Enum.Operation operation
        ) = _getTestTransactionParams();

        // Get current nonce before execution
        uint256 nonce = safe.nonce();

        // Calculate the transaction hash using current nonce
        bytes32 txHash = safe.getTransactionHash(
            to,
            value,
            data,
            operation,
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            nonce // Use stored nonce
        );

        // Submit validation through service provider
        vm.prank(serviceProvider);
        _submitValidation(txHash, true);

        // Verify approved status
        (SafeGuard.ValidationStatus status, uint256 remainingTime) = guard
            .getTransactionStatus(txHash);
        assertEq(uint(status), uint(SafeGuard.ValidationStatus.Approved));
        assertTrue(remainingTime > 0);

        // Get signature for this transaction
        bytes memory signature = _signTransaction(txHash, ownerKey);

        // Now execute the transaction
        vm.startPrank(owner);
        safe.execTransaction(
            to,
            value,
            data,
            operation,
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            signature
        );
        vm.stopPrank();
    }

    // Helper functions to break down the complexity
    function _getTestTransactionParams()
        internal
        pure
        returns (
            address to,
            uint256 value,
            bytes memory data,
            Enum.Operation operation
        )
    {
        return (address(0x123), 1 ether, "", Enum.Operation.Call);
    }

    function _prepareTransactionHashAndSignature(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation
    ) internal view returns (bytes32 txHash, bytes memory signature) {
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
        return (txHash, signature);
    }

    function _executeTransaction(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        bytes memory signature
    ) internal {
        safe.execTransaction(
            to,
            value,
            data,
            operation,
            0, // safeTxGas
            0, // baseGas
            0, // gasPrice
            address(0), // gasToken
            payable(address(0)), // refundReceiver
            signature
        );
    }

    function _submitValidation(bytes32 approvedHash, bool approved) internal {
        SafeGuard.ValidationPayload memory payload = SafeGuard
            .ValidationPayload({
                approvedHash: approvedHash,
                approved: approved
            });

        bytes memory validationData = abi.encode(payload);
        guard.handleAddPayload(validationData, "");
    }

    function _verifyTransactionStatus(
        bytes32 txHash,
        SafeGuard.ValidationStatus expectedStatus,
        uint256 expectedRemainingTime
    ) internal view {
        (SafeGuard.ValidationStatus status, uint256 remainingTime) = guard
            .getTransactionStatus(txHash);

        assertEq(uint(status), uint(expectedStatus));
        if (expectedRemainingTime > 0) {
            assertTrue(remainingTime > 0);
        } else {
            assertEq(remainingTime, expectedRemainingTime);
        }
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
