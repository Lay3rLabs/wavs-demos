// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "@gnosis.pm/safe-contracts/contracts/common/Enum.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@gnosis.pm/safe-contracts/contracts/base/OwnerManager.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ISimpleTrigger} from "./interfaces/ISimpleTrigger.sol";

contract SafeModule is IServiceHandler {
    // Address of the Gnosis Safe this module is connected to
    address public safe;

    // Store the owner who can use this module
    address public owner;

    // Address of the authorized service provider
    address public serviceProvider;

    // Flag to prevent re-initialization
    bool public initialized;

    struct Trigger {
        address creator;
        bytes data;
    }

    struct TransactionPayload {
        address to;
        uint256 value;
        bytes data;
    }

    mapping(ISimpleTrigger.TriggerId => Trigger) public triggersById;
    mapping(address => ISimpleTrigger.TriggerId[]) public triggerIdsByCreator;
    ISimpleTrigger.TriggerId public nextTriggerId;

    event NewTrigger(bytes);
    event Funded(address sender, uint256 amount);

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    modifier onlySafe() {
        require(msg.sender == safe, "Only safe can call this function");
        _;
    }

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
        owner = msg.sender;
    }

    function initialize(address _serviceProvider) external onlyOwner {
        require(!initialized, "Already initialized");
        require(
            _serviceProvider != address(0),
            "Invalid service provider address"
        );

        serviceProvider = _serviceProvider;
        initialized = true;
    }

    function fundModule() external payable {
        // Accept ETH funding
        emit Funded(msg.sender, msg.value);
    }

    function handleAddPayload(
        bytes calldata data,
        bytes calldata signature
    ) external override onlyServiceProvider {
        // Decode the transaction from the payload data
        TransactionPayload memory payload = abi.decode(
            data,
            (TransactionPayload)
        );

        require(payload.to != address(0), "Invalid target address");

        // Execute the transaction from the Safe
        bool success = ModuleManager(safe).execTransactionFromModule(
            payload.to,
            payload.value,
            payload.data,
            Enum.Operation.Call
        );

        require(success, "Module transaction failed");
    }

    function addTrigger(bytes memory data) external payable {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        // Forward the ETH to the Safe using low-level call
        (bool sent, ) = safe.call{value: msg.value}("");
        require(sent, "ETH transfer to Safe failed");

        // Get the next trigger id
        nextTriggerId = ISimpleTrigger.TriggerId.wrap(
            ISimpleTrigger.TriggerId.unwrap(nextTriggerId) + 1
        );
        ISimpleTrigger.TriggerId triggerId = nextTriggerId;

        // Create the trigger
        Trigger memory trigger = Trigger({creator: msg.sender, data: data});

        // Update storage
        triggersById[triggerId] = trigger;
        triggerIdsByCreator[msg.sender].push(triggerId);

        // Emit trigger info
        ISimpleTrigger.TriggerInfo memory triggerInfo = ISimpleTrigger
            .TriggerInfo({
                triggerId: triggerId,
                creator: trigger.creator,
                data: trigger.data
            });

        emit NewTrigger(abi.encode(triggerInfo));
    }

    function getTrigger(
        ISimpleTrigger.TriggerId triggerId
    ) external view returns (ISimpleTrigger.TriggerInfo memory) {
        Trigger storage trigger = triggersById[triggerId];

        return
            ISimpleTrigger.TriggerInfo({
                triggerId: triggerId,
                creator: trigger.creator,
                data: trigger.data
            });
    }

    function getTriggerCount(address creator) external view returns (uint256) {
        return triggerIdsByCreator[creator].length;
    }

    function getTriggerIdAtIndex(
        address creator,
        uint256 index
    ) external view returns (ISimpleTrigger.TriggerId) {
        require(
            index < triggerIdsByCreator[creator].length,
            "Index out of bounds"
        );
        return triggerIdsByCreator[creator][index];
    }
}
