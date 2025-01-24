///Module containing a contract's types and functions.
/**

```solidity
library Enum {
    type Operation is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod Enum {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operation(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Operation> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl Operation {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
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
        impl alloy_sol_types::SolType for Operation {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(address: alloy_sol_types::private::Address, provider: P) -> EnumInstance<T, P, N> {
        EnumInstance::<T, P, N>::new(address, provider)
    }
    /**A [`Enum`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Enum`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EnumInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EnumInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EnumInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EnumInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

See the [wrapper's documentation](`EnumInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
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
    impl<T, P: ::core::clone::Clone, N> EnumInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EnumInstance<T, P, N> {
            EnumInstance {
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
    > EnumInstance<T, P, N> {
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
    > EnumInstance<T, P, N> {
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
///Module containing a contract's types and functions.
/**

```solidity
library IWavsSDK {
    struct SignedPayload { bytes data; bytes signature; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IWavsSDK {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignedPayload { bytes data; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedPayload {
        pub data: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignedPayload> for UnderlyingRustTuple<'_> {
            fn from(value: SignedPayload) -> Self {
                (value.data, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedPayload {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    data: tuple.0,
                    signature: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedPayload {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedPayload {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
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
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignedPayload {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignedPayload {
            const NAME: &'static str = "SignedPayload";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedPayload(bytes data,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignedPayload {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IWavsSDK`](self) contract instance.

See the [wrapper's documentation](`IWavsSDKInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IWavsSDKInstance<T, P, N> {
        IWavsSDKInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IWavsSDK`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IWavsSDK`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IWavsSDKInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IWavsSDKInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IWavsSDKInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IWavsSDKInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IWavsSDK`](self) contract instance.

See the [wrapper's documentation](`IWavsSDKInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
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
    impl<T, P: ::core::clone::Clone, N> IWavsSDKInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IWavsSDKInstance<T, P, N> {
            IWavsSDKInstance {
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
    > IWavsSDKInstance<T, P, N> {
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
    > IWavsSDKInstance<T, P, N> {
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
library Enum {
    type Operation is uint8;
}

library IWavsSDK {
    struct SignedPayload {
        bytes data;
        bytes signature;
    }
}

interface SafeGuard {
    type ValidationStatus is uint8;

    error AsyncValidationRequired();
    error InvalidSignature();
    error TransactionExpired();
    error TransactionNotFound();

    event ValidationRequired(bytes32 indexed txHash, address indexed to, uint256 value, bytes data, Enum.Operation operation, address initiator, uint256 estimatedProcessingTime);
    event ValidationStatusUpdated(bytes32 indexed txHash, ValidationStatus status, string message);
    event WavsTriggerEvent(bytes);

    constructor(address _safe, address _stakeRegistry);

    function STAKE_REGISTRY() external view returns (address);
    function addPayload(IWavsSDK.SignedPayload memory signedPayload) external;
    function addPayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external;
    function checkAfterExecution(bytes32 txHash, bool success) external;
    function checkTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address payable refundReceiver, bytes memory signatures, address msgSender) external;
    function estimatedProcessingTime() external view returns (uint256);
    function getTransactionStatus(bytes32 txHash) external view returns (ValidationStatus status, string memory message, uint256 remainingTime);
    function getUserPendingTransactions(address user) external view returns (bytes32[] memory);
    function safe() external view returns (address);
    function supportsInterface(bytes4 interfaceId) external pure returns (bool);
    function txDetails(bytes32) external view returns (address to, uint256 value, bytes memory data, Enum.Operation operation, address initiator, uint256 timestamp, ValidationStatus status, string memory lastStatusMessage, uint256 expirationTime);
    function userPendingTxs(address, uint256) external view returns (bytes32);
    function validatePayload(IWavsSDK.SignedPayload memory signedPayload) external view returns (bool);
    function validatePayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external view returns (bool);
    function validatedTxs(bytes32) external view returns (bool);
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
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "STAKE_REGISTRY",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ECDSAStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "addPayload",
    "inputs": [
      {
        "name": "signedPayload",
        "type": "tuple",
        "internalType": "struct IWavsSDK.SignedPayload",
        "components": [
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
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addPayloadMulti",
    "inputs": [
      {
        "name": "signedPayloads",
        "type": "tuple[]",
        "internalType": "struct IWavsSDK.SignedPayload[]",
        "components": [
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
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkAfterExecution",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkTransaction",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      },
      {
        "name": "safeTxGas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "baseGas",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasPrice",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "gasToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "refundReceiver",
        "type": "address",
        "internalType": "address payable"
      },
      {
        "name": "signatures",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "msgSender",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "estimatedProcessingTime",
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
    "name": "getTransactionStatus",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum SafeGuard.ValidationStatus"
      },
      {
        "name": "message",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "remainingTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getUserPendingTransactions",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "txDetails",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "internalType": "enum Enum.Operation"
      },
      {
        "name": "initiator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "timestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum SafeGuard.ValidationStatus"
      },
      {
        "name": "lastStatusMessage",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "expirationTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "userPendingTxs",
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
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatePayload",
    "inputs": [
      {
        "name": "signedPayload",
        "type": "tuple",
        "internalType": "struct IWavsSDK.SignedPayload",
        "components": [
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
        ]
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
    "name": "validatePayloadMulti",
    "inputs": [
      {
        "name": "signedPayloads",
        "type": "tuple[]",
        "internalType": "struct IWavsSDK.SignedPayload[]",
        "components": [
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
        ]
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
    "name": "validatedTxs",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "type": "event",
    "name": "ValidationRequired",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "data",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "operation",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum Enum.Operation"
      },
      {
        "name": "initiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "estimatedProcessingTime",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidationStatusUpdated",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "status",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum SafeGuard.ValidationStatus"
      },
      {
        "name": "message",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WavsTriggerEvent",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AsyncValidationRequired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TransactionExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TransactionNotFound",
    "inputs": []
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
pub mod SafeGuard {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405260785f55348015610013575f5ffd5b50604051611c86380380611c86833981016040819052610032916100bf565b6001600160a01b0380821660805282166100925760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420736166652061646472657373000000000000000000000000604482015260640160405180910390fd5b506001600160a01b031660a0526100f0565b80516001600160a01b03811681146100ba575f5ffd5b919050565b5f5f604083850312156100d0575f5ffd5b6100d9836100a4565b91506100e7602084016100a4565b90509250929050565b60805160a051611b606101265f395f81816101320152818161048101526109e101525f81816101ac015261034b0152611b605ff3fe608060405234801561000f575f5ffd5b50600436106100f0575f3560e01c806393271368116100935780639aa9fda5116100635780639aa9fda51461026e5780639e83e30614610281578063a71f8da0146102a1578063c56fb0fd146102b4575f5ffd5b806393271368146101f6578063944074651461020957806397f5365a1461022b57806399d7cf4b1461024c575f5ffd5b806363710c05116100ce57806363710c051461017f57806375f0bb5214610194578063761de19f146101a75780637b4f3373146101ce575f5ffd5b806301ffc9a7146100f4578063186f03541461012d578063216a3e9a1461016c575b5f5ffd5b61011861010236600461113a565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b6101547f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610124565b61011861017a36600461115c565b6102bc565b61019261018d366004611193565b6103f2565b005b6101926101a23660046112df565b610476565b6101547f000000000000000000000000000000000000000000000000000000000000000081565b6101e16101dc3660046113bc565b610863565b60405161012499989796959493929190611458565b6101926102043660046114e7565b6109d6565b61021c6102173660046113bc565b610b6e565b60405161012493929190611515565b61023e610239366004611544565b610d0f565b604051908152602001610124565b61011861025a3660046113bc565b60016020525f908152604090205460ff1681565b61019261027c36600461115c565b610d3a565b61029461028f36600461156e565b610d6e565b6040516101249190611589565b6101186102af366004611193565b610dd7565b61023e5f5481565b5f806102c883806115cb565b6040516102d6929190611615565b604051809103902090505f610337826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e8361037e60208901896115cb565b6040518463ffffffff1660e01b815260040161039c9392919061164c565b602060405180830381865afa1580156103b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103db919061166e565b6001600160e01b0319918216911614949350505050565b81816103fe8282610dd7565b61041b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff811684111561046f5761045d85858363ffffffff1681811061044657610446611689565b9050602002810190610458919061169d565b610e9e565b80610467816116cf565b91505061041d565b5050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104e25760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064015b60405180910390fd5b5f8b8b8b8b85426040516020016104fe969594939291906116f3565b60408051601f1981840301815291815281516020928301205f818152600190935291205490915060ff16156105335750610856565b5f818152600260205260409020600401541580159061056157505f8181526002602052604090206007015442115b1561057f576040516338e5e54b60e21b815260040160405180910390fd5b6040518061012001604052808d6001600160a01b031681526020018c81526020018b81526020018a60018111156105b8576105b8611420565b81526001600160a01b0384166020820152426040820152606001600181526020016040518060400160405280601681526020017556616c69646174696f6e20696e2070726f677265737360501b81525081526020015f544261061a919061176a565b90525f82815260026020818152604092839020845181546001600160a01b0319166001600160a01b03909116178155908401516001820155918301519082019061066490826117fe565b50606082015160038201805460ff19166001838181111561068757610687611420565b021790555060808201518160030160016101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a0820151816004015560c0820151816005015f6101000a81548160ff021916908360048111156106ec576106ec611420565b021790555060e0820151600682019061070590826117fe565b50610100820151816007015590505060035f836001600160a01b03166001600160a01b031681526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f90919091909150557f37b1671f777b1ea11710dc816b92da0f8b5ea94730552bb5637b0ec536811908818d8d8d8d87604051602001610797969594939291906118b9565b60408051601f19818403018152908290526107b19161192d565b60405180910390a18b6001600160a01b0316817f72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb8d8d8d875f546040516107fc95949392919061193f565b60405180910390a3807f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd960016040516108359190611982565b60405180910390a26040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b600260208190525f91825260409091208054600182015492820180546001600160a01b039092169392916108969061177d565b80601f01602080910402602001604051908101604052809291908181526020018280546108c29061177d565b801561090d5780601f106108e45761010080835404028352916020019161090d565b820191905f5260205f20905b8154815290600101906020018083116108f057829003601f168201915b50505050600383015460048401546005850154600686018054959660ff808616976101009096046001600160a01b031696509394939092169261094f9061177d565b80601f016020809104026020016040519081016040528092919081815260200182805461097b9061177d565b80156109c65780601f1061099d576101008083540402835291602001916109c6565b820191905f5260205f20905b8154815290600101906020018083116109a957829003601f168201915b5050505050908060070154905089565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a3d5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016104d9565b5f8281526002602052604090206003810154610a679061010090046001600160a01b031684611013565b7f37b1671f777b1ea11710dc816b92da0f8b5ea94730552bb5637b0ec5368119088383604051602001610ac39291906060808252600490820152631c1bdcdd60e21b608082015260208101929092521515604082015260a00190565b60408051601f1981840301815290829052610add9161192d565b60405180910390a15f838152600260208190526040822080546001600160a01b0319168155600181018390559190610b17908301826110d3565b6003820180546001600160a81b03191690555f6004830181905560058301805460ff19169055610b4b9060068401906110d3565b505f6007919091018190559283525050600160205260409020805460ff19169055565b5f818152600260205260408120600481015460609183918203610ba657505060408051602081019091525f8082529250905081610d08565b8060070154421115610c4f576004816006015f818054610bc59061177d565b80601f0160208091040260200160405190810160405280929190818152602001828054610bf19061177d565b8015610c3c5780601f10610c1357610100808354040283529160200191610c3c565b820191905f5260205f20905b815481529060010190602001808311610c1f57829003601f168201915b5050505050915093509350935050610d08565b5f428260070154610c6091906119c2565b600583015460068401805492935060ff9091169183908290610c819061177d565b80601f0160208091040260200160405190810160405280929190818152602001828054610cad9061177d565b8015610cf85780601f10610ccf57610100808354040283529160200191610cf8565b820191905f5260205f20905b815481529060010190602001808311610cdb57829003601f168201915b5050505050915094509450945050505b9193909250565b6003602052815f5260405f208181548110610d28575f80fd5b905f5260205f20015f91509150505481565b80610d44816102bc565b610d6157604051638baa579f60e01b815260040160405180910390fd5b610d6a82610e9e565b5050565b6001600160a01b0381165f90815260036020908152604091829020805483518184028101840190945280845260609392830182828015610dcb57602002820191905f5260205f20905b815481526020019060010190808311610db7575b50505050509050919050565b5f805b63ffffffff8116831115610e92573063216a3e9a858563ffffffff8516818110610e0657610e06611689565b9050602002810190610e18919061169d565b6040518263ffffffff1660e01b8152600401610e349190611a17565b602060405180830381865afa158015610e4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e739190611a6c565b610e80575f915050610e98565b80610e8a816116cf565b915050610dda565b50600190505b92915050565b5f8080610eab84806115cb565b810190610eb89190611a87565b5f838152600260205260408120600481015494975092955090935090919003610ef4576040516331fb878f60e01b815260040160405180910390fd5b8060070154421115610f665760058101805460ff1916600490811790915560405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd991610f4591908690611aef565b60405180910390a26040516338e5e54b60e21b815260040160405180910390fd5b82610f72576003610f75565b60025b60058201805460ff19166001836004811115610f9357610f93611420565b021790555060068101610fa683826117fe565b508215610fc9575f848152600160208190526040909120805460ff191690911790555b600581015460405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd9916110049160ff16908690611aef565b60405180910390a25050505050565b6001600160a01b0382165f908152600360205260408120905b81548110156110cd578282828154811061104857611048611689565b905f5260205f200154036110c55781548290611066906001906119c2565b8154811061107657611076611689565b905f5260205f20015482828154811061109157611091611689565b905f5260205f200181905550818054806110ad576110ad611b16565b600190038181905f5260205f20015f905590556110cd565b60010161102c565b50505050565b5080546110df9061177d565b5f825580601f106110ee575050565b601f0160209004905f5260205f209081019061110a919061110d565b50565b5b80821115611121575f815560010161110e565b5090565b6001600160e01b03198116811461110a575f5ffd5b5f6020828403121561114a575f5ffd5b813561115581611125565b9392505050565b5f6020828403121561116c575f5ffd5b813567ffffffffffffffff811115611182575f5ffd5b820160408185031215611155575f5ffd5b5f5f602083850312156111a4575f5ffd5b823567ffffffffffffffff8111156111ba575f5ffd5b8301601f810185136111ca575f5ffd5b803567ffffffffffffffff8111156111e0575f5ffd5b8560208260051b84010111156111f4575f5ffd5b6020919091019590945092505050565b6001600160a01b038116811461110a575f5ffd5b803561122381611204565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f67ffffffffffffffff84111561125657611256611228565b50604051601f19601f85018116603f0116810181811067ffffffffffffffff8211171561128557611285611228565b60405283815290508082840185101561129c575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f8301126112c2575f5ffd5b6111558383356020850161123c565b803560028110611223575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e0312156112fa575f5ffd5b6113038c611218565b9a5060208c0135995060408c013567ffffffffffffffff811115611325575f5ffd5b6113318e828f016112b3565b99505061134060608d016112d1565b975060808c0135965060a08c0135955060c08c0135945061136360e08d01611218565b93506113726101008d01611218565b92506101208c013567ffffffffffffffff81111561138e575f5ffd5b61139a8e828f016112b3565b9250506113aa6101408d01611218565b90509295989b509295989b9093969950565b5f602082840312156113cc575f5ffd5b5035919050565b5f5b838110156113ed5781810151838201526020016113d5565b50505f910152565b5f815180845261140c8160208601602086016113d3565b601f01601f19169290920160200192915050565b634e487b7160e01b5f52602160045260245ffd5b6002811061144457611444611420565b9052565b6005811061144457611444611420565b60018060a01b038a16815288602082015261012060408201525f61148061012083018a6113f5565b61148d606084018a611434565b6001600160a01b038816608084015260a083018790526114b060c0840187611448565b82810360e08401526114c281866113f5565b915050826101008301529a9950505050505050505050565b801515811461110a575f5ffd5b5f5f604083850312156114f8575f5ffd5b82359150602083013561150a816114da565b809150509250929050565b61151f8185611448565b606060208201525f61153460608301856113f5565b9050826040830152949350505050565b5f5f60408385031215611555575f5ffd5b823561156081611204565b946020939093013593505050565b5f6020828403121561157e575f5ffd5b813561115581611204565b602080825282518282018190525f918401906040840190835b818110156115c05783518352602093840193909201916001016115a2565b509095945050505050565b5f5f8335601e198436030181126115e0575f5ffd5b83018035915067ffffffffffffffff8211156115fa575f5ffd5b60200191503681900382131561160e575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f611665604083018486611624565b95945050505050565b5f6020828403121561167e575f5ffd5b815161115581611125565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126116b1575f5ffd5b9190910192915050565b634e487b7160e01b5f52601160045260245ffd5b5f63ffffffff821663ffffffff81036116ea576116ea6116bb565b60010192915050565b6bffffffffffffffffffffffff198760601b1681528560148201525f8551611722816034850160208a016113d3565b82016002861061173457611734611420565b60f89590951b6034860152505060609190911b6bffffffffffffffffffffffff1916603583015260498201526069019392505050565b80820180821115610e9857610e986116bb565b600181811c9082168061179157607f821691505b6020821081036117af57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156117f957805f5260205f20601f840160051c810160208510156117da5750805b601f840160051c820191505b8181101561046f575f81556001016117e6565b505050565b815167ffffffffffffffff81111561181857611818611228565b61182c81611826845461177d565b846117b5565b6020601f82116001811461185e575f83156118475750848201515b5f19600385901b1c1916600184901b17845561046f565b5f84815260208120601f198516915b8281101561188d578785015182556020948501946001909201910161186d565b50848210156118aa57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60e08152600360e08201526270726560e81b61010082015286602082015260018060a01b038616604082015284606082015261012060808201525f6119026101208301866113f5565b905061191160a0830185611434565b6001600160a01b039290921660c0919091015295945050505050565b602081525f61115560208301846113f5565b85815260a060208201525f61195760a08301876113f5565b90506119666040830186611434565b6001600160a01b03939093166060820152608001529392505050565b61198c8183611448565b6040602082018190526016908201527556616c69646174696f6e20696e2070726f677265737360501b6060820152608001919050565b81810381811115610e9857610e986116bb565b5f5f8335601e198436030181126119ea575f5ffd5b830160208101925035905067ffffffffffffffff811115611a09575f5ffd5b80360382131561160e575f5ffd5b602081525f611a2683846119d5565b60406020850152611a3b606085018284611624565b915050611a4b60208501856119d5565b848303601f19016040860152611a62838284611624565b9695505050505050565b5f60208284031215611a7c575f5ffd5b8151611155816114da565b5f5f5f60608486031215611a99575f5ffd5b833592506020840135611aab816114da565b9150604084013567ffffffffffffffff811115611ac6575f5ffd5b8401601f81018613611ad6575f5ffd5b611ae58682356020840161123c565b9150509250925092565b611af98184611448565b604060208201525f611b0e60408301846113f5565b949350505050565b634e487b7160e01b5f52603160045260245ffdfea2646970667358221220ed7324760f0f7f908aec1df3d7938aef1776edb50a3ccdf8bbc607f90cb2c5d464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R`x_U4\x80\x15a\0\x13W__\xFD[P`@Qa\x1C\x868\x03\x80a\x1C\x86\x839\x81\x01`@\x81\x90Ra\x002\x91a\0\xBFV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\x80R\x82\x16a\0\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid safe address\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\xA0Ra\0\xF0V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xBAW__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0\xD0W__\xFD[a\0\xD9\x83a\0\xA4V[\x91Pa\0\xE7` \x84\x01a\0\xA4V[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x1B`a\x01&_9_\x81\x81a\x012\x01R\x81\x81a\x04\x81\x01Ra\t\xE1\x01R_\x81\x81a\x01\xAC\x01Ra\x03K\x01Ra\x1B`_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xF0W_5`\xE0\x1C\x80c\x93'\x13h\x11a\0\x93W\x80c\x9A\xA9\xFD\xA5\x11a\0cW\x80c\x9A\xA9\xFD\xA5\x14a\x02nW\x80c\x9E\x83\xE3\x06\x14a\x02\x81W\x80c\xA7\x1F\x8D\xA0\x14a\x02\xA1W\x80c\xC5o\xB0\xFD\x14a\x02\xB4W__\xFD[\x80c\x93'\x13h\x14a\x01\xF6W\x80c\x94@te\x14a\x02\tW\x80c\x97\xF56Z\x14a\x02+W\x80c\x99\xD7\xCFK\x14a\x02LW__\xFD[\x80ccq\x0C\x05\x11a\0\xCEW\x80ccq\x0C\x05\x14a\x01\x7FW\x80cu\xF0\xBBR\x14a\x01\x94W\x80cv\x1D\xE1\x9F\x14a\x01\xA7W\x80c{O3s\x14a\x01\xCEW__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xF4W\x80c\x18o\x03T\x14a\x01-W\x80c!j>\x9A\x14a\x01lW[__\xFD[a\x01\x18a\x01\x026`\x04a\x11:V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01$V[a\x01\x18a\x01z6`\x04a\x11\\V[a\x02\xBCV[a\x01\x92a\x01\x8D6`\x04a\x11\x93V[a\x03\xF2V[\0[a\x01\x92a\x01\xA26`\x04a\x12\xDFV[a\x04vV[a\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE1a\x01\xDC6`\x04a\x13\xBCV[a\x08cV[`@Qa\x01$\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x14XV[a\x01\x92a\x02\x046`\x04a\x14\xE7V[a\t\xD6V[a\x02\x1Ca\x02\x176`\x04a\x13\xBCV[a\x0BnV[`@Qa\x01$\x93\x92\x91\x90a\x15\x15V[a\x02>a\x0296`\x04a\x15DV[a\r\x0FV[`@Q\x90\x81R` \x01a\x01$V[a\x01\x18a\x02Z6`\x04a\x13\xBCV[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\x92a\x02|6`\x04a\x11\\V[a\r:V[a\x02\x94a\x02\x8F6`\x04a\x15nV[a\rnV[`@Qa\x01$\x91\x90a\x15\x89V[a\x01\x18a\x02\xAF6`\x04a\x11\x93V[a\r\xD7V[a\x02>_T\x81V[_\x80a\x02\xC8\x83\x80a\x15\xCBV[`@Qa\x02\xD6\x92\x91\x90a\x16\x15V[`@Q\x80\x91\x03\x90 \x90P_a\x037\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x03~` \x89\x01\x89a\x15\xCBV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x9C\x93\x92\x91\x90a\x16LV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x16nV[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[\x81\x81a\x03\xFE\x82\x82a\r\xD7V[a\x04\x1BW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04oWa\x04]\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04FWa\x04Fa\x16\x89V[\x90P` \x02\x81\x01\x90a\x04X\x91\x90a\x16\x9DV[a\x0E\x9EV[\x80a\x04g\x81a\x16\xCFV[\x91PPa\x04\x1DV[PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x8B\x8B\x8B\x8B\x85B`@Q` \x01a\x04\xFE\x96\x95\x94\x93\x92\x91\x90a\x16\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x01\x90\x93R\x91 T\x90\x91P`\xFF\x16\x15a\x053WPa\x08VV[_\x81\x81R`\x02` R`@\x90 `\x04\x01T\x15\x80\x15\x90a\x05aWP_\x81\x81R`\x02` R`@\x90 `\x07\x01TB\x11[\x15a\x05\x7FW`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01 \x01`@R\x80\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A`\x01\x81\x11\x15a\x05\xB8Wa\x05\xB8a\x14 V[\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01RB`@\x82\x01R``\x01`\x01\x81R` \x01`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uValidation in progress`P\x1B\x81RP\x81R` \x01_TBa\x06\x1A\x91\x90a\x17jV[\x90R_\x82\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q`\x01\x82\x01U\x91\x83\x01Q\x90\x82\x01\x90a\x06d\x90\x82a\x17\xFEV[P``\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x06\x87Wa\x06\x87a\x14 V[\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a\x06\xECWa\x06\xECa\x14 V[\x02\x17\x90UP`\xE0\x82\x01Q`\x06\x82\x01\x90a\x07\x05\x90\x82a\x17\xFEV[Pa\x01\0\x82\x01Q\x81`\x07\x01U\x90PP`\x03_\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x7F7\xB1g\x1Fw{\x1E\xA1\x17\x10\xDC\x81k\x92\xDA\x0F\x8B^\xA9G0U+\xB5c{\x0E\xC56\x81\x19\x08\x81\x8D\x8D\x8D\x8D\x87`@Q` \x01a\x07\x97\x96\x95\x94\x93\x92\x91\x90a\x18\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xB1\x91a\x19-V[`@Q\x80\x91\x03\x90\xA1\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81\x7Fr\xB8\xBE\xAA+\x16\xEF\xC2\x0F\xF7\xAE\xA9B\xA1\"\xF7\xB7\x81\x19rO\xAB\xBD\x80j\xCDd\xD7\x97\x89T\xCB\x8D\x8D\x8D\x87_T`@Qa\x07\xFC\x95\x94\x93\x92\x91\x90a\x19?V[`@Q\x80\x91\x03\x90\xA3\x80\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9`\x01`@Qa\x085\x91\x90a\x19\x82V[`@Q\x80\x91\x03\x90\xA2`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93\x92\x91a\x08\x96\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC2\x90a\x17}V[\x80\x15a\t\rW\x80`\x1F\x10a\x08\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01\x80T\x95\x96`\xFF\x80\x86\x16\x97a\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P\x93\x94\x93\x90\x92\x16\x92a\tO\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t{\x90a\x17}V[\x80\x15a\t\xC6W\x80`\x1F\x10a\t\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x07\x01T\x90P\x89V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x04\xD9V[_\x82\x81R`\x02` R`@\x90 `\x03\x81\x01Ta\ng\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a\x10\x13V[\x7F7\xB1g\x1Fw{\x1E\xA1\x17\x10\xDC\x81k\x92\xDA\x0F\x8B^\xA9G0U+\xB5c{\x0E\xC56\x81\x19\x08\x83\x83`@Q` \x01a\n\xC3\x92\x91\x90``\x80\x82R`\x04\x90\x82\x01Rc\x1C\x1B\xDC\xDD`\xE2\x1B`\x80\x82\x01R` \x81\x01\x92\x90\x92R\x15\x15`@\x82\x01R`\xA0\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\n\xDD\x91a\x19-V[`@Q\x80\x91\x03\x90\xA1_\x83\x81R`\x02` \x81\x90R`@\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U\x91\x90a\x0B\x17\x90\x83\x01\x82a\x10\xD3V[`\x03\x82\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U_`\x04\x83\x01\x81\x90U`\x05\x83\x01\x80T`\xFF\x19\x16\x90Ua\x0BK\x90`\x06\x84\x01\x90a\x10\xD3V[P_`\x07\x91\x90\x91\x01\x81\x90U\x92\x83RPP`\x01` R`@\x90 \x80T`\xFF\x19\x16\x90UV[_\x81\x81R`\x02` R`@\x81 `\x04\x81\x01T``\x91\x83\x91\x82\x03a\x0B\xA6WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x92P\x90P\x81a\r\x08V[\x80`\x07\x01TB\x11\x15a\x0COW`\x04\x81`\x06\x01_\x81\x80Ta\x0B\xC5\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xF1\x90a\x17}V[\x80\x15a\x0C<W\x80`\x1F\x10a\x0C\x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C<V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x1FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x93P\x93P\x93PPa\r\x08V[_B\x82`\x07\x01Ta\x0C`\x91\x90a\x19\xC2V[`\x05\x83\x01T`\x06\x84\x01\x80T\x92\x93P`\xFF\x90\x91\x16\x91\x83\x90\x82\x90a\x0C\x81\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xAD\x90a\x17}V[\x80\x15a\x0C\xF8W\x80`\x1F\x10a\x0C\xCFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xF8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\r(W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PPT\x81V[\x80a\rD\x81a\x02\xBCV[a\raW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rj\x82a\x0E\x9EV[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\r\xCBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\xB7W[PPPPP\x90P\x91\x90PV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x0E\x92W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x0E\x06Wa\x0E\x06a\x16\x89V[\x90P` \x02\x81\x01\x90a\x0E\x18\x91\x90a\x16\x9DV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E4\x91\x90a\x1A\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Es\x91\x90a\x1AlV[a\x0E\x80W_\x91PPa\x0E\x98V[\x80a\x0E\x8A\x81a\x16\xCFV[\x91PPa\r\xDAV[P`\x01\x90P[\x92\x91PPV[_\x80\x80a\x0E\xAB\x84\x80a\x15\xCBV[\x81\x01\x90a\x0E\xB8\x91\x90a\x1A\x87V[_\x83\x81R`\x02` R`@\x81 `\x04\x81\x01T\x94\x97P\x92\x95P\x90\x93P\x90\x91\x90\x03a\x0E\xF4W`@Qc1\xFB\x87\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x07\x01TB\x11\x15a\x0FfW`\x05\x81\x01\x80T`\xFF\x19\x16`\x04\x90\x81\x17\x90\x91U`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x0FE\x91\x90\x86\x90a\x1A\xEFV[`@Q\x80\x91\x03\x90\xA2`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0FrW`\x03a\x0FuV[`\x02[`\x05\x82\x01\x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x0F\x93Wa\x0F\x93a\x14 V[\x02\x17\x90UP`\x06\x81\x01a\x0F\xA6\x83\x82a\x17\xFEV[P\x82\x15a\x0F\xC9W_\x84\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`\x05\x81\x01T`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x10\x04\x91`\xFF\x16\x90\x86\x90a\x1A\xEFV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x81 \x90[\x81T\x81\x10\x15a\x10\xCDW\x82\x82\x82\x81T\x81\x10a\x10HWa\x10Ha\x16\x89V[\x90_R` _ \x01T\x03a\x10\xC5W\x81T\x82\x90a\x10f\x90`\x01\x90a\x19\xC2V[\x81T\x81\x10a\x10vWa\x10va\x16\x89V[\x90_R` _ \x01T\x82\x82\x81T\x81\x10a\x10\x91Wa\x10\x91a\x16\x89V[\x90_R` _ \x01\x81\x90UP\x81\x80T\x80a\x10\xADWa\x10\xADa\x1B\x16V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90Ua\x10\xCDV[`\x01\x01a\x10,V[PPPPV[P\x80Ta\x10\xDF\x90a\x17}V[_\x82U\x80`\x1F\x10a\x10\xEEWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x11\n\x91\x90a\x11\rV[PV[[\x80\x82\x11\x15a\x11!W_\x81U`\x01\x01a\x11\x0EV[P\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\nW__\xFD[_` \x82\x84\x03\x12\x15a\x11JW__\xFD[\x815a\x11U\x81a\x11%V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x11lW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x82W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x11UW__\xFD[__` \x83\x85\x03\x12\x15a\x11\xA4W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xBAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x11\xCAW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE0W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x11\xF4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\nW__\xFD[\x805a\x12#\x81a\x12\x04V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x12VWa\x12Va\x12(V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\x85Wa\x12\x85a\x12(V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x12\x9CW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x12\xC2W__\xFD[a\x11U\x83\x835` \x85\x01a\x12<V[\x805`\x02\x81\x10a\x12#W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\x12\xFAW__\xFD[a\x13\x03\x8Ca\x12\x18V[\x9AP` \x8C\x015\x99P`@\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13%W__\xFD[a\x131\x8E\x82\x8F\x01a\x12\xB3V[\x99PPa\x13@``\x8D\x01a\x12\xD1V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\x13c`\xE0\x8D\x01a\x12\x18V[\x93Pa\x13ra\x01\0\x8D\x01a\x12\x18V[\x92Pa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x8EW__\xFD[a\x13\x9A\x8E\x82\x8F\x01a\x12\xB3V[\x92PPa\x13\xAAa\x01@\x8D\x01a\x12\x18V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\x13\xCCW__\xFD[P5\x91\x90PV[_[\x83\x81\x10\x15a\x13\xEDW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13\xD5V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x14\x0C\x81` \x86\x01` \x86\x01a\x13\xD3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x14DWa\x14Da\x14 V[\x90RV[`\x05\x81\x10a\x14DWa\x14Da\x14 V[`\x01\x80`\xA0\x1B\x03\x8A\x16\x81R\x88` \x82\x01Ra\x01 `@\x82\x01R_a\x14\x80a\x01 \x83\x01\x8Aa\x13\xF5V[a\x14\x8D``\x84\x01\x8Aa\x144V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x84\x01R`\xA0\x83\x01\x87\x90Ra\x14\xB0`\xC0\x84\x01\x87a\x14HV[\x82\x81\x03`\xE0\x84\x01Ra\x14\xC2\x81\x86a\x13\xF5V[\x91PP\x82a\x01\0\x83\x01R\x9A\x99PPPPPPPPPPV[\x80\x15\x15\x81\x14a\x11\nW__\xFD[__`@\x83\x85\x03\x12\x15a\x14\xF8W__\xFD[\x825\x91P` \x83\x015a\x15\n\x81a\x14\xDAV[\x80\x91PP\x92P\x92\x90PV[a\x15\x1F\x81\x85a\x14HV[``` \x82\x01R_a\x154``\x83\x01\x85a\x13\xF5V[\x90P\x82`@\x83\x01R\x94\x93PPPPV[__`@\x83\x85\x03\x12\x15a\x15UW__\xFD[\x825a\x15`\x81a\x12\x04V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x15~W__\xFD[\x815a\x11U\x81a\x12\x04V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x15\xC0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xA2V[P\x90\x95\x94PPPPPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15\xE0W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xFAW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\x0EW__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\x16e`@\x83\x01\x84\x86a\x16$V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x16~W__\xFD[\x81Qa\x11U\x81a\x11%V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x16\xB1W__\xFD[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x16\xEAWa\x16\xEAa\x16\xBBV[`\x01\x01\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x87``\x1B\x16\x81R\x85`\x14\x82\x01R_\x85Qa\x17\"\x81`4\x85\x01` \x8A\x01a\x13\xD3V[\x82\x01`\x02\x86\x10a\x174Wa\x174a\x14 V[`\xF8\x95\x90\x95\x1B`4\x86\x01RPP``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`5\x83\x01R`I\x82\x01R`i\x01\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x98Wa\x0E\x98a\x16\xBBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x17\x91W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17\xAFWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x17\xF9W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x17\xDAWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04oW_\x81U`\x01\x01a\x17\xE6V[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x18Wa\x18\x18a\x12(V[a\x18,\x81a\x18&\x84Ta\x17}V[\x84a\x17\xB5V[` `\x1F\x82\x11`\x01\x81\x14a\x18^W_\x83\x15a\x18GWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x04oV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x18\x8DW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x18mV[P\x84\x82\x10\x15a\x18\xAAW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\xE0\x81R`\x03`\xE0\x82\x01Rbpre`\xE8\x1Ba\x01\0\x82\x01R\x86` \x82\x01R`\x01\x80`\xA0\x1B\x03\x86\x16`@\x82\x01R\x84``\x82\x01Ra\x01 `\x80\x82\x01R_a\x19\x02a\x01 \x83\x01\x86a\x13\xF5V[\x90Pa\x19\x11`\xA0\x83\x01\x85a\x144V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\xC0\x91\x90\x91\x01R\x95\x94PPPPPV[` \x81R_a\x11U` \x83\x01\x84a\x13\xF5V[\x85\x81R`\xA0` \x82\x01R_a\x19W`\xA0\x83\x01\x87a\x13\xF5V[\x90Pa\x19f`@\x83\x01\x86a\x144V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16``\x82\x01R`\x80\x01R\x93\x92PPPV[a\x19\x8C\x81\x83a\x14HV[`@` \x82\x01\x81\x90R`\x16\x90\x82\x01RuValidation in progress`P\x1B``\x82\x01R`\x80\x01\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0E\x98Wa\x0E\x98a\x16\xBBV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\xEAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\tW__\xFD[\x806\x03\x82\x13\x15a\x16\x0EW__\xFD[` \x81R_a\x1A&\x83\x84a\x19\xD5V[`@` \x85\x01Ra\x1A;``\x85\x01\x82\x84a\x16$V[\x91PPa\x1AK` \x85\x01\x85a\x19\xD5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1Ab\x83\x82\x84a\x16$V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1A|W__\xFD[\x81Qa\x11U\x81a\x14\xDAV[___``\x84\x86\x03\x12\x15a\x1A\x99W__\xFD[\x835\x92P` \x84\x015a\x1A\xAB\x81a\x14\xDAV[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xC6W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A\xD6W__\xFD[a\x1A\xE5\x86\x825` \x84\x01a\x12<V[\x91PP\x92P\x92P\x92V[a\x1A\xF9\x81\x84a\x14HV[`@` \x82\x01R_a\x1B\x0E`@\x83\x01\x84a\x13\xF5V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xEDs$v\x0F\x0F\x7F\x90\x8A\xEC\x1D\xF3\xD7\x93\x8A\xEF\x17v\xED\xB5\n<\xCD\xF8\xBB\xC6\x07\xF9\x0C\xB2\xC5\xD4dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100f0575f3560e01c806393271368116100935780639aa9fda5116100635780639aa9fda51461026e5780639e83e30614610281578063a71f8da0146102a1578063c56fb0fd146102b4575f5ffd5b806393271368146101f6578063944074651461020957806397f5365a1461022b57806399d7cf4b1461024c575f5ffd5b806363710c05116100ce57806363710c051461017f57806375f0bb5214610194578063761de19f146101a75780637b4f3373146101ce575f5ffd5b806301ffc9a7146100f4578063186f03541461012d578063216a3e9a1461016c575b5f5ffd5b61011861010236600461113a565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b6101547f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610124565b61011861017a36600461115c565b6102bc565b61019261018d366004611193565b6103f2565b005b6101926101a23660046112df565b610476565b6101547f000000000000000000000000000000000000000000000000000000000000000081565b6101e16101dc3660046113bc565b610863565b60405161012499989796959493929190611458565b6101926102043660046114e7565b6109d6565b61021c6102173660046113bc565b610b6e565b60405161012493929190611515565b61023e610239366004611544565b610d0f565b604051908152602001610124565b61011861025a3660046113bc565b60016020525f908152604090205460ff1681565b61019261027c36600461115c565b610d3a565b61029461028f36600461156e565b610d6e565b6040516101249190611589565b6101186102af366004611193565b610dd7565b61023e5f5481565b5f806102c883806115cb565b6040516102d6929190611615565b604051809103902090505f610337826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e8361037e60208901896115cb565b6040518463ffffffff1660e01b815260040161039c9392919061164c565b602060405180830381865afa1580156103b7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103db919061166e565b6001600160e01b0319918216911614949350505050565b81816103fe8282610dd7565b61041b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff811684111561046f5761045d85858363ffffffff1681811061044657610446611689565b9050602002810190610458919061169d565b610e9e565b80610467816116cf565b91505061041d565b5050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104e25760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064015b60405180910390fd5b5f8b8b8b8b85426040516020016104fe969594939291906116f3565b60408051601f1981840301815291815281516020928301205f818152600190935291205490915060ff16156105335750610856565b5f818152600260205260409020600401541580159061056157505f8181526002602052604090206007015442115b1561057f576040516338e5e54b60e21b815260040160405180910390fd5b6040518061012001604052808d6001600160a01b031681526020018c81526020018b81526020018a60018111156105b8576105b8611420565b81526001600160a01b0384166020820152426040820152606001600181526020016040518060400160405280601681526020017556616c69646174696f6e20696e2070726f677265737360501b81525081526020015f544261061a919061176a565b90525f82815260026020818152604092839020845181546001600160a01b0319166001600160a01b03909116178155908401516001820155918301519082019061066490826117fe565b50606082015160038201805460ff19166001838181111561068757610687611420565b021790555060808201518160030160016101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a0820151816004015560c0820151816005015f6101000a81548160ff021916908360048111156106ec576106ec611420565b021790555060e0820151600682019061070590826117fe565b50610100820151816007015590505060035f836001600160a01b03166001600160a01b031681526020019081526020015f2081908060018154018082558091505060019003905f5260205f20015f90919091909150557f37b1671f777b1ea11710dc816b92da0f8b5ea94730552bb5637b0ec536811908818d8d8d8d87604051602001610797969594939291906118b9565b60408051601f19818403018152908290526107b19161192d565b60405180910390a18b6001600160a01b0316817f72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb8d8d8d875f546040516107fc95949392919061193f565b60405180910390a3807f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd960016040516108359190611982565b60405180910390a26040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b600260208190525f91825260409091208054600182015492820180546001600160a01b039092169392916108969061177d565b80601f01602080910402602001604051908101604052809291908181526020018280546108c29061177d565b801561090d5780601f106108e45761010080835404028352916020019161090d565b820191905f5260205f20905b8154815290600101906020018083116108f057829003601f168201915b50505050600383015460048401546005850154600686018054959660ff808616976101009096046001600160a01b031696509394939092169261094f9061177d565b80601f016020809104026020016040519081016040528092919081815260200182805461097b9061177d565b80156109c65780601f1061099d576101008083540402835291602001916109c6565b820191905f5260205f20905b8154815290600101906020018083116109a957829003601f168201915b5050505050908060070154905089565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a3d5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016104d9565b5f8281526002602052604090206003810154610a679061010090046001600160a01b031684611013565b7f37b1671f777b1ea11710dc816b92da0f8b5ea94730552bb5637b0ec5368119088383604051602001610ac39291906060808252600490820152631c1bdcdd60e21b608082015260208101929092521515604082015260a00190565b60408051601f1981840301815290829052610add9161192d565b60405180910390a15f838152600260208190526040822080546001600160a01b0319168155600181018390559190610b17908301826110d3565b6003820180546001600160a81b03191690555f6004830181905560058301805460ff19169055610b4b9060068401906110d3565b505f6007919091018190559283525050600160205260409020805460ff19169055565b5f818152600260205260408120600481015460609183918203610ba657505060408051602081019091525f8082529250905081610d08565b8060070154421115610c4f576004816006015f818054610bc59061177d565b80601f0160208091040260200160405190810160405280929190818152602001828054610bf19061177d565b8015610c3c5780601f10610c1357610100808354040283529160200191610c3c565b820191905f5260205f20905b815481529060010190602001808311610c1f57829003601f168201915b5050505050915093509350935050610d08565b5f428260070154610c6091906119c2565b600583015460068401805492935060ff9091169183908290610c819061177d565b80601f0160208091040260200160405190810160405280929190818152602001828054610cad9061177d565b8015610cf85780601f10610ccf57610100808354040283529160200191610cf8565b820191905f5260205f20905b815481529060010190602001808311610cdb57829003601f168201915b5050505050915094509450945050505b9193909250565b6003602052815f5260405f208181548110610d28575f80fd5b905f5260205f20015f91509150505481565b80610d44816102bc565b610d6157604051638baa579f60e01b815260040160405180910390fd5b610d6a82610e9e565b5050565b6001600160a01b0381165f90815260036020908152604091829020805483518184028101840190945280845260609392830182828015610dcb57602002820191905f5260205f20905b815481526020019060010190808311610db7575b50505050509050919050565b5f805b63ffffffff8116831115610e92573063216a3e9a858563ffffffff8516818110610e0657610e06611689565b9050602002810190610e18919061169d565b6040518263ffffffff1660e01b8152600401610e349190611a17565b602060405180830381865afa158015610e4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e739190611a6c565b610e80575f915050610e98565b80610e8a816116cf565b915050610dda565b50600190505b92915050565b5f8080610eab84806115cb565b810190610eb89190611a87565b5f838152600260205260408120600481015494975092955090935090919003610ef4576040516331fb878f60e01b815260040160405180910390fd5b8060070154421115610f665760058101805460ff1916600490811790915560405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd991610f4591908690611aef565b60405180910390a26040516338e5e54b60e21b815260040160405180910390fd5b82610f72576003610f75565b60025b60058201805460ff19166001836004811115610f9357610f93611420565b021790555060068101610fa683826117fe565b508215610fc9575f848152600160208190526040909120805460ff191690911790555b600581015460405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd9916110049160ff16908690611aef565b60405180910390a25050505050565b6001600160a01b0382165f908152600360205260408120905b81548110156110cd578282828154811061104857611048611689565b905f5260205f200154036110c55781548290611066906001906119c2565b8154811061107657611076611689565b905f5260205f20015482828154811061109157611091611689565b905f5260205f200181905550818054806110ad576110ad611b16565b600190038181905f5260205f20015f905590556110cd565b60010161102c565b50505050565b5080546110df9061177d565b5f825580601f106110ee575050565b601f0160209004905f5260205f209081019061110a919061110d565b50565b5b80821115611121575f815560010161110e565b5090565b6001600160e01b03198116811461110a575f5ffd5b5f6020828403121561114a575f5ffd5b813561115581611125565b9392505050565b5f6020828403121561116c575f5ffd5b813567ffffffffffffffff811115611182575f5ffd5b820160408185031215611155575f5ffd5b5f5f602083850312156111a4575f5ffd5b823567ffffffffffffffff8111156111ba575f5ffd5b8301601f810185136111ca575f5ffd5b803567ffffffffffffffff8111156111e0575f5ffd5b8560208260051b84010111156111f4575f5ffd5b6020919091019590945092505050565b6001600160a01b038116811461110a575f5ffd5b803561122381611204565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f67ffffffffffffffff84111561125657611256611228565b50604051601f19601f85018116603f0116810181811067ffffffffffffffff8211171561128557611285611228565b60405283815290508082840185101561129c575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f8301126112c2575f5ffd5b6111558383356020850161123c565b803560028110611223575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e0312156112fa575f5ffd5b6113038c611218565b9a5060208c0135995060408c013567ffffffffffffffff811115611325575f5ffd5b6113318e828f016112b3565b99505061134060608d016112d1565b975060808c0135965060a08c0135955060c08c0135945061136360e08d01611218565b93506113726101008d01611218565b92506101208c013567ffffffffffffffff81111561138e575f5ffd5b61139a8e828f016112b3565b9250506113aa6101408d01611218565b90509295989b509295989b9093969950565b5f602082840312156113cc575f5ffd5b5035919050565b5f5b838110156113ed5781810151838201526020016113d5565b50505f910152565b5f815180845261140c8160208601602086016113d3565b601f01601f19169290920160200192915050565b634e487b7160e01b5f52602160045260245ffd5b6002811061144457611444611420565b9052565b6005811061144457611444611420565b60018060a01b038a16815288602082015261012060408201525f61148061012083018a6113f5565b61148d606084018a611434565b6001600160a01b038816608084015260a083018790526114b060c0840187611448565b82810360e08401526114c281866113f5565b915050826101008301529a9950505050505050505050565b801515811461110a575f5ffd5b5f5f604083850312156114f8575f5ffd5b82359150602083013561150a816114da565b809150509250929050565b61151f8185611448565b606060208201525f61153460608301856113f5565b9050826040830152949350505050565b5f5f60408385031215611555575f5ffd5b823561156081611204565b946020939093013593505050565b5f6020828403121561157e575f5ffd5b813561115581611204565b602080825282518282018190525f918401906040840190835b818110156115c05783518352602093840193909201916001016115a2565b509095945050505050565b5f5f8335601e198436030181126115e0575f5ffd5b83018035915067ffffffffffffffff8211156115fa575f5ffd5b60200191503681900382131561160e575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f611665604083018486611624565b95945050505050565b5f6020828403121561167e575f5ffd5b815161115581611125565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126116b1575f5ffd5b9190910192915050565b634e487b7160e01b5f52601160045260245ffd5b5f63ffffffff821663ffffffff81036116ea576116ea6116bb565b60010192915050565b6bffffffffffffffffffffffff198760601b1681528560148201525f8551611722816034850160208a016113d3565b82016002861061173457611734611420565b60f89590951b6034860152505060609190911b6bffffffffffffffffffffffff1916603583015260498201526069019392505050565b80820180821115610e9857610e986116bb565b600181811c9082168061179157607f821691505b6020821081036117af57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156117f957805f5260205f20601f840160051c810160208510156117da5750805b601f840160051c820191505b8181101561046f575f81556001016117e6565b505050565b815167ffffffffffffffff81111561181857611818611228565b61182c81611826845461177d565b846117b5565b6020601f82116001811461185e575f83156118475750848201515b5f19600385901b1c1916600184901b17845561046f565b5f84815260208120601f198516915b8281101561188d578785015182556020948501946001909201910161186d565b50848210156118aa57868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b60e08152600360e08201526270726560e81b61010082015286602082015260018060a01b038616604082015284606082015261012060808201525f6119026101208301866113f5565b905061191160a0830185611434565b6001600160a01b039290921660c0919091015295945050505050565b602081525f61115560208301846113f5565b85815260a060208201525f61195760a08301876113f5565b90506119666040830186611434565b6001600160a01b03939093166060820152608001529392505050565b61198c8183611448565b6040602082018190526016908201527556616c69646174696f6e20696e2070726f677265737360501b6060820152608001919050565b81810381811115610e9857610e986116bb565b5f5f8335601e198436030181126119ea575f5ffd5b830160208101925035905067ffffffffffffffff811115611a09575f5ffd5b80360382131561160e575f5ffd5b602081525f611a2683846119d5565b60406020850152611a3b606085018284611624565b915050611a4b60208501856119d5565b848303601f19016040860152611a62838284611624565b9695505050505050565b5f60208284031215611a7c575f5ffd5b8151611155816114da565b5f5f5f60608486031215611a99575f5ffd5b833592506020840135611aab816114da565b9150604084013567ffffffffffffffff811115611ac6575f5ffd5b8401601f81018613611ad6575f5ffd5b611ae58682356020840161123c565b9150509250925092565b611af98184611448565b604060208201525f611b0e60408301846113f5565b949350505050565b634e487b7160e01b5f52603160045260245ffdfea2646970667358221220ed7324760f0f7f908aec1df3d7938aef1776edb50a3ccdf8bbc607f90cb2c5d464736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xF0W_5`\xE0\x1C\x80c\x93'\x13h\x11a\0\x93W\x80c\x9A\xA9\xFD\xA5\x11a\0cW\x80c\x9A\xA9\xFD\xA5\x14a\x02nW\x80c\x9E\x83\xE3\x06\x14a\x02\x81W\x80c\xA7\x1F\x8D\xA0\x14a\x02\xA1W\x80c\xC5o\xB0\xFD\x14a\x02\xB4W__\xFD[\x80c\x93'\x13h\x14a\x01\xF6W\x80c\x94@te\x14a\x02\tW\x80c\x97\xF56Z\x14a\x02+W\x80c\x99\xD7\xCFK\x14a\x02LW__\xFD[\x80ccq\x0C\x05\x11a\0\xCEW\x80ccq\x0C\x05\x14a\x01\x7FW\x80cu\xF0\xBBR\x14a\x01\x94W\x80cv\x1D\xE1\x9F\x14a\x01\xA7W\x80c{O3s\x14a\x01\xCEW__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\0\xF4W\x80c\x18o\x03T\x14a\x01-W\x80c!j>\x9A\x14a\x01lW[__\xFD[a\x01\x18a\x01\x026`\x04a\x11:V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01$V[a\x01\x18a\x01z6`\x04a\x11\\V[a\x02\xBCV[a\x01\x92a\x01\x8D6`\x04a\x11\x93V[a\x03\xF2V[\0[a\x01\x92a\x01\xA26`\x04a\x12\xDFV[a\x04vV[a\x01T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xE1a\x01\xDC6`\x04a\x13\xBCV[a\x08cV[`@Qa\x01$\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x14XV[a\x01\x92a\x02\x046`\x04a\x14\xE7V[a\t\xD6V[a\x02\x1Ca\x02\x176`\x04a\x13\xBCV[a\x0BnV[`@Qa\x01$\x93\x92\x91\x90a\x15\x15V[a\x02>a\x0296`\x04a\x15DV[a\r\x0FV[`@Q\x90\x81R` \x01a\x01$V[a\x01\x18a\x02Z6`\x04a\x13\xBCV[`\x01` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x01\x92a\x02|6`\x04a\x11\\V[a\r:V[a\x02\x94a\x02\x8F6`\x04a\x15nV[a\rnV[`@Qa\x01$\x91\x90a\x15\x89V[a\x01\x18a\x02\xAF6`\x04a\x11\x93V[a\r\xD7V[a\x02>_T\x81V[_\x80a\x02\xC8\x83\x80a\x15\xCBV[`@Qa\x02\xD6\x92\x91\x90a\x16\x15V[`@Q\x80\x91\x03\x90 \x90P_a\x037\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x03~` \x89\x01\x89a\x15\xCBV[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x9C\x93\x92\x91\x90a\x16LV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xDB\x91\x90a\x16nV[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[\x81\x81a\x03\xFE\x82\x82a\r\xD7V[a\x04\x1BW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04oWa\x04]\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04FWa\x04Fa\x16\x89V[\x90P` \x02\x81\x01\x90a\x04X\x91\x90a\x16\x9DV[a\x0E\x9EV[\x80a\x04g\x81a\x16\xCFV[\x91PPa\x04\x1DV[PPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x8B\x8B\x8B\x8B\x85B`@Q` \x01a\x04\xFE\x96\x95\x94\x93\x92\x91\x90a\x16\xF3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x01\x90\x93R\x91 T\x90\x91P`\xFF\x16\x15a\x053WPa\x08VV[_\x81\x81R`\x02` R`@\x90 `\x04\x01T\x15\x80\x15\x90a\x05aWP_\x81\x81R`\x02` R`@\x90 `\x07\x01TB\x11[\x15a\x05\x7FW`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01 \x01`@R\x80\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A`\x01\x81\x11\x15a\x05\xB8Wa\x05\xB8a\x14 V[\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01RB`@\x82\x01R``\x01`\x01\x81R` \x01`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uValidation in progress`P\x1B\x81RP\x81R` \x01_TBa\x06\x1A\x91\x90a\x17jV[\x90R_\x82\x81R`\x02` \x81\x81R`@\x92\x83\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q`\x01\x82\x01U\x91\x83\x01Q\x90\x82\x01\x90a\x06d\x90\x82a\x17\xFEV[P``\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x06\x87Wa\x06\x87a\x14 V[\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a\x06\xECWa\x06\xECa\x14 V[\x02\x17\x90UP`\xE0\x82\x01Q`\x06\x82\x01\x90a\x07\x05\x90\x82a\x17\xFEV[Pa\x01\0\x82\x01Q\x81`\x07\x01U\x90PP`\x03_\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU\x7F7\xB1g\x1Fw{\x1E\xA1\x17\x10\xDC\x81k\x92\xDA\x0F\x8B^\xA9G0U+\xB5c{\x0E\xC56\x81\x19\x08\x81\x8D\x8D\x8D\x8D\x87`@Q` \x01a\x07\x97\x96\x95\x94\x93\x92\x91\x90a\x18\xB9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xB1\x91a\x19-V[`@Q\x80\x91\x03\x90\xA1\x8B`\x01`\x01`\xA0\x1B\x03\x16\x81\x7Fr\xB8\xBE\xAA+\x16\xEF\xC2\x0F\xF7\xAE\xA9B\xA1\"\xF7\xB7\x81\x19rO\xAB\xBD\x80j\xCDd\xD7\x97\x89T\xCB\x8D\x8D\x8D\x87_T`@Qa\x07\xFC\x95\x94\x93\x92\x91\x90a\x19?V[`@Q\x80\x91\x03\x90\xA3\x80\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9`\x01`@Qa\x085\x91\x90a\x19\x82V[`@Q\x80\x91\x03\x90\xA2`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\x02` \x81\x90R_\x91\x82R`@\x90\x91 \x80T`\x01\x82\x01T\x92\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93\x92\x91a\x08\x96\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xC2\x90a\x17}V[\x80\x15a\t\rW\x80`\x1F\x10a\x08\xE4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\rV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xF0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01\x80T\x95\x96`\xFF\x80\x86\x16\x97a\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P\x93\x94\x93\x90\x92\x16\x92a\tO\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t{\x90a\x17}V[\x80\x15a\t\xC6W\x80`\x1F\x10a\t\x9DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\xC6V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\xA9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x07\x01T\x90P\x89V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x04\xD9V[_\x82\x81R`\x02` R`@\x90 `\x03\x81\x01Ta\ng\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a\x10\x13V[\x7F7\xB1g\x1Fw{\x1E\xA1\x17\x10\xDC\x81k\x92\xDA\x0F\x8B^\xA9G0U+\xB5c{\x0E\xC56\x81\x19\x08\x83\x83`@Q` \x01a\n\xC3\x92\x91\x90``\x80\x82R`\x04\x90\x82\x01Rc\x1C\x1B\xDC\xDD`\xE2\x1B`\x80\x82\x01R` \x81\x01\x92\x90\x92R\x15\x15`@\x82\x01R`\xA0\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\n\xDD\x91a\x19-V[`@Q\x80\x91\x03\x90\xA1_\x83\x81R`\x02` \x81\x90R`@\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x83\x90U\x91\x90a\x0B\x17\x90\x83\x01\x82a\x10\xD3V[`\x03\x82\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U_`\x04\x83\x01\x81\x90U`\x05\x83\x01\x80T`\xFF\x19\x16\x90Ua\x0BK\x90`\x06\x84\x01\x90a\x10\xD3V[P_`\x07\x91\x90\x91\x01\x81\x90U\x92\x83RPP`\x01` R`@\x90 \x80T`\xFF\x19\x16\x90UV[_\x81\x81R`\x02` R`@\x81 `\x04\x81\x01T``\x91\x83\x91\x82\x03a\x0B\xA6WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x92P\x90P\x81a\r\x08V[\x80`\x07\x01TB\x11\x15a\x0COW`\x04\x81`\x06\x01_\x81\x80Ta\x0B\xC5\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xF1\x90a\x17}V[\x80\x15a\x0C<W\x80`\x1F\x10a\x0C\x13Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C<V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\x1FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x93P\x93P\x93PPa\r\x08V[_B\x82`\x07\x01Ta\x0C`\x91\x90a\x19\xC2V[`\x05\x83\x01T`\x06\x84\x01\x80T\x92\x93P`\xFF\x90\x91\x16\x91\x83\x90\x82\x90a\x0C\x81\x90a\x17}V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0C\xAD\x90a\x17}V[\x80\x15a\x0C\xF8W\x80`\x1F\x10a\x0C\xCFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xF8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\r(W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PPT\x81V[\x80a\rD\x81a\x02\xBCV[a\raW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\rj\x82a\x0E\x9EV[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\r\xCBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r\xB7W[PPPPP\x90P\x91\x90PV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x0E\x92W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x0E\x06Wa\x0E\x06a\x16\x89V[\x90P` \x02\x81\x01\x90a\x0E\x18\x91\x90a\x16\x9DV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E4\x91\x90a\x1A\x17V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Es\x91\x90a\x1AlV[a\x0E\x80W_\x91PPa\x0E\x98V[\x80a\x0E\x8A\x81a\x16\xCFV[\x91PPa\r\xDAV[P`\x01\x90P[\x92\x91PPV[_\x80\x80a\x0E\xAB\x84\x80a\x15\xCBV[\x81\x01\x90a\x0E\xB8\x91\x90a\x1A\x87V[_\x83\x81R`\x02` R`@\x81 `\x04\x81\x01T\x94\x97P\x92\x95P\x90\x93P\x90\x91\x90\x03a\x0E\xF4W`@Qc1\xFB\x87\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x07\x01TB\x11\x15a\x0FfW`\x05\x81\x01\x80T`\xFF\x19\x16`\x04\x90\x81\x17\x90\x91U`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x0FE\x91\x90\x86\x90a\x1A\xEFV[`@Q\x80\x91\x03\x90\xA2`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0FrW`\x03a\x0FuV[`\x02[`\x05\x82\x01\x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x0F\x93Wa\x0F\x93a\x14 V[\x02\x17\x90UP`\x06\x81\x01a\x0F\xA6\x83\x82a\x17\xFEV[P\x82\x15a\x0F\xC9W_\x84\x81R`\x01` \x81\x90R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`\x05\x81\x01T`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x10\x04\x91`\xFF\x16\x90\x86\x90a\x1A\xEFV[`@Q\x80\x91\x03\x90\xA2PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x03` R`@\x81 \x90[\x81T\x81\x10\x15a\x10\xCDW\x82\x82\x82\x81T\x81\x10a\x10HWa\x10Ha\x16\x89V[\x90_R` _ \x01T\x03a\x10\xC5W\x81T\x82\x90a\x10f\x90`\x01\x90a\x19\xC2V[\x81T\x81\x10a\x10vWa\x10va\x16\x89V[\x90_R` _ \x01T\x82\x82\x81T\x81\x10a\x10\x91Wa\x10\x91a\x16\x89V[\x90_R` _ \x01\x81\x90UP\x81\x80T\x80a\x10\xADWa\x10\xADa\x1B\x16V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90Ua\x10\xCDV[`\x01\x01a\x10,V[PPPPV[P\x80Ta\x10\xDF\x90a\x17}V[_\x82U\x80`\x1F\x10a\x10\xEEWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x11\n\x91\x90a\x11\rV[PV[[\x80\x82\x11\x15a\x11!W_\x81U`\x01\x01a\x11\x0EV[P\x90V[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x11\nW__\xFD[_` \x82\x84\x03\x12\x15a\x11JW__\xFD[\x815a\x11U\x81a\x11%V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x11lW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\x82W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x11UW__\xFD[__` \x83\x85\x03\x12\x15a\x11\xA4W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xBAW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x11\xCAW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE0W__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x11\xF4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x11\nW__\xFD[\x805a\x12#\x81a\x12\x04V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15a\x12VWa\x12Va\x12(V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x12\x85Wa\x12\x85a\x12(V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x12\x9CW__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x12\xC2W__\xFD[a\x11U\x83\x835` \x85\x01a\x12<V[\x805`\x02\x81\x10a\x12#W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\x12\xFAW__\xFD[a\x13\x03\x8Ca\x12\x18V[\x9AP` \x8C\x015\x99P`@\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13%W__\xFD[a\x131\x8E\x82\x8F\x01a\x12\xB3V[\x99PPa\x13@``\x8D\x01a\x12\xD1V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\x13c`\xE0\x8D\x01a\x12\x18V[\x93Pa\x13ra\x01\0\x8D\x01a\x12\x18V[\x92Pa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x13\x8EW__\xFD[a\x13\x9A\x8E\x82\x8F\x01a\x12\xB3V[\x92PPa\x13\xAAa\x01@\x8D\x01a\x12\x18V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\x13\xCCW__\xFD[P5\x91\x90PV[_[\x83\x81\x10\x15a\x13\xEDW\x81\x81\x01Q\x83\x82\x01R` \x01a\x13\xD5V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x14\x0C\x81` \x86\x01` \x86\x01a\x13\xD3V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x14DWa\x14Da\x14 V[\x90RV[`\x05\x81\x10a\x14DWa\x14Da\x14 V[`\x01\x80`\xA0\x1B\x03\x8A\x16\x81R\x88` \x82\x01Ra\x01 `@\x82\x01R_a\x14\x80a\x01 \x83\x01\x8Aa\x13\xF5V[a\x14\x8D``\x84\x01\x8Aa\x144V[`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x84\x01R`\xA0\x83\x01\x87\x90Ra\x14\xB0`\xC0\x84\x01\x87a\x14HV[\x82\x81\x03`\xE0\x84\x01Ra\x14\xC2\x81\x86a\x13\xF5V[\x91PP\x82a\x01\0\x83\x01R\x9A\x99PPPPPPPPPPV[\x80\x15\x15\x81\x14a\x11\nW__\xFD[__`@\x83\x85\x03\x12\x15a\x14\xF8W__\xFD[\x825\x91P` \x83\x015a\x15\n\x81a\x14\xDAV[\x80\x91PP\x92P\x92\x90PV[a\x15\x1F\x81\x85a\x14HV[``` \x82\x01R_a\x154``\x83\x01\x85a\x13\xF5V[\x90P\x82`@\x83\x01R\x94\x93PPPPV[__`@\x83\x85\x03\x12\x15a\x15UW__\xFD[\x825a\x15`\x81a\x12\x04V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a\x15~W__\xFD[\x815a\x11U\x81a\x12\x04V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x15\xC0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xA2V[P\x90\x95\x94PPPPPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x15\xE0W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x15\xFAW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x16\x0EW__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\x16e`@\x83\x01\x84\x86a\x16$V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x16~W__\xFD[\x81Qa\x11U\x81a\x11%V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x16\xB1W__\xFD[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x16\xEAWa\x16\xEAa\x16\xBBV[`\x01\x01\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x87``\x1B\x16\x81R\x85`\x14\x82\x01R_\x85Qa\x17\"\x81`4\x85\x01` \x8A\x01a\x13\xD3V[\x82\x01`\x02\x86\x10a\x174Wa\x174a\x14 V[`\xF8\x95\x90\x95\x1B`4\x86\x01RPP``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`5\x83\x01R`I\x82\x01R`i\x01\x93\x92PPPV[\x80\x82\x01\x80\x82\x11\x15a\x0E\x98Wa\x0E\x98a\x16\xBBV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x17\x91W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x17\xAFWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x17\xF9W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x17\xDAWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x04oW_\x81U`\x01\x01a\x17\xE6V[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x18Wa\x18\x18a\x12(V[a\x18,\x81a\x18&\x84Ta\x17}V[\x84a\x17\xB5V[` `\x1F\x82\x11`\x01\x81\x14a\x18^W_\x83\x15a\x18GWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x04oV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x18\x8DW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x18mV[P\x84\x82\x10\x15a\x18\xAAW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\xE0\x81R`\x03`\xE0\x82\x01Rbpre`\xE8\x1Ba\x01\0\x82\x01R\x86` \x82\x01R`\x01\x80`\xA0\x1B\x03\x86\x16`@\x82\x01R\x84``\x82\x01Ra\x01 `\x80\x82\x01R_a\x19\x02a\x01 \x83\x01\x86a\x13\xF5V[\x90Pa\x19\x11`\xA0\x83\x01\x85a\x144V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\xC0\x91\x90\x91\x01R\x95\x94PPPPPV[` \x81R_a\x11U` \x83\x01\x84a\x13\xF5V[\x85\x81R`\xA0` \x82\x01R_a\x19W`\xA0\x83\x01\x87a\x13\xF5V[\x90Pa\x19f`@\x83\x01\x86a\x144V[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16``\x82\x01R`\x80\x01R\x93\x92PPPV[a\x19\x8C\x81\x83a\x14HV[`@` \x82\x01\x81\x90R`\x16\x90\x82\x01RuValidation in progress`P\x1B``\x82\x01R`\x80\x01\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0E\x98Wa\x0E\x98a\x16\xBBV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x19\xEAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\tW__\xFD[\x806\x03\x82\x13\x15a\x16\x0EW__\xFD[` \x81R_a\x1A&\x83\x84a\x19\xD5V[`@` \x85\x01Ra\x1A;``\x85\x01\x82\x84a\x16$V[\x91PPa\x1AK` \x85\x01\x85a\x19\xD5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1Ab\x83\x82\x84a\x16$V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1A|W__\xFD[\x81Qa\x11U\x81a\x14\xDAV[___``\x84\x86\x03\x12\x15a\x1A\x99W__\xFD[\x835\x92P` \x84\x015a\x1A\xAB\x81a\x14\xDAV[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xC6W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A\xD6W__\xFD[a\x1A\xE5\x86\x825` \x84\x01a\x12<V[\x91PP\x92P\x92P\x92V[a\x1A\xF9\x81\x84a\x14HV[`@` \x82\x01R_a\x1B\x0E`@\x83\x01\x84a\x13\xF5V[\x94\x93PPPPV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xEDs$v\x0F\x0F\x7F\x90\x8A\xEC\x1D\xF3\xD7\x93\x8A\xEF\x17v\xED\xB5\n<\xCD\xF8\xBB\xC6\x07\xF9\x0C\xB2\xC5\xD4dsolcC\0\x08\x1C\x003",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidationStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ValidationStatus> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl ValidationStatus {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
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
        impl alloy_sol_types::SolType for ValidationStatus {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidationStatus {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**Custom error with signature `AsyncValidationRequired()` and selector `0x36fc5713`.
```solidity
error AsyncValidationRequired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AsyncValidationRequired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AsyncValidationRequired> for UnderlyingRustTuple<'_> {
            fn from(value: AsyncValidationRequired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AsyncValidationRequired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AsyncValidationRequired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AsyncValidationRequired()";
            const SELECTOR: [u8; 4] = [54u8, 252u8, 87u8, 19u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `TransactionExpired()` and selector `0xe397952c`.
```solidity
error TransactionExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionExpired {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TransactionExpired> for UnderlyingRustTuple<'_> {
            fn from(value: TransactionExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransactionExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransactionExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransactionExpired()";
            const SELECTOR: [u8; 4] = [227u8, 151u8, 149u8, 44u8];
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
    /**Custom error with signature `TransactionNotFound()` and selector `0x31fb878f`.
```solidity
error TransactionNotFound();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionNotFound {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TransactionNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: TransactionNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransactionNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransactionNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransactionNotFound()";
            const SELECTOR: [u8; 4] = [49u8, 251u8, 135u8, 143u8];
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
    /**Event with signature `ValidationRequired(bytes32,address,uint256,bytes,uint8,address,uint256)` and selector `0x72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb`.
```solidity
event ValidationRequired(bytes32 indexed txHash, address indexed to, uint256 value, bytes data, Enum.Operation operation, address initiator, uint256 estimatedProcessingTime);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidationRequired {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub data: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub initiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub estimatedProcessingTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidationRequired {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ValidationRequired(bytes32,address,uint256,bytes,uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                114u8,
                184u8,
                190u8,
                170u8,
                43u8,
                22u8,
                239u8,
                194u8,
                15u8,
                247u8,
                174u8,
                169u8,
                66u8,
                161u8,
                34u8,
                247u8,
                183u8,
                129u8,
                25u8,
                114u8,
                79u8,
                171u8,
                189u8,
                128u8,
                106u8,
                205u8,
                100u8,
                215u8,
                151u8,
                137u8,
                84u8,
                203u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    txHash: topics.1,
                    to: topics.2,
                    value: data.0,
                    data: data.1,
                    operation: data.2,
                    initiator: data.3,
                    estimatedProcessingTime: data.4,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initiator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.estimatedProcessingTime,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.txHash.clone(), self.to.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.txHash);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidationRequired {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidationRequired> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidationRequired) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ValidationStatusUpdated(bytes32,uint8,string)` and selector `0x5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd9`.
```solidity
event ValidationStatusUpdated(bytes32 indexed txHash, ValidationStatus status, string message);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidationStatusUpdated {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidationStatusUpdated {
            type DataTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::String);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ValidationStatusUpdated(bytes32,uint8,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                92u8,
                82u8,
                185u8,
                32u8,
                252u8,
                93u8,
                10u8,
                196u8,
                88u8,
                56u8,
                194u8,
                5u8,
                173u8,
                146u8,
                101u8,
                6u8,
                18u8,
                197u8,
                206u8,
                91u8,
                248u8,
                19u8,
                106u8,
                240u8,
                47u8,
                166u8,
                148u8,
                102u8,
                204u8,
                58u8,
                31u8,
                217u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    txHash: topics.1,
                    status: data.0,
                    message: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <ValidationStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.message,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.txHash.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.txHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidationStatusUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidationStatusUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ValidationStatusUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WavsTriggerEvent(bytes)` and selector `0x37b1671f777b1ea11710dc816b92da0f8b5ea94730552bb5637b0ec536811908`.
```solidity
event WavsTriggerEvent(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WavsTriggerEvent {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WavsTriggerEvent {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WavsTriggerEvent(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                55u8,
                177u8,
                103u8,
                31u8,
                119u8,
                123u8,
                30u8,
                161u8,
                23u8,
                16u8,
                220u8,
                129u8,
                107u8,
                146u8,
                218u8,
                15u8,
                139u8,
                94u8,
                169u8,
                71u8,
                48u8,
                85u8,
                43u8,
                181u8,
                99u8,
                123u8,
                14u8,
                197u8,
                54u8,
                129u8,
                25u8,
                8u8,
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
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WavsTriggerEvent {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WavsTriggerEvent> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WavsTriggerEvent) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _safe, address _stakeRegistry);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _safe: alloy::sol_types::private::Address,
        pub _stakeRegistry: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
                    (value._safe, value._stakeRegistry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _safe: tuple.0,
                        _stakeRegistry: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self._safe,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                )
            }
        }
    };
    /**Function with signature `STAKE_REGISTRY()` and selector `0x761de19f`.
```solidity
function STAKE_REGISTRY() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKE_REGISTRYCall {}
    ///Container type for the return parameters of the [`STAKE_REGISTRY()`](STAKE_REGISTRYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct STAKE_REGISTRYReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKE_REGISTRYCall> for UnderlyingRustTuple<'_> {
                fn from(value: STAKE_REGISTRYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for STAKE_REGISTRYCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<STAKE_REGISTRYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: STAKE_REGISTRYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for STAKE_REGISTRYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for STAKE_REGISTRYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = STAKE_REGISTRYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "STAKE_REGISTRY()";
            const SELECTOR: [u8; 4] = [118u8, 29u8, 225u8, 159u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addPayload((bytes,bytes))` and selector `0x9aa9fda5`.
```solidity
function addPayload(IWavsSDK.SignedPayload memory signedPayload) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPayloadCall {
        pub signedPayload: <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addPayload((bytes,bytes))`](addPayloadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPayloadReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IWavsSDK::SignedPayload,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addPayloadCall> for UnderlyingRustTuple<'_> {
                fn from(value: addPayloadCall) -> Self {
                    (value.signedPayload,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPayloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayload: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addPayloadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addPayloadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPayloadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPayloadCall {
            type Parameters<'a> = (IWavsSDK::SignedPayload,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPayloadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPayload((bytes,bytes))";
            const SELECTOR: [u8; 4] = [154u8, 169u8, 253u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IWavsSDK::SignedPayload as alloy_sol_types::SolType>::tokenize(
                        &self.signedPayload,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addPayloadMulti((bytes,bytes)[])` and selector `0x63710c05`.
```solidity
function addPayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPayloadMultiCall {
        pub signedPayloads: alloy::sol_types::private::Vec<
            <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`addPayloadMulti((bytes,bytes)[])`](addPayloadMultiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPayloadMultiReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IWavsSDK::SignedPayload>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addPayloadMultiCall> for UnderlyingRustTuple<'_> {
                fn from(value: addPayloadMultiCall) -> Self {
                    (value.signedPayloads,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPayloadMultiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayloads: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addPayloadMultiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addPayloadMultiReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addPayloadMultiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPayloadMultiCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IWavsSDK::SignedPayload>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPayloadMultiReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPayloadMulti((bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [99u8, 113u8, 12u8, 5u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IWavsSDK::SignedPayload,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedPayloads),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `checkAfterExecution(bytes32,bool)` and selector `0x93271368`.
```solidity
function checkAfterExecution(bytes32 txHash, bool success) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAfterExecutionCall {
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        pub success: bool,
    }
    ///Container type for the return parameters of the [`checkAfterExecution(bytes32,bool)`](checkAfterExecutionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkAfterExecutionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                bool,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAfterExecutionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionCall) -> Self {
                    (value.txHash, value.success)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAfterExecutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        txHash: tuple.0,
                        success: tuple.1,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAfterExecutionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkAfterExecutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkAfterExecutionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkAfterExecutionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkAfterExecution(bytes32,bool)";
            const SELECTOR: [u8; 4] = [147u8, 39u8, 19u8, 104u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.txHash),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.success,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)` and selector `0x75f0bb52`.
```solidity
function checkTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, bytes memory signatures, address msgSender) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkTransactionCall {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        pub safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
        pub baseGas: alloy::sol_types::private::primitives::aliases::U256,
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub gasToken: alloy::sol_types::private::Address,
        pub refundReceiver: alloy::sol_types::private::Address,
        pub signatures: alloy::sol_types::private::Bytes,
        pub msgSender: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)`](checkTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkTransactionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkTransactionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkTransactionCall) -> Self {
                    (
                        value.to,
                        value.value,
                        value.data,
                        value.operation,
                        value.safeTxGas,
                        value.baseGas,
                        value.gasPrice,
                        value.gasToken,
                        value.refundReceiver,
                        value.signatures,
                        value.msgSender,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkTransactionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
                        safeTxGas: tuple.4,
                        baseGas: tuple.5,
                        gasPrice: tuple.6,
                        gasToken: tuple.7,
                        refundReceiver: tuple.8,
                        signatures: tuple.9,
                        msgSender: tuple.10,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkTransactionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkTransactionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes,address)";
            const SELECTOR: [u8; 4] = [117u8, 240u8, 187u8, 82u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(
                        &self.operation,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.safeTxGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.baseGas),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gasPrice),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.gasToken,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.refundReceiver,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signatures,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.msgSender,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `estimatedProcessingTime()` and selector `0xc56fb0fd`.
```solidity
function estimatedProcessingTime() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct estimatedProcessingTimeCall {}
    ///Container type for the return parameters of the [`estimatedProcessingTime()`](estimatedProcessingTimeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct estimatedProcessingTimeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<estimatedProcessingTimeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: estimatedProcessingTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for estimatedProcessingTimeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<estimatedProcessingTimeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: estimatedProcessingTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for estimatedProcessingTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for estimatedProcessingTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = estimatedProcessingTimeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "estimatedProcessingTime()";
            const SELECTOR: [u8; 4] = [197u8, 111u8, 176u8, 253u8];
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getTransactionStatus(bytes32)` and selector `0x94407465`.
```solidity
function getTransactionStatus(bytes32 txHash) external view returns (ValidationStatus status, string memory message, uint256 remainingTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransactionStatusCall {
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getTransactionStatus(bytes32)`](getTransactionStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransactionStatusReturn {
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
        pub message: alloy::sol_types::private::String,
        pub remainingTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTransactionStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionStatusCall) -> Self {
                    (value.txHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTransactionStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { txHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                ValidationStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ValidationStatus as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTransactionStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionStatusReturn) -> Self {
                    (value.status, value.message, value.remainingTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTransactionStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        status: tuple.0,
                        message: tuple.1,
                        remainingTime: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTransactionStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTransactionStatusReturn;
            type ReturnTuple<'a> = (
                ValidationStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTransactionStatus(bytes32)";
            const SELECTOR: [u8; 4] = [148u8, 64u8, 116u8, 101u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.txHash),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getUserPendingTransactions(address)` and selector `0x9e83e306`.
```solidity
function getUserPendingTransactions(address user) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserPendingTransactionsCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getUserPendingTransactions(address)`](getUserPendingTransactionsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserPendingTransactionsReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserPendingTransactionsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserPendingTransactionsCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserPendingTransactionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserPendingTransactionsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserPendingTransactionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserPendingTransactionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserPendingTransactionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserPendingTransactionsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserPendingTransactions(address)";
            const SELECTOR: [u8; 4] = [158u8, 131u8, 227u8, 6u8];
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
                        &self.user,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = safeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external pure returns (bool);
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
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `txDetails(bytes32)` and selector `0x7b4f3373`.
```solidity
function txDetails(bytes32) external view returns (address to, uint256 value, bytes memory data, Enum.Operation operation, address initiator, uint256 timestamp, ValidationStatus status, string memory lastStatusMessage, uint256 expirationTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct txDetailsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`txDetails(bytes32)`](txDetailsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct txDetailsReturn {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        pub initiator: alloy::sol_types::private::Address,
        pub timestamp: alloy::sol_types::private::primitives::aliases::U256,
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
        pub lastStatusMessage: alloy::sol_types::private::String,
        pub expirationTime: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<txDetailsCall> for UnderlyingRustTuple<'_> {
                fn from(value: txDetailsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for txDetailsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                ValidationStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <ValidationStatus as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<txDetailsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: txDetailsReturn) -> Self {
                    (
                        value.to,
                        value.value,
                        value.data,
                        value.operation,
                        value.initiator,
                        value.timestamp,
                        value.status,
                        value.lastStatusMessage,
                        value.expirationTime,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for txDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        value: tuple.1,
                        data: tuple.2,
                        operation: tuple.3,
                        initiator: tuple.4,
                        timestamp: tuple.5,
                        status: tuple.6,
                        lastStatusMessage: tuple.7,
                        expirationTime: tuple.8,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for txDetailsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = txDetailsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                ValidationStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "txDetails(bytes32)";
            const SELECTOR: [u8; 4] = [123u8, 79u8, 51u8, 115u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `userPendingTxs(address,uint256)` and selector `0x97f5365a`.
```solidity
function userPendingTxs(address, uint256) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userPendingTxsCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`userPendingTxs(address,uint256)`](userPendingTxsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userPendingTxsReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userPendingTxsCall> for UnderlyingRustTuple<'_> {
                fn from(value: userPendingTxsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for userPendingTxsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userPendingTxsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userPendingTxsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userPendingTxsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userPendingTxsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userPendingTxsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userPendingTxs(address,uint256)";
            const SELECTOR: [u8; 4] = [151u8, 245u8, 54u8, 90u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `validatePayload((bytes,bytes))` and selector `0x216a3e9a`.
```solidity
function validatePayload(IWavsSDK.SignedPayload memory signedPayload) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePayloadCall {
        pub signedPayload: <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`validatePayload((bytes,bytes))`](validatePayloadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePayloadReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IWavsSDK::SignedPayload,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatePayloadCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatePayloadCall) -> Self {
                    (value.signedPayload,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatePayloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayload: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatePayloadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePayloadReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePayloadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatePayloadCall {
            type Parameters<'a> = (IWavsSDK::SignedPayload,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatePayloadReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatePayload((bytes,bytes))";
            const SELECTOR: [u8; 4] = [33u8, 106u8, 62u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IWavsSDK::SignedPayload as alloy_sol_types::SolType>::tokenize(
                        &self.signedPayload,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `validatePayloadMulti((bytes,bytes)[])` and selector `0xa71f8da0`.
```solidity
function validatePayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePayloadMultiCall {
        pub signedPayloads: alloy::sol_types::private::Vec<
            <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`validatePayloadMulti((bytes,bytes)[])`](validatePayloadMultiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatePayloadMultiReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IWavsSDK::SignedPayload>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatePayloadMultiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePayloadMultiCall) -> Self {
                    (value.signedPayloads,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePayloadMultiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayloads: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatePayloadMultiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatePayloadMultiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatePayloadMultiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatePayloadMultiCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IWavsSDK::SignedPayload>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatePayloadMultiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatePayloadMulti((bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [167u8, 31u8, 141u8, 160u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IWavsSDK::SignedPayload,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedPayloads),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `validatedTxs(bytes32)` and selector `0x99d7cf4b`.
```solidity
function validatedTxs(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatedTxsCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validatedTxs(bytes32)`](validatedTxsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatedTxsReturn {
        pub _0: bool,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatedTxsCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatedTxsCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatedTxsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatedTxsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: validatedTxsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatedTxsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatedTxsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatedTxsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatedTxs(bytes32)";
            const SELECTOR: [u8; 4] = [153u8, 215u8, 207u8, 75u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`SafeGuard`](self) function calls.
    pub enum SafeGuardCalls {
        STAKE_REGISTRY(STAKE_REGISTRYCall),
        addPayload(addPayloadCall),
        addPayloadMulti(addPayloadMultiCall),
        checkAfterExecution(checkAfterExecutionCall),
        checkTransaction(checkTransactionCall),
        estimatedProcessingTime(estimatedProcessingTimeCall),
        getTransactionStatus(getTransactionStatusCall),
        getUserPendingTransactions(getUserPendingTransactionsCall),
        safe(safeCall),
        supportsInterface(supportsInterfaceCall),
        txDetails(txDetailsCall),
        userPendingTxs(userPendingTxsCall),
        validatePayload(validatePayloadCall),
        validatePayloadMulti(validatePayloadMultiCall),
        validatedTxs(validatedTxsCall),
    }
    #[automatically_derived]
    impl SafeGuardCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 255u8, 201u8, 167u8],
            [24u8, 111u8, 3u8, 84u8],
            [33u8, 106u8, 62u8, 154u8],
            [99u8, 113u8, 12u8, 5u8],
            [117u8, 240u8, 187u8, 82u8],
            [118u8, 29u8, 225u8, 159u8],
            [123u8, 79u8, 51u8, 115u8],
            [147u8, 39u8, 19u8, 104u8],
            [148u8, 64u8, 116u8, 101u8],
            [151u8, 245u8, 54u8, 90u8],
            [153u8, 215u8, 207u8, 75u8],
            [154u8, 169u8, 253u8, 165u8],
            [158u8, 131u8, 227u8, 6u8],
            [167u8, 31u8, 141u8, 160u8],
            [197u8, 111u8, 176u8, 253u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardCalls {
        const NAME: &'static str = "SafeGuardCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::STAKE_REGISTRY(_) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addPayload(_) => {
                    <addPayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addPayloadMulti(_) => {
                    <addPayloadMultiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkAfterExecution(_) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkTransaction(_) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::estimatedProcessingTime(_) => {
                    <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTransactionStatus(_) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserPendingTransactions(_) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::txDetails(_) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userPendingTxs(_) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatePayload(_) => {
                    <validatePayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatePayloadMulti(_) => {
                    <validatePayloadMultiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatedTxs(_) => {
                    <validatedTxsCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SafeGuardCalls>] = &[
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn safe(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::safe)
                    }
                    safe
                },
                {
                    fn validatePayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <validatePayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::validatePayload)
                    }
                    validatePayload
                },
                {
                    fn addPayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <addPayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::addPayloadMulti)
                    }
                    addPayloadMulti
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn STAKE_REGISTRY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::STAKE_REGISTRY)
                    }
                    STAKE_REGISTRY
                },
                {
                    fn txDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <txDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::txDetails)
                    }
                    txDetails
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::checkAfterExecution)
                    }
                    checkAfterExecution
                },
                {
                    fn getTransactionStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::getTransactionStatus)
                    }
                    getTransactionStatus
                },
                {
                    fn userPendingTxs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <userPendingTxsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::userPendingTxs)
                    }
                    userPendingTxs
                },
                {
                    fn validatedTxs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <validatedTxsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::validatedTxs)
                    }
                    validatedTxs
                },
                {
                    fn addPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <addPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::addPayload)
                    }
                    addPayload
                },
                {
                    fn getUserPendingTransactions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::getUserPendingTransactions)
                    }
                    getUserPendingTransactions
                },
                {
                    fn validatePayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <validatePayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::validatePayloadMulti)
                    }
                    validatePayloadMulti
                },
                {
                    fn estimatedProcessingTime(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardCalls::estimatedProcessingTime)
                    }
                    estimatedProcessingTime
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addPayload(inner) => {
                    <addPayloadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addPayloadMulti(inner) => {
                    <addPayloadMultiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::estimatedProcessingTime(inner) => {
                    <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserPendingTransactions(inner) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::userPendingTxs(inner) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatePayload(inner) => {
                    <validatePayloadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatePayloadMulti(inner) => {
                    <validatePayloadMultiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatedTxs(inner) => {
                    <validatedTxsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addPayload(inner) => {
                    <addPayloadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addPayloadMulti(inner) => {
                    <addPayloadMultiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::estimatedProcessingTime(inner) => {
                    <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserPendingTransactions(inner) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userPendingTxs(inner) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatePayload(inner) => {
                    <validatePayloadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatePayloadMulti(inner) => {
                    <validatePayloadMultiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatedTxs(inner) => {
                    <validatedTxsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) custom errors.
    pub enum SafeGuardErrors {
        AsyncValidationRequired(AsyncValidationRequired),
        InvalidSignature(InvalidSignature),
        TransactionExpired(TransactionExpired),
        TransactionNotFound(TransactionNotFound),
    }
    #[automatically_derived]
    impl SafeGuardErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [49u8, 251u8, 135u8, 143u8],
            [54u8, 252u8, 87u8, 19u8],
            [139u8, 170u8, 87u8, 159u8],
            [227u8, 151u8, 149u8, 44u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardErrors {
        const NAME: &'static str = "SafeGuardErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AsyncValidationRequired(_) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransactionExpired(_) => {
                    <TransactionExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransactionNotFound(_) => {
                    <TransactionNotFound as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SafeGuardErrors>] = &[
                {
                    fn TransactionNotFound(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <TransactionNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardErrors::TransactionNotFound)
                    }
                    TransactionNotFound
                },
                {
                    fn AsyncValidationRequired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <AsyncValidationRequired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardErrors::AsyncValidationRequired)
                    }
                    AsyncValidationRequired
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn TransactionExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <TransactionExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeGuardErrors::TransactionExpired)
                    }
                    TransactionExpired
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AsyncValidationRequired(inner) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TransactionExpired(inner) => {
                    <TransactionExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TransactionNotFound(inner) => {
                    <TransactionNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AsyncValidationRequired(inner) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransactionExpired(inner) => {
                    <TransactionExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransactionNotFound(inner) => {
                    <TransactionNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) events.
    pub enum SafeGuardEvents {
        ValidationRequired(ValidationRequired),
        ValidationStatusUpdated(ValidationStatusUpdated),
        WavsTriggerEvent(WavsTriggerEvent),
    }
    #[automatically_derived]
    impl SafeGuardEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                55u8,
                177u8,
                103u8,
                31u8,
                119u8,
                123u8,
                30u8,
                161u8,
                23u8,
                16u8,
                220u8,
                129u8,
                107u8,
                146u8,
                218u8,
                15u8,
                139u8,
                94u8,
                169u8,
                71u8,
                48u8,
                85u8,
                43u8,
                181u8,
                99u8,
                123u8,
                14u8,
                197u8,
                54u8,
                129u8,
                25u8,
                8u8,
            ],
            [
                92u8,
                82u8,
                185u8,
                32u8,
                252u8,
                93u8,
                10u8,
                196u8,
                88u8,
                56u8,
                194u8,
                5u8,
                173u8,
                146u8,
                101u8,
                6u8,
                18u8,
                197u8,
                206u8,
                91u8,
                248u8,
                19u8,
                106u8,
                240u8,
                47u8,
                166u8,
                148u8,
                102u8,
                204u8,
                58u8,
                31u8,
                217u8,
            ],
            [
                114u8,
                184u8,
                190u8,
                170u8,
                43u8,
                22u8,
                239u8,
                194u8,
                15u8,
                247u8,
                174u8,
                169u8,
                66u8,
                161u8,
                34u8,
                247u8,
                183u8,
                129u8,
                25u8,
                114u8,
                79u8,
                171u8,
                189u8,
                128u8,
                106u8,
                205u8,
                100u8,
                215u8,
                151u8,
                137u8,
                84u8,
                203u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SafeGuardEvents {
        const NAME: &'static str = "SafeGuardEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <ValidationRequired as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidationRequired as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidationRequired)
                }
                Some(
                    <ValidationStatusUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidationStatusUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidationStatusUpdated)
                }
                Some(<WavsTriggerEvent as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WavsTriggerEvent as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WavsTriggerEvent)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for SafeGuardEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ValidationRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WavsTriggerEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ValidationRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WavsTriggerEvent(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`SafeGuard`](self) contract instance.

See the [wrapper's documentation](`SafeGuardInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SafeGuardInstance<T, P, N> {
        SafeGuardInstance::<T, P, N>::new(address, provider)
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
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SafeGuardInstance<T, P, N>>,
    > {
        SafeGuardInstance::<T, P, N>::deploy(provider, _safe, _stakeRegistry)
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
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        SafeGuardInstance::<T, P, N>::deploy_builder(provider, _safe, _stakeRegistry)
    }
    /**A [`SafeGuard`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`SafeGuard`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SafeGuardInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SafeGuardInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SafeGuardInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SafeGuardInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SafeGuard`](self) contract instance.

See the [wrapper's documentation](`SafeGuardInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _safe: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<SafeGuardInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _safe, _stakeRegistry);
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
            _stakeRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _safe,
                            _stakeRegistry,
                        },
                    )[..],
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
    impl<T, P: ::core::clone::Clone, N> SafeGuardInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SafeGuardInstance<T, P, N> {
            SafeGuardInstance {
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
    > SafeGuardInstance<T, P, N> {
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
        ///Creates a new call builder for the [`STAKE_REGISTRY`] function.
        pub fn STAKE_REGISTRY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, STAKE_REGISTRYCall, N> {
            self.call_builder(&STAKE_REGISTRYCall {})
        }
        ///Creates a new call builder for the [`addPayload`] function.
        pub fn addPayload(
            &self,
            signedPayload: <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPayloadCall, N> {
            self.call_builder(&addPayloadCall { signedPayload })
        }
        ///Creates a new call builder for the [`addPayloadMulti`] function.
        pub fn addPayloadMulti(
            &self,
            signedPayloads: alloy::sol_types::private::Vec<
                <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPayloadMultiCall, N> {
            self.call_builder(
                &addPayloadMultiCall {
                    signedPayloads,
                },
            )
        }
        ///Creates a new call builder for the [`checkAfterExecution`] function.
        pub fn checkAfterExecution(
            &self,
            txHash: alloy::sol_types::private::FixedBytes<32>,
            success: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkAfterExecutionCall, N> {
            self.call_builder(
                &checkAfterExecutionCall {
                    txHash,
                    success,
                },
            )
        }
        ///Creates a new call builder for the [`checkTransaction`] function.
        pub fn checkTransaction(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
            safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
            baseGas: alloy::sol_types::private::primitives::aliases::U256,
            gasPrice: alloy::sol_types::private::primitives::aliases::U256,
            gasToken: alloy::sol_types::private::Address,
            refundReceiver: alloy::sol_types::private::Address,
            signatures: alloy::sol_types::private::Bytes,
            msgSender: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkTransactionCall, N> {
            self.call_builder(
                &checkTransactionCall {
                    to,
                    value,
                    data,
                    operation,
                    safeTxGas,
                    baseGas,
                    gasPrice,
                    gasToken,
                    refundReceiver,
                    signatures,
                    msgSender,
                },
            )
        }
        ///Creates a new call builder for the [`estimatedProcessingTime`] function.
        pub fn estimatedProcessingTime(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, estimatedProcessingTimeCall, N> {
            self.call_builder(&estimatedProcessingTimeCall {})
        }
        ///Creates a new call builder for the [`getTransactionStatus`] function.
        pub fn getTransactionStatus(
            &self,
            txHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTransactionStatusCall, N> {
            self.call_builder(&getTransactionStatusCall { txHash })
        }
        ///Creates a new call builder for the [`getUserPendingTransactions`] function.
        pub fn getUserPendingTransactions(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserPendingTransactionsCall, N> {
            self.call_builder(
                &getUserPendingTransactionsCall {
                    user,
                },
            )
        }
        ///Creates a new call builder for the [`safe`] function.
        pub fn safe(&self) -> alloy_contract::SolCallBuilder<T, &P, safeCall, N> {
            self.call_builder(&safeCall {})
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`txDetails`] function.
        pub fn txDetails(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, txDetailsCall, N> {
            self.call_builder(&txDetailsCall { _0 })
        }
        ///Creates a new call builder for the [`userPendingTxs`] function.
        pub fn userPendingTxs(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, userPendingTxsCall, N> {
            self.call_builder(&userPendingTxsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`validatePayload`] function.
        pub fn validatePayload(
            &self,
            signedPayload: <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatePayloadCall, N> {
            self.call_builder(
                &validatePayloadCall {
                    signedPayload,
                },
            )
        }
        ///Creates a new call builder for the [`validatePayloadMulti`] function.
        pub fn validatePayloadMulti(
            &self,
            signedPayloads: alloy::sol_types::private::Vec<
                <IWavsSDK::SignedPayload as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatePayloadMultiCall, N> {
            self.call_builder(
                &validatePayloadMultiCall {
                    signedPayloads,
                },
            )
        }
        ///Creates a new call builder for the [`validatedTxs`] function.
        pub fn validatedTxs(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatedTxsCall, N> {
            self.call_builder(&validatedTxsCall { _0 })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SafeGuardInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`ValidationRequired`] event.
        pub fn ValidationRequired_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidationRequired, N> {
            self.event_filter::<ValidationRequired>()
        }
        ///Creates a new event filter for the [`ValidationStatusUpdated`] event.
        pub fn ValidationStatusUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidationStatusUpdated, N> {
            self.event_filter::<ValidationStatusUpdated>()
        }
        ///Creates a new event filter for the [`WavsTriggerEvent`] event.
        pub fn WavsTriggerEvent_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WavsTriggerEvent, N> {
            self.event_filter::<WavsTriggerEvent>()
        }
    }
}
