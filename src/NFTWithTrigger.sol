// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ISimpleTrigger} from "./interfaces/ISimpleTrigger.sol";

contract NFTWithTrigger is ERC721, Ownable, IServiceHandler, ISimpleTrigger {
    // Storage variables for trigger management
    mapping(uint256 => TriggerInfo) public triggersById;
    mapping(address => uint256[]) public triggerIdsByCreator;
    uint256 public nextTriggerId;

    // NFT-specific storage
    uint256 private _nextTokenId;
    address public serviceProvider;
    bool public initialized;

    struct TriggerInfo {
        uint256 triggerId;
        address creator;
        bytes data;
    }

    event NewTrigger(
        uint256 indexed triggerId,
        address indexed creator,
        bytes data
    );
    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri
    );

    constructor() ERC721("TriggerNFT", "TNFT") Ownable(msg.sender) {}

    function initialize(address _serviceProvider) external onlyOwner {
        require(!initialized, "Already initialized");
        require(_serviceProvider != address(0), "Invalid service provider");
        serviceProvider = _serviceProvider;
        initialized = true;
    }

    function getTrigger(
        uint256 triggerId
    ) external view returns (TriggerInfo memory) {
        return triggersById[triggerId];
    }

    function addTrigger(bytes memory data) external payable {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        uint256 triggerId = nextTriggerId++;

        TriggerInfo memory triggerInfo = TriggerInfo({
            triggerId: triggerId,
            creator: msg.sender,
            data: data
        });

        triggersById[triggerId] = triggerInfo;
        triggerIdsByCreator[msg.sender].push(triggerId);

        emit NewTrigger(triggerId, msg.sender, data);
    }

    function handleAddPayload(
        bytes calldata data,
        bytes calldata signature
    ) external {
        require(msg.sender == serviceProvider, "Only service provider");

        // Decode the mint parameters from the payload data
        (address creator, uint256 triggerId, string memory dataUri) = abi
            .decode(data, (address, uint256, string));

        require(triggersById[triggerId].creator == creator, "Invalid trigger");

        // Mint new NFT
        uint256 tokenId = _nextTokenId++;
        _safeMint(creator, tokenId);

        // Clean up trigger state
        delete triggersById[triggerId];
        // Note: We keep the triggerIdsByCreator mapping for history

        emit NFTMinted(creator, tokenId, dataUri);
    }

    // Helper view functions
    function getTriggerCount(address creator) external view returns (uint256) {
        return triggerIdsByCreator[creator].length;
    }

    function getTriggerIdAtIndex(
        address creator,
        uint256 index
    ) external view returns (uint256) {
        require(
            index < triggerIdsByCreator[creator].length,
            "Index out of bounds"
        );
        return triggerIdsByCreator[creator][index];
    }
}
