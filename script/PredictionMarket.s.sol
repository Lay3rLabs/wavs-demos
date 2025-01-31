// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
import {PredictionMarketFactory} from "../src/PredictionMarketFactory.sol";
import {PredictionMarketOracleController, ISimpleTrigger} from "../src/PredictionMarketOracleController.sol";
import {ConditionalTokens} from "@lay3rlabs/conditional-tokens-contracts/ConditionalTokens.sol";
import {LMSRMarketMaker} from "@lay3rlabs/conditional-tokens-market-makers/LMSRMarketMaker.sol";
import {ERC20Mintable} from "../src/ERC20Mintable.sol";
import {IERC20} from "forge-std/interfaces/IERC20.sol";

contract DeployPredictionMarket is Script {
    PredictionMarketOracleController public oracle;
    PredictionMarketFactory public factory;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);

        // Deploy the contracts
        oracle = new PredictionMarketOracleController();
        factory = oracle.factory();
        collateralToken = new ERC20Mintable("Collateral", "CLL");

        vm.stopBroadcast();

        // Log the deployment
        console.log("Oracle controller deployed at:", address(oracle));
        console.log("Factory deployed at:", address(factory));
        console.log("Collateral token deployed at:", address(collateralToken));

        // Write to .env file
        string memory path = string.concat(vm.projectRoot(), "/.env");
        string memory newContent = string.concat(
            "\nPREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS=",
            vm.toString(address(oracle)),
            "\nPREDICTION_MARKET_FACTORY_ADDRESS=",
            vm.toString(address(factory)),
            "\nCOLLATERAL_TOKEN_ADDRESS=",
            vm.toString(address(collateralToken))
        );
        vm.writeLine(path, newContent);

        console.log(
            "Updated .env file with PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS, PREDICTION_MARKET_FACTORY_ADDRESS, and COLLATERAL_TOKEN_ADDRESS"
        );
    }
}

contract LaunchPredictionMarket is Script {
    PredictionMarketFactory public factory;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress(
            "PREDICTION_MARKET_FACTORY_ADDRESS"
        );
        address collateralTokenAddress = vm.envAddress(
            "COLLATERAL_TOKEN_ADDRESS"
        );
        uint64 fee = 5e16; // 5% fee
        // fund with 1,000 collateral tokens
        uint256 funding = 1_000e18;

        factory = PredictionMarketFactory(factoryAddress);
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

contract BuyYesPredictionMarket is Script {
    LMSRMarketMaker public marketMaker;
    ConditionalTokens public conditionalTokens;
    ERC20Mintable public collateralToken;

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress(
            "PREDICTION_MARKET_FACTORY_ADDRESS"
        );
        address marketMakerAddress = vm.envAddress("MARKET_MAKER_ADDRESS");
        address collateralTokenAddress = vm.envAddress(
            "COLLATERAL_TOKEN_ADDRESS"
        );
        address conditionalTokensAddress = vm.envAddress(
            "CONDITIONAL_TOKENS_ADDRESS"
        );
        // buy with 1 collateral token
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
            2
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
        outcomeTokenAmounts[0] = 0;
        outcomeTokenAmounts[1] = buying;
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

contract ResolvePredictionMarketTrigger is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        address oracleAddress = vm.envAddress(
            "PREDICTION_MARKET_ORACLE_CONTROLLER_ADDRESS"
        );
        address marketMakerAddress = vm.envAddress("MARKET_MAKER_ADDRESS");
        address conditionalTokensAddress = vm.envAddress(
            "CONDITIONAL_TOKENS_ADDRESS"
        );

        PredictionMarketOracleController oracle = PredictionMarketOracleController(
                oracleAddress
            );

        vm.startBroadcast(deployerPrivateKey);

        // Create test trigger data using the provided message
        PredictionMarketOracleController.TriggerInputData
            memory triggerData = PredictionMarketOracleController
                .TriggerInputData({
                    lmsrMarketMaker: marketMakerAddress,
                    conditionalTokens: conditionalTokensAddress,
                    result: true
                });

        // Add trigger (sends 0.1 ETH)
        ISimpleTrigger.TriggerId triggerId = oracle.addTrigger{
            value: 0.1 ether
        }(triggerData);

        vm.stopBroadcast();

        uint64 tid = ISimpleTrigger.TriggerId.unwrap(triggerId);
        console.log("Trigger ID:", tid);
    }
}

contract RedeemPredictionMarket is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);
        address factoryAddress = vm.envAddress(
            "PREDICTION_MARKET_FACTORY_ADDRESS"
        );
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
            2
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
        indexSets[0] = 2;
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
