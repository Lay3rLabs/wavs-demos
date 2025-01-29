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

interface SafeModule {
    event NewTrigger(bytes);

    constructor(address _safe);

    function addTrigger(bytes memory data) external payable;
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    function getTriggerCount(address creator) external view returns (uint256);
    function getTriggerIdAtIndex(address creator, uint256 index) external view returns (ISimpleTrigger.TriggerId);
    function handleAddPayload(bytes memory data, bytes memory signature) external;
    function initialize(address _serviceProvider) external;
    function initialized() external view returns (bool);
    function nextTriggerId() external view returns (ISimpleTrigger.TriggerId);
    function owner() external view returns (address);
    function safe() external view returns (address);
    function serviceProvider() external view returns (address);
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    function triggersById(ISimpleTrigger.TriggerId) external view returns (address creator, bytes memory data);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_safe",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "nextTriggerId",
    "inputs": [],
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
    "name": "owner",
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
    "name": "safe",
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
    "type": "event",
    "name": "NewTrigger",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
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
pub mod SafeModule {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506040516110aa3803806110aa833981016040819052602b9160b0565b6001600160a01b03811660845760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420736166652061646472657373000000000000000000000000604482015260640160405180910390fd5b5f80546001600160a01b039092166001600160a01b0319928316179055600180549091163317905560db565b5f6020828403121560bf575f5ffd5b81516001600160a01b038116811460d4575f5ffd5b9392505050565b610fc2806100e85f395ff3fe6080604052600436106100be575f3560e01c80638da5cb5b1161007c578063ce28961211610057578063ce28961214610243578063e31e078814610270578063e328ed7714610283578063f7ee944c146102af575f5ffd5b80638da5cb5b146101e6578063913b1fbf14610205578063c4d66de814610224575f5ffd5b806273e1d7146100c2578063158ef93e146100e3578063186f0354146101185780633383abfe1461014e57806342227fa4146101905780638d69e95e146101c7575b5f5ffd5b3480156100cd575f5ffd5b506100e16100dc366004610ac9565b6102ce565b005b3480156100ee575f5ffd5b5060025461010390600160a01b900460ff1681565b60405190151581526020015b60405180910390f35b348015610123575f5ffd5b505f54610136906001600160a01b031681565b6040516001600160a01b03909116815260200161010f565b348015610159575f5ffd5b50610182610168366004610b4a565b6001600160a01b03165f9081526004602052604090205490565b60405190815260200161010f565b34801561019b575f5ffd5b506005546101af906001600160401b031681565b6040516001600160401b03909116815260200161010f565b3480156101d2575f5ffd5b50600254610136906001600160a01b031681565b3480156101f1575f5ffd5b50600154610136906001600160a01b031681565b348015610210575f5ffd5b506101af61021f366004610b6c565b610476565b34801561022f575f5ffd5b506100e161023e366004610b4a565b6104bd565b34801561024e575f5ffd5b5061026261025d366004610b96565b6105ee565b60405161010f929190610bff565b6100e161027e366004610cc7565b61069a565b34801561028e575f5ffd5b506102a261029d366004610b96565b6108e7565b60405161010f9190610cf8565b3480156102ba575f5ffd5b506101af6102c9366004610b6c565b6109d0565b6002546001600160a01b031633146103425760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f808061035186880188610d37565b919450925090506001600160a01b0383166103a75760405162461bcd60e51b8152602060048201526016602482015275496e76616c696420746172676574206164647265737360501b6044820152606401610339565b5f805460405163468721a760e01b81526001600160a01b039091169063468721a7906103dd908790879087908790600401610d8b565b6020604051808303815f875af11580156103f9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061041d9190610dde565b90508061046c5760405162461bcd60e51b815260206004820152601960248201527f4d6f64756c65207472616e73616374696f6e206661696c6564000000000000006044820152606401610339565b5050505050505050565b6004602052815f5260405f20818154811061048f575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b6001546001600160a01b031633146105215760405162461bcd60e51b815260206004820152602160248201527f4f6e6c79206f776e65722063616e2063616c6c20746869732066756e6374696f6044820152603760f91b6064820152608401610339565b600254600160a01b900460ff16156105715760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610339565b6001600160a01b0381166105c75760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f766964657220616464726573736044820152606401610339565b600280546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b60036020525f9081526040902080546001820180546001600160a01b03909216929161061990610dfd565b80601f016020809104026020016040519081016040528092919081815260200182805461064590610dfd565b80156106905780601f1061066757610100808354040283529160200191610690565b820191905f5260205f20905b81548152906001019060200180831161067357829003601f168201915b5050505050905082565b3467016345785d8a0000146106f15760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e3120455448006044820152606401610339565b5f80546040516001600160a01b039091169034908381818185875af1925050503d805f811461073b576040519150601f19603f3d011682016040523d82523d5f602084013e610740565b606091505b50509050806107915760405162461bcd60e51b815260206004820152601b60248201527f455448207472616e7366657220746f2053616665206661696c656400000000006044820152606401610339565b6005546107a8906001600160401b03166001610e35565b6005805467ffffffffffffffff19166001600160401b0392909216918217905560408051808201825233815260208082018681525f8581526003909252929020815181546001600160a01b0319166001600160a01b0390911617815591519091829160018201906108199082610eac565b5050335f90815260046020818152604080842080546001810182559085529382902092840490920180546001600160401b0380891660086003909716969096026101000a868102910219909116179055815160608101835292835284516001600160a01b031683820152848101518383015290519192507f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68916108be91849101610cf8565b60408051601f19818403018152908290526108d891610f66565b60405180910390a15050505050565b60408051606080820183525f80835260208084018290528385018390526001600160401b03861680835260038252918590208551938401865291835281546001600160a01b031690830152600181018054939491939183019161094990610dfd565b80601f016020809104026020016040519081016040528092919081815260200182805461097590610dfd565b80156109c05780601f10610997576101008083540402835291602001916109c0565b820191905f5260205f20905b8154815290600101906020018083116109a357829003601f168201915b5050505050815250915050919050565b6001600160a01b0382165f908152600460205260408120548210610a2c5760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610339565b6001600160a01b0383165f908152600460205260409020805483908110610a5557610a55610f78565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b031690505b92915050565b5f5f83601f840112610a95575f5ffd5b5081356001600160401b03811115610aab575f5ffd5b602083019150836020828501011115610ac2575f5ffd5b9250929050565b5f5f5f5f60408587031215610adc575f5ffd5b84356001600160401b03811115610af1575f5ffd5b610afd87828801610a85565b90955093505060208501356001600160401b03811115610b1b575f5ffd5b610b2787828801610a85565b95989497509550505050565b6001600160a01b0381168114610b47575f5ffd5b50565b5f60208284031215610b5a575f5ffd5b8135610b6581610b33565b9392505050565b5f5f60408385031215610b7d575f5ffd5b8235610b8881610b33565b946020939093013593505050565b5f60208284031215610ba6575f5ffd5b81356001600160401b0381168114610b65575f5ffd5b5f81518084525f5b81811015610be057602081850181015186830182015201610bc4565b505f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90610c2290830184610bbc565b949350505050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610c4d575f5ffd5b81356001600160401b03811115610c6657610c66610c2a565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610c9457610c94610c2a565b604052818152838201602001851015610cab575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215610cd7575f5ffd5b81356001600160401b03811115610cec575f5ffd5b610c2284828501610c3e565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f6040830151606080840152610c226080840182610bbc565b5f5f5f60608486031215610d49575f5ffd5b8335610d5481610b33565b92506020840135915060408401356001600160401b03811115610d75575f5ffd5b610d8186828701610c3e565b9150509250925092565b60018060a01b0385168152836020820152608060408201525f610db16080830185610bbc565b905060028310610dcf57634e487b7160e01b5f52602160045260245ffd5b82606083015295945050505050565b5f60208284031215610dee575f5ffd5b81518015158114610b65575f5ffd5b600181811c90821680610e1157607f821691505b602082108103610e2f57634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160401b038181168382160190811115610a7f57634e487b7160e01b5f52601160045260245ffd5b601f821115610ea757805f5260205f20601f840160051c81016020851015610e855750805b601f840160051c820191505b81811015610ea4575f8155600101610e91565b50505b505050565b81516001600160401b03811115610ec557610ec5610c2a565b610ed981610ed38454610dfd565b84610e60565b6020601f821160018114610f0b575f8315610ef45750848201515b5f19600385901b1c1916600184901b178455610ea4565b5f84815260208120601f198516915b82811015610f3a5787850151825560209485019460019092019101610f1a565b5084821015610f5757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602081525f610b656020830184610bbc565b634e487b7160e01b5f52603260045260245ffdfea2646970667358221220bbde014569881dea4e215c2e9eff7b3e20f2b78ed325ca7ddb6470d2f70eb50664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa\x10\xAA8\x03\x80a\x10\xAA\x839\x81\x01`@\x81\x90R`+\x91`\xB0V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid safe address\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U`\x01\x80T\x90\x91\x163\x17\x90U`\xDBV[_` \x82\x84\x03\x12\x15`\xBFW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\xD4W__\xFD[\x93\x92PPPV[a\x0F\xC2\x80a\0\xE8_9_\xF3\xFE`\x80`@R`\x046\x10a\0\xBEW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0|W\x80c\xCE(\x96\x12\x11a\0WW\x80c\xCE(\x96\x12\x14a\x02CW\x80c\xE3\x1E\x07\x88\x14a\x02pW\x80c\xE3(\xEDw\x14a\x02\x83W\x80c\xF7\xEE\x94L\x14a\x02\xAFW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xE6W\x80c\x91;\x1F\xBF\x14a\x02\x05W\x80c\xC4\xD6m\xE8\x14a\x02$W__\xFD[\x80bs\xE1\xD7\x14a\0\xC2W\x80c\x15\x8E\xF9>\x14a\0\xE3W\x80c\x18o\x03T\x14a\x01\x18W\x80c3\x83\xAB\xFE\x14a\x01NW\x80cB\"\x7F\xA4\x14a\x01\x90W\x80c\x8Di\xE9^\x14a\x01\xC7W[__\xFD[4\x80\x15a\0\xCDW__\xFD[Pa\0\xE1a\0\xDC6`\x04a\n\xC9V[a\x02\xCEV[\0[4\x80\x15a\0\xEEW__\xFD[P`\x02Ta\x01\x03\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01#W__\xFD[P_Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[4\x80\x15a\x01YW__\xFD[Pa\x01\x82a\x01h6`\x04a\x0BJV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\x0FV[4\x80\x15a\x01\x9BW__\xFD[P`\x05Ta\x01\xAF\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[4\x80\x15a\x01\xD2W__\xFD[P`\x02Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\xF1W__\xFD[P`\x01Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x10W__\xFD[Pa\x01\xAFa\x02\x1F6`\x04a\x0BlV[a\x04vV[4\x80\x15a\x02/W__\xFD[Pa\0\xE1a\x02>6`\x04a\x0BJV[a\x04\xBDV[4\x80\x15a\x02NW__\xFD[Pa\x02ba\x02]6`\x04a\x0B\x96V[a\x05\xEEV[`@Qa\x01\x0F\x92\x91\x90a\x0B\xFFV[a\0\xE1a\x02~6`\x04a\x0C\xC7V[a\x06\x9AV[4\x80\x15a\x02\x8EW__\xFD[Pa\x02\xA2a\x02\x9D6`\x04a\x0B\x96V[a\x08\xE7V[`@Qa\x01\x0F\x91\x90a\x0C\xF8V[4\x80\x15a\x02\xBAW__\xFD[Pa\x01\xAFa\x02\xC96`\x04a\x0BlV[a\t\xD0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x03Q\x86\x88\x01\x88a\r7V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\x03\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid target address`P\x1B`D\x82\x01R`d\x01a\x039V[_\x80T`@QcF\x87!\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\x87!\xA7\x90a\x03\xDD\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\r\x8BV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1D\x91\x90a\r\xDEV[\x90P\x80a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FModule transaction failed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x039V[PPPPPPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x04\x8FW_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FOnly owner can call this functio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x039V[`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x039V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x039V[`\x02\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x06\x19\x90a\r\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06E\x90a\r\xFDV[\x80\x15a\x06\x90W\x80`\x1F\x10a\x06gWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x90V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06sW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[4g\x01cEx]\x8A\0\0\x14a\x06\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\x039V[_\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x07;W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07@V[``\x91P[PP\x90P\x80a\x07\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FETH transfer to Safe failed\0\0\0\0\0`D\x82\x01R`d\x01a\x039V[`\x05Ta\x07\xA8\x90`\x01`\x01`@\x1B\x03\x16`\x01a\x0E5V[`\x05\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q\x80\x82\x01\x82R3\x81R` \x80\x82\x01\x86\x81R_\x85\x81R`\x03\x90\x92R\x92\x90 \x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x91Q\x90\x91\x82\x91`\x01\x82\x01\x90a\x08\x19\x90\x82a\x0E\xACV[PP3_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x80T`\x01\x81\x01\x82U\x90\x85R\x93\x82\x90 \x92\x84\x04\x90\x92\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x97\x16\x96\x90\x96\x02a\x01\0\n\x86\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x81Q``\x81\x01\x83R\x92\x83R\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x01R\x84\x81\x01Q\x83\x83\x01R\x90Q\x91\x92P\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x91a\x08\xBE\x91\x84\x91\x01a\x0C\xF8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x08\xD8\x91a\x0FfV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x16\x80\x83R`\x03\x82R\x91\x85\x90 \x85Q\x93\x84\x01\x86R\x91\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x01\x81\x01\x80T\x93\x94\x91\x93\x91\x83\x01\x91a\tI\x90a\r\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tu\x90a\r\xFDV[\x80\x15a\t\xC0W\x80`\x1F\x10a\t\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x04` R`@\x81 T\x82\x10a\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x039V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x04` R`@\x90 \x80T\x83\x90\x81\x10a\nUWa\nUa\x0FxV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\n\x95W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xABW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\xC2W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\n\xDCW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xF1W__\xFD[a\n\xFD\x87\x82\x88\x01a\n\x85V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x1BW__\xFD[a\x0B'\x87\x82\x88\x01a\n\x85V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BGW__\xFD[PV[_` \x82\x84\x03\x12\x15a\x0BZW__\xFD[\x815a\x0Be\x81a\x0B3V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[\x825a\x0B\x88\x81a\x0B3V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x0B\xA6W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0BeW__\xFD[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x0B\xE0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\xC4V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0C\"\x90\x83\x01\x84a\x0B\xBCV[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x0CMW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CfWa\x0Cfa\x0C*V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x94Wa\x0C\x94a\x0C*V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x0C\xABW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C\xD7W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xECW__\xFD[a\x0C\"\x84\x82\x85\x01a\x0C>V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x0C\"`\x80\x84\x01\x82a\x0B\xBCV[___``\x84\x86\x03\x12\x15a\rIW__\xFD[\x835a\rT\x81a\x0B3V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\ruW__\xFD[a\r\x81\x86\x82\x87\x01a\x0C>V[\x91PP\x92P\x92P\x92V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a\r\xB1`\x80\x83\x01\x85a\x0B\xBCV[\x90P`\x02\x83\x10a\r\xCFWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82``\x83\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\r\xEEW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0BeW__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\x11W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E/WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\x7FWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x0E\xA7W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x0E\x85WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\xA4W_\x81U`\x01\x01a\x0E\x91V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC5Wa\x0E\xC5a\x0C*V[a\x0E\xD9\x81a\x0E\xD3\x84Ta\r\xFDV[\x84a\x0E`V[` `\x1F\x82\x11`\x01\x81\x14a\x0F\x0BW_\x83\x15a\x0E\xF4WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0E\xA4V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x0F:W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0F\x1AV[P\x84\x82\x10\x15a\x0FWW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R_a\x0Be` \x83\x01\x84a\x0B\xBCV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xBB\xDE\x01Ei\x88\x1D\xEAN!\\.\x9E\xFF{> \xF2\xB7\x8E\xD3%\xCA}\xDBdp\xD2\xF7\x0E\xB5\x06dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100be575f3560e01c80638da5cb5b1161007c578063ce28961211610057578063ce28961214610243578063e31e078814610270578063e328ed7714610283578063f7ee944c146102af575f5ffd5b80638da5cb5b146101e6578063913b1fbf14610205578063c4d66de814610224575f5ffd5b806273e1d7146100c2578063158ef93e146100e3578063186f0354146101185780633383abfe1461014e57806342227fa4146101905780638d69e95e146101c7575b5f5ffd5b3480156100cd575f5ffd5b506100e16100dc366004610ac9565b6102ce565b005b3480156100ee575f5ffd5b5060025461010390600160a01b900460ff1681565b60405190151581526020015b60405180910390f35b348015610123575f5ffd5b505f54610136906001600160a01b031681565b6040516001600160a01b03909116815260200161010f565b348015610159575f5ffd5b50610182610168366004610b4a565b6001600160a01b03165f9081526004602052604090205490565b60405190815260200161010f565b34801561019b575f5ffd5b506005546101af906001600160401b031681565b6040516001600160401b03909116815260200161010f565b3480156101d2575f5ffd5b50600254610136906001600160a01b031681565b3480156101f1575f5ffd5b50600154610136906001600160a01b031681565b348015610210575f5ffd5b506101af61021f366004610b6c565b610476565b34801561022f575f5ffd5b506100e161023e366004610b4a565b6104bd565b34801561024e575f5ffd5b5061026261025d366004610b96565b6105ee565b60405161010f929190610bff565b6100e161027e366004610cc7565b61069a565b34801561028e575f5ffd5b506102a261029d366004610b96565b6108e7565b60405161010f9190610cf8565b3480156102ba575f5ffd5b506101af6102c9366004610b6c565b6109d0565b6002546001600160a01b031633146103425760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f808061035186880188610d37565b919450925090506001600160a01b0383166103a75760405162461bcd60e51b8152602060048201526016602482015275496e76616c696420746172676574206164647265737360501b6044820152606401610339565b5f805460405163468721a760e01b81526001600160a01b039091169063468721a7906103dd908790879087908790600401610d8b565b6020604051808303815f875af11580156103f9573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061041d9190610dde565b90508061046c5760405162461bcd60e51b815260206004820152601960248201527f4d6f64756c65207472616e73616374696f6e206661696c6564000000000000006044820152606401610339565b5050505050505050565b6004602052815f5260405f20818154811061048f575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b6001546001600160a01b031633146105215760405162461bcd60e51b815260206004820152602160248201527f4f6e6c79206f776e65722063616e2063616c6c20746869732066756e6374696f6044820152603760f91b6064820152608401610339565b600254600160a01b900460ff16156105715760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610339565b6001600160a01b0381166105c75760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f766964657220616464726573736044820152606401610339565b600280546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b60036020525f9081526040902080546001820180546001600160a01b03909216929161061990610dfd565b80601f016020809104026020016040519081016040528092919081815260200182805461064590610dfd565b80156106905780601f1061066757610100808354040283529160200191610690565b820191905f5260205f20905b81548152906001019060200180831161067357829003601f168201915b5050505050905082565b3467016345785d8a0000146106f15760405162461bcd60e51b815260206004820152601f60248201527f5061796d656e74206d7573742062652065786163746c7920302e3120455448006044820152606401610339565b5f80546040516001600160a01b039091169034908381818185875af1925050503d805f811461073b576040519150601f19603f3d011682016040523d82523d5f602084013e610740565b606091505b50509050806107915760405162461bcd60e51b815260206004820152601b60248201527f455448207472616e7366657220746f2053616665206661696c656400000000006044820152606401610339565b6005546107a8906001600160401b03166001610e35565b6005805467ffffffffffffffff19166001600160401b0392909216918217905560408051808201825233815260208082018681525f8581526003909252929020815181546001600160a01b0319166001600160a01b0390911617815591519091829160018201906108199082610eac565b5050335f90815260046020818152604080842080546001810182559085529382902092840490920180546001600160401b0380891660086003909716969096026101000a868102910219909116179055815160608101835292835284516001600160a01b031683820152848101518383015290519192507f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68916108be91849101610cf8565b60408051601f19818403018152908290526108d891610f66565b60405180910390a15050505050565b60408051606080820183525f80835260208084018290528385018390526001600160401b03861680835260038252918590208551938401865291835281546001600160a01b031690830152600181018054939491939183019161094990610dfd565b80601f016020809104026020016040519081016040528092919081815260200182805461097590610dfd565b80156109c05780601f10610997576101008083540402835291602001916109c0565b820191905f5260205f20905b8154815290600101906020018083116109a357829003601f168201915b5050505050815250915050919050565b6001600160a01b0382165f908152600460205260408120548210610a2c5760405162461bcd60e51b8152602060048201526013602482015272496e646578206f7574206f6620626f756e647360681b6044820152606401610339565b6001600160a01b0383165f908152600460205260409020805483908110610a5557610a55610f78565b905f5260205f2090600491828204019190066008029054906101000a90046001600160401b031690505b92915050565b5f5f83601f840112610a95575f5ffd5b5081356001600160401b03811115610aab575f5ffd5b602083019150836020828501011115610ac2575f5ffd5b9250929050565b5f5f5f5f60408587031215610adc575f5ffd5b84356001600160401b03811115610af1575f5ffd5b610afd87828801610a85565b90955093505060208501356001600160401b03811115610b1b575f5ffd5b610b2787828801610a85565b95989497509550505050565b6001600160a01b0381168114610b47575f5ffd5b50565b5f60208284031215610b5a575f5ffd5b8135610b6581610b33565b9392505050565b5f5f60408385031215610b7d575f5ffd5b8235610b8881610b33565b946020939093013593505050565b5f60208284031215610ba6575f5ffd5b81356001600160401b0381168114610b65575f5ffd5b5f81518084525f5b81811015610be057602081850181015186830182015201610bc4565b505f602082860101526020601f19601f83011685010191505092915050565b6001600160a01b03831681526040602082018190525f90610c2290830184610bbc565b949350505050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610c4d575f5ffd5b81356001600160401b03811115610c6657610c66610c2a565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610c9457610c94610c2a565b604052818152838201602001851015610cab575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f60208284031215610cd7575f5ffd5b81356001600160401b03811115610cec575f5ffd5b610c2284828501610c3e565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f6040830151606080840152610c226080840182610bbc565b5f5f5f60608486031215610d49575f5ffd5b8335610d5481610b33565b92506020840135915060408401356001600160401b03811115610d75575f5ffd5b610d8186828701610c3e565b9150509250925092565b60018060a01b0385168152836020820152608060408201525f610db16080830185610bbc565b905060028310610dcf57634e487b7160e01b5f52602160045260245ffd5b82606083015295945050505050565b5f60208284031215610dee575f5ffd5b81518015158114610b65575f5ffd5b600181811c90821680610e1157607f821691505b602082108103610e2f57634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160401b038181168382160190811115610a7f57634e487b7160e01b5f52601160045260245ffd5b601f821115610ea757805f5260205f20601f840160051c81016020851015610e855750805b601f840160051c820191505b81811015610ea4575f8155600101610e91565b50505b505050565b81516001600160401b03811115610ec557610ec5610c2a565b610ed981610ed38454610dfd565b84610e60565b6020601f821160018114610f0b575f8315610ef45750848201515b5f19600385901b1c1916600184901b178455610ea4565b5f84815260208120601f198516915b82811015610f3a5787850151825560209485019460019092019101610f1a565b5084821015610f5757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b602081525f610b656020830184610bbc565b634e487b7160e01b5f52603260045260245ffdfea2646970667358221220bbde014569881dea4e215c2e9eff7b3e20f2b78ed325ca7ddb6470d2f70eb50664736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xBEW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0|W\x80c\xCE(\x96\x12\x11a\0WW\x80c\xCE(\x96\x12\x14a\x02CW\x80c\xE3\x1E\x07\x88\x14a\x02pW\x80c\xE3(\xEDw\x14a\x02\x83W\x80c\xF7\xEE\x94L\x14a\x02\xAFW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x01\xE6W\x80c\x91;\x1F\xBF\x14a\x02\x05W\x80c\xC4\xD6m\xE8\x14a\x02$W__\xFD[\x80bs\xE1\xD7\x14a\0\xC2W\x80c\x15\x8E\xF9>\x14a\0\xE3W\x80c\x18o\x03T\x14a\x01\x18W\x80c3\x83\xAB\xFE\x14a\x01NW\x80cB\"\x7F\xA4\x14a\x01\x90W\x80c\x8Di\xE9^\x14a\x01\xC7W[__\xFD[4\x80\x15a\0\xCDW__\xFD[Pa\0\xE1a\0\xDC6`\x04a\n\xC9V[a\x02\xCEV[\0[4\x80\x15a\0\xEEW__\xFD[P`\x02Ta\x01\x03\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01#W__\xFD[P_Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[4\x80\x15a\x01YW__\xFD[Pa\x01\x82a\x01h6`\x04a\x0BJV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x04` R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\x0FV[4\x80\x15a\x01\x9BW__\xFD[P`\x05Ta\x01\xAF\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x0FV[4\x80\x15a\x01\xD2W__\xFD[P`\x02Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x01\xF1W__\xFD[P`\x01Ta\x016\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x02\x10W__\xFD[Pa\x01\xAFa\x02\x1F6`\x04a\x0BlV[a\x04vV[4\x80\x15a\x02/W__\xFD[Pa\0\xE1a\x02>6`\x04a\x0BJV[a\x04\xBDV[4\x80\x15a\x02NW__\xFD[Pa\x02ba\x02]6`\x04a\x0B\x96V[a\x05\xEEV[`@Qa\x01\x0F\x92\x91\x90a\x0B\xFFV[a\0\xE1a\x02~6`\x04a\x0C\xC7V[a\x06\x9AV[4\x80\x15a\x02\x8EW__\xFD[Pa\x02\xA2a\x02\x9D6`\x04a\x0B\x96V[a\x08\xE7V[`@Qa\x01\x0F\x91\x90a\x0C\xF8V[4\x80\x15a\x02\xBAW__\xFD[Pa\x01\xAFa\x02\xC96`\x04a\x0BlV[a\t\xD0V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x03Q\x86\x88\x01\x88a\r7V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\x03\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid target address`P\x1B`D\x82\x01R`d\x01a\x039V[_\x80T`@QcF\x87!\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\x87!\xA7\x90a\x03\xDD\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\r\x8BV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\xF9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x1D\x91\x90a\r\xDEV[\x90P\x80a\x04lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FModule transaction failed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x039V[PPPPPPPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x04\x8FW_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FOnly owner can call this functio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x01a\x039V[`\x02T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x039V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x05\xC7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x039V[`\x02\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x06\x19\x90a\r\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06E\x90a\r\xFDV[\x80\x15a\x06\x90W\x80`\x1F\x10a\x06gWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\x90V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06sW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[4g\x01cEx]\x8A\0\0\x14a\x06\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FPayment must be exactly 0.1 ETH\0`D\x82\x01R`d\x01a\x039V[_\x80T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x904\x90\x83\x81\x81\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x07;W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x07@V[``\x91P[PP\x90P\x80a\x07\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FETH transfer to Safe failed\0\0\0\0\0`D\x82\x01R`d\x01a\x039V[`\x05Ta\x07\xA8\x90`\x01`\x01`@\x1B\x03\x16`\x01a\x0E5V[`\x05\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q\x80\x82\x01\x82R3\x81R` \x80\x82\x01\x86\x81R_\x85\x81R`\x03\x90\x92R\x92\x90 \x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x91Q\x90\x91\x82\x91`\x01\x82\x01\x90a\x08\x19\x90\x82a\x0E\xACV[PP3_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x80T`\x01\x81\x01\x82U\x90\x85R\x93\x82\x90 \x92\x84\x04\x90\x92\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x97\x16\x96\x90\x96\x02a\x01\0\n\x86\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x81Q``\x81\x01\x83R\x92\x83R\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x82\x01R\x84\x81\x01Q\x83\x83\x01R\x90Q\x91\x92P\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x91a\x08\xBE\x91\x84\x91\x01a\x0C\xF8V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x08\xD8\x91a\x0FfV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x16\x80\x83R`\x03\x82R\x91\x85\x90 \x85Q\x93\x84\x01\x86R\x91\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x01\x81\x01\x80T\x93\x94\x91\x93\x91\x83\x01\x91a\tI\x90a\r\xFDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tu\x90a\r\xFDV[\x80\x15a\t\xC0W\x80`\x1F\x10a\t\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x04` R`@\x81 T\x82\x10a\n,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01RrIndex out of bounds`h\x1B`D\x82\x01R`d\x01a\x039V[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x04` R`@\x90 \x80T\x83\x90\x81\x10a\nUWa\nUa\x0FxV[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x90P[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\n\x95W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xABW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\xC2W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\n\xDCW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xF1W__\xFD[a\n\xFD\x87\x82\x88\x01a\n\x85V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\x1BW__\xFD[a\x0B'\x87\x82\x88\x01a\n\x85V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0BGW__\xFD[PV[_` \x82\x84\x03\x12\x15a\x0BZW__\xFD[\x815a\x0Be\x81a\x0B3V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x0B}W__\xFD[\x825a\x0B\x88\x81a\x0B3V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x0B\xA6W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0BeW__\xFD[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x0B\xE0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\xC4V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x0C\"\x90\x83\x01\x84a\x0B\xBCV[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\x0CMW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0CfWa\x0Cfa\x0C*V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x94Wa\x0C\x94a\x0C*V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\x0C\xABW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0C\xD7W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xECW__\xFD[a\x0C\"\x84\x82\x85\x01a\x0C>V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x0C\"`\x80\x84\x01\x82a\x0B\xBCV[___``\x84\x86\x03\x12\x15a\rIW__\xFD[\x835a\rT\x81a\x0B3V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\ruW__\xFD[a\r\x81\x86\x82\x87\x01a\x0C>V[\x91PP\x92P\x92P\x92V[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_a\r\xB1`\x80\x83\x01\x85a\x0B\xBCV[\x90P`\x02\x83\x10a\r\xCFWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82``\x83\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\r\xEEW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0BeW__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0E\x11W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0E/WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\n\x7FWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x1F\x82\x11\x15a\x0E\xA7W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x0E\x85WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0E\xA4W_\x81U`\x01\x01a\x0E\x91V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\xC5Wa\x0E\xC5a\x0C*V[a\x0E\xD9\x81a\x0E\xD3\x84Ta\r\xFDV[\x84a\x0E`V[` `\x1F\x82\x11`\x01\x81\x14a\x0F\x0BW_\x83\x15a\x0E\xF4WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0E\xA4V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x0F:W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x0F\x1AV[P\x84\x82\x10\x15a\x0FWW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[` \x81R_a\x0Be` \x83\x01\x84a\x0B\xBCV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xBB\xDE\x01Ei\x88\x1D\xEAN!\\.\x9E\xFF{> \xF2\xB7\x8E\xD3%\xCA}\xDBdp\xD2\xF7\x0E\xB5\x06dsolcC\0\x08\x1C\x003",
    );
    /**Event with signature `NewTrigger(bytes)` and selector `0x86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e68`.
    ```solidity
    event NewTrigger(bytes);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct NewTrigger {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NewTrigger {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NewTrigger(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    134u8, 234u8, 205u8, 35u8, 97u8, 13u8, 129u8, 112u8, 101u8, 22u8, 222u8, 30u8,
                    208u8, 71u8, 108u8, 135u8, 119u8, 47u8, 223u8, 147u8, 156u8, 124u8, 119u8,
                    31u8, 187u8, 215u8, 240u8, 35u8, 13u8, 97u8, 158u8, 104u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    &self._0,
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
    /**Constructor`.
    ```solidity
    constructor(address _safe);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _safe: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._safe,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _safe: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self._safe,
                ),)
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
    /**Function with signature `nextTriggerId()` and selector `0x42227fa4`.
    ```solidity
    function nextTriggerId() external view returns (ISimpleTrigger.TriggerId);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdCall {}
    ///Container type for the return parameters of the [`nextTriggerId()`](nextTriggerIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTriggerIdReturn {
        pub _0: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<nextTriggerIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<nextTriggerIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextTriggerIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTriggerIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextTriggerIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextTriggerIdReturn;
            type ReturnTuple<'a> = (ISimpleTrigger::TriggerId,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextTriggerId()";
            const SELECTOR: [u8; 4] = [66u8, 34u8, 127u8, 164u8];
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
    /**Function with signature `safe()` and selector `0x186f0354`.
    ```solidity
    function safe() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeCall {}
    ///Container type for the return parameters of the [`safe()`](safeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct safeReturn {
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
            impl ::core::convert::From<safeCall> for UnderlyingRustTuple<'_> {
                fn from(value: safeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeCall {
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
            impl ::core::convert::From<safeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: safeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for safeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for safeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = safeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "safe()";
            const SELECTOR: [u8; 4] = [24u8, 111u8, 3u8, 84u8];
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
    function triggersById(ISimpleTrigger.TriggerId) external view returns (address creator, bytes memory data);
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Address, alloy::sol_types::private::Bytes);
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
                    (value.creator, value.data)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for triggersByIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { creator: tuple.0, data: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for triggersByIdCall {
            type Parameters<'a> = (ISimpleTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = triggersByIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
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
    ///Container for all the [`SafeModule`](self) function calls.
    pub enum SafeModuleCalls {
        addTrigger(addTriggerCall),
        getTrigger(getTriggerCall),
        getTriggerCount(getTriggerCountCall),
        getTriggerIdAtIndex(getTriggerIdAtIndexCall),
        handleAddPayload(handleAddPayloadCall),
        initialize(initializeCall),
        initialized(initializedCall),
        nextTriggerId(nextTriggerIdCall),
        owner(ownerCall),
        safe(safeCall),
        serviceProvider(serviceProviderCall),
        triggerIdsByCreator(triggerIdsByCreatorCall),
        triggersById(triggersByIdCall),
    }
    #[automatically_derived]
    impl SafeModuleCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 115u8, 225u8, 215u8],
            [21u8, 142u8, 249u8, 62u8],
            [24u8, 111u8, 3u8, 84u8],
            [51u8, 131u8, 171u8, 254u8],
            [66u8, 34u8, 127u8, 164u8],
            [141u8, 105u8, 233u8, 94u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 59u8, 31u8, 191u8],
            [196u8, 214u8, 109u8, 232u8],
            [206u8, 40u8, 150u8, 18u8],
            [227u8, 30u8, 7u8, 136u8],
            [227u8, 40u8, 237u8, 119u8],
            [247u8, 238u8, 148u8, 76u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeModuleCalls {
        const NAME: &'static str = "SafeModuleCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addTrigger(_) => <addTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTrigger(_) => <getTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTriggerCount(_) => {
                    <getTriggerCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTriggerIdAtIndex(_) => {
                    <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::handleAddPayload(_) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialized(_) => <initializedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nextTriggerId(_) => <nextTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceProvider(_) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggerIdsByCreator(_) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggersById(_) => <triggersByIdCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<SafeModuleCalls>] = &[
                {
                    fn handleAddPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::handleAddPayload)
                    }
                    handleAddPayload
                },
                {
                    fn initialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <initializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::initialized)
                    }
                    initialized
                },
                {
                    fn safe(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeModuleCalls::safe)
                    }
                    safe
                },
                {
                    fn getTriggerCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <getTriggerCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::getTriggerCount)
                    }
                    getTriggerCount
                },
                {
                    fn nextTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::nextTriggerId)
                    }
                    nextTriggerId
                },
                {
                    fn serviceProvider(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <serviceProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::serviceProvider)
                    }
                    serviceProvider
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeModuleCalls::owner)
                    }
                    owner
                },
                {
                    fn triggerIdsByCreator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::triggerIdsByCreator)
                    }
                    triggerIdsByCreator
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeModuleCalls::initialize)
                    }
                    initialize
                },
                {
                    fn triggersById(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <triggersByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::triggersById)
                    }
                    triggersById
                },
                {
                    fn addTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <addTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeModuleCalls::addTrigger)
                    }
                    addTrigger
                },
                {
                    fn getTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <getTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeModuleCalls::getTrigger)
                    }
                    getTrigger
                },
                {
                    fn getTriggerIdAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <getTriggerIdAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeModuleCalls::getTriggerIdAtIndex)
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
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::handleAddPayload(inner) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialized(inner) => {
                    <initializedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addTrigger(inner) => {
                    <addTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::handleAddPayload(inner) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initialized(inner) => {
                    <initializedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nextTriggerId(inner) => {
                    <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SafeModule`](self) events.
    pub enum SafeModuleEvents {
        NewTrigger(NewTrigger),
    }
    #[automatically_derived]
    impl SafeModuleEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[[
            134u8, 234u8, 205u8, 35u8, 97u8, 13u8, 129u8, 112u8, 101u8, 22u8, 222u8, 30u8, 208u8,
            71u8, 108u8, 135u8, 119u8, 47u8, 223u8, 147u8, 156u8, 124u8, 119u8, 31u8, 187u8, 215u8,
            240u8, 35u8, 13u8, 97u8, 158u8, 104u8,
        ]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SafeModuleEvents {
        const NAME: &'static str = "SafeModuleEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<NewTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTrigger)
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
    impl alloy_sol_types::private::IntoLogData for SafeModuleEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SafeModule`](self) contract instance.

    See the [wrapper's documentation](`SafeModuleInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SafeModuleInstance<T, P, N> {
        SafeModuleInstance::<T, P, N>::new(address, provider)
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
        _safe: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<SafeModuleInstance<T, P, N>>>
    {
        SafeModuleInstance::<T, P, N>::deploy(provider, _safe)
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
        _safe: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SafeModuleInstance::<T, P, N>::deploy_builder(provider, _safe)
    }
    /**A [`SafeModule`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`SafeModule`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SafeModuleInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SafeModuleInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SafeModuleInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SafeModuleInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SafeModule`](self) contract instance.

        See the [wrapper's documentation](`SafeModuleInstance`) for more details.*/
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
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SafeModuleInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _safe);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _safe: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall { _safe })[..],
                ]
                .concat()
                .into(),
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
    impl<T, P: ::core::clone::Clone, N> SafeModuleInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SafeModuleInstance<T, P, N> {
            SafeModuleInstance {
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
        > SafeModuleInstance<T, P, N>
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
        ///Creates a new call builder for the [`addTrigger`] function.
        pub fn addTrigger(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, addTriggerCall, N> {
            self.call_builder(&addTriggerCall { data })
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
        ///Creates a new call builder for the [`handleAddPayload`] function.
        pub fn handleAddPayload(
            &self,
            data: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleAddPayloadCall, N> {
            self.call_builder(&handleAddPayloadCall { data, signature })
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
        ///Creates a new call builder for the [`nextTriggerId`] function.
        pub fn nextTriggerId(&self) -> alloy_contract::SolCallBuilder<T, &P, nextTriggerIdCall, N> {
            self.call_builder(&nextTriggerIdCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`safe`] function.
        pub fn safe(&self) -> alloy_contract::SolCallBuilder<T, &P, safeCall, N> {
            self.call_builder(&safeCall {})
        }
        ///Creates a new call builder for the [`serviceProvider`] function.
        pub fn serviceProvider(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceProviderCall, N> {
            self.call_builder(&serviceProviderCall {})
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SafeModuleInstance<T, P, N>
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
        ///Creates a new event filter for the [`NewTrigger`] event.
        pub fn NewTrigger_filter(&self) -> alloy_contract::Event<T, &P, NewTrigger, N> {
            self.event_filter::<NewTrigger>()
        }
    }
}
