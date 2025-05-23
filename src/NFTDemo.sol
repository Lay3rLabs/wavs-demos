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
import {IWavsServiceHandler} from "@wavs/interfaces/IWavsServiceHandler.sol";
import {IWavsServiceManager} from "@wavs/interfaces/IWavsServiceManager.sol";
import {ITypes} from "./interfaces/ITypes.sol";

contract NFTDemo is
    ERC721,
    ERC721Enumerable,
    ERC721URIStorage,
    ERC721Pausable,
    AccessControl,
    ERC721Burnable,
    EIP712,
    ERC721Votes,
    IWavsServiceHandler
{
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");

    mapping(ITypes.TriggerId => Trigger) public triggersById;
    mapping(address => ITypes.TriggerId[]) public triggerIdsByCreator;

    IWavsServiceManager public serviceManager;
    ITypes.TriggerId public nextTriggerId;
    uint256 public nextTokenId;

    event NFTMinted(
        address indexed to,
        uint256 indexed tokenId,
        string dataUri
    );

    struct Trigger {
        address creator;
        bytes data;
    }

    constructor(
        address serviceManager_
    ) ERC721("TriggerNFT", "TNFT") EIP712("TriggerNFT", "1") {
        require(serviceManager_ != address(0), "Invalid service manager");

        // TODO consider what the permissions of this contract should be
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);

        serviceManager = IWavsServiceManager(serviceManager_);
    }

    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    /**
     * @notice Add a new trigger.
     * @param data The request data (bytes).
     */
    function addTrigger(
        bytes memory data
    ) external payable returns (ITypes.TriggerId triggerId) {
        require(msg.value == 0.1 ether, "Payment must be exactly 0.1 ETH");

        // Get the next trigger ID
        triggerId = nextTriggerId;
        nextTriggerId = ITypes.TriggerId.wrap(
            ITypes.TriggerId.unwrap(nextTriggerId) + 1
        );

        // Create the trigger
        Trigger memory trigger = Trigger({creator: msg.sender, data: data});

        // update storages
        triggersById[triggerId] = trigger;
        triggerIdsByCreator[msg.sender].push(triggerId);

        ITypes.TriggerInfo memory triggerInfo = ITypes.TriggerInfo({
            triggerId: triggerId,
            creator: trigger.creator,
            data: trigger.data
        });

        emit ITypes.NewTrigger(abi.encode(triggerInfo));
    }

    // Add this function to allow withdrawal of collected fees
    function withdrawFees() external onlyRole(DEFAULT_ADMIN_ROLE) {
        uint256 balance = address(this).balance;
        require(balance > 0, "No balance to withdraw");

        (bool success, ) = payable(msg.sender).call{value: balance}("");
        require(success, "Transfer failed");
    }

    /**
     * @notice Handle the signed AVS output data.
     * @param data The data to handle.
     * @param signature The signature of the data.
     */
    function handleSignedData(
        bytes calldata data,
        bytes calldata signature
    ) external override {
        serviceManager.validate(data, signature);

        ITypes.DataWithId memory dataWithId = abi.decode(
            data,
            (ITypes.DataWithId)
        );

        Trigger memory trigger = triggersById[dataWithId.triggerId];
        require(trigger.creator != address(0), "Trigger does not exist");
        require(dataWithId.data.length > 0, "URI is empty");

        string memory dataUri = abi.decode(dataWithId.data, (string));
        uint256 tokenId = nextTokenId++;

        _safeMint(trigger.creator, tokenId);
        _setTokenURI(tokenId, dataUri);

        emit NFTMinted(trigger.creator, tokenId, dataUri);
    }

    /**
     * @notice Get a single trigger by triggerId.
     * @param triggerId The identifier of the trigger.
     */
    function getTrigger(
        ITypes.TriggerId triggerId
    ) public view returns (ITypes.TriggerInfo memory) {
        Trigger storage trigger = triggersById[triggerId];

        return
            ITypes.TriggerInfo({
                triggerId: triggerId,
                creator: trigger.creator,
                data: trigger.data
            });
    }

    function getTriggerCount(address creator) external view returns (uint256) {
        return triggerIdsByCreator[creator].length;
    }

    function getTriggerIdAtIndex(
        address creator,
        uint256 index
    ) external view returns (ITypes.TriggerId) {
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
