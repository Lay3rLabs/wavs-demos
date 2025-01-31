// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/SafeModule.sol";

contract TriggerScript is Script {
    function addTrigger(bytes calldata triggerData) public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        // Deal ETH to the script before broadcast
        vm.deal(address(this), 1 ether);

        // Log pre-broadcast state
        console.log("Starting trigger addition...");
        console.log("Trigger data length:", triggerData.length);

        vm.startBroadcast(deployerPrivateKey);

        // Get the existing module address from environment
        address existingModuleAddress = vm.envAddress("WAVS_SAFE_MODULE");
        require(existingModuleAddress != address(0), "Invalid module address");

        console.log("Module address:", existingModuleAddress);
        console.log("Sender:", msg.sender);
        console.log("Sender balance:", msg.sender.balance);
        console.log("Script balance:", address(this).balance);

        // Check if we have enough ETH
        require(
            msg.sender.balance >= 0.1 ether,
            "Insufficient ETH balance in sender"
        );
        require(
            address(this).balance >= 0.1 ether,
            "Insufficient ETH balance in script"
        );

        // Get the module instance
        SafeModule module = SafeModule(existingModuleAddress);

        // Verify the module exists first
        require(address(module).code.length > 0, "No code at module address");
        console.log("Module code exists at address");

        // Add more detailed error handling for safe() call
        try module.safe() returns (address safeAddress) {
            console.log("Safe address:", safeAddress);
            require(safeAddress != address(0), "Safe address is zero");
        } catch (bytes memory) {
            console.log(
                "Module code size:",
                address(existingModuleAddress).code.length
            );
            console.log(
                "Raw call to module failed - likely wrong address or undeployed contract"
            );
            revert(
                "Failed to read Safe address - module might not exist or not be deployed"
            );
        }

        // Add verification that module is actually initialized
        try module.initialized() returns (bool isInitialized) {
            console.log("Module initialized:", isInitialized);
            require(isInitialized, "Module not initialized");
        } catch {
            revert("Failed to check module initialization");
        }

        try module.addTrigger{value: 0.1 ether}(triggerData) {
            console.log("Successfully added trigger");
        } catch Error(string memory reason) {
            console.log("Transaction failed with reason:", reason);
            revert(reason);
        } catch Panic(uint errorCode) {
            string memory panicReason = getPanicReason(errorCode);
            console.log("Transaction failed with panic:", panicReason);
            revert(panicReason);
        } catch (bytes memory returnData) {
            console.log(
                "Transaction failed with raw data:",
                vm.toString(returnData)
            );
            revert("Transaction failed with raw data");
        }

        vm.stopBroadcast();
    }

    function getPanicReason(
        uint errorCode
    ) internal pure returns (string memory) {
        if (errorCode == 0x01) return "Assertion failed";
        if (errorCode == 0x11) return "Arithmetic overflow/underflow";
        if (errorCode == 0x12) return "Division by zero";
        if (errorCode == 0x21) return "Invalid enum value";
        if (errorCode == 0x22)
            return "Storage write to incorrectly encoded storage slot";
        if (errorCode == 0x31) return "Pop on empty array";
        if (errorCode == 0x32) return "Array index out of bounds";
        if (errorCode == 0x41) return "Memory allocation overflow";
        if (errorCode == 0x51) return "Zero initialized variable";
        return string(abi.encodePacked("Panic code: ", vm.toString(errorCode)));
    }

    function run() public {
        // Empty run function as we'll call addTrigger directly
    }
}
