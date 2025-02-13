// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Burnable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Pausable.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Votes.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import {IServiceHandler} from "@wavs/interfaces/IServiceHandler.sol";
import {ISimpleTrigger} from "./interfaces/ISimpleTrigger.sol";

contract NFTWithTrigger is
    ERC721,
    ERC721Enumerable,
    ERC721URIStorage,
    ERC721Pausable,
    AccessControl,
    ERC721Burnable,
    EIP712,
    ERC721Votes,
    IServiceHandler,
    ISimpleTrigger
{
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant SERVICE_PROVIDER_ROLE =
        keccak256("SERVICE_PROVIDER_ROLE");

    mapping(TriggerId => TriggerInfo) public triggersById;
    mapping(address => TriggerId[]) public triggerIdsByCreator;
    uint64 private _nextTriggerId; // Changed to uint64 to match TriggerId

    uint256 private _nextTokenId;
    address public serviceProvider;
    bool public initialized;

    event NewTrigger(
        TriggerId indexed triggerId,
        address indexed creator,
        bytes data
    );
    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri
    );
    event Debug(string message, uint256 tokenId);

    struct ReturnData {
        address creator;
        uint64 triggerId;
        string dataUri;
    }

    constructor() ERC721("TriggerNFT", "TNFT") EIP712("TriggerNFT", "1") {
        // TODO consider what the permissions of this contract should be
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);

        // Initialize these values in constructor instead
        initialized = false;
        serviceProvider = address(0);
    }

    function initialize(
        address _serviceProvider
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        require(!initialized, "Already initialized");
        require(_serviceProvider != address(0), "Invalid service provider");
        _grantRole(SERVICE_PROVIDER_ROLE, _serviceProvider);
        serviceProvider = _serviceProvider;
        initialized = true;
    }

    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    // Implement getTrigger from interface
    function getTrigger(
        TriggerId triggerId
    ) external view override returns (TriggerInfo memory) {
        return triggersById[triggerId];
    }

    function addTrigger(bytes memory data) external payable {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        TriggerId triggerId = TriggerId.wrap(uint64(_nextTriggerId++));

        TriggerInfo memory triggerInfo = TriggerInfo({
            triggerId: triggerId,
            creator: msg.sender,
            data: data
        });

        triggersById[triggerId] = triggerInfo;
        triggerIdsByCreator[msg.sender].push(triggerId);

        emit NewTrigger(triggerId, msg.sender, data);
    }

    // Add this function to allow withdrawal of collected fees
    function withdrawFees() external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 balance = address(this).balance;
        require(balance > 0, "No balance to withdraw");

        (bool success, ) = payable(msg.sender).call{value: balance}("");
        require(success, "Transfer failed");
    }

    function handleAddPayload(
        bytes calldata data,
        bytes calldata signature
    ) external {
        require(
            hasRole(SERVICE_PROVIDER_ROLE, msg.sender),
            "Only service provider"
        );

        ReturnData memory returnData = abi.decode(data, (ReturnData));
        TriggerId triggerId = TriggerId.wrap(returnData.triggerId);

        require(
            triggersById[triggerId].creator != address(0),
            "Trigger does not exist"
        );
        require(bytes(returnData.dataUri).length > 0, "URI is empty");

        delete triggersById[triggerId];
        uint256 tokenId = _nextTokenId++;

        _safeMint(returnData.creator, tokenId);
        _setTokenURI(tokenId, returnData.dataUri);

        emit NFTMinted(returnData.creator, tokenId, returnData.dataUri);
    }

    // Update helper functions
    function getTriggerCount(address creator) external view returns (uint256) {
        return triggerIdsByCreator[creator].length;
    }

    function getTriggerIdAtIndex(
        address creator,
        uint256 index
    ) external view returns (TriggerId) {
        require(
            index < triggerIdsByCreator[creator].length,
            "Index out of bounds"
        );
        return triggerIdsByCreator[creator][index];
    }

    // Add tokenURI override
    function tokenURI(
        uint256 tokenId
    ) public view override(ERC721, ERC721URIStorage) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function _update(
        address to,
        uint256 tokenId,
        address auth
    )
        internal
        override(ERC721, ERC721Enumerable, ERC721Pausable, ERC721Votes)
        returns (address)
    {
        return super._update(to, tokenId, auth);
    }

    function _increaseBalance(
        address account,
        uint128 value
    ) internal override(ERC721, ERC721Enumerable, ERC721Votes) {
        super._increaseBalance(account, value);
    }

    function supportsInterface(
        bytes4 interfaceId
    )
        public
        view
        override(ERC721, ERC721Enumerable, ERC721URIStorage, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    function clock() public view override returns (uint48) {
        return uint48(block.timestamp);
    }

    function CLOCK_MODE() public pure override returns (string memory) {
        return "mode=timestamp";
    }

    // Do we need this?
    function _setTokenURI(
        uint256 tokenId,
        string memory _tokenURI
    ) internal override(ERC721URIStorage) {
        super._setTokenURI(tokenId, _tokenURI);
    }
}
