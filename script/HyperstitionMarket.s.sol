// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {HyperstitionMarketFactory, ISimpleTrigger} from "../src/HyperstitionMarketFactory.sol";
import {ConditionalTokens} from "@lay3rlabs/conditional-tokens-contracts/ConditionalTokens.sol";
import {LMSRMarketMaker} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMaker.sol";
import {ERC20Mintable} from "../src/ERC20Mintable.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract DeployHyperstitionMarket is Script {
    HyperstitionMarketFactory public factory;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy the contract
        factory = new HyperstitionMarketFactory();
        collateralToken = new ERC20Mintable("Collateral", "CLL");

        vm.stopBroadcast();

        // Log the deployment
        console.log("Factory deployed at:", address(factory));
        console.log("Collateral token deployed at:", address(collateralToken));

        // Write to .env file
        string memory path = string.concat(vm.projectRoot(), "/.env");
        string memory newContent = string.concat(
            "\nHYPERSTITION_FACTORY_ADDRESS=",
            vm.toString(address(factory)),
            "\nCOLLATERAL_TOKEN_ADDRESS=",
            vm.toString(address(collateralToken))
        );
        vm.writeLine(path, newContent);

        console.log(
            "Updated .env file with HYPERSTITION_FACTORY_ADDRESS and COLLATERAL_TOKEN_ADDRESS"
        );
    }
}

contract LaunchHyperstitionMarket is Script {
    HyperstitionMarketFactory public factory;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress("HYPERSTITION_FACTORY_ADDRESS");
        address collateralTokenAddress = vm.envAddress(
            "COLLATERAL_TOKEN_ADDRESS"
        );
        uint64 fee = 5e16; // 5% fee
        uint256 funding = 1e18;

        factory = HyperstitionMarketFactory(factoryAddress);
        collateralToken = ERC20Mintable(collateralTokenAddress);

        console.log("Factory address:", factoryAddress);
        console.log("Collateral token address:", collateralTokenAddress);

        vm.startBroadcast(deployerPrivateKey);

        collateralToken.mint(deployer, funding);
        collateralToken.approve(address(factory), funding);
        (
            ConditionalTokens conditionalTokens,
            LMSRMarketMaker lmsrMarketMaker
        ) = factory.createConditionalTokenAndLMSRMarketMaker(
                "uri",
                bytes32(0),
                collateralTokenAddress,
                fee,
                funding
            );

        vm.stopBroadcast();

        // Log the deployment
        console.log("Conditional tokens address:", address(conditionalTokens));
        console.log("Market maker address:", address(lmsrMarketMaker));

        // Write to .env file
        string memory path = string.concat(vm.projectRoot(), "/.env");
        string memory newContent = string.concat(
            "\nMARKET_MAKER_ADDRESS=",
            vm.toString(address(lmsrMarketMaker)),
            "\nCONDITIONAL_TOKENS_ADDRESS=",
            vm.toString(address(conditionalTokens))
        );
        vm.writeLine(path, newContent);

        console.log(
            "Updated .env file with MARKET_MAKER_ADDRESS and CONDITIONAL_TOKENS_ADDRESS"
        );
    }
}

contract BuyYesHyperstitionMarket is Script {
    LMSRMarketMaker public marketMaker;
    ConditionalTokens public conditionalTokens;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress("HYPERSTITION_FACTORY_ADDRESS");
        address marketMakerAddress = vm.envAddress("MARKET_MAKER_ADDRESS");
        address collateralTokenAddress = vm.envAddress(
            "COLLATERAL_TOKEN_ADDRESS"
        );
        address conditionalTokensAddress = vm.envAddress(
            "CONDITIONAL_TOKENS_ADDRESS"
        );
        int256 buying = 1e18;

        marketMaker = LMSRMarketMaker(marketMakerAddress);
        conditionalTokens = ConditionalTokens(conditionalTokensAddress);
        collateralToken = ERC20Mintable(collateralTokenAddress);

        // Add more detailed logging
        console.log("Factory address:", factoryAddress);
        console.log("Market maker address:", marketMakerAddress);
        console.log("Collateral token address:", collateralTokenAddress);
        console.log("Conditional tokens address:", conditionalTokensAddress);

        vm.startBroadcast(deployerPrivateKey);

        collateralToken.mint(deployer, uint256(buying));
        collateralToken.approve(address(marketMaker), uint256(buying));

        bytes32 conditionId = conditionalTokens.getConditionId(
            factoryAddress,
            bytes32(0),
            2
        );
        bytes32 collectionId = conditionalTokens.getCollectionId(
            bytes32(0),
            conditionId,
            1
        );
        uint256 positionId = conditionalTokens.getPositionId(
            IERC20(collateralTokenAddress),
            collectionId
        );
        console.log(
            "Collateral balance before:",
            collateralToken.balanceOf(deployer)
        );
        console.log(
            "Outcome token balance before:",
            conditionalTokens.balanceOf(deployer, positionId)
        );

        // buy all YES
        int256[] memory outcomeTokenAmounts = new int256[](2);
        outcomeTokenAmounts[0] = buying;
        outcomeTokenAmounts[1] = 0;
        int256 netCost = marketMaker.trade(outcomeTokenAmounts, buying);

        console.log("Net cost:", netCost);
        console.log(
            "Collateral balance after:",
            collateralToken.balanceOf(deployer)
        );
        console.log(
            "Outcome token balance after:",
            conditionalTokens.balanceOf(deployer, positionId)
        );

        vm.stopBroadcast();
    }
}

