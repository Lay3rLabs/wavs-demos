// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "@gnosis.pm/safe-contracts/contracts/common/Enum.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@gnosis.pm/safe-contracts/contracts/base/OwnerManager.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";

contract SafeModule is IServiceHandler {
    // Address of the Gnosis Safe this module is connected to
    address public safe;

    // Store the owner who can use this module
    address public owner;

    // Address of the authorized service provider
    address public serviceProvider;

    // Flag to prevent re-initialization
    bool public initialized;

    event NewTrigger(bytes triggerData);

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

    function handleAddPayload(
        bytes calldata data,
        bytes calldata signature
    ) external override onlyServiceProvider {
        // Decode the transaction parameters from the payload data
        (address to, uint256 value, bytes memory txData) = abi.decode(
            data,
            (address, uint256, bytes)
        );

        require(to != address(0), "Invalid target address");

        // Execute the transaction from the Safe
        bool success = ModuleManager(safe).execTransactionFromModule(
            to,
            value,
            txData,
            Enum.Operation.Call
        );

        require(success, "Module transaction failed");
    }

    function addTrigger(string calldata triggerData) external payable {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        // Forward the ETH to the Safe using low-level call
        (bool sent, ) = safe.call{value: msg.value}("");
        require(sent, "ETH transfer to Safe failed");

        // Emit the trigger data as bytes
        emit NewTrigger(bytes(triggerData));
    }
}
