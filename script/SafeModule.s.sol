// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/SafeModule.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import "@gnosis.pm/safe-contracts/contracts/base/ModuleManager.sol";

contract SafeModuleScript is Script {
    // Remove or modify the constant addresses
    Safe public safeSingleton;
    SafeProxyFactory public factory;

    // Add state variables to store deployed addresses
    address public deployedSafeAddress;
    address public deployedModuleAddress;

    function setUp() public {}

    function deployContracts() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy Safe singleton and factory first
        safeSingleton = new Safe();
        factory = new SafeProxyFactory();
        console.log("Deployed Safe singleton at:", address(safeSingleton));
        console.log("Deployed Safe factory at:", address(factory));

        // Get Safe setup parameters from environment
        address[] memory owners = _getOwners();
        uint256 threshold = vm.envUint("SAFE_THRESHOLD");
        address fallbackHandler = vm.envAddress("SAFE_FALLBACK_HANDLER");

        // Check DEPLOY_NEW_SAFE first
        bool deployNewSafe = vm.envBool("DEPLOY_NEW_SAFE");
        console.log("Deploy new Safe:", deployNewSafe);

        if (deployNewSafe) {
            deployedSafeAddress = _deploySafe(
                owners,
                threshold,
                fallbackHandler
            );
            console.log("Deployed new Safe at:", deployedSafeAddress);
        } else {
            // Only try to read EXISTING_SAFE_ADDRESS if we're not deploying a new Safe
            try vm.envAddress("EXISTING_SAFE_ADDRESS") returns (
                address existingSafe
            ) {
                deployedSafeAddress = existingSafe;
                console.log("Using existing Safe at:", deployedSafeAddress);
            } catch {
                revert(
                    "When DEPLOY_NEW_SAFE is false, EXISTING_SAFE_ADDRESS must be set"
                );
            }
        }

        // Fund the Safe proxy (not the singleton or module) with 1 ETH
        payable(deployedSafeAddress).transfer(1 ether);
        console.log("Funded Safe proxy at", deployedSafeAddress, "with 1 ETH");

        // Deploy SafeModule with the Safe proxy address
        SafeModule module = new SafeModule(deployedSafeAddress);
        deployedModuleAddress = address(module);
        console.log("Deployed SafeModule at:", deployedModuleAddress);
        console.log("Module owner:", module.owner());
        console.log("Module safe:", module.safe());

        // Fund the module using receive() function
        try module.fundModule{value: 1 ether}() {
            console.log("Funded module with:", 1 ether, "wei (1 ETH)");
        } catch Error(string memory reason) {
            console.log("Failed to fund module:", reason);
            revert(reason);
        }

        // Write deployment info to files
        _writeDeploymentToFile();

        vm.stopBroadcast();
    }

    function initializeModule() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Get the service manager address from environment
        address serviceProvider = vm.envAddress("SERVICE_MANAGER_ADDRESS");
        require(
            serviceProvider != address(0),
            "Invalid service provider address"
        );

        // Get module address - try deployedModuleAddress first, fall back to env var
        address moduleAddress;
        if (deployedModuleAddress != address(0)) {
            moduleAddress = deployedModuleAddress;
            console.log("Using newly deployed module at:", moduleAddress);
        } else {
            moduleAddress = vm.envAddress("WAVS_SAFE_MODULE");
            console.log("Using existing module from env at:", moduleAddress);
        }
        require(moduleAddress != address(0), "No module address found");

        // Get the module instance and verify it exists
        SafeModule module = SafeModule(moduleAddress);
        require(address(module).code.length > 0, "No code at module address");

        // Get the Safe address from the module
        address safeAddress = module.safe();
        console.log("Safe address from module:", safeAddress);
        require(safeAddress != address(0), "Invalid Safe address from module");

        // Initialize the module
        try module.initialize(serviceProvider) {
            console.log(
                "Successfully initialized SafeModule with service provider:",
                serviceProvider
            );
        } catch Error(string memory reason) {
            console.log("Initialization failed with reason:", reason);
            revert(reason);
        }

        // If we're working with a new Safe, enable the module automatically
        bool deployNewSafe = vm.envBool("DEPLOY_NEW_SAFE");
        if (deployNewSafe) {
            Safe safe = Safe(payable(safeAddress));
            _enableModule(safe, moduleAddress);
            console.log("Enabled module on Safe");
        } else {
            // Try to get existing Safe address if needed
            try vm.envAddress("EXISTING_SAFE_ADDRESS") returns (
                address existingSafe
            ) {
                if (existingSafe != address(0)) {
                    require(
                        existingSafe == safeAddress,
                        "Safe address mismatch"
                    );
                    Safe safe = Safe(payable(safeAddress));
                    _enableModule(safe, moduleAddress);
                    console.log(
                        "Enabled module on existing Safe at:",
                        existingSafe
                    );
                } else {
                    console.log(
                        "Please enable the module manually through the Safe UI"
                    );
                }
            } catch {
                console.log(
                    "Please enable the module manually through the Safe UI"
                );
            }
        }

        vm.stopBroadcast();
    }

    // Update run function to include new method if needed
    function run() public {
        deployContracts();
        initializeModule();
        // Note: addNewTrigger() is not included in the default run
        // as it should be called separately when needed
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
}