contract RedeemHyperstitionMarket is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress("HYPERSTITION_FACTORY_ADDRESS");
        address collateralTokenAddress = vm.envAddress(
            "COLLATERAL_TOKEN_ADDRESS"
        );
        address conditionalTokensAddress = vm.envAddress(
            "CONDITIONAL_TOKENS_ADDRESS"
        );

        ERC20Mintable collateralToken = ERC20Mintable(collateralTokenAddress);
        ConditionalTokens conditionalTokens = ConditionalTokens(
            conditionalTokensAddress
        );

        // Add more detailed logging
        console.log("Factory address:", factoryAddress);
        console.log("Collateral token address:", collateralTokenAddress);
        console.log("Conditional tokens address:", conditionalTokensAddress);

        vm.startBroadcast(deployerPrivateKey);

        bytes32 conditionId = conditionalTokens.getConditionId(
            factoryAddress,
            bytes32(0),
            2
        );
        bytes32 collectionId = conditionalTokens.getCollectionId(
            bytes32(0),
            conditionId,
            1
        );
        uint256 positionId = conditionalTokens.getPositionId(
            IERC20(collateralTokenAddress),
            collectionId
        );
        console.log(
            "Collateral balance before:",
            collateralToken.balanceOf(deployer)
        );
        console.log(
            "Outcome token balance before:",
            conditionalTokens.balanceOf(deployer, positionId)
        );

        // redeem payout
        uint256[] memory indexSets = new uint256[](1);
        indexSets[0] = 1;
        conditionalTokens.redeemPositions(
            IERC20(collateralTokenAddress),
            bytes32(0),
            conditionId,
            indexSets
        );

        console.log(
            "Collateral balance after:",
            collateralToken.balanceOf(deployer)
        );
        console.log(
            "Outcome token balance after:",
            conditionalTokens.balanceOf(deployer, positionId)
        );

        vm.stopBroadcast();
    }
}

contract ResolveHyperstitionMarketTrigger is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        address factoryAddress = vm.envAddress("HYPERSTITION_FACTORY_ADDRESS");
        address marketMakerAddress = vm.envAddress("MARKET_MAKER_ADDRESS");
        address conditionalTokensAddress = vm.envAddress(
            "CONDITIONAL_TOKENS_ADDRESS"
        );

        HyperstitionMarketFactory factory = HyperstitionMarketFactory(
            factoryAddress
        );

        vm.startBroadcast(deployerPrivateKey);

        // Create test trigger data using the provided message
        HyperstitionMarketFactory.TriggerData
            memory triggerData = HyperstitionMarketFactory.TriggerData({
                lmsrMarketMaker: marketMakerAddress,
                conditionalTokens: conditionalTokensAddress,
                result: true
            });

        // Add trigger (sends 0.1 ETH)
        ISimpleTrigger.TriggerId triggerId = factory.addTrigger{
            value: 0.1 ether
        }(triggerData);

        vm.stopBroadcast();

        uint64 tid = ISimpleTrigger.TriggerId.unwrap(triggerId);
        console.log("Trigger ID:", tid);

        // Fetch and log the trigger info
        ISimpleTrigger.TriggerInfo memory info = factory.getTrigger(triggerId);
        console.log("Trigger created by:", info.creator);
        console.logBytes(info.data);
    }
}
