// SPDX-License-Identifier: LGPL-3.0-only
pragma solidity ^0.8.22;

import "@gnosis.pm/safe-contracts/contracts/common/Enum.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";
import "@gnosis.pm/safe-contracts/contracts/base/OwnerManager.sol";
import {WavsSDK} from "./WavsSDK.sol";
import {IWavsSDK} from "./interfaces/IWavsSDK.sol";

// TODO: there are a number of obvious improvements to make to this.
contract SafeModule is WavsSDK {
    // Address of the Gnosis Safe this module is connected to
    address public safe;

    // Store the owner who can use this module
    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "Only owner can call this function");
        _;
    }

    modifier onlySafe() {
        require(msg.sender == safe, "Only safe can call this function");
        _;
    }

    constructor(address _safe, address _stakeRegistry) WavsSDK(_stakeRegistry) {
        require(_safe != address(0), "Invalid safe address");
        safe = _safe;
        owner = msg.sender;
    }


    /// @dev Overrides _handleAddPayload to execute Safe transactions from signed payloads
    /// @param signedPayload The signed payload containing Safe transaction data
    function _handleAddPayload(IWavsSDK.SignedPayload calldata signedPayload) internal override {
        // Decode the transaction parameters from the payload data
        (address to, uint256 value, bytes memory data) = abi.decode(signedPayload.data, (address, uint256, bytes));
        
        require(to != address(0), "Invalid target address");

        // Execute the transaction from the Safe
        bool success = ModuleManager(safe).execTransactionFromModule(
            to,
            value,
            data,
            Enum.Operation.Call
        );

        require(success, "Module transaction failed");
    }
}
