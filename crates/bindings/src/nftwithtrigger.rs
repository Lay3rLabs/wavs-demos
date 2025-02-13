///Module containing a contract's types and functions.
/**

```solidity
library ISimpleTrigger {
    type TriggerId is uint64;
    struct TriggerInfo { TriggerId triggerId; address creator; bytes data; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISimpleTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerId(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TriggerId> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl TriggerId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerId {
            type RustType = u64;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct TriggerInfo { TriggerId triggerId; address creator; bytes data; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerInfo {
        pub triggerId: <TriggerId as alloy::sol_types::SolType>::RustType,
        pub creator: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (TriggerId, alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TriggerId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TriggerInfo> for UnderlyingRustTuple<'_> {
            fn from(value: TriggerInfo) -> Self {
                (value.triggerId, value.creator, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TriggerInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { triggerId: tuple.0, creator: tuple.1, data: tuple.2 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TriggerInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TriggerInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.creator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for TriggerInfo {
            const NAME: &'static str = "TriggerInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TriggerInfo(uint64 triggerId,address creator,bytes data)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <TriggerId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.triggerId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.creator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TriggerId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.triggerId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.creator,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <TriggerId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.triggerId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.creator,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISimpleTrigger`](self) contract instance.

    See the [wrapper's documentation](`ISimpleTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISimpleTriggerInstance<T, P, N> {
        ISimpleTriggerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISimpleTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISimpleTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISimpleTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISimpleTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISimpleTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISimpleTrigger`](self) contract instance.

        See the [wrapper's documentation](`ISimpleTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISimpleTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISimpleTriggerInstance<T, P, N> {
            ISimpleTriggerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > ISimpleTriggerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ISimpleTrigger {
    type TriggerId is uint64;
    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }
}

interface NFTWithTrigger {
    error AccessControlBadConfirmation();
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    error CheckpointUnorderedInsertion();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
    error ERC6372InconsistentClock();
    error ERC721EnumerableForbiddenBatchMint();
    error ERC721IncorrectOwner(address sender, uint256 tokenId, address owner);
    error ERC721InsufficientApproval(address operator, uint256 tokenId);
    error ERC721InvalidApprover(address approver);
    error ERC721InvalidOperator(address operator);
    error ERC721InvalidOwner(address owner);
    error ERC721InvalidReceiver(address receiver);
    error ERC721InvalidSender(address sender);
    error ERC721NonexistentToken(uint256 tokenId);
    error ERC721OutOfBoundsIndex(address owner, uint256 index);
    error EnforcedPause();
    error ExpectedPause();
    error InvalidAccountNonce(address account, uint256 currentNonce);
    error InvalidShortString();
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    error StringTooLong(string str);
    error VotesExpiredSignature(uint256 expiry);

    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);
    event Debug(string message, uint256 tokenId);
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    event EIP712DomainChanged();
    event MetadataUpdate(uint256 _tokenId);
    event NFTMinted(address indexed to, uint256 indexed tokenId, string dataUri);
    event NewTrigger(ISimpleTrigger.TriggerId indexed triggerId, address indexed creator, bytes data);
    event Paused(address account);
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Unpaused(address account);

    constructor();

    function CLOCK_MODE() external pure returns (string memory);
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    function PAUSER_ROLE() external view returns (bytes32);
    function SERVICE_PROVIDER_ROLE() external view returns (bytes32);
    function addTrigger(bytes memory data) external payable;
    function approve(address to, uint256 tokenId) external;
    function balanceOf(address owner) external view returns (uint256);
    function burn(uint256 tokenId) external;
    function clock() external view returns (uint48);
    function delegate(address delegatee) external;
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    function delegates(address account) external view returns (address);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getApproved(uint256 tokenId) external view returns (address);
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    function getTriggerCount(address creator) external view returns (uint256);
    function getTriggerIdAtIndex(address creator, uint256 index) external view returns (ISimpleTrigger.TriggerId);
    function getVotes(address account) external view returns (uint256);
    function grantRole(bytes32 role, address account) external;
    function handleAddPayload(bytes memory data, bytes memory signature) external;
    function hasRole(bytes32 role, address account) external view returns (bool);
    function initialize(address _serviceProvider) external;
    function initialized() external view returns (bool);
    function isApprovedForAll(address owner, address operator) external view returns (bool);
    function name() external view returns (string memory);
    function nonces(address owner) external view returns (uint256);
    function ownerOf(uint256 tokenId) external view returns (address);
    function pause() external;
    function paused() external view returns (bool);
    function renounceRole(bytes32 role, address callerConfirmation) external;
    function revokeRole(bytes32 role, address account) external;
    function safeTransferFrom(address from, address to, uint256 tokenId) external;
    function safeTransferFrom(address from, address to, uint256 tokenId, bytes memory data) external;
    function serviceProvider() external view returns (address);
    function setApprovalForAll(address operator, bool approved) external;
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function symbol() external view returns (string memory);
    function tokenByIndex(uint256 index) external view returns (uint256);
    function tokenOfOwnerByIndex(address owner, uint256 index) external view returns (uint256);
    function tokenURI(uint256 tokenId) external view returns (string memory);
    function totalSupply() external view returns (uint256);
    function transferFrom(address from, address to, uint256 tokenId) external;
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    function triggersById(ISimpleTrigger.TriggerId) external view returns (ISimpleTrigger.TriggerId triggerId, address creator, bytes memory data);
    function unpause() external;
    function withdrawFees() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CLOCK_MODE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "DEFAULT_ADMIN_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "PAUSER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "SERVICE_PROVIDER_ROLE",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addTrigger",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "approve",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burn",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "clock",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint48",
        "internalType": "uint48"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegate",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegateBySig",
    "inputs": [
      {
        "name": "delegatee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "nonce",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "v",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "r",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegates",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getApproved",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastTotalSupply",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPastVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRoleAdmin",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTrigger",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ISimpleTrigger.TriggerInfo",
        "components": [
          {
            "name": "triggerId",
            "type": "uint64",
            "internalType": "ISimpleTrigger.TriggerId"
          },
          {
            "name": "creator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTriggerCount",
    "inputs": [
      {
        "name": "creator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTriggerIdAtIndex",
    "inputs": [
      {
        "name": "creator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getVotes",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "grantRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "handleAddPayload",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hasRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceProvider",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initialized",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isApprovedForAll",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "name",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonces",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ownerOf",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "callerConfirmation",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "revokeRole",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "safeTransferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "safeTransferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "serviceProvider",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setApprovalForAll",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "symbol",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenByIndex",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenOfOwnerByIndex",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "tokenURI",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "totalSupply",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "triggerIdsByCreator",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "triggersById",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "internalType": "ISimpleTrigger.TriggerId"
      },
      {
        "name": "creator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawFees",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ApprovalForAll",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "BatchMetadataUpdate",
    "inputs": [
      {
        "name": "_fromTokenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_toTokenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Debug",
    "inputs": [
      {
        "name": "message",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateChanged",
    "inputs": [
      {
        "name": "delegator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "fromDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "toDelegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelegateVotesChanged",
    "inputs": [
      {
        "name": "delegate",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "previousVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newVotes",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EIP712DomainChanged",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MetadataUpdate",
    "inputs": [
      {
        "name": "_tokenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NFTMinted",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "dataUri",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NewTrigger",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "indexed": true,
        "internalType": "ISimpleTrigger.TriggerId"
      },
      {
        "name": "creator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleAdminChanged",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "previousAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "newAdminRole",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleGranted",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RoleRevoked",
    "inputs": [
      {
        "name": "role",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AccessControlBadConfirmation",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AccessControlUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "neededRole",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "CheckpointUnorderedInsertion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ECDSAInvalidSignatureS",
    "inputs": [
      {
        "name": "s",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC5805FutureLookup",
    "inputs": [
      {
        "name": "timepoint",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "clock",
        "type": "uint48",
        "internalType": "uint48"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC6372InconsistentClock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ERC721EnumerableForbiddenBatchMint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ERC721IncorrectOwner",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InsufficientApproval",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InvalidApprover",
    "inputs": [
      {
        "name": "approver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InvalidOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InvalidReceiver",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721InvalidSender",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721NonexistentToken",
    "inputs": [
      {
        "name": "tokenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ERC721OutOfBoundsIndex",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAccountNonce",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currentNonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SafeCastOverflowedUintDowncast",
    "inputs": [
      {
        "name": "bits",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "VotesExpiredSignature",
    "inputs": [
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod NFTWithTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f5ffd5b50604080518082018252600a80825269151c9a59d9d95c93919560b21b60208084018290528451808601865260018152603160f81b818301528551808701875293845283820192909252845180860190955260048552631513919560e21b9085015291925f61007f838261034e565b50600161008c828261034e565b5050600b805460ff19169055506100a482600d610193565b610120526100b381600e610193565b61014052815160208084019190912060e052815190820120610100524660a05261013f60e05161010051604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201529081019290925260608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b60805250503060c0526101525f336101c5565b5061017d7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a336101c5565b50601780546001600160a81b0319169055610476565b5f6020835110156101ae576101a783610270565b90506101bf565b816101b9848261034e565b5060ff90505b92915050565b5f828152600c602090815260408083206001600160a01b038516845290915281205460ff16610269575f838152600c602090815260408083206001600160a01b03861684529091529020805460ff191660011790556102213390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45060016101bf565b505f6101bf565b5f5f829050601f815111156102a3578260405163305a27a960e01b815260040161029a9190610408565b60405180910390fd5b80516102ae82610453565b179392505050565b634e487b7160e01b5f52604160045260245ffd5b600181811c908216806102de57607f821691505b6020821081036102fc57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561034957805f5260205f20601f840160051c810160208510156103275750805b601f840160051c820191505b81811015610346575f8155600101610333565b50505b505050565b81516001600160401b03811115610367576103676102b6565b61037b8161037584546102ca565b84610302565b6020601f8211600181146103ad575f83156103965750848201515b5f19600385901b1c1916600184901b178455610346565b5f84815260208120601f198516915b828110156103dc57878501518255602094850194600190920191016103bc565b50848210156103f957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602081525f82518060208401525f5b818110156104345760208186018101516040868401015201610417565b505f604082850101526040601f19601f83011684010191505092915050565b805160208083015191908110156102fc575f1960209190910360031b1b16919050565b60805160a05160c05160e051610100516101205161014051613afc6104c75f395f611b3301525f611b0101525f61255c01525f61253401525f61248f01525f6124b901525f6124e30152613afc5ff3fe6080604052600436106102a4575f3560e01c80636352211e1161016f578063a217fddf116100d8578063ce28961211610092578063e328ed771161006d578063e328ed77146108ae578063e63ab1e9146108da578063e985e9c51461090d578063f7ee944c1461092c575f5ffd5b8063ce2896121461084e578063d547741f1461087c578063e31e07881461089b575f5ffd5b8063a217fddf146107a0578063a22cb465146107b3578063b88d4fde146107d2578063c3cda520146107f1578063c4d66de814610810578063c87b56dd1461082f575f5ffd5b80638e539e8c116101295780638e539e8c146106d6578063913b1fbf146106f557806391d148541461072c57806391ddadf41461074b57806395d89b411461076d5780639ab24eb014610781575f5ffd5b80636352211e1461060a57806370a08231146106295780637ecebe00146106485780638456cb591461067c57806384b0196e146106905780638d69e95e146106b7575f5ffd5b80633383abfe11610211578063476343ee116101cb578063476343ee146105345780634bf5d7e9146105485780634f6ccce71461057e578063587cde1e1461059d5780635c19a95c146105d45780635c975abb146105f3575f5ffd5b80633383abfe1461047057806336568abe146104a45780633a46b1a8146104c35780633f4ba83a146104e257806342842e0e146104f657806342966c6814610515575f5ffd5b806318160ddd1161026257806318160ddd146103945780631bc6ae8a146103b257806323b872dd146103e5578063248a9ca3146104045780632f2ff15d146104325780632f745c5914610451575f5ffd5b806273e1d7146102a857806301ffc9a7146102c957806306fdde03146102fd578063081812fc1461031e578063095ea7b314610355578063158ef93e14610374575b5f5ffd5b3480156102b3575f5ffd5b506102c76102c2366004613210565b61094b565b005b3480156102d4575f5ffd5b506102e86102e336600461328f565b610b3e565b60405190151581526020015b60405180910390f35b348015610308575f5ffd5b50610311610b4e565b6040516102f491906132f7565b348015610329575f5ffd5b5061033d610338366004613309565b610bdd565b6040516001600160a01b0390911681526020016102f4565b348015610360575f5ffd5b506102c761036f36600461333b565b610c04565b34801561037f575f5ffd5b506017546102e890600160a01b900460ff1681565b34801561039f575f5ffd5b506008545b6040519081526020016102f4565b3480156103bd575f5ffd5b506103a47fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b81565b3480156103f0575f5ffd5b506102c76103ff366004613363565b610c13565b34801561040f575f5ffd5b506103a461041e366004613309565b5f908152600c602052604090206001015490565b34801561043d575f5ffd5b506102c761044c36600461339d565b610c9c565b34801561045c575f5ffd5b506103a461046b36600461333b565b610cc0565b34801561047b575f5ffd5b506103a461048a3660046133c7565b6001600160a01b03165f9081526014602052604090205490565b3480156104af575f5ffd5b506102c76104be36600461339d565b610d23565b3480156104ce575f5ffd5b506103a46104dd36600461333b565b610d5b565b3480156104ed575f5ffd5b506102c7610dd0565b348015610501575f5ffd5b506102c7610510366004613363565b610e05565b348015610520575f5ffd5b506102c761052f366004613309565b610e1f565b34801561053f575f5ffd5b506102c7610e2a565b348015610553575f5ffd5b5060408051808201909152600e81526d06d6f64653d74696d657374616d760941b6020820152610311565b348015610589575f5ffd5b506103a4610598366004613309565b610f02565b3480156105a8575f5ffd5b5061033d6105b73660046133c7565b6001600160a01b039081165f908152601060205260409020541690565b3480156105df575f5ffd5b506102c76105ee3660046133c7565b610f57565b3480156105fe575f5ffd5b50600b5460ff166102e8565b348015610615575f5ffd5b5061033d610624366004613309565b610f62565b348015610634575f5ffd5b506103a46106433660046133c7565b610f6c565b348015610653575f5ffd5b506103a46106623660046133c7565b6001600160a01b03165f908152600f602052604090205490565b348015610687575f5ffd5b506102c7610fb1565b34801561069b575f5ffd5b506106a4610fe3565b6040516102f497969594939291906133e0565b3480156106c2575f5ffd5b5060175461033d906001600160a01b031681565b3480156106e1575f5ffd5b506103a46106f0366004613309565b611025565b348015610700575f5ffd5b5061071461070f36600461333b565b611084565b6040516001600160401b0390911681526020016102f4565b348015610737575f5ffd5b506102e861074636600461339d565b6110cb565b348015610756575f5ffd5b5060405165ffffffffffff421681526020016102f4565b348015610778575f5ffd5b506103116110f5565b34801561078c575f5ffd5b506103a461079b3660046133c7565b611104565b3480156107ab575f5ffd5b506103a45f81565b3480156107be575f5ffd5b506102c76107cd366004613476565b611133565b3480156107dd575f5ffd5b506102c76107ec36600461357e565b61113e565b3480156107fc575f5ffd5b506102c761080b3660046135e1565b611155565b34801561081b575f5ffd5b506102c761082a3660046133c7565b611211565b34801561083a575f5ffd5b50610311610849366004613309565b611314565b348015610859575f5ffd5b5061086d610868366004613650565b61131f565b6040516102f49392919061366b565b348015610887575f5ffd5b506102c761089636600461339d565b6113dd565b6102c76108a93660046136a6565b611401565b3480156108b9575f5ffd5b506108cd6108c8366004613650565b611594565b6040516102f491906136d7565b3480156108e5575f5ffd5b506103a47f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a81565b348015610918575f5ffd5b506102e8610927366004613716565b61168d565b348015610937575f5ffd5b5061071461094636600461333b565b6116ba565b6109757fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b336110cb565b6109be5760405162461bcd60e51b815260206004820152601560248201527427b7363c9039b2b93b34b1b290383937bb34b232b960591b60448201526064015b60405180910390fd5b5f6109cb8486018661373e565b6020808201516001600160401b0381165f908152601390925260409091205491925090600160401b90046001600160a01b0316610a435760405162461bcd60e51b8152602060048201526016602482015275151c9a59d9d95c88191bd95cc81b9bdd08195e1a5cdd60521b60448201526064016109b5565b5f82604001515111610a865760405162461bcd60e51b815260206004820152600c60248201526b55524920697320656d70747960a01b60448201526064016109b5565b6001600160401b0381165f90815260136020526040812080546001600160e01b031916815590610ab96001830182613186565b5050601680545f9182610acb836137f6565b919050559050610ade835f01518261176e565b610aec818460400151611787565b80835f01516001600160a01b03167fd35bb95e09c04b219e35047ce7b7b300e3384264ef84a40456943dbc0fc17c148560400151604051610b2d91906132f7565b60405180910390a350505050505050565b5f610b4882611791565b92915050565b60605f8054610b5c9061380e565b80601f0160208091040260200160405190810160405280929190818152602001828054610b889061380e565b8015610bd35780601f10610baa57610100808354040283529160200191610bd3565b820191905f5260205f20905b815481529060010190602001808311610bb657829003601f168201915b5050505050905090565b5f610be7826117b5565b505f828152600460205260409020546001600160a01b0316610b48565b610c0f8282336117ed565b5050565b6001600160a01b038216610c3c57604051633250574960e11b81525f60048201526024016109b5565b5f610c488383336117fa565b9050836001600160a01b0316816001600160a01b031614610c96576040516364283d7b60e01b81526001600160a01b03808616600483015260248201849052821660448201526064016109b5565b50505050565b5f828152600c6020526040902060010154610cb68161180e565b610c968383611818565b5f610cca83610f6c565b8210610cfb5760405163295f44f760e21b81526001600160a01b0384166004820152602481018390526044016109b5565b506001600160a01b03919091165f908152600660209081526040808320938352929052205490565b6001600160a01b0381163314610d4c5760405163334bd91960e11b815260040160405180910390fd5b610d5682826118a9565b505050565b5f4265ffffffffffff81168310610d9657604051637669fc0f60e11b81526004810184905265ffffffffffff821660248201526044016109b5565b610dbf610da284611914565b6001600160a01b0386165f9081526011602052604090209061194a565b6001600160d01b0316949350505050565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610dfa8161180e565b610e026119fa565b50565b610d5683838360405180602001604052805f81525061113e565b610c0f5f82336117fa565b5f610e348161180e565b4780610e7b5760405162461bcd60e51b81526020600482015260166024820152754e6f2062616c616e636520746f20776974686472617760501b60448201526064016109b5565b6040515f90339083908381818185875af1925050503d805f8114610eba576040519150601f19603f3d011682016040523d82523d5f602084013e610ebf565b606091505b5050905080610d565760405162461bcd60e51b815260206004820152600f60248201526e151c985b9cd9995c8819985a5b1959608a1b60448201526064016109b5565b5f610f0c60085490565b8210610f345760405163295f44f760e21b81525f6004820152602481018390526044016109b5565b60088281548110610f4757610f47613846565b905f5260205f2001549050919050565b33610c0f8183611a4c565b5f610b48826117b5565b5f6001600160a01b038216610f96576040516322718ad960e21b81525f60048201526024016109b5565b506001600160a01b03165f9081526003602052604090205490565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610fdb8161180e565b610e02611abd565b5f6060805f5f5f6060610ff4611afa565b610ffc611b2c565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f4265ffffffffffff8116831061106057604051637669fc0f60e11b81526004810184905265ffffffffffff821660248201526044016109b5565b61107461106c84611914565b60129061194a565b6001600160d01b03169392505050565b6014602052815f5260405f20818154811061109d575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f918252600c602090815260408084206001600160a01b0393909316845291905290205460ff1690565b606060018054610b5c9061380e565b6001600160a01b0381165f90815260116020526040812061112490611b59565b6001600160d01b031692915050565b610c0f338383611b91565b611149848484610c13565b610c9684848484611c27565b8342111561117957604051632341d78760e11b8152600481018590526024016109b5565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f906111f2906111ea9060a00160405160208183030381529060405280519060200120611d4d565b858585611d79565b90506111fe8187611da5565b6112088188611a4c565b50505050505050565b5f61121b8161180e565b601754600160a01b900460ff161561126b5760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b60448201526064016109b5565b6001600160a01b0382166112c15760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420736572766963652070726f7669646572000000000000000060448201526064016109b5565b6112eb7fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b83611818565b5050601780546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b6060610b4882611df7565b60136020525f9081526040902080546001820180546001600160401b03831693600160401b9093046001600160a01b031692919061135c9061380e565b80601f01602080910402602001604051908101604052809291908181526020018280546113889061380e565b80156113d35780601f106113aa576101008083540402835291602001916113d3565b820191905f5260205f20905b8154815290600101906020018083116113b657829003601f168201915b5050505050905083565b5f828152600c60205260409020600101546113f78161180e565b610c9683836118a9565b3467016345785d8a0000146114585760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e31204554480060448201526064016109b5565b601580545f916001600160401b0390911690826114748361385a565b82546001600160401b039182166101009390930a928302928202191691909117909155604080516060810182528383168082523360208084019182528385018981525f9384526013909152939091208251815492516001600160a01b0316600160401b026001600160e01b0319909316951694909417178355905192935091829190600182019061150590826138c8565b5050335f818152601460209081526040808320805460018101825590845291909220600482040180546001600160401b0380891660086003909516949094026101000a8481029102199091161790559051919250907ff3f411d853486b9f53da63009a21cd284ea18a800d4de55ce5bd935d197e4cf1906115879087906132f7565b60405180910390a3505050565b60408051606080820183525f8083526020830152918101919091526001600160401b038281165f90815260136020908152604091829020825160608101845281549485168152600160401b9094046001600160a01b03169184019190915260018101805491928401916116069061380e565b80601f01602080910402602001604051908101604052809291908181526020018280546116329061380e565b801561167d5780601f106116545761010080835404028352916020019161167d565b820191905f5260205f20905b81548152906001019060200180831161166057829003601f168201915b5050505050815250509050919050565b6001600160a01b039182165f90815260056020908152604080832093909416825291909152205460ff1690565b6001600160a01b0382165f9081526014602052604081205482106117165760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b60448201526064016109b5565b6001600160a01b0383165f90815260146020526040902080548390811061173f5761173f613846565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b0316905092915050565b610c0f828260405180602001604052805f815250611efa565b610c0f8282611f10565b5f6001600160e01b03198216637965db0b60e01b1480610b485750610b4882611f5f565b5f818152600260205260408120546001600160a01b031680610b4857604051637e27328960e01b8152600481018490526024016109b5565b610d568383836001611f83565b5f611806848484612087565b949350505050565b610e0281336120a2565b5f61182383836110cb565b6118a2575f838152600c602090815260408083206001600160a01b03861684529091529020805460ff1916600117905561185a3390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4506001610b48565b505f610b48565b5f6118b483836110cb565b156118a2575f838152600c602090815260408083206001600160a01b0386168085529252808320805460ff1916905551339286917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a4506001610b48565b5f65ffffffffffff821115611946576040516306dfcc6560e41b815260306004820152602481018390526044016109b5565b5090565b81545f90818160058111156119a6575f611963846120db565b61196d9085613982565b5f8881526020902090915081015465ffffffffffff9081169087161015611996578091506119a4565b6119a1816001613995565b92505b505b5f6119b3878785856121bf565b905080156119ed576119d7876119ca600184613982565b5f91825260209091200190565b54600160301b90046001600160d01b03166119ef565b5f5b979650505050505050565b611a0261221e565b600b805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038281165f8181526010602052604080822080548686166001600160a01b0319821681179092559151919094169392849290917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4610d568183611ab886612243565b61224d565b611ac56123b6565b600b805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258611a2f3390565b6060611b277f0000000000000000000000000000000000000000000000000000000000000000600d6123da565b905090565b6060611b277f0000000000000000000000000000000000000000000000000000000000000000600e6123da565b80545f908015611b8857611b72836119ca600184613982565b54600160301b90046001600160d01b0316611b8a565b5f5b9392505050565b6001600160a01b038216611bc357604051630b61174360e31b81526001600160a01b03831660048201526024016109b5565b6001600160a01b038381165f81815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c319101611587565b6001600160a01b0383163b15610c9657604051630a85bd0160e11b81526001600160a01b0384169063150b7a0290611c699033908890879087906004016139a8565b6020604051808303815f875af1925050508015611ca3575060408051601f3d908101601f19168201909252611ca0918101906139e4565b60015b611d0a573d808015611cd0576040519150601f19603f3d011682016040523d82523d5f602084013e611cd5565b606091505b5080515f03611d0257604051633250574960e11b81526001600160a01b03851660048201526024016109b5565b805181602001fd5b6001600160e01b03198116630a85bd0160e11b14611d4657604051633250574960e11b81526001600160a01b03851660048201526024016109b5565b5050505050565b5f610b48611d59612483565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f5f611d89888888886125ac565b925092509250611d998282612674565b50909695505050505050565b6001600160a01b0382165f908152600f60205260409020805460018101909155818114610d56576040516301d4b62360e61b81526001600160a01b0384166004820152602481018290526044016109b5565b6060611e02826117b5565b505f828152600a602052604081208054611e1b9061380e565b80601f0160208091040260200160405190810160405280929190818152602001828054611e479061380e565b8015611e925780601f10611e6957610100808354040283529160200191611e92565b820191905f5260205f20905b815481529060010190602001808311611e7557829003601f168201915b505050505090505f611eae60408051602081019091525f815290565b905080515f03611ebf575092915050565b815115611ef1578082604051602001611ed99291906139ff565b60405160208183030381529060405292505050919050565b6118068461272c565b611f04838361279c565b610d565f848484611c27565b5f828152600a60205260409020611f2782826138c8565b506040518281527ff8e1a15aba9398e019f0b49df1a4fde98ee17ae345cb5f6b5e2c27f5033e8ce79060200160405180910390a15050565b5f6001600160e01b03198216632483248360e11b1480610b485750610b48826127fd565b8080611f9757506001600160a01b03821615155b15612058575f611fa6846117b5565b90506001600160a01b03831615801590611fd25750826001600160a01b0316816001600160a01b031614155b8015611fe55750611fe3818461168d565b155b1561200e5760405163a9fbf51f60e01b81526001600160a01b03841660048201526024016109b5565b81156120565783856001600160a01b0316826001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45b505b50505f90815260046020526040902080546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f612094858585612821565b905061180681866001612835565b6120ac82826110cb565b610c0f5760405163e2517d3f60e01b81526001600160a01b0382166004820152602481018390526044016109b5565b5f815f036120ea57505f919050565b5f60016120f6846128aa565b901c6001901b9050600181848161210f5761210f613a2d565b048201901c9050600181848161212757612127613a2d565b048201901c9050600181848161213f5761213f613a2d565b048201901c9050600181848161215757612157613a2d565b048201901c9050600181848161216f5761216f613a2d565b048201901c9050600181848161218757612187613a2d565b048201901c9050600181848161219f5761219f613a2d565b048201901c9050611b8a818285816121b9576121b9613a2d565b0461293d565b5f5b81831015612216575f6121d48484612952565b5f8781526020902090915065ffffffffffff86169082015465ffffffffffff16111561220257809250612210565b61220d816001613995565b93505b506121c1565b509392505050565b600b5460ff1661224157604051638dfc202b60e01b815260040160405180910390fd5b565b5f610b4882610f6c565b816001600160a01b0316836001600160a01b03161415801561226e57505f81115b15610d56576001600160a01b03831615612315576001600160a01b0383165f90815260116020526040812081906122b09061296c6122ab86612977565b6129aa565b6001600160d01b031691506001600160d01b03169150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161230a929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615610d56576001600160a01b0382165f908152601160205260408120819061234d906129db6122ab86612977565b6001600160d01b031691506001600160d01b03169150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516123a7929190918252602082015260400190565b60405180910390a25050505050565b600b5460ff16156122415760405163d93c066560e01b815260040160405180910390fd5b606060ff83146123f4576123ed836129e6565b9050610b48565b8180546124009061380e565b80601f016020809104026020016040519081016040528092919081815260200182805461242c9061380e565b80156124775780601f1061244e57610100808354040283529160200191612477565b820191905f5260205f20905b81548152906001019060200180831161245a57829003601f168201915b50505050509050610b48565b5f306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480156124db57507f000000000000000000000000000000000000000000000000000000000000000046145b1561250557507f000000000000000000000000000000000000000000000000000000000000000090565b611b27604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201527f0000000000000000000000000000000000000000000000000000000000000000918101919091527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411156125e557505f9150600390508261266a565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612636573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b03811661266157505f92506001915082905061266a565b92505f91508190505b9450945094915050565b5f82600381111561268757612687613a41565b03612690575050565b60018260038111156126a4576126a4613a41565b036126c25760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156126d6576126d6613a41565b036126f75760405163fce698f760e01b8152600481018290526024016109b5565b600382600381111561270b5761270b613a41565b03610c0f576040516335e2f38360e21b8152600481018290526024016109b5565b6060612737826117b5565b505f61274d60408051602081019091525f815290565b90505f81511161276b5760405180602001604052805f815250611b8a565b8061277584612a23565b6040516020016127869291906139ff565b6040516020818303038152906040529392505050565b6001600160a01b0382166127c557604051633250574960e11b81525f60048201526024016109b5565b5f6127d183835f6117fa565b90506001600160a01b03811615610d56576040516339e3563760e11b81525f60048201526024016109b5565b5f6001600160e01b0319821663780e9d6360e01b1480610b485750610b4882612ab2565b5f61282a6123b6565b611806848484612b01565b6001600160a01b0383166128575761285460126129db6122ab84612977565b50505b6001600160a01b03821661287957612876601261296c6122ab84612977565b50505b6001600160a01b038381165f90815260106020526040808220548584168352912054610d569291821691168361224d565b5f80608083901c156128be57608092831c92015b604083901c156128d057604092831c92015b602083901c156128e257602092831c92015b601083901c156128f457601092831c92015b600883901c1561290657600892831c92015b600483901c1561291857600492831c92015b600283901c1561292a57600292831c92015b600183901c15610b485760010192915050565b5f81831061294b5781611b8a565b5090919050565b5f6129606002848418613a55565b611b8a90848416613995565b5f611b8a8284613a74565b5f6001600160d01b03821115611946576040516306dfcc6560e41b815260d06004820152602481018390526044016109b5565b5f806129ce426129c66129bc88611b59565b868863ffffffff16565b879190612bcc565b915091505b935093915050565b5f611b8a8284613a93565b60605f6129f283612bd9565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b60605f612a2f83612c00565b60010190505f816001600160401b03811115612a4d57612a4d6134af565b6040519080825280601f01601f191660200182016040528015612a77576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a8504945084612a8157509392505050565b5f6001600160e01b031982166380ac58cd60e01b1480612ae257506001600160e01b03198216635b5e139f60e01b145b80610b4857506301ffc9a760e01b6001600160e01b0319831614610b48565b5f5f612b0e858585612cd7565b90506001600160a01b038116612b6a57612b6584600880545f838152600960205260408120829055600182018355919091527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee30155565b612b8d565b846001600160a01b0316816001600160a01b031614612b8d57612b8d8185612dc9565b6001600160a01b038516612ba957612ba484612e56565b611806565b846001600160a01b0316816001600160a01b031614611806576118068585612efd565b5f806129ce858585612f4b565b5f60ff8216601f811115610b4857604051632cd44ac360e21b815260040160405180910390fd5b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310612c3e5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310612c6a576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310612c8857662386f26fc10000830492506010015b6305f5e1008310612ca0576305f5e100830492506008015b6127108310612cb457612710830492506004015b60648310612cc6576064830492506002015b600a8310610b485760010192915050565b5f828152600260205260408120546001600160a01b0390811690831615612d0357612d038184866130c1565b6001600160a01b03811615612d3d57612d1e5f855f5f611f83565b6001600160a01b0381165f90815260036020526040902080545f190190555b6001600160a01b03851615612d6b576001600160a01b0385165f908152600360205260409020805460010190555b5f8481526002602052604080822080546001600160a01b0319166001600160a01b0389811691821790925591518793918516917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4949350505050565b5f612dd383610f6c565b5f83815260076020526040902054909150808214612e24576001600160a01b0384165f9081526006602090815260408083208584528252808320548484528184208190558352600790915290208190555b505f9182526007602090815260408084208490556001600160a01b039094168352600681528383209183525290812055565b6008545f90612e6790600190613982565b5f8381526009602052604081205460088054939450909284908110612e8e57612e8e613846565b905f5260205f20015490508060088381548110612ead57612ead613846565b5f918252602080832090910192909255828152600990915260408082208490558582528120556008805480612ee457612ee4613ab2565b600190038181905f5260205f20015f9055905550505050565b5f6001612f0984610f6c565b612f139190613982565b6001600160a01b039093165f908152600660209081526040808320868452825280832085905593825260079052919091209190915550565b82545f9081908015613067575f612f67876119ca600185613982565b60408051808201909152905465ffffffffffff808216808452600160301b9092046001600160d01b031660208401529192509087161015612fbb57604051632520601d60e01b815260040160405180910390fd5b805165ffffffffffff8088169116036130075784612fde886119ca600186613982565b80546001600160d01b0392909216600160301b0265ffffffffffff909216919091179055613057565b6040805180820190915265ffffffffffff80881682526001600160d01b0380881660208085019182528b54600181018d555f8d81529190912094519151909216600160301b029216919091179101555b6020015192508391506129d39050565b50506040805180820190915265ffffffffffff80851682526001600160d01b0380851660208085019182528854600181018a555f8a815291822095519251909316600160301b0291909316179201919091559050816129d3565b6130cc838383613125565b610d56576001600160a01b0383166130fa57604051637e27328960e01b8152600481018290526024016109b5565b60405163177e802f60e01b81526001600160a01b0383166004820152602481018290526044016109b5565b5f6001600160a01b038316158015906118065750826001600160a01b0316846001600160a01b0316148061315e575061315e848461168d565b806118065750505f908152600460205260409020546001600160a01b03908116911614919050565b5080546131929061380e565b5f825580601f106131a1575050565b601f0160209004905f5260205f2090810190610e0291905b80821115611946575f81556001016131b9565b5f5f83601f8401126131dc575f5ffd5b5081356001600160401b038111156131f2575f5ffd5b602083019150836020828501011115613209575f5ffd5b9250929050565b5f5f5f5f60408587031215613223575f5ffd5b84356001600160401b03811115613238575f5ffd5b613244878288016131cc565b90955093505060208501356001600160401b03811115613262575f5ffd5b61326e878288016131cc565b95989497509550505050565b6001600160e01b031981168114610e02575f5ffd5b5f6020828403121561329f575f5ffd5b8135611b8a8161327a565b5f5b838110156132c45781810151838201526020016132ac565b50505f910152565b5f81518084526132e38160208601602086016132aa565b601f01601f19169290920160200192915050565b602081525f611b8a60208301846132cc565b5f60208284031215613319575f5ffd5b5035919050565b80356001600160a01b0381168114613336575f5ffd5b919050565b5f5f6040838503121561334c575f5ffd5b61335583613320565b946020939093013593505050565b5f5f5f60608486031215613375575f5ffd5b61337e84613320565b925061338c60208501613320565b929592945050506040919091013590565b5f5f604083850312156133ae575f5ffd5b823591506133be60208401613320565b90509250929050565b5f602082840312156133d7575f5ffd5b611b8a82613320565b60ff60f81b8816815260e060208201525f6133fe60e08301896132cc565b828103604084015261341081896132cc565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015613465578351835260209384019390920191600101613447565b50909b9a5050505050505050505050565b5f5f60408385031215613487575f5ffd5b61349083613320565b9150602083013580151581146134a4575f5ffd5b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b03811182821017156134e5576134e56134af565b60405290565b5f5f6001600160401b03841115613504576135046134af565b50604051601f19601f85018116603f011681018181106001600160401b0382111715613532576135326134af565b604052838152905080828401851015613549575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f83011261356f575f5ffd5b611b8a838335602085016134eb565b5f5f5f5f60808587031215613591575f5ffd5b61359a85613320565b93506135a860208601613320565b92506040850135915060608501356001600160401b038111156135c9575f5ffd5b6135d587828801613560565b91505092959194509250565b5f5f5f5f5f5f60c087890312156135f6575f5ffd5b6135ff87613320565b95506020870135945060408701359350606087013560ff81168114613622575f5ffd5b9598949750929560808101359460a0909101359350915050565b6001600160401b0381168114610e02575f5ffd5b5f60208284031215613660575f5ffd5b8135611b8a8161363c565b6001600160401b03841681526001600160a01b03831660208201526060604082018190525f9061369d908301846132cc565b95945050505050565b5f602082840312156136b6575f5ffd5b81356001600160401b038111156136cb575f5ffd5b61180684828501613560565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f604083015160608084015261180660808401826132cc565b5f5f60408385031215613727575f5ffd5b61373083613320565b91506133be60208401613320565b5f6020828403121561374e575f5ffd5b81356001600160401b03811115613763575f5ffd5b820160608185031215613774575f5ffd5b61377c6134c3565b61378582613320565b815260208201356137958161363c565b602082015260408201356001600160401b038111156137b2575f5ffd5b80830192505084601f8301126137c6575f5ffd5b6137d5858335602085016134eb565b6040820152949350505050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201613807576138076137e2565b5060010190565b600181811c9082168061382257607f821691505b60208210810361384057634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f6001600160401b0382166001600160401b03810361387b5761387b6137e2565b60010192915050565b601f821115610d5657805f5260205f20601f840160051c810160208510156138a95750805b601f840160051c820191505b81811015611d46575f81556001016138b5565b81516001600160401b038111156138e1576138e16134af565b6138f5816138ef845461380e565b84613884565b6020601f821160018114613927575f83156139105750848201515b5f19600385901b1c1916600184901b178455611d46565b5f84815260208120601f198516915b828110156139565787850151825560209485019460019092019101613936565b508482101561397357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81810381811115610b4857610b486137e2565b80820180821115610b4857610b486137e2565b6001600160a01b03858116825284166020820152604081018390526080606082018190525f906139da908301846132cc565b9695505050505050565b5f602082840312156139f4575f5ffd5b8151611b8a8161327a565b5f8351613a108184602088016132aa565b835190830190613a248183602088016132aa565b01949350505050565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b5f82613a6f57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160d01b038281168282160390811115610b4857610b486137e2565b6001600160d01b038181168382160190811115610b4857610b486137e2565b634e487b7160e01b5f52603160045260245ffdfea2646970667358221220d9750f5cc7bb6a1014375f6329d701ed6c5e748053095e9004dac55dacbe936364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W__\xFD[P`@\x80Q\x80\x82\x01\x82R`\n\x80\x82Ri\x15\x1C\x9AY\xD9\xD9\\\x93\x91\x95`\xB2\x1B` \x80\x84\x01\x82\x90R\x84Q\x80\x86\x01\x86R`\x01\x81R`1`\xF8\x1B\x81\x83\x01R\x85Q\x80\x87\x01\x87R\x93\x84R\x83\x82\x01\x92\x90\x92R\x84Q\x80\x86\x01\x90\x95R`\x04\x85Rc\x15\x13\x91\x95`\xE2\x1B\x90\x85\x01R\x91\x92_a\0\x7F\x83\x82a\x03NV[P`\x01a\0\x8C\x82\x82a\x03NV[PP`\x0B\x80T`\xFF\x19\x16\x90UPa\0\xA4\x82`\ra\x01\x93V[a\x01 Ra\0\xB3\x81`\x0Ea\x01\x93V[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\x01?`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Ra\x01R_3a\x01\xC5V[Pa\x01}\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*3a\x01\xC5V[P`\x17\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90Ua\x04vV[_` \x83Q\x10\x15a\x01\xAEWa\x01\xA7\x83a\x02pV[\x90Pa\x01\xBFV[\x81a\x01\xB9\x84\x82a\x03NV[P`\xFF\x90P[\x92\x91PPV[_\x82\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x02iW_\x83\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x02!3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x01\xBFV[P_a\x01\xBFV[__\x82\x90P`\x1F\x81Q\x11\x15a\x02\xA3W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x02\x9A\x91\x90a\x04\x08V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\x02\xAE\x82a\x04SV[\x17\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x02\xDEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x02\xFCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x03IW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x03'WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x03FW_\x81U`\x01\x01a\x033V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03gWa\x03ga\x02\xB6V[a\x03{\x81a\x03u\x84Ta\x02\xCAV[\x84a\x03\x02V[` `\x1F\x82\x11`\x01\x81\x14a\x03\xADW_\x83\x15a\x03\x96WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x03FV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x03\xDCW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x03\xBCV[P\x84\x82\x10\x15a\x03\xF9W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15a\x044W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x04\x17V[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\xFCW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa:\xFCa\x04\xC7_9_a\x1B3\x01R_a\x1B\x01\x01R_a%\\\x01R_a%4\x01R_a$\x8F\x01R_a$\xB9\x01R_a$\xE3\x01Ra:\xFC_\xF3\xFE`\x80`@R`\x046\x10a\x02\xA4W_5`\xE0\x1C\x80ccR!\x1E\x11a\x01oW\x80c\xA2\x17\xFD\xDF\x11a\0\xD8W\x80c\xCE(\x96\x12\x11a\0\x92W\x80c\xE3(\xEDw\x11a\0mW\x80c\xE3(\xEDw\x14a\x08\xAEW\x80c\xE6:\xB1\xE9\x14a\x08\xDAW\x80c\xE9\x85\xE9\xC5\x14a\t\rW\x80c\xF7\xEE\x94L\x14a\t,W__\xFD[\x80c\xCE(\x96\x12\x14a\x08NW\x80c\xD5Gt\x1F\x14a\x08|W\x80c\xE3\x1E\x07\x88\x14a\x08\x9BW__\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x07\xA0W\x80c\xA2,\xB4e\x14a\x07\xB3W\x80c\xB8\x8DO\xDE\x14a\x07\xD2W\x80c\xC3\xCD\xA5 \x14a\x07\xF1W\x80c\xC4\xD6m\xE8\x14a\x08\x10W\x80c\xC8{V\xDD\x14a\x08/W__\xFD[\x80c\x8ES\x9E\x8C\x11a\x01)W\x80c\x8ES\x9E\x8C\x14a\x06\xD6W\x80c\x91;\x1F\xBF\x14a\x06\xF5W\x80c\x91\xD1HT\x14a\x07,W\x80c\x91\xDD\xAD\xF4\x14a\x07KW\x80c\x95\xD8\x9BA\x14a\x07mW\x80c\x9A\xB2N\xB0\x14a\x07\x81W__\xFD[\x80ccR!\x1E\x14a\x06\nW\x80cp\xA0\x821\x14a\x06)W\x80c~\xCE\xBE\0\x14a\x06HW\x80c\x84V\xCBY\x14a\x06|W\x80c\x84\xB0\x19n\x14a\x06\x90W\x80c\x8Di\xE9^\x14a\x06\xB7W__\xFD[\x80c3\x83\xAB\xFE\x11a\x02\x11W\x80cGcC\xEE\x11a\x01\xCBW\x80cGcC\xEE\x14a\x054W\x80cK\xF5\xD7\xE9\x14a\x05HW\x80cOl\xCC\xE7\x14a\x05~W\x80cX|\xDE\x1E\x14a\x05\x9DW\x80c\\\x19\xA9\\\x14a\x05\xD4W\x80c\\\x97Z\xBB\x14a\x05\xF3W__\xFD[\x80c3\x83\xAB\xFE\x14a\x04pW\x80c6V\x8A\xBE\x14a\x04\xA4W\x80c:F\xB1\xA8\x14a\x04\xC3W\x80c?K\xA8:\x14a\x04\xE2W\x80cB\x84.\x0E\x14a\x04\xF6W\x80cB\x96lh\x14a\x05\x15W__\xFD[\x80c\x18\x16\r\xDD\x11a\x02bW\x80c\x18\x16\r\xDD\x14a\x03\x94W\x80c\x1B\xC6\xAE\x8A\x14a\x03\xB2W\x80c#\xB8r\xDD\x14a\x03\xE5W\x80c$\x8A\x9C\xA3\x14a\x04\x04W\x80c//\xF1]\x14a\x042W\x80c/t\\Y\x14a\x04QW__\xFD[\x80bs\xE1\xD7\x14a\x02\xA8W\x80c\x01\xFF\xC9\xA7\x14a\x02\xC9W\x80c\x06\xFD\xDE\x03\x14a\x02\xFDW\x80c\x08\x18\x12\xFC\x14a\x03\x1EW\x80c\t^\xA7\xB3\x14a\x03UW\x80c\x15\x8E\xF9>\x14a\x03tW[__\xFD[4\x80\x15a\x02\xB3W__\xFD[Pa\x02\xC7a\x02\xC26`\x04a2\x10V[a\tKV[\0[4\x80\x15a\x02\xD4W__\xFD[Pa\x02\xE8a\x02\xE36`\x04a2\x8FV[a\x0B>V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x08W__\xFD[Pa\x03\x11a\x0BNV[`@Qa\x02\xF4\x91\x90a2\xF7V[4\x80\x15a\x03)W__\xFD[Pa\x03=a\x0386`\x04a3\tV[a\x0B\xDDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x03`W__\xFD[Pa\x02\xC7a\x03o6`\x04a3;V[a\x0C\x04V[4\x80\x15a\x03\x7FW__\xFD[P`\x17Ta\x02\xE8\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x03\x9FW__\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x02\xF4V[4\x80\x15a\x03\xBDW__\xFD[Pa\x03\xA4\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x81V[4\x80\x15a\x03\xF0W__\xFD[Pa\x02\xC7a\x03\xFF6`\x04a3cV[a\x0C\x13V[4\x80\x15a\x04\x0FW__\xFD[Pa\x03\xA4a\x04\x1E6`\x04a3\tV[_\x90\x81R`\x0C` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04=W__\xFD[Pa\x02\xC7a\x04L6`\x04a3\x9DV[a\x0C\x9CV[4\x80\x15a\x04\\W__\xFD[Pa\x03\xA4a\x04k6`\x04a3;V[a\x0C\xC0V[4\x80\x15a\x04{W__\xFD[Pa\x03\xA4a\x04\x8A6`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x14` R`@\x90 T\x90V[4\x80\x15a\x04\xAFW__\xFD[Pa\x02\xC7a\x04\xBE6`\x04a3\x9DV[a\r#V[4\x80\x15a\x04\xCEW__\xFD[Pa\x03\xA4a\x04\xDD6`\x04a3;V[a\r[V[4\x80\x15a\x04\xEDW__\xFD[Pa\x02\xC7a\r\xD0V[4\x80\x15a\x05\x01W__\xFD[Pa\x02\xC7a\x05\x106`\x04a3cV[a\x0E\x05V[4\x80\x15a\x05 W__\xFD[Pa\x02\xC7a\x05/6`\x04a3\tV[a\x0E\x1FV[4\x80\x15a\x05?W__\xFD[Pa\x02\xC7a\x0E*V[4\x80\x15a\x05SW__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x03\x11V[4\x80\x15a\x05\x89W__\xFD[Pa\x03\xA4a\x05\x986`\x04a3\tV[a\x0F\x02V[4\x80\x15a\x05\xA8W__\xFD[Pa\x03=a\x05\xB76`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x10` R`@\x90 T\x16\x90V[4\x80\x15a\x05\xDFW__\xFD[Pa\x02\xC7a\x05\xEE6`\x04a3\xC7V[a\x0FWV[4\x80\x15a\x05\xFEW__\xFD[P`\x0BT`\xFF\x16a\x02\xE8V[4\x80\x15a\x06\x15W__\xFD[Pa\x03=a\x06$6`\x04a3\tV[a\x0FbV[4\x80\x15a\x064W__\xFD[Pa\x03\xA4a\x06C6`\x04a3\xC7V[a\x0FlV[4\x80\x15a\x06SW__\xFD[Pa\x03\xA4a\x06b6`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x0F` R`@\x90 T\x90V[4\x80\x15a\x06\x87W__\xFD[Pa\x02\xC7a\x0F\xB1V[4\x80\x15a\x06\x9BW__\xFD[Pa\x06\xA4a\x0F\xE3V[`@Qa\x02\xF4\x97\x96\x95\x94\x93\x92\x91\x90a3\xE0V[4\x80\x15a\x06\xC2W__\xFD[P`\x17Ta\x03=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xE1W__\xFD[Pa\x03\xA4a\x06\xF06`\x04a3\tV[a\x10%V[4\x80\x15a\x07\0W__\xFD[Pa\x07\x14a\x07\x0F6`\x04a3;V[a\x10\x84V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x077W__\xFD[Pa\x02\xE8a\x07F6`\x04a3\x9DV[a\x10\xCBV[4\x80\x15a\x07VW__\xFD[P`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x07xW__\xFD[Pa\x03\x11a\x10\xF5V[4\x80\x15a\x07\x8CW__\xFD[Pa\x03\xA4a\x07\x9B6`\x04a3\xC7V[a\x11\x04V[4\x80\x15a\x07\xABW__\xFD[Pa\x03\xA4_\x81V[4\x80\x15a\x07\xBEW__\xFD[Pa\x02\xC7a\x07\xCD6`\x04a4vV[a\x113V[4\x80\x15a\x07\xDDW__\xFD[Pa\x02\xC7a\x07\xEC6`\x04a5~V[a\x11>V[4\x80\x15a\x07\xFCW__\xFD[Pa\x02\xC7a\x08\x0B6`\x04a5\xE1V[a\x11UV[4\x80\x15a\x08\x1BW__\xFD[Pa\x02\xC7a\x08*6`\x04a3\xC7V[a\x12\x11V[4\x80\x15a\x08:W__\xFD[Pa\x03\x11a\x08I6`\x04a3\tV[a\x13\x14V[4\x80\x15a\x08YW__\xFD[Pa\x08ma\x08h6`\x04a6PV[a\x13\x1FV[`@Qa\x02\xF4\x93\x92\x91\x90a6kV[4\x80\x15a\x08\x87W__\xFD[Pa\x02\xC7a\x08\x966`\x04a3\x9DV[a\x13\xDDV[a\x02\xC7a\x08\xA96`\x04a6\xA6V[a\x14\x01V[4\x80\x15a\x08\xB9W__\xFD[Pa\x08\xCDa\x08\xC86`\x04a6PV[a\x15\x94V[`@Qa\x02\xF4\x91\x90a6\xD7V[4\x80\x15a\x08\xE5W__\xFD[Pa\x03\xA4\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[4\x80\x15a\t\x18W__\xFD[Pa\x02\xE8a\t'6`\x04a7\x16V[a\x16\x8DV[4\x80\x15a\t7W__\xFD[Pa\x07\x14a\tF6`\x04a3;V[a\x16\xBAV[a\tu\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K3a\x10\xCBV[a\t\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt'\xB76<\x909\xB2\xB9;4\xB1\xB2\x90897\xBB4\xB22\xB9`Y\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\t\xCB\x84\x86\x01\x86a7>V[` \x80\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R`\x13\x90\x92R`@\x90\x91 T\x91\x92P\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\nCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15\x1C\x9AY\xD9\xD9\\\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`R\x1B`D\x82\x01R`d\x01a\t\xB5V[_\x82`@\x01QQ\x11a\n\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkURI is empty`\xA0\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R`\x13` R`@\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\n\xB9`\x01\x83\x01\x82a1\x86V[PP`\x16\x80T_\x91\x82a\n\xCB\x83a7\xF6V[\x91\x90PU\x90Pa\n\xDE\x83_\x01Q\x82a\x17nV[a\n\xEC\x81\x84`@\x01Qa\x17\x87V[\x80\x83_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD3[\xB9^\t\xC0K!\x9E5\x04|\xE7\xB7\xB3\0\xE38Bd\xEF\x84\xA4\x04V\x94=\xBC\x0F\xC1|\x14\x85`@\x01Q`@Qa\x0B-\x91\x90a2\xF7V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_a\x0BH\x82a\x17\x91V[\x92\x91PPV[``_\x80Ta\x0B\\\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x88\x90a8\x0EV[\x80\x15a\x0B\xD3W\x80`\x1F\x10a\x0B\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x0B\xE7\x82a\x17\xB5V[P_\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0BHV[a\x0C\x0F\x82\x823a\x17\xEDV[PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C<W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_a\x0CH\x83\x833a\x17\xFAV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\x96W`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\t\xB5V[PPPPV[_\x82\x81R`\x0C` R`@\x90 `\x01\x01Ta\x0C\xB6\x81a\x18\x0EV[a\x0C\x96\x83\x83a\x18\x18V[_a\x0C\xCA\x83a\x0FlV[\x82\x10a\x0C\xFBW`@Qc)_D\xF7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\rLW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rV\x82\x82a\x18\xA9V[PPPV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\r\x96W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\xB5V[a\r\xBFa\r\xA2\x84a\x19\x14V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x11` R`@\x90 \x90a\x19JV[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\r\xFA\x81a\x18\x0EV[a\x0E\x02a\x19\xFAV[PV[a\rV\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x11>V[a\x0C\x0F_\x823a\x17\xFAV[_a\x0E4\x81a\x18\x0EV[G\x80a\x0E{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuNo balance to withdraw`P\x1B`D\x82\x01R`d\x01a\t\xB5V[`@Q_\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0E\xBAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\xBFV[``\x91P[PP\x90P\x80a\rVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R`d\x01a\t\xB5V[_a\x0F\x0C`\x08T\x90V[\x82\x10a\x0F4W`@Qc)_D\xF7`\xE2\x1B\x81R_`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[`\x08\x82\x81T\x81\x10a\x0FGWa\x0FGa8FV[\x90_R` _ \x01T\x90P\x91\x90PV[3a\x0C\x0F\x81\x83a\x1ALV[_a\x0BH\x82a\x17\xB5V[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0F\x96W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x0F\xDB\x81a\x18\x0EV[a\x0E\x02a\x1A\xBDV[_``\x80___``a\x0F\xF4a\x1A\xFAV[a\x0F\xFCa\x1B,V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x10`W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\xB5V[a\x10ta\x10l\x84a\x19\x14V[`\x12\x90a\x19JV[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[`\x14` R\x81_R`@_ \x81\x81T\x81\x10a\x10\x9DW_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_\x91\x82R`\x0C` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x01\x80Ta\x0B\\\x90a8\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x11` R`@\x81 a\x11$\x90a\x1BYV[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[a\x0C\x0F3\x83\x83a\x1B\x91V[a\x11I\x84\x84\x84a\x0C\x13V[a\x0C\x96\x84\x84\x84\x84a\x1C'V[\x83B\x11\x15a\x11yW`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\t\xB5V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x11\xF2\x90a\x11\xEA\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1DMV[\x85\x85\x85a\x1DyV[\x90Pa\x11\xFE\x81\x87a\x1D\xA5V[a\x12\x08\x81\x88a\x1ALV[PPPPPPPV[_a\x12\x1B\x81a\x18\x0EV[`\x17T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x12kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid service provider\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xB5V[a\x12\xEB\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x83a\x18\x18V[PP`\x17\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[``a\x0BH\x82a\x1D\xF7V[`\x13` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`@\x1B\x03\x83\x16\x93`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x13\\\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x88\x90a8\x0EV[\x80\x15a\x13\xD3W\x80`\x1F\x10a\x13\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[_\x82\x81R`\x0C` R`@\x90 `\x01\x01Ta\x13\xF7\x81a\x18\x0EV[a\x0C\x96\x83\x83a\x18\xA9V[4g\x01cEx]\x8A\0\0\x14a\x14XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\t\xB5V[`\x15\x80T_\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x82a\x14t\x83a8ZV[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x92\x82\x02\x19\x16\x91\x90\x91\x17\x90\x91U`@\x80Q``\x81\x01\x82R\x83\x83\x16\x80\x82R3` \x80\x84\x01\x91\x82R\x83\x85\x01\x89\x81R_\x93\x84R`\x13\x90\x91R\x93\x90\x91 \x82Q\x81T\x92Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x95\x16\x94\x90\x94\x17\x17\x83U\x90Q\x92\x93P\x91\x82\x91\x90`\x01\x82\x01\x90a\x15\x05\x90\x82a8\xC8V[PP3_\x81\x81R`\x14` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x91\x90\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x91\x92P\x90\x7F\xF3\xF4\x11\xD8SHk\x9FS\xDAc\0\x9A!\xCD(N\xA1\x8A\x80\rM\xE5\\\xE5\xBD\x93]\x19~L\xF1\x90a\x15\x87\x90\x87\x90a2\xF7V[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x81\x16_\x90\x81R`\x13` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x16\x06\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x162\x90a8\x0EV[\x80\x15a\x16}W\x80`\x1F\x10a\x16TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x14` R`@\x81 T\x82\x10a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x14` R`@\x90 \x80T\x83\x90\x81\x10a\x17?Wa\x17?a8FV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P\x92\x91PPV[a\x0C\x0F\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x1E\xFAV[a\x0C\x0F\x82\x82a\x1F\x10V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x0BHWPa\x0BH\x82a\x1F_V[_\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x0BHW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\xB5V[a\rV\x83\x83\x83`\x01a\x1F\x83V[_a\x18\x06\x84\x84\x84a \x87V[\x94\x93PPPPV[a\x0E\x02\x813a \xA2V[_a\x18#\x83\x83a\x10\xCBV[a\x18\xA2W_\x83\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x18Z3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x0BHV[P_a\x0BHV[_a\x18\xB4\x83\x83a\x10\xCBV[\x15a\x18\xA2W_\x83\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x0BHV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19FW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[P\x90V[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x19\xA6W_a\x19c\x84a \xDBV[a\x19m\x90\x85a9\x82V[_\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x19\x96W\x80\x91Pa\x19\xA4V[a\x19\xA1\x81`\x01a9\x95V[\x92P[P[_a\x19\xB3\x87\x87\x85\x85a!\xBFV[\x90P\x80\x15a\x19\xEDWa\x19\xD7\x87a\x19\xCA`\x01\x84a9\x82V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x19\xEFV[_[\x97\x96PPPPPPPV[a\x1A\x02a\"\x1EV[`\x0B\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x10` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\rV\x81\x83a\x1A\xB8\x86a\"CV[a\"MV[a\x1A\xC5a#\xB6V[`\x0B\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1A/3\x90V[``a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\ra#\xDAV[\x90P\x90V[``a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0Ea#\xDAV[\x80T_\x90\x80\x15a\x1B\x88Wa\x1Br\x83a\x19\xCA`\x01\x84a9\x82V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x1B\x8AV[_[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B\xC3W`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x15\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x0C\x96W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\x1Ci\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a9\xA8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x1C\xA3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1C\xA0\x91\x81\x01\x90a9\xE4V[`\x01[a\x1D\nW=\x80\x80\x15a\x1C\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\xD5V[``\x91P[P\x80Q_\x03a\x1D\x02W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\xB5V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x1DFW`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\xB5V[PPPPPV[_a\x0BHa\x1DYa$\x83V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a\x1D\x89\x88\x88\x88\x88a%\xACV[\x92P\x92P\x92Pa\x1D\x99\x82\x82a&tV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x0F` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\rVW`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\xB5V[``a\x1E\x02\x82a\x17\xB5V[P_\x82\x81R`\n` R`@\x81 \x80Ta\x1E\x1B\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1EG\x90a8\x0EV[\x80\x15a\x1E\x92W\x80`\x1F\x10a\x1EiWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EuW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P_a\x1E\xAE`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[\x90P\x80Q_\x03a\x1E\xBFWP\x92\x91PPV[\x81Q\x15a\x1E\xF1W\x80\x82`@Q` \x01a\x1E\xD9\x92\x91\x90a9\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[a\x18\x06\x84a',V[a\x1F\x04\x83\x83a'\x9CV[a\rV_\x84\x84\x84a\x1C'V[_\x82\x81R`\n` R`@\x90 a\x1F'\x82\x82a8\xC8V[P`@Q\x82\x81R\x7F\xF8\xE1\xA1Z\xBA\x93\x98\xE0\x19\xF0\xB4\x9D\xF1\xA4\xFD\xE9\x8E\xE1z\xE3E\xCB_k^,'\xF5\x03>\x8C\xE7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c$\x83$\x83`\xE1\x1B\x14\x80a\x0BHWPa\x0BH\x82a'\xFDV[\x80\x80a\x1F\x97WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a XW_a\x1F\xA6\x84a\x17\xB5V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1F\xD2WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x1F\xE5WPa\x1F\xE3\x81\x84a\x16\x8DV[\x15[\x15a \x0EW`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\t\xB5V[\x81\x15a VW\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP_\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[__a \x94\x85\x85\x85a(!V[\x90Pa\x18\x06\x81\x86`\x01a(5V[a \xAC\x82\x82a\x10\xCBV[a\x0C\x0FW`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[_\x81_\x03a \xEAWP_\x91\x90PV[_`\x01a \xF6\x84a(\xAAV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a!\x0FWa!\x0Fa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!'Wa!'a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!?Wa!?a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!WWa!Wa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!oWa!oa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!\x87Wa!\x87a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!\x9FWa!\x9Fa:-V[\x04\x82\x01\x90\x1C\x90Pa\x1B\x8A\x81\x82\x85\x81a!\xB9Wa!\xB9a:-V[\x04a)=V[_[\x81\x83\x10\x15a\"\x16W_a!\xD4\x84\x84a)RV[_\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\"\x02W\x80\x92Pa\"\x10V[a\"\r\x81`\x01a9\x95V[\x93P[Pa!\xC1V[P\x93\x92PPPV[`\x0BT`\xFF\x16a\"AW`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x0BH\x82a\x0FlV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\"nWP_\x81\x11[\x15a\rVW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a#\x15W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x11` R`@\x81 \x81\x90a\"\xB0\x90a)la\"\xAB\x86a)wV[a)\xAAV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa#\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\rVW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x11` R`@\x81 \x81\x90a#M\x90a)\xDBa\"\xAB\x86a)wV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa#\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x0BT`\xFF\x16\x15a\"AW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\xFF\x83\x14a#\xF4Wa#\xED\x83a)\xE6V[\x90Pa\x0BHV[\x81\x80Ta$\0\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$,\x90a8\x0EV[\x80\x15a$wW\x80`\x1F\x10a$NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0BHV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a$\xDBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a%\x05WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x1B'`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a%\xE5WP_\x91P`\x03\x90P\x82a&jV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&6W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&aWP_\x92P`\x01\x91P\x82\x90Pa&jV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a&\x87Wa&\x87a:AV[\x03a&\x90WPPV[`\x01\x82`\x03\x81\x11\x15a&\xA4Wa&\xA4a:AV[\x03a&\xC2W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a&\xD6Wa&\xD6a:AV[\x03a&\xF7W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[`\x03\x82`\x03\x81\x11\x15a'\x0BWa'\x0Ba:AV[\x03a\x0C\x0FW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[``a'7\x82a\x17\xB5V[P_a'M`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[\x90P_\x81Q\x11a'kW`@Q\x80` \x01`@R\x80_\x81RPa\x1B\x8AV[\x80a'u\x84a*#V[`@Q` \x01a'\x86\x92\x91\x90a9\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'\xC5W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_a'\xD1\x83\x83_a\x17\xFAV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\rVW`@Qc9\xE3V7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14\x80a\x0BHWPa\x0BH\x82a*\xB2V[_a(*a#\xB6V[a\x18\x06\x84\x84\x84a+\x01V[`\x01`\x01`\xA0\x1B\x03\x83\x16a(WWa(T`\x12a)\xDBa\"\xAB\x84a)wV[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a(yWa(v`\x12a)la\"\xAB\x84a)wV[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x10` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\rV\x92\x91\x82\x16\x91\x16\x83a\"MV[_\x80`\x80\x83\x90\x1C\x15a(\xBEW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a(\xD0W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a(\xE2W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a(\xF4W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a)\x06W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a)\x18W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a)*W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0BHW`\x01\x01\x92\x91PPV[_\x81\x83\x10a)KW\x81a\x1B\x8AV[P\x90\x91\x90PV[_a)``\x02\x84\x84\x18a:UV[a\x1B\x8A\x90\x84\x84\x16a9\x95V[_a\x1B\x8A\x82\x84a:tV[_`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\x19FW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[_\x80a)\xCEBa)\xC6a)\xBC\x88a\x1BYV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a+\xCCV[\x91P\x91P[\x93P\x93\x91PPV[_a\x1B\x8A\x82\x84a:\x93V[``_a)\xF2\x83a+\xD9V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[``_a*/\x83a,\0V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a*MWa*Ma4\xAFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*wW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a*\x81WP\x93\x92PPPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a*\xE2WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x0BHWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x0BHV[__a+\x0E\x85\x85\x85a,\xD7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a+jWa+e\x84`\x08\x80T_\x83\x81R`\t` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x01UV[a+\x8DV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x8DWa+\x8D\x81\x85a-\xC9V[`\x01`\x01`\xA0\x1B\x03\x85\x16a+\xA9Wa+\xA4\x84a.VV[a\x18\x06V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x06Wa\x18\x06\x85\x85a.\xFDV[_\x80a)\xCE\x85\x85\x85a/KV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0BHW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a,>Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a,jWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a,\x88Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a,\xA0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a,\xB4Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a,\xC6W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0BHW`\x01\x01\x92\x91PPV[_\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a-\x03Wa-\x03\x81\x84\x86a0\xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a-=Wa-\x1E_\x85__a\x1F\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a-kW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[_\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[_a-\xD3\x83a\x0FlV[_\x83\x81R`\x07` R`@\x90 T\x90\x91P\x80\x82\x14a.$W`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x07\x90\x91R\x90 \x81\x90U[P_\x91\x82R`\x07` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x06\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\x08T_\x90a.g\x90`\x01\x90a9\x82V[_\x83\x81R`\t` R`@\x81 T`\x08\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a.\x8EWa.\x8Ea8FV[\x90_R` _ \x01T\x90P\x80`\x08\x83\x81T\x81\x10a.\xADWa.\xADa8FV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\t\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\x08\x80T\x80a.\xE4Wa.\xE4a:\xB2V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPV[_`\x01a/\t\x84a\x0FlV[a/\x13\x91\x90a9\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x07\x90R\x91\x90\x91 \x91\x90\x91UPV[\x82T_\x90\x81\x90\x80\x15a0gW_a/g\x87a\x19\xCA`\x01\x85a9\x82V[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a/\xBBW`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a0\x07W\x84a/\xDE\x88a\x19\xCA`\x01\x86a9\x82V[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua0WV[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa)\xD3\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a)\xD3V[a0\xCC\x83\x83\x83a1%V[a\rVW`\x01`\x01`\xA0\x1B\x03\x83\x16a0\xFAW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\xB5V[_`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x18\x06WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a1^WPa1^\x84\x84a\x16\x8DV[\x80a\x18\x06WPP_\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[P\x80Ta1\x92\x90a8\x0EV[_\x82U\x80`\x1F\x10a1\xA1WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x0E\x02\x91\x90[\x80\x82\x11\x15a\x19FW_\x81U`\x01\x01a1\xB9V[__\x83`\x1F\x84\x01\x12a1\xDCW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xF2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\tW__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a2#W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a28W__\xFD[a2D\x87\x82\x88\x01a1\xCCV[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2bW__\xFD[a2n\x87\x82\x88\x01a1\xCCV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\x02W__\xFD[_` \x82\x84\x03\x12\x15a2\x9FW__\xFD[\x815a\x1B\x8A\x81a2zV[_[\x83\x81\x10\x15a2\xC4W\x81\x81\x01Q\x83\x82\x01R` \x01a2\xACV[PP_\x91\x01RV[_\x81Q\x80\x84Ra2\xE3\x81` \x86\x01` \x86\x01a2\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x1B\x8A` \x83\x01\x84a2\xCCV[_` \x82\x84\x03\x12\x15a3\x19W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a36W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a3LW__\xFD[a3U\x83a3 V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a3uW__\xFD[a3~\x84a3 V[\x92Pa3\x8C` \x85\x01a3 V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a3\xAEW__\xFD[\x825\x91Pa3\xBE` \x84\x01a3 V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a3\xD7W__\xFD[a\x1B\x8A\x82a3 V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a3\xFE`\xE0\x83\x01\x89a2\xCCV[\x82\x81\x03`@\x84\x01Ra4\x10\x81\x89a2\xCCV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a4eW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4GV[P\x90\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a4\x87W__\xFD[a4\x90\x83a3 V[\x91P` \x83\x015\x80\x15\x15\x81\x14a4\xA4W__\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE5Wa4\xE5a4\xAFV[`@R\x90V[__`\x01`\x01`@\x1B\x03\x84\x11\x15a5\x04Wa5\x04a4\xAFV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a52Wa52a4\xAFV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a5IW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a5oW__\xFD[a\x1B\x8A\x83\x835` \x85\x01a4\xEBV[____`\x80\x85\x87\x03\x12\x15a5\x91W__\xFD[a5\x9A\x85a3 V[\x93Pa5\xA8` \x86\x01a3 V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xC9W__\xFD[a5\xD5\x87\x82\x88\x01a5`V[\x91PP\x92\x95\x91\x94P\x92PV[______`\xC0\x87\x89\x03\x12\x15a5\xF6W__\xFD[a5\xFF\x87a3 V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a6\"W__\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0E\x02W__\xFD[_` \x82\x84\x03\x12\x15a6`W__\xFD[\x815a\x1B\x8A\x81a6<V[`\x01`\x01`@\x1B\x03\x84\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90a6\x9D\x90\x83\x01\x84a2\xCCV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a6\xB6W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xCBW__\xFD[a\x18\x06\x84\x82\x85\x01a5`V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x18\x06`\x80\x84\x01\x82a2\xCCV[__`@\x83\x85\x03\x12\x15a7'W__\xFD[a70\x83a3 V[\x91Pa3\xBE` \x84\x01a3 V[_` \x82\x84\x03\x12\x15a7NW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a7cW__\xFD[\x82\x01``\x81\x85\x03\x12\x15a7tW__\xFD[a7|a4\xC3V[a7\x85\x82a3 V[\x81R` \x82\x015a7\x95\x81a6<V[` \x82\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xB2W__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12a7\xC6W__\xFD[a7\xD5\x85\x835` \x85\x01a4\xEBV[`@\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a8\x07Wa8\x07a7\xE2V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a8\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a8@WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a8{Wa8{a7\xE2V[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a\rVW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a8\xA9WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1DFW_\x81U`\x01\x01a8\xB5V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE1Wa8\xE1a4\xAFV[a8\xF5\x81a8\xEF\x84Ta8\x0EV[\x84a8\x84V[` `\x1F\x82\x11`\x01\x81\x14a9'W_\x83\x15a9\x10WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1DFV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a9VW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a96V[P\x84\x82\x10\x15a9sW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[\x80\x82\x01\x80\x82\x11\x15a\x0BHWa\x0BHa7\xE2V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a9\xDA\x90\x83\x01\x84a2\xCCV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a9\xF4W__\xFD[\x81Qa\x1B\x8A\x81a2zV[_\x83Qa:\x10\x81\x84` \x88\x01a2\xAAV[\x83Q\x90\x83\x01\x90a:$\x81\x83` \x88\x01a2\xAAV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x82a:oWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xD9u\x0F\\\xC7\xBBj\x10\x147_c)\xD7\x01\xEDl^t\x80S\t^\x90\x04\xDA\xC5]\xAC\xBE\x93cdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102a4575f3560e01c80636352211e1161016f578063a217fddf116100d8578063ce28961211610092578063e328ed771161006d578063e328ed77146108ae578063e63ab1e9146108da578063e985e9c51461090d578063f7ee944c1461092c575f5ffd5b8063ce2896121461084e578063d547741f1461087c578063e31e07881461089b575f5ffd5b8063a217fddf146107a0578063a22cb465146107b3578063b88d4fde146107d2578063c3cda520146107f1578063c4d66de814610810578063c87b56dd1461082f575f5ffd5b80638e539e8c116101295780638e539e8c146106d6578063913b1fbf146106f557806391d148541461072c57806391ddadf41461074b57806395d89b411461076d5780639ab24eb014610781575f5ffd5b80636352211e1461060a57806370a08231146106295780637ecebe00146106485780638456cb591461067c57806384b0196e146106905780638d69e95e146106b7575f5ffd5b80633383abfe11610211578063476343ee116101cb578063476343ee146105345780634bf5d7e9146105485780634f6ccce71461057e578063587cde1e1461059d5780635c19a95c146105d45780635c975abb146105f3575f5ffd5b80633383abfe1461047057806336568abe146104a45780633a46b1a8146104c35780633f4ba83a146104e257806342842e0e146104f657806342966c6814610515575f5ffd5b806318160ddd1161026257806318160ddd146103945780631bc6ae8a146103b257806323b872dd146103e5578063248a9ca3146104045780632f2ff15d146104325780632f745c5914610451575f5ffd5b806273e1d7146102a857806301ffc9a7146102c957806306fdde03146102fd578063081812fc1461031e578063095ea7b314610355578063158ef93e14610374575b5f5ffd5b3480156102b3575f5ffd5b506102c76102c2366004613210565b61094b565b005b3480156102d4575f5ffd5b506102e86102e336600461328f565b610b3e565b60405190151581526020015b60405180910390f35b348015610308575f5ffd5b50610311610b4e565b6040516102f491906132f7565b348015610329575f5ffd5b5061033d610338366004613309565b610bdd565b6040516001600160a01b0390911681526020016102f4565b348015610360575f5ffd5b506102c761036f36600461333b565b610c04565b34801561037f575f5ffd5b506017546102e890600160a01b900460ff1681565b34801561039f575f5ffd5b506008545b6040519081526020016102f4565b3480156103bd575f5ffd5b506103a47fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b81565b3480156103f0575f5ffd5b506102c76103ff366004613363565b610c13565b34801561040f575f5ffd5b506103a461041e366004613309565b5f908152600c602052604090206001015490565b34801561043d575f5ffd5b506102c761044c36600461339d565b610c9c565b34801561045c575f5ffd5b506103a461046b36600461333b565b610cc0565b34801561047b575f5ffd5b506103a461048a3660046133c7565b6001600160a01b03165f9081526014602052604090205490565b3480156104af575f5ffd5b506102c76104be36600461339d565b610d23565b3480156104ce575f5ffd5b506103a46104dd36600461333b565b610d5b565b3480156104ed575f5ffd5b506102c7610dd0565b348015610501575f5ffd5b506102c7610510366004613363565b610e05565b348015610520575f5ffd5b506102c761052f366004613309565b610e1f565b34801561053f575f5ffd5b506102c7610e2a565b348015610553575f5ffd5b5060408051808201909152600e81526d06d6f64653d74696d657374616d760941b6020820152610311565b348015610589575f5ffd5b506103a4610598366004613309565b610f02565b3480156105a8575f5ffd5b5061033d6105b73660046133c7565b6001600160a01b039081165f908152601060205260409020541690565b3480156105df575f5ffd5b506102c76105ee3660046133c7565b610f57565b3480156105fe575f5ffd5b50600b5460ff166102e8565b348015610615575f5ffd5b5061033d610624366004613309565b610f62565b348015610634575f5ffd5b506103a46106433660046133c7565b610f6c565b348015610653575f5ffd5b506103a46106623660046133c7565b6001600160a01b03165f908152600f602052604090205490565b348015610687575f5ffd5b506102c7610fb1565b34801561069b575f5ffd5b506106a4610fe3565b6040516102f497969594939291906133e0565b3480156106c2575f5ffd5b5060175461033d906001600160a01b031681565b3480156106e1575f5ffd5b506103a46106f0366004613309565b611025565b348015610700575f5ffd5b5061071461070f36600461333b565b611084565b6040516001600160401b0390911681526020016102f4565b348015610737575f5ffd5b506102e861074636600461339d565b6110cb565b348015610756575f5ffd5b5060405165ffffffffffff421681526020016102f4565b348015610778575f5ffd5b506103116110f5565b34801561078c575f5ffd5b506103a461079b3660046133c7565b611104565b3480156107ab575f5ffd5b506103a45f81565b3480156107be575f5ffd5b506102c76107cd366004613476565b611133565b3480156107dd575f5ffd5b506102c76107ec36600461357e565b61113e565b3480156107fc575f5ffd5b506102c761080b3660046135e1565b611155565b34801561081b575f5ffd5b506102c761082a3660046133c7565b611211565b34801561083a575f5ffd5b50610311610849366004613309565b611314565b348015610859575f5ffd5b5061086d610868366004613650565b61131f565b6040516102f49392919061366b565b348015610887575f5ffd5b506102c761089636600461339d565b6113dd565b6102c76108a93660046136a6565b611401565b3480156108b9575f5ffd5b506108cd6108c8366004613650565b611594565b6040516102f491906136d7565b3480156108e5575f5ffd5b506103a47f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a81565b348015610918575f5ffd5b506102e8610927366004613716565b61168d565b348015610937575f5ffd5b5061071461094636600461333b565b6116ba565b6109757fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b336110cb565b6109be5760405162461bcd60e51b815260206004820152601560248201527427b7363c9039b2b93b34b1b290383937bb34b232b960591b60448201526064015b60405180910390fd5b5f6109cb8486018661373e565b6020808201516001600160401b0381165f908152601390925260409091205491925090600160401b90046001600160a01b0316610a435760405162461bcd60e51b8152602060048201526016602482015275151c9a59d9d95c88191bd95cc81b9bdd08195e1a5cdd60521b60448201526064016109b5565b5f82604001515111610a865760405162461bcd60e51b815260206004820152600c60248201526b55524920697320656d70747960a01b60448201526064016109b5565b6001600160401b0381165f90815260136020526040812080546001600160e01b031916815590610ab96001830182613186565b5050601680545f9182610acb836137f6565b919050559050610ade835f01518261176e565b610aec818460400151611787565b80835f01516001600160a01b03167fd35bb95e09c04b219e35047ce7b7b300e3384264ef84a40456943dbc0fc17c148560400151604051610b2d91906132f7565b60405180910390a350505050505050565b5f610b4882611791565b92915050565b60605f8054610b5c9061380e565b80601f0160208091040260200160405190810160405280929190818152602001828054610b889061380e565b8015610bd35780601f10610baa57610100808354040283529160200191610bd3565b820191905f5260205f20905b815481529060010190602001808311610bb657829003601f168201915b5050505050905090565b5f610be7826117b5565b505f828152600460205260409020546001600160a01b0316610b48565b610c0f8282336117ed565b5050565b6001600160a01b038216610c3c57604051633250574960e11b81525f60048201526024016109b5565b5f610c488383336117fa565b9050836001600160a01b0316816001600160a01b031614610c96576040516364283d7b60e01b81526001600160a01b03808616600483015260248201849052821660448201526064016109b5565b50505050565b5f828152600c6020526040902060010154610cb68161180e565b610c968383611818565b5f610cca83610f6c565b8210610cfb5760405163295f44f760e21b81526001600160a01b0384166004820152602481018390526044016109b5565b506001600160a01b03919091165f908152600660209081526040808320938352929052205490565b6001600160a01b0381163314610d4c5760405163334bd91960e11b815260040160405180910390fd5b610d5682826118a9565b505050565b5f4265ffffffffffff81168310610d9657604051637669fc0f60e11b81526004810184905265ffffffffffff821660248201526044016109b5565b610dbf610da284611914565b6001600160a01b0386165f9081526011602052604090209061194a565b6001600160d01b0316949350505050565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610dfa8161180e565b610e026119fa565b50565b610d5683838360405180602001604052805f81525061113e565b610c0f5f82336117fa565b5f610e348161180e565b4780610e7b5760405162461bcd60e51b81526020600482015260166024820152754e6f2062616c616e636520746f20776974686472617760501b60448201526064016109b5565b6040515f90339083908381818185875af1925050503d805f8114610eba576040519150601f19603f3d011682016040523d82523d5f602084013e610ebf565b606091505b5050905080610d565760405162461bcd60e51b815260206004820152600f60248201526e151c985b9cd9995c8819985a5b1959608a1b60448201526064016109b5565b5f610f0c60085490565b8210610f345760405163295f44f760e21b81525f6004820152602481018390526044016109b5565b60088281548110610f4757610f47613846565b905f5260205f2001549050919050565b33610c0f8183611a4c565b5f610b48826117b5565b5f6001600160a01b038216610f96576040516322718ad960e21b81525f60048201526024016109b5565b506001600160a01b03165f9081526003602052604090205490565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610fdb8161180e565b610e02611abd565b5f6060805f5f5f6060610ff4611afa565b610ffc611b2c565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f4265ffffffffffff8116831061106057604051637669fc0f60e11b81526004810184905265ffffffffffff821660248201526044016109b5565b61107461106c84611914565b60129061194a565b6001600160d01b03169392505050565b6014602052815f5260405f20818154811061109d575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f918252600c602090815260408084206001600160a01b0393909316845291905290205460ff1690565b606060018054610b5c9061380e565b6001600160a01b0381165f90815260116020526040812061112490611b59565b6001600160d01b031692915050565b610c0f338383611b91565b611149848484610c13565b610c9684848484611c27565b8342111561117957604051632341d78760e11b8152600481018590526024016109b5565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f906111f2906111ea9060a00160405160208183030381529060405280519060200120611d4d565b858585611d79565b90506111fe8187611da5565b6112088188611a4c565b50505050505050565b5f61121b8161180e565b601754600160a01b900460ff161561126b5760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b60448201526064016109b5565b6001600160a01b0382166112c15760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420736572766963652070726f7669646572000000000000000060448201526064016109b5565b6112eb7fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b83611818565b5050601780546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b6060610b4882611df7565b60136020525f9081526040902080546001820180546001600160401b03831693600160401b9093046001600160a01b031692919061135c9061380e565b80601f01602080910402602001604051908101604052809291908181526020018280546113889061380e565b80156113d35780601f106113aa576101008083540402835291602001916113d3565b820191905f5260205f20905b8154815290600101906020018083116113b657829003601f168201915b5050505050905083565b5f828152600c60205260409020600101546113f78161180e565b610c9683836118a9565b3467016345785d8a0000146114585760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e31204554480060448201526064016109b5565b601580545f916001600160401b0390911690826114748361385a565b82546001600160401b039182166101009390930a928302928202191691909117909155604080516060810182528383168082523360208084019182528385018981525f9384526013909152939091208251815492516001600160a01b0316600160401b026001600160e01b0319909316951694909417178355905192935091829190600182019061150590826138c8565b5050335f818152601460209081526040808320805460018101825590845291909220600482040180546001600160401b0380891660086003909516949094026101000a8481029102199091161790559051919250907ff3f411d853486b9f53da63009a21cd284ea18a800d4de55ce5bd935d197e4cf1906115879087906132f7565b60405180910390a3505050565b60408051606080820183525f8083526020830152918101919091526001600160401b038281165f90815260136020908152604091829020825160608101845281549485168152600160401b9094046001600160a01b03169184019190915260018101805491928401916116069061380e565b80601f01602080910402602001604051908101604052809291908181526020018280546116329061380e565b801561167d5780601f106116545761010080835404028352916020019161167d565b820191905f5260205f20905b81548152906001019060200180831161166057829003601f168201915b5050505050815250509050919050565b6001600160a01b039182165f90815260056020908152604080832093909416825291909152205460ff1690565b6001600160a01b0382165f9081526014602052604081205482106117165760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b60448201526064016109b5565b6001600160a01b0383165f90815260146020526040902080548390811061173f5761173f613846565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b0316905092915050565b610c0f828260405180602001604052805f815250611efa565b610c0f8282611f10565b5f6001600160e01b03198216637965db0b60e01b1480610b485750610b4882611f5f565b5f818152600260205260408120546001600160a01b031680610b4857604051637e27328960e01b8152600481018490526024016109b5565b610d568383836001611f83565b5f611806848484612087565b949350505050565b610e0281336120a2565b5f61182383836110cb565b6118a2575f838152600c602090815260408083206001600160a01b03861684529091529020805460ff1916600117905561185a3390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4506001610b48565b505f610b48565b5f6118b483836110cb565b156118a2575f838152600c602090815260408083206001600160a01b0386168085529252808320805460ff1916905551339286917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a4506001610b48565b5f65ffffffffffff821115611946576040516306dfcc6560e41b815260306004820152602481018390526044016109b5565b5090565b81545f90818160058111156119a6575f611963846120db565b61196d9085613982565b5f8881526020902090915081015465ffffffffffff9081169087161015611996578091506119a4565b6119a1816001613995565b92505b505b5f6119b3878785856121bf565b905080156119ed576119d7876119ca600184613982565b5f91825260209091200190565b54600160301b90046001600160d01b03166119ef565b5f5b979650505050505050565b611a0261221e565b600b805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038281165f8181526010602052604080822080548686166001600160a01b0319821681179092559151919094169392849290917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4610d568183611ab886612243565b61224d565b611ac56123b6565b600b805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258611a2f3390565b6060611b277f0000000000000000000000000000000000000000000000000000000000000000600d6123da565b905090565b6060611b277f0000000000000000000000000000000000000000000000000000000000000000600e6123da565b80545f908015611b8857611b72836119ca600184613982565b54600160301b90046001600160d01b0316611b8a565b5f5b9392505050565b6001600160a01b038216611bc357604051630b61174360e31b81526001600160a01b03831660048201526024016109b5565b6001600160a01b038381165f81815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c319101611587565b6001600160a01b0383163b15610c9657604051630a85bd0160e11b81526001600160a01b0384169063150b7a0290611c699033908890879087906004016139a8565b6020604051808303815f875af1925050508015611ca3575060408051601f3d908101601f19168201909252611ca0918101906139e4565b60015b611d0a573d808015611cd0576040519150601f19603f3d011682016040523d82523d5f602084013e611cd5565b606091505b5080515f03611d0257604051633250574960e11b81526001600160a01b03851660048201526024016109b5565b805181602001fd5b6001600160e01b03198116630a85bd0160e11b14611d4657604051633250574960e11b81526001600160a01b03851660048201526024016109b5565b5050505050565b5f610b48611d59612483565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f5f611d89888888886125ac565b925092509250611d998282612674565b50909695505050505050565b6001600160a01b0382165f908152600f60205260409020805460018101909155818114610d56576040516301d4b62360e61b81526001600160a01b0384166004820152602481018290526044016109b5565b6060611e02826117b5565b505f828152600a602052604081208054611e1b9061380e565b80601f0160208091040260200160405190810160405280929190818152602001828054611e479061380e565b8015611e925780601f10611e6957610100808354040283529160200191611e92565b820191905f5260205f20905b815481529060010190602001808311611e7557829003601f168201915b505050505090505f611eae60408051602081019091525f815290565b905080515f03611ebf575092915050565b815115611ef1578082604051602001611ed99291906139ff565b60405160208183030381529060405292505050919050565b6118068461272c565b611f04838361279c565b610d565f848484611c27565b5f828152600a60205260409020611f2782826138c8565b506040518281527ff8e1a15aba9398e019f0b49df1a4fde98ee17ae345cb5f6b5e2c27f5033e8ce79060200160405180910390a15050565b5f6001600160e01b03198216632483248360e11b1480610b485750610b48826127fd565b8080611f9757506001600160a01b03821615155b15612058575f611fa6846117b5565b90506001600160a01b03831615801590611fd25750826001600160a01b0316816001600160a01b031614155b8015611fe55750611fe3818461168d565b155b1561200e5760405163a9fbf51f60e01b81526001600160a01b03841660048201526024016109b5565b81156120565783856001600160a01b0316826001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45b505b50505f90815260046020526040902080546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f612094858585612821565b905061180681866001612835565b6120ac82826110cb565b610c0f5760405163e2517d3f60e01b81526001600160a01b0382166004820152602481018390526044016109b5565b5f815f036120ea57505f919050565b5f60016120f6846128aa565b901c6001901b9050600181848161210f5761210f613a2d565b048201901c9050600181848161212757612127613a2d565b048201901c9050600181848161213f5761213f613a2d565b048201901c9050600181848161215757612157613a2d565b048201901c9050600181848161216f5761216f613a2d565b048201901c9050600181848161218757612187613a2d565b048201901c9050600181848161219f5761219f613a2d565b048201901c9050611b8a818285816121b9576121b9613a2d565b0461293d565b5f5b81831015612216575f6121d48484612952565b5f8781526020902090915065ffffffffffff86169082015465ffffffffffff16111561220257809250612210565b61220d816001613995565b93505b506121c1565b509392505050565b600b5460ff1661224157604051638dfc202b60e01b815260040160405180910390fd5b565b5f610b4882610f6c565b816001600160a01b0316836001600160a01b03161415801561226e57505f81115b15610d56576001600160a01b03831615612315576001600160a01b0383165f90815260116020526040812081906122b09061296c6122ab86612977565b6129aa565b6001600160d01b031691506001600160d01b03169150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161230a929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615610d56576001600160a01b0382165f908152601160205260408120819061234d906129db6122ab86612977565b6001600160d01b031691506001600160d01b03169150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516123a7929190918252602082015260400190565b60405180910390a25050505050565b600b5460ff16156122415760405163d93c066560e01b815260040160405180910390fd5b606060ff83146123f4576123ed836129e6565b9050610b48565b8180546124009061380e565b80601f016020809104026020016040519081016040528092919081815260200182805461242c9061380e565b80156124775780601f1061244e57610100808354040283529160200191612477565b820191905f5260205f20905b81548152906001019060200180831161245a57829003601f168201915b50505050509050610b48565b5f306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480156124db57507f000000000000000000000000000000000000000000000000000000000000000046145b1561250557507f000000000000000000000000000000000000000000000000000000000000000090565b611b27604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201527f0000000000000000000000000000000000000000000000000000000000000000918101919091527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411156125e557505f9150600390508261266a565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015612636573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b03811661266157505f92506001915082905061266a565b92505f91508190505b9450945094915050565b5f82600381111561268757612687613a41565b03612690575050565b60018260038111156126a4576126a4613a41565b036126c25760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156126d6576126d6613a41565b036126f75760405163fce698f760e01b8152600481018290526024016109b5565b600382600381111561270b5761270b613a41565b03610c0f576040516335e2f38360e21b8152600481018290526024016109b5565b6060612737826117b5565b505f61274d60408051602081019091525f815290565b90505f81511161276b5760405180602001604052805f815250611b8a565b8061277584612a23565b6040516020016127869291906139ff565b6040516020818303038152906040529392505050565b6001600160a01b0382166127c557604051633250574960e11b81525f60048201526024016109b5565b5f6127d183835f6117fa565b90506001600160a01b03811615610d56576040516339e3563760e11b81525f60048201526024016109b5565b5f6001600160e01b0319821663780e9d6360e01b1480610b485750610b4882612ab2565b5f61282a6123b6565b611806848484612b01565b6001600160a01b0383166128575761285460126129db6122ab84612977565b50505b6001600160a01b03821661287957612876601261296c6122ab84612977565b50505b6001600160a01b038381165f90815260106020526040808220548584168352912054610d569291821691168361224d565b5f80608083901c156128be57608092831c92015b604083901c156128d057604092831c92015b602083901c156128e257602092831c92015b601083901c156128f457601092831c92015b600883901c1561290657600892831c92015b600483901c1561291857600492831c92015b600283901c1561292a57600292831c92015b600183901c15610b485760010192915050565b5f81831061294b5781611b8a565b5090919050565b5f6129606002848418613a55565b611b8a90848416613995565b5f611b8a8284613a74565b5f6001600160d01b03821115611946576040516306dfcc6560e41b815260d06004820152602481018390526044016109b5565b5f806129ce426129c66129bc88611b59565b868863ffffffff16565b879190612bcc565b915091505b935093915050565b5f611b8a8284613a93565b60605f6129f283612bd9565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b60605f612a2f83612c00565b60010190505f816001600160401b03811115612a4d57612a4d6134af565b6040519080825280601f01601f191660200182016040528015612a77576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a8504945084612a8157509392505050565b5f6001600160e01b031982166380ac58cd60e01b1480612ae257506001600160e01b03198216635b5e139f60e01b145b80610b4857506301ffc9a760e01b6001600160e01b0319831614610b48565b5f5f612b0e858585612cd7565b90506001600160a01b038116612b6a57612b6584600880545f838152600960205260408120829055600182018355919091527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee30155565b612b8d565b846001600160a01b0316816001600160a01b031614612b8d57612b8d8185612dc9565b6001600160a01b038516612ba957612ba484612e56565b611806565b846001600160a01b0316816001600160a01b031614611806576118068585612efd565b5f806129ce858585612f4b565b5f60ff8216601f811115610b4857604051632cd44ac360e21b815260040160405180910390fd5b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310612c3e5772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310612c6a576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310612c8857662386f26fc10000830492506010015b6305f5e1008310612ca0576305f5e100830492506008015b6127108310612cb457612710830492506004015b60648310612cc6576064830492506002015b600a8310610b485760010192915050565b5f828152600260205260408120546001600160a01b0390811690831615612d0357612d038184866130c1565b6001600160a01b03811615612d3d57612d1e5f855f5f611f83565b6001600160a01b0381165f90815260036020526040902080545f190190555b6001600160a01b03851615612d6b576001600160a01b0385165f908152600360205260409020805460010190555b5f8481526002602052604080822080546001600160a01b0319166001600160a01b0389811691821790925591518793918516917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4949350505050565b5f612dd383610f6c565b5f83815260076020526040902054909150808214612e24576001600160a01b0384165f9081526006602090815260408083208584528252808320548484528184208190558352600790915290208190555b505f9182526007602090815260408084208490556001600160a01b039094168352600681528383209183525290812055565b6008545f90612e6790600190613982565b5f8381526009602052604081205460088054939450909284908110612e8e57612e8e613846565b905f5260205f20015490508060088381548110612ead57612ead613846565b5f918252602080832090910192909255828152600990915260408082208490558582528120556008805480612ee457612ee4613ab2565b600190038181905f5260205f20015f9055905550505050565b5f6001612f0984610f6c565b612f139190613982565b6001600160a01b039093165f908152600660209081526040808320868452825280832085905593825260079052919091209190915550565b82545f9081908015613067575f612f67876119ca600185613982565b60408051808201909152905465ffffffffffff808216808452600160301b9092046001600160d01b031660208401529192509087161015612fbb57604051632520601d60e01b815260040160405180910390fd5b805165ffffffffffff8088169116036130075784612fde886119ca600186613982565b80546001600160d01b0392909216600160301b0265ffffffffffff909216919091179055613057565b6040805180820190915265ffffffffffff80881682526001600160d01b0380881660208085019182528b54600181018d555f8d81529190912094519151909216600160301b029216919091179101555b6020015192508391506129d39050565b50506040805180820190915265ffffffffffff80851682526001600160d01b0380851660208085019182528854600181018a555f8a815291822095519251909316600160301b0291909316179201919091559050816129d3565b6130cc838383613125565b610d56576001600160a01b0383166130fa57604051637e27328960e01b8152600481018290526024016109b5565b60405163177e802f60e01b81526001600160a01b0383166004820152602481018290526044016109b5565b5f6001600160a01b038316158015906118065750826001600160a01b0316846001600160a01b0316148061315e575061315e848461168d565b806118065750505f908152600460205260409020546001600160a01b03908116911614919050565b5080546131929061380e565b5f825580601f106131a1575050565b601f0160209004905f5260205f2090810190610e0291905b80821115611946575f81556001016131b9565b5f5f83601f8401126131dc575f5ffd5b5081356001600160401b038111156131f2575f5ffd5b602083019150836020828501011115613209575f5ffd5b9250929050565b5f5f5f5f60408587031215613223575f5ffd5b84356001600160401b03811115613238575f5ffd5b613244878288016131cc565b90955093505060208501356001600160401b03811115613262575f5ffd5b61326e878288016131cc565b95989497509550505050565b6001600160e01b031981168114610e02575f5ffd5b5f6020828403121561329f575f5ffd5b8135611b8a8161327a565b5f5b838110156132c45781810151838201526020016132ac565b50505f910152565b5f81518084526132e38160208601602086016132aa565b601f01601f19169290920160200192915050565b602081525f611b8a60208301846132cc565b5f60208284031215613319575f5ffd5b5035919050565b80356001600160a01b0381168114613336575f5ffd5b919050565b5f5f6040838503121561334c575f5ffd5b61335583613320565b946020939093013593505050565b5f5f5f60608486031215613375575f5ffd5b61337e84613320565b925061338c60208501613320565b929592945050506040919091013590565b5f5f604083850312156133ae575f5ffd5b823591506133be60208401613320565b90509250929050565b5f602082840312156133d7575f5ffd5b611b8a82613320565b60ff60f81b8816815260e060208201525f6133fe60e08301896132cc565b828103604084015261341081896132cc565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015613465578351835260209384019390920191600101613447565b50909b9a5050505050505050505050565b5f5f60408385031215613487575f5ffd5b61349083613320565b9150602083013580151581146134a4575f5ffd5b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b03811182821017156134e5576134e56134af565b60405290565b5f5f6001600160401b03841115613504576135046134af565b50604051601f19601f85018116603f011681018181106001600160401b0382111715613532576135326134af565b604052838152905080828401851015613549575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f83011261356f575f5ffd5b611b8a838335602085016134eb565b5f5f5f5f60808587031215613591575f5ffd5b61359a85613320565b93506135a860208601613320565b92506040850135915060608501356001600160401b038111156135c9575f5ffd5b6135d587828801613560565b91505092959194509250565b5f5f5f5f5f5f60c087890312156135f6575f5ffd5b6135ff87613320565b95506020870135945060408701359350606087013560ff81168114613622575f5ffd5b9598949750929560808101359460a0909101359350915050565b6001600160401b0381168114610e02575f5ffd5b5f60208284031215613660575f5ffd5b8135611b8a8161363c565b6001600160401b03841681526001600160a01b03831660208201526060604082018190525f9061369d908301846132cc565b95945050505050565b5f602082840312156136b6575f5ffd5b81356001600160401b038111156136cb575f5ffd5b61180684828501613560565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f604083015160608084015261180660808401826132cc565b5f5f60408385031215613727575f5ffd5b61373083613320565b91506133be60208401613320565b5f6020828403121561374e575f5ffd5b81356001600160401b03811115613763575f5ffd5b820160608185031215613774575f5ffd5b61377c6134c3565b61378582613320565b815260208201356137958161363c565b602082015260408201356001600160401b038111156137b2575f5ffd5b80830192505084601f8301126137c6575f5ffd5b6137d5858335602085016134eb565b6040820152949350505050565b634e487b7160e01b5f52601160045260245ffd5b5f60018201613807576138076137e2565b5060010190565b600181811c9082168061382257607f821691505b60208210810361384057634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f6001600160401b0382166001600160401b03810361387b5761387b6137e2565b60010192915050565b601f821115610d5657805f5260205f20601f840160051c810160208510156138a95750805b601f840160051c820191505b81811015611d46575f81556001016138b5565b81516001600160401b038111156138e1576138e16134af565b6138f5816138ef845461380e565b84613884565b6020601f821160018114613927575f83156139105750848201515b5f19600385901b1c1916600184901b178455611d46565b5f84815260208120601f198516915b828110156139565787850151825560209485019460019092019101613936565b508482101561397357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b81810381811115610b4857610b486137e2565b80820180821115610b4857610b486137e2565b6001600160a01b03858116825284166020820152604081018390526080606082018190525f906139da908301846132cc565b9695505050505050565b5f602082840312156139f4575f5ffd5b8151611b8a8161327a565b5f8351613a108184602088016132aa565b835190830190613a248183602088016132aa565b01949350505050565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b5f82613a6f57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160d01b038281168282160390811115610b4857610b486137e2565b6001600160d01b038181168382160190811115610b4857610b486137e2565b634e487b7160e01b5f52603160045260245ffdfea2646970667358221220d9750f5cc7bb6a1014375f6329d701ed6c5e748053095e9004dac55dacbe936364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\xA4W_5`\xE0\x1C\x80ccR!\x1E\x11a\x01oW\x80c\xA2\x17\xFD\xDF\x11a\0\xD8W\x80c\xCE(\x96\x12\x11a\0\x92W\x80c\xE3(\xEDw\x11a\0mW\x80c\xE3(\xEDw\x14a\x08\xAEW\x80c\xE6:\xB1\xE9\x14a\x08\xDAW\x80c\xE9\x85\xE9\xC5\x14a\t\rW\x80c\xF7\xEE\x94L\x14a\t,W__\xFD[\x80c\xCE(\x96\x12\x14a\x08NW\x80c\xD5Gt\x1F\x14a\x08|W\x80c\xE3\x1E\x07\x88\x14a\x08\x9BW__\xFD[\x80c\xA2\x17\xFD\xDF\x14a\x07\xA0W\x80c\xA2,\xB4e\x14a\x07\xB3W\x80c\xB8\x8DO\xDE\x14a\x07\xD2W\x80c\xC3\xCD\xA5 \x14a\x07\xF1W\x80c\xC4\xD6m\xE8\x14a\x08\x10W\x80c\xC8{V\xDD\x14a\x08/W__\xFD[\x80c\x8ES\x9E\x8C\x11a\x01)W\x80c\x8ES\x9E\x8C\x14a\x06\xD6W\x80c\x91;\x1F\xBF\x14a\x06\xF5W\x80c\x91\xD1HT\x14a\x07,W\x80c\x91\xDD\xAD\xF4\x14a\x07KW\x80c\x95\xD8\x9BA\x14a\x07mW\x80c\x9A\xB2N\xB0\x14a\x07\x81W__\xFD[\x80ccR!\x1E\x14a\x06\nW\x80cp\xA0\x821\x14a\x06)W\x80c~\xCE\xBE\0\x14a\x06HW\x80c\x84V\xCBY\x14a\x06|W\x80c\x84\xB0\x19n\x14a\x06\x90W\x80c\x8Di\xE9^\x14a\x06\xB7W__\xFD[\x80c3\x83\xAB\xFE\x11a\x02\x11W\x80cGcC\xEE\x11a\x01\xCBW\x80cGcC\xEE\x14a\x054W\x80cK\xF5\xD7\xE9\x14a\x05HW\x80cOl\xCC\xE7\x14a\x05~W\x80cX|\xDE\x1E\x14a\x05\x9DW\x80c\\\x19\xA9\\\x14a\x05\xD4W\x80c\\\x97Z\xBB\x14a\x05\xF3W__\xFD[\x80c3\x83\xAB\xFE\x14a\x04pW\x80c6V\x8A\xBE\x14a\x04\xA4W\x80c:F\xB1\xA8\x14a\x04\xC3W\x80c?K\xA8:\x14a\x04\xE2W\x80cB\x84.\x0E\x14a\x04\xF6W\x80cB\x96lh\x14a\x05\x15W__\xFD[\x80c\x18\x16\r\xDD\x11a\x02bW\x80c\x18\x16\r\xDD\x14a\x03\x94W\x80c\x1B\xC6\xAE\x8A\x14a\x03\xB2W\x80c#\xB8r\xDD\x14a\x03\xE5W\x80c$\x8A\x9C\xA3\x14a\x04\x04W\x80c//\xF1]\x14a\x042W\x80c/t\\Y\x14a\x04QW__\xFD[\x80bs\xE1\xD7\x14a\x02\xA8W\x80c\x01\xFF\xC9\xA7\x14a\x02\xC9W\x80c\x06\xFD\xDE\x03\x14a\x02\xFDW\x80c\x08\x18\x12\xFC\x14a\x03\x1EW\x80c\t^\xA7\xB3\x14a\x03UW\x80c\x15\x8E\xF9>\x14a\x03tW[__\xFD[4\x80\x15a\x02\xB3W__\xFD[Pa\x02\xC7a\x02\xC26`\x04a2\x10V[a\tKV[\0[4\x80\x15a\x02\xD4W__\xFD[Pa\x02\xE8a\x02\xE36`\x04a2\x8FV[a\x0B>V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x08W__\xFD[Pa\x03\x11a\x0BNV[`@Qa\x02\xF4\x91\x90a2\xF7V[4\x80\x15a\x03)W__\xFD[Pa\x03=a\x0386`\x04a3\tV[a\x0B\xDDV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x03`W__\xFD[Pa\x02\xC7a\x03o6`\x04a3;V[a\x0C\x04V[4\x80\x15a\x03\x7FW__\xFD[P`\x17Ta\x02\xE8\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x03\x9FW__\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x02\xF4V[4\x80\x15a\x03\xBDW__\xFD[Pa\x03\xA4\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x81V[4\x80\x15a\x03\xF0W__\xFD[Pa\x02\xC7a\x03\xFF6`\x04a3cV[a\x0C\x13V[4\x80\x15a\x04\x0FW__\xFD[Pa\x03\xA4a\x04\x1E6`\x04a3\tV[_\x90\x81R`\x0C` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04=W__\xFD[Pa\x02\xC7a\x04L6`\x04a3\x9DV[a\x0C\x9CV[4\x80\x15a\x04\\W__\xFD[Pa\x03\xA4a\x04k6`\x04a3;V[a\x0C\xC0V[4\x80\x15a\x04{W__\xFD[Pa\x03\xA4a\x04\x8A6`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x14` R`@\x90 T\x90V[4\x80\x15a\x04\xAFW__\xFD[Pa\x02\xC7a\x04\xBE6`\x04a3\x9DV[a\r#V[4\x80\x15a\x04\xCEW__\xFD[Pa\x03\xA4a\x04\xDD6`\x04a3;V[a\r[V[4\x80\x15a\x04\xEDW__\xFD[Pa\x02\xC7a\r\xD0V[4\x80\x15a\x05\x01W__\xFD[Pa\x02\xC7a\x05\x106`\x04a3cV[a\x0E\x05V[4\x80\x15a\x05 W__\xFD[Pa\x02\xC7a\x05/6`\x04a3\tV[a\x0E\x1FV[4\x80\x15a\x05?W__\xFD[Pa\x02\xC7a\x0E*V[4\x80\x15a\x05SW__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x03\x11V[4\x80\x15a\x05\x89W__\xFD[Pa\x03\xA4a\x05\x986`\x04a3\tV[a\x0F\x02V[4\x80\x15a\x05\xA8W__\xFD[Pa\x03=a\x05\xB76`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x10` R`@\x90 T\x16\x90V[4\x80\x15a\x05\xDFW__\xFD[Pa\x02\xC7a\x05\xEE6`\x04a3\xC7V[a\x0FWV[4\x80\x15a\x05\xFEW__\xFD[P`\x0BT`\xFF\x16a\x02\xE8V[4\x80\x15a\x06\x15W__\xFD[Pa\x03=a\x06$6`\x04a3\tV[a\x0FbV[4\x80\x15a\x064W__\xFD[Pa\x03\xA4a\x06C6`\x04a3\xC7V[a\x0FlV[4\x80\x15a\x06SW__\xFD[Pa\x03\xA4a\x06b6`\x04a3\xC7V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x0F` R`@\x90 T\x90V[4\x80\x15a\x06\x87W__\xFD[Pa\x02\xC7a\x0F\xB1V[4\x80\x15a\x06\x9BW__\xFD[Pa\x06\xA4a\x0F\xE3V[`@Qa\x02\xF4\x97\x96\x95\x94\x93\x92\x91\x90a3\xE0V[4\x80\x15a\x06\xC2W__\xFD[P`\x17Ta\x03=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xE1W__\xFD[Pa\x03\xA4a\x06\xF06`\x04a3\tV[a\x10%V[4\x80\x15a\x07\0W__\xFD[Pa\x07\x14a\x07\x0F6`\x04a3;V[a\x10\x84V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x077W__\xFD[Pa\x02\xE8a\x07F6`\x04a3\x9DV[a\x10\xCBV[4\x80\x15a\x07VW__\xFD[P`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x02\xF4V[4\x80\x15a\x07xW__\xFD[Pa\x03\x11a\x10\xF5V[4\x80\x15a\x07\x8CW__\xFD[Pa\x03\xA4a\x07\x9B6`\x04a3\xC7V[a\x11\x04V[4\x80\x15a\x07\xABW__\xFD[Pa\x03\xA4_\x81V[4\x80\x15a\x07\xBEW__\xFD[Pa\x02\xC7a\x07\xCD6`\x04a4vV[a\x113V[4\x80\x15a\x07\xDDW__\xFD[Pa\x02\xC7a\x07\xEC6`\x04a5~V[a\x11>V[4\x80\x15a\x07\xFCW__\xFD[Pa\x02\xC7a\x08\x0B6`\x04a5\xE1V[a\x11UV[4\x80\x15a\x08\x1BW__\xFD[Pa\x02\xC7a\x08*6`\x04a3\xC7V[a\x12\x11V[4\x80\x15a\x08:W__\xFD[Pa\x03\x11a\x08I6`\x04a3\tV[a\x13\x14V[4\x80\x15a\x08YW__\xFD[Pa\x08ma\x08h6`\x04a6PV[a\x13\x1FV[`@Qa\x02\xF4\x93\x92\x91\x90a6kV[4\x80\x15a\x08\x87W__\xFD[Pa\x02\xC7a\x08\x966`\x04a3\x9DV[a\x13\xDDV[a\x02\xC7a\x08\xA96`\x04a6\xA6V[a\x14\x01V[4\x80\x15a\x08\xB9W__\xFD[Pa\x08\xCDa\x08\xC86`\x04a6PV[a\x15\x94V[`@Qa\x02\xF4\x91\x90a6\xD7V[4\x80\x15a\x08\xE5W__\xFD[Pa\x03\xA4\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[4\x80\x15a\t\x18W__\xFD[Pa\x02\xE8a\t'6`\x04a7\x16V[a\x16\x8DV[4\x80\x15a\t7W__\xFD[Pa\x07\x14a\tF6`\x04a3;V[a\x16\xBAV[a\tu\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K3a\x10\xCBV[a\t\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt'\xB76<\x909\xB2\xB9;4\xB1\xB2\x90897\xBB4\xB22\xB9`Y\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\t\xCB\x84\x86\x01\x86a7>V[` \x80\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R`\x13\x90\x92R`@\x90\x91 T\x91\x92P\x90`\x01`@\x1B\x90\x04`\x01`\x01`\xA0\x1B\x03\x16a\nCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15\x1C\x9AY\xD9\xD9\\\x88\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`R\x1B`D\x82\x01R`d\x01a\t\xB5V[_\x82`@\x01QQ\x11a\n\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkURI is empty`\xA0\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R`\x13` R`@\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\n\xB9`\x01\x83\x01\x82a1\x86V[PP`\x16\x80T_\x91\x82a\n\xCB\x83a7\xF6V[\x91\x90PU\x90Pa\n\xDE\x83_\x01Q\x82a\x17nV[a\n\xEC\x81\x84`@\x01Qa\x17\x87V[\x80\x83_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD3[\xB9^\t\xC0K!\x9E5\x04|\xE7\xB7\xB3\0\xE38Bd\xEF\x84\xA4\x04V\x94=\xBC\x0F\xC1|\x14\x85`@\x01Q`@Qa\x0B-\x91\x90a2\xF7V[`@Q\x80\x91\x03\x90\xA3PPPPPPPV[_a\x0BH\x82a\x17\x91V[\x92\x91PPV[``_\x80Ta\x0B\\\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\x88\x90a8\x0EV[\x80\x15a\x0B\xD3W\x80`\x1F\x10a\x0B\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x0B\xE7\x82a\x17\xB5V[P_\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x0BHV[a\x0C\x0F\x82\x823a\x17\xEDV[PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0C<W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_a\x0CH\x83\x833a\x17\xFAV[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\x96W`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\t\xB5V[PPPPV[_\x82\x81R`\x0C` R`@\x90 `\x01\x01Ta\x0C\xB6\x81a\x18\x0EV[a\x0C\x96\x83\x83a\x18\x18V[_a\x0C\xCA\x83a\x0FlV[\x82\x10a\x0C\xFBW`@Qc)_D\xF7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\rLW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rV\x82\x82a\x18\xA9V[PPPV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\r\x96W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\xB5V[a\r\xBFa\r\xA2\x84a\x19\x14V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x11` R`@\x90 \x90a\x19JV[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\r\xFA\x81a\x18\x0EV[a\x0E\x02a\x19\xFAV[PV[a\rV\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x11>V[a\x0C\x0F_\x823a\x17\xFAV[_a\x0E4\x81a\x18\x0EV[G\x80a\x0E{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuNo balance to withdraw`P\x1B`D\x82\x01R`d\x01a\t\xB5V[`@Q_\x903\x90\x83\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0E\xBAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0E\xBFV[``\x91P[PP\x90P\x80a\rVW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R`d\x01a\t\xB5V[_a\x0F\x0C`\x08T\x90V[\x82\x10a\x0F4W`@Qc)_D\xF7`\xE2\x1B\x81R_`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[`\x08\x82\x81T\x81\x10a\x0FGWa\x0FGa8FV[\x90_R` _ \x01T\x90P\x91\x90PV[3a\x0C\x0F\x81\x83a\x1ALV[_a\x0BH\x82a\x17\xB5V[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0F\x96W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x0F\xDB\x81a\x18\x0EV[a\x0E\x02a\x1A\xBDV[_``\x80___``a\x0F\xF4a\x1A\xFAV[a\x0F\xFCa\x1B,V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x10`W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\xB5V[a\x10ta\x10l\x84a\x19\x14V[`\x12\x90a\x19JV[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[`\x14` R\x81_R`@_ \x81\x81T\x81\x10a\x10\x9DW_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_\x91\x82R`\x0C` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x01\x80Ta\x0B\\\x90a8\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x11` R`@\x81 a\x11$\x90a\x1BYV[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[a\x0C\x0F3\x83\x83a\x1B\x91V[a\x11I\x84\x84\x84a\x0C\x13V[a\x0C\x96\x84\x84\x84\x84a\x1C'V[\x83B\x11\x15a\x11yW`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\t\xB5V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x11\xF2\x90a\x11\xEA\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1DMV[\x85\x85\x85a\x1DyV[\x90Pa\x11\xFE\x81\x87a\x1D\xA5V[a\x12\x08\x81\x88a\x1ALV[PPPPPPPV[_a\x12\x1B\x81a\x18\x0EV[`\x17T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x12kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x12\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid service provider\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\xB5V[a\x12\xEB\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x83a\x18\x18V[PP`\x17\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[``a\x0BH\x82a\x1D\xF7V[`\x13` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`@\x1B\x03\x83\x16\x93`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x13\\\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x88\x90a8\x0EV[\x80\x15a\x13\xD3W\x80`\x1F\x10a\x13\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xD3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xB6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[_\x82\x81R`\x0C` R`@\x90 `\x01\x01Ta\x13\xF7\x81a\x18\x0EV[a\x0C\x96\x83\x83a\x18\xA9V[4g\x01cEx]\x8A\0\0\x14a\x14XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\t\xB5V[`\x15\x80T_\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x82a\x14t\x83a8ZV[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x92\x82\x02\x19\x16\x91\x90\x91\x17\x90\x91U`@\x80Q``\x81\x01\x82R\x83\x83\x16\x80\x82R3` \x80\x84\x01\x91\x82R\x83\x85\x01\x89\x81R_\x93\x84R`\x13\x90\x91R\x93\x90\x91 \x82Q\x81T\x92Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x95\x16\x94\x90\x94\x17\x17\x83U\x90Q\x92\x93P\x91\x82\x91\x90`\x01\x82\x01\x90a\x15\x05\x90\x82a8\xC8V[PP3_\x81\x81R`\x14` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x91\x90\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x91\x92P\x90\x7F\xF3\xF4\x11\xD8SHk\x9FS\xDAc\0\x9A!\xCD(N\xA1\x8A\x80\rM\xE5\\\xE5\xBD\x93]\x19~L\xF1\x90a\x15\x87\x90\x87\x90a2\xF7V[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x81\x16_\x90\x81R`\x13` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x16\x06\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x162\x90a8\x0EV[\x80\x15a\x16}W\x80`\x1F\x10a\x16TWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16}V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16`W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x14` R`@\x81 T\x82\x10a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x14` R`@\x90 \x80T\x83\x90\x81\x10a\x17?Wa\x17?a8FV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P\x92\x91PPV[a\x0C\x0F\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x1E\xFAV[a\x0C\x0F\x82\x82a\x1F\x10V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\x0BHWPa\x0BH\x82a\x1F_V[_\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x0BHW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\xB5V[a\rV\x83\x83\x83`\x01a\x1F\x83V[_a\x18\x06\x84\x84\x84a \x87V[\x94\x93PPPPV[a\x0E\x02\x813a \xA2V[_a\x18#\x83\x83a\x10\xCBV[a\x18\xA2W_\x83\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x18Z3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x0BHV[P_a\x0BHV[_a\x18\xB4\x83\x83a\x10\xCBV[\x15a\x18\xA2W_\x83\x81R`\x0C` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\x0BHV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x19FW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[P\x90V[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x19\xA6W_a\x19c\x84a \xDBV[a\x19m\x90\x85a9\x82V[_\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x19\x96W\x80\x91Pa\x19\xA4V[a\x19\xA1\x81`\x01a9\x95V[\x92P[P[_a\x19\xB3\x87\x87\x85\x85a!\xBFV[\x90P\x80\x15a\x19\xEDWa\x19\xD7\x87a\x19\xCA`\x01\x84a9\x82V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x19\xEFV[_[\x97\x96PPPPPPPV[a\x1A\x02a\"\x1EV[`\x0B\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x10` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\rV\x81\x83a\x1A\xB8\x86a\"CV[a\"MV[a\x1A\xC5a#\xB6V[`\x0B\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x1A/3\x90V[``a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\ra#\xDAV[\x90P\x90V[``a\x1B'\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0Ea#\xDAV[\x80T_\x90\x80\x15a\x1B\x88Wa\x1Br\x83a\x19\xCA`\x01\x84a9\x82V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x1B\x8AV[_[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B\xC3W`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\t\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x15\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x0C\x96W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\x1Ci\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a9\xA8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x1C\xA3WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1C\xA0\x91\x81\x01\x90a9\xE4V[`\x01[a\x1D\nW=\x80\x80\x15a\x1C\xD0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\xD5V[``\x91P[P\x80Q_\x03a\x1D\x02W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\xB5V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x1DFW`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\xB5V[PPPPPV[_a\x0BHa\x1DYa$\x83V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a\x1D\x89\x88\x88\x88\x88a%\xACV[\x92P\x92P\x92Pa\x1D\x99\x82\x82a&tV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x0F` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\rVW`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\xB5V[``a\x1E\x02\x82a\x17\xB5V[P_\x82\x81R`\n` R`@\x81 \x80Ta\x1E\x1B\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1EG\x90a8\x0EV[\x80\x15a\x1E\x92W\x80`\x1F\x10a\x1EiWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EuW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P_a\x1E\xAE`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[\x90P\x80Q_\x03a\x1E\xBFWP\x92\x91PPV[\x81Q\x15a\x1E\xF1W\x80\x82`@Q` \x01a\x1E\xD9\x92\x91\x90a9\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x91\x90PV[a\x18\x06\x84a',V[a\x1F\x04\x83\x83a'\x9CV[a\rV_\x84\x84\x84a\x1C'V[_\x82\x81R`\n` R`@\x90 a\x1F'\x82\x82a8\xC8V[P`@Q\x82\x81R\x7F\xF8\xE1\xA1Z\xBA\x93\x98\xE0\x19\xF0\xB4\x9D\xF1\xA4\xFD\xE9\x8E\xE1z\xE3E\xCB_k^,'\xF5\x03>\x8C\xE7\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c$\x83$\x83`\xE1\x1B\x14\x80a\x0BHWPa\x0BH\x82a'\xFDV[\x80\x80a\x1F\x97WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a XW_a\x1F\xA6\x84a\x17\xB5V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1F\xD2WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x1F\xE5WPa\x1F\xE3\x81\x84a\x16\x8DV[\x15[\x15a \x0EW`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\t\xB5V[\x81\x15a VW\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP_\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[__a \x94\x85\x85\x85a(!V[\x90Pa\x18\x06\x81\x86`\x01a(5V[a \xAC\x82\x82a\x10\xCBV[a\x0C\x0FW`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[_\x81_\x03a \xEAWP_\x91\x90PV[_`\x01a \xF6\x84a(\xAAV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a!\x0FWa!\x0Fa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!'Wa!'a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!?Wa!?a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!WWa!Wa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!oWa!oa:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!\x87Wa!\x87a:-V[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a!\x9FWa!\x9Fa:-V[\x04\x82\x01\x90\x1C\x90Pa\x1B\x8A\x81\x82\x85\x81a!\xB9Wa!\xB9a:-V[\x04a)=V[_[\x81\x83\x10\x15a\"\x16W_a!\xD4\x84\x84a)RV[_\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\"\x02W\x80\x92Pa\"\x10V[a\"\r\x81`\x01a9\x95V[\x93P[Pa!\xC1V[P\x93\x92PPPV[`\x0BT`\xFF\x16a\"AW`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x0BH\x82a\x0FlV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\"nWP_\x81\x11[\x15a\rVW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a#\x15W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x11` R`@\x81 \x81\x90a\"\xB0\x90a)la\"\xAB\x86a)wV[a)\xAAV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa#\n\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\rVW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x11` R`@\x81 \x81\x90a#M\x90a)\xDBa\"\xAB\x86a)wV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa#\xA7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x0BT`\xFF\x16\x15a\"AW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\xFF\x83\x14a#\xF4Wa#\xED\x83a)\xE6V[\x90Pa\x0BHV[\x81\x80Ta$\0\x90a8\x0EV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$,\x90a8\x0EV[\x80\x15a$wW\x80`\x1F\x10a$NWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$wV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$ZW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\x0BHV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a$\xDBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a%\x05WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x1B'`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a%\xE5WP_\x91P`\x03\x90P\x82a&jV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a&6W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a&aWP_\x92P`\x01\x91P\x82\x90Pa&jV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a&\x87Wa&\x87a:AV[\x03a&\x90WPPV[`\x01\x82`\x03\x81\x11\x15a&\xA4Wa&\xA4a:AV[\x03a&\xC2W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a&\xD6Wa&\xD6a:AV[\x03a&\xF7W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[`\x03\x82`\x03\x81\x11\x15a'\x0BWa'\x0Ba:AV[\x03a\x0C\x0FW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[``a'7\x82a\x17\xB5V[P_a'M`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[\x90P_\x81Q\x11a'kW`@Q\x80` \x01`@R\x80_\x81RPa\x1B\x8AV[\x80a'u\x84a*#V[`@Q` \x01a'\x86\x92\x91\x90a9\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a'\xC5W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_a'\xD1\x83\x83_a\x17\xFAV[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\rVW`@Qc9\xE3V7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\xB5V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14\x80a\x0BHWPa\x0BH\x82a*\xB2V[_a(*a#\xB6V[a\x18\x06\x84\x84\x84a+\x01V[`\x01`\x01`\xA0\x1B\x03\x83\x16a(WWa(T`\x12a)\xDBa\"\xAB\x84a)wV[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a(yWa(v`\x12a)la\"\xAB\x84a)wV[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x10` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\rV\x92\x91\x82\x16\x91\x16\x83a\"MV[_\x80`\x80\x83\x90\x1C\x15a(\xBEW`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a(\xD0W`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a(\xE2W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a(\xF4W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a)\x06W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a)\x18W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a)*W`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\x0BHW`\x01\x01\x92\x91PPV[_\x81\x83\x10a)KW\x81a\x1B\x8AV[P\x90\x91\x90PV[_a)``\x02\x84\x84\x18a:UV[a\x1B\x8A\x90\x84\x84\x16a9\x95V[_a\x1B\x8A\x82\x84a:tV[_`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\x19FW`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\xB5V[_\x80a)\xCEBa)\xC6a)\xBC\x88a\x1BYV[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a+\xCCV[\x91P\x91P[\x93P\x93\x91PPV[_a\x1B\x8A\x82\x84a:\x93V[``_a)\xF2\x83a+\xD9V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[``_a*/\x83a,\0V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a*MWa*Ma4\xAFV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a*wW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a*\x81WP\x93\x92PPPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a*\xE2WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\x0BHWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x0BHV[__a+\x0E\x85\x85\x85a,\xD7V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a+jWa+e\x84`\x08\x80T_\x83\x81R`\t` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x01UV[a+\x8DV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x8DWa+\x8D\x81\x85a-\xC9V[`\x01`\x01`\xA0\x1B\x03\x85\x16a+\xA9Wa+\xA4\x84a.VV[a\x18\x06V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x06Wa\x18\x06\x85\x85a.\xFDV[_\x80a)\xCE\x85\x85\x85a/KV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0BHW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a,>Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a,jWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a,\x88Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a,\xA0Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a,\xB4Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a,\xC6W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0BHW`\x01\x01\x92\x91PPV[_\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a-\x03Wa-\x03\x81\x84\x86a0\xC1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a-=Wa-\x1E_\x85__a\x1F\x83V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a-kW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[_\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[_a-\xD3\x83a\x0FlV[_\x83\x81R`\x07` R`@\x90 T\x90\x91P\x80\x82\x14a.$W`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x07\x90\x91R\x90 \x81\x90U[P_\x91\x82R`\x07` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x06\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\x08T_\x90a.g\x90`\x01\x90a9\x82V[_\x83\x81R`\t` R`@\x81 T`\x08\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a.\x8EWa.\x8Ea8FV[\x90_R` _ \x01T\x90P\x80`\x08\x83\x81T\x81\x10a.\xADWa.\xADa8FV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\t\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\x08\x80T\x80a.\xE4Wa.\xE4a:\xB2V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPV[_`\x01a/\t\x84a\x0FlV[a/\x13\x91\x90a9\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x07\x90R\x91\x90\x91 \x91\x90\x91UPV[\x82T_\x90\x81\x90\x80\x15a0gW_a/g\x87a\x19\xCA`\x01\x85a9\x82V[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a/\xBBW`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a0\x07W\x84a/\xDE\x88a\x19\xCA`\x01\x86a9\x82V[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua0WV[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa)\xD3\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a)\xD3V[a0\xCC\x83\x83\x83a1%V[a\rVW`\x01`\x01`\xA0\x1B\x03\x83\x16a0\xFAW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\xB5V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\xB5V[_`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x18\x06WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a1^WPa1^\x84\x84a\x16\x8DV[\x80a\x18\x06WPP_\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[P\x80Ta1\x92\x90a8\x0EV[_\x82U\x80`\x1F\x10a1\xA1WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x0E\x02\x91\x90[\x80\x82\x11\x15a\x19FW_\x81U`\x01\x01a1\xB9V[__\x83`\x1F\x84\x01\x12a1\xDCW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xF2W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\tW__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a2#W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a28W__\xFD[a2D\x87\x82\x88\x01a1\xCCV[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2bW__\xFD[a2n\x87\x82\x88\x01a1\xCCV[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0E\x02W__\xFD[_` \x82\x84\x03\x12\x15a2\x9FW__\xFD[\x815a\x1B\x8A\x81a2zV[_[\x83\x81\x10\x15a2\xC4W\x81\x81\x01Q\x83\x82\x01R` \x01a2\xACV[PP_\x91\x01RV[_\x81Q\x80\x84Ra2\xE3\x81` \x86\x01` \x86\x01a2\xAAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x1B\x8A` \x83\x01\x84a2\xCCV[_` \x82\x84\x03\x12\x15a3\x19W__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a36W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a3LW__\xFD[a3U\x83a3 V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a3uW__\xFD[a3~\x84a3 V[\x92Pa3\x8C` \x85\x01a3 V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a3\xAEW__\xFD[\x825\x91Pa3\xBE` \x84\x01a3 V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a3\xD7W__\xFD[a\x1B\x8A\x82a3 V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a3\xFE`\xE0\x83\x01\x89a2\xCCV[\x82\x81\x03`@\x84\x01Ra4\x10\x81\x89a2\xCCV[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a4eW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4GV[P\x90\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a4\x87W__\xFD[a4\x90\x83a3 V[\x91P` \x83\x015\x80\x15\x15\x81\x14a4\xA4W__\xFD[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE5Wa4\xE5a4\xAFV[`@R\x90V[__`\x01`\x01`@\x1B\x03\x84\x11\x15a5\x04Wa5\x04a4\xAFV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a52Wa52a4\xAFV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a5IW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a5oW__\xFD[a\x1B\x8A\x83\x835` \x85\x01a4\xEBV[____`\x80\x85\x87\x03\x12\x15a5\x91W__\xFD[a5\x9A\x85a3 V[\x93Pa5\xA8` \x86\x01a3 V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xC9W__\xFD[a5\xD5\x87\x82\x88\x01a5`V[\x91PP\x92\x95\x91\x94P\x92PV[______`\xC0\x87\x89\x03\x12\x15a5\xF6W__\xFD[a5\xFF\x87a3 V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a6\"W__\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0E\x02W__\xFD[_` \x82\x84\x03\x12\x15a6`W__\xFD[\x815a\x1B\x8A\x81a6<V[`\x01`\x01`@\x1B\x03\x84\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90a6\x9D\x90\x83\x01\x84a2\xCCV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a6\xB6W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xCBW__\xFD[a\x18\x06\x84\x82\x85\x01a5`V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x18\x06`\x80\x84\x01\x82a2\xCCV[__`@\x83\x85\x03\x12\x15a7'W__\xFD[a70\x83a3 V[\x91Pa3\xBE` \x84\x01a3 V[_` \x82\x84\x03\x12\x15a7NW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a7cW__\xFD[\x82\x01``\x81\x85\x03\x12\x15a7tW__\xFD[a7|a4\xC3V[a7\x85\x82a3 V[\x81R` \x82\x015a7\x95\x81a6<V[` \x82\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xB2W__\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12a7\xC6W__\xFD[a7\xD5\x85\x835` \x85\x01a4\xEBV[`@\x82\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a8\x07Wa8\x07a7\xE2V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a8\"W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a8@WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a8{Wa8{a7\xE2V[`\x01\x01\x92\x91PPV[`\x1F\x82\x11\x15a\rVW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a8\xA9WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1DFW_\x81U`\x01\x01a8\xB5V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE1Wa8\xE1a4\xAFV[a8\xF5\x81a8\xEF\x84Ta8\x0EV[\x84a8\x84V[` `\x1F\x82\x11`\x01\x81\x14a9'W_\x83\x15a9\x10WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1DFV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a9VW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a96V[P\x84\x82\x10\x15a9sW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03\x81\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[\x80\x82\x01\x80\x82\x11\x15a\x0BHWa\x0BHa7\xE2V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a9\xDA\x90\x83\x01\x84a2\xCCV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a9\xF4W__\xFD[\x81Qa\x1B\x8A\x81a2zV[_\x83Qa:\x10\x81\x84` \x88\x01a2\xAAV[\x83Q\x90\x83\x01\x90a:$\x81\x83` \x88\x01a2\xAAV[\x01\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x82a:oWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0BHWa\x0BHa7\xE2V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xD9u\x0F\\\xC7\xBBj\x10\x147_c)\xD7\x01\xEDl^t\x80S\t^\x90\x04\xDA\xC5]\xAC\xBE\x93cdsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `AccessControlBadConfirmation()` and selector `0x6697b232`.
    ```solidity
    error AccessControlBadConfirmation();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlBadConfirmation {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlBadConfirmation> for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlBadConfirmation) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AccessControlBadConfirmation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlBadConfirmation {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlBadConfirmation()";
            const SELECTOR: [u8; 4] = [102u8, 151u8, 178u8, 50u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `AccessControlUnauthorizedAccount(address,bytes32)` and selector `0xe2517d3f`.
    ```solidity
    error AccessControlUnauthorizedAccount(address account, bytes32 neededRole);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AccessControlUnauthorizedAccount {
        pub account: alloy::sol_types::private::Address,
        pub neededRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::FixedBytes<32>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> =
            (alloy::sol_types::private::Address, alloy::sol_types::private::FixedBytes<32>);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AccessControlUnauthorizedAccount> for UnderlyingRustTuple<'_> {
            fn from(value: AccessControlUnauthorizedAccount) -> Self {
                (value.account, value.neededRole)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AccessControlUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0, neededRole: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AccessControlUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AccessControlUnauthorizedAccount(address,bytes32)";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 125u8, 63u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.neededRole),
                )
            }
        }
    };
    /**Custom error with signature `CheckpointUnorderedInsertion()` and selector `0x2520601d`.
    ```solidity
    error CheckpointUnorderedInsertion();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointUnorderedInsertion {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CheckpointUnorderedInsertion> for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointUnorderedInsertion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckpointUnorderedInsertion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointUnorderedInsertion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointUnorderedInsertion()";
            const SELECTOR: [u8; 4] = [37u8, 32u8, 96u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
    ```solidity
    error ECDSAInvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignature()";
            const SELECTOR: [u8; 4] = [246u8, 69u8, 238u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureLength(uint256)` and selector `0xfce698f7`.
    ```solidity
    error ECDSAInvalidSignatureLength(uint256 length);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureLength {
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureLength> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureLength) -> Self {
                (value.length,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { length: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureLength(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 230u8, 152u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.length,
                ),)
            }
        }
    };
    /**Custom error with signature `ECDSAInvalidSignatureS(bytes32)` and selector `0xd78bce0c`.
    ```solidity
    error ECDSAInvalidSignatureS(bytes32 s);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignatureS {
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ECDSAInvalidSignatureS> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAInvalidSignatureS) -> Self {
                (value.s,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAInvalidSignatureS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { s: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECDSAInvalidSignatureS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSAInvalidSignatureS(bytes32)";
            const SELECTOR: [u8; 4] = [215u8, 139u8, 206u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
        }
    };
    /**Custom error with signature `ERC5805FutureLookup(uint256,uint48)` and selector `0xecd3f81e`.
    ```solidity
    error ERC5805FutureLookup(uint256 timepoint, uint48 clock);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC5805FutureLookup {
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
        pub clock: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<48>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U48,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC5805FutureLookup> for UnderlyingRustTuple<'_> {
            fn from(value: ERC5805FutureLookup) -> Self {
                (value.timepoint, value.clock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC5805FutureLookup {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { timepoint: tuple.0, clock: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC5805FutureLookup {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC5805FutureLookup(uint256,uint48)";
            const SELECTOR: [u8; 4] = [236u8, 211u8, 248u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.timepoint,
                    ),
                    <alloy::sol_types::sol_data::Uint<48> as alloy_sol_types::SolType>::tokenize(
                        &self.clock,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC6372InconsistentClock()` and selector `0x6ff07140`.
    ```solidity
    error ERC6372InconsistentClock();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC6372InconsistentClock {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC6372InconsistentClock> for UnderlyingRustTuple<'_> {
            fn from(value: ERC6372InconsistentClock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC6372InconsistentClock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC6372InconsistentClock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC6372InconsistentClock()";
            const SELECTOR: [u8; 4] = [111u8, 240u8, 113u8, 64u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ERC721EnumerableForbiddenBatchMint()` and selector `0x59171fc1`.
    ```solidity
    error ERC721EnumerableForbiddenBatchMint();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721EnumerableForbiddenBatchMint {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721EnumerableForbiddenBatchMint> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721EnumerableForbiddenBatchMint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721EnumerableForbiddenBatchMint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721EnumerableForbiddenBatchMint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721EnumerableForbiddenBatchMint()";
            const SELECTOR: [u8; 4] = [89u8, 23u8, 31u8, 193u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ERC721IncorrectOwner(address,uint256,address)` and selector `0x64283d7b`.
    ```solidity
    error ERC721IncorrectOwner(address sender, uint256 tokenId, address owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721IncorrectOwner {
        pub sender: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721IncorrectOwner> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721IncorrectOwner) -> Self {
                (value.sender, value.tokenId, value.owner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721IncorrectOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0, tokenId: tuple.1, owner: tuple.2 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721IncorrectOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721IncorrectOwner(address,uint256,address)";
            const SELECTOR: [u8; 4] = [100u8, 40u8, 61u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC721InsufficientApproval(address,uint256)` and selector `0x177e802f`.
    ```solidity
    error ERC721InsufficientApproval(address operator, uint256 tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InsufficientApproval {
        pub operator: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InsufficientApproval> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InsufficientApproval) -> Self {
                (value.operator, value.tokenId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InsufficientApproval {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { operator: tuple.0, tokenId: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InsufficientApproval {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InsufficientApproval(address,uint256)";
            const SELECTOR: [u8; 4] = [23u8, 126u8, 128u8, 47u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `ERC721InvalidApprover(address)` and selector `0xa9fbf51f`.
    ```solidity
    error ERC721InvalidApprover(address approver);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InvalidApprover {
        pub approver: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InvalidApprover> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InvalidApprover) -> Self {
                (value.approver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InvalidApprover {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { approver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InvalidApprover {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InvalidApprover(address)";
            const SELECTOR: [u8; 4] = [169u8, 251u8, 245u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.approver,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721InvalidOperator(address)` and selector `0x5b08ba18`.
    ```solidity
    error ERC721InvalidOperator(address operator);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InvalidOperator {
        pub operator: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InvalidOperator> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InvalidOperator) -> Self {
                (value.operator,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InvalidOperator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { operator: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InvalidOperator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InvalidOperator(address)";
            const SELECTOR: [u8; 4] = [91u8, 8u8, 186u8, 24u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.operator,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721InvalidOwner(address)` and selector `0x89c62b64`.
    ```solidity
    error ERC721InvalidOwner(address owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InvalidOwner {
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InvalidOwner(address)";
            const SELECTOR: [u8; 4] = [137u8, 198u8, 43u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.owner,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721InvalidReceiver(address)` and selector `0x64a0ae92`.
    ```solidity
    error ERC721InvalidReceiver(address receiver);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InvalidReceiver {
        pub receiver: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InvalidReceiver> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InvalidReceiver) -> Self {
                (value.receiver,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InvalidReceiver {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { receiver: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InvalidReceiver {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InvalidReceiver(address)";
            const SELECTOR: [u8; 4] = [100u8, 160u8, 174u8, 146u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.receiver,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721InvalidSender(address)` and selector `0x73c6ac6e`.
    ```solidity
    error ERC721InvalidSender(address sender);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721InvalidSender {
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721InvalidSender> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721InvalidSender) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721InvalidSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721InvalidSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721InvalidSender(address)";
            const SELECTOR: [u8; 4] = [115u8, 198u8, 172u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.sender,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721NonexistentToken(uint256)` and selector `0x7e273289`.
    ```solidity
    error ERC721NonexistentToken(uint256 tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721NonexistentToken {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721NonexistentToken> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721NonexistentToken) -> Self {
                (value.tokenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721NonexistentToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tokenId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721NonexistentToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721NonexistentToken(uint256)";
            const SELECTOR: [u8; 4] = [126u8, 39u8, 50u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.tokenId,
                ),)
            }
        }
    };
    /**Custom error with signature `ERC721OutOfBoundsIndex(address,uint256)` and selector `0xa57d13dc`.
    ```solidity
    error ERC721OutOfBoundsIndex(address owner, uint256 index);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC721OutOfBoundsIndex {
        pub owner: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ERC721OutOfBoundsIndex> for UnderlyingRustTuple<'_> {
            fn from(value: ERC721OutOfBoundsIndex) -> Self {
                (value.owner, value.index)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ERC721OutOfBoundsIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0, index: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ERC721OutOfBoundsIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ERC721OutOfBoundsIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [165u8, 125u8, 19u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
    ```solidity
    error EnforcedPause();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
    ```solidity
    error ExpectedPause();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidAccountNonce(address,uint256)` and selector `0x752d88c0`.
    ```solidity
    error InvalidAccountNonce(address account, uint256 currentNonce);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAccountNonce {
        pub account: alloy::sol_types::private::Address,
        pub currentNonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidAccountNonce> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAccountNonce) -> Self {
                (value.account, value.currentNonce)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAccountNonce {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0, currentNonce: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAccountNonce {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAccountNonce(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 45u8, 136u8, 192u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.currentNonce,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
    ```solidity
    error InvalidShortString();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `SafeCastOverflowedUintDowncast(uint8,uint256)` and selector `0x6dfcc650`.
    ```solidity
    error SafeCastOverflowedUintDowncast(uint8 bits, uint256 value);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SafeCastOverflowedUintDowncast {
        pub bits: u8,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> =
            (alloy::sol_types::sol_data::Uint<8>, alloy::sol_types::sol_data::Uint<256>);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::primitives::aliases::U256);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SafeCastOverflowedUintDowncast> for UnderlyingRustTuple<'_> {
            fn from(value: SafeCastOverflowedUintDowncast) -> Self {
                (value.bits, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SafeCastOverflowedUintDowncast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { bits: tuple.0, value: tuple.1 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SafeCastOverflowedUintDowncast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SafeCastOverflowedUintDowncast(uint8,uint256)";
            const SELECTOR: [u8; 4] = [109u8, 252u8, 198u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.bits,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.value,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
    ```solidity
    error StringTooLong(string str);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        pub str: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.str,
                ),)
            }
        }
    };
    /**Custom error with signature `VotesExpiredSignature(uint256)` and selector `0x4683af0e`.
    ```solidity
    error VotesExpiredSignature(uint256 expiry);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VotesExpiredSignature {
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<VotesExpiredSignature> for UnderlyingRustTuple<'_> {
            fn from(value: VotesExpiredSignature) -> Self {
                (value.expiry,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VotesExpiredSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { expiry: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for VotesExpiredSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VotesExpiredSignature(uint256)";
            const SELECTOR: [u8; 4] = [70u8, 131u8, 175u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.expiry,
                ),)
            }
        }
    };
    /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
    ```solidity
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approved: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8, 66u8,
                    125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8, 41u8, 30u8,
                    91u8, 32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { owner: topics.1, approved: topics.2, tokenId: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.owner.clone(),
                    self.approved.clone(),
                    self.tokenId.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.approved,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tokenId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ApprovalForAll(address,address,bool)` and selector `0x17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c31`.
    ```solidity
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ApprovalForAll {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approved: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ApprovalForAll {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ApprovalForAll(address,address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    23u8, 48u8, 126u8, 171u8, 57u8, 171u8, 97u8, 7u8, 232u8, 137u8, 152u8, 69u8,
                    173u8, 61u8, 89u8, 189u8, 150u8, 83u8, 242u8, 0u8, 242u8, 32u8, 146u8, 4u8,
                    137u8, 202u8, 43u8, 89u8, 55u8, 105u8, 108u8, 49u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { owner: topics.1, operator: topics.2, approved: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                    &self.approved,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ApprovalForAll {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ApprovalForAll> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ApprovalForAll) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `BatchMetadataUpdate(uint256,uint256)` and selector `0x6bd5c950a8d8df17f772f5af37cb3655737899cbf903264b9795592da439661c`.
    ```solidity
    event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct BatchMetadataUpdate {
        #[allow(missing_docs)]
        pub _fromTokenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _toTokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for BatchMetadataUpdate {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "BatchMetadataUpdate(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    107u8, 213u8, 201u8, 80u8, 168u8, 216u8, 223u8, 23u8, 247u8, 114u8, 245u8,
                    175u8, 55u8, 203u8, 54u8, 85u8, 115u8, 120u8, 153u8, 203u8, 249u8, 3u8, 38u8,
                    75u8, 151u8, 149u8, 89u8, 45u8, 164u8, 57u8, 102u8, 28u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _fromTokenId: data.0, _toTokenId: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._fromTokenId,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._toTokenId,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for BatchMetadataUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&BatchMetadataUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &BatchMetadataUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Debug(string,uint256)` and selector `0x3c5ad147104e56be34a9176a6692f7df8d2f4b29a5af06bc6b98970d329d6577`.
    ```solidity
    event Debug(string message, uint256 tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Debug {
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Debug {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::String, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Debug(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    60u8, 90u8, 209u8, 71u8, 16u8, 78u8, 86u8, 190u8, 52u8, 169u8, 23u8, 106u8,
                    102u8, 146u8, 247u8, 223u8, 141u8, 47u8, 75u8, 41u8, 165u8, 175u8, 6u8, 188u8,
                    107u8, 152u8, 151u8, 13u8, 50u8, 157u8, 101u8, 119u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { message: data.0, tokenId: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Debug {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Debug> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Debug) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateChanged(address,address,address)` and selector `0x3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f`.
    ```solidity
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct DelegateChanged {
        #[allow(missing_docs)]
        pub delegator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fromDelegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub toDelegate: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "DelegateChanged(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8, 1u8,
                    30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8, 77u8, 78u8,
                    249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { delegator: topics.1, fromDelegate: topics.2, toDelegate: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.delegator.clone(),
                    self.fromDelegate.clone(),
                    self.toDelegate.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.fromDelegate,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.toDelegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelegateVotesChanged(address,uint256,uint256)` and selector `0xdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724`.
    ```solidity
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct DelegateVotesChanged {
        #[allow(missing_docs)]
        pub delegate: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub previousVotes: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newVotes: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DelegateVotesChanged {
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "DelegateVotesChanged(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8,
                    155u8, 82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8, 200u8,
                    47u8, 219u8, 11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { delegate: topics.1, previousVotes: data.0, newVotes: data.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.previousVotes,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newVotes,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.delegate.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.delegate,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DelegateVotesChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelegateVotesChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DelegateVotesChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EIP712DomainChanged()` and selector `0x0a6387c9ea3628b88a633bb4f3b151770f70085117a15f9bf3787cda53f13d31`.
    ```solidity
    event EIP712DomainChanged();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct EIP712DomainChanged {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EIP712DomainChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EIP712DomainChanged()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8, 180u8,
                    243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8, 95u8, 155u8,
                    243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {}
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EIP712DomainChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EIP712DomainChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EIP712DomainChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MetadataUpdate(uint256)` and selector `0xf8e1a15aba9398e019f0b49df1a4fde98ee17ae345cb5f6b5e2c27f5033e8ce7`.
    ```solidity
    event MetadataUpdate(uint256 _tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct MetadataUpdate {
        #[allow(missing_docs)]
        pub _tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for MetadataUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MetadataUpdate(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    248u8, 225u8, 161u8, 90u8, 186u8, 147u8, 152u8, 224u8, 25u8, 240u8, 180u8,
                    157u8, 241u8, 164u8, 253u8, 233u8, 142u8, 225u8, 122u8, 227u8, 69u8, 203u8,
                    95u8, 107u8, 94u8, 44u8, 39u8, 245u8, 3u8, 62u8, 140u8, 231u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _tokenId: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self._tokenId,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MetadataUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MetadataUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MetadataUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NFTMinted(address,uint256,string)` and selector `0xd35bb95e09c04b219e35047ce7b7b300e3384264ef84a40456943dbc0fc17c14`.
    ```solidity
    event NFTMinted(address indexed to, uint256 indexed tokenId, string dataUri);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct NFTMinted {
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub dataUri: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NFTMinted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NFTMinted(address,uint256,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    211u8, 91u8, 185u8, 94u8, 9u8, 192u8, 75u8, 33u8, 158u8, 53u8, 4u8, 124u8,
                    231u8, 183u8, 179u8, 0u8, 227u8, 56u8, 66u8, 100u8, 239u8, 132u8, 164u8, 4u8,
                    86u8, 148u8, 61u8, 188u8, 15u8, 193u8, 124u8, 20u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { to: topics.1, tokenId: topics.2, dataUri: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                    &self.dataUri,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.to.clone(), self.tokenId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tokenId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NFTMinted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NFTMinted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NFTMinted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NewTrigger(uint64,address,bytes)` and selector `0xf3f411d853486b9f53da63009a21cd284ea18a800d4de55ce5bd935d197e4cf1`.
    ```solidity
    event NewTrigger(ISimpleTrigger.TriggerId indexed triggerId, address indexed creator, bytes data);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct NewTrigger {
        #[allow(missing_docs)]
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub creator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewTrigger {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                ISimpleTrigger::TriggerId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NewTrigger(uint64,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    243u8, 244u8, 17u8, 216u8, 83u8, 72u8, 107u8, 159u8, 83u8, 218u8, 99u8, 0u8,
                    154u8, 33u8, 205u8, 40u8, 78u8, 161u8, 138u8, 128u8, 13u8, 77u8, 229u8, 92u8,
                    229u8, 189u8, 147u8, 93u8, 25u8, 126u8, 76u8, 241u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { triggerId: topics.1, creator: topics.2, data: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self.data,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.triggerId.clone(), self.creator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] =
                    <ISimpleTrigger::TriggerId as alloy_sol_types::EventTopic>::encode_topic(
                        &self.triggerId,
                    );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.creator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTrigger {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTrigger> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTrigger) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
    ```solidity
    event Paused(address account);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8, 2u8,
                    112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8, 71u8, 84u8,
                    235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.account,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleAdminChanged(bytes32,bytes32,bytes32)` and selector `0xbd79b86ffe0ab8e8776151514217cd7cacd52c909f66475c3af44e129f0b00ff`.
    ```solidity
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previousAdminRole, bytes32 indexed newAdminRole);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct RoleAdminChanged {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub previousAdminRole: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub newAdminRole: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleAdminChanged {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RoleAdminChanged(bytes32,bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8, 81u8,
                    66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8, 71u8, 92u8,
                    58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { role: topics.1, previousAdminRole: topics.2, newAdminRole: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.previousAdminRole.clone(),
                    self.newAdminRole.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.previousAdminRole);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.newAdminRole);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleAdminChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleAdminChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleAdminChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleGranted(bytes32,address,address)` and selector `0x2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d`.
    ```solidity
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct RoleGranted {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleGranted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleGranted(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8, 236u8,
                    121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8, 64u8, 48u8,
                    69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { role: topics.1, account: topics.2, sender: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleGranted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleGranted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleGranted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RoleRevoked(bytes32,address,address)` and selector `0xf6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b`.
    ```solidity
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct RoleRevoked {
        #[allow(missing_docs)]
        pub role: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RoleRevoked {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RoleRevoked(bytes32,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8, 103u8,
                    11u8, 68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8, 253u8, 100u8,
                    235u8, 33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { role: topics.1, account: topics.2, sender: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.role.clone(),
                    self.account.clone(),
                    self.sender.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.role);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.account,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RoleRevoked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RoleRevoked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RoleRevoked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
    ```solidity
    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8, 176u8,
                    104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8, 196u8,
                    161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { from: topics.1, to: topics.2, tokenId: topics.3 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.from.clone(),
                    self.to.clone(),
                    self.tokenId.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.tokenId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
    ```solidity
    event Unpaused(address account);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8, 167u8,
                    131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8, 78u8, 83u8,
                    123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.account,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {}
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Function with signature `CLOCK_MODE()` and selector `0x4bf5d7e9`.
    ```solidity
    function CLOCK_MODE() external pure returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODECall {}
    ///Container type for the return parameters of the [`CLOCK_MODE()`](CLOCK_MODECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CLOCK_MODEReturn {
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODECall> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CLOCK_MODEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CLOCK_MODEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CLOCK_MODEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CLOCK_MODECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = CLOCK_MODEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CLOCK_MODE()";
            const SELECTOR: [u8; 4] = [75u8, 245u8, 215u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `DEFAULT_ADMIN_ROLE()` and selector `0xa217fddf`.
    ```solidity
    function DEFAULT_ADMIN_ROLE() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLECall {}
    ///Container type for the return parameters of the [`DEFAULT_ADMIN_ROLE()`](DEFAULT_ADMIN_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEFAULT_ADMIN_ROLEReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEFAULT_ADMIN_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEFAULT_ADMIN_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DEFAULT_ADMIN_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEFAULT_ADMIN_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEFAULT_ADMIN_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEFAULT_ADMIN_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEFAULT_ADMIN_ROLE()";
            const SELECTOR: [u8; 4] = [162u8, 23u8, 253u8, 223u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `PAUSER_ROLE()` and selector `0xe63ab1e9`.
    ```solidity
    function PAUSER_ROLE() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAUSER_ROLECall {}
    ///Container type for the return parameters of the [`PAUSER_ROLE()`](PAUSER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PAUSER_ROLEReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PAUSER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: PAUSER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PAUSER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<PAUSER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: PAUSER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for PAUSER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for PAUSER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = PAUSER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PAUSER_ROLE()";
            const SELECTOR: [u8; 4] = [230u8, 58u8, 177u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `SERVICE_PROVIDER_ROLE()` and selector `0x1bc6ae8a`.
    ```solidity
    function SERVICE_PROVIDER_ROLE() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SERVICE_PROVIDER_ROLECall {}
    ///Container type for the return parameters of the [`SERVICE_PROVIDER_ROLE()`](SERVICE_PROVIDER_ROLECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SERVICE_PROVIDER_ROLEReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SERVICE_PROVIDER_ROLECall> for UnderlyingRustTuple<'_> {
                fn from(value: SERVICE_PROVIDER_ROLECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SERVICE_PROVIDER_ROLECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SERVICE_PROVIDER_ROLEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SERVICE_PROVIDER_ROLEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SERVICE_PROVIDER_ROLEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SERVICE_PROVIDER_ROLECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = SERVICE_PROVIDER_ROLEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SERVICE_PROVIDER_ROLE()";
            const SELECTOR: [u8; 4] = [27u8, 198u8, 174u8, 138u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `addTrigger(bytes)` and selector `0xe31e0788`.
    ```solidity
    function addTrigger(bytes memory data) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerCall {
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`addTrigger(bytes)`](addTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTriggerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addTriggerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addTriggerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addTriggerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addTrigger(bytes)";
            const SELECTOR: [u8; 4] = [227u8, 30u8, 7u8, 136u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                    &self.data,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
    ```solidity
    function approve(address to, uint256 tokenId) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveCall {
        pub to: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.to, value.tokenId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0, tokenId: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
    ```solidity
    function balanceOf(address owner) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address)";
            const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.owner,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `burn(uint256)` and selector `0x42966c68`.
    ```solidity
    function burn(uint256 tokenId) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnCall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct burnReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(uint256)";
            const SELECTOR: [u8; 4] = [66u8, 150u8, 108u8, 104u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.tokenId,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `clock()` and selector `0x91ddadf4`.
    ```solidity
    function clock() external view returns (uint48);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockCall {}
    ///Container type for the return parameters of the [`clock()`](clockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U48,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockCall> for UnderlyingRustTuple<'_> {
                fn from(value: clockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U48,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = clockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<48>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clock()";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 173u8, 244u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegate(address)` and selector `0x5c19a95c`.
    ```solidity
    function delegate(address delegatee) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateCall {
        pub delegatee: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegate(address)`](delegateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateCall) -> Self {
                    (value.delegatee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delegatee: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegate(address)";
            const SELECTOR: [u8; 4] = [92u8, 25u8, 169u8, 92u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.delegatee,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xc3cda520`.
    ```solidity
    function delegateBySig(address delegatee, uint256 nonce, uint256 expiry, uint8 v, bytes32 r, bytes32 s) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigCall {
        pub delegatee: alloy::sol_types::private::Address,
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
        pub v: u8,
        pub r: alloy::sol_types::private::FixedBytes<32>,
        pub s: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)`](delegateBySigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegateBySigReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                u8,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateBySigCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigCall) -> Self {
                    (value.delegatee, value.nonce, value.expiry, value.v, value.r, value.s)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        delegatee: tuple.0,
                        nonce: tuple.1,
                        expiry: tuple.2,
                        v: tuple.3,
                        r: tuple.4,
                        s: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegateBySigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegateBySigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegateBySigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegateBySigCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegateBySigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "delegateBySig(address,uint256,uint256,uint8,bytes32,bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 205u8, 165u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatee,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.v),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.r),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.s),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `delegates(address)` and selector `0x587cde1e`.
    ```solidity
    function delegates(address account) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`delegates(address)`](delegatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatesReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatesCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<delegatesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegatesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegates(address)";
            const SELECTOR: [u8; 4] = [88u8, 124u8, 222u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.account,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
    ```solidity
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        pub name: alloy::sol_types::private::String,
        pub version: alloy::sol_types::private::String,
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        pub verifyingContract: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub extensions:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getApproved(uint256)` and selector `0x081812fc`.
    ```solidity
    function getApproved(uint256 tokenId) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApprovedCall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getApproved(uint256)`](getApprovedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getApprovedReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getApprovedCall> for UnderlyingRustTuple<'_> {
                fn from(value: getApprovedCall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApprovedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getApprovedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getApprovedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getApprovedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getApprovedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getApprovedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getApproved(uint256)";
            const SELECTOR: [u8; 4] = [8u8, 24u8, 18u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.tokenId,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getPastTotalSupply(uint256)` and selector `0x8e539e8c`.
    ```solidity
    function getPastTotalSupply(uint256 timepoint) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyCall {
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastTotalSupply(uint256)`](getPastTotalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastTotalSupplyReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastTotalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyCall) -> Self {
                    (value.timepoint,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastTotalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timepoint: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastTotalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastTotalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastTotalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastTotalSupplyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastTotalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastTotalSupply(uint256)";
            const SELECTOR: [u8; 4] = [142u8, 83u8, 158u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.timepoint,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getPastVotes(address,uint256)` and selector `0x3a46b1a8`.
    ```solidity
    function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesCall {
        pub account: alloy::sol_types::private::Address,
        pub timepoint: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getPastVotes(address,uint256)`](getPastVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPastVotesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesCall) -> Self {
                    (value.account, value.timepoint)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0, timepoint: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPastVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPastVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPastVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPastVotesCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPastVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPastVotes(address,uint256)";
            const SELECTOR: [u8; 4] = [58u8, 70u8, 177u8, 168u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.timepoint,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getRoleAdmin(bytes32)` and selector `0x248a9ca3`.
    ```solidity
    function getRoleAdmin(bytes32 role) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRoleAdmin(bytes32)`](getRoleAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRoleAdminReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminCall) -> Self {
                    (value.role,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRoleAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRoleAdminReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRoleAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRoleAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRoleAdminReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRoleAdmin(bytes32)";
            const SELECTOR: [u8; 4] = [36u8, 138u8, 156u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getTrigger(uint64)` and selector `0xe328ed77`.
    ```solidity
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerCall {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getTrigger(uint64)`](getTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerReturn {
        pub _0: <ISimpleTrigger::TriggerInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerCall) -> Self {
                    (value.triggerId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTriggerCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTriggerReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTrigger(uint64)";
            const SELECTOR: [u8; 4] = [227u8, 40u8, 237u8, 119u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self.triggerId),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getTriggerCount(address)` and selector `0x3383abfe`.
    ```solidity
    function getTriggerCount(address creator) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerCountCall {
        pub creator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getTriggerCount(address)`](getTriggerCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerCountReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerCountCall) -> Self {
                    (value.creator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTriggerCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTriggerCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTriggerCount(address)";
            const SELECTOR: [u8; 4] = [51u8, 131u8, 171u8, 254u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.creator,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getTriggerIdAtIndex(address,uint256)` and selector `0xf7ee944c`.
    ```solidity
    function getTriggerIdAtIndex(address creator, uint256 index) external view returns (ISimpleTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerIdAtIndexCall {
        pub creator: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTriggerIdAtIndex(address,uint256)`](getTriggerIdAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTriggerIdAtIndexReturn {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerIdAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerIdAtIndexCall) -> Self {
                    (value.creator, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerIdAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator: tuple.0, index: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTriggerIdAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTriggerIdAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTriggerIdAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTriggerIdAtIndexCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTriggerIdAtIndexReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTriggerIdAtIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [247u8, 238u8, 148u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.creator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getVotes(address)` and selector `0x9ab24eb0`.
    ```solidity
    function getVotes(address account) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesCall {
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getVotes(address)`](getVotesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVotesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getVotesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getVotesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getVotesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getVotesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getVotesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getVotes(address)";
            const SELECTOR: [u8; 4] = [154u8, 178u8, 78u8, 176u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.account,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `grantRole(bytes32,address)` and selector `0x2f2ff15d`.
    ```solidity
    function grantRole(bytes32 role, address account) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`grantRole(bytes32,address)`](grantRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct grantRoleReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0, account: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<grantRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: grantRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for grantRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for grantRoleCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = grantRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "grantRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [47u8, 47u8, 241u8, 93u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `handleAddPayload(bytes,bytes)` and selector `0x0073e1d7`.
    ```solidity
    function handleAddPayload(bytes memory data, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAddPayloadCall {
        pub data: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`handleAddPayload(bytes,bytes)`](handleAddPayloadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAddPayloadReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Bytes, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Bytes, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleAddPayloadCall> for UnderlyingRustTuple<'_> {
                fn from(value: handleAddPayloadCall) -> Self {
                    (value.data, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleAddPayloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0, signature: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<handleAddPayloadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: handleAddPayloadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleAddPayloadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for handleAddPayloadCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Bytes, alloy::sol_types::sol_data::Bytes);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = handleAddPayloadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "handleAddPayload(bytes,bytes)";
            const SELECTOR: [u8; 4] = [0u8, 115u8, 225u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `hasRole(bytes32,address)` and selector `0x91d14854`.
    ```solidity
    function hasRole(bytes32 role, address account) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`hasRole(bytes32,address)`](hasRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRoleReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0, account: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRoleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRoleCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasRoleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [145u8, 209u8, 72u8, 84u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
    ```solidity
    function initialize(address _serviceProvider) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceProvider: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceProvider,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _serviceProvider: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._serviceProvider,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `initialized()` and selector `0x158ef93e`.
    ```solidity
    function initialized() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializedCall {}
    ///Container type for the return parameters of the [`initialized()`](initializedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializedReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializedCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialized()";
            const SELECTOR: [u8; 4] = [21u8, 142u8, 249u8, 62u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`.
    ```solidity
    function isApprovedForAll(address owner, address operator) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isApprovedForAllCall {
        pub owner: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isApprovedForAll(address,address)`](isApprovedForAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isApprovedForAllReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Address, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isApprovedForAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: isApprovedForAllCall) -> Self {
                    (value.owner, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isApprovedForAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0, operator: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isApprovedForAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isApprovedForAllReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isApprovedForAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isApprovedForAllCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isApprovedForAllReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isApprovedForAll(address,address)";
            const SELECTOR: [u8; 4] = [233u8, 133u8, 233u8, 197u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `name()` and selector `0x06fdde03`.
    ```solidity
    function name() external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameCall {}
    ///Container type for the return parameters of the [`name()`](nameCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nameReturn {
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                fn from(value: nameCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nameReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nameCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nameReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "name()";
            const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `nonces(address)` and selector `0x7ecebe00`.
    ```solidity
    function nonces(address owner) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesCall {
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`nonces(address)`](noncesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct noncesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesCall> for UnderlyingRustTuple<'_> {
                fn from(value: noncesCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<noncesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: noncesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for noncesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for noncesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = noncesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonces(address)";
            const SELECTOR: [u8; 4] = [126u8, 206u8, 190u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.owner,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `ownerOf(uint256)` and selector `0x6352211e`.
    ```solidity
    function ownerOf(uint256 tokenId) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerOfCall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`ownerOf(uint256)`](ownerOfCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerOfReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerOfCall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerOfReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerOfCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ownerOf(uint256)";
            const SELECTOR: [u8; 4] = [99u8, 82u8, 33u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.tokenId,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `pause()` and selector `0x8456cb59`.
    ```solidity
    function pause() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {}
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall {}
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pausedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceRole(bytes32,address)` and selector `0x36568abe`.
    ```solidity
    function renounceRole(bytes32 role, address callerConfirmation) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub callerConfirmation: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`renounceRole(bytes32,address)`](renounceRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceRoleReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleCall) -> Self {
                    (value.role, value.callerConfirmation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0, callerConfirmation: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceRoleCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [54u8, 86u8, 138u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.callerConfirmation,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `revokeRole(bytes32,address)` and selector `0xd547741f`.
    ```solidity
    function revokeRole(bytes32 role, address account) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleCall {
        pub role: alloy::sol_types::private::FixedBytes<32>,
        pub account: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`revokeRole(bytes32,address)`](revokeRoleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct revokeRoleReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::FixedBytes<32>, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleCall> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleCall) -> Self {
                    (value.role, value.account)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { role: tuple.0, account: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<revokeRoleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: revokeRoleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for revokeRoleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for revokeRoleCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = revokeRoleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "revokeRole(bytes32,address)";
            const SELECTOR: [u8; 4] = [213u8, 71u8, 116u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.role),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`.
    ```solidity
    function safeTransferFrom(address from, address to, uint256 tokenId) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeTransferFrom_0Call {
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`safeTransferFrom(address,address,uint256)`](safeTransferFrom_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeTransferFrom_0Return {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<safeTransferFrom_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: safeTransferFrom_0Call) -> Self {
                    (value.from, value.to, value.tokenId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeTransferFrom_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { from: tuple.0, to: tuple.1, tokenId: tuple.2 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<safeTransferFrom_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: safeTransferFrom_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeTransferFrom_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for safeTransferFrom_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = safeTransferFrom_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "safeTransferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [66u8, 132u8, 46u8, 14u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`.
    ```solidity
    function safeTransferFrom(address from, address to, uint256 tokenId, bytes memory data) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeTransferFrom_1Call {
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`safeTransferFrom(address,address,uint256,bytes)`](safeTransferFrom_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeTransferFrom_1Return {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<safeTransferFrom_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: safeTransferFrom_1Call) -> Self {
                    (value.from, value.to, value.tokenId, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeTransferFrom_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { from: tuple.0, to: tuple.1, tokenId: tuple.2, data: tuple.3 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<safeTransferFrom_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: safeTransferFrom_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeTransferFrom_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for safeTransferFrom_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = safeTransferFrom_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "safeTransferFrom(address,address,uint256,bytes)";
            const SELECTOR: [u8; 4] = [184u8, 141u8, 79u8, 222u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `serviceProvider()` and selector `0x8d69e95e`.
    ```solidity
    function serviceProvider() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceProviderCall {}
    ///Container type for the return parameters of the [`serviceProvider()`](serviceProviderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceProviderReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceProviderCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceProviderCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceProviderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceProviderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceProviderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceProviderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceProviderCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceProviderReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceProvider()";
            const SELECTOR: [u8; 4] = [141u8, 105u8, 233u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`.
    ```solidity
    function setApprovalForAll(address operator, bool approved) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setApprovalForAllCall {
        pub operator: alloy::sol_types::private::Address,
        pub approved: bool,
    }
    ///Container type for the return parameters of the [`setApprovalForAll(address,bool)`](setApprovalForAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setApprovalForAllReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bool);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setApprovalForAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: setApprovalForAllCall) -> Self {
                    (value.operator, value.approved)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setApprovalForAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0, approved: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setApprovalForAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setApprovalForAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setApprovalForAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setApprovalForAllCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bool);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setApprovalForAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setApprovalForAll(address,bool)";
            const SELECTOR: [u8; 4] = [162u8, 44u8, 180u8, 101u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.approved,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
    ```solidity
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall> for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::FixedBytes<4> as alloy_sol_types::SolType>::tokenize(
                    &self.interfaceId,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `symbol()` and selector `0x95d89b41`.
    ```solidity
    function symbol() external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolCall {}
    ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct symbolReturn {
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                fn from(value: symbolCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: symbolReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for symbolCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = symbolReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "symbol()";
            const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`.
    ```solidity
    function tokenByIndex(uint256 index) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenByIndexCall {
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`tokenByIndex(uint256)`](tokenByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenByIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenByIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenByIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenByIndex(uint256)";
            const SELECTOR: [u8; 4] = [79u8, 108u8, 204u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.index,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`.
    ```solidity
    function tokenOfOwnerByIndex(address owner, uint256 index) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOfOwnerByIndexCall {
        pub owner: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`tokenOfOwnerByIndex(address,uint256)`](tokenOfOwnerByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenOfOwnerByIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenOfOwnerByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOfOwnerByIndexCall) -> Self {
                    (value.owner, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOfOwnerByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0, index: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenOfOwnerByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenOfOwnerByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenOfOwnerByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenOfOwnerByIndexCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenOfOwnerByIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenOfOwnerByIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [47u8, 116u8, 92u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `tokenURI(uint256)` and selector `0xc87b56dd`.
    ```solidity
    function tokenURI(uint256 tokenId) external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenURICall {
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`tokenURI(uint256)`](tokenURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct tokenURIReturn {
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenURICall> for UnderlyingRustTuple<'_> {
                fn from(value: tokenURICall) -> Self {
                    (value.tokenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tokenId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tokenURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tokenURIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tokenURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tokenURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = tokenURIReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tokenURI(uint256)";
            const SELECTOR: [u8; 4] = [200u8, 123u8, 86u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self.tokenId,
                ),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `totalSupply()` and selector `0x18160ddd`.
    ```solidity
    function totalSupply() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyCall {}
    ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalSupplyReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<totalSupplyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalSupplyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalSupplyCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalSupplyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalSupply()";
            const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
    ```solidity
    function transferFrom(address from, address to, uint256 tokenId) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromCall {
        pub from: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub tokenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferFromReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.from, value.to, value.tokenId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { from: tuple.0, to: tuple.1, tokenId: tuple.2 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferFromReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
            const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.tokenId,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `triggerIdsByCreator(address,uint256)` and selector `0x913b1fbf`.
    ```solidity
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`triggerIdsByCreator(address,uint256)`](triggerIdsByCreatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggerIdsByCreatorReturn {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<triggerIdsByCreatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<triggerIdsByCreatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggerIdsByCreatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggerIdsByCreatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggerIdsByCreatorCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggerIdsByCreatorReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggerIdsByCreator(address,uint256)";
            const SELECTOR: [u8; 4] = [145u8, 59u8, 31u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `triggersById(uint64)` and selector `0xce289612`.
    ```solidity
    function triggersById(ISimpleTrigger.TriggerId) external view returns (ISimpleTrigger.TriggerId triggerId, address creator, bytes memory data);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdCall {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`triggersById(uint64)`](triggersByIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct triggersByIdReturn {
        pub triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        pub creator: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ISimpleTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<triggersByIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ISimpleTrigger::TriggerId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<triggersByIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: triggersByIdReturn) -> Self {
                    (value.triggerId, value.creator, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { triggerId: tuple.0, creator: tuple.1, data: tuple.2 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggersByIdCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggersByIdReturn;
            type ReturnTuple<'a> = (
                ISimpleTrigger::TriggerId,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "triggersById(uint64)";
            const SELECTOR: [u8; 4] = [206u8, 40u8, 150u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<ISimpleTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(&self._0),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
    ```solidity
    function unpause() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {}
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `withdrawFees()` and selector `0x476343ee`.
    ```solidity
    function withdrawFees() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawFeesCall {}
    ///Container type for the return parameters of the [`withdrawFees()`](withdrawFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawFeesReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawFeesCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawFeesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawFeesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawFeesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawFeesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawFeesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawFees()";
            const SELECTOR: [u8; 4] = [71u8, 99u8, 67u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`NFTWithTrigger`](self) function calls.
    pub enum NFTWithTriggerCalls {
        CLOCK_MODE(CLOCK_MODECall),
        DEFAULT_ADMIN_ROLE(DEFAULT_ADMIN_ROLECall),
        PAUSER_ROLE(PAUSER_ROLECall),
        SERVICE_PROVIDER_ROLE(SERVICE_PROVIDER_ROLECall),
        addTrigger(addTriggerCall),
        approve(approveCall),
        balanceOf(balanceOfCall),
        burn(burnCall),
        clock(clockCall),
        delegate(delegateCall),
        delegateBySig(delegateBySigCall),
        delegates(delegatesCall),
        eip712Domain(eip712DomainCall),
        getApproved(getApprovedCall),
        getPastTotalSupply(getPastTotalSupplyCall),
        getPastVotes(getPastVotesCall),
        getRoleAdmin(getRoleAdminCall),
        getTrigger(getTriggerCall),
        getTriggerCount(getTriggerCountCall),
        getTriggerIdAtIndex(getTriggerIdAtIndexCall),
        getVotes(getVotesCall),
        grantRole(grantRoleCall),
        handleAddPayload(handleAddPayloadCall),
        hasRole(hasRoleCall),
        initialize(initializeCall),
        initialized(initializedCall),
        isApprovedForAll(isApprovedForAllCall),
        name(nameCall),
        nonces(noncesCall),
        ownerOf(ownerOfCall),
        pause(pauseCall),
        paused(pausedCall),
        renounceRole(renounceRoleCall),
        revokeRole(revokeRoleCall),
        safeTransferFrom_0(safeTransferFrom_0Call),
        safeTransferFrom_1(safeTransferFrom_1Call),
        serviceProvider(serviceProviderCall),
        setApprovalForAll(setApprovalForAllCall),
        supportsInterface(supportsInterfaceCall),
        symbol(symbolCall),
        tokenByIndex(tokenByIndexCall),
        tokenOfOwnerByIndex(tokenOfOwnerByIndexCall),
        tokenURI(tokenURICall),
        totalSupply(totalSupplyCall),
        transferFrom(transferFromCall),
        triggerIdsByCreator(triggerIdsByCreatorCall),
        triggersById(triggersByIdCall),
        unpause(unpauseCall),
        withdrawFees(withdrawFeesCall),
    }
    #[automatically_derived]
    impl NFTWithTriggerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 115u8, 225u8, 215u8],
            [1u8, 255u8, 201u8, 167u8],
            [6u8, 253u8, 222u8, 3u8],
            [8u8, 24u8, 18u8, 252u8],
            [9u8, 94u8, 167u8, 179u8],
            [21u8, 142u8, 249u8, 62u8],
            [24u8, 22u8, 13u8, 221u8],
            [27u8, 198u8, 174u8, 138u8],
            [35u8, 184u8, 114u8, 221u8],
            [36u8, 138u8, 156u8, 163u8],
            [47u8, 47u8, 241u8, 93u8],
            [47u8, 116u8, 92u8, 89u8],
            [51u8, 131u8, 171u8, 254u8],
            [54u8, 86u8, 138u8, 190u8],
            [58u8, 70u8, 177u8, 168u8],
            [63u8, 75u8, 168u8, 58u8],
            [66u8, 132u8, 46u8, 14u8],
            [66u8, 150u8, 108u8, 104u8],
            [71u8, 99u8, 67u8, 238u8],
            [75u8, 245u8, 215u8, 233u8],
            [79u8, 108u8, 204u8, 231u8],
            [88u8, 124u8, 222u8, 30u8],
            [92u8, 25u8, 169u8, 92u8],
            [92u8, 151u8, 90u8, 187u8],
            [99u8, 82u8, 33u8, 30u8],
            [112u8, 160u8, 130u8, 49u8],
            [126u8, 206u8, 190u8, 0u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 105u8, 233u8, 94u8],
            [142u8, 83u8, 158u8, 140u8],
            [145u8, 59u8, 31u8, 191u8],
            [145u8, 209u8, 72u8, 84u8],
            [145u8, 221u8, 173u8, 244u8],
            [149u8, 216u8, 155u8, 65u8],
            [154u8, 178u8, 78u8, 176u8],
            [162u8, 23u8, 253u8, 223u8],
            [162u8, 44u8, 180u8, 101u8],
            [184u8, 141u8, 79u8, 222u8],
            [195u8, 205u8, 165u8, 32u8],
            [196u8, 214u8, 109u8, 232u8],
            [200u8, 123u8, 86u8, 221u8],
            [206u8, 40u8, 150u8, 18u8],
            [213u8, 71u8, 116u8, 31u8],
            [227u8, 30u8, 7u8, 136u8],
            [227u8, 40u8, 237u8, 119u8],
            [230u8, 58u8, 177u8, 233u8],
            [233u8, 133u8, 233u8, 197u8],
            [247u8, 238u8, 148u8, 76u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for NFTWithTriggerCalls {
        const NAME: &'static str = "NFTWithTriggerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 49usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CLOCK_MODE(_) => <CLOCK_MODECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::DEFAULT_ADMIN_ROLE(_) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::PAUSER_ROLE(_) => <PAUSER_ROLECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::SERVICE_PROVIDER_ROLE(_) => {
                    <SERVICE_PROVIDER_ROLECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addTrigger(_) => <addTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::clock(_) => <clockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegate(_) => <delegateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegateBySig(_) => <delegateBySigCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegates(_) => <delegatesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::eip712Domain(_) => <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getApproved(_) => <getApprovedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getPastTotalSupply(_) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPastVotes(_) => <getPastVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getRoleAdmin(_) => <getRoleAdminCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTrigger(_) => <getTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTriggerCount(_) => {
                    <getTriggerCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTriggerIdAtIndex(_) => {
                    <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVotes(_) => <getVotesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::grantRole(_) => <grantRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::handleAddPayload(_) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRole(_) => <hasRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialized(_) => <initializedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isApprovedForAll(_) => {
                    <isApprovedForAllCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonces(_) => <noncesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ownerOf(_) => <ownerOfCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceRole(_) => <renounceRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::revokeRole(_) => <revokeRoleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safeTransferFrom_0(_) => {
                    <safeTransferFrom_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::safeTransferFrom_1(_) => {
                    <safeTransferFrom_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::serviceProvider(_) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setApprovalForAll(_) => {
                    <setApprovalForAllCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::tokenByIndex(_) => <tokenByIndexCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::tokenOfOwnerByIndex(_) => {
                    <tokenOfOwnerByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::tokenURI(_) => <tokenURICall as alloy_sol_types::SolCall>::SELECTOR,
                Self::totalSupply(_) => <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => <transferFromCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::triggerIdsByCreator(_) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggersById(_) => <triggersByIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdrawFees(_) => <withdrawFeesCall as alloy_sol_types::SolCall>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<NFTWithTriggerCalls>] = &[
                {
                    fn handleAddPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::handleAddPayload)
                    }
                    handleAddPayload
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn name(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::name)
                    }
                    name
                },
                {
                    fn getApproved(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getApprovedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getApproved)
                    }
                    getApproved
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::approve)
                    }
                    approve
                },
                {
                    fn initialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <initializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::initialized)
                    }
                    initialized
                },
                {
                    fn totalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::totalSupply)
                    }
                    totalSupply
                },
                {
                    fn SERVICE_PROVIDER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <SERVICE_PROVIDER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::SERVICE_PROVIDER_ROLE)
                    }
                    SERVICE_PROVIDER_ROLE
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::transferFrom)
                    }
                    transferFrom
                },
                {
                    fn getRoleAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getRoleAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getRoleAdmin)
                    }
                    getRoleAdmin
                },
                {
                    fn grantRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <grantRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::grantRole)
                    }
                    grantRole
                },
                {
                    fn tokenOfOwnerByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <tokenOfOwnerByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::tokenOfOwnerByIndex)
                    }
                    tokenOfOwnerByIndex
                },
                {
                    fn getTriggerCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getTriggerCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getTriggerCount)
                    }
                    getTriggerCount
                },
                {
                    fn renounceRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <renounceRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::renounceRole)
                    }
                    renounceRole
                },
                {
                    fn getPastVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getPastVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getPastVotes)
                    }
                    getPastVotes
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::unpause)
                    }
                    unpause
                },
                {
                    fn safeTransferFrom_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <safeTransferFrom_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::safeTransferFrom_0)
                    }
                    safeTransferFrom_0
                },
                {
                    fn burn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::burn)
                    }
                    burn
                },
                {
                    fn withdrawFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <withdrawFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::withdrawFees)
                    }
                    withdrawFees
                },
                {
                    fn CLOCK_MODE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::CLOCK_MODE)
                    }
                    CLOCK_MODE
                },
                {
                    fn tokenByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <tokenByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::tokenByIndex)
                    }
                    tokenByIndex
                },
                {
                    fn delegates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <delegatesCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::delegates)
                    }
                    delegates
                },
                {
                    fn delegate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <delegateCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::delegate)
                    }
                    delegate
                },
                {
                    fn paused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::paused)
                    }
                    paused
                },
                {
                    fn ownerOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <ownerOfCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::ownerOf)
                    }
                    ownerOf
                },
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn nonces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <noncesCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::nonces)
                    }
                    nonces
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::pause)
                    }
                    pause
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn serviceProvider(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <serviceProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::serviceProvider)
                    }
                    serviceProvider
                },
                {
                    fn getPastTotalSupply(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getPastTotalSupply)
                    }
                    getPastTotalSupply
                },
                {
                    fn triggerIdsByCreator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::triggerIdsByCreator)
                    }
                    triggerIdsByCreator
                },
                {
                    fn hasRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <hasRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::hasRole)
                    }
                    hasRole
                },
                {
                    fn clock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <clockCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::clock)
                    }
                    clock
                },
                {
                    fn symbol(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::symbol)
                    }
                    symbol
                },
                {
                    fn getVotes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getVotesCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::getVotes)
                    }
                    getVotes
                },
                {
                    fn DEFAULT_ADMIN_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::DEFAULT_ADMIN_ROLE)
                    }
                    DEFAULT_ADMIN_ROLE
                },
                {
                    fn setApprovalForAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <setApprovalForAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::setApprovalForAll)
                    }
                    setApprovalForAll
                },
                {
                    fn safeTransferFrom_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <safeTransferFrom_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::safeTransferFrom_1)
                    }
                    safeTransferFrom_1
                },
                {
                    fn delegateBySig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <delegateBySigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::delegateBySig)
                    }
                    delegateBySig
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn tokenURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <tokenURICall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::tokenURI)
                    }
                    tokenURI
                },
                {
                    fn triggersById(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <triggersByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::triggersById)
                    }
                    triggersById
                },
                {
                    fn revokeRole(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <revokeRoleCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::revokeRole)
                    }
                    revokeRole
                },
                {
                    fn addTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <addTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::addTrigger)
                    }
                    addTrigger
                },
                {
                    fn getTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerCalls::getTrigger)
                    }
                    getTrigger
                },
                {
                    fn PAUSER_ROLE(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <PAUSER_ROLECall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::PAUSER_ROLE)
                    }
                    PAUSER_ROLE
                },
                {
                    fn isApprovedForAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <isApprovedForAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::isApprovedForAll)
                    }
                    isApprovedForAll
                },
                {
                    fn getTriggerIdAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerCalls> {
                        <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerCalls::getTriggerIdAtIndex)
                    }
                    getTriggerIdAtIndex
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::PAUSER_ROLE(inner) => {
                    <PAUSER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::SERVICE_PROVIDER_ROLE(inner) => {
                    <SERVICE_PROVIDER_ROLECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getApproved(inner) => {
                    <getApprovedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTriggerCount(inner) => {
                    <getTriggerCountCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTriggerIdAtIndex(inner) => {
                    <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::handleAddPayload(inner) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialized(inner) => {
                    <initializedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isApprovedForAll(inner) => {
                    <isApprovedForAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::ownerOf(inner) => {
                    <ownerOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::safeTransferFrom_0(inner) => {
                    <safeTransferFrom_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::safeTransferFrom_1(inner) => {
                    <safeTransferFrom_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setApprovalForAll(inner) => {
                    <setApprovalForAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tokenByIndex(inner) => {
                    <tokenByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tokenOfOwnerByIndex(inner) => {
                    <tokenOfOwnerByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tokenURI(inner) => {
                    <tokenURICall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdrawFees(inner) => {
                    <withdrawFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CLOCK_MODE(inner) => {
                    <CLOCK_MODECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::DEFAULT_ADMIN_ROLE(inner) => {
                    <DEFAULT_ADMIN_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::PAUSER_ROLE(inner) => {
                    <PAUSER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::SERVICE_PROVIDER_ROLE(inner) => {
                    <SERVICE_PROVIDER_ROLECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::clock(inner) => {
                    <clockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegate(inner) => {
                    <delegateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegateBySig(inner) => {
                    <delegateBySigCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegates(inner) => {
                    <delegatesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getApproved(inner) => {
                    <getApprovedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getPastTotalSupply(inner) => {
                    <getPastTotalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getPastVotes(inner) => {
                    <getPastVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getRoleAdmin(inner) => {
                    <getRoleAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTriggerCount(inner) => {
                    <getTriggerCountCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTriggerIdAtIndex(inner) => {
                    <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getVotes(inner) => {
                    <getVotesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::grantRole(inner) => {
                    <grantRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::handleAddPayload(inner) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::hasRole(inner) => {
                    <hasRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialized(inner) => {
                    <initializedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::isApprovedForAll(inner) => {
                    <isApprovedForAllCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::name(inner) => {
                    <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonces(inner) => {
                    <noncesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::ownerOf(inner) => {
                    <ownerOfCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::renounceRole(inner) => {
                    <renounceRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::revokeRole(inner) => {
                    <revokeRoleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::safeTransferFrom_0(inner) => {
                    <safeTransferFrom_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::safeTransferFrom_1(inner) => {
                    <safeTransferFrom_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setApprovalForAll(inner) => {
                    <setApprovalForAllCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::symbol(inner) => {
                    <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::tokenByIndex(inner) => {
                    <tokenByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::tokenOfOwnerByIndex(inner) => {
                    <tokenOfOwnerByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::tokenURI(inner) => {
                    <tokenURICall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalSupply(inner) => {
                    <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::withdrawFees(inner) => {
                    <withdrawFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`NFTWithTrigger`](self) custom errors.
    pub enum NFTWithTriggerErrors {
        AccessControlBadConfirmation(AccessControlBadConfirmation),
        AccessControlUnauthorizedAccount(AccessControlUnauthorizedAccount),
        CheckpointUnorderedInsertion(CheckpointUnorderedInsertion),
        ECDSAInvalidSignature(ECDSAInvalidSignature),
        ECDSAInvalidSignatureLength(ECDSAInvalidSignatureLength),
        ECDSAInvalidSignatureS(ECDSAInvalidSignatureS),
        ERC5805FutureLookup(ERC5805FutureLookup),
        ERC6372InconsistentClock(ERC6372InconsistentClock),
        ERC721EnumerableForbiddenBatchMint(ERC721EnumerableForbiddenBatchMint),
        ERC721IncorrectOwner(ERC721IncorrectOwner),
        ERC721InsufficientApproval(ERC721InsufficientApproval),
        ERC721InvalidApprover(ERC721InvalidApprover),
        ERC721InvalidOperator(ERC721InvalidOperator),
        ERC721InvalidOwner(ERC721InvalidOwner),
        ERC721InvalidReceiver(ERC721InvalidReceiver),
        ERC721InvalidSender(ERC721InvalidSender),
        ERC721NonexistentToken(ERC721NonexistentToken),
        ERC721OutOfBoundsIndex(ERC721OutOfBoundsIndex),
        EnforcedPause(EnforcedPause),
        ExpectedPause(ExpectedPause),
        InvalidAccountNonce(InvalidAccountNonce),
        InvalidShortString(InvalidShortString),
        SafeCastOverflowedUintDowncast(SafeCastOverflowedUintDowncast),
        StringTooLong(StringTooLong),
        VotesExpiredSignature(VotesExpiredSignature),
    }
    #[automatically_derived]
    impl NFTWithTriggerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 126u8, 128u8, 47u8],
            [37u8, 32u8, 96u8, 29u8],
            [48u8, 90u8, 39u8, 169u8],
            [70u8, 131u8, 175u8, 14u8],
            [89u8, 23u8, 31u8, 193u8],
            [91u8, 8u8, 186u8, 24u8],
            [100u8, 40u8, 61u8, 123u8],
            [100u8, 160u8, 174u8, 146u8],
            [102u8, 151u8, 178u8, 50u8],
            [109u8, 252u8, 198u8, 80u8],
            [111u8, 240u8, 113u8, 64u8],
            [115u8, 198u8, 172u8, 110u8],
            [117u8, 45u8, 136u8, 192u8],
            [126u8, 39u8, 50u8, 137u8],
            [137u8, 198u8, 43u8, 100u8],
            [141u8, 252u8, 32u8, 43u8],
            [165u8, 125u8, 19u8, 220u8],
            [169u8, 251u8, 245u8, 31u8],
            [179u8, 81u8, 43u8, 12u8],
            [215u8, 139u8, 206u8, 12u8],
            [217u8, 60u8, 6u8, 101u8],
            [226u8, 81u8, 125u8, 63u8],
            [236u8, 211u8, 248u8, 30u8],
            [246u8, 69u8, 238u8, 223u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for NFTWithTriggerErrors {
        const NAME: &'static str = "NFTWithTriggerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AccessControlBadConfirmation(_) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AccessControlUnauthorizedAccount(_) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointUnorderedInsertion(_) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignature(_) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureLength(_) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECDSAInvalidSignatureS(_) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC5805FutureLookup(_) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC6372InconsistentClock(_) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721EnumerableForbiddenBatchMint(_) => {
                    <ERC721EnumerableForbiddenBatchMint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721IncorrectOwner(_) => {
                    <ERC721IncorrectOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InsufficientApproval(_) => {
                    <ERC721InsufficientApproval as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InvalidApprover(_) => {
                    <ERC721InvalidApprover as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InvalidOperator(_) => {
                    <ERC721InvalidOperator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InvalidOwner(_) => {
                    <ERC721InvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InvalidReceiver(_) => {
                    <ERC721InvalidReceiver as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721InvalidSender(_) => {
                    <ERC721InvalidSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721NonexistentToken(_) => {
                    <ERC721NonexistentToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC721OutOfBoundsIndex(_) => {
                    <ERC721OutOfBoundsIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => <EnforcedPause as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpectedPause(_) => <ExpectedPause as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidAccountNonce(_) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SafeCastOverflowedUintDowncast(_) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
                Self::VotesExpiredSignature(_) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            )
                -> alloy_sol_types::Result<NFTWithTriggerErrors>] = &[
                {
                    fn ERC721InsufficientApproval(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InsufficientApproval as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InsufficientApproval)
                    }
                    ERC721InsufficientApproval
                },
                {
                    fn CheckpointUnorderedInsertion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::CheckpointUnorderedInsertion)
                    }
                    CheckpointUnorderedInsertion
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VotesExpiredSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <VotesExpiredSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::VotesExpiredSignature)
                    }
                    VotesExpiredSignature
                },
                {
                    fn ERC721EnumerableForbiddenBatchMint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721EnumerableForbiddenBatchMint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                NFTWithTriggerErrors::ERC721EnumerableForbiddenBatchMint,
                            )
                    }
                    ERC721EnumerableForbiddenBatchMint
                },
                {
                    fn ERC721InvalidOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InvalidOperator as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InvalidOperator)
                    }
                    ERC721InvalidOperator
                },
                {
                    fn ERC721IncorrectOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721IncorrectOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721IncorrectOwner)
                    }
                    ERC721IncorrectOwner
                },
                {
                    fn ERC721InvalidReceiver(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InvalidReceiver as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InvalidReceiver)
                    }
                    ERC721InvalidReceiver
                },
                {
                    fn AccessControlBadConfirmation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::AccessControlBadConfirmation)
                    }
                    AccessControlBadConfirmation
                },
                {
                    fn SafeCastOverflowedUintDowncast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(NFTWithTriggerErrors::SafeCastOverflowedUintDowncast)
                    }
                    SafeCastOverflowedUintDowncast
                },
                {
                    fn ERC6372InconsistentClock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC6372InconsistentClock)
                    }
                    ERC6372InconsistentClock
                },
                {
                    fn ERC721InvalidSender(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InvalidSender as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InvalidSender)
                    }
                    ERC721InvalidSender
                },
                {
                    fn InvalidAccountNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <InvalidAccountNonce as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::InvalidAccountNonce)
                    }
                    InvalidAccountNonce
                },
                {
                    fn ERC721NonexistentToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721NonexistentToken as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721NonexistentToken)
                    }
                    ERC721NonexistentToken
                },
                {
                    fn ERC721InvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InvalidOwner)
                    }
                    ERC721InvalidOwner
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn ERC721OutOfBoundsIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721OutOfBoundsIndex as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721OutOfBoundsIndex)
                    }
                    ERC721OutOfBoundsIndex
                },
                {
                    fn ERC721InvalidApprover(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC721InvalidApprover as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC721InvalidApprover)
                    }
                    ERC721InvalidApprover
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(NFTWithTriggerErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn AccessControlUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(NFTWithTriggerErrors::AccessControlUnauthorizedAccount)
                    }
                    AccessControlUnauthorizedAccount
                },
                {
                    fn ERC5805FutureLookup(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ERC5805FutureLookup)
                    }
                    ERC5805FutureLookup
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<NFTWithTriggerErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(NFTWithTriggerErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721EnumerableForbiddenBatchMint(inner) => {
                    <ERC721EnumerableForbiddenBatchMint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721IncorrectOwner(inner) => {
                    <ERC721IncorrectOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InsufficientApproval(inner) => {
                    <ERC721InsufficientApproval as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InvalidApprover(inner) => {
                    <ERC721InvalidApprover as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InvalidOperator(inner) => {
                    <ERC721InvalidOperator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InvalidOwner(inner) => {
                    <ERC721InvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InvalidReceiver(inner) => {
                    <ERC721InvalidReceiver as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721InvalidSender(inner) => {
                    <ERC721InvalidSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721NonexistentToken(inner) => {
                    <ERC721NonexistentToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ERC721OutOfBoundsIndex(inner) => {
                    <ERC721OutOfBoundsIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AccessControlBadConfirmation(inner) => {
                    <AccessControlBadConfirmation as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AccessControlUnauthorizedAccount(inner) => {
                    <AccessControlUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointUnorderedInsertion(inner) => {
                    <CheckpointUnorderedInsertion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignature(inner) => {
                    <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureLength(inner) => {
                    <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSAInvalidSignatureS(inner) => {
                    <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC5805FutureLookup(inner) => {
                    <ERC5805FutureLookup as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC6372InconsistentClock(inner) => {
                    <ERC6372InconsistentClock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721EnumerableForbiddenBatchMint(inner) => {
                    <ERC721EnumerableForbiddenBatchMint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721IncorrectOwner(inner) => {
                    <ERC721IncorrectOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InsufficientApproval(inner) => {
                    <ERC721InsufficientApproval as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InvalidApprover(inner) => {
                    <ERC721InvalidApprover as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InvalidOperator(inner) => {
                    <ERC721InvalidOperator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InvalidOwner(inner) => {
                    <ERC721InvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InvalidReceiver(inner) => {
                    <ERC721InvalidReceiver as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721InvalidSender(inner) => {
                    <ERC721InvalidSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721NonexistentToken(inner) => {
                    <ERC721NonexistentToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ERC721OutOfBoundsIndex(inner) => {
                    <ERC721OutOfBoundsIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAccountNonce(inner) => {
                    <InvalidAccountNonce as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SafeCastOverflowedUintDowncast(inner) => {
                    <SafeCastOverflowedUintDowncast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::VotesExpiredSignature(inner) => {
                    <VotesExpiredSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`NFTWithTrigger`](self) events.
    pub enum NFTWithTriggerEvents {
        Approval(Approval),
        ApprovalForAll(ApprovalForAll),
        BatchMetadataUpdate(BatchMetadataUpdate),
        Debug(Debug),
        DelegateChanged(DelegateChanged),
        DelegateVotesChanged(DelegateVotesChanged),
        EIP712DomainChanged(EIP712DomainChanged),
        MetadataUpdate(MetadataUpdate),
        NFTMinted(NFTMinted),
        NewTrigger(NewTrigger),
        Paused(Paused),
        RoleAdminChanged(RoleAdminChanged),
        RoleGranted(RoleGranted),
        RoleRevoked(RoleRevoked),
        Transfer(Transfer),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl NFTWithTriggerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8, 180u8,
                243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8, 95u8, 155u8, 243u8,
                120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                23u8, 48u8, 126u8, 171u8, 57u8, 171u8, 97u8, 7u8, 232u8, 137u8, 152u8, 69u8, 173u8,
                61u8, 89u8, 189u8, 150u8, 83u8, 242u8, 0u8, 242u8, 32u8, 146u8, 4u8, 137u8, 202u8,
                43u8, 89u8, 55u8, 105u8, 108u8, 49u8,
            ],
            [
                47u8, 135u8, 136u8, 17u8, 126u8, 126u8, 255u8, 29u8, 130u8, 233u8, 38u8, 236u8,
                121u8, 73u8, 1u8, 209u8, 124u8, 120u8, 2u8, 74u8, 80u8, 39u8, 9u8, 64u8, 48u8,
                69u8, 64u8, 167u8, 51u8, 101u8, 111u8, 13u8,
            ],
            [
                49u8, 52u8, 232u8, 162u8, 230u8, 217u8, 126u8, 146u8, 154u8, 126u8, 84u8, 1u8,
                30u8, 165u8, 72u8, 93u8, 125u8, 25u8, 109u8, 213u8, 240u8, 186u8, 77u8, 78u8,
                249u8, 88u8, 3u8, 232u8, 227u8, 252u8, 37u8, 127u8,
            ],
            [
                60u8, 90u8, 209u8, 71u8, 16u8, 78u8, 86u8, 190u8, 52u8, 169u8, 23u8, 106u8, 102u8,
                146u8, 247u8, 223u8, 141u8, 47u8, 75u8, 41u8, 165u8, 175u8, 6u8, 188u8, 107u8,
                152u8, 151u8, 13u8, 50u8, 157u8, 101u8, 119u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8, 167u8,
                131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8, 78u8, 83u8,
                123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8, 2u8, 112u8,
                181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8, 71u8, 84u8, 235u8,
                219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                107u8, 213u8, 201u8, 80u8, 168u8, 216u8, 223u8, 23u8, 247u8, 114u8, 245u8, 175u8,
                55u8, 203u8, 54u8, 85u8, 115u8, 120u8, 153u8, 203u8, 249u8, 3u8, 38u8, 75u8, 151u8,
                149u8, 89u8, 45u8, 164u8, 57u8, 102u8, 28u8,
            ],
            [
                140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8, 113u8, 66u8,
                125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8, 247u8, 178u8, 41u8, 30u8, 91u8,
                32u8, 10u8, 200u8, 199u8, 195u8, 185u8, 37u8,
            ],
            [
                189u8, 121u8, 184u8, 111u8, 254u8, 10u8, 184u8, 232u8, 119u8, 97u8, 81u8, 81u8,
                66u8, 23u8, 205u8, 124u8, 172u8, 213u8, 44u8, 144u8, 159u8, 102u8, 71u8, 92u8,
                58u8, 244u8, 78u8, 18u8, 159u8, 11u8, 0u8, 255u8,
            ],
            [
                211u8, 91u8, 185u8, 94u8, 9u8, 192u8, 75u8, 33u8, 158u8, 53u8, 4u8, 124u8, 231u8,
                183u8, 179u8, 0u8, 227u8, 56u8, 66u8, 100u8, 239u8, 132u8, 164u8, 4u8, 86u8, 148u8,
                61u8, 188u8, 15u8, 193u8, 124u8, 20u8,
            ],
            [
                221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8, 176u8, 104u8,
                252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8, 99u8, 196u8, 161u8, 22u8,
                40u8, 245u8, 90u8, 77u8, 245u8, 35u8, 179u8, 239u8,
            ],
            [
                222u8, 194u8, 186u8, 205u8, 210u8, 240u8, 91u8, 89u8, 222u8, 52u8, 218u8, 155u8,
                82u8, 61u8, 255u8, 139u8, 228u8, 46u8, 94u8, 56u8, 232u8, 24u8, 200u8, 47u8, 219u8,
                11u8, 174u8, 119u8, 67u8, 135u8, 167u8, 36u8,
            ],
            [
                243u8, 244u8, 17u8, 216u8, 83u8, 72u8, 107u8, 159u8, 83u8, 218u8, 99u8, 0u8, 154u8,
                33u8, 205u8, 40u8, 78u8, 161u8, 138u8, 128u8, 13u8, 77u8, 229u8, 92u8, 229u8,
                189u8, 147u8, 93u8, 25u8, 126u8, 76u8, 241u8,
            ],
            [
                246u8, 57u8, 31u8, 92u8, 50u8, 217u8, 198u8, 157u8, 42u8, 71u8, 234u8, 103u8, 11u8,
                68u8, 41u8, 116u8, 181u8, 57u8, 53u8, 209u8, 237u8, 199u8, 253u8, 100u8, 235u8,
                33u8, 224u8, 71u8, 168u8, 57u8, 23u8, 27u8,
            ],
            [
                248u8, 225u8, 161u8, 90u8, 186u8, 147u8, 152u8, 224u8, 25u8, 240u8, 180u8, 157u8,
                241u8, 164u8, 253u8, 233u8, 142u8, 225u8, 122u8, 227u8, 69u8, 203u8, 95u8, 107u8,
                94u8, 44u8, 39u8, 245u8, 3u8, 62u8, 140u8, 231u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for NFTWithTriggerEvents {
        const NAME: &'static str = "NFTWithTriggerEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Approval)
                }
                Some(<ApprovalForAll as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ApprovalForAll as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ApprovalForAll)
                }
                Some(<BatchMetadataUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <BatchMetadataUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::BatchMetadataUpdate)
                }
                Some(<Debug as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Debug as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Debug)
                }
                Some(<DelegateChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::DelegateChanged)
                }
                Some(<DelegateVotesChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DelegateVotesChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::DelegateVotesChanged)
                }
                Some(<EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EIP712DomainChanged)
                }
                Some(<MetadataUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MetadataUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MetadataUpdate)
                }
                Some(<NFTMinted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NFTMinted as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::NFTMinted)
                }
                Some(<NewTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTrigger)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Paused)
                }
                Some(<RoleAdminChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleAdminChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RoleAdminChanged)
                }
                Some(<RoleGranted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleGranted as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RoleGranted)
                }
                Some(<RoleRevoked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RoleRevoked as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RoleRevoked)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Transfer)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::Unpaused)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for NFTWithTriggerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::ApprovalForAll(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::BatchMetadataUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Debug(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MetadataUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NFTMinted(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ApprovalForAll(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::BatchMetadataUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Debug(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MetadataUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NFTMinted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::RoleAdminChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleGranted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RoleRevoked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`NFTWithTrigger`](self) contract instance.

    See the [wrapper's documentation](`NFTWithTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> NFTWithTriggerInstance<T, P, N> {
        NFTWithTriggerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<NFTWithTriggerInstance<T, P, N>>>
    {
        NFTWithTriggerInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        NFTWithTriggerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`NFTWithTrigger`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`NFTWithTrigger`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct NFTWithTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for NFTWithTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("NFTWithTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTWithTriggerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`NFTWithTrigger`](self) contract instance.

        See the [wrapper's documentation](`NFTWithTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<NFTWithTriggerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> NFTWithTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> NFTWithTriggerInstance<T, P, N> {
            NFTWithTriggerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTWithTriggerInstance<T, P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`CLOCK_MODE`] function.
        pub fn CLOCK_MODE(&self) -> alloy_contract::SolCallBuilder<T, &P, CLOCK_MODECall, N> {
            self.call_builder(&CLOCK_MODECall {})
        }
        ///Creates a new call builder for the [`DEFAULT_ADMIN_ROLE`] function.
        pub fn DEFAULT_ADMIN_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEFAULT_ADMIN_ROLECall, N> {
            self.call_builder(&DEFAULT_ADMIN_ROLECall {})
        }
        ///Creates a new call builder for the [`PAUSER_ROLE`] function.
        pub fn PAUSER_ROLE(&self) -> alloy_contract::SolCallBuilder<T, &P, PAUSER_ROLECall, N> {
            self.call_builder(&PAUSER_ROLECall {})
        }
        ///Creates a new call builder for the [`SERVICE_PROVIDER_ROLE`] function.
        pub fn SERVICE_PROVIDER_ROLE(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SERVICE_PROVIDER_ROLECall, N> {
            self.call_builder(&SERVICE_PROVIDER_ROLECall {})
        }
        ///Creates a new call builder for the [`addTrigger`] function.
        pub fn addTrigger(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, addTriggerCall, N> {
            self.call_builder(&addTriggerCall { data })
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            to: alloy::sol_types::private::Address,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { to, tokenId })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { owner })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnCall, N> {
            self.call_builder(&burnCall { tokenId })
        }
        ///Creates a new call builder for the [`clock`] function.
        pub fn clock(&self) -> alloy_contract::SolCallBuilder<T, &P, clockCall, N> {
            self.call_builder(&clockCall {})
        }
        ///Creates a new call builder for the [`delegate`] function.
        pub fn delegate(
            &self,
            delegatee: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateCall, N> {
            self.call_builder(&delegateCall { delegatee })
        }
        ///Creates a new call builder for the [`delegateBySig`] function.
        pub fn delegateBySig(
            &self,
            delegatee: alloy::sol_types::private::Address,
            nonce: alloy::sol_types::private::primitives::aliases::U256,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
            v: u8,
            r: alloy::sol_types::private::FixedBytes<32>,
            s: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegateBySigCall, N> {
            self.call_builder(&delegateBySigCall { delegatee, nonce, expiry, v, r, s })
        }
        ///Creates a new call builder for the [`delegates`] function.
        pub fn delegates(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegatesCall, N> {
            self.call_builder(&delegatesCall { account })
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(&self) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getApproved`] function.
        pub fn getApproved(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getApprovedCall, N> {
            self.call_builder(&getApprovedCall { tokenId })
        }
        ///Creates a new call builder for the [`getPastTotalSupply`] function.
        pub fn getPastTotalSupply(
            &self,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastTotalSupplyCall, N> {
            self.call_builder(&getPastTotalSupplyCall { timepoint })
        }
        ///Creates a new call builder for the [`getPastVotes`] function.
        pub fn getPastVotes(
            &self,
            account: alloy::sol_types::private::Address,
            timepoint: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPastVotesCall, N> {
            self.call_builder(&getPastVotesCall { account, timepoint })
        }
        ///Creates a new call builder for the [`getRoleAdmin`] function.
        pub fn getRoleAdmin(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRoleAdminCall, N> {
            self.call_builder(&getRoleAdminCall { role })
        }
        ///Creates a new call builder for the [`getTrigger`] function.
        pub fn getTrigger(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerCall, N> {
            self.call_builder(&getTriggerCall { triggerId })
        }
        ///Creates a new call builder for the [`getTriggerCount`] function.
        pub fn getTriggerCount(
            &self,
            creator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerCountCall, N> {
            self.call_builder(&getTriggerCountCall { creator })
        }
        ///Creates a new call builder for the [`getTriggerIdAtIndex`] function.
        pub fn getTriggerIdAtIndex(
            &self,
            creator: alloy::sol_types::private::Address,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerIdAtIndexCall, N> {
            self.call_builder(&getTriggerIdAtIndexCall { creator, index })
        }
        ///Creates a new call builder for the [`getVotes`] function.
        pub fn getVotes(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVotesCall, N> {
            self.call_builder(&getVotesCall { account })
        }
        ///Creates a new call builder for the [`grantRole`] function.
        pub fn grantRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, grantRoleCall, N> {
            self.call_builder(&grantRoleCall { role, account })
        }
        ///Creates a new call builder for the [`handleAddPayload`] function.
        pub fn handleAddPayload(
            &self,
            data: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleAddPayloadCall, N> {
            self.call_builder(&handleAddPayloadCall { data, signature })
        }
        ///Creates a new call builder for the [`hasRole`] function.
        pub fn hasRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRoleCall, N> {
            self.call_builder(&hasRoleCall { role, account })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceProvider: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _serviceProvider })
        }
        ///Creates a new call builder for the [`initialized`] function.
        pub fn initialized(&self) -> alloy_contract::SolCallBuilder<T, &P, initializedCall, N> {
            self.call_builder(&initializedCall {})
        }
        ///Creates a new call builder for the [`isApprovedForAll`] function.
        pub fn isApprovedForAll(
            &self,
            owner: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isApprovedForAllCall, N> {
            self.call_builder(&isApprovedForAllCall { owner, operator })
        }
        ///Creates a new call builder for the [`name`] function.
        pub fn name(&self) -> alloy_contract::SolCallBuilder<T, &P, nameCall, N> {
            self.call_builder(&nameCall {})
        }
        ///Creates a new call builder for the [`nonces`] function.
        pub fn nonces(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, noncesCall, N> {
            self.call_builder(&noncesCall { owner })
        }
        ///Creates a new call builder for the [`ownerOf`] function.
        pub fn ownerOf(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, ownerOfCall, N> {
            self.call_builder(&ownerOfCall { tokenId })
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<T, &P, pauseCall, N> {
            self.call_builder(&pauseCall {})
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<T, &P, pausedCall, N> {
            self.call_builder(&pausedCall {})
        }
        ///Creates a new call builder for the [`renounceRole`] function.
        pub fn renounceRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            callerConfirmation: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceRoleCall, N> {
            self.call_builder(&renounceRoleCall { role, callerConfirmation })
        }
        ///Creates a new call builder for the [`revokeRole`] function.
        pub fn revokeRole(
            &self,
            role: alloy::sol_types::private::FixedBytes<32>,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, revokeRoleCall, N> {
            self.call_builder(&revokeRoleCall { role, account })
        }
        ///Creates a new call builder for the [`safeTransferFrom_0`] function.
        pub fn safeTransferFrom_0(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, safeTransferFrom_0Call, N> {
            self.call_builder(&safeTransferFrom_0Call { from, to, tokenId })
        }
        ///Creates a new call builder for the [`safeTransferFrom_1`] function.
        pub fn safeTransferFrom_1(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, safeTransferFrom_1Call, N> {
            self.call_builder(&safeTransferFrom_1Call { from, to, tokenId, data })
        }
        ///Creates a new call builder for the [`serviceProvider`] function.
        pub fn serviceProvider(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceProviderCall, N> {
            self.call_builder(&serviceProviderCall {})
        }
        ///Creates a new call builder for the [`setApprovalForAll`] function.
        pub fn setApprovalForAll(
            &self,
            operator: alloy::sol_types::private::Address,
            approved: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setApprovalForAllCall, N> {
            self.call_builder(&setApprovalForAllCall { operator, approved })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(&supportsInterfaceCall { interfaceId })
        }
        ///Creates a new call builder for the [`symbol`] function.
        pub fn symbol(&self) -> alloy_contract::SolCallBuilder<T, &P, symbolCall, N> {
            self.call_builder(&symbolCall {})
        }
        ///Creates a new call builder for the [`tokenByIndex`] function.
        pub fn tokenByIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenByIndexCall, N> {
            self.call_builder(&tokenByIndexCall { index })
        }
        ///Creates a new call builder for the [`tokenOfOwnerByIndex`] function.
        pub fn tokenOfOwnerByIndex(
            &self,
            owner: alloy::sol_types::private::Address,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenOfOwnerByIndexCall, N> {
            self.call_builder(&tokenOfOwnerByIndexCall { owner, index })
        }
        ///Creates a new call builder for the [`tokenURI`] function.
        pub fn tokenURI(
            &self,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, tokenURICall, N> {
            self.call_builder(&tokenURICall { tokenId })
        }
        ///Creates a new call builder for the [`totalSupply`] function.
        pub fn totalSupply(&self) -> alloy_contract::SolCallBuilder<T, &P, totalSupplyCall, N> {
            self.call_builder(&totalSupplyCall {})
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            from: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            tokenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
            self.call_builder(&transferFromCall { from, to, tokenId })
        }
        ///Creates a new call builder for the [`triggerIdsByCreator`] function.
        pub fn triggerIdsByCreator(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggerIdsByCreatorCall, N> {
            self.call_builder(&triggerIdsByCreatorCall { _0, _1 })
        }
        ///Creates a new call builder for the [`triggersById`] function.
        pub fn triggersById(
            &self,
            _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, triggersByIdCall, N> {
            self.call_builder(&triggersByIdCall { _0 })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall {})
        }
        ///Creates a new call builder for the [`withdrawFees`] function.
        pub fn withdrawFees(&self) -> alloy_contract::SolCallBuilder<T, &P, withdrawFeesCall, N> {
            self.call_builder(&withdrawFeesCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > NFTWithTriggerInstance<T, P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`ApprovalForAll`] event.
        pub fn ApprovalForAll_filter(&self) -> alloy_contract::Event<T, &P, ApprovalForAll, N> {
            self.event_filter::<ApprovalForAll>()
        }
        ///Creates a new event filter for the [`BatchMetadataUpdate`] event.
        pub fn BatchMetadataUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, BatchMetadataUpdate, N> {
            self.event_filter::<BatchMetadataUpdate>()
        }
        ///Creates a new event filter for the [`Debug`] event.
        pub fn Debug_filter(&self) -> alloy_contract::Event<T, &P, Debug, N> {
            self.event_filter::<Debug>()
        }
        ///Creates a new event filter for the [`DelegateChanged`] event.
        pub fn DelegateChanged_filter(&self) -> alloy_contract::Event<T, &P, DelegateChanged, N> {
            self.event_filter::<DelegateChanged>()
        }
        ///Creates a new event filter for the [`DelegateVotesChanged`] event.
        pub fn DelegateVotesChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelegateVotesChanged, N> {
            self.event_filter::<DelegateVotesChanged>()
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`MetadataUpdate`] event.
        pub fn MetadataUpdate_filter(&self) -> alloy_contract::Event<T, &P, MetadataUpdate, N> {
            self.event_filter::<MetadataUpdate>()
        }
        ///Creates a new event filter for the [`NFTMinted`] event.
        pub fn NFTMinted_filter(&self) -> alloy_contract::Event<T, &P, NFTMinted, N> {
            self.event_filter::<NFTMinted>()
        }
        ///Creates a new event filter for the [`NewTrigger`] event.
        pub fn NewTrigger_filter(&self) -> alloy_contract::Event<T, &P, NewTrigger, N> {
            self.event_filter::<NewTrigger>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<T, &P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`RoleAdminChanged`] event.
        pub fn RoleAdminChanged_filter(&self) -> alloy_contract::Event<T, &P, RoleAdminChanged, N> {
            self.event_filter::<RoleAdminChanged>()
        }
        ///Creates a new event filter for the [`RoleGranted`] event.
        pub fn RoleGranted_filter(&self) -> alloy_contract::Event<T, &P, RoleGranted, N> {
            self.event_filter::<RoleGranted>()
        }
        ///Creates a new event filter for the [`RoleRevoked`] event.
        pub fn RoleRevoked_filter(&self) -> alloy_contract::Event<T, &P, RoleRevoked, N> {
            self.event_filter::<RoleRevoked>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
