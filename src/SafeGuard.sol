// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "@gnosis.pm/safe-contracts/contracts/base/GuardManager.sol";
import "@gnosis.pm/safe-contracts/contracts/common/Enum.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";

contract SafeGuard is Guard, IServiceHandler {
    enum ValidationStatus {
        NotExists,
        Pending,
        Approved,
        Rejected,
        Expired
    }

    // Add validation timeout
    uint256 public constant VALIDATION_TIMEOUT = 1 hours;

    struct TransactionDetails {
        ValidationStatus status;
        string lastStatusMessage;
        uint256 validationExpiry;
        address to;
        uint256 value;
        bytes data;
        Enum.Operation operation;
    }

    // Address of the Gnosis Safe this guard is connected to
    address payable public immutable safe;

    // Address of the authorized service provider
    address public serviceProvider;
    // Flag to prevent re-initialization
    bool public initialized;

    // Validation state mappings
    mapping(bytes32 => TransactionDetails) public txDetails;

    event ValidationStatusUpdated(
        bytes32 indexed txHash,
        ValidationStatus status,
        string message
    );

    error AsyncValidationRequired();
    error TransactionExpired();

    modifier onlyServiceProvider() {
        require(
            msg.sender == serviceProvider,
            "Only service provider can call this function"
        );
        _;
    }

    constructor(address payable _safe) {
        require(_safe != address(0), "Invalid safe address");
        safe = _safe;
    }

    function initialize(address _serviceProvider) external {
        require(!initialized, "Already initialized");
        require(
            _serviceProvider != address(0),
            "Invalid service provider address"
        );

        serviceProvider = _serviceProvider;
        initialized = true;
    }

    function checkTransaction(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address payable refundReceiver,
        bytes memory signatures,
        address initiator
    ) external view override {
        require(msg.sender == address(safe), "Unauthorized");

        // Hash core transaction parameters (independent of nonce)
        bytes32 paramsHash = keccak256(abi.encode(to, value, data, operation));

        TransactionDetails storage details = txDetails[paramsHash];

        // If no validation exists yet, revert with AsyncValidationRequired
        if (details.status == ValidationStatus.NotExists) {
            revert AsyncValidationRequired();
        }

        // Only verify parameters if validation exists
        require(
            details.to == to &&
                details.value == value &&
                keccak256(details.data) == keccak256(data) &&
                details.operation == operation,
            "Transaction parameters mismatch"
        );

        if (details.status == ValidationStatus.Rejected) {
            revert("Transaction was rejected");
        }
        if (details.status == ValidationStatus.Approved) {
            if (block.timestamp > details.validationExpiry) {
                revert TransactionExpired();
            }
            return; // Allow execution if validated and not expired
        }

        // If pending or other status, revert
        revert AsyncValidationRequired();
    }

    function handleAddPayload(
        bytes calldata validationData,
        bytes calldata signature
    ) external override onlyServiceProvider {
        (
            address to,
            uint256 value,
            bytes memory data,
            Enum.Operation operation,
            bool approved,
            string memory message
        ) = abi.decode(
                validationData,
                (address, uint256, bytes, Enum.Operation, bool, string)
            );

        bytes32 paramsHash = keccak256(abi.encode(to, value, data, operation));

        ValidationStatus newStatus = approved
            ? ValidationStatus.Approved
            : ValidationStatus.Rejected;

        txDetails[paramsHash] = TransactionDetails({
            status: newStatus,
            lastStatusMessage: message,
            validationExpiry: approved
                ? block.timestamp + VALIDATION_TIMEOUT
                : 0,
            to: to,
            value: value,
            data: data,
            operation: operation
        });

        emit ValidationStatusUpdated(paramsHash, newStatus, message);
    }

    function getTransactionStatus(
        address to,
        uint256 value,
        bytes memory data,
        Enum.Operation operation
    )
        external
        view
        returns (
            ValidationStatus status,
            string memory message,
            uint256 remainingTime
        )
    {
        bytes32 paramsHash = keccak256(abi.encode(to, value, data, operation));
        TransactionDetails storage details = txDetails[paramsHash];

        if (details.status == ValidationStatus.NotExists) {
            return (ValidationStatus.NotExists, "", 0);
        }

        uint256 remaining = details.validationExpiry > block.timestamp
            ? details.validationExpiry - block.timestamp
            : 0;

        return (details.status, details.lastStatusMessage, remaining);
    }

    /// @dev Called after a transaction is executed
    /// @param txHash Hash of the transaction
    /// @param success True if the transaction was successful
    function checkAfterExecution(
        bytes32 txHash,
        bool success
    ) external override {
        require(msg.sender == address(safe), "Unauthorized");
        require(success, "Transaction failed");
        // Note: We don't clean up state here anymore since it's tied to parameters
        // not the specific transaction hash
    }

    /// @dev Returns whether the contract implements the given interface
    /// @param interfaceId The interface identifier
    /// @return true if the contract implements the interface
    function supportsInterface(
        bytes4 interfaceId
    ) external pure override returns (bool) {
        return interfaceId == type(Guard).interfaceId;
    }
}
