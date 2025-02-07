// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/SafeModule.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";

contract DeploySafeModule is Script {
    Safe public safeSingleton;
    SafeProxyFactory public factory;
    address public deployedSafeAddress;
    address public deployedModuleAddress;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy Safe singleton and factory
        safeSingleton = new Safe();
        factory = new SafeProxyFactory();
        console.log("Deployed Safe singleton at:", address(safeSingleton));
        console.log("Deployed Safe factory at:", address(factory));

        // Deploy Safe if needed
        bool deployNewSafe = vm.envBool("DEPLOY_NEW_SAFE");
        if (deployNewSafe) {
            deployedSafeAddress = _deploySafe(
                _getOwners(),
                vm.envUint("SAFE_THRESHOLD"),
                vm.envAddress("SAFE_FALLBACK_HANDLER")
            );
            console.log("Deployed new Safe at:", deployedSafeAddress);

            // Fund the Safe
            payable(deployedSafeAddress).transfer(1 ether);
            console.log("Funded Safe with 1 ETH");
        } else {
            deployedSafeAddress = vm.envAddress("EXISTING_SAFE_ADDRESS");
            console.log("Using existing Safe at:", deployedSafeAddress);
        }

        // Deploy SafeModule
        SafeModule module = new SafeModule(deployedSafeAddress);
        deployedModuleAddress = address(module);
        console.log("Deployed SafeModule at:", deployedModuleAddress);

        // Fund the module
        try module.fundModule{value: 1 ether}() {
            console.log("Funded module with 1 ETH");
        } catch Error(string memory reason) {
            console.log("Failed to fund module:", reason);
            revert(reason);
        }

        _writeDeploymentToFile();

        vm.stopBroadcast();
    }

    function _getOwners() internal view returns (address[] memory) {
        string memory ownersRaw = vm.envString("SAFE_OWNERS");
        // Split the comma-separated string of addresses
        string[] memory ownerStrings = _split(ownersRaw, ",");

        address[] memory owners = new address[](ownerStrings.length);
        for (uint i = 0; i < ownerStrings.length; i++) {
            owners[i] = vm.parseAddress(ownerStrings[i]);
        }
        return owners;
    }

    function _deploySafe(
        address[] memory owners,
        uint256 threshold,
        address fallbackHandler
    ) internal returns (address) {
        // Use the deployed contracts instead of mainnet addresses
        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector,
            owners,
            threshold,
            address(0),
            "",
            fallbackHandler,
            address(0),
            0,
            payable(address(0))
        );

        address safeAddress = address(
            factory.createProxyWithNonce(address(safeSingleton), initializer, 0)
        );

        return safeAddress;
    }

    function _writeDeploymentToFile() internal {
        // Write JSON deployment info
        string memory deploymentInfo = string(
            abi.encodePacked(
                "{\n",
                '  "safeAddress": "',
                vm.toString(deployedSafeAddress),
                '",\n',
                '  "moduleAddress": "',
                vm.toString(deployedModuleAddress),
                '",\n',
                '  "timestamp": "',
                vm.toString(block.timestamp),
                '"\n',
                "}"
            )
        );
        // vm.writeFile("deployments.json", deploymentInfo);

        // Update .env file with new addresses
        string memory envPath = ".env";
        string memory currentEnv = vm.readFile(envPath);

        // Prepare new environment variables
        string memory moduleAddressVar = string.concat(
            "WAVS_SAFE_MODULE=",
            vm.toString(deployedModuleAddress)
        );
        string memory serviceHandlerVar = string.concat(
            "CLI_EIGEN_SERVICE_HANDLER=",
            vm.toString(deployedModuleAddress)
        );

        // Update or append to .env file
        string memory updatedEnv;
        if (bytes(currentEnv).length > 0) {
            // If .env exists, append to it
            updatedEnv = string.concat(
                currentEnv,
                "\n# Updated by deployment script\n",
                moduleAddressVar,
                "\n",
                serviceHandlerVar,
                "\n"
            );
        } else {
            // If .env doesn't exist, create it
            updatedEnv = string.concat(
                "# Generated by deployment script\n",
                moduleAddressVar,
                "\n",
                serviceHandlerVar,
                "\n"
            );
        }

        vm.writeFile(envPath, updatedEnv);

        console.log("\n=== Environment Variables Updated ===");
        console.log(moduleAddressVar);
        console.log(serviceHandlerVar);
    }

    function _split(
        string memory _str,
        string memory _delimiter
    ) internal pure returns (string[] memory) {
        uint count = 1;
        for (uint i = 0; i < bytes(_str).length; i++) {
            if (bytes(_str)[i] == bytes(_delimiter)[0]) count++;
        }

        string[] memory parts = new string[](count);
        count = 0;

        uint lastIndex = 0;
        for (uint i = 0; i < bytes(_str).length; i++) {
            if (bytes(_str)[i] == bytes(_delimiter)[0]) {
                parts[count] = _substring(_str, lastIndex, i);
                lastIndex = i + 1;
                count++;
            }
        }
        parts[count] = _substring(_str, lastIndex, bytes(_str).length);

        return parts;
    }

    function _substring(
        string memory _str,
        uint _start,
        uint _end
    ) internal pure returns (string memory) {
        bytes memory strBytes = bytes(_str);
        bytes memory result = new bytes(_end - _start);
        for (uint i = _start; i < _end; i++) {
            result[i - _start] = strBytes[i];
        }
        return string(result);
    }
}

