pragma solidity ^0.8.0;

import {ECDSAStakeRegistry} from "@eigenlayer/middleware/src/unaudited/ECDSAStakeRegistry.sol";
import {IERC1271Upgradeable} from "@openzeppelin-upgrades/contracts/interfaces/IERC1271Upgradeable.sol";
import {ECDSAUpgradeable} from "@openzeppelin-upgrades/contracts/utils/cryptography/ECDSAUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {IWavsSDK} from "./interfaces/IWavsSDK.sol";
import {MessageHashUtils} from "@oz/utils/cryptography/MessageHashUtils.sol";

// TODO we might want to use the one in lib/WAVS
abstract contract WavsSDK {
    using MessageHashUtils for bytes32;

    /// @notice bytes4(keccak256("isValidSignature(bytes32,bytes)")
    bytes4 internal constant _ERC1271_SIGNATURE = 0x1626ba7e;

    ECDSAStakeRegistry public immutable STAKE_REGISTRY;

    /**
     * @notice Modifier to ensure a task is valid
     * @param _task The task to validate
     */
    modifier onlyValidLayerTask(IWavsSDK.Task memory _task) {
        if (!_validateWavsTask(_task)) revert IWavsSDK.InvalidWavsTask();
        _;
    }

    // Modifiers
    modifier onlyOperator() {
        require(STAKE_REGISTRY.operatorRegistered(msg.sender), "Operator must be the caller");
        _;
    }

    modifier onlyValidPayload(IWavsSDK.SignedPayload calldata signedPayload) {
        if (!validatePayload(signedPayload)) revert InvalidSignature();
        _;
    }

    modifier onlyValidPayloads(IWavsSDK.SignedPayload[] calldata signedPayloads) {
        if (!validatePayloadMulti(signedPayloads)) revert InvalidSignature();
        _;
    }

    // Errors
    error InvalidSignature();

    /**
     * @notice Initializer
     * @param _stakeRegistry The address of the stake registry contract
     */
    constructor(address _stakeRegistry) {
        STAKE_REGISTRY = ECDSAStakeRegistry(_stakeRegistry);
    }

    // Subcontracts should override this function
    function _handleAddPayload(IWavsSDK.SignedPayload calldata signedPayload) internal virtual;

    function addPayload(IWavsSDK.SignedPayload calldata signedPayload) public onlyValidPayload(signedPayload) {
        _handleAddPayload(signedPayload);
    }

    function addPayloadMulti(IWavsSDK.SignedPayload[] calldata signedPayloads)
        public
        onlyValidPayloads(signedPayloads)
    {
        for (uint32 i = 0; i < signedPayloads.length; i++) {
            _handleAddPayload(signedPayloads[i]);
        }
    }

    function validatePayload(IWavsSDK.SignedPayload calldata signedPayload) public view returns (bool) {
        bytes32 message = keccak256(signedPayload.data);
        bytes32 ethSignedMessageHash = ECDSAUpgradeable.toEthSignedMessageHash(message);
        bytes4 magicValue = IERC1271Upgradeable.isValidSignature.selector;

        return (magicValue == STAKE_REGISTRY.isValidSignature(ethSignedMessageHash, signedPayload.signature));
    }

    function validatePayloadMulti(IWavsSDK.SignedPayload[] calldata signedPayloads) public view returns (bool) {
        for (uint32 i = 0; i < signedPayloads.length; i++) {
            if (!WavsSDK(address(this)).validatePayload(signedPayloads[i])) {
                return false;
            }
        }

        return true;
    }

    /**
     * @notice Validates a WAVS task from off-chain AVS operator
     * @param _task The message and signatures to verify
     * @return _isValid Whether the task is valid
     */
    function _validateWavsTask(IWavsSDK.Task memory _task) internal view returns (bool _isValid) {
        _isValid = (_ERC1271_SIGNATURE == STAKE_REGISTRY.isValidSignature(_task.dataHash, _task.signatureData));
    }

    /**
     * @notice Validates the format of an EIP-712 signed message
     * @param _messageHash The hash of the message
     * @param _message The message to verify is hashed according to EIP-712
     * @return _isValid Whether the message hash represents the message according to EIP-712
     */
    function _validateEthSignedMessage(bytes32 _messageHash, string memory _message)
        internal
        pure
        returns (bool _isValid)
    {
        bytes32 _msgToHash = keccak256(abi.encodePacked(_message));
        _isValid = _messageHash == _msgToHash.toEthSignedMessageHash();
    }
}
