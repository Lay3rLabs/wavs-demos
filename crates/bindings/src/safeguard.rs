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
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
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
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
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
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EnumInstance<T, P, N> {
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
        > EnumInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`Enum`](self) contract instance.

        See the [wrapper's documentation](`EnumInstance`) for more details.*/
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
        > EnumInstance<T, P, N>
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
        > EnumInstance<T, P, N>
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
library Enum {
    type Operation is uint8;
}

library ISimpleTrigger {
    type TriggerId is uint64;
    struct TriggerInfo {
        TriggerId triggerId;
        address creator;
        bytes data;
    }
}

interface SafeGuard {
    type ValidationStatus is uint8;

    error AsyncValidationRequired();
    error TransactionExpired();
    error TransactionNotFound();

    event NewTrigger(bytes);
    event ValidationRequired(bytes32 indexed txHash, address indexed to, uint256 value, bytes data, Enum.Operation operation, address initiator, uint256 estimatedProcessingTime);
    event ValidationStatusUpdated(bytes32 indexed txHash, ValidationStatus status, string message);

    constructor(address _safe);

    function checkAfterExecution(bytes32 txHash, bool success) external;
    function checkTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address payable refundReceiver, bytes memory signatures, address msgSender) external;
    function estimatedProcessingTime() external view returns (uint256);
    function getTransactionStatus(bytes32 txHash) external view returns (ValidationStatus status, string memory message, uint256 remainingTime);
    function getTrigger(ISimpleTrigger.TriggerId triggerId) external view returns (ISimpleTrigger.TriggerInfo memory);
    function getUserPendingTransactions(address user) external view returns (bytes32[] memory);
    function handleAddPayload(bytes memory data, bytes memory signature) external;
    function initialize(address _serviceProvider) external;
    function initialized() external view returns (bool);
    function nextTriggerId() external view returns (ISimpleTrigger.TriggerId);
    function safe() external view returns (address);
    function serviceProvider() external view returns (address);
    function supportsInterface(bytes4 interfaceId) external pure returns (bool);
    function triggerIdsByCreator(address, uint256) external view returns (ISimpleTrigger.TriggerId);
    function triggersById(ISimpleTrigger.TriggerId) external view returns (address creator, bytes memory data);
    function txDetails(bytes32) external view returns (address to, uint256 value, bytes memory data, Enum.Operation operation, address initiator, uint256 timestamp, ValidationStatus status, string memory lastStatusMessage, uint256 expirationTime);
    function userPendingTxs(address, uint256) external view returns (bytes32);
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
      }
    ],
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
    "type": "error",
    "name": "AsyncValidationRequired",
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
    ///0x60a060405260785f55348015610013575f5ffd5b50604051611e9f380380611e9f8339810160408190526100329161009d565b6001600160a01b03811661008c5760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420736166652061646472657373000000000000000000000000604482015260640160405180910390fd5b6001600160a01b03166080526100ca565b5f602082840312156100ad575f5ffd5b81516001600160a01b03811681146100c3575f5ffd5b9392505050565b608051611daf6100f05f395f818161017b0152818161051e0152610c100152611daf5ff3fe608060405234801561000f575f5ffd5b5060043610610110575f3560e01c8063932713681161009e5780639e83e3061161006e5780639e83e306146102b9578063c4d66de8146102d9578063c56fb0fd146102ec578063ce289612146102f4578063e328ed7714610315575f5ffd5b80639327136814610241578063944074651461025457806397f5365a1461027657806399d7cf4b14610297575f5ffd5b806342227fa4116100e457806342227fa4146101b557806375f0bb52146101e05780637b4f3373146101f35780638d69e95e1461021b578063913b1fbf1461022e575f5ffd5b806273e1d71461011457806301ffc9a714610129578063158ef93e14610162578063186f035414610176575b5f5ffd5b6101276101223660046114c7565b610335565b005b61014d610137366004611531565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b60015461014d90600160a01b900460ff1681565b61019d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610159565b6007546101c8906001600160401b031681565b6040516001600160401b039091168152602001610159565b6101276101ee366004611638565b610513565b610206610201366004611713565b610a4d565b604051610159999897969594939291906117af565b60015461019d906001600160a01b031681565b6101c861023c366004611831565b610bbe565b61012761024f36600461186a565b610c05565b610267610262366004611713565b610eda565b60405161015993929190611894565b610289610284366004611831565b61107b565b604051908152602001610159565b61014d6102a5366004611713565b60026020525f908152604090205460ff1681565b6102cc6102c73660046118c3565b6110a6565b60405161015991906118de565b6101276102e73660046118c3565b61110f565b6102895f5481565b610307610302366004611920565b6111dc565b604051610159929190611946565b610328610323366004611920565b611288565b6040516101599190611971565b6001546001600160a01b031633146103a95760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f80806103b8868801886119b0565b5f8381526003602052604081206004810154949750929550909350909190036103f4576040516331fb878f60e01b815260040160405180910390fd5b80600701544211156104665760058101805460ff1916600490811790915560405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd99161044591908690611a15565b60405180910390a26040516338e5e54b60e21b815260040160405180910390fd5b82610472576003610475565b60025b60058201805460ff1916600183600481111561049357610493611777565b0217905550600681016104a68382611ab8565b5082156104c6575f848152600260205260409020805460ff191660011790555b600581015460405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd9916105019160ff16908690611a15565b60405180910390a25050505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461057a5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016103a0565b5f6040518061012001604052808d6001600160a01b031681526020018c81526020018b81526020018a60018111156105b4576105b4611777565b81526001600160a01b0384166020820152426040820152606001600181526020016040518060400160405280601681526020017556616c69646174696f6e20696e2070726f677265737360501b81525081526020015f54426106169190611b86565b81525090505f815f015182602001518360400151846060015185608001518660a0015160405160200161064e96959493929190611b9f565b60408051601f1981840301815291815281516020928301205f818152600290935291205490915060ff1615610684575050610a40565b5f81815260036020526040902060040154158015906106b257505f8181526003602052604090206007015442115b156106d0576040516338e5e54b60e21b815260040160405180910390fd5b5f81815260036020908152604091829020845181546001600160a01b0319166001600160a01b0390911617815590840151600182015590830151839190600282019061071c9082611ab8565b50606082015160038201805460ff19166001838181111561073f5761073f611777565b021790555060808201518160030160016101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a0820151816004015560c0820151816005015f6101000a81548160ff021916908360048111156107a4576107a4611777565b021790555060e082015160068201906107bd9082611ab8565b5061010091909101516007918201556001600160a01b0384165f908152600460209081526040822080546001818101835591845291832090910184905591549091610811916001600160401b031690611c16565b6007805467ffffffffffffffff19166001600160401b0383161790558351602080860151604080880151606089015160808a015192519697505f9661085c968a969095949101611c35565b60408051808303601f19018152828201825260808701516001600160a01b03908116845260208085018381526001600160401b0388165f9081526005909252939020845181546001600160a01b0319169216919091178155915190935060018201906108c89082611ab8565b5050506001600160a01b038086165f908152600660209081526040808320805460018101825590845292829020600484040180546001600160401b0380891660086003909716969096026101000a868102910219909116179055805160608101825292835260808801519093168282015281830184905291517f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e689261096e929101611971565b60408051601f198184030181529082905261098891611ca9565b60405180910390a1835f01516001600160a01b0316837f72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb86602001518760400151886060015189608001515f546040516109e6959493929190611cbb565b60405180910390a3827f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd96001604051610a1f9190611cfe565b60405180910390a26040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b60036020525f90815260409020805460018201546002830180546001600160a01b03909316939192610a7e90611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610aaa90611a34565b8015610af55780601f10610acc57610100808354040283529160200191610af5565b820191905f5260205f20905b815481529060010190602001808311610ad857829003601f168201915b50505050600383015460048401546005850154600686018054959660ff808616976101009096046001600160a01b0316965093949390921692610b3790611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610b6390611a34565b8015610bae5780601f10610b8557610100808354040283529160200191610bae565b820191905f5260205f20905b815481529060010190602001808311610b9157829003601f168201915b5050505050908060070154905089565b6006602052815f5260405f208181548110610bd7575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610c6c5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016103a0565b5f82815260036020819052604090912090810154610c989061010090046001600160a01b031684611371565b600754610caf906001600160401b03166001611c16565b6007805467ffffffffffffffff19166001600160401b039290921691821790556040805160606020820181905260046080830152631c1bdcdd60e21b60a0830152918101869052841515918101919091525f9060c00160408051808303601f190181528282018252600386015461010090046001600160a01b03908116845260208085018381526001600160401b0388165f9081526005909252939020845181546001600160a01b031916921691909117815591519093508291906001820190610d799082611ab8565b5050506003808501546001600160a01b036101009182900481165f908152600660209081526040808320805460018101825590845292829020600484040180546001600160401b03808c166008969099169590950290960a8781029402199095169290921790935580516060810182529384528451909116838301528382015183820152517f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e6891610e2c91849101611971565b60408051601f1981840301815290829052610e4691611ca9565b60405180910390a15f87815260036020526040812080546001600160a01b03191681556001810182905590610e7e6002830182611431565b6003820180546001600160a81b03191690555f6004830181905560058301805460ff19169055610eb2906006840190611431565b505f6007919091018190559687525050600260205250506040909220805460ff191690555050565b5f818152600360205260408120600481015460609183918203610f1257505060408051602081019091525f8082529250905081611074565b8060070154421115610fbb576004816006015f818054610f3190611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610f5d90611a34565b8015610fa85780601f10610f7f57610100808354040283529160200191610fa8565b820191905f5260205f20905b815481529060010190602001808311610f8b57829003601f168201915b5050505050915093509350935050611074565b5f428260070154610fcc9190611d3e565b600583015460068401805492935060ff9091169183908290610fed90611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461101990611a34565b80156110645780601f1061103b57610100808354040283529160200191611064565b820191905f5260205f20905b81548152906001019060200180831161104757829003601f168201915b5050505050915094509450945050505b9193909250565b6004602052815f5260405f208181548110611094575f80fd5b905f5260205f20015f91509150505481565b6001600160a01b0381165f9081526004602090815260409182902080548351818402810184019094528084526060939283018282801561110357602002820191905f5260205f20905b8154815260200190600101908083116110ef575b50505050509050919050565b600154600160a01b900460ff161561115f5760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b60448201526064016103a0565b6001600160a01b0381166111b55760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f7669646572206164647265737360448201526064016103a0565b600180546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b60056020525f9081526040902080546001820180546001600160a01b03909216929161120790611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461123390611a34565b801561127e5780601f106112555761010080835404028352916020019161127e565b820191905f5260205f20905b81548152906001019060200180831161126157829003601f168201915b5050505050905082565b60408051606080820183525f80835260208084018290528385018390526001600160401b03861680835260058252918590208551938401865291835281546001600160a01b03169083015260018101805493949193918301916112ea90611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461131690611a34565b80156113615780601f1061133857610100808354040283529160200191611361565b820191905f5260205f20905b81548152906001019060200180831161134457829003601f168201915b5050505050815250915050919050565b6001600160a01b0382165f908152600460205260408120905b815481101561142b57828282815481106113a6576113a6611d51565b905f5260205f2001540361142357815482906113c490600190611d3e565b815481106113d4576113d4611d51565b905f5260205f2001548282815481106113ef576113ef611d51565b905f5260205f2001819055508180548061140b5761140b611d65565b600190038181905f5260205f20015f9055905561142b565b60010161138a565b50505050565b50805461143d90611a34565b5f825580601f1061144c575050565b601f0160209004905f5260205f2090810190611468919061146b565b50565b5b8082111561147f575f815560010161146c565b5090565b5f5f83601f840112611493575f5ffd5b5081356001600160401b038111156114a9575f5ffd5b6020830191508360208285010111156114c0575f5ffd5b9250929050565b5f5f5f5f604085870312156114da575f5ffd5b84356001600160401b038111156114ef575f5ffd5b6114fb87828801611483565b90955093505060208501356001600160401b03811115611519575f5ffd5b61152587828801611483565b95989497509550505050565b5f60208284031215611541575f5ffd5b81356001600160e01b031981168114611558575f5ffd5b9392505050565b6001600160a01b0381168114611468575f5ffd5b803561157e8161155f565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b038411156115b0576115b0611583565b50604051601f19601f85018116603f011681018181106001600160401b03821117156115de576115de611583565b6040528381529050808284018510156115f5575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f83011261161b575f5ffd5b61155883833560208501611597565b80356002811061157e575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e031215611653575f5ffd5b61165c8c611573565b9a5060208c0135995060408c01356001600160401b0381111561167d575f5ffd5b6116898e828f0161160c565b99505061169860608d0161162a565b975060808c0135965060a08c0135955060c08c013594506116bb60e08d01611573565b93506116ca6101008d01611573565b92506101208c01356001600160401b038111156116e5575f5ffd5b6116f18e828f0161160c565b9250506117016101408d01611573565b90509295989b509295989b9093969950565b5f60208284031215611723575f5ffd5b5035919050565b5f5b8381101561174457818101518382015260200161172c565b50505f910152565b5f815180845261176381602086016020860161172a565b601f01601f19169290920160200192915050565b634e487b7160e01b5f52602160045260245ffd5b6002811061179b5761179b611777565b9052565b6005811061179b5761179b611777565b60018060a01b038a16815288602082015261012060408201525f6117d761012083018a61174c565b6117e4606084018a61178b565b6001600160a01b038816608084015260a0830187905261180760c084018761179f565b82810360e0840152611819818661174c565b915050826101008301529a9950505050505050505050565b5f5f60408385031215611842575f5ffd5b823561184d8161155f565b946020939093013593505050565b8035801515811461157e575f5ffd5b5f5f6040838503121561187b575f5ffd5b8235915061188b6020840161185b565b90509250929050565b61189e818561179f565b606060208201525f6118b3606083018561174c565b9050826040830152949350505050565b5f602082840312156118d3575f5ffd5b81356115588161155f565b602080825282518282018190525f918401906040840190835b818110156119155783518352602093840193909201916001016118f7565b509095945050505050565b5f60208284031215611930575f5ffd5b81356001600160401b0381168114611558575f5ffd5b6001600160a01b03831681526040602082018190525f906119699083018461174c565b949350505050565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f6040830151606080840152611969608084018261174c565b5f5f5f606084860312156119c2575f5ffd5b833592506119d26020850161185b565b915060408401356001600160401b038111156119ec575f5ffd5b8401601f810186136119fc575f5ffd5b611a0b86823560208401611597565b9150509250925092565b611a1f818461179f565b604060208201525f611969604083018461174c565b600181811c90821680611a4857607f821691505b602082108103611a6657634e487b7160e01b5f52602260045260245ffd5b50919050565b601f821115611ab357805f5260205f20601f840160051c81016020851015611a915750805b601f840160051c820191505b81811015611ab0575f8155600101611a9d565b50505b505050565b81516001600160401b03811115611ad157611ad1611583565b611ae581611adf8454611a34565b84611a6c565b6020601f821160018114611b17575f8315611b005750848201515b5f19600385901b1c1916600184901b178455611ab0565b5f84815260208120601f198516915b82811015611b465787850151825560209485019460019092019101611b26565b5084821015611b6357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b5f52601160045260245ffd5b80820180821115611b9957611b99611b72565b92915050565b6bffffffffffffffffffffffff198760601b1681528560148201525f8551611bce816034850160208a0161172a565b820160028610611be057611be0611777565b60f89590951b6034860152505060609190911b6bffffffffffffffffffffffff1916603583015260498201526069019392505050565b6001600160401b038181168382160190811115611b9957611b99611b72565b60e08152600360e08201526270726560e81b61010082015286602082015260018060a01b038616604082015284606082015261012060808201525f611c7e61012083018661174c565b9050611c8d60a083018561178b565b6001600160a01b039290921660c0919091015295945050505050565b602081525f611558602083018461174c565b85815260a060208201525f611cd360a083018761174c565b9050611ce2604083018661178b565b6001600160a01b03939093166060820152608001529392505050565b611d08818361179f565b6040602082018190526016908201527556616c69646174696f6e20696e2070726f677265737360501b6060820152608001919050565b81810381811115611b9957611b99611b72565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffdfea26469706673582212201fe1295866af94a0ed68b7c941d130f0a872e7b7dd99e6d7eac6bd08d0d2e84a64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R`x_U4\x80\x15a\0\x13W__\xFD[P`@Qa\x1E\x9F8\x03\x80a\x1E\x9F\x839\x81\x01`@\x81\x90Ra\x002\x91a\0\x9DV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid safe address\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0\xCAV[_` \x82\x84\x03\x12\x15a\0\xADW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xC3W__\xFD[\x93\x92PPPV[`\x80Qa\x1D\xAFa\0\xF0_9_\x81\x81a\x01{\x01R\x81\x81a\x05\x1E\x01Ra\x0C\x10\x01Ra\x1D\xAF_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x10W_5`\xE0\x1C\x80c\x93'\x13h\x11a\0\x9EW\x80c\x9E\x83\xE3\x06\x11a\0nW\x80c\x9E\x83\xE3\x06\x14a\x02\xB9W\x80c\xC4\xD6m\xE8\x14a\x02\xD9W\x80c\xC5o\xB0\xFD\x14a\x02\xECW\x80c\xCE(\x96\x12\x14a\x02\xF4W\x80c\xE3(\xEDw\x14a\x03\x15W__\xFD[\x80c\x93'\x13h\x14a\x02AW\x80c\x94@te\x14a\x02TW\x80c\x97\xF56Z\x14a\x02vW\x80c\x99\xD7\xCFK\x14a\x02\x97W__\xFD[\x80cB\"\x7F\xA4\x11a\0\xE4W\x80cB\"\x7F\xA4\x14a\x01\xB5W\x80cu\xF0\xBBR\x14a\x01\xE0W\x80c{O3s\x14a\x01\xF3W\x80c\x8Di\xE9^\x14a\x02\x1BW\x80c\x91;\x1F\xBF\x14a\x02.W__\xFD[\x80bs\xE1\xD7\x14a\x01\x14W\x80c\x01\xFF\xC9\xA7\x14a\x01)W\x80c\x15\x8E\xF9>\x14a\x01bW\x80c\x18o\x03T\x14a\x01vW[__\xFD[a\x01'a\x01\"6`\x04a\x14\xC7V[a\x035V[\0[a\x01Ma\x0176`\x04a\x151V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01M\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x01\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[`\x07Ta\x01\xC8\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[a\x01'a\x01\xEE6`\x04a\x168V[a\x05\x13V[a\x02\x06a\x02\x016`\x04a\x17\x13V[a\nMV[`@Qa\x01Y\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x17\xAFV[`\x01Ta\x01\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xC8a\x02<6`\x04a\x181V[a\x0B\xBEV[a\x01'a\x02O6`\x04a\x18jV[a\x0C\x05V[a\x02ga\x02b6`\x04a\x17\x13V[a\x0E\xDAV[`@Qa\x01Y\x93\x92\x91\x90a\x18\x94V[a\x02\x89a\x02\x846`\x04a\x181V[a\x10{V[`@Q\x90\x81R` \x01a\x01YV[a\x01Ma\x02\xA56`\x04a\x17\x13V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xCCa\x02\xC76`\x04a\x18\xC3V[a\x10\xA6V[`@Qa\x01Y\x91\x90a\x18\xDEV[a\x01'a\x02\xE76`\x04a\x18\xC3V[a\x11\x0FV[a\x02\x89_T\x81V[a\x03\x07a\x03\x026`\x04a\x19 V[a\x11\xDCV[`@Qa\x01Y\x92\x91\x90a\x19FV[a\x03(a\x03#6`\x04a\x19 V[a\x12\x88V[`@Qa\x01Y\x91\x90a\x19qV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x03\xB8\x86\x88\x01\x88a\x19\xB0V[_\x83\x81R`\x03` R`@\x81 `\x04\x81\x01T\x94\x97P\x92\x95P\x90\x93P\x90\x91\x90\x03a\x03\xF4W`@Qc1\xFB\x87\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x07\x01TB\x11\x15a\x04fW`\x05\x81\x01\x80T`\xFF\x19\x16`\x04\x90\x81\x17\x90\x91U`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x04E\x91\x90\x86\x90a\x1A\x15V[`@Q\x80\x91\x03\x90\xA2`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x04rW`\x03a\x04uV[`\x02[`\x05\x82\x01\x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x04\x93Wa\x04\x93a\x17wV[\x02\x17\x90UP`\x06\x81\x01a\x04\xA6\x83\x82a\x1A\xB8V[P\x82\x15a\x04\xC6W_\x84\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[`\x05\x81\x01T`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x05\x01\x91`\xFF\x16\x90\x86\x90a\x1A\x15V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x03\xA0V[_`@Q\x80a\x01 \x01`@R\x80\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A`\x01\x81\x11\x15a\x05\xB4Wa\x05\xB4a\x17wV[\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01RB`@\x82\x01R``\x01`\x01\x81R` \x01`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uValidation in progress`P\x1B\x81RP\x81R` \x01_TBa\x06\x16\x91\x90a\x1B\x86V[\x81RP\x90P_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\x80\x01Q\x86`\xA0\x01Q`@Q` \x01a\x06N\x96\x95\x94\x93\x92\x91\x90a\x1B\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x02\x90\x93R\x91 T\x90\x91P`\xFF\x16\x15a\x06\x84WPPa\n@V[_\x81\x81R`\x03` R`@\x90 `\x04\x01T\x15\x80\x15\x90a\x06\xB2WP_\x81\x81R`\x03` R`@\x90 `\x07\x01TB\x11[\x15a\x06\xD0W`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q`\x01\x82\x01U\x90\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a\x07\x1C\x90\x82a\x1A\xB8V[P``\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x07?Wa\x07?a\x17wV[\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a\x07\xA4Wa\x07\xA4a\x17wV[\x02\x17\x90UP`\xE0\x82\x01Q`\x06\x82\x01\x90a\x07\xBD\x90\x82a\x1A\xB8V[Pa\x01\0\x91\x90\x91\x01Q`\x07\x91\x82\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x82 \x80T`\x01\x81\x81\x01\x83U\x91\x84R\x91\x83 \x90\x91\x01\x84\x90U\x91T\x90\x91a\x08\x11\x91`\x01`\x01`@\x1B\x03\x16\x90a\x1C\x16V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17\x90U\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q``\x89\x01Q`\x80\x8A\x01Q\x92Q\x96\x97P_\x96a\x08\\\x96\x8A\x96\x90\x95\x94\x91\x01a\x1C5V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R`\x80\x87\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R` \x80\x85\x01\x83\x81R`\x01`\x01`@\x1B\x03\x88\x16_\x90\x81R`\x05\x90\x92R\x93\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x81U\x91Q\x90\x93P`\x01\x82\x01\x90a\x08\xC8\x90\x82a\x1A\xB8V[PPP`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 `\x04\x84\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x97\x16\x96\x90\x96\x02a\x01\0\n\x86\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x80Q``\x81\x01\x82R\x92\x83R`\x80\x88\x01Q\x90\x93\x16\x82\x82\x01R\x81\x83\x01\x84\x90R\x91Q\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x92a\tn\x92\x91\x01a\x19qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\x88\x91a\x1C\xA9V[`@Q\x80\x91\x03\x90\xA1\x83_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x7Fr\xB8\xBE\xAA+\x16\xEF\xC2\x0F\xF7\xAE\xA9B\xA1\"\xF7\xB7\x81\x19rO\xAB\xBD\x80j\xCDd\xD7\x97\x89T\xCB\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q_T`@Qa\t\xE6\x95\x94\x93\x92\x91\x90a\x1C\xBBV[`@Q\x80\x91\x03\x90\xA3\x82\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9`\x01`@Qa\n\x1F\x91\x90a\x1C\xFEV[`@Q\x80\x91\x03\x90\xA2`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93\x91\x92a\n~\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xAA\x90a\x1A4V[\x80\x15a\n\xF5W\x80`\x1F\x10a\n\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01\x80T\x95\x96`\xFF\x80\x86\x16\x97a\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P\x93\x94\x93\x90\x92\x16\x92a\x0B7\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bc\x90a\x1A4V[\x80\x15a\x0B\xAEW\x80`\x1F\x10a\x0B\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x07\x01T\x90P\x89V[`\x06` R\x81_R`@_ \x81\x81T\x81\x10a\x0B\xD7W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0ClW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x03\xA0V[_\x82\x81R`\x03` \x81\x90R`@\x90\x91 \x90\x81\x01Ta\x0C\x98\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a\x13qV[`\x07Ta\x0C\xAF\x90`\x01`\x01`@\x1B\x03\x16`\x01a\x1C\x16V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q``` \x82\x01\x81\x90R`\x04`\x80\x83\x01Rc\x1C\x1B\xDC\xDD`\xE2\x1B`\xA0\x83\x01R\x91\x81\x01\x86\x90R\x84\x15\x15\x91\x81\x01\x91\x90\x91R_\x90`\xC0\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R`\x03\x86\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R` \x80\x85\x01\x83\x81R`\x01`\x01`@\x1B\x03\x88\x16_\x90\x81R`\x05\x90\x92R\x93\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x81U\x91Q\x90\x93P\x82\x91\x90`\x01\x82\x01\x90a\ry\x90\x82a\x1A\xB8V[PPP`\x03\x80\x85\x01T`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 `\x04\x84\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x8C\x16`\x08\x96\x90\x99\x16\x95\x90\x95\x02\x90\x96\n\x87\x81\x02\x94\x02\x19\x90\x95\x16\x92\x90\x92\x17\x90\x93U\x80Q``\x81\x01\x82R\x93\x84R\x84Q\x90\x91\x16\x83\x83\x01R\x83\x82\x01Q\x83\x82\x01RQ\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x91a\x0E,\x91\x84\x91\x01a\x19qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0EF\x91a\x1C\xA9V[`@Q\x80\x91\x03\x90\xA1_\x87\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U\x90a\x0E~`\x02\x83\x01\x82a\x141V[`\x03\x82\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U_`\x04\x83\x01\x81\x90U`\x05\x83\x01\x80T`\xFF\x19\x16\x90Ua\x0E\xB2\x90`\x06\x84\x01\x90a\x141V[P_`\x07\x91\x90\x91\x01\x81\x90U\x96\x87RPP`\x02` RPP`@\x90\x92 \x80T`\xFF\x19\x16\x90UPPV[_\x81\x81R`\x03` R`@\x81 `\x04\x81\x01T``\x91\x83\x91\x82\x03a\x0F\x12WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x92P\x90P\x81a\x10tV[\x80`\x07\x01TB\x11\x15a\x0F\xBBW`\x04\x81`\x06\x01_\x81\x80Ta\x0F1\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F]\x90a\x1A4V[\x80\x15a\x0F\xA8W\x80`\x1F\x10a\x0F\x7FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xA8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x93P\x93P\x93PPa\x10tV[_B\x82`\x07\x01Ta\x0F\xCC\x91\x90a\x1D>V[`\x05\x83\x01T`\x06\x84\x01\x80T\x92\x93P`\xFF\x90\x91\x16\x91\x83\x90\x82\x90a\x0F\xED\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\x19\x90a\x1A4V[\x80\x15a\x10dW\x80`\x1F\x10a\x10;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10dV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x10\x94W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PPT\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x11\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10\xEFW[PPPPP\x90P\x91\x90PV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x11_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x03\xA0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x03\xA0V[`\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[`\x05` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x12\x07\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x123\x90a\x1A4V[\x80\x15a\x12~W\x80`\x1F\x10a\x12UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12~V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12aW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x16\x80\x83R`\x05\x82R\x91\x85\x90 \x85Q\x93\x84\x01\x86R\x91\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x01\x81\x01\x80T\x93\x94\x91\x93\x91\x83\x01\x91a\x12\xEA\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x16\x90a\x1A4V[\x80\x15a\x13aW\x80`\x1F\x10a\x138Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13aV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x04` R`@\x81 \x90[\x81T\x81\x10\x15a\x14+W\x82\x82\x82\x81T\x81\x10a\x13\xA6Wa\x13\xA6a\x1DQV[\x90_R` _ \x01T\x03a\x14#W\x81T\x82\x90a\x13\xC4\x90`\x01\x90a\x1D>V[\x81T\x81\x10a\x13\xD4Wa\x13\xD4a\x1DQV[\x90_R` _ \x01T\x82\x82\x81T\x81\x10a\x13\xEFWa\x13\xEFa\x1DQV[\x90_R` _ \x01\x81\x90UP\x81\x80T\x80a\x14\x0BWa\x14\x0Ba\x1DeV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90Ua\x14+V[`\x01\x01a\x13\x8AV[PPPPV[P\x80Ta\x14=\x90a\x1A4V[_\x82U\x80`\x1F\x10a\x14LWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x14h\x91\x90a\x14kV[PV[[\x80\x82\x11\x15a\x14\x7FW_\x81U`\x01\x01a\x14lV[P\x90V[__\x83`\x1F\x84\x01\x12a\x14\x93W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA9W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14\xC0W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\x14\xDAW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xEFW__\xFD[a\x14\xFB\x87\x82\x88\x01a\x14\x83V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x19W__\xFD[a\x15%\x87\x82\x88\x01a\x14\x83V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x15AW__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x15XW__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14hW__\xFD[\x805a\x15~\x81a\x15_V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a\x15\xB0Wa\x15\xB0a\x15\x83V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x15\xDEWa\x15\xDEa\x15\x83V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x15\xF5W__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x16\x1BW__\xFD[a\x15X\x83\x835` \x85\x01a\x15\x97V[\x805`\x02\x81\x10a\x15~W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\x16SW__\xFD[a\x16\\\x8Ca\x15sV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16}W__\xFD[a\x16\x89\x8E\x82\x8F\x01a\x16\x0CV[\x99PPa\x16\x98``\x8D\x01a\x16*V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\x16\xBB`\xE0\x8D\x01a\x15sV[\x93Pa\x16\xCAa\x01\0\x8D\x01a\x15sV[\x92Pa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE5W__\xFD[a\x16\xF1\x8E\x82\x8F\x01a\x16\x0CV[\x92PPa\x17\x01a\x01@\x8D\x01a\x15sV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\x17#W__\xFD[P5\x91\x90PV[_[\x83\x81\x10\x15a\x17DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x17,V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x17c\x81` \x86\x01` \x86\x01a\x17*V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x17\x9BWa\x17\x9Ba\x17wV[\x90RV[`\x05\x81\x10a\x17\x9BWa\x17\x9Ba\x17wV[`\x01\x80`\xA0\x1B\x03\x8A\x16\x81R\x88` \x82\x01Ra\x01 `@\x82\x01R_a\x17\xD7a\x01 \x83\x01\x8Aa\x17LV[a\x17\xE4``\x84\x01\x8Aa\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x84\x01R`\xA0\x83\x01\x87\x90Ra\x18\x07`\xC0\x84\x01\x87a\x17\x9FV[\x82\x81\x03`\xE0\x84\x01Ra\x18\x19\x81\x86a\x17LV[\x91PP\x82a\x01\0\x83\x01R\x9A\x99PPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a\x18BW__\xFD[\x825a\x18M\x81a\x15_V[\x94` \x93\x90\x93\x015\x93PPPV[\x805\x80\x15\x15\x81\x14a\x15~W__\xFD[__`@\x83\x85\x03\x12\x15a\x18{W__\xFD[\x825\x91Pa\x18\x8B` \x84\x01a\x18[V[\x90P\x92P\x92\x90PV[a\x18\x9E\x81\x85a\x17\x9FV[``` \x82\x01R_a\x18\xB3``\x83\x01\x85a\x17LV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x18\xD3W__\xFD[\x815a\x15X\x81a\x15_V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x19\x15W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18\xF7V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x190W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x15XW__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x19i\x90\x83\x01\x84a\x17LV[\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x19i`\x80\x84\x01\x82a\x17LV[___``\x84\x86\x03\x12\x15a\x19\xC2W__\xFD[\x835\x92Pa\x19\xD2` \x85\x01a\x18[V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xECW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x19\xFCW__\xFD[a\x1A\x0B\x86\x825` \x84\x01a\x15\x97V[\x91PP\x92P\x92P\x92V[a\x1A\x1F\x81\x84a\x17\x9FV[`@` \x82\x01R_a\x19i`@\x83\x01\x84a\x17LV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1AHW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AfWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x1A\xB3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1A\x91WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A\xB0W_\x81U`\x01\x01a\x1A\x9DV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xD1Wa\x1A\xD1a\x15\x83V[a\x1A\xE5\x81a\x1A\xDF\x84Ta\x1A4V[\x84a\x1AlV[` `\x1F\x82\x11`\x01\x81\x14a\x1B\x17W_\x83\x15a\x1B\0WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1A\xB0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x1BFW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1B&V[P\x84\x82\x10\x15a\x1BcW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x87``\x1B\x16\x81R\x85`\x14\x82\x01R_\x85Qa\x1B\xCE\x81`4\x85\x01` \x8A\x01a\x17*V[\x82\x01`\x02\x86\x10a\x1B\xE0Wa\x1B\xE0a\x17wV[`\xF8\x95\x90\x95\x1B`4\x86\x01RPP``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`5\x83\x01R`I\x82\x01R`i\x01\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[`\xE0\x81R`\x03`\xE0\x82\x01Rbpre`\xE8\x1Ba\x01\0\x82\x01R\x86` \x82\x01R`\x01\x80`\xA0\x1B\x03\x86\x16`@\x82\x01R\x84``\x82\x01Ra\x01 `\x80\x82\x01R_a\x1C~a\x01 \x83\x01\x86a\x17LV[\x90Pa\x1C\x8D`\xA0\x83\x01\x85a\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\xC0\x91\x90\x91\x01R\x95\x94PPPPPV[` \x81R_a\x15X` \x83\x01\x84a\x17LV[\x85\x81R`\xA0` \x82\x01R_a\x1C\xD3`\xA0\x83\x01\x87a\x17LV[\x90Pa\x1C\xE2`@\x83\x01\x86a\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16``\x82\x01R`\x80\x01R\x93\x92PPPV[a\x1D\x08\x81\x83a\x17\x9FV[`@` \x82\x01\x81\x90R`\x16\x90\x82\x01RuValidation in progress`P\x1B``\x82\x01R`\x80\x01\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x1F\xE1)Xf\xAF\x94\xA0\xEDh\xB7\xC9A\xD10\xF0\xA8r\xE7\xB7\xDD\x99\xE6\xD7\xEA\xC6\xBD\x08\xD0\xD2\xE8JdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610110575f3560e01c8063932713681161009e5780639e83e3061161006e5780639e83e306146102b9578063c4d66de8146102d9578063c56fb0fd146102ec578063ce289612146102f4578063e328ed7714610315575f5ffd5b80639327136814610241578063944074651461025457806397f5365a1461027657806399d7cf4b14610297575f5ffd5b806342227fa4116100e457806342227fa4146101b557806375f0bb52146101e05780637b4f3373146101f35780638d69e95e1461021b578063913b1fbf1461022e575f5ffd5b806273e1d71461011457806301ffc9a714610129578063158ef93e14610162578063186f035414610176575b5f5ffd5b6101276101223660046114c7565b610335565b005b61014d610137366004611531565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b60015461014d90600160a01b900460ff1681565b61019d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610159565b6007546101c8906001600160401b031681565b6040516001600160401b039091168152602001610159565b6101276101ee366004611638565b610513565b610206610201366004611713565b610a4d565b604051610159999897969594939291906117af565b60015461019d906001600160a01b031681565b6101c861023c366004611831565b610bbe565b61012761024f36600461186a565b610c05565b610267610262366004611713565b610eda565b60405161015993929190611894565b610289610284366004611831565b61107b565b604051908152602001610159565b61014d6102a5366004611713565b60026020525f908152604090205460ff1681565b6102cc6102c73660046118c3565b6110a6565b60405161015991906118de565b6101276102e73660046118c3565b61110f565b6102895f5481565b610307610302366004611920565b6111dc565b604051610159929190611946565b610328610323366004611920565b611288565b6040516101599190611971565b6001546001600160a01b031633146103a95760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f80806103b8868801886119b0565b5f8381526003602052604081206004810154949750929550909350909190036103f4576040516331fb878f60e01b815260040160405180910390fd5b80600701544211156104665760058101805460ff1916600490811790915560405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd99161044591908690611a15565b60405180910390a26040516338e5e54b60e21b815260040160405180910390fd5b82610472576003610475565b60025b60058201805460ff1916600183600481111561049357610493611777565b0217905550600681016104a68382611ab8565b5082156104c6575f848152600260205260409020805460ff191660011790555b600581015460405185917f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd9916105019160ff16908690611a15565b60405180910390a25050505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461057a5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016103a0565b5f6040518061012001604052808d6001600160a01b031681526020018c81526020018b81526020018a60018111156105b4576105b4611777565b81526001600160a01b0384166020820152426040820152606001600181526020016040518060400160405280601681526020017556616c69646174696f6e20696e2070726f677265737360501b81525081526020015f54426106169190611b86565b81525090505f815f015182602001518360400151846060015185608001518660a0015160405160200161064e96959493929190611b9f565b60408051601f1981840301815291815281516020928301205f818152600290935291205490915060ff1615610684575050610a40565b5f81815260036020526040902060040154158015906106b257505f8181526003602052604090206007015442115b156106d0576040516338e5e54b60e21b815260040160405180910390fd5b5f81815260036020908152604091829020845181546001600160a01b0319166001600160a01b0390911617815590840151600182015590830151839190600282019061071c9082611ab8565b50606082015160038201805460ff19166001838181111561073f5761073f611777565b021790555060808201518160030160016101000a8154816001600160a01b0302191690836001600160a01b0316021790555060a0820151816004015560c0820151816005015f6101000a81548160ff021916908360048111156107a4576107a4611777565b021790555060e082015160068201906107bd9082611ab8565b5061010091909101516007918201556001600160a01b0384165f908152600460209081526040822080546001818101835591845291832090910184905591549091610811916001600160401b031690611c16565b6007805467ffffffffffffffff19166001600160401b0383161790558351602080860151604080880151606089015160808a015192519697505f9661085c968a969095949101611c35565b60408051808303601f19018152828201825260808701516001600160a01b03908116845260208085018381526001600160401b0388165f9081526005909252939020845181546001600160a01b0319169216919091178155915190935060018201906108c89082611ab8565b5050506001600160a01b038086165f908152600660209081526040808320805460018101825590845292829020600484040180546001600160401b0380891660086003909716969096026101000a868102910219909116179055805160608101825292835260808801519093168282015281830184905291517f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e689261096e929101611971565b60408051601f198184030181529082905261098891611ca9565b60405180910390a1835f01516001600160a01b0316837f72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb86602001518760400151886060015189608001515f546040516109e6959493929190611cbb565b60405180910390a3827f5c52b920fc5d0ac45838c205ad92650612c5ce5bf8136af02fa69466cc3a1fd96001604051610a1f9190611cfe565b60405180910390a26040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b60036020525f90815260409020805460018201546002830180546001600160a01b03909316939192610a7e90611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610aaa90611a34565b8015610af55780601f10610acc57610100808354040283529160200191610af5565b820191905f5260205f20905b815481529060010190602001808311610ad857829003601f168201915b50505050600383015460048401546005850154600686018054959660ff808616976101009096046001600160a01b0316965093949390921692610b3790611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610b6390611a34565b8015610bae5780601f10610b8557610100808354040283529160200191610bae565b820191905f5260205f20905b815481529060010190602001808311610b9157829003601f168201915b5050505050908060070154905089565b6006602052815f5260405f208181548110610bd7575f80fd5b905f5260205f209060049182820401919006600802915091509054906101000a90046001600160401b031681565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610c6c5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b60448201526064016103a0565b5f82815260036020819052604090912090810154610c989061010090046001600160a01b031684611371565b600754610caf906001600160401b03166001611c16565b6007805467ffffffffffffffff19166001600160401b039290921691821790556040805160606020820181905260046080830152631c1bdcdd60e21b60a0830152918101869052841515918101919091525f9060c00160408051808303601f190181528282018252600386015461010090046001600160a01b03908116845260208085018381526001600160401b0388165f9081526005909252939020845181546001600160a01b031916921691909117815591519093508291906001820190610d799082611ab8565b5050506003808501546001600160a01b036101009182900481165f908152600660209081526040808320805460018101825590845292829020600484040180546001600160401b03808c166008969099169590950290960a8781029402199095169290921790935580516060810182529384528451909116838301528382015183820152517f86eacd23610d81706516de1ed0476c87772fdf939c7c771fbbd7f0230d619e6891610e2c91849101611971565b60408051601f1981840301815290829052610e4691611ca9565b60405180910390a15f87815260036020526040812080546001600160a01b03191681556001810182905590610e7e6002830182611431565b6003820180546001600160a81b03191690555f6004830181905560058301805460ff19169055610eb2906006840190611431565b505f6007919091018190559687525050600260205250506040909220805460ff191690555050565b5f818152600360205260408120600481015460609183918203610f1257505060408051602081019091525f8082529250905081611074565b8060070154421115610fbb576004816006015f818054610f3190611a34565b80601f0160208091040260200160405190810160405280929190818152602001828054610f5d90611a34565b8015610fa85780601f10610f7f57610100808354040283529160200191610fa8565b820191905f5260205f20905b815481529060010190602001808311610f8b57829003601f168201915b5050505050915093509350935050611074565b5f428260070154610fcc9190611d3e565b600583015460068401805492935060ff9091169183908290610fed90611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461101990611a34565b80156110645780601f1061103b57610100808354040283529160200191611064565b820191905f5260205f20905b81548152906001019060200180831161104757829003601f168201915b5050505050915094509450945050505b9193909250565b6004602052815f5260405f208181548110611094575f80fd5b905f5260205f20015f91509150505481565b6001600160a01b0381165f9081526004602090815260409182902080548351818402810184019094528084526060939283018282801561110357602002820191905f5260205f20905b8154815260200190600101908083116110ef575b50505050509050919050565b600154600160a01b900460ff161561115f5760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b60448201526064016103a0565b6001600160a01b0381166111b55760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f7669646572206164647265737360448201526064016103a0565b600180546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b60056020525f9081526040902080546001820180546001600160a01b03909216929161120790611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461123390611a34565b801561127e5780601f106112555761010080835404028352916020019161127e565b820191905f5260205f20905b81548152906001019060200180831161126157829003601f168201915b5050505050905082565b60408051606080820183525f80835260208084018290528385018390526001600160401b03861680835260058252918590208551938401865291835281546001600160a01b03169083015260018101805493949193918301916112ea90611a34565b80601f016020809104026020016040519081016040528092919081815260200182805461131690611a34565b80156113615780601f1061133857610100808354040283529160200191611361565b820191905f5260205f20905b81548152906001019060200180831161134457829003601f168201915b5050505050815250915050919050565b6001600160a01b0382165f908152600460205260408120905b815481101561142b57828282815481106113a6576113a6611d51565b905f5260205f2001540361142357815482906113c490600190611d3e565b815481106113d4576113d4611d51565b905f5260205f2001548282815481106113ef576113ef611d51565b905f5260205f2001819055508180548061140b5761140b611d65565b600190038181905f5260205f20015f9055905561142b565b60010161138a565b50505050565b50805461143d90611a34565b5f825580601f1061144c575050565b601f0160209004905f5260205f2090810190611468919061146b565b50565b5b8082111561147f575f815560010161146c565b5090565b5f5f83601f840112611493575f5ffd5b5081356001600160401b038111156114a9575f5ffd5b6020830191508360208285010111156114c0575f5ffd5b9250929050565b5f5f5f5f604085870312156114da575f5ffd5b84356001600160401b038111156114ef575f5ffd5b6114fb87828801611483565b90955093505060208501356001600160401b03811115611519575f5ffd5b61152587828801611483565b95989497509550505050565b5f60208284031215611541575f5ffd5b81356001600160e01b031981168114611558575f5ffd5b9392505050565b6001600160a01b0381168114611468575f5ffd5b803561157e8161155f565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f5f6001600160401b038411156115b0576115b0611583565b50604051601f19601f85018116603f011681018181106001600160401b03821117156115de576115de611583565b6040528381529050808284018510156115f5575f5ffd5b838360208301375f60208583010152509392505050565b5f82601f83011261161b575f5ffd5b61155883833560208501611597565b80356002811061157e575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e031215611653575f5ffd5b61165c8c611573565b9a5060208c0135995060408c01356001600160401b0381111561167d575f5ffd5b6116898e828f0161160c565b99505061169860608d0161162a565b975060808c0135965060a08c0135955060c08c013594506116bb60e08d01611573565b93506116ca6101008d01611573565b92506101208c01356001600160401b038111156116e5575f5ffd5b6116f18e828f0161160c565b9250506117016101408d01611573565b90509295989b509295989b9093969950565b5f60208284031215611723575f5ffd5b5035919050565b5f5b8381101561174457818101518382015260200161172c565b50505f910152565b5f815180845261176381602086016020860161172a565b601f01601f19169290920160200192915050565b634e487b7160e01b5f52602160045260245ffd5b6002811061179b5761179b611777565b9052565b6005811061179b5761179b611777565b60018060a01b038a16815288602082015261012060408201525f6117d761012083018a61174c565b6117e4606084018a61178b565b6001600160a01b038816608084015260a0830187905261180760c084018761179f565b82810360e0840152611819818661174c565b915050826101008301529a9950505050505050505050565b5f5f60408385031215611842575f5ffd5b823561184d8161155f565b946020939093013593505050565b8035801515811461157e575f5ffd5b5f5f6040838503121561187b575f5ffd5b8235915061188b6020840161185b565b90509250929050565b61189e818561179f565b606060208201525f6118b3606083018561174c565b9050826040830152949350505050565b5f602082840312156118d3575f5ffd5b81356115588161155f565b602080825282518282018190525f918401906040840190835b818110156119155783518352602093840193909201916001016118f7565b509095945050505050565b5f60208284031215611930575f5ffd5b81356001600160401b0381168114611558575f5ffd5b6001600160a01b03831681526040602082018190525f906119699083018461174c565b949350505050565b602081526001600160401b03825116602082015260018060a01b0360208301511660408201525f6040830151606080840152611969608084018261174c565b5f5f5f606084860312156119c2575f5ffd5b833592506119d26020850161185b565b915060408401356001600160401b038111156119ec575f5ffd5b8401601f810186136119fc575f5ffd5b611a0b86823560208401611597565b9150509250925092565b611a1f818461179f565b604060208201525f611969604083018461174c565b600181811c90821680611a4857607f821691505b602082108103611a6657634e487b7160e01b5f52602260045260245ffd5b50919050565b601f821115611ab357805f5260205f20601f840160051c81016020851015611a915750805b601f840160051c820191505b81811015611ab0575f8155600101611a9d565b50505b505050565b81516001600160401b03811115611ad157611ad1611583565b611ae581611adf8454611a34565b84611a6c565b6020601f821160018114611b17575f8315611b005750848201515b5f19600385901b1c1916600184901b178455611ab0565b5f84815260208120601f198516915b82811015611b465787850151825560209485019460019092019101611b26565b5084821015611b6357868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b634e487b7160e01b5f52601160045260245ffd5b80820180821115611b9957611b99611b72565b92915050565b6bffffffffffffffffffffffff198760601b1681528560148201525f8551611bce816034850160208a0161172a565b820160028610611be057611be0611777565b60f89590951b6034860152505060609190911b6bffffffffffffffffffffffff1916603583015260498201526069019392505050565b6001600160401b038181168382160190811115611b9957611b99611b72565b60e08152600360e08201526270726560e81b61010082015286602082015260018060a01b038616604082015284606082015261012060808201525f611c7e61012083018661174c565b9050611c8d60a083018561178b565b6001600160a01b039290921660c0919091015295945050505050565b602081525f611558602083018461174c565b85815260a060208201525f611cd360a083018761174c565b9050611ce2604083018661178b565b6001600160a01b03939093166060820152608001529392505050565b611d08818361179f565b6040602082018190526016908201527556616c69646174696f6e20696e2070726f677265737360501b6060820152608001919050565b81810381811115611b9957611b99611b72565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffdfea26469706673582212201fe1295866af94a0ed68b7c941d130f0a872e7b7dd99e6d7eac6bd08d0d2e84a64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x10W_5`\xE0\x1C\x80c\x93'\x13h\x11a\0\x9EW\x80c\x9E\x83\xE3\x06\x11a\0nW\x80c\x9E\x83\xE3\x06\x14a\x02\xB9W\x80c\xC4\xD6m\xE8\x14a\x02\xD9W\x80c\xC5o\xB0\xFD\x14a\x02\xECW\x80c\xCE(\x96\x12\x14a\x02\xF4W\x80c\xE3(\xEDw\x14a\x03\x15W__\xFD[\x80c\x93'\x13h\x14a\x02AW\x80c\x94@te\x14a\x02TW\x80c\x97\xF56Z\x14a\x02vW\x80c\x99\xD7\xCFK\x14a\x02\x97W__\xFD[\x80cB\"\x7F\xA4\x11a\0\xE4W\x80cB\"\x7F\xA4\x14a\x01\xB5W\x80cu\xF0\xBBR\x14a\x01\xE0W\x80c{O3s\x14a\x01\xF3W\x80c\x8Di\xE9^\x14a\x02\x1BW\x80c\x91;\x1F\xBF\x14a\x02.W__\xFD[\x80bs\xE1\xD7\x14a\x01\x14W\x80c\x01\xFF\xC9\xA7\x14a\x01)W\x80c\x15\x8E\xF9>\x14a\x01bW\x80c\x18o\x03T\x14a\x01vW[__\xFD[a\x01'a\x01\"6`\x04a\x14\xC7V[a\x035V[\0[a\x01Ma\x0176`\x04a\x151V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\x01Ta\x01M\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x01\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[`\x07Ta\x01\xC8\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01YV[a\x01'a\x01\xEE6`\x04a\x168V[a\x05\x13V[a\x02\x06a\x02\x016`\x04a\x17\x13V[a\nMV[`@Qa\x01Y\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x17\xAFV[`\x01Ta\x01\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01\xC8a\x02<6`\x04a\x181V[a\x0B\xBEV[a\x01'a\x02O6`\x04a\x18jV[a\x0C\x05V[a\x02ga\x02b6`\x04a\x17\x13V[a\x0E\xDAV[`@Qa\x01Y\x93\x92\x91\x90a\x18\x94V[a\x02\x89a\x02\x846`\x04a\x181V[a\x10{V[`@Q\x90\x81R` \x01a\x01YV[a\x01Ma\x02\xA56`\x04a\x17\x13V[`\x02` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xCCa\x02\xC76`\x04a\x18\xC3V[a\x10\xA6V[`@Qa\x01Y\x91\x90a\x18\xDEV[a\x01'a\x02\xE76`\x04a\x18\xC3V[a\x11\x0FV[a\x02\x89_T\x81V[a\x03\x07a\x03\x026`\x04a\x19 V[a\x11\xDCV[`@Qa\x01Y\x92\x91\x90a\x19FV[a\x03(a\x03#6`\x04a\x19 V[a\x12\x88V[`@Qa\x01Y\x91\x90a\x19qV[`\x01T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80\x80a\x03\xB8\x86\x88\x01\x88a\x19\xB0V[_\x83\x81R`\x03` R`@\x81 `\x04\x81\x01T\x94\x97P\x92\x95P\x90\x93P\x90\x91\x90\x03a\x03\xF4W`@Qc1\xFB\x87\x8F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x07\x01TB\x11\x15a\x04fW`\x05\x81\x01\x80T`\xFF\x19\x16`\x04\x90\x81\x17\x90\x91U`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x04E\x91\x90\x86\x90a\x1A\x15V[`@Q\x80\x91\x03\x90\xA2`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x04rW`\x03a\x04uV[`\x02[`\x05\x82\x01\x80T`\xFF\x19\x16`\x01\x83`\x04\x81\x11\x15a\x04\x93Wa\x04\x93a\x17wV[\x02\x17\x90UP`\x06\x81\x01a\x04\xA6\x83\x82a\x1A\xB8V[P\x82\x15a\x04\xC6W_\x84\x81R`\x02` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[`\x05\x81\x01T`@Q\x85\x91\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9\x91a\x05\x01\x91`\xFF\x16\x90\x86\x90a\x1A\x15V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x03\xA0V[_`@Q\x80a\x01 \x01`@R\x80\x8D`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x8A`\x01\x81\x11\x15a\x05\xB4Wa\x05\xB4a\x17wV[\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16` \x82\x01RB`@\x82\x01R``\x01`\x01\x81R` \x01`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uValidation in progress`P\x1B\x81RP\x81R` \x01_TBa\x06\x16\x91\x90a\x1B\x86V[\x81RP\x90P_\x81_\x01Q\x82` \x01Q\x83`@\x01Q\x84``\x01Q\x85`\x80\x01Q\x86`\xA0\x01Q`@Q` \x01a\x06N\x96\x95\x94\x93\x92\x91\x90a\x1B\x9FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x02\x90\x93R\x91 T\x90\x91P`\xFF\x16\x15a\x06\x84WPPa\n@V[_\x81\x81R`\x03` R`@\x90 `\x04\x01T\x15\x80\x15\x90a\x06\xB2WP_\x81\x81R`\x03` R`@\x90 `\x07\x01TB\x11[\x15a\x06\xD0W`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x81U\x90\x84\x01Q`\x01\x82\x01U\x90\x83\x01Q\x83\x91\x90`\x02\x82\x01\x90a\x07\x1C\x90\x82a\x1A\xB8V[P``\x82\x01Q`\x03\x82\x01\x80T`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x07?Wa\x07?a\x17wV[\x02\x17\x90UP`\x80\x82\x01Q\x81`\x03\x01`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`\xA0\x82\x01Q\x81`\x04\x01U`\xC0\x82\x01Q\x81`\x05\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x04\x81\x11\x15a\x07\xA4Wa\x07\xA4a\x17wV[\x02\x17\x90UP`\xE0\x82\x01Q`\x06\x82\x01\x90a\x07\xBD\x90\x82a\x1A\xB8V[Pa\x01\0\x91\x90\x91\x01Q`\x07\x91\x82\x01U`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x82 \x80T`\x01\x81\x81\x01\x83U\x91\x84R\x91\x83 \x90\x91\x01\x84\x90U\x91T\x90\x91a\x08\x11\x91`\x01`\x01`@\x1B\x03\x16\x90a\x1C\x16V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17\x90U\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q``\x89\x01Q`\x80\x8A\x01Q\x92Q\x96\x97P_\x96a\x08\\\x96\x8A\x96\x90\x95\x94\x91\x01a\x1C5V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R`\x80\x87\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R` \x80\x85\x01\x83\x81R`\x01`\x01`@\x1B\x03\x88\x16_\x90\x81R`\x05\x90\x92R\x93\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x81U\x91Q\x90\x93P`\x01\x82\x01\x90a\x08\xC8\x90\x82a\x1A\xB8V[PPP`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 `\x04\x84\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x89\x16`\x08`\x03\x90\x97\x16\x96\x90\x96\x02a\x01\0\n\x86\x81\x02\x91\x02\x19\x90\x91\x16\x17\x90U\x80Q``\x81\x01\x82R\x92\x83R`\x80\x88\x01Q\x90\x93\x16\x82\x82\x01R\x81\x83\x01\x84\x90R\x91Q\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x92a\tn\x92\x91\x01a\x19qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\x88\x91a\x1C\xA9V[`@Q\x80\x91\x03\x90\xA1\x83_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x7Fr\xB8\xBE\xAA+\x16\xEF\xC2\x0F\xF7\xAE\xA9B\xA1\"\xF7\xB7\x81\x19rO\xAB\xBD\x80j\xCDd\xD7\x97\x89T\xCB\x86` \x01Q\x87`@\x01Q\x88``\x01Q\x89`\x80\x01Q_T`@Qa\t\xE6\x95\x94\x93\x92\x91\x90a\x1C\xBBV[`@Q\x80\x91\x03\x90\xA3\x82\x7F\\R\xB9 \xFC]\n\xC4X8\xC2\x05\xAD\x92e\x06\x12\xC5\xCE[\xF8\x13j\xF0/\xA6\x94f\xCC:\x1F\xD9`\x01`@Qa\n\x1F\x91\x90a\x1C\xFEV[`@Q\x80\x91\x03\x90\xA2`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\x03` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x93\x91\x92a\n~\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\xAA\x90a\x1A4V[\x80\x15a\n\xF5W\x80`\x1F\x10a\n\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x84\x01T`\x05\x85\x01T`\x06\x86\x01\x80T\x95\x96`\xFF\x80\x86\x16\x97a\x01\0\x90\x96\x04`\x01`\x01`\xA0\x1B\x03\x16\x96P\x93\x94\x93\x90\x92\x16\x92a\x0B7\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Bc\x90a\x1A4V[\x80\x15a\x0B\xAEW\x80`\x1F\x10a\x0B\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x07\x01T\x90P\x89V[`\x06` R\x81_R`@_ \x81\x81T\x81\x10a\x0B\xD7W_\x80\xFD[\x90_R` _ \x90`\x04\x91\x82\x82\x04\x01\x91\x90\x06`\x08\x02\x91P\x91P\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0ClW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x03\xA0V[_\x82\x81R`\x03` \x81\x90R`@\x90\x91 \x90\x81\x01Ta\x0C\x98\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x84a\x13qV[`\x07Ta\x0C\xAF\x90`\x01`\x01`@\x1B\x03\x16`\x01a\x1C\x16V[`\x07\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@\x80Q``` \x82\x01\x81\x90R`\x04`\x80\x83\x01Rc\x1C\x1B\xDC\xDD`\xE2\x1B`\xA0\x83\x01R\x91\x81\x01\x86\x90R\x84\x15\x15\x91\x81\x01\x91\x90\x91R_\x90`\xC0\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R`\x03\x86\x01Ta\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R` \x80\x85\x01\x83\x81R`\x01`\x01`@\x1B\x03\x88\x16_\x90\x81R`\x05\x90\x92R\x93\x90 \x84Q\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x81U\x91Q\x90\x93P\x82\x91\x90`\x01\x82\x01\x90a\ry\x90\x82a\x1A\xB8V[PPP`\x03\x80\x85\x01T`\x01`\x01`\xA0\x1B\x03a\x01\0\x91\x82\x90\x04\x81\x16_\x90\x81R`\x06` \x90\x81R`@\x80\x83 \x80T`\x01\x81\x01\x82U\x90\x84R\x92\x82\x90 `\x04\x84\x04\x01\x80T`\x01`\x01`@\x1B\x03\x80\x8C\x16`\x08\x96\x90\x99\x16\x95\x90\x95\x02\x90\x96\n\x87\x81\x02\x94\x02\x19\x90\x95\x16\x92\x90\x92\x17\x90\x93U\x80Q``\x81\x01\x82R\x93\x84R\x84Q\x90\x91\x16\x83\x83\x01R\x83\x82\x01Q\x83\x82\x01RQ\x7F\x86\xEA\xCD#a\r\x81pe\x16\xDE\x1E\xD0Gl\x87w/\xDF\x93\x9C|w\x1F\xBB\xD7\xF0#\ra\x9Eh\x91a\x0E,\x91\x84\x91\x01a\x19qV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0EF\x91a\x1C\xA9V[`@Q\x80\x91\x03\x90\xA1_\x87\x81R`\x03` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x81U`\x01\x81\x01\x82\x90U\x90a\x0E~`\x02\x83\x01\x82a\x141V[`\x03\x82\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16\x90U_`\x04\x83\x01\x81\x90U`\x05\x83\x01\x80T`\xFF\x19\x16\x90Ua\x0E\xB2\x90`\x06\x84\x01\x90a\x141V[P_`\x07\x91\x90\x91\x01\x81\x90U\x96\x87RPP`\x02` RPP`@\x90\x92 \x80T`\xFF\x19\x16\x90UPPV[_\x81\x81R`\x03` R`@\x81 `\x04\x81\x01T``\x91\x83\x91\x82\x03a\x0F\x12WPP`@\x80Q` \x81\x01\x90\x91R_\x80\x82R\x92P\x90P\x81a\x10tV[\x80`\x07\x01TB\x11\x15a\x0F\xBBW`\x04\x81`\x06\x01_\x81\x80Ta\x0F1\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F]\x90a\x1A4V[\x80\x15a\x0F\xA8W\x80`\x1F\x10a\x0F\x7FWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0F\xA8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0F\x8BW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x93P\x93P\x93PPa\x10tV[_B\x82`\x07\x01Ta\x0F\xCC\x91\x90a\x1D>V[`\x05\x83\x01T`\x06\x84\x01\x80T\x92\x93P`\xFF\x90\x91\x16\x91\x83\x90\x82\x90a\x0F\xED\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\x19\x90a\x1A4V[\x80\x15a\x10dW\x80`\x1F\x10a\x10;Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10dV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10GW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91P\x94P\x94P\x94PPP[\x91\x93\x90\x92PV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x10\x94W_\x80\xFD[\x90_R` _ \x01_\x91P\x91PPT\x81V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x04` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R``\x93\x92\x83\x01\x82\x82\x80\x15a\x11\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10\xEFW[PPPPP\x90P\x91\x90PV[`\x01T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x11_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x03\xA0V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x11\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x03\xA0V[`\x01\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[`\x05` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92\x91a\x12\x07\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x123\x90a\x1A4V[\x80\x15a\x12~W\x80`\x1F\x10a\x12UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12~V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12aW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x83\x90R`\x01`\x01`@\x1B\x03\x86\x16\x80\x83R`\x05\x82R\x91\x85\x90 \x85Q\x93\x84\x01\x86R\x91\x83R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x90\x83\x01R`\x01\x81\x01\x80T\x93\x94\x91\x93\x91\x83\x01\x91a\x12\xEA\x90a\x1A4V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x16\x90a\x1A4V[\x80\x15a\x13aW\x80`\x1F\x10a\x138Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13aV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13DW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RP\x91PP\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x04` R`@\x81 \x90[\x81T\x81\x10\x15a\x14+W\x82\x82\x82\x81T\x81\x10a\x13\xA6Wa\x13\xA6a\x1DQV[\x90_R` _ \x01T\x03a\x14#W\x81T\x82\x90a\x13\xC4\x90`\x01\x90a\x1D>V[\x81T\x81\x10a\x13\xD4Wa\x13\xD4a\x1DQV[\x90_R` _ \x01T\x82\x82\x81T\x81\x10a\x13\xEFWa\x13\xEFa\x1DQV[\x90_R` _ \x01\x81\x90UP\x81\x80T\x80a\x14\x0BWa\x14\x0Ba\x1DeV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90Ua\x14+V[`\x01\x01a\x13\x8AV[PPPPV[P\x80Ta\x14=\x90a\x1A4V[_\x82U\x80`\x1F\x10a\x14LWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x14h\x91\x90a\x14kV[PV[[\x80\x82\x11\x15a\x14\x7FW_\x81U`\x01\x01a\x14lV[P\x90V[__\x83`\x1F\x84\x01\x12a\x14\x93W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xA9W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x14\xC0W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\x14\xDAW__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xEFW__\xFD[a\x14\xFB\x87\x82\x88\x01a\x14\x83V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x19W__\xFD[a\x15%\x87\x82\x88\x01a\x14\x83V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x15AW__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x15XW__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x14hW__\xFD[\x805a\x15~\x81a\x15_V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`\x01`\x01`@\x1B\x03\x84\x11\x15a\x15\xB0Wa\x15\xB0a\x15\x83V[P`@Q`\x1F\x19`\x1F\x85\x01\x81\x16`?\x01\x16\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x15\xDEWa\x15\xDEa\x15\x83V[`@R\x83\x81R\x90P\x80\x82\x84\x01\x85\x10\x15a\x15\xF5W__\xFD[\x83\x83` \x83\x017_` \x85\x83\x01\x01RP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\x16\x1BW__\xFD[a\x15X\x83\x835` \x85\x01a\x15\x97V[\x805`\x02\x81\x10a\x15~W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\x16SW__\xFD[a\x16\\\x8Ca\x15sV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16}W__\xFD[a\x16\x89\x8E\x82\x8F\x01a\x16\x0CV[\x99PPa\x16\x98``\x8D\x01a\x16*V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\x16\xBB`\xE0\x8D\x01a\x15sV[\x93Pa\x16\xCAa\x01\0\x8D\x01a\x15sV[\x92Pa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xE5W__\xFD[a\x16\xF1\x8E\x82\x8F\x01a\x16\x0CV[\x92PPa\x17\x01a\x01@\x8D\x01a\x15sV[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\x17#W__\xFD[P5\x91\x90PV[_[\x83\x81\x10\x15a\x17DW\x81\x81\x01Q\x83\x82\x01R` \x01a\x17,V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x17c\x81` \x86\x01` \x86\x01a\x17*V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a\x17\x9BWa\x17\x9Ba\x17wV[\x90RV[`\x05\x81\x10a\x17\x9BWa\x17\x9Ba\x17wV[`\x01\x80`\xA0\x1B\x03\x8A\x16\x81R\x88` \x82\x01Ra\x01 `@\x82\x01R_a\x17\xD7a\x01 \x83\x01\x8Aa\x17LV[a\x17\xE4``\x84\x01\x8Aa\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x84\x01R`\xA0\x83\x01\x87\x90Ra\x18\x07`\xC0\x84\x01\x87a\x17\x9FV[\x82\x81\x03`\xE0\x84\x01Ra\x18\x19\x81\x86a\x17LV[\x91PP\x82a\x01\0\x83\x01R\x9A\x99PPPPPPPPPPV[__`@\x83\x85\x03\x12\x15a\x18BW__\xFD[\x825a\x18M\x81a\x15_V[\x94` \x93\x90\x93\x015\x93PPPV[\x805\x80\x15\x15\x81\x14a\x15~W__\xFD[__`@\x83\x85\x03\x12\x15a\x18{W__\xFD[\x825\x91Pa\x18\x8B` \x84\x01a\x18[V[\x90P\x92P\x92\x90PV[a\x18\x9E\x81\x85a\x17\x9FV[``` \x82\x01R_a\x18\xB3``\x83\x01\x85a\x17LV[\x90P\x82`@\x83\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x18\xD3W__\xFD[\x815a\x15X\x81a\x15_V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x19\x15W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18\xF7V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x190W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x15XW__\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x19i\x90\x83\x01\x84a\x17LV[\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\x01\x80`\xA0\x1B\x03` \x83\x01Q\x16`@\x82\x01R_`@\x83\x01Q``\x80\x84\x01Ra\x19i`\x80\x84\x01\x82a\x17LV[___``\x84\x86\x03\x12\x15a\x19\xC2W__\xFD[\x835\x92Pa\x19\xD2` \x85\x01a\x18[V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xECW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x19\xFCW__\xFD[a\x1A\x0B\x86\x825` \x84\x01a\x15\x97V[\x91PP\x92P\x92P\x92V[a\x1A\x1F\x81\x84a\x17\x9FV[`@` \x82\x01R_a\x19i`@\x83\x01\x84a\x17LV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1AHW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1AfWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x1A\xB3W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1A\x91WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x1A\xB0W_\x81U`\x01\x01a\x1A\x9DV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xD1Wa\x1A\xD1a\x15\x83V[a\x1A\xE5\x81a\x1A\xDF\x84Ta\x1A4V[\x84a\x1AlV[` `\x1F\x82\x11`\x01\x81\x14a\x1B\x17W_\x83\x15a\x1B\0WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x1A\xB0V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x1BFW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1B&V[P\x84\x82\x10\x15a\x1BcW\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[\x92\x91PPV[k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x87``\x1B\x16\x81R\x85`\x14\x82\x01R_\x85Qa\x1B\xCE\x81`4\x85\x01` \x8A\x01a\x17*V[\x82\x01`\x02\x86\x10a\x1B\xE0Wa\x1B\xE0a\x17wV[`\xF8\x95\x90\x95\x1B`4\x86\x01RPP``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`5\x83\x01R`I\x82\x01R`i\x01\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[`\xE0\x81R`\x03`\xE0\x82\x01Rbpre`\xE8\x1Ba\x01\0\x82\x01R\x86` \x82\x01R`\x01\x80`\xA0\x1B\x03\x86\x16`@\x82\x01R\x84``\x82\x01Ra\x01 `\x80\x82\x01R_a\x1C~a\x01 \x83\x01\x86a\x17LV[\x90Pa\x1C\x8D`\xA0\x83\x01\x85a\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\xC0\x91\x90\x91\x01R\x95\x94PPPPPV[` \x81R_a\x15X` \x83\x01\x84a\x17LV[\x85\x81R`\xA0` \x82\x01R_a\x1C\xD3`\xA0\x83\x01\x87a\x17LV[\x90Pa\x1C\xE2`@\x83\x01\x86a\x17\x8BV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16``\x82\x01R`\x80\x01R\x93\x92PPPV[a\x1D\x08\x81\x83a\x17\x9FV[`@` \x82\x01\x81\x90R`\x16\x90\x82\x01RuValidation in progress`P\x1B``\x82\x01R`\x80\x01\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x1B\x99Wa\x1B\x99a\x1BrV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x1F\xE1)Xf\xAF\x94\xA0\xEDh\xB7\xC9A\xD10\xF0\xA8r\xE7\xB7\xDD\x99\xE6\xD7\xEA\xC6\xBD\x08\xD0\xD2\xE8JdsolcC\0\x08\x1C\x003",
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
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
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
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `TransactionExpired()` and selector `0xe397952c`.
    ```solidity
    error TransactionExpired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransactionExpired {}
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Event with signature `ValidationRequired(bytes32,address,uint256,bytes,uint8,address,uint256)` and selector `0x72b8beaa2b16efc20ff7aea942a122f7b78119724fabbd806acd64d7978954cb`.
    ```solidity
    event ValidationRequired(bytes32 indexed txHash, address indexed to, uint256 value, bytes data, Enum.Operation operation, address initiator, uint256 estimatedProcessingTime);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "ValidationRequired(bytes32,address,uint256,bytes,uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    114u8, 184u8, 190u8, 170u8, 43u8, 22u8, 239u8, 194u8, 15u8, 247u8, 174u8,
                    169u8, 66u8, 161u8, 34u8, 247u8, 183u8, 129u8, 25u8, 114u8, 79u8, 171u8, 189u8,
                    128u8, 106u8, 205u8, 100u8, 215u8, 151u8, 137u8, 84u8, 203u8,
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
                        &self.value,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(&self.operation),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initiator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ValidationStatusUpdated {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub message: alloy::sol_types::private::String,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidationStatusUpdated {
            type DataTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::String);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ValidationStatusUpdated(bytes32,uint8,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    92u8, 82u8, 185u8, 32u8, 252u8, 93u8, 10u8, 196u8, 88u8, 56u8, 194u8, 5u8,
                    173u8, 146u8, 101u8, 6u8, 18u8, 197u8, 206u8, 91u8, 248u8, 19u8, 106u8, 240u8,
                    47u8, 166u8, 148u8, 102u8, 204u8, 58u8, 31u8, 217u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { txHash: topics.1, status: data.0, message: data.1 }
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
                    <ValidationStatus as alloy_sol_types::SolType>::tokenize(&self.status),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &ValidationStatusUpdated) -> alloy_sol_types::private::LogData {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Bool);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkAfterExecutionCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionCall) -> Self {
                    (value.txHash, value.success)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkAfterExecutionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { txHash: tuple.0, success: tuple.1 }
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
            impl ::core::convert::From<checkAfterExecutionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkAfterExecutionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkAfterExecutionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkAfterExecutionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Bool);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkAfterExecutionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkTransactionCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkTransactionCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkTransactionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkTransactionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkTransactionReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkTransactionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.value,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <Enum::Operation as alloy_sol_types::SolType>::tokenize(&self.operation),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.safeTxGas,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.baseGas,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.gasPrice,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<estimatedProcessingTimeCall> for UnderlyingRustTuple<'_> {
                fn from(value: estimatedProcessingTimeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for estimatedProcessingTimeCall {
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
            impl ::core::convert::From<estimatedProcessingTimeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: estimatedProcessingTimeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for estimatedProcessingTimeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for estimatedProcessingTimeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = estimatedProcessingTimeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<getTransactionStatusCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionStatusCall) -> Self {
                    (value.txHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransactionStatusCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTransactionStatusReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionStatusReturn) -> Self {
                    (value.status, value.message, value.remainingTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransactionStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { status: tuple.0, message: tuple.1, remainingTime: tuple.2 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTransactionStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTransactionStatusReturn;
            type ReturnTuple<'a> = (
                ValidationStatus,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getUserPendingTransactionsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getUserPendingTransactionsCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserPendingTransactionsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getUserPendingTransactionsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getUserPendingTransactionsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getUserPendingTransactionsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserPendingTransactionsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserPendingTransactionsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.user,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<userPendingTxsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: userPendingTxsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for userPendingTxsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userPendingTxsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = userPendingTxsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatedTxsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`SafeGuard`](self) function calls.
    pub enum SafeGuardCalls {
        checkAfterExecution(checkAfterExecutionCall),
        checkTransaction(checkTransactionCall),
        estimatedProcessingTime(estimatedProcessingTimeCall),
        getTransactionStatus(getTransactionStatusCall),
        getTrigger(getTriggerCall),
        getUserPendingTransactions(getUserPendingTransactionsCall),
        handleAddPayload(handleAddPayloadCall),
        initialize(initializeCall),
        initialized(initializedCall),
        nextTriggerId(nextTriggerIdCall),
        safe(safeCall),
        serviceProvider(serviceProviderCall),
        supportsInterface(supportsInterfaceCall),
        triggerIdsByCreator(triggerIdsByCreatorCall),
        triggersById(triggersByIdCall),
        txDetails(txDetailsCall),
        userPendingTxs(userPendingTxsCall),
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
            [0u8, 115u8, 225u8, 215u8],
            [1u8, 255u8, 201u8, 167u8],
            [21u8, 142u8, 249u8, 62u8],
            [24u8, 111u8, 3u8, 84u8],
            [66u8, 34u8, 127u8, 164u8],
            [117u8, 240u8, 187u8, 82u8],
            [123u8, 79u8, 51u8, 115u8],
            [141u8, 105u8, 233u8, 94u8],
            [145u8, 59u8, 31u8, 191u8],
            [147u8, 39u8, 19u8, 104u8],
            [148u8, 64u8, 116u8, 101u8],
            [151u8, 245u8, 54u8, 90u8],
            [153u8, 215u8, 207u8, 75u8],
            [158u8, 131u8, 227u8, 6u8],
            [196u8, 214u8, 109u8, 232u8],
            [197u8, 111u8, 176u8, 253u8],
            [206u8, 40u8, 150u8, 18u8],
            [227u8, 40u8, 237u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardCalls {
        const NAME: &'static str = "SafeGuardCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
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
                Self::getTrigger(_) => <getTriggerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getUserPendingTransactions(_) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::handleAddPayload(_) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialized(_) => <initializedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nextTriggerId(_) => <nextTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceProvider(_) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggerIdsByCreator(_) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::triggersById(_) => <triggersByIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::txDetails(_) => <txDetailsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::userPendingTxs(_) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatedTxs(_) => <validatedTxsCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<SafeGuardCalls>] = &[
                {
                    fn handleAddPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <handleAddPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::handleAddPayload)
                    }
                    handleAddPayload
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn initialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <initializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::initialized)
                    }
                    initialized
                },
                {
                    fn safe(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeGuardCalls::safe)
                    }
                    safe
                },
                {
                    fn nextTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <nextTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::nextTriggerId)
                    }
                    nextTriggerId
                },
                {
                    fn checkTransaction(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <checkTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::checkTransaction)
                    }
                    checkTransaction
                },
                {
                    fn txDetails(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <txDetailsCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeGuardCalls::txDetails)
                    }
                    txDetails
                },
                {
                    fn serviceProvider(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <serviceProviderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::serviceProvider)
                    }
                    serviceProvider
                },
                {
                    fn triggerIdsByCreator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::triggerIdsByCreator)
                    }
                    triggerIdsByCreator
                },
                {
                    fn checkAfterExecution(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(SafeGuardCalls::validatedTxs)
                    }
                    validatedTxs
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
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeGuardCalls::initialize)
                    }
                    initialize
                },
                {
                    fn estimatedProcessingTime(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::estimatedProcessingTime)
                    }
                    estimatedProcessingTime
                },
                {
                    fn triggersById(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <triggersByIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::triggersById)
                    }
                    triggersById
                },
                {
                    fn getTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <getTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeGuardCalls::getTrigger)
                    }
                    getTrigger
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
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::estimatedProcessingTime(inner) => {
                    <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getUserPendingTransactions(inner) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::userPendingTxs(inner) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::validatedTxs(inner) => {
                    <validatedTxsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::estimatedProcessingTime(inner) => {
                    <estimatedProcessingTimeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getTrigger(inner) => {
                    <getTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getUserPendingTransactions(inner) => {
                    <getUserPendingTransactionsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::triggerIdsByCreator(inner) => {
                    <triggerIdsByCreatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::triggersById(inner) => {
                    <triggersByIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::userPendingTxs(inner) => {
                    <userPendingTxsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::validatedTxs(inner) => {
                    <validatedTxsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) custom errors.
    pub enum SafeGuardErrors {
        AsyncValidationRequired(AsyncValidationRequired),
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
        pub const SELECTORS: &'static [[u8; 4usize]] =
            &[[49u8, 251u8, 135u8, 143u8], [54u8, 252u8, 87u8, 19u8], [227u8, 151u8, 149u8, 44u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardErrors {
        const NAME: &'static str = "SafeGuardErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AsyncValidationRequired(_) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::SELECTOR
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<SafeGuardErrors>] = &[
                {
                    fn TransactionNotFound(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <TransactionNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
                        )
                        .map(SafeGuardErrors::AsyncValidationRequired)
                    }
                    AsyncValidationRequired
                },
                {
                    fn TransactionExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardErrors> {
                        <TransactionExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardErrors::TransactionExpired)
                    }
                    TransactionExpired
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
                Self::AsyncValidationRequired(inner) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TransactionExpired(inner) => {
                    <TransactionExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TransactionNotFound(inner) => {
                    <TransactionNotFound as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AsyncValidationRequired(inner) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::TransactionExpired(inner) => {
                    <TransactionExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::TransactionNotFound(inner) => {
                    <TransactionNotFound as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) events.
    pub enum SafeGuardEvents {
        NewTrigger(NewTrigger),
        ValidationRequired(ValidationRequired),
        ValidationStatusUpdated(ValidationStatusUpdated),
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
                92u8, 82u8, 185u8, 32u8, 252u8, 93u8, 10u8, 196u8, 88u8, 56u8, 194u8, 5u8, 173u8,
                146u8, 101u8, 6u8, 18u8, 197u8, 206u8, 91u8, 248u8, 19u8, 106u8, 240u8, 47u8,
                166u8, 148u8, 102u8, 204u8, 58u8, 31u8, 217u8,
            ],
            [
                114u8, 184u8, 190u8, 170u8, 43u8, 22u8, 239u8, 194u8, 15u8, 247u8, 174u8, 169u8,
                66u8, 161u8, 34u8, 247u8, 183u8, 129u8, 25u8, 114u8, 79u8, 171u8, 189u8, 128u8,
                106u8, 205u8, 100u8, 215u8, 151u8, 137u8, 84u8, 203u8,
            ],
            [
                134u8, 234u8, 205u8, 35u8, 97u8, 13u8, 129u8, 112u8, 101u8, 22u8, 222u8, 30u8,
                208u8, 71u8, 108u8, 135u8, 119u8, 47u8, 223u8, 147u8, 156u8, 124u8, 119u8, 31u8,
                187u8, 215u8, 240u8, 35u8, 13u8, 97u8, 158u8, 104u8,
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
                Some(<NewTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTrigger)
                }
                Some(<ValidationRequired as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidationRequired as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidationRequired)
                }
                Some(<ValidationStatusUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidationStatusUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidationStatusUpdated)
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
    impl alloy_sol_types::private::IntoLogData for SafeGuardEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidationRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::NewTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidationRequired(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidationStatusUpdated(inner) => {
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<SafeGuardInstance<T, P, N>>>
    {
        SafeGuardInstance::<T, P, N>::deploy(provider, _safe)
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
        SafeGuardInstance::<T, P, N>::deploy_builder(provider, _safe)
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
        > SafeGuardInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`SafeGuard`](self) contract instance.

        See the [wrapper's documentation](`SafeGuardInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SafeGuardInstance<T, P, N>> {
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
        > SafeGuardInstance<T, P, N>
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
        ///Creates a new call builder for the [`checkAfterExecution`] function.
        pub fn checkAfterExecution(
            &self,
            txHash: alloy::sol_types::private::FixedBytes<32>,
            success: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkAfterExecutionCall, N> {
            self.call_builder(&checkAfterExecutionCall { txHash, success })
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
            self.call_builder(&checkTransactionCall {
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
            })
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
        ///Creates a new call builder for the [`getTrigger`] function.
        pub fn getTrigger(
            &self,
            triggerId: <ISimpleTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTriggerCall, N> {
            self.call_builder(&getTriggerCall { triggerId })
        }
        ///Creates a new call builder for the [`getUserPendingTransactions`] function.
        pub fn getUserPendingTransactions(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserPendingTransactionsCall, N> {
            self.call_builder(&getUserPendingTransactionsCall { user })
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
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(&supportsInterfaceCall { interfaceId })
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
        > SafeGuardInstance<T, P, N>
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
    }
}
