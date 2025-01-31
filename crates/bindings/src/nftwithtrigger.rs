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
    event DelegateChanged(address indexed delegator, address indexed fromDelegate, address indexed toDelegate);
    event DelegateVotesChanged(address indexed delegate, uint256 previousVotes, uint256 newVotes);
    event EIP712DomainChanged();
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
    ///0x610160604052348015610010575f5ffd5b50604080518082018252600a80825269151c9a59d9d95c93919560b21b60208084018290528451808601865260018152603160f81b818301528551808701875293845283820192909252845180860190955260048552631513919560e21b9085015291925f61007f8382610379565b50600161008c8282610379565b5050600a805460ff19169055506100a482600c6101be565b610120526100b381600d6101be565b61014052815160208084019190912060e052815190820120610100524660a05261013f60e05161010051604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201529081019290925260608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b60805250503060c0526101525f336101f0565b5061017d7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a336101f0565b506101a87fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b336101f0565b50601680546001600160a81b03191690556104a1565b5f6020835110156101d9576101d28361029b565b90506101ea565b816101e48482610379565b5060ff90505b92915050565b5f828152600b602090815260408083206001600160a01b038516845290915281205460ff16610294575f838152600b602090815260408083206001600160a01b03861684529091529020805460ff1916600117905561024c3390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a45060016101ea565b505f6101ea565b5f5f829050601f815111156102ce578260405163305a27a960e01b81526004016102c59190610433565b60405180910390fd5b80516102d98261047e565b179392505050565b634e487b7160e01b5f52604160045260245ffd5b600181811c9082168061030957607f821691505b60208210810361032757634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561037457805f5260205f20601f840160051c810160208510156103525750805b601f840160051c820191505b81811015610371575f815560010161035e565b50505b505050565b81516001600160401b03811115610392576103926102e1565b6103a6816103a084546102f5565b8461032d565b6020601f8211600181146103d8575f83156103c15750848201515b5f19600385901b1c1916600184901b178455610371565b5f84815260208120601f198516915b8281101561040757878501518255602094850194600190920191016103e7565b508482101561042457868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602081525f82518060208401525f5b8181101561045f5760208186018101516040868401015201610442565b505f604082850101526040601f19601f83011684010191505092915050565b80516020808301519190811015610327575f1960209190910360031b1b16919050565b60805160a05160c05160e0516101005161012051610140516136476104f25f395f611a6a01525f611a3801525f61234101525f61231901525f61227401525f61229e01525f6122c801526136475ff3fe60806040526004361061028a575f3560e01c806370a0823111610155578063a22cb465116100be578063d547741f11610078578063d547741f1461084e578063e31e07881461086d578063e328ed7714610880578063e63ab1e9146108ac578063e985e9c5146108df578063f7ee944c146108fe575f5ffd5b8063a22cb46514610785578063b88d4fde146107a4578063c3cda520146107c3578063c4d66de8146107e2578063c87b56dd14610801578063ce28961214610820575f5ffd5b8063913b1fbf1161010f578063913b1fbf146106c757806391d14854146106fe57806391ddadf41461071d57806395d89b411461073f5780639ab24eb014610753578063a217fddf14610772575f5ffd5b806370a08231146105fb5780637ecebe001461061a5780638456cb591461064e57806384b0196e146106625780638d69e95e146106895780638e539e8c146106a8575f5ffd5b80633383abfe116101f75780634bf5d7e9116101b15780634bf5d7e91461051a5780634f6ccce714610550578063587cde1e1461056f5780635c19a95c146105a65780635c975abb146105c55780636352211e146105dc575f5ffd5b80633383abfe1461045657806336568abe1461048a5780633a46b1a8146104a95780633f4ba83a146104c857806342842e0e146104dc57806342966c68146104fb575f5ffd5b806318160ddd1161024857806318160ddd1461037a5780631bc6ae8a1461039857806323b872dd146103cb578063248a9ca3146103ea5780632f2ff15d146104185780632f745c5914610437575f5ffd5b806273e1d71461028e57806301ffc9a7146102af57806306fdde03146102e3578063081812fc14610304578063095ea7b31461033b578063158ef93e1461035a575b5f5ffd5b348015610299575f5ffd5b506102ad6102a8366004612dfb565b61091d565b005b3480156102ba575f5ffd5b506102ce6102c9366004612e7a565b610a61565b60405190151581526020015b60405180910390f35b3480156102ee575f5ffd5b506102f7610a71565b6040516102da9190612ed8565b34801561030f575f5ffd5b5061032361031e366004612eea565b610b00565b6040516001600160a01b0390911681526020016102da565b348015610346575f5ffd5b506102ad610355366004612f15565b610b27565b348015610365575f5ffd5b506016546102ce90600160a01b900460ff1681565b348015610385575f5ffd5b506008545b6040519081526020016102da565b3480156103a3575f5ffd5b5061038a7fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b81565b3480156103d6575f5ffd5b506102ad6103e5366004612f3f565b610b36565b3480156103f5575f5ffd5b5061038a610404366004612eea565b5f908152600b602052604090206001015490565b348015610423575f5ffd5b506102ad610432366004612f7d565b610bbf565b348015610442575f5ffd5b5061038a610451366004612f15565b610be3565b348015610461575f5ffd5b5061038a610470366004612fab565b6001600160a01b03165f9081526013602052604090205490565b348015610495575f5ffd5b506102ad6104a4366004612f7d565b610c46565b3480156104b4575f5ffd5b5061038a6104c3366004612f15565b610c7e565b3480156104d3575f5ffd5b506102ad610cf3565b3480156104e7575f5ffd5b506102ad6104f6366004612f3f565b610d28565b348015610506575f5ffd5b506102ad610515366004612eea565b610d42565b348015610525575f5ffd5b5060408051808201909152600e81526d06d6f64653d74696d657374616d760941b60208201526102f7565b34801561055b575f5ffd5b5061038a61056a366004612eea565b610d4d565b34801561057a575f5ffd5b50610323610589366004612fab565b6001600160a01b039081165f908152600f60205260409020541690565b3480156105b1575f5ffd5b506102ad6105c0366004612fab565b610da2565b3480156105d0575f5ffd5b50600a5460ff166102ce565b3480156105e7575f5ffd5b506103236105f6366004612eea565b610dad565b348015610606575f5ffd5b5061038a610615366004612fab565b610db7565b348015610625575f5ffd5b5061038a610634366004612fab565b6001600160a01b03165f908152600e602052604090205490565b348015610659575f5ffd5b506102ad610dfc565b34801561066d575f5ffd5b50610676610e2e565b6040516102da9796959493929190612fc6565b348015610694575f5ffd5b50601654610323906001600160a01b031681565b3480156106b3575f5ffd5b5061038a6106c2366004612eea565b610e70565b3480156106d2575f5ffd5b506106e66106e1366004612f15565b610ecf565b6040516001600160401b0390911681526020016102da565b348015610709575f5ffd5b506102ce610718366004612f7d565b610f16565b348015610728575f5ffd5b5060405165ffffffffffff421681526020016102da565b34801561074a575f5ffd5b506102f7610f40565b34801561075e575f5ffd5b5061038a61076d366004612fab565b610f4f565b34801561077d575f5ffd5b5061038a5f81565b348015610790575f5ffd5b506102ad61079f36600461305c565b610f7e565b3480156107af575f5ffd5b506102ad6107be366004613133565b610f89565b3480156107ce575f5ffd5b506102ad6107dd36600461319a565b610fa0565b3480156107ed575f5ffd5b506102ad6107fc366004612fab565b61105c565b34801561080c575f5ffd5b506102f761081b366004612eea565b61115f565b34801561082b575f5ffd5b5061083f61083a3660046131f7565b611260565b6040516102da9392919061321d565b348015610859575f5ffd5b506102ad610868366004612f7d565b61131e565b6102ad61087b366004613258565b611342565b34801561088b575f5ffd5b5061089f61089a3660046131f7565b6114d5565b6040516102da9190613289565b3480156108b7575f5ffd5b5061038a7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a81565b3480156108ea575f5ffd5b506102ce6108f93660046132c8565b6115ce565b348015610909575f5ffd5b506106e6610918366004612f15565b6115fb565b6109477fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b33610f16565b6109905760405162461bcd60e51b815260206004820152601560248201527427b7363c9039b2b93b34b1b290383937bb34b232b960591b60448201526064015b60405180910390fd5b5f808061099f868801886132f4565b9250925092505f60155f8154809291906109b89061336f565b9190505590506109c884826116af565b5f8181526017602052604090206109df8382613403565b506001600160401b0383165f90815260126020526040812080546001600160e01b031916815590610a136001830182612d71565b505080846001600160a01b03167fd35bb95e09c04b219e35047ce7b7b300e3384264ef84a40456943dbc0fc17c1484604051610a4f9190612ed8565b60405180910390a35050505050505050565b5f610a6b826116c8565b92915050565b60605f8054610a7f90613387565b80601f0160208091040260200160405190810160405280929190818152602001828054610aab90613387565b8015610af65780601f10610acd57610100808354040283529160200191610af6565b820191905f5260205f20905b815481529060010190602001808311610ad957829003601f168201915b5050505050905090565b5f610b0a826116ec565b505f828152600460205260409020546001600160a01b0316610a6b565b610b32828233611724565b5050565b6001600160a01b038216610b5f57604051633250574960e11b81525f6004820152602401610987565b5f610b6b838333611731565b9050836001600160a01b0316816001600160a01b031614610bb9576040516364283d7b60e01b81526001600160a01b0380861660048301526024820184905282166044820152606401610987565b50505050565b5f828152600b6020526040902060010154610bd981611745565b610bb9838361174f565b5f610bed83610db7565b8210610c1e5760405163295f44f760e21b81526001600160a01b038416600482015260248101839052604401610987565b506001600160a01b03919091165f908152600660209081526040808320938352929052205490565b6001600160a01b0381163314610c6f5760405163334bd91960e11b815260040160405180910390fd5b610c7982826117e0565b505050565b5f4265ffffffffffff81168310610cb957604051637669fc0f60e11b81526004810184905265ffffffffffff82166024820152604401610987565b610ce2610cc58461184b565b6001600160a01b0386165f90815260106020526040902090611881565b6001600160d01b0316949350505050565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610d1d81611745565b610d25611931565b50565b610c7983838360405180602001604052805f815250610f89565b610b325f8233611731565b5f610d5760085490565b8210610d7f5760405163295f44f760e21b81525f600482015260248101839052604401610987565b60088281548110610d9257610d926134bd565b905f5260205f2001549050919050565b33610b328183611983565b5f610a6b826116ec565b5f6001600160a01b038216610de1576040516322718ad960e21b81525f6004820152602401610987565b506001600160a01b03165f9081526003602052604090205490565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610e2681611745565b610d256119f4565b5f6060805f5f5f6060610e3f611a31565b610e47611a63565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f4265ffffffffffff81168310610eab57604051637669fc0f60e11b81526004810184905265ffffffffffff82166024820152604401610987565b610ebf610eb78461184b565b601190611881565b6001600160d01b03169392505050565b6013602052815f5260405f208181548110610ee8575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f918252600b602090815260408084206001600160a01b0393909316845291905290205460ff1690565b606060018054610a7f90613387565b6001600160a01b0381165f908152601060205260408120610f6f90611a90565b6001600160d01b031692915050565b610b32338383611ac8565b610f94848484610b36565b610bb984848484611b5e565b83421115610fc457604051632341d78760e11b815260048101859052602401610987565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f9061103d906110359060a00160405160208183030381529060405280519060200120611c84565b858585611cb0565b90506110498187611cdc565b6110538188611983565b50505050505050565b5f61106681611745565b601654600160a01b900460ff16156110b65760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610987565b6001600160a01b03821661110c5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420736572766963652070726f766964657200000000000000006044820152606401610987565b6111367fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b8361174f565b5050601680546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b5f818152600260205260409020546060906001600160a01b03166111c55760405162461bcd60e51b815260206004820152601f60248201527f55524920717565727920666f72206e6f6e6578697374656e7420746f6b656e006044820152606401610987565b5f82815260176020526040902080546111dd90613387565b80601f016020809104026020016040519081016040528092919081815260200182805461120990613387565b80156112545780601f1061122b57610100808354040283529160200191611254565b820191905f5260205f20905b81548152906001019060200180831161123757829003601f168201915b50505050509050919050565b60126020525f9081526040902080546001820180546001600160401b03831693600160401b9093046001600160a01b031692919061129d90613387565b80601f01602080910402602001604051908101604052809291908181526020018280546112c990613387565b80156113145780601f106112eb57610100808354040283529160200191611314565b820191905f5260205f20905b8154815290600101906020018083116112f757829003601f168201915b5050505050905083565b5f828152600b602052604090206001015461133881611745565b610bb983836117e0565b3467016345785d8a0000146113995760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e3120455448006044820152606401610987565b601480545f916001600160401b0390911690826113b5836134d1565b82546001600160401b039182166101009390930a928302928202191691909117909155604080516060810182528383168082523360208084019182528385018981525f9384526012909152939091208251815492516001600160a01b0316600160401b026001600160e01b031990931695169490941717835590519293509182919060018201906114469082613403565b5050335f818152601360209081526040808320805460018101825590845291909220600482040180546001600160401b0380891660086003909516949094026101000a8481029102199091161790559051919250907ff3f411d853486b9f53da63009a21cd284ea18a800d4de55ce5bd935d197e4cf1906114c8908790612ed8565b60405180910390a3505050565b60408051606080820183525f8083526020830152918101919091526001600160401b038281165f90815260126020908152604091829020825160608101845281549485168152600160401b9094046001600160a01b031691840191909152600181018054919284019161154790613387565b80601f016020809104026020016040519081016040528092919081815260200182805461157390613387565b80156115be5780601f10611595576101008083540402835291602001916115be565b820191905f5260205f20905b8154815290600101906020018083116115a157829003601f168201915b5050505050815250509050919050565b6001600160a01b039182165f90815260056020908152604080832093909416825291909152205460ff1690565b6001600160a01b0382165f9081526013602052604081205482106116575760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610987565b6001600160a01b0383165f908152601360205260409020805483908110611680576116806134bd565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b0316905092915050565b610b32828260405180602001604052805f815250611d2e565b5f6001600160e01b03198216637965db0b60e01b1480610a6b5750610a6b82611d44565b5f818152600260205260408120546001600160a01b031680610a6b57604051637e27328960e01b815260048101849052602401610987565b610c798383836001611d68565b5f61173d848484611e6c565b949350505050565b610d258133611e87565b5f61175a8383610f16565b6117d9575f838152600b602090815260408083206001600160a01b03861684529091529020805460ff191660011790556117913390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4506001610a6b565b505f610a6b565b5f6117eb8383610f16565b156117d9575f838152600b602090815260408083206001600160a01b0386168085529252808320805460ff1916905551339286917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a4506001610a6b565b5f65ffffffffffff82111561187d576040516306dfcc6560e41b81526030600482015260248101839052604401610987565b5090565b81545f90818160058111156118dd575f61189a84611ec0565b6118a490856134fb565b5f8881526020902090915081015465ffffffffffff90811690871610156118cd578091506118db565b6118d881600161350e565b92505b505b5f6118ea87878585611fa4565b905080156119245761190e876119016001846134fb565b5f91825260209091200190565b54600160301b90046001600160d01b0316611926565b5f5b979650505050505050565b611939612003565b600a805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038281165f818152600f602052604080822080548686166001600160a01b0319821681179092559151919094169392849290917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4610c7981836119ef86612028565b612032565b6119fc61219b565b600a805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586119663390565b6060611a5e7f0000000000000000000000000000000000000000000000000000000000000000600c6121bf565b905090565b6060611a5e7f0000000000000000000000000000000000000000000000000000000000000000600d6121bf565b80545f908015611abf57611aa9836119016001846134fb565b54600160301b90046001600160d01b0316611ac1565b5f5b9392505050565b6001600160a01b038216611afa57604051630b61174360e31b81526001600160a01b0383166004820152602401610987565b6001600160a01b038381165f81815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3191016114c8565b6001600160a01b0383163b15610bb957604051630a85bd0160e11b81526001600160a01b0384169063150b7a0290611ba0903390889087908790600401613521565b6020604051808303815f875af1925050508015611bda575060408051601f3d908101601f19168201909252611bd79181019061355d565b60015b611c41573d808015611c07576040519150601f19603f3d011682016040523d82523d5f602084013e611c0c565b606091505b5080515f03611c3957604051633250574960e11b81526001600160a01b0385166004820152602401610987565b805181602001fd5b6001600160e01b03198116630a85bd0160e11b14611c7d57604051633250574960e11b81526001600160a01b0385166004820152602401610987565b5050505050565b5f610a6b611c90612268565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f5f611cc088888888612391565b925092509250611cd08282612459565b50909695505050505050565b6001600160a01b0382165f908152600e60205260409020805460018101909155818114610c79576040516301d4b62360e61b81526001600160a01b038416600482015260248101829052604401610987565b611d388383612511565b610c795f848484611b5e565b5f6001600160e01b0319821663780e9d6360e01b1480610a6b5750610a6b82612572565b8080611d7c57506001600160a01b03821615155b15611e3d575f611d8b846116ec565b90506001600160a01b03831615801590611db75750826001600160a01b0316816001600160a01b031614155b8015611dca5750611dc881846115ce565b155b15611df35760405163a9fbf51f60e01b81526001600160a01b0384166004820152602401610987565b8115611e3b5783856001600160a01b0316826001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45b505b50505f90815260046020526040902080546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f611e798585856125c1565b905061173d818660016125d5565b611e918282610f16565b610b325760405163e2517d3f60e01b81526001600160a01b038216600482015260248101839052604401610987565b5f815f03611ecf57505f919050565b5f6001611edb8461264a565b901c6001901b90506001818481611ef457611ef4613578565b048201901c90506001818481611f0c57611f0c613578565b048201901c90506001818481611f2457611f24613578565b048201901c90506001818481611f3c57611f3c613578565b048201901c90506001818481611f5457611f54613578565b048201901c90506001818481611f6c57611f6c613578565b048201901c90506001818481611f8457611f84613578565b048201901c9050611ac181828581611f9e57611f9e613578565b046126dd565b5f5b81831015611ffb575f611fb984846126f2565b5f8781526020902090915065ffffffffffff86169082015465ffffffffffff161115611fe757809250611ff5565b611ff281600161350e565b93505b50611fa6565b509392505050565b600a5460ff1661202657604051638dfc202b60e01b815260040160405180910390fd5b565b5f610a6b82610db7565b816001600160a01b0316836001600160a01b03161415801561205357505f81115b15610c79576001600160a01b038316156120fa576001600160a01b0383165f90815260106020526040812081906120959061270c61209086612717565b61274a565b6001600160d01b031691506001600160d01b03169150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516120ef929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615610c79576001600160a01b0382165f90815260106020526040812081906121329061277b61209086612717565b6001600160d01b031691506001600160d01b03169150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161218c929190918252602082015260400190565b60405180910390a25050505050565b600a5460ff16156120265760405163d93c066560e01b815260040160405180910390fd5b606060ff83146121d9576121d283612786565b9050610a6b565b8180546121e590613387565b80601f016020809104026020016040519081016040528092919081815260200182805461221190613387565b801561225c5780601f106122335761010080835404028352916020019161225c565b820191905f5260205f20905b81548152906001019060200180831161223f57829003601f168201915b50505050509050610a6b565b5f306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480156122c057507f000000000000000000000000000000000000000000000000000000000000000046145b156122ea57507f000000000000000000000000000000000000000000000000000000000000000090565b611a5e604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201527f0000000000000000000000000000000000000000000000000000000000000000918101919091527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411156123ca57505f9150600390508261244f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561241b573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b03811661244657505f92506001915082905061244f565b92505f91508190505b9450945094915050565b5f82600381111561246c5761246c61358c565b03612475575050565b60018260038111156124895761248961358c565b036124a75760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156124bb576124bb61358c565b036124dc5760405163fce698f760e01b815260048101829052602401610987565b60038260038111156124f0576124f061358c565b03610b32576040516335e2f38360e21b815260048101829052602401610987565b6001600160a01b03821661253a57604051633250574960e11b81525f6004820152602401610987565b5f61254683835f611731565b90506001600160a01b03811615610c79576040516339e3563760e11b81525f6004820152602401610987565b5f6001600160e01b031982166380ac58cd60e01b14806125a257506001600160e01b03198216635b5e139f60e01b145b80610a6b57506301ffc9a760e01b6001600160e01b0319831614610a6b565b5f6125ca61219b565b61173d8484846127c3565b6001600160a01b0383166125f7576125f4601161277b61209084612717565b50505b6001600160a01b03821661261957612616601161270c61209084612717565b50505b6001600160a01b038381165f908152600f6020526040808220548584168352912054610c7992918216911683612032565b5f80608083901c1561265e57608092831c92015b604083901c1561267057604092831c92015b602083901c1561268257602092831c92015b601083901c1561269457601092831c92015b600883901c156126a657600892831c92015b600483901c156126b857600492831c92015b600283901c156126ca57600292831c92015b600183901c15610a6b5760010192915050565b5f8183106126eb5781611ac1565b5090919050565b5f61270060028484186135a0565b611ac19084841661350e565b5f611ac182846135bf565b5f6001600160d01b0382111561187d576040516306dfcc6560e41b815260d0600482015260248101839052604401610987565b5f8061276e4261276661275c88611a90565b868863ffffffff16565b87919061288e565b915091505b935093915050565b5f611ac182846135de565b60605f6127928361289b565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f5f6127d08585856128c2565b90506001600160a01b03811661282c5761282784600880545f838152600960205260408120829055600182018355919091527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee30155565b61284f565b846001600160a01b0316816001600160a01b03161461284f5761284f81856129b4565b6001600160a01b03851661286b5761286684612a41565b61173d565b846001600160a01b0316816001600160a01b03161461173d5761173d8585612ae8565b5f8061276e858585612b36565b5f60ff8216601f811115610a6b57604051632cd44ac360e21b815260040160405180910390fd5b5f828152600260205260408120546001600160a01b03908116908316156128ee576128ee818486612cac565b6001600160a01b03811615612928576129095f855f5f611d68565b6001600160a01b0381165f90815260036020526040902080545f190190555b6001600160a01b03851615612956576001600160a01b0385165f908152600360205260409020805460010190555b5f8481526002602052604080822080546001600160a01b0319166001600160a01b0389811691821790925591518793918516917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4949350505050565b5f6129be83610db7565b5f83815260076020526040902054909150808214612a0f576001600160a01b0384165f9081526006602090815260408083208584528252808320548484528184208190558352600790915290208190555b505f9182526007602090815260408084208490556001600160a01b039094168352600681528383209183525290812055565b6008545f90612a52906001906134fb565b5f8381526009602052604081205460088054939450909284908110612a7957612a796134bd565b905f5260205f20015490508060088381548110612a9857612a986134bd565b5f918252602080832090910192909255828152600990915260408082208490558582528120556008805480612acf57612acf6135fd565b600190038181905f5260205f20015f9055905550505050565b5f6001612af484610db7565b612afe91906134fb565b6001600160a01b039093165f908152600660209081526040808320868452825280832085905593825260079052919091209190915550565b82545f9081908015612c52575f612b52876119016001856134fb565b60408051808201909152905465ffffffffffff808216808452600160301b9092046001600160d01b031660208401529192509087161015612ba657604051632520601d60e01b815260040160405180910390fd5b805165ffffffffffff808816911603612bf25784612bc9886119016001866134fb565b80546001600160d01b0392909216600160301b0265ffffffffffff909216919091179055612c42565b6040805180820190915265ffffffffffff80881682526001600160d01b0380881660208085019182528b54600181018d555f8d81529190912094519151909216600160301b029216919091179101555b6020015192508391506127739050565b50506040805180820190915265ffffffffffff80851682526001600160d01b0380851660208085019182528854600181018a555f8a815291822095519251909316600160301b029190931617920191909155905081612773565b612cb7838383612d10565b610c79576001600160a01b038316612ce557604051637e27328960e01b815260048101829052602401610987565b60405163177e802f60e01b81526001600160a01b038316600482015260248101829052604401610987565b5f6001600160a01b0383161580159061173d5750826001600160a01b0316846001600160a01b03161480612d495750612d4984846115ce565b8061173d5750505f908152600460205260409020546001600160a01b03908116911614919050565b508054612d7d90613387565b5f825580601f10612d8c575050565b601f0160209004905f5260205f2090810190610d2591905b8082111561187d575f8155600101612da4565b5f5f83601f840112612dc7575f5ffd5b5081356001600160401b03811115612ddd575f5ffd5b602083019150836020828501011115612df4575f5ffd5b9250929050565b5f5f5f5f60408587031215612e0e575f5ffd5b84356001600160401b03811115612e23575f5ffd5b612e2f87828801612db7565b90955093505060208501356001600160401b03811115612e4d575f5ffd5b612e5987828801612db7565b95989497509550505050565b6001600160e01b031981168114610d25575f5ffd5b5f60208284031215612e8a575f5ffd5b8135611ac181612e65565b5f81518084525f5b81811015612eb957602081850181015186830182015201612e9d565b505f602082860101526020601f19601f83011685010191505092915050565b602081525f611ac16020830184612e95565b5f60208284031215612efa575f5ffd5b5035919050565b6001600160a01b0381168114610d25575f5ffd5b5f5f60408385031215612f26575f5ffd5b8235612f3181612f01565b946020939093013593505050565b5f5f5f60608486031215612f51575f5ffd5b8335612f5c81612f01565b92506020840135612f6c81612f01565b929592945050506040919091013590565b5f5f60408385031215612f8e575f5ffd5b823591506020830135612fa081612f01565b809150509250929050565b5f60208284031215612fbb575f5ffd5b8135611ac181612f01565b60ff60f81b8816815260e060208201525f612fe460e0830189612e95565b8281036040840152612ff68189612e95565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b8181101561304b57835183526020938401939092019160010161302d565b50909b9a5050505050505050505050565b5f5f6040838503121561306d575f5ffd5b823561307881612f01565b915060208301358015158114612fa0575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b038411156130b9576130b961308c565b50604051601f19601f85018116603f011681018181106001600160401b03821117156130e7576130e761308c565b6040528381529050808284018510156130fe575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f830112613124575f5ffd5b611ac1838335602085016130a0565b5f5f5f5f60808587031215613146575f5ffd5b843561315181612f01565b9350602085013561316181612f01565b92506040850135915060608501356001600160401b03811115613182575f5ffd5b61318e87828801613115565b91505092959194509250565b5f5f5f5f5f5f60c087890312156131af575f5ffd5b86356131ba81612f01565b95506020870135945060408701359350606087013560ff811681146131dd575f5ffd5b9598949750929560808101359460a0909101359350915050565b5f60208284031215613207575f5ffd5b81356001600160401b0381168114611ac1575f5ffd5b6001600160401b03841681526001600160a01b03831660208201526060604082018190525f9061324f90830184612e95565b95945050505050565b5f60208284031215613268575f5ffd5b81356001600160401b0381111561327d575f5ffd5b61173d84828501613115565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f604083015160608084015261173d6080840182612e95565b5f5f604083850312156132d9575f5ffd5b82356132e481612f01565b91506020830135612fa081612f01565b5f5f5f60608486031215613306575f5ffd5b833561331181612f01565b92506020840135915060408401356001600160401b03811115613332575f5ffd5b8401601f81018613613342575f5ffd5b613351868235602084016130a0565b9150509250925092565b634e487b7160e01b5f52601160045260245ffd5b5f600182016133805761338061335b565b5060010190565b600181811c9082168061339b57607f821691505b6020821081036133b957634e487b7160e01b5f52602260045260245ffd5b50919050565b601f821115610c7957805f5260205f20601f840160051c810160208510156133e45750805b601f840160051c820191505b81811015611c7d575f81556001016133f0565b81516001600160401b0381111561341c5761341c61308c565b6134308161342a8454613387565b846133bf565b6020601f821160018114613462575f831561344b5750848201515b5f19600385901b1c1916600184901b178455611c7d565b5f84815260208120601f198516915b828110156134915787850151825560209485019460019092019101613471565b50848210156134ae57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b5f52603260045260245ffd5b5f6001600160401b0382166001600160401b0381036134f2576134f261335b565b60010192915050565b81810381811115610a6b57610a6b61335b565b80820180821115610a6b57610a6b61335b565b6001600160a01b03858116825284166020820152604081018390526080606082018190525f9061355390830184612e95565b9695505050505050565b5f6020828403121561356d575f5ffd5b8151611ac181612e65565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b5f826135ba57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160d01b038281168282160390811115610a6b57610a6b61335b565b6001600160d01b038181168382160190811115610a6b57610a6b61335b565b634e487b7160e01b5f52603160045260245ffdfea26469706673582212206bc5d5e6e5a603eebe85c9ec7af325bedd9507e3e28d42ad63a5644732a728aa64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W__\xFD[P`@\x80Q\x80\x82\x01\x82R`\n\x80\x82Ri\x15\x1C\x9AY\xD9\xD9\\\x93\x91\x95`\xB2\x1B` \x80\x84\x01\x82\x90R\x84Q\x80\x86\x01\x86R`\x01\x81R`1`\xF8\x1B\x81\x83\x01R\x85Q\x80\x87\x01\x87R\x93\x84R\x83\x82\x01\x92\x90\x92R\x84Q\x80\x86\x01\x90\x95R`\x04\x85Rc\x15\x13\x91\x95`\xE2\x1B\x90\x85\x01R\x91\x92_a\0\x7F\x83\x82a\x03yV[P`\x01a\0\x8C\x82\x82a\x03yV[PP`\n\x80T`\xFF\x19\x16\x90UPa\0\xA4\x82`\x0Ca\x01\xBEV[a\x01 Ra\0\xB3\x81`\ra\x01\xBEV[a\x01@R\x81Q` \x80\x84\x01\x91\x90\x91 `\xE0R\x81Q\x90\x82\x01 a\x01\0RF`\xA0Ra\x01?`\xE0Qa\x01\0Q`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x90\x81\x01\x92\x90\x92R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[`\x80RPP0`\xC0Ra\x01R_3a\x01\xF0V[Pa\x01}\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*3a\x01\xF0V[Pa\x01\xA8\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K3a\x01\xF0V[P`\x16\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90Ua\x04\xA1V[_` \x83Q\x10\x15a\x01\xD9Wa\x01\xD2\x83a\x02\x9BV[\x90Pa\x01\xEAV[\x81a\x01\xE4\x84\x82a\x03yV[P`\xFF\x90P[\x92\x91PPV[_\x82\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x81 T`\xFF\x16a\x02\x94W_\x83\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x02L3\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\x01\xEAV[P_a\x01\xEAV[__\x82\x90P`\x1F\x81Q\x11\x15a\x02\xCEW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\x02\xC5\x91\x90a\x043V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\x02\xD9\x82a\x04~V[\x17\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x03'WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x03tW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x03RWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x03qW_\x81U`\x01\x01a\x03^V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x92Wa\x03\x92a\x02\xE1V[a\x03\xA6\x81a\x03\xA0\x84Ta\x02\xF5V[\x84a\x03-V[` `\x1F\x82\x11`\x01\x81\x14a\x03\xD8W_\x83\x15a\x03\xC1WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x03qV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x04\x07W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x03\xE7V[P\x84\x82\x10\x15a\x04$W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15a\x04_W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x04BV[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03'W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa6Ga\x04\xF2_9_a\x1Aj\x01R_a\x1A8\x01R_a#A\x01R_a#\x19\x01R_a\"t\x01R_a\"\x9E\x01R_a\"\xC8\x01Ra6G_\xF3\xFE`\x80`@R`\x046\x10a\x02\x8AW_5`\xE0\x1C\x80cp\xA0\x821\x11a\x01UW\x80c\xA2,\xB4e\x11a\0\xBEW\x80c\xD5Gt\x1F\x11a\0xW\x80c\xD5Gt\x1F\x14a\x08NW\x80c\xE3\x1E\x07\x88\x14a\x08mW\x80c\xE3(\xEDw\x14a\x08\x80W\x80c\xE6:\xB1\xE9\x14a\x08\xACW\x80c\xE9\x85\xE9\xC5\x14a\x08\xDFW\x80c\xF7\xEE\x94L\x14a\x08\xFEW__\xFD[\x80c\xA2,\xB4e\x14a\x07\x85W\x80c\xB8\x8DO\xDE\x14a\x07\xA4W\x80c\xC3\xCD\xA5 \x14a\x07\xC3W\x80c\xC4\xD6m\xE8\x14a\x07\xE2W\x80c\xC8{V\xDD\x14a\x08\x01W\x80c\xCE(\x96\x12\x14a\x08 W__\xFD[\x80c\x91;\x1F\xBF\x11a\x01\x0FW\x80c\x91;\x1F\xBF\x14a\x06\xC7W\x80c\x91\xD1HT\x14a\x06\xFEW\x80c\x91\xDD\xAD\xF4\x14a\x07\x1DW\x80c\x95\xD8\x9BA\x14a\x07?W\x80c\x9A\xB2N\xB0\x14a\x07SW\x80c\xA2\x17\xFD\xDF\x14a\x07rW__\xFD[\x80cp\xA0\x821\x14a\x05\xFBW\x80c~\xCE\xBE\0\x14a\x06\x1AW\x80c\x84V\xCBY\x14a\x06NW\x80c\x84\xB0\x19n\x14a\x06bW\x80c\x8Di\xE9^\x14a\x06\x89W\x80c\x8ES\x9E\x8C\x14a\x06\xA8W__\xFD[\x80c3\x83\xAB\xFE\x11a\x01\xF7W\x80cK\xF5\xD7\xE9\x11a\x01\xB1W\x80cK\xF5\xD7\xE9\x14a\x05\x1AW\x80cOl\xCC\xE7\x14a\x05PW\x80cX|\xDE\x1E\x14a\x05oW\x80c\\\x19\xA9\\\x14a\x05\xA6W\x80c\\\x97Z\xBB\x14a\x05\xC5W\x80ccR!\x1E\x14a\x05\xDCW__\xFD[\x80c3\x83\xAB\xFE\x14a\x04VW\x80c6V\x8A\xBE\x14a\x04\x8AW\x80c:F\xB1\xA8\x14a\x04\xA9W\x80c?K\xA8:\x14a\x04\xC8W\x80cB\x84.\x0E\x14a\x04\xDCW\x80cB\x96lh\x14a\x04\xFBW__\xFD[\x80c\x18\x16\r\xDD\x11a\x02HW\x80c\x18\x16\r\xDD\x14a\x03zW\x80c\x1B\xC6\xAE\x8A\x14a\x03\x98W\x80c#\xB8r\xDD\x14a\x03\xCBW\x80c$\x8A\x9C\xA3\x14a\x03\xEAW\x80c//\xF1]\x14a\x04\x18W\x80c/t\\Y\x14a\x047W__\xFD[\x80bs\xE1\xD7\x14a\x02\x8EW\x80c\x01\xFF\xC9\xA7\x14a\x02\xAFW\x80c\x06\xFD\xDE\x03\x14a\x02\xE3W\x80c\x08\x18\x12\xFC\x14a\x03\x04W\x80c\t^\xA7\xB3\x14a\x03;W\x80c\x15\x8E\xF9>\x14a\x03ZW[__\xFD[4\x80\x15a\x02\x99W__\xFD[Pa\x02\xADa\x02\xA86`\x04a-\xFBV[a\t\x1DV[\0[4\x80\x15a\x02\xBAW__\xFD[Pa\x02\xCEa\x02\xC96`\x04a.zV[a\naV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEEW__\xFD[Pa\x02\xF7a\nqV[`@Qa\x02\xDA\x91\x90a.\xD8V[4\x80\x15a\x03\x0FW__\xFD[Pa\x03#a\x03\x1E6`\x04a.\xEAV[a\x0B\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x03FW__\xFD[Pa\x02\xADa\x03U6`\x04a/\x15V[a\x0B'V[4\x80\x15a\x03eW__\xFD[P`\x16Ta\x02\xCE\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x03\x85W__\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x02\xDAV[4\x80\x15a\x03\xA3W__\xFD[Pa\x03\x8A\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x81V[4\x80\x15a\x03\xD6W__\xFD[Pa\x02\xADa\x03\xE56`\x04a/?V[a\x0B6V[4\x80\x15a\x03\xF5W__\xFD[Pa\x03\x8Aa\x04\x046`\x04a.\xEAV[_\x90\x81R`\x0B` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04#W__\xFD[Pa\x02\xADa\x0426`\x04a/}V[a\x0B\xBFV[4\x80\x15a\x04BW__\xFD[Pa\x03\x8Aa\x04Q6`\x04a/\x15V[a\x0B\xE3V[4\x80\x15a\x04aW__\xFD[Pa\x03\x8Aa\x04p6`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x13` R`@\x90 T\x90V[4\x80\x15a\x04\x95W__\xFD[Pa\x02\xADa\x04\xA46`\x04a/}V[a\x0CFV[4\x80\x15a\x04\xB4W__\xFD[Pa\x03\x8Aa\x04\xC36`\x04a/\x15V[a\x0C~V[4\x80\x15a\x04\xD3W__\xFD[Pa\x02\xADa\x0C\xF3V[4\x80\x15a\x04\xE7W__\xFD[Pa\x02\xADa\x04\xF66`\x04a/?V[a\r(V[4\x80\x15a\x05\x06W__\xFD[Pa\x02\xADa\x05\x156`\x04a.\xEAV[a\rBV[4\x80\x15a\x05%W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\xF7V[4\x80\x15a\x05[W__\xFD[Pa\x03\x8Aa\x05j6`\x04a.\xEAV[a\rMV[4\x80\x15a\x05zW__\xFD[Pa\x03#a\x05\x896`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x0F` R`@\x90 T\x16\x90V[4\x80\x15a\x05\xB1W__\xFD[Pa\x02\xADa\x05\xC06`\x04a/\xABV[a\r\xA2V[4\x80\x15a\x05\xD0W__\xFD[P`\nT`\xFF\x16a\x02\xCEV[4\x80\x15a\x05\xE7W__\xFD[Pa\x03#a\x05\xF66`\x04a.\xEAV[a\r\xADV[4\x80\x15a\x06\x06W__\xFD[Pa\x03\x8Aa\x06\x156`\x04a/\xABV[a\r\xB7V[4\x80\x15a\x06%W__\xFD[Pa\x03\x8Aa\x0646`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x0E` R`@\x90 T\x90V[4\x80\x15a\x06YW__\xFD[Pa\x02\xADa\r\xFCV[4\x80\x15a\x06mW__\xFD[Pa\x06va\x0E.V[`@Qa\x02\xDA\x97\x96\x95\x94\x93\x92\x91\x90a/\xC6V[4\x80\x15a\x06\x94W__\xFD[P`\x16Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xB3W__\xFD[Pa\x03\x8Aa\x06\xC26`\x04a.\xEAV[a\x0EpV[4\x80\x15a\x06\xD2W__\xFD[Pa\x06\xE6a\x06\xE16`\x04a/\x15V[a\x0E\xCFV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x07\tW__\xFD[Pa\x02\xCEa\x07\x186`\x04a/}V[a\x0F\x16V[4\x80\x15a\x07(W__\xFD[P`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x07JW__\xFD[Pa\x02\xF7a\x0F@V[4\x80\x15a\x07^W__\xFD[Pa\x03\x8Aa\x07m6`\x04a/\xABV[a\x0FOV[4\x80\x15a\x07}W__\xFD[Pa\x03\x8A_\x81V[4\x80\x15a\x07\x90W__\xFD[Pa\x02\xADa\x07\x9F6`\x04a0\\V[a\x0F~V[4\x80\x15a\x07\xAFW__\xFD[Pa\x02\xADa\x07\xBE6`\x04a13V[a\x0F\x89V[4\x80\x15a\x07\xCEW__\xFD[Pa\x02\xADa\x07\xDD6`\x04a1\x9AV[a\x0F\xA0V[4\x80\x15a\x07\xEDW__\xFD[Pa\x02\xADa\x07\xFC6`\x04a/\xABV[a\x10\\V[4\x80\x15a\x08\x0CW__\xFD[Pa\x02\xF7a\x08\x1B6`\x04a.\xEAV[a\x11_V[4\x80\x15a\x08+W__\xFD[Pa\x08?a\x08:6`\x04a1\xF7V[a\x12`V[`@Qa\x02\xDA\x93\x92\x91\x90a2\x1DV[4\x80\x15a\x08YW__\xFD[Pa\x02\xADa\x08h6`\x04a/}V[a\x13\x1EV[a\x02\xADa\x08{6`\x04a2XV[a\x13BV[4\x80\x15a\x08\x8BW__\xFD[Pa\x08\x9Fa\x08\x9A6`\x04a1\xF7V[a\x14\xD5V[`@Qa\x02\xDA\x91\x90a2\x89V[4\x80\x15a\x08\xB7W__\xFD[Pa\x03\x8A\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[4\x80\x15a\x08\xEAW__\xFD[Pa\x02\xCEa\x08\xF96`\x04a2\xC8V[a\x15\xCEV[4\x80\x15a\t\tW__\xFD[Pa\x06\xE6a\t\x186`\x04a/\x15V[a\x15\xFBV[a\tG\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K3a\x0F\x16V[a\t\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt'\xB76<\x909\xB2\xB9;4\xB1\xB2\x90897\xBB4\xB22\xB9`Y\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\t\x9F\x86\x88\x01\x88a2\xF4V[\x92P\x92P\x92P_`\x15_\x81T\x80\x92\x91\x90a\t\xB8\x90a3oV[\x91\x90PU\x90Pa\t\xC8\x84\x82a\x16\xAFV[_\x81\x81R`\x17` R`@\x90 a\t\xDF\x83\x82a4\x03V[P`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R`\x12` R`@\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\n\x13`\x01\x83\x01\x82a-qV[PP\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD3[\xB9^\t\xC0K!\x9E5\x04|\xE7\xB7\xB3\0\xE38Bd\xEF\x84\xA4\x04V\x94=\xBC\x0F\xC1|\x14\x84`@Qa\nO\x91\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[_a\nk\x82a\x16\xC8V[\x92\x91PPV[``_\x80Ta\n\x7F\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xAB\x90a3\x87V[\x80\x15a\n\xF6W\x80`\x1F\x10a\n\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x0B\n\x82a\x16\xECV[P_\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\nkV[a\x0B2\x82\x823a\x17$V[PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B_W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_a\x0Bk\x83\x833a\x171V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xB9W`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\t\x87V[PPPPV[_\x82\x81R`\x0B` R`@\x90 `\x01\x01Ta\x0B\xD9\x81a\x17EV[a\x0B\xB9\x83\x83a\x17OV[_a\x0B\xED\x83a\r\xB7V[\x82\x10a\x0C\x1EW`@Qc)_D\xF7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0CoW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cy\x82\x82a\x17\xE0V[PPPV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x0C\xB9W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\x87V[a\x0C\xE2a\x0C\xC5\x84a\x18KV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x10` R`@\x90 \x90a\x18\x81V[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\r\x1D\x81a\x17EV[a\r%a\x191V[PV[a\x0Cy\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x0F\x89V[a\x0B2_\x823a\x171V[_a\rW`\x08T\x90V[\x82\x10a\r\x7FW`@Qc)_D\xF7`\xE2\x1B\x81R_`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[`\x08\x82\x81T\x81\x10a\r\x92Wa\r\x92a4\xBDV[\x90_R` _ \x01T\x90P\x91\x90PV[3a\x0B2\x81\x83a\x19\x83V[_a\nk\x82a\x16\xECV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\xE1W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x0E&\x81a\x17EV[a\r%a\x19\xF4V[_``\x80___``a\x0E?a\x1A1V[a\x0EGa\x1AcV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x0E\xABW`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\x87V[a\x0E\xBFa\x0E\xB7\x84a\x18KV[`\x11\x90a\x18\x81V[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[`\x13` R\x81_R`@_ \x81\x81T\x81\x10a\x0E\xE8W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_\x91\x82R`\x0B` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x01\x80Ta\n\x7F\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x10` R`@\x81 a\x0Fo\x90a\x1A\x90V[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[a\x0B23\x83\x83a\x1A\xC8V[a\x0F\x94\x84\x84\x84a\x0B6V[a\x0B\xB9\x84\x84\x84\x84a\x1B^V[\x83B\x11\x15a\x0F\xC4W`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\t\x87V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x10=\x90a\x105\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1C\x84V[\x85\x85\x85a\x1C\xB0V[\x90Pa\x10I\x81\x87a\x1C\xDCV[a\x10S\x81\x88a\x19\x83V[PPPPPPPV[_a\x10f\x81a\x17EV[`\x16T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x10\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x11\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid service provider\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\x87V[a\x116\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x83a\x17OV[PP`\x16\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[_\x81\x81R`\x02` R`@\x90 T``\x90`\x01`\x01`\xA0\x1B\x03\x16a\x11\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FURI query for nonexistent token\0`D\x82\x01R`d\x01a\t\x87V[_\x82\x81R`\x17` R`@\x90 \x80Ta\x11\xDD\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\t\x90a3\x87V[\x80\x15a\x12TW\x80`\x1F\x10a\x12+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12TV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x127W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\x12` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`@\x1B\x03\x83\x16\x93`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x12\x9D\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xC9\x90a3\x87V[\x80\x15a\x13\x14W\x80`\x1F\x10a\x12\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x14V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[_\x82\x81R`\x0B` R`@\x90 `\x01\x01Ta\x138\x81a\x17EV[a\x0B\xB9\x83\x83a\x17\xE0V[4g\x01cEx]\x8A\0\0\x14a\x13\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\t\x87V[`\x14\x80T_\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x82a\x13\xB5\x83a4\xD1V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x92\x82\x02\x19\x16\x91\x90\x91\x17\x90\x91U`@\x80Q``\x81\x01\x82R\x83\x83\x16\x80\x82R3` \x80\x84\x01\x91\x82R\x83\x85\x01\x89\x81R_\x93\x84R`\x12\x90\x91R\x93\x90\x91 \x82Q\x81T\x92Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x95\x16\x94\x90\x94\x17\x17\x83U\x90Q\x92\x93P\x91\x82\x91\x90`\x01\x82\x01\x90a\x14F\x90\x82a4\x03V[PP3_\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x91\x90\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x91\x92P\x90\x7F\xF3\xF4\x11\xD8SHk\x9FS\xDAc\0\x9A!\xCD(N\xA1\x8A\x80\rM\xE5\\\xE5\xBD\x93]\x19~L\xF1\x90a\x14\xC8\x90\x87\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x81\x16_\x90\x81R`\x12` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x15G\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15s\x90a3\x87V[\x80\x15a\x15\xBEW\x80`\x1F\x10a\x15\x95Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xBEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x13` R`@\x81 T\x82\x10a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x13` R`@\x90 \x80T\x83\x90\x81\x10a\x16\x80Wa\x16\x80a4\xBDV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P\x92\x91PPV[a\x0B2\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x1D.V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\nkWPa\nk\x82a\x1DDV[_\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\nkW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\x87V[a\x0Cy\x83\x83\x83`\x01a\x1DhV[_a\x17=\x84\x84\x84a\x1ElV[\x94\x93PPPPV[a\r%\x813a\x1E\x87V[_a\x17Z\x83\x83a\x0F\x16V[a\x17\xD9W_\x83\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x17\x913\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\nkV[P_a\nkV[_a\x17\xEB\x83\x83a\x0F\x16V[\x15a\x17\xD9W_\x83\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\nkV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18}W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[P\x90V[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x18\xDDW_a\x18\x9A\x84a\x1E\xC0V[a\x18\xA4\x90\x85a4\xFBV[_\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x18\xCDW\x80\x91Pa\x18\xDBV[a\x18\xD8\x81`\x01a5\x0EV[\x92P[P[_a\x18\xEA\x87\x87\x85\x85a\x1F\xA4V[\x90P\x80\x15a\x19$Wa\x19\x0E\x87a\x19\x01`\x01\x84a4\xFBV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x19&V[_[\x97\x96PPPPPPPV[a\x199a \x03V[`\n\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x0F` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x0Cy\x81\x83a\x19\xEF\x86a (V[a 2V[a\x19\xFCa!\x9BV[`\n\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x19f3\x90V[``a\x1A^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0Ca!\xBFV[\x90P\x90V[``a\x1A^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\ra!\xBFV[\x80T_\x90\x80\x15a\x1A\xBFWa\x1A\xA9\x83a\x19\x01`\x01\x84a4\xFBV[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x1A\xC1V[_[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1A\xFAW`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x14\xC8V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x0B\xB9W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\x1B\xA0\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a5!V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x1B\xDAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1B\xD7\x91\x81\x01\x90a5]V[`\x01[a\x1CAW=\x80\x80\x15a\x1C\x07W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\x0CV[``\x91P[P\x80Q_\x03a\x1C9W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\x87V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x1C}W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\x87V[PPPPPV[_a\nka\x1C\x90a\"hV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a\x1C\xC0\x88\x88\x88\x88a#\x91V[\x92P\x92P\x92Pa\x1C\xD0\x82\x82a$YV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x0E` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\x0CyW`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\x87V[a\x1D8\x83\x83a%\x11V[a\x0Cy_\x84\x84\x84a\x1B^V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14\x80a\nkWPa\nk\x82a%rV[\x80\x80a\x1D|WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a\x1E=W_a\x1D\x8B\x84a\x16\xECV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1D\xB7WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x1D\xCAWPa\x1D\xC8\x81\x84a\x15\xCEV[\x15[\x15a\x1D\xF3W`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\t\x87V[\x81\x15a\x1E;W\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP_\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[__a\x1Ey\x85\x85\x85a%\xC1V[\x90Pa\x17=\x81\x86`\x01a%\xD5V[a\x1E\x91\x82\x82a\x0F\x16V[a\x0B2W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[_\x81_\x03a\x1E\xCFWP_\x91\x90PV[_`\x01a\x1E\xDB\x84a&JV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1E\xF4Wa\x1E\xF4a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F\x0CWa\x1F\x0Ca5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F$Wa\x1F$a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F<Wa\x1F<a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1FTWa\x1FTa5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1FlWa\x1Fla5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F\x84Wa\x1F\x84a5xV[\x04\x82\x01\x90\x1C\x90Pa\x1A\xC1\x81\x82\x85\x81a\x1F\x9EWa\x1F\x9Ea5xV[\x04a&\xDDV[_[\x81\x83\x10\x15a\x1F\xFBW_a\x1F\xB9\x84\x84a&\xF2V[_\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xE7W\x80\x92Pa\x1F\xF5V[a\x1F\xF2\x81`\x01a5\x0EV[\x93P[Pa\x1F\xA6V[P\x93\x92PPPV[`\nT`\xFF\x16a &W`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\nk\x82a\r\xB7V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a SWP_\x81\x11[\x15a\x0CyW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a \xFAW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x10` R`@\x81 \x81\x90a \x95\x90a'\x0Ca \x90\x86a'\x17V[a'JV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa \xEF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x0CyW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x10` R`@\x81 \x81\x90a!2\x90a'{a \x90\x86a'\x17V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\x8C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\nT`\xFF\x16\x15a &W`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\xFF\x83\x14a!\xD9Wa!\xD2\x83a'\x86V[\x90Pa\nkV[\x81\x80Ta!\xE5\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\x11\x90a3\x87V[\x80\x15a\"\\W\x80`\x1F\x10a\"3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\\V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"?W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\nkV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\"\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\"\xEAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x1A^`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a#\xCAWP_\x91P`\x03\x90P\x82a$OV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a$\x1BW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a$FWP_\x92P`\x01\x91P\x82\x90Pa$OV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a$lWa$la5\x8CV[\x03a$uWPPV[`\x01\x82`\x03\x81\x11\x15a$\x89Wa$\x89a5\x8CV[\x03a$\xA7W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a$\xBBWa$\xBBa5\x8CV[\x03a$\xDCW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`\x03\x82`\x03\x81\x11\x15a$\xF0Wa$\xF0a5\x8CV[\x03a\x0B2W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x82\x16a%:W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_a%F\x83\x83_a\x171V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0CyW`@Qc9\xE3V7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a%\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\nkWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\nkV[_a%\xCAa!\x9BV[a\x17=\x84\x84\x84a'\xC3V[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\xF7Wa%\xF4`\x11a'{a \x90\x84a'\x17V[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a&\x19Wa&\x16`\x11a'\x0Ca \x90\x84a'\x17V[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x0F` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x0Cy\x92\x91\x82\x16\x91\x16\x83a 2V[_\x80`\x80\x83\x90\x1C\x15a&^W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a&pW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a&\x82W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a&\x94W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a&\xA6W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a&\xB8W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a&\xCAW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\nkW`\x01\x01\x92\x91PPV[_\x81\x83\x10a&\xEBW\x81a\x1A\xC1V[P\x90\x91\x90PV[_a'\0`\x02\x84\x84\x18a5\xA0V[a\x1A\xC1\x90\x84\x84\x16a5\x0EV[_a\x1A\xC1\x82\x84a5\xBFV[_`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\x18}W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[_\x80a'nBa'fa'\\\x88a\x1A\x90V[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a(\x8EV[\x91P\x91P[\x93P\x93\x91PPV[_a\x1A\xC1\x82\x84a5\xDEV[``_a'\x92\x83a(\x9BV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[__a'\xD0\x85\x85\x85a(\xC2V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a(,Wa('\x84`\x08\x80T_\x83\x81R`\t` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x01UV[a(OV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a(OWa(O\x81\x85a)\xB4V[`\x01`\x01`\xA0\x1B\x03\x85\x16a(kWa(f\x84a*AV[a\x17=V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17=Wa\x17=\x85\x85a*\xE8V[_\x80a'n\x85\x85\x85a+6V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\nkW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a(\xEEWa(\xEE\x81\x84\x86a,\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a)(Wa)\t_\x85__a\x1DhV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a)VW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[_\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[_a)\xBE\x83a\r\xB7V[_\x83\x81R`\x07` R`@\x90 T\x90\x91P\x80\x82\x14a*\x0FW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x07\x90\x91R\x90 \x81\x90U[P_\x91\x82R`\x07` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x06\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\x08T_\x90a*R\x90`\x01\x90a4\xFBV[_\x83\x81R`\t` R`@\x81 T`\x08\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a*yWa*ya4\xBDV[\x90_R` _ \x01T\x90P\x80`\x08\x83\x81T\x81\x10a*\x98Wa*\x98a4\xBDV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\t\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\x08\x80T\x80a*\xCFWa*\xCFa5\xFDV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPV[_`\x01a*\xF4\x84a\r\xB7V[a*\xFE\x91\x90a4\xFBV[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x07\x90R\x91\x90\x91 \x91\x90\x91UPV[\x82T_\x90\x81\x90\x80\x15a,RW_a+R\x87a\x19\x01`\x01\x85a4\xFBV[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a+\xA6W`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a+\xF2W\x84a+\xC9\x88a\x19\x01`\x01\x86a4\xFBV[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua,BV[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa's\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a'sV[a,\xB7\x83\x83\x83a-\x10V[a\x0CyW`\x01`\x01`\xA0\x1B\x03\x83\x16a,\xE5W`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\x87V[_`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x17=WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a-IWPa-I\x84\x84a\x15\xCEV[\x80a\x17=WPP_\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[P\x80Ta-}\x90a3\x87V[_\x82U\x80`\x1F\x10a-\x8CWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\r%\x91\x90[\x80\x82\x11\x15a\x18}W_\x81U`\x01\x01a-\xA4V[__\x83`\x1F\x84\x01\x12a-\xC7W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xDDW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-\xF4W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a.\x0EW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a.#W__\xFD[a./\x87\x82\x88\x01a-\xB7V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.MW__\xFD[a.Y\x87\x82\x88\x01a-\xB7V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r%W__\xFD[_` \x82\x84\x03\x12\x15a.\x8AW__\xFD[\x815a\x1A\xC1\x81a.eV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a.\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.\x9DV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x1A\xC1` \x83\x01\x84a.\x95V[_` \x82\x84\x03\x12\x15a.\xFAW__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r%W__\xFD[__`@\x83\x85\x03\x12\x15a/&W__\xFD[\x825a/1\x81a/\x01V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a/QW__\xFD[\x835a/\\\x81a/\x01V[\x92P` \x84\x015a/l\x81a/\x01V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a/\x8EW__\xFD[\x825\x91P` \x83\x015a/\xA0\x81a/\x01V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a/\xBBW__\xFD[\x815a\x1A\xC1\x81a/\x01V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a/\xE4`\xE0\x83\x01\x89a.\x95V[\x82\x81\x03`@\x84\x01Ra/\xF6\x81\x89a.\x95V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a0KW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0-V[P\x90\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a0mW__\xFD[\x825a0x\x81a/\x01V[\x91P` \x83\x015\x80\x15\x15\x81\x14a/\xA0W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a0\xB9Wa0\xB9a0\x8CV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a0\xE7Wa0\xE7a0\x8CV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a0\xFEW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1$W__\xFD[a\x1A\xC1\x83\x835` \x85\x01a0\xA0V[____`\x80\x85\x87\x03\x12\x15a1FW__\xFD[\x845a1Q\x81a/\x01V[\x93P` \x85\x015a1a\x81a/\x01V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x82W__\xFD[a1\x8E\x87\x82\x88\x01a1\x15V[\x91PP\x92\x95\x91\x94P\x92PV[______`\xC0\x87\x89\x03\x12\x15a1\xAFW__\xFD[\x865a1\xBA\x81a/\x01V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a1\xDDW__\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_` \x82\x84\x03\x12\x15a2\x07W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x1A\xC1W__\xFD[`\x01`\x01`@\x1B\x03\x84\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90a2O\x90\x83\x01\x84a.\x95V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a2hW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2}W__\xFD[a\x17=\x84\x82\x85\x01a1\x15V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x17=`\x80\x84\x01\x82a.\x95V[__`@\x83\x85\x03\x12\x15a2\xD9W__\xFD[\x825a2\xE4\x81a/\x01V[\x91P` \x83\x015a/\xA0\x81a/\x01V[___``\x84\x86\x03\x12\x15a3\x06W__\xFD[\x835a3\x11\x81a/\x01V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a32W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3BW__\xFD[a3Q\x86\x825` \x84\x01a0\xA0V[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a3\x80Wa3\x80a3[V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a3\x9BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a3\xB9WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0CyW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a3\xE4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1C}W_\x81U`\x01\x01a3\xF0V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x1CWa4\x1Ca0\x8CV[a40\x81a4*\x84Ta3\x87V[\x84a3\xBFV[` `\x1F\x82\x11`\x01\x81\x14a4bW_\x83\x15a4KWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1C}V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a4\x91W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4qV[P\x84\x82\x10\x15a4\xAEW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a4\xF2Wa4\xF2a3[V[`\x01\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\nkWa\nka3[V[\x80\x82\x01\x80\x82\x11\x15a\nkWa\nka3[V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a5S\x90\x83\x01\x84a.\x95V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a5mW__\xFD[\x81Qa\x1A\xC1\x81a.eV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x82a5\xBAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\nkWa\nka3[V[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\nkWa\nka3[V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 k\xC5\xD5\xE6\xE5\xA6\x03\xEE\xBE\x85\xC9\xECz\xF3%\xBE\xDD\x95\x07\xE3\xE2\x8DB\xADc\xA5dG2\xA7(\xAAdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061028a575f3560e01c806370a0823111610155578063a22cb465116100be578063d547741f11610078578063d547741f1461084e578063e31e07881461086d578063e328ed7714610880578063e63ab1e9146108ac578063e985e9c5146108df578063f7ee944c146108fe575f5ffd5b8063a22cb46514610785578063b88d4fde146107a4578063c3cda520146107c3578063c4d66de8146107e2578063c87b56dd14610801578063ce28961214610820575f5ffd5b8063913b1fbf1161010f578063913b1fbf146106c757806391d14854146106fe57806391ddadf41461071d57806395d89b411461073f5780639ab24eb014610753578063a217fddf14610772575f5ffd5b806370a08231146105fb5780637ecebe001461061a5780638456cb591461064e57806384b0196e146106625780638d69e95e146106895780638e539e8c146106a8575f5ffd5b80633383abfe116101f75780634bf5d7e9116101b15780634bf5d7e91461051a5780634f6ccce714610550578063587cde1e1461056f5780635c19a95c146105a65780635c975abb146105c55780636352211e146105dc575f5ffd5b80633383abfe1461045657806336568abe1461048a5780633a46b1a8146104a95780633f4ba83a146104c857806342842e0e146104dc57806342966c68146104fb575f5ffd5b806318160ddd1161024857806318160ddd1461037a5780631bc6ae8a1461039857806323b872dd146103cb578063248a9ca3146103ea5780632f2ff15d146104185780632f745c5914610437575f5ffd5b806273e1d71461028e57806301ffc9a7146102af57806306fdde03146102e3578063081812fc14610304578063095ea7b31461033b578063158ef93e1461035a575b5f5ffd5b348015610299575f5ffd5b506102ad6102a8366004612dfb565b61091d565b005b3480156102ba575f5ffd5b506102ce6102c9366004612e7a565b610a61565b60405190151581526020015b60405180910390f35b3480156102ee575f5ffd5b506102f7610a71565b6040516102da9190612ed8565b34801561030f575f5ffd5b5061032361031e366004612eea565b610b00565b6040516001600160a01b0390911681526020016102da565b348015610346575f5ffd5b506102ad610355366004612f15565b610b27565b348015610365575f5ffd5b506016546102ce90600160a01b900460ff1681565b348015610385575f5ffd5b506008545b6040519081526020016102da565b3480156103a3575f5ffd5b5061038a7fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b81565b3480156103d6575f5ffd5b506102ad6103e5366004612f3f565b610b36565b3480156103f5575f5ffd5b5061038a610404366004612eea565b5f908152600b602052604090206001015490565b348015610423575f5ffd5b506102ad610432366004612f7d565b610bbf565b348015610442575f5ffd5b5061038a610451366004612f15565b610be3565b348015610461575f5ffd5b5061038a610470366004612fab565b6001600160a01b03165f9081526013602052604090205490565b348015610495575f5ffd5b506102ad6104a4366004612f7d565b610c46565b3480156104b4575f5ffd5b5061038a6104c3366004612f15565b610c7e565b3480156104d3575f5ffd5b506102ad610cf3565b3480156104e7575f5ffd5b506102ad6104f6366004612f3f565b610d28565b348015610506575f5ffd5b506102ad610515366004612eea565b610d42565b348015610525575f5ffd5b5060408051808201909152600e81526d06d6f64653d74696d657374616d760941b60208201526102f7565b34801561055b575f5ffd5b5061038a61056a366004612eea565b610d4d565b34801561057a575f5ffd5b50610323610589366004612fab565b6001600160a01b039081165f908152600f60205260409020541690565b3480156105b1575f5ffd5b506102ad6105c0366004612fab565b610da2565b3480156105d0575f5ffd5b50600a5460ff166102ce565b3480156105e7575f5ffd5b506103236105f6366004612eea565b610dad565b348015610606575f5ffd5b5061038a610615366004612fab565b610db7565b348015610625575f5ffd5b5061038a610634366004612fab565b6001600160a01b03165f908152600e602052604090205490565b348015610659575f5ffd5b506102ad610dfc565b34801561066d575f5ffd5b50610676610e2e565b6040516102da9796959493929190612fc6565b348015610694575f5ffd5b50601654610323906001600160a01b031681565b3480156106b3575f5ffd5b5061038a6106c2366004612eea565b610e70565b3480156106d2575f5ffd5b506106e66106e1366004612f15565b610ecf565b6040516001600160401b0390911681526020016102da565b348015610709575f5ffd5b506102ce610718366004612f7d565b610f16565b348015610728575f5ffd5b5060405165ffffffffffff421681526020016102da565b34801561074a575f5ffd5b506102f7610f40565b34801561075e575f5ffd5b5061038a61076d366004612fab565b610f4f565b34801561077d575f5ffd5b5061038a5f81565b348015610790575f5ffd5b506102ad61079f36600461305c565b610f7e565b3480156107af575f5ffd5b506102ad6107be366004613133565b610f89565b3480156107ce575f5ffd5b506102ad6107dd36600461319a565b610fa0565b3480156107ed575f5ffd5b506102ad6107fc366004612fab565b61105c565b34801561080c575f5ffd5b506102f761081b366004612eea565b61115f565b34801561082b575f5ffd5b5061083f61083a3660046131f7565b611260565b6040516102da9392919061321d565b348015610859575f5ffd5b506102ad610868366004612f7d565b61131e565b6102ad61087b366004613258565b611342565b34801561088b575f5ffd5b5061089f61089a3660046131f7565b6114d5565b6040516102da9190613289565b3480156108b7575f5ffd5b5061038a7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a81565b3480156108ea575f5ffd5b506102ce6108f93660046132c8565b6115ce565b348015610909575f5ffd5b506106e6610918366004612f15565b6115fb565b6109477fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b33610f16565b6109905760405162461bcd60e51b815260206004820152601560248201527427b7363c9039b2b93b34b1b290383937bb34b232b960591b60448201526064015b60405180910390fd5b5f808061099f868801886132f4565b9250925092505f60155f8154809291906109b89061336f565b9190505590506109c884826116af565b5f8181526017602052604090206109df8382613403565b506001600160401b0383165f90815260126020526040812080546001600160e01b031916815590610a136001830182612d71565b505080846001600160a01b03167fd35bb95e09c04b219e35047ce7b7b300e3384264ef84a40456943dbc0fc17c1484604051610a4f9190612ed8565b60405180910390a35050505050505050565b5f610a6b826116c8565b92915050565b60605f8054610a7f90613387565b80601f0160208091040260200160405190810160405280929190818152602001828054610aab90613387565b8015610af65780601f10610acd57610100808354040283529160200191610af6565b820191905f5260205f20905b815481529060010190602001808311610ad957829003601f168201915b5050505050905090565b5f610b0a826116ec565b505f828152600460205260409020546001600160a01b0316610a6b565b610b32828233611724565b5050565b6001600160a01b038216610b5f57604051633250574960e11b81525f6004820152602401610987565b5f610b6b838333611731565b9050836001600160a01b0316816001600160a01b031614610bb9576040516364283d7b60e01b81526001600160a01b0380861660048301526024820184905282166044820152606401610987565b50505050565b5f828152600b6020526040902060010154610bd981611745565b610bb9838361174f565b5f610bed83610db7565b8210610c1e5760405163295f44f760e21b81526001600160a01b038416600482015260248101839052604401610987565b506001600160a01b03919091165f908152600660209081526040808320938352929052205490565b6001600160a01b0381163314610c6f5760405163334bd91960e11b815260040160405180910390fd5b610c7982826117e0565b505050565b5f4265ffffffffffff81168310610cb957604051637669fc0f60e11b81526004810184905265ffffffffffff82166024820152604401610987565b610ce2610cc58461184b565b6001600160a01b0386165f90815260106020526040902090611881565b6001600160d01b0316949350505050565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610d1d81611745565b610d25611931565b50565b610c7983838360405180602001604052805f815250610f89565b610b325f8233611731565b5f610d5760085490565b8210610d7f5760405163295f44f760e21b81525f600482015260248101839052604401610987565b60088281548110610d9257610d926134bd565b905f5260205f2001549050919050565b33610b328183611983565b5f610a6b826116ec565b5f6001600160a01b038216610de1576040516322718ad960e21b81525f6004820152602401610987565b506001600160a01b03165f9081526003602052604090205490565b7f65d7a28e3265b37a6474929f336521b332c1681b933f6cb9f3376673440d862a610e2681611745565b610d256119f4565b5f6060805f5f5f6060610e3f611a31565b610e47611a63565b604080515f80825260208201909252600f60f81b9b939a50919850469750309650945092509050565b5f4265ffffffffffff81168310610eab57604051637669fc0f60e11b81526004810184905265ffffffffffff82166024820152604401610987565b610ebf610eb78461184b565b601190611881565b6001600160d01b03169392505050565b6013602052815f5260405f208181548110610ee8575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b5f918252600b602090815260408084206001600160a01b0393909316845291905290205460ff1690565b606060018054610a7f90613387565b6001600160a01b0381165f908152601060205260408120610f6f90611a90565b6001600160d01b031692915050565b610b32338383611ac8565b610f94848484610b36565b610bb984848484611b5e565b83421115610fc457604051632341d78760e11b815260048101859052602401610987565b604080517fe48329057bfd03d55e49b547132e39cffd9c1820ad7b9d4c5307691425d15adf60208201526001600160a01b0388169181019190915260608101869052608081018590525f9061103d906110359060a00160405160208183030381529060405280519060200120611c84565b858585611cb0565b90506110498187611cdc565b6110538188611983565b50505050505050565b5f61106681611745565b601654600160a01b900460ff16156110b65760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610987565b6001600160a01b03821661110c5760405162461bcd60e51b815260206004820152601860248201527f496e76616c696420736572766963652070726f766964657200000000000000006044820152606401610987565b6111367fd8c0b0264fb5d225f4ba2fb92454d9f4f912be4d27b355562e6ae90ce2f5e74b8361174f565b5050601680546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b5f818152600260205260409020546060906001600160a01b03166111c55760405162461bcd60e51b815260206004820152601f60248201527f55524920717565727920666f72206e6f6e6578697374656e7420746f6b656e006044820152606401610987565b5f82815260176020526040902080546111dd90613387565b80601f016020809104026020016040519081016040528092919081815260200182805461120990613387565b80156112545780601f1061122b57610100808354040283529160200191611254565b820191905f5260205f20905b81548152906001019060200180831161123757829003601f168201915b50505050509050919050565b60126020525f9081526040902080546001820180546001600160401b03831693600160401b9093046001600160a01b031692919061129d90613387565b80601f01602080910402602001604051908101604052809291908181526020018280546112c990613387565b80156113145780601f106112eb57610100808354040283529160200191611314565b820191905f5260205f20905b8154815290600101906020018083116112f757829003601f168201915b5050505050905083565b5f828152600b602052604090206001015461133881611745565b610bb983836117e0565b3467016345785d8a0000146113995760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e3120455448006044820152606401610987565b601480545f916001600160401b0390911690826113b5836134d1565b82546001600160401b039182166101009390930a928302928202191691909117909155604080516060810182528383168082523360208084019182528385018981525f9384526012909152939091208251815492516001600160a01b0316600160401b026001600160e01b031990931695169490941717835590519293509182919060018201906114469082613403565b5050335f818152601360209081526040808320805460018101825590845291909220600482040180546001600160401b0380891660086003909516949094026101000a8481029102199091161790559051919250907ff3f411d853486b9f53da63009a21cd284ea18a800d4de55ce5bd935d197e4cf1906114c8908790612ed8565b60405180910390a3505050565b60408051606080820183525f8083526020830152918101919091526001600160401b038281165f90815260126020908152604091829020825160608101845281549485168152600160401b9094046001600160a01b031691840191909152600181018054919284019161154790613387565b80601f016020809104026020016040519081016040528092919081815260200182805461157390613387565b80156115be5780601f10611595576101008083540402835291602001916115be565b820191905f5260205f20905b8154815290600101906020018083116115a157829003601f168201915b5050505050815250509050919050565b6001600160a01b039182165f90815260056020908152604080832093909416825291909152205460ff1690565b6001600160a01b0382165f9081526013602052604081205482106116575760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610987565b6001600160a01b0383165f908152601360205260409020805483908110611680576116806134bd565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b0316905092915050565b610b32828260405180602001604052805f815250611d2e565b5f6001600160e01b03198216637965db0b60e01b1480610a6b5750610a6b82611d44565b5f818152600260205260408120546001600160a01b031680610a6b57604051637e27328960e01b815260048101849052602401610987565b610c798383836001611d68565b5f61173d848484611e6c565b949350505050565b610d258133611e87565b5f61175a8383610f16565b6117d9575f838152600b602090815260408083206001600160a01b03861684529091529020805460ff191660011790556117913390565b6001600160a01b0316826001600160a01b0316847f2f8788117e7eff1d82e926ec794901d17c78024a50270940304540a733656f0d60405160405180910390a4506001610a6b565b505f610a6b565b5f6117eb8383610f16565b156117d9575f838152600b602090815260408083206001600160a01b0386168085529252808320805460ff1916905551339286917ff6391f5c32d9c69d2a47ea670b442974b53935d1edc7fd64eb21e047a839171b9190a4506001610a6b565b5f65ffffffffffff82111561187d576040516306dfcc6560e41b81526030600482015260248101839052604401610987565b5090565b81545f90818160058111156118dd575f61189a84611ec0565b6118a490856134fb565b5f8881526020902090915081015465ffffffffffff90811690871610156118cd578091506118db565b6118d881600161350e565b92505b505b5f6118ea87878585611fa4565b905080156119245761190e876119016001846134fb565b5f91825260209091200190565b54600160301b90046001600160d01b0316611926565b5f5b979650505050505050565b611939612003565b600a805460ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a1565b6001600160a01b038281165f818152600f602052604080822080548686166001600160a01b0319821681179092559151919094169392849290917f3134e8a2e6d97e929a7e54011ea5485d7d196dd5f0ba4d4ef95803e8e3fc257f9190a4610c7981836119ef86612028565b612032565b6119fc61219b565b600a805460ff191660011790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586119663390565b6060611a5e7f0000000000000000000000000000000000000000000000000000000000000000600c6121bf565b905090565b6060611a5e7f0000000000000000000000000000000000000000000000000000000000000000600d6121bf565b80545f908015611abf57611aa9836119016001846134fb565b54600160301b90046001600160d01b0316611ac1565b5f5b9392505050565b6001600160a01b038216611afa57604051630b61174360e31b81526001600160a01b0383166004820152602401610987565b6001600160a01b038381165f81815260056020908152604080832094871680845294825291829020805460ff191686151590811790915591519182527f17307eab39ab6107e8899845ad3d59bd9653f200f220920489ca2b5937696c3191016114c8565b6001600160a01b0383163b15610bb957604051630a85bd0160e11b81526001600160a01b0384169063150b7a0290611ba0903390889087908790600401613521565b6020604051808303815f875af1925050508015611bda575060408051601f3d908101601f19168201909252611bd79181019061355d565b60015b611c41573d808015611c07576040519150601f19603f3d011682016040523d82523d5f602084013e611c0c565b606091505b5080515f03611c3957604051633250574960e11b81526001600160a01b0385166004820152602401610987565b805181602001fd5b6001600160e01b03198116630a85bd0160e11b14611c7d57604051633250574960e11b81526001600160a01b0385166004820152602401610987565b5050505050565b5f610a6b611c90612268565b8360405161190160f01b8152600281019290925260228201526042902090565b5f5f5f5f611cc088888888612391565b925092509250611cd08282612459565b50909695505050505050565b6001600160a01b0382165f908152600e60205260409020805460018101909155818114610c79576040516301d4b62360e61b81526001600160a01b038416600482015260248101829052604401610987565b611d388383612511565b610c795f848484611b5e565b5f6001600160e01b0319821663780e9d6360e01b1480610a6b5750610a6b82612572565b8080611d7c57506001600160a01b03821615155b15611e3d575f611d8b846116ec565b90506001600160a01b03831615801590611db75750826001600160a01b0316816001600160a01b031614155b8015611dca5750611dc881846115ce565b155b15611df35760405163a9fbf51f60e01b81526001600160a01b0384166004820152602401610987565b8115611e3b5783856001600160a01b0316826001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b92560405160405180910390a45b505b50505f90815260046020526040902080546001600160a01b0319166001600160a01b0392909216919091179055565b5f5f611e798585856125c1565b905061173d818660016125d5565b611e918282610f16565b610b325760405163e2517d3f60e01b81526001600160a01b038216600482015260248101839052604401610987565b5f815f03611ecf57505f919050565b5f6001611edb8461264a565b901c6001901b90506001818481611ef457611ef4613578565b048201901c90506001818481611f0c57611f0c613578565b048201901c90506001818481611f2457611f24613578565b048201901c90506001818481611f3c57611f3c613578565b048201901c90506001818481611f5457611f54613578565b048201901c90506001818481611f6c57611f6c613578565b048201901c90506001818481611f8457611f84613578565b048201901c9050611ac181828581611f9e57611f9e613578565b046126dd565b5f5b81831015611ffb575f611fb984846126f2565b5f8781526020902090915065ffffffffffff86169082015465ffffffffffff161115611fe757809250611ff5565b611ff281600161350e565b93505b50611fa6565b509392505050565b600a5460ff1661202657604051638dfc202b60e01b815260040160405180910390fd5b565b5f610a6b82610db7565b816001600160a01b0316836001600160a01b03161415801561205357505f81115b15610c79576001600160a01b038316156120fa576001600160a01b0383165f90815260106020526040812081906120959061270c61209086612717565b61274a565b6001600160d01b031691506001600160d01b03169150846001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a72483836040516120ef929190918252602082015260400190565b60405180910390a250505b6001600160a01b03821615610c79576001600160a01b0382165f90815260106020526040812081906121329061277b61209086612717565b6001600160d01b031691506001600160d01b03169150836001600160a01b03167fdec2bacdd2f05b59de34da9b523dff8be42e5e38e818c82fdb0bae774387a724838360405161218c929190918252602082015260400190565b60405180910390a25050505050565b600a5460ff16156120265760405163d93c066560e01b815260040160405180910390fd5b606060ff83146121d9576121d283612786565b9050610a6b565b8180546121e590613387565b80601f016020809104026020016040519081016040528092919081815260200182805461221190613387565b801561225c5780601f106122335761010080835404028352916020019161225c565b820191905f5260205f20905b81548152906001019060200180831161223f57829003601f168201915b50505050509050610a6b565b5f306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161480156122c057507f000000000000000000000000000000000000000000000000000000000000000046145b156122ea57507f000000000000000000000000000000000000000000000000000000000000000090565b611a5e604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f60208201527f0000000000000000000000000000000000000000000000000000000000000000918101919091527f000000000000000000000000000000000000000000000000000000000000000060608201524660808201523060a08201525f9060c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411156123ca57505f9150600390508261244f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561241b573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b03811661244657505f92506001915082905061244f565b92505f91508190505b9450945094915050565b5f82600381111561246c5761246c61358c565b03612475575050565b60018260038111156124895761248961358c565b036124a75760405163f645eedf60e01b815260040160405180910390fd5b60028260038111156124bb576124bb61358c565b036124dc5760405163fce698f760e01b815260048101829052602401610987565b60038260038111156124f0576124f061358c565b03610b32576040516335e2f38360e21b815260048101829052602401610987565b6001600160a01b03821661253a57604051633250574960e11b81525f6004820152602401610987565b5f61254683835f611731565b90506001600160a01b03811615610c79576040516339e3563760e11b81525f6004820152602401610987565b5f6001600160e01b031982166380ac58cd60e01b14806125a257506001600160e01b03198216635b5e139f60e01b145b80610a6b57506301ffc9a760e01b6001600160e01b0319831614610a6b565b5f6125ca61219b565b61173d8484846127c3565b6001600160a01b0383166125f7576125f4601161277b61209084612717565b50505b6001600160a01b03821661261957612616601161270c61209084612717565b50505b6001600160a01b038381165f908152600f6020526040808220548584168352912054610c7992918216911683612032565b5f80608083901c1561265e57608092831c92015b604083901c1561267057604092831c92015b602083901c1561268257602092831c92015b601083901c1561269457601092831c92015b600883901c156126a657600892831c92015b600483901c156126b857600492831c92015b600283901c156126ca57600292831c92015b600183901c15610a6b5760010192915050565b5f8183106126eb5781611ac1565b5090919050565b5f61270060028484186135a0565b611ac19084841661350e565b5f611ac182846135bf565b5f6001600160d01b0382111561187d576040516306dfcc6560e41b815260d0600482015260248101839052604401610987565b5f8061276e4261276661275c88611a90565b868863ffffffff16565b87919061288e565b915091505b935093915050565b5f611ac182846135de565b60605f6127928361289b565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f5f6127d08585856128c2565b90506001600160a01b03811661282c5761282784600880545f838152600960205260408120829055600182018355919091527ff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a84c8169e8b636ee30155565b61284f565b846001600160a01b0316816001600160a01b03161461284f5761284f81856129b4565b6001600160a01b03851661286b5761286684612a41565b61173d565b846001600160a01b0316816001600160a01b03161461173d5761173d8585612ae8565b5f8061276e858585612b36565b5f60ff8216601f811115610a6b57604051632cd44ac360e21b815260040160405180910390fd5b5f828152600260205260408120546001600160a01b03908116908316156128ee576128ee818486612cac565b6001600160a01b03811615612928576129095f855f5f611d68565b6001600160a01b0381165f90815260036020526040902080545f190190555b6001600160a01b03851615612956576001600160a01b0385165f908152600360205260409020805460010190555b5f8481526002602052604080822080546001600160a01b0319166001600160a01b0389811691821790925591518793918516917fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef91a4949350505050565b5f6129be83610db7565b5f83815260076020526040902054909150808214612a0f576001600160a01b0384165f9081526006602090815260408083208584528252808320548484528184208190558352600790915290208190555b505f9182526007602090815260408084208490556001600160a01b039094168352600681528383209183525290812055565b6008545f90612a52906001906134fb565b5f8381526009602052604081205460088054939450909284908110612a7957612a796134bd565b905f5260205f20015490508060088381548110612a9857612a986134bd565b5f918252602080832090910192909255828152600990915260408082208490558582528120556008805480612acf57612acf6135fd565b600190038181905f5260205f20015f9055905550505050565b5f6001612af484610db7565b612afe91906134fb565b6001600160a01b039093165f908152600660209081526040808320868452825280832085905593825260079052919091209190915550565b82545f9081908015612c52575f612b52876119016001856134fb565b60408051808201909152905465ffffffffffff808216808452600160301b9092046001600160d01b031660208401529192509087161015612ba657604051632520601d60e01b815260040160405180910390fd5b805165ffffffffffff808816911603612bf25784612bc9886119016001866134fb565b80546001600160d01b0392909216600160301b0265ffffffffffff909216919091179055612c42565b6040805180820190915265ffffffffffff80881682526001600160d01b0380881660208085019182528b54600181018d555f8d81529190912094519151909216600160301b029216919091179101555b6020015192508391506127739050565b50506040805180820190915265ffffffffffff80851682526001600160d01b0380851660208085019182528854600181018a555f8a815291822095519251909316600160301b029190931617920191909155905081612773565b612cb7838383612d10565b610c79576001600160a01b038316612ce557604051637e27328960e01b815260048101829052602401610987565b60405163177e802f60e01b81526001600160a01b038316600482015260248101829052604401610987565b5f6001600160a01b0383161580159061173d5750826001600160a01b0316846001600160a01b03161480612d495750612d4984846115ce565b8061173d5750505f908152600460205260409020546001600160a01b03908116911614919050565b508054612d7d90613387565b5f825580601f10612d8c575050565b601f0160209004905f5260205f2090810190610d2591905b8082111561187d575f8155600101612da4565b5f5f83601f840112612dc7575f5ffd5b5081356001600160401b03811115612ddd575f5ffd5b602083019150836020828501011115612df4575f5ffd5b9250929050565b5f5f5f5f60408587031215612e0e575f5ffd5b84356001600160401b03811115612e23575f5ffd5b612e2f87828801612db7565b90955093505060208501356001600160401b03811115612e4d575f5ffd5b612e5987828801612db7565b95989497509550505050565b6001600160e01b031981168114610d25575f5ffd5b5f60208284031215612e8a575f5ffd5b8135611ac181612e65565b5f81518084525f5b81811015612eb957602081850181015186830182015201612e9d565b505f602082860101526020601f19601f83011685010191505092915050565b602081525f611ac16020830184612e95565b5f60208284031215612efa575f5ffd5b5035919050565b6001600160a01b0381168114610d25575f5ffd5b5f5f60408385031215612f26575f5ffd5b8235612f3181612f01565b946020939093013593505050565b5f5f5f60608486031215612f51575f5ffd5b8335612f5c81612f01565b92506020840135612f6c81612f01565b929592945050506040919091013590565b5f5f60408385031215612f8e575f5ffd5b823591506020830135612fa081612f01565b809150509250929050565b5f60208284031215612fbb575f5ffd5b8135611ac181612f01565b60ff60f81b8816815260e060208201525f612fe460e0830189612e95565b8281036040840152612ff68189612e95565b606084018890526001600160a01b038716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b8181101561304b57835183526020938401939092019160010161302d565b50909b9a5050505050505050505050565b5f5f6040838503121561306d575f5ffd5b823561307881612f01565b915060208301358015158114612fa0575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b038411156130b9576130b961308c565b50604051601f19601f85018116603f011681018181106001600160401b03821117156130e7576130e761308c565b6040528381529050808284018510156130fe575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f830112613124575f5ffd5b611ac1838335602085016130a0565b5f5f5f5f60808587031215613146575f5ffd5b843561315181612f01565b9350602085013561316181612f01565b92506040850135915060608501356001600160401b03811115613182575f5ffd5b61318e87828801613115565b91505092959194509250565b5f5f5f5f5f5f60c087890312156131af575f5ffd5b86356131ba81612f01565b95506020870135945060408701359350606087013560ff811681146131dd575f5ffd5b9598949750929560808101359460a0909101359350915050565b5f60208284031215613207575f5ffd5b81356001600160401b0381168114611ac1575f5ffd5b6001600160401b03841681526001600160a01b03831660208201526060604082018190525f9061324f90830184612e95565b95945050505050565b5f60208284031215613268575f5ffd5b81356001600160401b0381111561327d575f5ffd5b61173d84828501613115565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f604083015160608084015261173d6080840182612e95565b5f5f604083850312156132d9575f5ffd5b82356132e481612f01565b91506020830135612fa081612f01565b5f5f5f60608486031215613306575f5ffd5b833561331181612f01565b92506020840135915060408401356001600160401b03811115613332575f5ffd5b8401601f81018613613342575f5ffd5b613351868235602084016130a0565b9150509250925092565b634e487b7160e01b5f52601160045260245ffd5b5f600182016133805761338061335b565b5060010190565b600181811c9082168061339b57607f821691505b6020821081036133b957634e487b7160e01b5f52602260045260245ffd5b50919050565b601f821115610c7957805f5260205f20601f840160051c810160208510156133e45750805b601f840160051c820191505b81811015611c7d575f81556001016133f0565b81516001600160401b0381111561341c5761341c61308c565b6134308161342a8454613387565b846133bf565b6020601f821160018114613462575f831561344b5750848201515b5f19600385901b1c1916600184901b178455611c7d565b5f84815260208120601f198516915b828110156134915787850151825560209485019460019092019101613471565b50848210156134ae57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b5f52603260045260245ffd5b5f6001600160401b0382166001600160401b0381036134f2576134f261335b565b60010192915050565b81810381811115610a6b57610a6b61335b565b80820180821115610a6b57610a6b61335b565b6001600160a01b03858116825284166020820152604081018390526080606082018190525f9061355390830184612e95565b9695505050505050565b5f6020828403121561356d575f5ffd5b8151611ac181612e65565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b5f826135ba57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160d01b038281168282160390811115610a6b57610a6b61335b565b6001600160d01b038181168382160190811115610a6b57610a6b61335b565b634e487b7160e01b5f52603160045260245ffdfea26469706673582212206bc5d5e6e5a603eebe85c9ec7af325bedd9507e3e28d42ad63a5644732a728aa64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x8AW_5`\xE0\x1C\x80cp\xA0\x821\x11a\x01UW\x80c\xA2,\xB4e\x11a\0\xBEW\x80c\xD5Gt\x1F\x11a\0xW\x80c\xD5Gt\x1F\x14a\x08NW\x80c\xE3\x1E\x07\x88\x14a\x08mW\x80c\xE3(\xEDw\x14a\x08\x80W\x80c\xE6:\xB1\xE9\x14a\x08\xACW\x80c\xE9\x85\xE9\xC5\x14a\x08\xDFW\x80c\xF7\xEE\x94L\x14a\x08\xFEW__\xFD[\x80c\xA2,\xB4e\x14a\x07\x85W\x80c\xB8\x8DO\xDE\x14a\x07\xA4W\x80c\xC3\xCD\xA5 \x14a\x07\xC3W\x80c\xC4\xD6m\xE8\x14a\x07\xE2W\x80c\xC8{V\xDD\x14a\x08\x01W\x80c\xCE(\x96\x12\x14a\x08 W__\xFD[\x80c\x91;\x1F\xBF\x11a\x01\x0FW\x80c\x91;\x1F\xBF\x14a\x06\xC7W\x80c\x91\xD1HT\x14a\x06\xFEW\x80c\x91\xDD\xAD\xF4\x14a\x07\x1DW\x80c\x95\xD8\x9BA\x14a\x07?W\x80c\x9A\xB2N\xB0\x14a\x07SW\x80c\xA2\x17\xFD\xDF\x14a\x07rW__\xFD[\x80cp\xA0\x821\x14a\x05\xFBW\x80c~\xCE\xBE\0\x14a\x06\x1AW\x80c\x84V\xCBY\x14a\x06NW\x80c\x84\xB0\x19n\x14a\x06bW\x80c\x8Di\xE9^\x14a\x06\x89W\x80c\x8ES\x9E\x8C\x14a\x06\xA8W__\xFD[\x80c3\x83\xAB\xFE\x11a\x01\xF7W\x80cK\xF5\xD7\xE9\x11a\x01\xB1W\x80cK\xF5\xD7\xE9\x14a\x05\x1AW\x80cOl\xCC\xE7\x14a\x05PW\x80cX|\xDE\x1E\x14a\x05oW\x80c\\\x19\xA9\\\x14a\x05\xA6W\x80c\\\x97Z\xBB\x14a\x05\xC5W\x80ccR!\x1E\x14a\x05\xDCW__\xFD[\x80c3\x83\xAB\xFE\x14a\x04VW\x80c6V\x8A\xBE\x14a\x04\x8AW\x80c:F\xB1\xA8\x14a\x04\xA9W\x80c?K\xA8:\x14a\x04\xC8W\x80cB\x84.\x0E\x14a\x04\xDCW\x80cB\x96lh\x14a\x04\xFBW__\xFD[\x80c\x18\x16\r\xDD\x11a\x02HW\x80c\x18\x16\r\xDD\x14a\x03zW\x80c\x1B\xC6\xAE\x8A\x14a\x03\x98W\x80c#\xB8r\xDD\x14a\x03\xCBW\x80c$\x8A\x9C\xA3\x14a\x03\xEAW\x80c//\xF1]\x14a\x04\x18W\x80c/t\\Y\x14a\x047W__\xFD[\x80bs\xE1\xD7\x14a\x02\x8EW\x80c\x01\xFF\xC9\xA7\x14a\x02\xAFW\x80c\x06\xFD\xDE\x03\x14a\x02\xE3W\x80c\x08\x18\x12\xFC\x14a\x03\x04W\x80c\t^\xA7\xB3\x14a\x03;W\x80c\x15\x8E\xF9>\x14a\x03ZW[__\xFD[4\x80\x15a\x02\x99W__\xFD[Pa\x02\xADa\x02\xA86`\x04a-\xFBV[a\t\x1DV[\0[4\x80\x15a\x02\xBAW__\xFD[Pa\x02\xCEa\x02\xC96`\x04a.zV[a\naV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEEW__\xFD[Pa\x02\xF7a\nqV[`@Qa\x02\xDA\x91\x90a.\xD8V[4\x80\x15a\x03\x0FW__\xFD[Pa\x03#a\x03\x1E6`\x04a.\xEAV[a\x0B\0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x03FW__\xFD[Pa\x02\xADa\x03U6`\x04a/\x15V[a\x0B'V[4\x80\x15a\x03eW__\xFD[P`\x16Ta\x02\xCE\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[4\x80\x15a\x03\x85W__\xFD[P`\x08T[`@Q\x90\x81R` \x01a\x02\xDAV[4\x80\x15a\x03\xA3W__\xFD[Pa\x03\x8A\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x81V[4\x80\x15a\x03\xD6W__\xFD[Pa\x02\xADa\x03\xE56`\x04a/?V[a\x0B6V[4\x80\x15a\x03\xF5W__\xFD[Pa\x03\x8Aa\x04\x046`\x04a.\xEAV[_\x90\x81R`\x0B` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04#W__\xFD[Pa\x02\xADa\x0426`\x04a/}V[a\x0B\xBFV[4\x80\x15a\x04BW__\xFD[Pa\x03\x8Aa\x04Q6`\x04a/\x15V[a\x0B\xE3V[4\x80\x15a\x04aW__\xFD[Pa\x03\x8Aa\x04p6`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x13` R`@\x90 T\x90V[4\x80\x15a\x04\x95W__\xFD[Pa\x02\xADa\x04\xA46`\x04a/}V[a\x0CFV[4\x80\x15a\x04\xB4W__\xFD[Pa\x03\x8Aa\x04\xC36`\x04a/\x15V[a\x0C~V[4\x80\x15a\x04\xD3W__\xFD[Pa\x02\xADa\x0C\xF3V[4\x80\x15a\x04\xE7W__\xFD[Pa\x02\xADa\x04\xF66`\x04a/?V[a\r(V[4\x80\x15a\x05\x06W__\xFD[Pa\x02\xADa\x05\x156`\x04a.\xEAV[a\rBV[4\x80\x15a\x05%W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81Rm\x06\xD6\xF6FS\xD7F\x96\xD6W7F\x16\xD7`\x94\x1B` \x82\x01Ra\x02\xF7V[4\x80\x15a\x05[W__\xFD[Pa\x03\x8Aa\x05j6`\x04a.\xEAV[a\rMV[4\x80\x15a\x05zW__\xFD[Pa\x03#a\x05\x896`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x0F` R`@\x90 T\x16\x90V[4\x80\x15a\x05\xB1W__\xFD[Pa\x02\xADa\x05\xC06`\x04a/\xABV[a\r\xA2V[4\x80\x15a\x05\xD0W__\xFD[P`\nT`\xFF\x16a\x02\xCEV[4\x80\x15a\x05\xE7W__\xFD[Pa\x03#a\x05\xF66`\x04a.\xEAV[a\r\xADV[4\x80\x15a\x06\x06W__\xFD[Pa\x03\x8Aa\x06\x156`\x04a/\xABV[a\r\xB7V[4\x80\x15a\x06%W__\xFD[Pa\x03\x8Aa\x0646`\x04a/\xABV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x0E` R`@\x90 T\x90V[4\x80\x15a\x06YW__\xFD[Pa\x02\xADa\r\xFCV[4\x80\x15a\x06mW__\xFD[Pa\x06va\x0E.V[`@Qa\x02\xDA\x97\x96\x95\x94\x93\x92\x91\x90a/\xC6V[4\x80\x15a\x06\x94W__\xFD[P`\x16Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x06\xB3W__\xFD[Pa\x03\x8Aa\x06\xC26`\x04a.\xEAV[a\x0EpV[4\x80\x15a\x06\xD2W__\xFD[Pa\x06\xE6a\x06\xE16`\x04a/\x15V[a\x0E\xCFV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x07\tW__\xFD[Pa\x02\xCEa\x07\x186`\x04a/}V[a\x0F\x16V[4\x80\x15a\x07(W__\xFD[P`@Qe\xFF\xFF\xFF\xFF\xFF\xFFB\x16\x81R` \x01a\x02\xDAV[4\x80\x15a\x07JW__\xFD[Pa\x02\xF7a\x0F@V[4\x80\x15a\x07^W__\xFD[Pa\x03\x8Aa\x07m6`\x04a/\xABV[a\x0FOV[4\x80\x15a\x07}W__\xFD[Pa\x03\x8A_\x81V[4\x80\x15a\x07\x90W__\xFD[Pa\x02\xADa\x07\x9F6`\x04a0\\V[a\x0F~V[4\x80\x15a\x07\xAFW__\xFD[Pa\x02\xADa\x07\xBE6`\x04a13V[a\x0F\x89V[4\x80\x15a\x07\xCEW__\xFD[Pa\x02\xADa\x07\xDD6`\x04a1\x9AV[a\x0F\xA0V[4\x80\x15a\x07\xEDW__\xFD[Pa\x02\xADa\x07\xFC6`\x04a/\xABV[a\x10\\V[4\x80\x15a\x08\x0CW__\xFD[Pa\x02\xF7a\x08\x1B6`\x04a.\xEAV[a\x11_V[4\x80\x15a\x08+W__\xFD[Pa\x08?a\x08:6`\x04a1\xF7V[a\x12`V[`@Qa\x02\xDA\x93\x92\x91\x90a2\x1DV[4\x80\x15a\x08YW__\xFD[Pa\x02\xADa\x08h6`\x04a/}V[a\x13\x1EV[a\x02\xADa\x08{6`\x04a2XV[a\x13BV[4\x80\x15a\x08\x8BW__\xFD[Pa\x08\x9Fa\x08\x9A6`\x04a1\xF7V[a\x14\xD5V[`@Qa\x02\xDA\x91\x90a2\x89V[4\x80\x15a\x08\xB7W__\xFD[Pa\x03\x8A\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*\x81V[4\x80\x15a\x08\xEAW__\xFD[Pa\x02\xCEa\x08\xF96`\x04a2\xC8V[a\x15\xCEV[4\x80\x15a\t\tW__\xFD[Pa\x06\xE6a\t\x186`\x04a/\x15V[a\x15\xFBV[a\tG\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K3a\x0F\x16V[a\t\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt'\xB76<\x909\xB2\xB9;4\xB1\xB2\x90897\xBB4\xB22\xB9`Y\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\t\x9F\x86\x88\x01\x88a2\xF4V[\x92P\x92P\x92P_`\x15_\x81T\x80\x92\x91\x90a\t\xB8\x90a3oV[\x91\x90PU\x90Pa\t\xC8\x84\x82a\x16\xAFV[_\x81\x81R`\x17` R`@\x90 a\t\xDF\x83\x82a4\x03V[P`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R`\x12` R`@\x81 \x80T`\x01`\x01`\xE0\x1B\x03\x19\x16\x81U\x90a\n\x13`\x01\x83\x01\x82a-qV[PP\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD3[\xB9^\t\xC0K!\x9E5\x04|\xE7\xB7\xB3\0\xE38Bd\xEF\x84\xA4\x04V\x94=\xBC\x0F\xC1|\x14\x84`@Qa\nO\x91\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPV[_a\nk\x82a\x16\xC8V[\x92\x91PPV[``_\x80Ta\n\x7F\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xAB\x90a3\x87V[\x80\x15a\n\xF6W\x80`\x1F\x10a\n\xCDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x0B\n\x82a\x16\xECV[P_\x82\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\nkV[a\x0B2\x82\x823a\x17$V[PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x0B_W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_a\x0Bk\x83\x833a\x171V[\x90P\x83`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xB9W`@Qcd(={`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16`D\x82\x01R`d\x01a\t\x87V[PPPPV[_\x82\x81R`\x0B` R`@\x90 `\x01\x01Ta\x0B\xD9\x81a\x17EV[a\x0B\xB9\x83\x83a\x17OV[_a\x0B\xED\x83a\r\xB7V[\x82\x10a\x0C\x1EW`@Qc)_D\xF7`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x93\x83R\x92\x90R T\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x0CoW`@Qc3K\xD9\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Cy\x82\x82a\x17\xE0V[PPPV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x0C\xB9W`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\x87V[a\x0C\xE2a\x0C\xC5\x84a\x18KV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x10` R`@\x90 \x90a\x18\x81V[`\x01`\x01`\xD0\x1B\x03\x16\x94\x93PPPPV[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\r\x1D\x81a\x17EV[a\r%a\x191V[PV[a\x0Cy\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x0F\x89V[a\x0B2_\x823a\x171V[_a\rW`\x08T\x90V[\x82\x10a\r\x7FW`@Qc)_D\xF7`\xE2\x1B\x81R_`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[`\x08\x82\x81T\x81\x10a\r\x92Wa\r\x92a4\xBDV[\x90_R` _ \x01T\x90P\x91\x90PV[3a\x0B2\x81\x83a\x19\x83V[_a\nk\x82a\x16\xECV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\r\xE1W`@Qc\"q\x8A\xD9`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x03` R`@\x90 T\x90V[\x7Fe\xD7\xA2\x8E2e\xB3zdt\x92\x9F3e!\xB32\xC1h\x1B\x93?l\xB9\xF37fsD\r\x86*a\x0E&\x81a\x17EV[a\r%a\x19\xF4V[_``\x80___``a\x0E?a\x1A1V[a\x0EGa\x1AcV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9B\x93\x9AP\x91\x98PF\x97P0\x96P\x94P\x92P\x90PV[_Be\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x83\x10a\x0E\xABW`@Qcvi\xFC\x0F`\xE1\x1B\x81R`\x04\x81\x01\x84\x90Re\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x01a\t\x87V[a\x0E\xBFa\x0E\xB7\x84a\x18KV[`\x11\x90a\x18\x81V[`\x01`\x01`\xD0\x1B\x03\x16\x93\x92PPPV[`\x13` R\x81_R`@_ \x81\x81T\x81\x10a\x0E\xE8W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[_\x91\x82R`\x0B` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T`\xFF\x16\x90V[```\x01\x80Ta\n\x7F\x90a3\x87V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x10` R`@\x81 a\x0Fo\x90a\x1A\x90V[`\x01`\x01`\xD0\x1B\x03\x16\x92\x91PPV[a\x0B23\x83\x83a\x1A\xC8V[a\x0F\x94\x84\x84\x84a\x0B6V[a\x0B\xB9\x84\x84\x84\x84a\x1B^V[\x83B\x11\x15a\x0F\xC4W`@Qc#A\xD7\x87`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\t\x87V[`@\x80Q\x7F\xE4\x83)\x05{\xFD\x03\xD5^I\xB5G\x13.9\xCF\xFD\x9C\x18 \xAD{\x9DLS\x07i\x14%\xD1Z\xDF` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x88\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R_\x90a\x10=\x90a\x105\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1C\x84V[\x85\x85\x85a\x1C\xB0V[\x90Pa\x10I\x81\x87a\x1C\xDCV[a\x10S\x81\x88a\x19\x83V[PPPPPPPV[_a\x10f\x81a\x17EV[`\x16T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x10\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x11\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FInvalid service provider\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\t\x87V[a\x116\x7F\xD8\xC0\xB0&O\xB5\xD2%\xF4\xBA/\xB9$T\xD9\xF4\xF9\x12\xBEM'\xB3UV.j\xE9\x0C\xE2\xF5\xE7K\x83a\x17OV[PP`\x16\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[_\x81\x81R`\x02` R`@\x90 T``\x90`\x01`\x01`\xA0\x1B\x03\x16a\x11\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FURI query for nonexistent token\0`D\x82\x01R`d\x01a\t\x87V[_\x82\x81R`\x17` R`@\x90 \x80Ta\x11\xDD\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\t\x90a3\x87V[\x80\x15a\x12TW\x80`\x1F\x10a\x12+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12TV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x127W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\x12` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`@\x1B\x03\x83\x16\x93`\x01`@\x1B\x90\x93\x04`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a\x12\x9D\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xC9\x90a3\x87V[\x80\x15a\x13\x14W\x80`\x1F\x10a\x12\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x14V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x83V[_\x82\x81R`\x0B` R`@\x90 `\x01\x01Ta\x138\x81a\x17EV[a\x0B\xB9\x83\x83a\x17\xE0V[4g\x01cEx]\x8A\0\0\x14a\x13\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\t\x87V[`\x14\x80T_\x91`\x01`\x01`@\x1B\x03\x90\x91\x16\x90\x82a\x13\xB5\x83a4\xD1V[\x82T`\x01`\x01`@\x1B\x03\x91\x82\x16a\x01\0\x93\x90\x93\n\x92\x83\x02\x92\x82\x02\x19\x16\x91\x90\x91\x17\x90\x91U`@\x80Q``\x81\x01\x82R\x83\x83\x16\x80\x82R3` \x80\x84\x01\x91\x82R\x83\x85\x01\x89\x81R_\x93\x84R`\x12\x90\x91R\x93\x90\x91 \x82Q\x81T\x92Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`@\x1B\x02`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x95\x16\x94\x90\x94\x17\x17\x83U\x90Q\x92\x93P\x91\x82\x91\x90`\x01\x82\x01\x90a\x14F\x90\x82a4\x03V[PP3_\x81\x81R`\x13` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x91\x90\x92 `\x04\x82\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x95\x16\x94\x90\x94\x02a\x01\0\n\x84\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x90Q\x91\x92P\x90\x7F\xF3\xF4\x11\xD8SHk\x9FS\xDAc\0\x9A!\xCD(N\xA1\x8A\x80\rM\xE5\\\xE5\xBD\x93]\x19~L\xF1\x90a\x14\xC8\x90\x87\x90a.\xD8V[`@Q\x80\x91\x03\x90\xA3PPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R`\x01`\x01`@\x1B\x03\x82\x81\x16_\x90\x81R`\x12` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x81T\x94\x85\x16\x81R`\x01`@\x1B\x90\x94\x04`\x01`\x01`\xA0\x1B\x03\x16\x91\x84\x01\x91\x90\x91R`\x01\x81\x01\x80T\x91\x92\x84\x01\x91a\x15G\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15s\x90a3\x87V[\x80\x15a\x15\xBEW\x80`\x1F\x10a\x15\x95Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xBEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xA1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x13` R`@\x81 T\x82\x10a\x16WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x13` R`@\x90 \x80T\x83\x90\x81\x10a\x16\x80Wa\x16\x80a4\xBDV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P\x92\x91PPV[a\x0B2\x82\x82`@Q\x80` \x01`@R\x80_\x81RPa\x1D.V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cye\xDB\x0B`\xE0\x1B\x14\x80a\nkWPa\nk\x82a\x1DDV[_\x81\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\nkW`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\t\x87V[a\x0Cy\x83\x83\x83`\x01a\x1DhV[_a\x17=\x84\x84\x84a\x1ElV[\x94\x93PPPPV[a\r%\x813a\x1E\x87V[_a\x17Z\x83\x83a\x0F\x16V[a\x17\xD9W_\x83\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x17\x913\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x7F/\x87\x88\x11~~\xFF\x1D\x82\xE9&\xECyI\x01\xD1|x\x02JP'\t@0E@\xA73eo\r`@Q`@Q\x80\x91\x03\x90\xA4P`\x01a\nkV[P_a\nkV[_a\x17\xEB\x83\x83a\x0F\x16V[\x15a\x17\xD9W_\x83\x81R`\x0B` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x92R\x80\x83 \x80T`\xFF\x19\x16\x90UQ3\x92\x86\x91\x7F\xF69\x1F\\2\xD9\xC6\x9D*G\xEAg\x0BD)t\xB595\xD1\xED\xC7\xFDd\xEB!\xE0G\xA89\x17\x1B\x91\x90\xA4P`\x01a\nkV[_e\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x18}W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[P\x90V[\x81T_\x90\x81\x81`\x05\x81\x11\x15a\x18\xDDW_a\x18\x9A\x84a\x1E\xC0V[a\x18\xA4\x90\x85a4\xFBV[_\x88\x81R` \x90 \x90\x91P\x81\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x87\x16\x10\x15a\x18\xCDW\x80\x91Pa\x18\xDBV[a\x18\xD8\x81`\x01a5\x0EV[\x92P[P[_a\x18\xEA\x87\x87\x85\x85a\x1F\xA4V[\x90P\x80\x15a\x19$Wa\x19\x0E\x87a\x19\x01`\x01\x84a4\xFBV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x19&V[_[\x97\x96PPPPPPPV[a\x199a \x03V[`\n\x80T`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x0F` R`@\x80\x82 \x80T\x86\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x91Q\x91\x90\x94\x16\x93\x92\x84\x92\x90\x91\x7F14\xE8\xA2\xE6\xD9~\x92\x9A~T\x01\x1E\xA5H]}\x19m\xD5\xF0\xBAMN\xF9X\x03\xE8\xE3\xFC%\x7F\x91\x90\xA4a\x0Cy\x81\x83a\x19\xEF\x86a (V[a 2V[a\x19\xFCa!\x9BV[`\n\x80T`\xFF\x19\x16`\x01\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2Xa\x19f3\x90V[``a\x1A^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0Ca!\xBFV[\x90P\x90V[``a\x1A^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\ra!\xBFV[\x80T_\x90\x80\x15a\x1A\xBFWa\x1A\xA9\x83a\x19\x01`\x01\x84a4\xFBV[T`\x01`0\x1B\x90\x04`\x01`\x01`\xD0\x1B\x03\x16a\x1A\xC1V[_[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1A\xFAW`@Qc\x0Ba\x17C`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x14\xC8V[`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\x0B\xB9W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90c\x15\x0Bz\x02\x90a\x1B\xA0\x903\x90\x88\x90\x87\x90\x87\x90`\x04\x01a5!V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x1B\xDAWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1B\xD7\x91\x81\x01\x90a5]V[`\x01[a\x1CAW=\x80\x80\x15a\x1C\x07W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\x0CV[``\x91P[P\x80Q_\x03a\x1C9W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\x87V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\n\x85\xBD\x01`\xE1\x1B\x14a\x1C}W`@Qc2PWI`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\t\x87V[PPPPPV[_a\nka\x1C\x90a\"hV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[____a\x1C\xC0\x88\x88\x88\x88a#\x91V[\x92P\x92P\x92Pa\x1C\xD0\x82\x82a$YV[P\x90\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x0E` R`@\x90 \x80T`\x01\x81\x01\x90\x91U\x81\x81\x14a\x0CyW`@Qc\x01\xD4\xB6#`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\x87V[a\x1D8\x83\x83a%\x11V[a\x0Cy_\x84\x84\x84a\x1B^V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14\x80a\nkWPa\nk\x82a%rV[\x80\x80a\x1D|WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[\x15a\x1E=W_a\x1D\x8B\x84a\x16\xECV[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x1D\xB7WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x80\x15a\x1D\xCAWPa\x1D\xC8\x81\x84a\x15\xCEV[\x15[\x15a\x1D\xF3W`@Qc\xA9\xFB\xF5\x1F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\t\x87V[\x81\x15a\x1E;W\x83\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4[P[PP_\x90\x81R`\x04` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[__a\x1Ey\x85\x85\x85a%\xC1V[\x90Pa\x17=\x81\x86`\x01a%\xD5V[a\x1E\x91\x82\x82a\x0F\x16V[a\x0B2W`@Qc\xE2Q}?`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[_\x81_\x03a\x1E\xCFWP_\x91\x90PV[_`\x01a\x1E\xDB\x84a&JV[\x90\x1C`\x01\x90\x1B\x90P`\x01\x81\x84\x81a\x1E\xF4Wa\x1E\xF4a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F\x0CWa\x1F\x0Ca5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F$Wa\x1F$a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F<Wa\x1F<a5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1FTWa\x1FTa5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1FlWa\x1Fla5xV[\x04\x82\x01\x90\x1C\x90P`\x01\x81\x84\x81a\x1F\x84Wa\x1F\x84a5xV[\x04\x82\x01\x90\x1C\x90Pa\x1A\xC1\x81\x82\x85\x81a\x1F\x9EWa\x1F\x9Ea5xV[\x04a&\xDDV[_[\x81\x83\x10\x15a\x1F\xFBW_a\x1F\xB9\x84\x84a&\xF2V[_\x87\x81R` \x90 \x90\x91Pe\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Te\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1F\xE7W\x80\x92Pa\x1F\xF5V[a\x1F\xF2\x81`\x01a5\x0EV[\x93P[Pa\x1F\xA6V[P\x93\x92PPPV[`\nT`\xFF\x16a &W`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\nk\x82a\r\xB7V[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a SWP_\x81\x11[\x15a\x0CyW`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a \xFAW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x10` R`@\x81 \x81\x90a \x95\x90a'\x0Ca \x90\x86a'\x17V[a'JV[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa \xEF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PP[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x0CyW`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x10` R`@\x81 \x81\x90a!2\x90a'{a \x90\x86a'\x17V[`\x01`\x01`\xD0\x1B\x03\x16\x91P`\x01`\x01`\xD0\x1B\x03\x16\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDE\xC2\xBA\xCD\xD2\xF0[Y\xDE4\xDA\x9BR=\xFF\x8B\xE4.^8\xE8\x18\xC8/\xDB\x0B\xAEwC\x87\xA7$\x83\x83`@Qa!\x8C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\nT`\xFF\x16\x15a &W`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[```\xFF\x83\x14a!\xD9Wa!\xD2\x83a'\x86V[\x90Pa\nkV[\x81\x80Ta!\xE5\x90a3\x87V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"\x11\x90a3\x87V[\x80\x15a\"\\W\x80`\x1F\x10a\"3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\\V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"?W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90Pa\nkV[_0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80\x15a\"\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14[\x15a\"\xEAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[a\x1A^`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R_\x90`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a#\xCAWP_\x91P`\x03\x90P\x82a$OV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a$\x1BW=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a$FWP_\x92P`\x01\x91P\x82\x90Pa$OV[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_\x82`\x03\x81\x11\x15a$lWa$la5\x8CV[\x03a$uWPPV[`\x01\x82`\x03\x81\x11\x15a$\x89Wa$\x89a5\x8CV[\x03a$\xA7W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a$\xBBWa$\xBBa5\x8CV[\x03a$\xDCW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`\x03\x82`\x03\x81\x11\x15a$\xF0Wa$\xF0a5\x8CV[\x03a\x0B2W`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`\x01`\x01`\xA0\x1B\x03\x82\x16a%:W`@Qc2PWI`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_a%F\x83\x83_a\x171V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x0CyW`@Qc9\xE3V7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\t\x87V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14\x80a%\xA2WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x80a\nkWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\nkV[_a%\xCAa!\x9BV[a\x17=\x84\x84\x84a'\xC3V[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\xF7Wa%\xF4`\x11a'{a \x90\x84a'\x17V[PP[`\x01`\x01`\xA0\x1B\x03\x82\x16a&\x19Wa&\x16`\x11a'\x0Ca \x90\x84a'\x17V[PP[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x0F` R`@\x80\x82 T\x85\x84\x16\x83R\x91 Ta\x0Cy\x92\x91\x82\x16\x91\x16\x83a 2V[_\x80`\x80\x83\x90\x1C\x15a&^W`\x80\x92\x83\x1C\x92\x01[`@\x83\x90\x1C\x15a&pW`@\x92\x83\x1C\x92\x01[` \x83\x90\x1C\x15a&\x82W` \x92\x83\x1C\x92\x01[`\x10\x83\x90\x1C\x15a&\x94W`\x10\x92\x83\x1C\x92\x01[`\x08\x83\x90\x1C\x15a&\xA6W`\x08\x92\x83\x1C\x92\x01[`\x04\x83\x90\x1C\x15a&\xB8W`\x04\x92\x83\x1C\x92\x01[`\x02\x83\x90\x1C\x15a&\xCAW`\x02\x92\x83\x1C\x92\x01[`\x01\x83\x90\x1C\x15a\nkW`\x01\x01\x92\x91PPV[_\x81\x83\x10a&\xEBW\x81a\x1A\xC1V[P\x90\x91\x90PV[_a'\0`\x02\x84\x84\x18a5\xA0V[a\x1A\xC1\x90\x84\x84\x16a5\x0EV[_a\x1A\xC1\x82\x84a5\xBFV[_`\x01`\x01`\xD0\x1B\x03\x82\x11\x15a\x18}W`@Qc\x06\xDF\xCCe`\xE4\x1B\x81R`\xD0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x01a\t\x87V[_\x80a'nBa'fa'\\\x88a\x1A\x90V[\x86\x88c\xFF\xFF\xFF\xFF\x16V[\x87\x91\x90a(\x8EV[\x91P\x91P[\x93P\x93\x91PPV[_a\x1A\xC1\x82\x84a5\xDEV[``_a'\x92\x83a(\x9BV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[__a'\xD0\x85\x85\x85a(\xC2V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a(,Wa('\x84`\x08\x80T_\x83\x81R`\t` R`@\x81 \x82\x90U`\x01\x82\x01\x83U\x91\x90\x91R\x7F\xF3\xF7\xA9\xFE6O\xAA\xB9;!m\xA5\n2\x14\x15O\"\xA0\xA2\xB4\x15\xB2:\x84\xC8\x16\x9E\x8Bcn\xE3\x01UV[a(OV[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a(OWa(O\x81\x85a)\xB4V[`\x01`\x01`\xA0\x1B\x03\x85\x16a(kWa(f\x84a*AV[a\x17=V[\x84`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17=Wa\x17=\x85\x85a*\xE8V[_\x80a'n\x85\x85\x85a+6V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\nkW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x02` R`@\x81 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x15a(\xEEWa(\xEE\x81\x84\x86a,\xACV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a)(Wa)\t_\x85__a\x1DhV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` R`@\x90 \x80T_\x19\x01\x90U[`\x01`\x01`\xA0\x1B\x03\x85\x16\x15a)VW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x03` R`@\x90 \x80T`\x01\x01\x90U[_\x84\x81R`\x02` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x91\x82\x17\x90\x92U\x91Q\x87\x93\x91\x85\x16\x91\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x91\xA4\x94\x93PPPPV[_a)\xBE\x83a\r\xB7V[_\x83\x81R`\x07` R`@\x90 T\x90\x91P\x80\x82\x14a*\x0FW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 T\x84\x84R\x81\x84 \x81\x90U\x83R`\x07\x90\x91R\x90 \x81\x90U[P_\x91\x82R`\x07` \x90\x81R`@\x80\x84 \x84\x90U`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x83R`\x06\x81R\x83\x83 \x91\x83RR\x90\x81 UV[`\x08T_\x90a*R\x90`\x01\x90a4\xFBV[_\x83\x81R`\t` R`@\x81 T`\x08\x80T\x93\x94P\x90\x92\x84\x90\x81\x10a*yWa*ya4\xBDV[\x90_R` _ \x01T\x90P\x80`\x08\x83\x81T\x81\x10a*\x98Wa*\x98a4\xBDV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x82\x81R`\t\x90\x91R`@\x80\x82 \x84\x90U\x85\x82R\x81 U`\x08\x80T\x80a*\xCFWa*\xCFa5\xFDV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPV[_`\x01a*\xF4\x84a\r\xB7V[a*\xFE\x91\x90a4\xFBV[`\x01`\x01`\xA0\x1B\x03\x90\x93\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x80\x83 \x85\x90U\x93\x82R`\x07\x90R\x91\x90\x91 \x91\x90\x91UPV[\x82T_\x90\x81\x90\x80\x15a,RW_a+R\x87a\x19\x01`\x01\x85a4\xFBV[`@\x80Q\x80\x82\x01\x90\x91R\x90Te\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01`0\x1B\x90\x92\x04`\x01`\x01`\xD0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a+\xA6W`@Qc% `\x1D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qe\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a+\xF2W\x84a+\xC9\x88a\x19\x01`\x01\x86a4\xFBV[\x80T`\x01`\x01`\xD0\x1B\x03\x92\x90\x92\x16`\x01`0\x1B\x02e\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90Ua,BV[`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`0\x1B\x02\x92\x16\x91\x90\x91\x17\x91\x01U[` \x01Q\x92P\x83\x91Pa's\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Re\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`\xD0\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x92Q\x90\x93\x16`\x01`0\x1B\x02\x91\x90\x93\x16\x17\x92\x01\x91\x90\x91U\x90P\x81a'sV[a,\xB7\x83\x83\x83a-\x10V[a\x0CyW`\x01`\x01`\xA0\x1B\x03\x83\x16a,\xE5W`@Qc~'2\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\t\x87V[`@Qc\x17~\x80/`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\t\x87V[_`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x80\x15\x90a\x17=WP\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a-IWPa-I\x84\x84a\x15\xCEV[\x80a\x17=WPP_\x90\x81R`\x04` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x16\x14\x91\x90PV[P\x80Ta-}\x90a3\x87V[_\x82U\x80`\x1F\x10a-\x8CWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\r%\x91\x90[\x80\x82\x11\x15a\x18}W_\x81U`\x01\x01a-\xA4V[__\x83`\x1F\x84\x01\x12a-\xC7W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xDDW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-\xF4W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a.\x0EW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a.#W__\xFD[a./\x87\x82\x88\x01a-\xB7V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.MW__\xFD[a.Y\x87\x82\x88\x01a-\xB7V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\r%W__\xFD[_` \x82\x84\x03\x12\x15a.\x8AW__\xFD[\x815a\x1A\xC1\x81a.eV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a.\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a.\x9DV[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x1A\xC1` \x83\x01\x84a.\x95V[_` \x82\x84\x03\x12\x15a.\xFAW__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\r%W__\xFD[__`@\x83\x85\x03\x12\x15a/&W__\xFD[\x825a/1\x81a/\x01V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a/QW__\xFD[\x835a/\\\x81a/\x01V[\x92P` \x84\x015a/l\x81a/\x01V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[__`@\x83\x85\x03\x12\x15a/\x8EW__\xFD[\x825\x91P` \x83\x015a/\xA0\x81a/\x01V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a/\xBBW__\xFD[\x815a\x1A\xC1\x81a/\x01V[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_a/\xE4`\xE0\x83\x01\x89a.\x95V[\x82\x81\x03`@\x84\x01Ra/\xF6\x81\x89a.\x95V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a0KW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a0-V[P\x90\x9B\x9APPPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a0mW__\xFD[\x825a0x\x81a/\x01V[\x91P` \x83\x015\x80\x15\x15\x81\x14a/\xA0W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a0\xB9Wa0\xB9a0\x8CV[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a0\xE7Wa0\xE7a0\x8CV[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a0\xFEW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a1$W__\xFD[a\x1A\xC1\x83\x835` \x85\x01a0\xA0V[____`\x80\x85\x87\x03\x12\x15a1FW__\xFD[\x845a1Q\x81a/\x01V[\x93P` \x85\x015a1a\x81a/\x01V[\x92P`@\x85\x015\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x82W__\xFD[a1\x8E\x87\x82\x88\x01a1\x15V[\x91PP\x92\x95\x91\x94P\x92PV[______`\xC0\x87\x89\x03\x12\x15a1\xAFW__\xFD[\x865a1\xBA\x81a/\x01V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015`\xFF\x81\x16\x81\x14a1\xDDW__\xFD[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_` \x82\x84\x03\x12\x15a2\x07W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x1A\xC1W__\xFD[`\x01`\x01`@\x1B\x03\x84\x16\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90a2O\x90\x83\x01\x84a.\x95V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a2hW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2}W__\xFD[a\x17=\x84\x82\x85\x01a1\x15V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x17=`\x80\x84\x01\x82a.\x95V[__`@\x83\x85\x03\x12\x15a2\xD9W__\xFD[\x825a2\xE4\x81a/\x01V[\x91P` \x83\x015a/\xA0\x81a/\x01V[___``\x84\x86\x03\x12\x15a3\x06W__\xFD[\x835a3\x11\x81a/\x01V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a32W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a3BW__\xFD[a3Q\x86\x825` \x84\x01a0\xA0V[\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a3\x80Wa3\x80a3[V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a3\x9BW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a3\xB9WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x0CyW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a3\xE4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1C}W_\x81U`\x01\x01a3\xF0V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x1CWa4\x1Ca0\x8CV[a40\x81a4*\x84Ta3\x87V[\x84a3\xBFV[` `\x1F\x82\x11`\x01\x81\x14a4bW_\x83\x15a4KWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1C}V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a4\x91W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a4qV[P\x84\x82\x10\x15a4\xAEW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`\x01`\x01`@\x1B\x03\x82\x16`\x01`\x01`@\x1B\x03\x81\x03a4\xF2Wa4\xF2a3[V[`\x01\x01\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\nkWa\nka3[V[\x80\x82\x01\x80\x82\x11\x15a\nkWa\nka3[V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a5S\x90\x83\x01\x84a.\x95V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a5mW__\xFD[\x81Qa\x1A\xC1\x81a.eV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_\x82a5\xBAWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01`\xD0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\nkWa\nka3[V[`\x01`\x01`\xD0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\nkWa\nka3[V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 k\xC5\xD5\xE6\xE5\xA6\x03\xEE\xBE\x85\xC9\xECz\xF3%\xBE\xDD\x95\x07\xE3\xE2\x8DB\xADc\xA5dG2\xA7(\xAAdsolcC\0\x08\x1C\x003",
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
        const COUNT: usize = 48usize;
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
        DelegateChanged(DelegateChanged),
        DelegateVotesChanged(DelegateVotesChanged),
        EIP712DomainChanged(EIP712DomainChanged),
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for NFTWithTriggerEvents {
        const NAME: &'static str = "NFTWithTriggerEvents";
        const COUNT: usize = 13usize;
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
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
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
                Self::DelegateChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelegateVotesChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
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