contract InitializeSafeModule is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address moduleAddress = vm.envAddress("WAVS_SAFE_MODULE");
        address serviceProvider = vm.envAddress("SERVICE_MANAGER_ADDRESS");

        console.log("Module address:", moduleAddress);
        console.log("Service provider:", serviceProvider);

        SafeModule module = SafeModule(moduleAddress);
        require(address(module).code.length > 0, "No code at module address");

        vm.startBroadcast(deployerPrivateKey);

        try module.initialize(serviceProvider) {
            console.log("Successfully initialized SafeModule");
        } catch Error(string memory reason) {
            console.log("Initialization failed:", reason);
            revert(reason);
        }

        // Enable module on Safe if needed
        address safeAddress = module.safe();
        bool deployNewSafe = vm.envBool("DEPLOY_NEW_SAFE");
        if (deployNewSafe || vm.envBool("ENABLE_MODULE")) {
            Safe safe = Safe(payable(safeAddress));
            _enableModule(safe, moduleAddress);
            console.log("Enabled module on Safe at:", safeAddress);
        }

        vm.stopBroadcast();
    }

    function _enableModule(Safe safe, address moduleAddress) internal {
        // First check if the Safe exists and has code
        require(address(safe).code.length > 0, "No code at Safe address");

        // Try to get owners to verify it's a valid Safe
        try safe.getOwners() returns (address[] memory owners) {
            require(owners.length > 0, "Safe has no owners");

            bytes memory data = abi.encodeWithSelector(
                ModuleManager.enableModule.selector,
                moduleAddress
            );

            // Execute transaction to enable module
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
                _generateSingleSignature(safe)
            );
        } catch {
            revert(
                "Failed to interact with Safe - invalid Safe address or not deployed"
            );
        }
    }

    function _generateSingleSignature(
        Safe safe
    ) internal view returns (bytes memory) {
        // Assumes the deployer is the first owner
        address owner = safe.getOwners()[0];
        return abi.encodePacked(uint256(uint160(owner)), uint256(0), uint8(1));
    }
}

contract AddTrigger is Script {
    function run(string calldata triggerData) public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address moduleAddress = vm.envAddress("WAVS_SAFE_MODULE");

        console.log("Adding trigger to module at:", moduleAddress);
        console.log("Trigger data:", triggerData);

        SafeModule module = SafeModule(moduleAddress);
        require(address(module).code.length > 0, "No code at module address");
        require(module.initialized(), "Module not initialized");

        vm.startBroadcast(deployerPrivateKey);

        // Convert string to bytes
        bytes memory triggerBytes = bytes(triggerData);

        try module.addTrigger{value: 0.1 ether}(triggerBytes) {
            console.log("Successfully added trigger");
        } catch Error(string memory reason) {
            console.log("Failed to add trigger:", reason);
            revert(reason);
        }

        vm.stopBroadcast();
    }
}
