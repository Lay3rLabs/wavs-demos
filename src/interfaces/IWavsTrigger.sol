// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

interface IWavsTrigger {
    event WavsTriggerEvent(bytes);

    // From ISimpleTrigger
    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }

    type TriggerId is uint64;

    function getTrigger(
        TriggerId triggerId
    ) external view returns (TriggerInfo memory);
}
