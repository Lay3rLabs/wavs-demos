// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {NFTWithTrigger} from "../src/NFTWithTrigger.sol";
import {ISimpleTrigger} from "../src/interfaces/ISimpleTrigger.sol";

contract DeployNFTWithTrigger is Script {
    NFTWithTrigger public nft;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy the contract
        nft = new NFTWithTrigger();

        vm.stopBroadcast();

        // Log the deployment
        console.log("NFTWithTrigger deployed at:", address(nft));

        // Write to .env file
        string memory path = vm.projectRoot();
        string memory envContent = vm.readFile(string.concat(path, "/.env"));
        vm.writeFile(
            string.concat(path, "/.env"),
            string.concat(
                envContent,
                "\nNFT_ADDRESS=",
                vm.toString(address(nft))
            )
        );
        console.log("Updated .env file with NFT_ADDRESS");
    }
}

contract InitializeNFTWithTrigger is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address nftAddress = vm.envAddress("NFT_ADDRESS");
        address serviceProvider = vm.envAddress("SERVICE_PROVIDER");

        NFTWithTrigger nft = NFTWithTrigger(nftAddress);

        // Add more detailed logging
        console.log("Contract address:", nftAddress);
        console.log("Service provider to set:", serviceProvider);
        console.log("Is contract initialized?", nft.initialized());
        console.log("Current service provider:", nft.serviceProvider());
        console.log(
            "Caller has admin role?",
            nft.hasRole(nft.DEFAULT_ADMIN_ROLE(), vm.addr(deployerPrivateKey))
        );

        vm.startBroadcast(deployerPrivateKey);

        try nft.initialize(serviceProvider) {
            console.log("Successfully initialized NFT");
        } catch Error(string memory reason) {
            console.log("Initialization failed with reason:", reason);
            revert(reason);
        } catch {
            console.log("Initialization failed with unknown reason");
            revert("Initialization failed with unknown reason");
        }

        vm.stopBroadcast();

        console.log("NFTWithTrigger initialized at:", nftAddress);
        console.log("Service Provider set to:", serviceProvider);

        // Write to .env file
        string memory path = vm.projectRoot();
        string memory envContent = vm.readFile(string.concat(path, "/.env"));
        vm.writeFile(
            string.concat(path, "/.env"),
            string.concat(
                envContent,
                "\nINITIALIZED_NFT_ADDRESS=",
                vm.toString(nftAddress),
                "\nINITIALIZED_SERVICE_PROVIDER=",
                vm.toString(serviceProvider)
            )
        );
        console.log("Updated .env file with initialization details");
    }
}

contract TestTrigger is Script {
    function run(string memory testMessage) public {
        uint256 userPrivateKey = vm.envUint("PRIVATE_KEY");
        address nftAddress = vm.envAddress("NFT_ADDRESS");

        NFTWithTrigger nft = NFTWithTrigger(nftAddress);

        vm.startBroadcast(userPrivateKey);

        // Create test trigger data using the provided message
        bytes memory testData = abi.encode(testMessage);

        // Add trigger (sends 0.1 ETH)
        nft.addTrigger{value: 0.1 ether}(testData);

        vm.stopBroadcast();

        // Get the trigger ID (it will be 0 for the first trigger)
        ISimpleTrigger.TriggerId triggerId = ISimpleTrigger.TriggerId.wrap(0);

        // Fetch and log the trigger info
        ISimpleTrigger.TriggerInfo memory info = nft.getTrigger(triggerId);
        console.log("Trigger created by:", info.creator);
        console.log("Trigger message:", testMessage);
        console.logBytes(info.data);
    }
}
