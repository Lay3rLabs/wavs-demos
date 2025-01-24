// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {ECDSAStakeRegistry} from "@eigenlayer/middleware/src/unaudited/ECDSAStakeRegistry.sol";

interface IWavsSDK {
    struct SignedPayload {
        bytes data;
        bytes signature;
    }

    /**
     * @notice The task structure
     * @param dataHash The hash of the data to verify
     * @param signatureData The signature(s) to verify
     */
    struct Task {
        bytes32 dataHash;
        bytes signatureData;
    }

    /// @notice The error thrown when a layer task is invalid
    error InvalidWavsTask();

    /**
     * @notice The stake registry contract
     * @return _stakeRegistry The stake registry address
     */
    function STAKE_REGISTRY() external view returns (ECDSAStakeRegistry _stakeRegistry); // solhint-disable-line func-name-mixedcase
}
