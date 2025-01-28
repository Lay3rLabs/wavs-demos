// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "@gnosis.pm/safe-contracts/contracts/base/GuardManager.sol";
import "@gnosis.pm/safe-contracts/contracts/common/Enum.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {IWavsTrigger} from "./interfaces/IWavsTrigger.sol";

contract SafeGuard is Guard, IServiceHandler, IWavsTrigger {
    enum ValidationStatus {
        NotExists,
        Pending,
        Approved,
        Rejected,
        Expired
    }

    struct TransactionDetails {
        address to;
        uint256 value;
        bytes data;
        Enum.Operation operation;
        address initiator;
        uint256 timestamp;
        ValidationStatus status;
        string lastStatusMessage;
        uint256 expirationTime;
    }

    // Address of the Gnosis Safe this guard is connected to
    address public immutable safe;
    uint256 public estimatedProcessingTime = 2 minutes;

    // Address of the authorized service provider
    address public serviceProvider;
    // Flag to prevent re-initialization
    bool public initialized;

    // Validation state mappings
    mapping(bytes32 => bool) public validatedTxs;
    mapping(bytes32 => TransactionDetails) public txDetails;
    mapping(address => bytes32[]) public userPendingTxs;

    // Add new mappings for trigger storage
    mapping(TriggerId => TriggerInfo) private triggers;
    TriggerId private nextTriggerId;

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
        ValidationStatus status,
        string message
    );

    error AsyncValidationRequired();
    error TransactionExpired();
    error TransactionNotFound();

    modifier onlyServiceProvider() {
        require(
            msg.sender == serviceProvider,
            "Only service provider can call this function"
        );
        _;
    }

    constructor(address _safe) {
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
        address msgSender
    ) external override {
        require(msg.sender == safe, "Unauthorized");

        bytes32 txHash = keccak256(
            abi.encodePacked(
                to,
                value,
                data,
                operation,
                msgSender,
                block.timestamp
            )
        );

        if (validatedTxs[txHash]) {
            return;
        }

        if (
            txDetails[txHash].timestamp != 0 &&
            block.timestamp > txDetails[txHash].expirationTime
        ) {
            revert TransactionExpired();
        }

        // Initialize transaction details
        txDetails[txHash] = TransactionDetails({
            to: to,
            value: value,
            data: data,
            operation: operation,
            initiator: msgSender,
            timestamp: block.timestamp,
            status: ValidationStatus.Pending,
            lastStatusMessage: "Validation in progress",
            expirationTime: block.timestamp + estimatedProcessingTime
        });

        // Add to user's pending transactions
        userPendingTxs[msgSender].push(txHash);

        // Store trigger info before emitting event
        TriggerId triggerId = nextTriggerId;
        nextTriggerId = TriggerId.wrap(TriggerId.unwrap(nextTriggerId) + 1);

        triggers[triggerId] = TriggerInfo({
            triggerId: triggerId,
            creator: msgSender,
            data: abi.encode(
                "pre",
                txHash,
                to,
                value,
                data,
                operation,
                msgSender
            )
        });

        // Emit WAVS trigger with triggerId
        emit WavsTriggerEvent(triggers[triggerId].data);

        emit ValidationRequired(
            txHash,
            to,
            value,
            data,
            operation,
            msgSender,
            estimatedProcessingTime
        );

        emit ValidationStatusUpdated(
            txHash,
            ValidationStatus.Pending,
            "Validation in progress"
        );

        revert AsyncValidationRequired();
    }

    function checkAfterExecution(
        bytes32 txHash,
        bool success
    ) external override {
        require(msg.sender == safe, "Unauthorized");

        TransactionDetails storage details = txDetails[txHash];
        _removeFromPendingTxs(details.initiator, txHash);

        // Create and store post-execution trigger
        TriggerId triggerId = nextTriggerId;
        nextTriggerId = TriggerId.wrap(TriggerId.unwrap(nextTriggerId) + 1);

        triggers[triggerId] = TriggerInfo({
            triggerId: triggerId,
            creator: details.initiator,
            data: abi.encode("post", txHash, success)
        });

        // Emit WAVS trigger for post-execution validation
        emit WavsTriggerEvent(triggers[triggerId].data);

        delete txDetails[txHash];
        delete validatedTxs[txHash];
    }

    function handleAddPayload(
        bytes calldata data,
        bytes calldata signature
    ) external override onlyServiceProvider {
        (bytes32 txHash, bool isValid, string memory reason) = abi.decode(
            data,
            (bytes32, bool, string)
        );

        TransactionDetails storage details = txDetails[txHash];
        if (details.timestamp == 0) revert TransactionNotFound();

        if (block.timestamp > details.expirationTime) {
            details.status = ValidationStatus.Expired;
            emit ValidationStatusUpdated(
                txHash,
                ValidationStatus.Expired,
                reason
            );
            revert TransactionExpired();
        }

        details.status = isValid
            ? ValidationStatus.Approved
            : ValidationStatus.Rejected;
        details.lastStatusMessage = reason;

        if (isValid) {
            validatedTxs[txHash] = true;
        }

        emit ValidationStatusUpdated(txHash, details.status, reason);
    }

    function getTransactionStatus(
        bytes32 txHash
    )
        external
        view
        returns (
            ValidationStatus status,
            string memory message,
            uint256 remainingTime
        )
    {
        TransactionDetails storage details = txDetails[txHash];
        if (details.timestamp == 0) return (ValidationStatus.NotExists, "", 0);

        if (block.timestamp > details.expirationTime) {
            return (ValidationStatus.Expired, details.lastStatusMessage, 0);
        }

        uint256 remaining = details.expirationTime - block.timestamp;
        return (details.status, details.lastStatusMessage, remaining);
    }

    function getUserPendingTransactions(
        address user
    ) external view returns (bytes32[] memory) {
        return userPendingTxs[user];
    }

    function _removeFromPendingTxs(address user, bytes32 txHash) internal {
        bytes32[] storage pending = userPendingTxs[user];
        for (uint i = 0; i < pending.length; i++) {
            if (pending[i] == txHash) {
                pending[i] = pending[pending.length - 1];
                pending.pop();
                break;
            }
        }
    }

    /// @dev Returns whether the contract implements the given interface
    /// @param interfaceId The interface identifier
    /// @return true if the contract implements the interface
    function supportsInterface(
        bytes4 interfaceId
    ) external pure override returns (bool) {
        return interfaceId == type(Guard).interfaceId;
    }

    // Add new function to implement IWavsTrigger interface
    function getTrigger(
        TriggerId triggerId
    ) external view override returns (TriggerInfo memory) {
        return triggers[triggerId];
    }
}
