// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/SafeGuard.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";

contract SafeGuardScript is Script {
    Safe public safeSingleton;
    SafeProxyFactory public factory;

    function setUp() public {}

    function run() public {
        // Get deployment private key and start broadcasting
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Deploy Safe singleton and factory first if needed
        safeSingleton = new Safe();
        factory = new SafeProxyFactory();
        console.log("Deployed Safe singleton at:", address(safeSingleton));
        console.log("Deployed Safe factory at:", address(factory));

        // Get the service provider address from environment
        address serviceProvider = vm.envAddress("SERVICE_PROVIDER");

        // Get Safe setup parameters from environment
        address[] memory owners = _getOwners();
        uint256 threshold = vm.envUint("SAFE_THRESHOLD");
        address fallbackHandler = vm.envAddress("SAFE_FALLBACK_HANDLER");

        // Deploy new Safe if DEPLOY_NEW_SAFE is true
        address safeAddress;
        if (vm.envBool("DEPLOY_NEW_SAFE")) {
            safeAddress = _deploySafe(owners, threshold, fallbackHandler);
            console.log("Deployed new Safe at:", safeAddress);
        } else {
            safeAddress = vm.envAddress("EXISTING_SAFE_ADDRESS");
            console.log("Using existing Safe at:", safeAddress);
        }

        // Deploy SafeGuard with just the Safe address
        SafeGuard guard = new SafeGuard(safeAddress);
        console.log("Deployed SafeGuard at:", address(guard));

        // Initialize the guard with service provider
        guard.initialize(serviceProvider);
        console.log(
            "Initialized SafeGuard with service provider:",
            serviceProvider
        );

        // If we're working with a new Safe, set the guard automatically
        if (vm.envBool("DEPLOY_NEW_SAFE")) {
            Safe safe = Safe(payable(safeAddress));
            _setGuard(safe, address(guard));
            console.log("Set guard on Safe");
        } else {
            console.log("Please set the guard manually through the Safe UI");
        }

        vm.stopBroadcast();
    }

    function _deploySafe(
        address[] memory owners,
        uint256 threshold,
        address fallbackHandler
    ) internal returns (address) {
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

    function _setGuard(Safe safe, address guard) internal {
        bytes memory data = abi.encodeWithSelector(
            GuardManager.setGuard.selector,
            guard
        );

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
    }

    function _generateSingleSignature(
        Safe safe
    ) internal view returns (bytes memory) {
        address owner = safe.getOwners()[0];
        return abi.encodePacked(uint256(uint160(owner)), uint256(0), uint8(1));
    }

    function _getOwners() internal view returns (address[] memory) {
        string memory ownersRaw = vm.envString("SAFE_OWNERS");
        string[] memory ownerStrings = _split(ownersRaw, ",");

        address[] memory owners = new address[](ownerStrings.length);
        for (uint i = 0; i < ownerStrings.length; i++) {
            owners[i] = vm.parseAddress(ownerStrings[i]);
        }
        return owners;
    }

    // Helper functions for string manipulation
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
