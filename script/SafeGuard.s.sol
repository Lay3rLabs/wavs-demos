// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import "forge-std/Script.sol";
import "../src/SafeGuard.sol";
import "@gnosis.pm/safe-contracts/contracts/Safe.sol";
import "@gnosis.pm/safe-contracts/contracts/proxies/SafeProxyFactory.sol";

contract SafeGuardScript is Script {
    // Known contract addresses - replace with actual addresses for different networks
    address constant SAFE_SINGLETON_MAINNET =
        0x41675C099F32341bf84BFc5382aF534df5C7461a;
    address constant SAFE_FACTORY_MAINNET =
        0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67;

    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        // Get the stake registry address from environment
        address stakeRegistry = vm.envAddress("STAKE_REGISTRY");

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

        // Deploy SafeGuard
        SafeGuard guard = new SafeGuard(safeAddress, stakeRegistry);
        console.log("Deployed SafeGuard at:", address(guard));

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

    function _getOwners() internal view returns (address[] memory) {
        string memory ownersRaw = vm.envString("SAFE_OWNERS");
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
        Safe safeSingleton = Safe(payable(SAFE_SINGLETON_MAINNET));
        SafeProxyFactory factory = SafeProxyFactory(SAFE_FACTORY_MAINNET);

        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector,
            owners,
            threshold,
            address(0), // to
            "", // data
            fallbackHandler,
            address(0), // payment token
            0, // payment
            payable(address(0)) // payment receiver
        );

        address safeAddress = address(
            factory.createProxyWithNonce(address(safeSingleton), initializer, 0)
        );

        return safeAddress;
    }

    function _setGuard(Safe safe, address guard) internal {
        bytes memory data = abi.encodeWithSelector(
            bytes4(keccak256("setGuard(address)")),
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

    // Helper functions for string manipulation
    function _split(
        string memory _str,
        string memory _delimiter
    ) internal pure returns (string[] memory) {
        // ... existing code ...
    }

    function _substring(
        string memory _str,
        uint _start,
        uint _end
    ) internal pure returns (string memory) {
        // ... existing code ...
    }
}
