///Module containing a contract's types and functions.
/**

```solidity
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        pub artifact: alloy::sol_types::private::String,
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**```solidity
struct FuzzInterface { address addr; string[] artifacts; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        pub addr: alloy::sol_types::private::Address,
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        pub addr: alloy::sol_types::private::Address,
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
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
    > StdInvariantInstance<T, P, N> {
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
    > StdInvariantInstance<T, P, N> {
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
library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface WavsConditionalMarketTest {
    event MarketCreated(bytes32 indexed questionId, bytes32 indexed conditionId);
    event MarketResolved(bytes32 indexed questionId, bytes32 indexed conditionId, uint256 outcome);
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    function IS_TEST() external view returns (bool);
    function OUTCOME_SLOT_COUNT() external view returns (uint256);
    function QUESTION_ID() external view returns (bytes32);
    function STAKE_REGISTRY() external view returns (address);
    function collateral() external view returns (address);
    function conditionalTokens() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function market() external view returns (address);
    function setUp() external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function testFail_InvalidOutcome() external;
    function testFail_ResolveMarketTwice() external;
    function test_CreateMarket() external;
    function test_ResolveMarket() external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "IS_TEST",
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
    "name": "OUTCOME_SLOT_COUNT",
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
    "name": "QUESTION_ID",
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
    "name": "STAKE_REGISTRY",
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
    "name": "collateral",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockERC20"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "conditionalTokens",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract MockConditionalTokens"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
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
    "name": "market",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract WavsConditionalMarket"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setUp",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "testFail_InvalidOutcome",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "testFail_ResolveMarketTwice",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_CreateMarket",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "test_ResolveMarket",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "MarketCreated",
    "inputs": [
      {
        "name": "questionId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "conditionId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "MarketResolved",
    "inputs": [
      {
        "name": "questionId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "conditionId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "outcome",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
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
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
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
pub mod WavsConditionalMarketTest {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052600c8054600160ff199182168117909255601f80549091169091179055348015602b575f5ffd5b50613b34806100395f395ff3fe608060405234801561000f575f5ffd5b5060043610610148575f3560e01c80637a896a93116100bf578063b0464fdc11610079578063b0464fdc1461026d578063b5508aa914610275578063ba414fa61461027d578063d8dfeb4514610295578063e20c9f71146102a8578063fa7626d4146102b0575f5ffd5b80637a896a931461021357806380f556051461021b57806385226c8114610233578063916a17c6146102485780639f77ae501461025d578063a63c26bf14610265575f5ffd5b80633f7286f4116101105780633f7286f4146101995780635bd9e299146101a157806364cd43af146101cc57806366d9a9a0146101d4578063679e54e8146101e9578063761de19f1461020b575f5ffd5b80630a9254e41461014c5780631ed7831c146101565780632ade38801461017457806333595c46146101895780633e5e3c2314610191575b5f5ffd5b6101546102bd565b005b61015e6103ee565b60405161016b9190611b5f565b60405180910390f35b61017c61044e565b60405161016b9190611bed565b61015461058a565b61015e610dea565b61015e610e48565b6021546101b4906001600160a01b031681565b6040516001600160a01b03909116815260200161016b565b610154610ea6565b6101dc61108a565b60405161016b9190611cfa565b6101fd5f516020613ab95f395f51905f5281565b60405190815260200161016b565b6101b4600181565b6101546111ee565b601f546101b49061010090046001600160a01b031681565b61023b611340565b60405161016b9190611d78565b61025061140b565b60405161016b9190611dcf565b6101fd600281565b6101546114ec565b61025061169e565b61023b61177f565b61028561184a565b604051901515815260200161016b565b6020546101b4906001600160a01b031681565b61015e6118e6565b601f546102859060ff1681565b6040516102c990611b38565b6040808252600a90820152692a32b9ba102a37b5b2b760b11b606082015260806020820181905260049082015263151154d560e21b60a082015260c001604051809103905ff08015801561031f573d5f5f3e3d5ffd5b50602080546001600160a01b0319166001600160a01b039290921691909117905560405161034c90611b45565b604051809103905ff080158015610365573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039290921691821790556040516001919061039590611b52565b6001600160a01b03928316815291166020820152604001604051809103905ff0801580156103c5573d5f5f3e3d5ffd5b50601f60016101000a8154816001600160a01b0302191690836001600160a01b03160217905550565b6060601680548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610426575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610581575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561056a578382905f5260205f200180546104df90611e46565b80601f016020809104026020016040519081016040528092919081815260200182805461050b90611e46565b80156105565780601f1061052d57610100808354040283529160200191610556565b820191905f5260205f20905b81548152906001019060200180831161053957829003601f168201915b5050505050815260200190600101906104c2565b505050508152505081526020019060010190610471565b50505050905090565b601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c305710926105d5929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b1580156105ec575f5ffd5b505af11580156105fe573d5f5f3e3d5ffd5b5050602154601f54604051634296357160e11b81525f94506001600160a01b03928316935063852c6ae29261064e92610100900416905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015610669573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061068d9190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526001858701908152865192830193909352519481019490945251908301529192505f9060800160408051601f19818403018152828201909152600c82526b28bab2b9ba34b7b71024a21d60a11b6020830152915061071c905f516020613ab95f395f51905f52611944565b60408051808201909152600d81526c21b7b73234ba34b7b71024a21d60991b602082015261074a9084611944565b6107786040518060400160405280600881526020016727baba31b7b6b29d60c11b8152508360400151611944565b5f818051906020012090505f816040516020016107c191907f19457468657265756d205369676e6564204d6573736167653a0a3332000000008152601c810191909152603c0190565b60408051601f198184030181528282528051602091820120838301909252600d83526c26b2b9b9b0b3b2902430b9b41d60991b9083015291506108049083611944565b60408051808201909152601881527f457468205369676e6564204d65737361676520486173683a000000000000000060208201526108429082611944565b6040516338d07aa960e21b815260016004820152602481018290525f90819081905f516020613a995f395f51905f529063e341eaa490604401606060405180830381865afa158015610896573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ba9190611eb6565b60408051602081018490529081018290526001600160f81b031960f885901b16606082015292955090935091505f9060610160408051808303601f1901815282820182528983526020830181905290516001625e79b760e01b03198152600160048201529092505f905f516020613a995f395f51905f529063ffa1864990602401602060405180830381865afa158015610956573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061097a9190611eef565b90506109ad6040518060400160405280600f81526020016e29b4b3b732b91030b2323932b9b99d60891b8152508261198d565b6040516001600160a01b03821660248201525f516020613a995f395f51905f529063b96213e49060019060440160408051601f19818403018152918152602080830180516001600160e01b0316636f1e853360e01b1790528151600191810191909152016040516020818303038152906040526040518463ffffffff1660e01b8152600401610a3e93929190611f1c565b5f604051808303815f87803b158015610a55575f5ffd5b505af1158015610a67573d5f5f3e3d5ffd5b505060405163ca669fa760e01b81526001600160a01b03841660048201525f516020613a995f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015610ab6575f5ffd5b505af1158015610ac8573d5f5f3e3d5ffd5b505060405163248e63e160e11b8152600160048201819052602482018190525f604483015260648201525f516020613a995f395f51905f52925063491cc7c291506084015f604051808303815f87803b158015610b23575f5ffd5b505af1158015610b35573d5f5f3e3d5ffd5b505050508a5f516020613ab95f395f51905f527f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d16001604051610b7a91815260200190565b60405180910390a3601f54604051639aa9fda560e01b81526101009091046001600160a01b031690639aa9fda590610bb6908590600401611f5b565b5f604051808303815f87803b158015610bcd575f5ffd5b505af1925050508015610bde575060015b610ca057610bea611f9c565b806308c379a003610c4e5750610bfe611fed565b80610c095750610c50565b610c486040518060400160405280601e81526020017f6164645061796c6f6164206661696c6564207769746820726561736f6e3a0000815250826119d2565b50610cd5565b505b3d808015610c79576040519150601f19603f3d011682016040523d82523d5f602084013e610c7e565b606091505b50610c48604051806060016040528060268152602001613ad960269139611a17565b610cd56040518060400160405280601481526020017318591914185e5b1bd859081cdd58d8d95959195960621b815250611a17565b601f54604051638c93344f60e01b81525f516020613ab95f395f51905f526004820152610d589161010090046001600160a01b031690638c93344f90602401602060405180830381865afa158015610d2f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d539190612071565b611a5d565b601f5460405163ce1f456760e01b81525f516020613ab95f395f51905f526004820152610ddd9161010090046001600160a01b03169063ce1f456790602401602060405180830381865afa158015610db2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd69190611e9f565b6001611ab2565b5050505050505050505050565b6060601880548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b610eae61058a565b602154601f54604051634296357160e11b81525f926001600160a01b039081169263852c6ae292610efd92610100909204909116905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015610f18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3c9190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526001858701908152865192830193909352519481019490945251908301529192505f906080015b60408051808303601f1901815260418084526080840190925292505f91906020820181803683370190505060408051808201825284815260208101839052905163ca669fa760e01b81526101236004820152919250905f516020613a995f395f51905f529063ca669fa7906024015f604051808303815f87803b15801561100c575f5ffd5b505af115801561101e573d5f5f3e3d5ffd5b5050601f54604051639aa9fda560e01b81526101009091046001600160a01b03169250639aa9fda59150611056908490600401611f5b565b5f604051808303815f87803b15801561106d575f5ffd5b505af115801561107f573d5f5f3e3d5ffd5b505050505050505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f2090600202016040518060400160405290815f820180546110dd90611e46565b80601f016020809104026020016040519081016040528092919081815260200182805461110990611e46565b80156111545780601f1061112b57610100808354040283529160200191611154565b820191905f5260205f20905b81548152906001019060200180831161113757829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156111d657602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116111985790505b505050505081525050815260200190600101906110ad565b601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c30571092611239929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b158015611250575f5ffd5b505af1158015611262573d5f5f3e3d5ffd5b5050602154601f54604051634296357160e11b81525f94506001600160a01b03928316935063852c6ae2926112b292610100900416905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa1580156112cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f19190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526002858701908152865192830193909352519481019490945251908301529192505f90608001610f87565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f2001805461138090611e46565b80601f01602080910402602001604051908101604052809291908181526020018280546113ac90611e46565b80156113f75780601f106113ce576101008083540402835291602001916113f7565b820191905f5260205f20905b8154815290600101906020018083116113da57829003601f168201915b505050505081526020019060010190611363565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610581575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114d457602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114965790505b5050505050815250508152602001906001019061142e565b602154601f54604051634296357160e11b81525f926001600160a01b039081169263852c6ae29261153b92610100909204909116905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015611556573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061157a9190611e9f565b60405163248e63e160e11b8152600160048201819052602482018190525f604483015260648201529091505f516020613a995f395f51905f529063491cc7c2906084015f604051808303815f87803b1580156115d4575f5ffd5b505af11580156115e6573d5f5f3e3d5ffd5b50506040518392505f516020613ab95f395f51905f5291507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a3601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c3057109261166e929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b158015611685575f5ffd5b505af1158015611697573d5f5f3e3d5ffd5b5050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610581575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561176757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116117295790505b505050505081525050815260200190600101906116c1565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f200180546117bf90611e46565b80601f01602080910402602001604051908101604052809291908181526020018280546117eb90611e46565b80156118365780601f1061180d57610100808354040283529160200191611836565b820191905f5260205f20905b81548152906001019060200180831161181957829003601f168201915b5050505050815260200190600101906117a2565b6008545f9060ff1615611861575060085460ff1690565b604051630667f9d760e41b81525f516020613a995f395f51905f52600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa1580156118b8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118dc9190611e9f565b1415905090565b90565b6060601580548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b611989828260405160240161195a929190612090565b60408051601f198184030181529190526020810180516001600160e01b0316632d839cb360e21b179052611b15565b5050565b61198982826040516024016119a39291906120b1565b60408051601f198184030181529190526020810180516001600160e01b031663319af33360e01b179052611b15565b61198982826040516024016119e89291906120da565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052611b15565b611a5a81604051602401611a2b91906120fe565b60408051601f198184030181529190526020810180516001600160e01b031663104c13eb60e21b179052611b15565b50565b604051630c9fd58160e01b815281151560048201525f516020613a995f395f51905f5290630c9fd581906024015f6040518083038186803b158015611aa0575f5ffd5b505afa158015611697573d5f5f3e3d5ffd5b60405163260a5b1560e21b815260048101839052602481018290525f516020613a995f395f51905f52906398296c54906044015f6040518083038186803b158015611afb575f5ffd5b505afa158015611b0d573d5f5f3e3d5ffd5b505050505050565b611a5a815f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b610a108061211183390190565b61034880612b2183390190565b610c3080612e6983390190565b602080825282518282018190525f918401906040840190835b81811015611b9f5783516001600160a01b0316835260209384019390920191600101611b78565b509095945050505050565b5f81518084525f5b81811015611bce57602081850181015186830182015201611bb2565b505f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b81811015611c9057605f198a8503018352611c7a848651611baa565b6020958601959094509290920191600101611c5e565b509197505050602094850194929092019150600101611c13565b50929695505050505050565b5f8151808452602084019350602083015f5b82811015611cf05781516001600160e01b031916865260209586019590910190600101611cc8565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f198786030184528151805160408752611d466040880182611baa565b9050602082015191508681036020880152611d618183611cb6565b965050506020938401939190910190600101611d20565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f19878603018452611dba858351611baa565b94506020938401939190910190600101611d9e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57868503603f19018452815180516001600160a01b03168652602090810151604091870182905290611e3090870182611cb6565b9550506020938401939190910190600101611df5565b600181811c90821680611e5a57607f821691505b602082108103611e7857634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160a01b039390931683526020830191909152604082015260600190565b5f60208284031215611eaf575f5ffd5b5051919050565b5f5f5f60608486031215611ec8575f5ffd5b835160ff81168114611ed8575f5ffd5b602085015160409095015190969495509392505050565b5f60208284031215611eff575f5ffd5b81516001600160a01b0381168114611f15575f5ffd5b9392505050565b6001600160a01b03841681526060602082018190525f90611f3f90830185611baa565b8281036040840152611f518185611baa565b9695505050505050565b602081525f825160406020840152611f766060840182611baa565b90506020840151601f19848303016040850152611f938282611baa565b95945050505050565b5f60033d11156118e35760045f5f3e505f5160e01c90565b601f8201601f1916810167ffffffffffffffff81118282101715611fe657634e487b7160e01b5f52604160045260245ffd5b6040525050565b5f60443d1015611ffa5790565b6040513d600319016004823e80513d602482011167ffffffffffffffff8211171561202457505090565b808201805167ffffffffffffffff811115612040575050505090565b3d840160031901828201602001111561205a575050505090565b61206960208285010185611fb4565b509392505050565b5f60208284031215612081575f5ffd5b81518015158114611f15575f5ffd5b604081525f6120a26040830185611baa565b90508260208301529392505050565b604081525f6120c36040830185611baa565b905060018060a01b03831660208301529392505050565b604081525f6120ec6040830185611baa565b8281036020840152611f938185611baa565b602081525f611f156020830184611baa56fe608060405234801561000f575f5ffd5b50604051610a10380380610a1083398101604081905261002e91610105565b8181600361003c83826101ee565b50600461004982826101ee565b50505050506102a8565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610076575f5ffd5b81516001600160401b0381111561008f5761008f610053565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100bd576100bd610053565b6040528181528382016020018510156100d4575f5ffd5b5f5b828110156100f2576020818601810151838301820152016100d6565b505f918101602001919091529392505050565b5f5f60408385031215610116575f5ffd5b82516001600160401b0381111561012b575f5ffd5b61013785828601610067565b602085015190935090506001600160401b03811115610154575f5ffd5b61016085828601610067565b9150509250929050565b600181811c9082168061017e57607f821691505b60208210810361019c57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156101e957805f5260205f20601f840160051c810160208510156101c75750805b601f840160051c820191505b818110156101e6575f81556001016101d3565b50505b505050565b81516001600160401b0381111561020757610207610053565b61021b81610215845461016a565b846101a2565b6020601f82116001811461024d575f83156102365750848201515b5f19600385901b1c1916600184901b1784556101e6565b5f84815260208120601f198516915b8281101561027c578785015182556020948501946001909201910161025c565b508482101561029957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b61075b806102b55f395ff3fe608060405234801561000f575f5ffd5b506004361061009b575f3560e01c806340c10f191161006357806340c10f191461011457806370a082311461012957806395d89b4114610151578063a9059cbb14610159578063dd62ed3e1461016c575f5ffd5b806306fdde031461009f578063095ea7b3146100bd57806318160ddd146100e057806323b872dd146100f2578063313ce56714610105575b5f5ffd5b6100a76101a4565b6040516100b491906105b5565b60405180910390f35b6100d06100cb36600461061b565b610234565b60405190151581526020016100b4565b6002545b6040519081526020016100b4565b6100d0610100366004610643565b61024d565b604051601281526020016100b4565b61012761012236600461061b565b610270565b005b6100e461013736600461067d565b6001600160a01b03165f9081526020819052604090205490565b6100a761027e565b6100d061016736600461061b565b61028d565b6100e461017a36600461069d565b6001600160a01b039182165f90815260016020908152604080832093909416825291909152205490565b6060600380546101b3906106ce565b80601f01602080910402602001604051908101604052809291908181526020018280546101df906106ce565b801561022a5780601f106102015761010080835404028352916020019161022a565b820191905f5260205f20905b81548152906001019060200180831161020d57829003601f168201915b5050505050905090565b5f3361024181858561029a565b60019150505b92915050565b5f3361025a8582856102ac565b61026585858561032c565b506001949350505050565b61027a8282610389565b5050565b6060600480546101b3906106ce565b5f3361024181858561032c565b6102a783838360016103bd565b505050565b6001600160a01b038381165f908152600160209081526040808320938616835292905220545f198114610326578181101561031857604051637dc7a0d960e11b81526001600160a01b038416600482015260248101829052604481018390526064015b60405180910390fd5b61032684848484035f6103bd565b50505050565b6001600160a01b03831661035557604051634b637e8f60e11b81525f600482015260240161030f565b6001600160a01b03821661037e5760405163ec442f0560e01b81525f600482015260240161030f565b6102a783838361048f565b6001600160a01b0382166103b25760405163ec442f0560e01b81525f600482015260240161030f565b61027a5f838361048f565b6001600160a01b0384166103e65760405163e602df0560e01b81525f600482015260240161030f565b6001600160a01b03831661040f57604051634a1406b160e11b81525f600482015260240161030f565b6001600160a01b038085165f908152600160209081526040808320938716835292905220829055801561032657826001600160a01b0316846001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9258460405161048191815260200190565b60405180910390a350505050565b6001600160a01b0383166104b9578060025f8282546104ae9190610706565b909155506105299050565b6001600160a01b0383165f908152602081905260409020548181101561050b5760405163391434e360e21b81526001600160a01b0385166004820152602481018290526044810183905260640161030f565b6001600160a01b0384165f9081526020819052604090209082900390555b6001600160a01b03821661054557600280548290039055610563565b6001600160a01b0382165f9081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040516105a891815260200190565b60405180910390a3505050565b602081525f82518060208401525f5b818110156105e157602081860181015160408684010152016105c4565b505f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610616575f5ffd5b919050565b5f5f6040838503121561062c575f5ffd5b61063583610600565b946020939093013593505050565b5f5f5f60608486031215610655575f5ffd5b61065e84610600565b925061066c60208501610600565b929592945050506040919091013590565b5f6020828403121561068d575f5ffd5b61069682610600565b9392505050565b5f5f604083850312156106ae575f5ffd5b6106b783610600565b91506106c560208401610600565b90509250929050565b600181811c908216806106e257607f821691505b60208210810361070057634e487b7160e01b5f52602260045260245ffd5b50919050565b8082018082111561024757634e487b7160e01b5f52601160045260245ffdfea2646970667358221220fe1dfb04b261515d783c42c37727c6a547db0da7c9b24141248edff106da93be64736f6c634300081c00336080604052348015600e575f5ffd5b5061032c8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c806301b7037c1461004e578063852c6ae214610065578063c49298ac146100d3578063d96ee754146100e6575b5f5ffd5b61006361005c3660046101e4565b5050505050565b005b6100c1610073366004610247565b6040516bffffffffffffffffffffffff19606085901b16602082015260348101839052605481018290525f906074016040516020818303038152906040528051906020012090509392505050565b60405190815260200160405180910390f35b6100636100e1366004610277565b6100f9565b6100636100f4366004610247565b610138565b827ffbd3494747d87a8762583f3697021d2eae7789c823492b672da727715c3cb367838360405161012b9291906102bf565b60405180910390a2505050565b81836001600160a01b03167f97404ccd19300d892cb8813ac95c441f995c0db13e2b71b3661afabf9385bc818360405161017491815260200190565b60405180910390a3505050565b80356001600160a01b0381168114610197575f5ffd5b919050565b5f5f83601f8401126101ac575f5ffd5b50813567ffffffffffffffff8111156101c3575f5ffd5b6020830191508360208260051b85010111156101dd575f5ffd5b9250929050565b5f5f5f5f5f608086880312156101f8575f5ffd5b61020186610181565b94506020860135935060408601359250606086013567ffffffffffffffff81111561022a575f5ffd5b6102368882890161019c565b969995985093965092949392505050565b5f5f5f60608486031215610259575f5ffd5b61026284610181565b95602085013595506040909401359392505050565b5f5f5f60408486031215610289575f5ffd5b83359250602084013567ffffffffffffffff8111156102a6575f5ffd5b6102b28682870161019c565b9497909650939450505050565b602080825281018290525f6001600160fb1b038311156102dd575f5ffd5b8260051b8085604085013791909101604001939250505056fea264697066735822122064d4850aebfe59e4c2100896cf62961476b078f4a33a9b2316864eb020ccd4f964736f6c634300081c003360c060405234801561000f575f5ffd5b50604051610c30380380610c3083398101604081905261002e91610060565b6001600160a01b039182166080521660a052610091565b80516001600160a01b038116811461005b575f5ffd5b919050565b5f5f60408385031215610071575f5ffd5b61007a83610045565b915061008860208401610045565b90509250929050565b60805160a051610b636100cd5f395f818160d60152818161031a015281816103b1015261067201525f8181610128015261024e0152610b635ff3fe608060405234801561000f575f5ffd5b5060043610610090575f3560e01c8063761de19f11610063578063761de19f146101235780638c93344f1461014a5780639aa9fda51461016c578063a71f8da01461017f578063ce1f456714610192575f5ffd5b8063216a3e9a146100945780634c305710146100bc5780635bd9e299146100d157806363710c0514610110575b5f5ffd5b6100a76100a236600461077f565b6101bf565b60405190151581526020015b60405180910390f35b6100cf6100ca3660046107bd565b6102f5565b005b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100b3565b6100cf61011e3660046107fa565b610442565b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6100a761015836600461086b565b5f6020819052908152604090205460ff1681565b6100cf61017a36600461077f565b6104c6565b6100a761018d3660046107fa565b6104fa565b6101b16101a036600461086b565b60016020525f908152604090205481565b6040519081526020016100b3565b5f806101cb8380610882565b6040516101d99291906108cc565b604051809103902090505f61023a826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836102816020890189610882565b6040518463ffffffff1660e01b815260040161029f93929190610903565b602060405180830381865afa1580156102ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102de9190610925565b6001600160e01b0319918216911614949350505050565b604051634296357160e11b815230600482015260248101839052604481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063852c6ae290606401602060405180830381865afa158015610367573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038b919061094c565b60405163365bb9d560e21b815230600482015260248101859052604481018490529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d96ee754906064015f604051808303815f87803b1580156103fa575f5ffd5b505af115801561040c573d5f5f3e3d5ffd5b50506040518392508591507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a350505050565b818161044e82826104fa565b61046b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156104bf576104ad85858363ffffffff1681811061049657610496610963565b90506020028101906104a89190610977565b6105c1565b806104b781610995565b91505061046d565b5050505050565b806104d0816101bf565b6104ed57604051638baa579f60e01b815260040160405180910390fd5b6104f6826105c1565b5050565b5f805b63ffffffff81168311156105b5573063216a3e9a858563ffffffff851681811061052957610529610963565b905060200281019061053b9190610977565b6040518263ffffffff1660e01b81526004016105579190610a07565b602060405180830381865afa158015610572573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105969190610a5c565b6105a3575f9150506105bb565b806105ad81610995565b9150506104fd565b50600190505b92915050565b5f6105cc8280610882565b8101906105d99190610a7b565b80515f9081526020819052604090205490915060ff161561060d5760405163aa43cb2d60e01b815260040160405180910390fd5b6001816040015111156106335760405163c74a206d60e01b815260040160405180910390fd5b80515f90815260208181526040808320805460ff1916600190811790915581850180518651865291909352922091909155815190516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163c49298ac916106a290610736565b6040518363ffffffff1660e01b81526004016106bf929190610ae0565b5f604051808303815f87803b1580156106d6575f5ffd5b505af11580156106e8573d5f5f3e3d5ffd5b505050508060200151815f01517f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d1836040015160405161072a91815260200190565b60405180910390a35050565b60408051600280825260608083018452925f929190602083019080368337019050509050600181848151811061076e5761076e610963565b602090810291909101015292915050565b5f6020828403121561078f575f5ffd5b813567ffffffffffffffff8111156107a5575f5ffd5b8201604081850312156107b6575f5ffd5b9392505050565b5f5f5f606084860312156107cf575f5ffd5b83356001600160a01b03811681146107e5575f5ffd5b95602085013595506040909401359392505050565b5f5f6020838503121561080b575f5ffd5b823567ffffffffffffffff811115610821575f5ffd5b8301601f81018513610831575f5ffd5b803567ffffffffffffffff811115610847575f5ffd5b8560208260051b840101111561085b575f5ffd5b6020919091019590945092505050565b5f6020828403121561087b575f5ffd5b5035919050565b5f5f8335601e19843603018112610897575f5ffd5b83018035915067ffffffffffffffff8211156108b1575f5ffd5b6020019150368190038213156108c5575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61091c6040830184866108db565b95945050505050565b5f60208284031215610935575f5ffd5b81516001600160e01b0319811681146107b6575f5ffd5b5f6020828403121561095c575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261098b575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff81036109bc57634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e198436030181126109da575f5ffd5b830160208101925035905067ffffffffffffffff8111156109f9575f5ffd5b8036038213156108c5575f5ffd5b602081525f610a1683846109c5565b60406020850152610a2b6060850182846108db565b915050610a3b60208501856109c5565b848303601f19016040860152610a528382846108db565b9695505050505050565b5f60208284031215610a6c575f5ffd5b815180151581146107b6575f5ffd5b5f6060828403128015610a8c575f5ffd5b506040516060810167ffffffffffffffff81118282101715610abc57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b5f60408201848352604060208401528084518083526060850191506020860192505f5b81811015610b21578351835260209384019390920191600101610b03565b5090969550505050505056fea26469706673582212207a5dc298a257036afb5a67be155c7a7ab06a20bc1df591976b60bfd43121f3da64736f6c634300081c00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d591eed45d83ee01c18f61af559cbb698bc6d377a62748889af9cd67b0e69e2696164645061796c6f6164206661696c65642077697468206c6f772d6c6576656c206572726f72a2646970667358221220ad4297ee0ee6eeca5361dd2af6e09be93f1f187d8b6e9643678239805c582dd364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15`+W__\xFD[Pa;4\x80a\09_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80cz\x89j\x93\x11a\0\xBFW\x80c\xB0FO\xDC\x11a\0yW\x80c\xB0FO\xDC\x14a\x02mW\x80c\xB5P\x8A\xA9\x14a\x02uW\x80c\xBAAO\xA6\x14a\x02}W\x80c\xD8\xDF\xEBE\x14a\x02\x95W\x80c\xE2\x0C\x9Fq\x14a\x02\xA8W\x80c\xFAv&\xD4\x14a\x02\xB0W__\xFD[\x80cz\x89j\x93\x14a\x02\x13W\x80c\x80\xF5V\x05\x14a\x02\x1BW\x80c\x85\"l\x81\x14a\x023W\x80c\x91j\x17\xC6\x14a\x02HW\x80c\x9Fw\xAEP\x14a\x02]W\x80c\xA6<&\xBF\x14a\x02eW__\xFD[\x80c?r\x86\xF4\x11a\x01\x10W\x80c?r\x86\xF4\x14a\x01\x99W\x80c[\xD9\xE2\x99\x14a\x01\xA1W\x80cd\xCDC\xAF\x14a\x01\xCCW\x80cf\xD9\xA9\xA0\x14a\x01\xD4W\x80cg\x9ET\xE8\x14a\x01\xE9W\x80cv\x1D\xE1\x9F\x14a\x02\x0BW__\xFD[\x80c\n\x92T\xE4\x14a\x01LW\x80c\x1E\xD7\x83\x1C\x14a\x01VW\x80c*\xDE8\x80\x14a\x01tW\x80c3Y\\F\x14a\x01\x89W\x80c>^<#\x14a\x01\x91W[__\xFD[a\x01Ta\x02\xBDV[\0[a\x01^a\x03\xEEV[`@Qa\x01k\x91\x90a\x1B_V[`@Q\x80\x91\x03\x90\xF3[a\x01|a\x04NV[`@Qa\x01k\x91\x90a\x1B\xEDV[a\x01Ta\x05\x8AV[a\x01^a\r\xEAV[a\x01^a\x0EHV[`!Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01kV[a\x01Ta\x0E\xA6V[a\x01\xDCa\x10\x8AV[`@Qa\x01k\x91\x90a\x1C\xFAV[a\x01\xFD_Q` a:\xB9_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x01kV[a\x01\xB4`\x01\x81V[a\x01Ta\x11\xEEV[`\x1FTa\x01\xB4\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02;a\x13@V[`@Qa\x01k\x91\x90a\x1DxV[a\x02Pa\x14\x0BV[`@Qa\x01k\x91\x90a\x1D\xCFV[a\x01\xFD`\x02\x81V[a\x01Ta\x14\xECV[a\x02Pa\x16\x9EV[a\x02;a\x17\x7FV[a\x02\x85a\x18JV[`@Q\x90\x15\x15\x81R` \x01a\x01kV[` Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01^a\x18\xE6V[`\x1FTa\x02\x85\x90`\xFF\x16\x81V[`@Qa\x02\xC9\x90a\x1B8V[`@\x80\x82R`\n\x90\x82\x01Ri*2\xB9\xBA\x10*7\xB5\xB2\xB7`\xB1\x1B``\x82\x01R`\x80` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\x15\x11T\xD5`\xE2\x1B`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03\x1FW=__>=_\xFD[P` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x03L\x90a\x1BEV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03eW=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Q`\x01\x91\x90a\x03\x95\x90a\x1BRV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03\xC5W=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05jW\x83\x82\x90_R` _ \x01\x80Ta\x04\xDF\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x0B\x90a\x1EFV[\x80\x15a\x05VW\x80`\x1F\x10a\x05-Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05VV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x059W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xC2V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04qV[PPPP\x90P\x90V[`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x05\xD5\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xECW__\xFD[PZ\xF1\x15\x80\x15a\x05\xFEW=__>=_\xFD[PP`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x85,j\xE2\x92a\x06N\x92a\x01\0\x90\x04\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x8D\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x01\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x0C\x82Rk(\xBA\xB2\xB9\xBA4\xB7\xB7\x10$\xA2\x1D`\xA1\x1B` \x83\x01R\x91Pa\x07\x1C\x90_Q` a:\xB9_9_Q\x90_Ra\x19DV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81Rl!\xB7\xB724\xBA4\xB7\xB7\x10$\xA2\x1D`\x99\x1B` \x82\x01Ra\x07J\x90\x84a\x19DV[a\x07x`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g'\xBA\xBA1\xB7\xB6\xB2\x9D`\xC1\x1B\x81RP\x83`@\x01Qa\x19DV[_\x81\x80Q\x90` \x01 \x90P_\x81`@Q` \x01a\x07\xC1\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x83\x83\x01\x90\x92R`\r\x83Rl&\xB2\xB9\xB9\xB0\xB3\xB2\x90$0\xB9\xB4\x1D`\x99\x1B\x90\x83\x01R\x91Pa\x08\x04\x90\x83a\x19DV[`@\x80Q\x80\x82\x01\x90\x91R`\x18\x81R\x7FEth Signed Message Hash:\0\0\0\0\0\0\0\0` \x82\x01Ra\x08B\x90\x82a\x19DV[`@Qc8\xD0z\xA9`\xE2\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x82\x90R_\x90\x81\x90\x81\x90_Q` a:\x99_9_Q\x90_R\x90c\xE3A\xEA\xA4\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBA\x91\x90a\x1E\xB6V[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x85\x90\x1B\x16``\x82\x01R\x92\x95P\x90\x93P\x91P_\x90`a\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R\x89\x83R` \x83\x01\x81\x90R\x90Q`\x01b^y\xB7`\xE0\x1B\x03\x19\x81R`\x01`\x04\x82\x01R\x90\x92P_\x90_Q` a:\x99_9_Q\x90_R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tz\x91\x90a\x1E\xEFV[\x90Pa\t\xAD`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n)\xB4\xB3\xB72\xB9\x100\xB2292\xB9\xB9\x9D`\x89\x1B\x81RP\x82a\x19\x8DV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_Q` a:\x99_9_Q\x90_R\x90c\xB9b\x13\xE4\x90`\x01\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16co\x1E\x853`\xE0\x1B\x17\x90R\x81Q`\x01\x91\x81\x01\x91\x90\x91R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n>\x93\x92\x91\x90a\x1F\x1CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\nUW__\xFD[PZ\xF1\x15\x80\x15a\ngW=__>=_\xFD[PP`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R_Q` a:\x99_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xB6W__\xFD[PZ\xF1\x15\x80\x15a\n\xC8W=__>=_\xFD[PP`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R_`D\x83\x01R`d\x82\x01R_Q` a:\x99_9_Q\x90_R\x92PcI\x1C\xC7\xC2\x91P`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B#W__\xFD[PZ\xF1\x15\x80\x15a\x0B5W=__>=_\xFD[PPPP\x8A_Q` a:\xB9_9_Q\x90_R\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1`\x01`@Qa\x0Bz\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x1FT`@Qc\x9A\xA9\xFD\xA5`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9A\xA9\xFD\xA5\x90a\x0B\xB6\x90\x85\x90`\x04\x01a\x1F[V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xCDW__\xFD[PZ\xF1\x92PPP\x80\x15a\x0B\xDEWP`\x01[a\x0C\xA0Wa\x0B\xEAa\x1F\x9CV[\x80c\x08\xC3y\xA0\x03a\x0CNWPa\x0B\xFEa\x1F\xEDV[\x80a\x0C\tWPa\x0CPV[a\x0CH`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FaddPayload failed with reason:\0\0\x81RP\x82a\x19\xD2V[Pa\x0C\xD5V[P[=\x80\x80\x15a\x0CyW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C~V[``\x91P[Pa\x0CH`@Q\x80``\x01`@R\x80`&\x81R` \x01a:\xD9`&\x919a\x1A\x17V[a\x0C\xD5`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x18Y\x19\x14\x18^[\x1B\xD8Y\x08\x1C\xDDX\xD8\xD9YY\x19Y`b\x1B\x81RPa\x1A\x17V[`\x1FT`@Qc\x8C\x934O`\xE0\x1B\x81R_Q` a:\xB9_9_Q\x90_R`\x04\x82\x01Ra\rX\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8C\x934O\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rS\x91\x90a qV[a\x1A]V[`\x1FT`@Qc\xCE\x1FEg`\xE0\x1B\x81R_Q` a:\xB9_9_Q\x90_R`\x04\x82\x01Ra\r\xDD\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x1FEg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD6\x91\x90a\x1E\x9FV[`\x01a\x1A\xB2V[PPPPPPPPPPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[a\x0E\xAEa\x05\x8AV[`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\x85,j\xE2\x92a\x0E\xFD\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F<\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x01\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R`A\x80\x84R`\x80\x84\x01\x90\x92R\x92P_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP`@\x80Q\x80\x82\x01\x82R\x84\x81R` \x81\x01\x83\x90R\x90Qc\xCAf\x9F\xA7`\xE0\x1B\x81Ra\x01#`\x04\x82\x01R\x91\x92P\x90_Q` a:\x99_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x0CW__\xFD[PZ\xF1\x15\x80\x15a\x10\x1EW=__>=_\xFD[PP`\x1FT`@Qc\x9A\xA9\xFD\xA5`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x9A\xA9\xFD\xA5\x91Pa\x10V\x90\x84\x90`\x04\x01a\x1F[V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10mW__\xFD[PZ\xF1\x15\x80\x15a\x10\x7FW=__>=_\xFD[PPPPPPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x10\xDD\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\t\x90a\x1EFV[\x80\x15a\x11TW\x80`\x1F\x10a\x11+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11TV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x117W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\xD6W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11\x98W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\xADV[`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x129\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12PW__\xFD[PZ\xF1\x15\x80\x15a\x12bW=__>=_\xFD[PP`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x85,j\xE2\x92a\x12\xB2\x92a\x01\0\x90\x04\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF1\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x02\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01a\x0F\x87V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x01\x80Ta\x13\x80\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xAC\x90a\x1EFV[\x80\x15a\x13\xF7W\x80`\x1F\x10a\x13\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xF7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x13cV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xD4W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\x96W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14.V[`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\x85,j\xE2\x92a\x15;\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15z\x91\x90a\x1E\x9FV[`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R_`D\x83\x01R`d\x82\x01R\x90\x91P_Q` a:\x99_9_Q\x90_R\x90cI\x1C\xC7\xC2\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xD4W__\xFD[PZ\xF1\x15\x80\x15a\x15\xE6W=__>=_\xFD[PP`@Q\x83\x92P_Q` a:\xB9_9_Q\x90_R\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x16n\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x85W__\xFD[PZ\xF1\x15\x80\x15a\x16\x97W=__>=_\xFD[PPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x17gW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x17)W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x16\xC1V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x01\x80Ta\x17\xBF\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xEB\x90a\x1EFV[\x80\x15a\x186W\x80`\x1F\x10a\x18\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x186V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x17\xA2V[`\x08T_\x90`\xFF\x16\x15a\x18aWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_Q` a:\x99_9_Q\x90_R`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xDC\x91\x90a\x1E\x9FV[\x14\x15\x90P\x90V[\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[a\x19\x89\x82\x82`@Q`$\x01a\x19Z\x92\x91\x90a \x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1B\x15V[PPV[a\x19\x89\x82\x82`@Q`$\x01a\x19\xA3\x92\x91\x90a \xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x1B\x15V[a\x19\x89\x82\x82`@Q`$\x01a\x19\xE8\x92\x91\x90a \xDAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra\x1B\x15V[a\x1AZ\x81`@Q`$\x01a\x1A+\x91\x90a \xFEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1B\x15V[PV[`@Qc\x0C\x9F\xD5\x81`\xE0\x1B\x81R\x81\x15\x15`\x04\x82\x01R_Q` a:\x99_9_Q\x90_R\x90c\x0C\x9F\xD5\x81\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xA0W__\xFD[PZ\xFA\x15\x80\x15a\x16\x97W=__>=_\xFD[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a:\x99_9_Q\x90_R\x90c\x98)lT\x90`D\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xFBW__\xFD[PZ\xFA\x15\x80\x15a\x1B\rW=__>=_\xFD[PPPPPPV[a\x1AZ\x81_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[a\n\x10\x80a!\x11\x839\x01\x90V[a\x03H\x80a+!\x839\x01\x90V[a\x0C0\x80a.i\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1B\x9FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BxV[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x1B\xCEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1B\xB2V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a\x1C\x90W`_\x19\x8A\x85\x03\x01\x83Ra\x1Cz\x84\x86Qa\x1B\xAAV[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a\x1C^V[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a\x1C\x13V[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x1C\xF0W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x1C\xC8V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra\x1DF`@\x88\x01\x82a\x1B\xAAV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra\x1Da\x81\x83a\x1C\xB6V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84Ra\x1D\xBA\x85\x83Qa\x1B\xAAV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D\x9EV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\x1E0\x90\x87\x01\x82a\x1C\xB6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D\xF5V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1ExWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x1E\xAFW__\xFD[PQ\x91\x90PV[___``\x84\x86\x03\x12\x15a\x1E\xC8W__\xFD[\x83Q`\xFF\x81\x16\x81\x14a\x1E\xD8W__\xFD[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1E\xFFW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\x15W__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a\x1F?\x90\x83\x01\x85a\x1B\xAAV[\x82\x81\x03`@\x84\x01Ra\x1FQ\x81\x85a\x1B\xAAV[\x96\x95PPPPPPV[` \x81R_\x82Q`@` \x84\x01Ra\x1Fv``\x84\x01\x82a\x1B\xAAV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x1F\x93\x82\x82a\x1B\xAAV[\x95\x94PPPPPV[_`\x03=\x11\x15a\x18\xE3W`\x04__>P_Q`\xE0\x1C\x90V[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xE6WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@RPPV[_`D=\x10\x15a\x1F\xFAW\x90V[`@Q=`\x03\x19\x01`\x04\x82>\x80Q=`$\x82\x01\x11g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a $WPP\x90V[\x80\x82\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a @WPPPP\x90V[=\x84\x01`\x03\x19\x01\x82\x82\x01` \x01\x11\x15a ZWPPPP\x90V[a i` \x82\x85\x01\x01\x85a\x1F\xB4V[P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \x81W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\x15W__\xFD[`@\x81R_a \xA2`@\x83\x01\x85a\x1B\xAAV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a \xC3`@\x83\x01\x85a\x1B\xAAV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@\x81R_a \xEC`@\x83\x01\x85a\x1B\xAAV[\x82\x81\x03` \x84\x01Ra\x1F\x93\x81\x85a\x1B\xAAV[` \x81R_a\x1F\x15` \x83\x01\x84a\x1B\xAAV\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\n\x108\x03\x80a\n\x10\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x05V[\x81\x81`\x03a\0<\x83\x82a\x01\xEEV[P`\x04a\0I\x82\x82a\x01\xEEV[PPPPPa\x02\xA8V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0vW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\x8FWa\0\x8Fa\0SV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xBDWa\0\xBDa\0SV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xD4W__\xFD[_[\x82\x81\x10\x15a\0\xF2W` \x81\x86\x01\x81\x01Q\x83\x83\x01\x82\x01R\x01a\0\xD6V[P_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x01\x16W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01+W__\xFD[a\x017\x85\x82\x86\x01a\0gV[` \x85\x01Q\x90\x93P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01TW__\xFD[a\x01`\x85\x82\x86\x01a\0gV[\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01~W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\x9CWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xE9W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xC7WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xE6W_\x81U`\x01\x01a\x01\xD3V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x07Wa\x02\x07a\0SV[a\x02\x1B\x81a\x02\x15\x84Ta\x01jV[\x84a\x01\xA2V[` `\x1F\x82\x11`\x01\x81\x14a\x02MW_\x83\x15a\x026WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x01\xE6V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\\V[P\x84\x82\x10\x15a\x02\x99W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x07[\x80a\x02\xB5_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9BW_5`\xE0\x1C\x80c@\xC1\x0F\x19\x11a\0cW\x80c@\xC1\x0F\x19\x14a\x01\x14W\x80cp\xA0\x821\x14a\x01)W\x80c\x95\xD8\x9BA\x14a\x01QW\x80c\xA9\x05\x9C\xBB\x14a\x01YW\x80c\xDDb\xED>\x14a\x01lW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\x9FW\x80c\t^\xA7\xB3\x14a\0\xBDW\x80c\x18\x16\r\xDD\x14a\0\xE0W\x80c#\xB8r\xDD\x14a\0\xF2W\x80c1<\xE5g\x14a\x01\x05W[__\xFD[a\0\xA7a\x01\xA4V[`@Qa\0\xB4\x91\x90a\x05\xB5V[`@Q\x80\x91\x03\x90\xF3[a\0\xD0a\0\xCB6`\x04a\x06\x1BV[a\x024V[`@Q\x90\x15\x15\x81R` \x01a\0\xB4V[`\x02T[`@Q\x90\x81R` \x01a\0\xB4V[a\0\xD0a\x01\x006`\x04a\x06CV[a\x02MV[`@Q`\x12\x81R` \x01a\0\xB4V[a\x01'a\x01\"6`\x04a\x06\x1BV[a\x02pV[\0[a\0\xE4a\x0176`\x04a\x06}V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0\xA7a\x02~V[a\0\xD0a\x01g6`\x04a\x06\x1BV[a\x02\x8DV[a\0\xE4a\x01z6`\x04a\x06\x9DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\x03\x80Ta\x01\xB3\x90a\x06\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xDF\x90a\x06\xCEV[\x80\x15a\x02*W\x80`\x1F\x10a\x02\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02*V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x02A\x81\x85\x85a\x02\x9AV[`\x01\x91PP[\x92\x91PPV[_3a\x02Z\x85\x82\x85a\x02\xACV[a\x02e\x85\x85\x85a\x03,V[P`\x01\x94\x93PPPPV[a\x02z\x82\x82a\x03\x89V[PPV[```\x04\x80Ta\x01\xB3\x90a\x06\xCEV[_3a\x02A\x81\x85\x85a\x03,V[a\x02\xA7\x83\x83\x83`\x01a\x03\xBDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T_\x19\x81\x14a\x03&W\x81\x81\x10\x15a\x03\x18W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03&\x84\x84\x84\x84\x03_a\x03\xBDV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x03UW`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03~W`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[a\x02\xA7\x83\x83\x83a\x04\x8FV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xB2W`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[a\x02z_\x83\x83a\x04\x8FV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x03\xE6W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\x0FW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x03&W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x04\x81\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\xB9W\x80`\x02_\x82\x82Ta\x04\xAE\x91\x90a\x07\x06V[\x90\x91UPa\x05)\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x05\x0BW`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05EW`\x02\x80T\x82\x90\x03\x90Ua\x05cV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x05\xA8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15a\x05\xE1W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x05\xC4V[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x16W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x06,W__\xFD[a\x065\x83a\x06\0V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x06UW__\xFD[a\x06^\x84a\x06\0V[\x92Pa\x06l` \x85\x01a\x06\0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x06\x8DW__\xFD[a\x06\x96\x82a\x06\0V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x06\xAEW__\xFD[a\x06\xB7\x83a\x06\0V[\x91Pa\x06\xC5` \x84\x01a\x06\0V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02GWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xFE\x1D\xFB\x04\xB2aQ]x<B\xC3w'\xC6\xA5G\xDB\r\xA7\xC9\xB2AA$\x8E\xDF\xF1\x06\xDA\x93\xBEdsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x03,\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\x01\xB7\x03|\x14a\0NW\x80c\x85,j\xE2\x14a\0eW\x80c\xC4\x92\x98\xAC\x14a\0\xD3W\x80c\xD9n\xE7T\x14a\0\xE6W[__\xFD[a\0ca\0\\6`\x04a\x01\xE4V[PPPPPV[\0[a\0\xC1a\0s6`\x04a\x02GV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x85\x90\x1B\x16` \x82\x01R`4\x81\x01\x83\x90R`T\x81\x01\x82\x90R_\x90`t\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ca\0\xE16`\x04a\x02wV[a\0\xF9V[a\0ca\0\xF46`\x04a\x02GV[a\x018V[\x82\x7F\xFB\xD3IGG\xD8z\x87bX?6\x97\x02\x1D.\xAEw\x89\xC8#I+g-\xA7'q\\<\xB3g\x83\x83`@Qa\x01+\x92\x91\x90a\x02\xBFV[`@Q\x80\x91\x03\x90\xA2PPPV[\x81\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x97@L\xCD\x190\r\x89,\xB8\x81:\xC9\\D\x1F\x99\\\r\xB1>+q\xB3f\x1A\xFA\xBF\x93\x85\xBC\x81\x83`@Qa\x01t\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x97W__\xFD[\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x01\xACW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xC3W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x01\xDDW__\xFD[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x01\xF8W__\xFD[a\x02\x01\x86a\x01\x81V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02*W__\xFD[a\x026\x88\x82\x89\x01a\x01\x9CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x02YW__\xFD[a\x02b\x84a\x01\x81V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[___`@\x84\x86\x03\x12\x15a\x02\x89W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xA6W__\xFD[a\x02\xB2\x86\x82\x87\x01a\x01\x9CV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x81\x01\x82\x90R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x02\xDDW__\xFD[\x82`\x05\x1B\x80\x85`@\x85\x017\x91\x90\x91\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 d\xD4\x85\n\xEB\xFEY\xE4\xC2\x10\x08\x96\xCFb\x96\x14v\xB0x\xF4\xA3:\x9B#\x16\x86N\xB0 \xCC\xD4\xF9dsolcC\0\x08\x1C\x003`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0C08\x03\x80a\x0C0\x839\x81\x01`@\x81\x90Ra\0.\x91a\0`V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x91V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0qW__\xFD[a\0z\x83a\0EV[\x91Pa\0\x88` \x84\x01a\0EV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0Bca\0\xCD_9_\x81\x81`\xD6\x01R\x81\x81a\x03\x1A\x01R\x81\x81a\x03\xB1\x01Ra\x06r\x01R_\x81\x81a\x01(\x01Ra\x02N\x01Ra\x0Bc_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x90W_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0cW\x80cv\x1D\xE1\x9F\x14a\x01#W\x80c\x8C\x934O\x14a\x01JW\x80c\x9A\xA9\xFD\xA5\x14a\x01lW\x80c\xA7\x1F\x8D\xA0\x14a\x01\x7FW\x80c\xCE\x1FEg\x14a\x01\x92W__\xFD[\x80c!j>\x9A\x14a\0\x94W\x80cL0W\x10\x14a\0\xBCW\x80c[\xD9\xE2\x99\x14a\0\xD1W\x80ccq\x0C\x05\x14a\x01\x10W[__\xFD[a\0\xA7a\0\xA26`\x04a\x07\x7FV[a\x01\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a\x07\xBDV[a\x02\xF5V[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xCFa\x01\x1E6`\x04a\x07\xFAV[a\x04BV[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA7a\x01X6`\x04a\x08kV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xCFa\x01z6`\x04a\x07\x7FV[a\x04\xC6V[a\0\xA7a\x01\x8D6`\x04a\x07\xFAV[a\x04\xFAV[a\x01\xB1a\x01\xA06`\x04a\x08kV[`\x01` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB3V[_\x80a\x01\xCB\x83\x80a\x08\x82V[`@Qa\x01\xD9\x92\x91\x90a\x08\xCCV[`@Q\x80\x91\x03\x90 \x90P_a\x02:\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x81` \x89\x01\x89a\x08\x82V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x9F\x93\x92\x91\x90a\t\x03V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDE\x91\x90a\t%V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[`@QcB\x965q`\xE1\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85,j\xE2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03gW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8B\x91\x90a\tLV[`@Qc6[\xB9\xD5`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD9n\xE7T\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xFAW__\xFD[PZ\xF1\x15\x80\x15a\x04\x0CW=__>=_\xFD[PP`@Q\x83\x92P\x85\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3PPPPV[\x81\x81a\x04N\x82\x82a\x04\xFAV[a\x04kW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04\xBFWa\x04\xAD\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04\x96Wa\x04\x96a\tcV[\x90P` \x02\x81\x01\x90a\x04\xA8\x91\x90a\twV[a\x05\xC1V[\x80a\x04\xB7\x81a\t\x95V[\x91PPa\x04mV[PPPPPV[\x80a\x04\xD0\x81a\x01\xBFV[a\x04\xEDW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xF6\x82a\x05\xC1V[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x05\xB5W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x05)Wa\x05)a\tcV[\x90P` \x02\x81\x01\x90a\x05;\x91\x90a\twV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05W\x91\x90a\n\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\n\\V[a\x05\xA3W_\x91PPa\x05\xBBV[\x80a\x05\xAD\x81a\t\x95V[\x91PPa\x04\xFDV[P`\x01\x90P[\x92\x91PPV[_a\x05\xCC\x82\x80a\x08\x82V[\x81\x01\x90a\x05\xD9\x91\x90a\n{V[\x80Q_\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x15a\x06\rW`@Qc\xAAC\xCB-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`@\x01Q\x11\x15a\x063W`@Qc\xC7J m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q_\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x81\x85\x01\x80Q\x86Q\x86R\x91\x90\x93R\x92 \x91\x90\x91U\x81Q\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xC4\x92\x98\xAC\x91a\x06\xA2\x90a\x076V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBF\x92\x91\x90a\n\xE0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD6W__\xFD[PZ\xF1\x15\x80\x15a\x06\xE8W=__>=_\xFD[PPPP\x80` \x01Q\x81_\x01Q\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1\x83`@\x01Q`@Qa\x07*\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92_\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P`\x01\x81\x84\x81Q\x81\x10a\x07nWa\x07na\tcV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\x8FW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA5W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x07\xB6W__\xFD[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x07\xCFW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE5W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x08\x0BW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08!W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x081W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08GW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x08[W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x08{W__\xFD[P5\x91\x90PV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x97W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB1W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xC5W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\t\x1C`@\x83\x01\x84\x86a\x08\xDBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\t5W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xB6W__\xFD[_` \x82\x84\x03\x12\x15a\t\\W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\t\x8BW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\t\xBCWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xDAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF9W__\xFD[\x806\x03\x82\x13\x15a\x08\xC5W__\xFD[` \x81R_a\n\x16\x83\x84a\t\xC5V[`@` \x85\x01Ra\n+``\x85\x01\x82\x84a\x08\xDBV[\x91PPa\n;` \x85\x01\x85a\t\xC5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\nR\x83\x82\x84a\x08\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\nlW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xB6W__\xFD[_``\x82\x84\x03\x12\x80\x15a\n\x8CW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBCWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a\x0B!W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x03V[P\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 z]\xC2\x98\xA2W\x03j\xFBZg\xBE\x15\\zz\xB0j \xBC\x1D\xF5\x91\x97k`\xBF\xD41!\xF3\xDAdsolcC\0\x08\x1C\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Y\x1E\xEDE\xD8>\xE0\x1C\x18\xF6\x1A\xF5Y\xCB\xB6\x98\xBCm7zbt\x88\x89\xAF\x9C\xD6{\x0Ei\xE2iaddPayload failed with low-level error\xA2dipfsX\"\x12 \xADB\x97\xEE\x0E\xE6\xEE\xCASa\xDD*\xF6\xE0\x9B\xE9?\x1F\x18}\x8Bn\x96Cg\x829\x80\\X-\xD3dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610148575f3560e01c80637a896a93116100bf578063b0464fdc11610079578063b0464fdc1461026d578063b5508aa914610275578063ba414fa61461027d578063d8dfeb4514610295578063e20c9f71146102a8578063fa7626d4146102b0575f5ffd5b80637a896a931461021357806380f556051461021b57806385226c8114610233578063916a17c6146102485780639f77ae501461025d578063a63c26bf14610265575f5ffd5b80633f7286f4116101105780633f7286f4146101995780635bd9e299146101a157806364cd43af146101cc57806366d9a9a0146101d4578063679e54e8146101e9578063761de19f1461020b575f5ffd5b80630a9254e41461014c5780631ed7831c146101565780632ade38801461017457806333595c46146101895780633e5e3c2314610191575b5f5ffd5b6101546102bd565b005b61015e6103ee565b60405161016b9190611b5f565b60405180910390f35b61017c61044e565b60405161016b9190611bed565b61015461058a565b61015e610dea565b61015e610e48565b6021546101b4906001600160a01b031681565b6040516001600160a01b03909116815260200161016b565b610154610ea6565b6101dc61108a565b60405161016b9190611cfa565b6101fd5f516020613ab95f395f51905f5281565b60405190815260200161016b565b6101b4600181565b6101546111ee565b601f546101b49061010090046001600160a01b031681565b61023b611340565b60405161016b9190611d78565b61025061140b565b60405161016b9190611dcf565b6101fd600281565b6101546114ec565b61025061169e565b61023b61177f565b61028561184a565b604051901515815260200161016b565b6020546101b4906001600160a01b031681565b61015e6118e6565b601f546102859060ff1681565b6040516102c990611b38565b6040808252600a90820152692a32b9ba102a37b5b2b760b11b606082015260806020820181905260049082015263151154d560e21b60a082015260c001604051809103905ff08015801561031f573d5f5f3e3d5ffd5b50602080546001600160a01b0319166001600160a01b039290921691909117905560405161034c90611b45565b604051809103905ff080158015610365573d5f5f3e3d5ffd5b50602180546001600160a01b0319166001600160a01b039290921691821790556040516001919061039590611b52565b6001600160a01b03928316815291166020820152604001604051809103905ff0801580156103c5573d5f5f3e3d5ffd5b50601f60016101000a8154816001600160a01b0302191690836001600160a01b03160217905550565b6060601680548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610426575b5050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610581575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561056a578382905f5260205f200180546104df90611e46565b80601f016020809104026020016040519081016040528092919081815260200182805461050b90611e46565b80156105565780601f1061052d57610100808354040283529160200191610556565b820191905f5260205f20905b81548152906001019060200180831161053957829003601f168201915b5050505050815260200190600101906104c2565b505050508152505081526020019060010190610471565b50505050905090565b601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c305710926105d5929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b1580156105ec575f5ffd5b505af11580156105fe573d5f5f3e3d5ffd5b5050602154601f54604051634296357160e11b81525f94506001600160a01b03928316935063852c6ae29261064e92610100900416905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015610669573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061068d9190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526001858701908152865192830193909352519481019490945251908301529192505f9060800160408051601f19818403018152828201909152600c82526b28bab2b9ba34b7b71024a21d60a11b6020830152915061071c905f516020613ab95f395f51905f52611944565b60408051808201909152600d81526c21b7b73234ba34b7b71024a21d60991b602082015261074a9084611944565b6107786040518060400160405280600881526020016727baba31b7b6b29d60c11b8152508360400151611944565b5f818051906020012090505f816040516020016107c191907f19457468657265756d205369676e6564204d6573736167653a0a3332000000008152601c810191909152603c0190565b60408051601f198184030181528282528051602091820120838301909252600d83526c26b2b9b9b0b3b2902430b9b41d60991b9083015291506108049083611944565b60408051808201909152601881527f457468205369676e6564204d65737361676520486173683a000000000000000060208201526108429082611944565b6040516338d07aa960e21b815260016004820152602481018290525f90819081905f516020613a995f395f51905f529063e341eaa490604401606060405180830381865afa158015610896573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108ba9190611eb6565b60408051602081018490529081018290526001600160f81b031960f885901b16606082015292955090935091505f9060610160408051808303601f1901815282820182528983526020830181905290516001625e79b760e01b03198152600160048201529092505f905f516020613a995f395f51905f529063ffa1864990602401602060405180830381865afa158015610956573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061097a9190611eef565b90506109ad6040518060400160405280600f81526020016e29b4b3b732b91030b2323932b9b99d60891b8152508261198d565b6040516001600160a01b03821660248201525f516020613a995f395f51905f529063b96213e49060019060440160408051601f19818403018152918152602080830180516001600160e01b0316636f1e853360e01b1790528151600191810191909152016040516020818303038152906040526040518463ffffffff1660e01b8152600401610a3e93929190611f1c565b5f604051808303815f87803b158015610a55575f5ffd5b505af1158015610a67573d5f5f3e3d5ffd5b505060405163ca669fa760e01b81526001600160a01b03841660048201525f516020613a995f395f51905f52925063ca669fa791506024015f604051808303815f87803b158015610ab6575f5ffd5b505af1158015610ac8573d5f5f3e3d5ffd5b505060405163248e63e160e11b8152600160048201819052602482018190525f604483015260648201525f516020613a995f395f51905f52925063491cc7c291506084015f604051808303815f87803b158015610b23575f5ffd5b505af1158015610b35573d5f5f3e3d5ffd5b505050508a5f516020613ab95f395f51905f527f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d16001604051610b7a91815260200190565b60405180910390a3601f54604051639aa9fda560e01b81526101009091046001600160a01b031690639aa9fda590610bb6908590600401611f5b565b5f604051808303815f87803b158015610bcd575f5ffd5b505af1925050508015610bde575060015b610ca057610bea611f9c565b806308c379a003610c4e5750610bfe611fed565b80610c095750610c50565b610c486040518060400160405280601e81526020017f6164645061796c6f6164206661696c6564207769746820726561736f6e3a0000815250826119d2565b50610cd5565b505b3d808015610c79576040519150601f19603f3d011682016040523d82523d5f602084013e610c7e565b606091505b50610c48604051806060016040528060268152602001613ad960269139611a17565b610cd56040518060400160405280601481526020017318591914185e5b1bd859081cdd58d8d95959195960621b815250611a17565b601f54604051638c93344f60e01b81525f516020613ab95f395f51905f526004820152610d589161010090046001600160a01b031690638c93344f90602401602060405180830381865afa158015610d2f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d539190612071565b611a5d565b601f5460405163ce1f456760e01b81525f516020613ab95f395f51905f526004820152610ddd9161010090046001600160a01b03169063ce1f456790602401602060405180830381865afa158015610db2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd69190611e9f565b6001611ab2565b5050505050505050505050565b6060601880548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b610eae61058a565b602154601f54604051634296357160e11b81525f926001600160a01b039081169263852c6ae292610efd92610100909204909116905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015610f18573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f3c9190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526001858701908152865192830193909352519481019490945251908301529192505f906080015b60408051808303601f1901815260418084526080840190925292505f91906020820181803683370190505060408051808201825284815260208101839052905163ca669fa760e01b81526101236004820152919250905f516020613a995f395f51905f529063ca669fa7906024015f604051808303815f87803b15801561100c575f5ffd5b505af115801561101e573d5f5f3e3d5ffd5b5050601f54604051639aa9fda560e01b81526101009091046001600160a01b03169250639aa9fda59150611056908490600401611f5b565b5f604051808303815f87803b15801561106d575f5ffd5b505af115801561107f573d5f5f3e3d5ffd5b505050505050505050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f2090600202016040518060400160405290815f820180546110dd90611e46565b80601f016020809104026020016040519081016040528092919081815260200182805461110990611e46565b80156111545780601f1061112b57610100808354040283529160200191611154565b820191905f5260205f20905b81548152906001019060200180831161113757829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156111d657602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116111985790505b505050505081525050815260200190600101906110ad565b601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c30571092611239929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b158015611250575f5ffd5b505af1158015611262573d5f5f3e3d5ffd5b5050602154601f54604051634296357160e11b81525f94506001600160a01b03928316935063852c6ae2926112b292610100900416905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa1580156112cd573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112f19190611e9f565b60408051606080820183525f516020613ab95f395f51905f5280835260208084018681526002858701908152865192830193909352519481019490945251908301529192505f90608001610f87565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f2001805461138090611e46565b80601f01602080910402602001604051908101604052809291908181526020018280546113ac90611e46565b80156113f75780601f106113ce576101008083540402835291602001916113f7565b820191905f5260205f20905b8154815290600101906020018083116113da57829003601f168201915b505050505081526020019060010190611363565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610581575f8481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114d457602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114965790505b5050505050815250508152602001906001019061142e565b602154601f54604051634296357160e11b81525f926001600160a01b039081169263852c6ae29261153b92610100909204909116905f516020613ab95f395f51905f5290600290600401611e7e565b602060405180830381865afa158015611556573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061157a9190611e9f565b60405163248e63e160e11b8152600160048201819052602482018190525f604483015260648201529091505f516020613a995f395f51905f529063491cc7c2906084015f604051808303815f87803b1580156115d4575f5ffd5b505af11580156115e6573d5f5f3e3d5ffd5b50506040518392505f516020613ab95f395f51905f5291507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a3601f546020546040516304c3057160e41b81526001600160a01b03610100909304831692634c3057109261166e929116905f516020613ab95f395f51905f5290600290600401611e7e565b5f604051808303815f87803b158015611685575f5ffd5b505af1158015611697573d5f5f3e3d5ffd5b5050505050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610581575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561176757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116117295790505b505050505081525050815260200190600101906116c1565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610581578382905f5260205f200180546117bf90611e46565b80601f01602080910402602001604051908101604052809291908181526020018280546117eb90611e46565b80156118365780601f1061180d57610100808354040283529160200191611836565b820191905f5260205f20905b81548152906001019060200180831161181957829003601f168201915b5050505050815260200190600101906117a2565b6008545f9060ff1615611861575060085460ff1690565b604051630667f9d760e41b81525f516020613a995f395f51905f52600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa1580156118b8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906118dc9190611e9f565b1415905090565b90565b6060601580548060200260200160405190810160405280929190818152602001828054801561044457602002820191905f5260205f209081546001600160a01b03168152600190910190602001808311610426575050505050905090565b611989828260405160240161195a929190612090565b60408051601f198184030181529190526020810180516001600160e01b0316632d839cb360e21b179052611b15565b5050565b61198982826040516024016119a39291906120b1565b60408051601f198184030181529190526020810180516001600160e01b031663319af33360e01b179052611b15565b61198982826040516024016119e89291906120da565b60408051601f198184030181529190526020810180516001600160e01b0316634b5c427760e01b179052611b15565b611a5a81604051602401611a2b91906120fe565b60408051601f198184030181529190526020810180516001600160e01b031663104c13eb60e21b179052611b15565b50565b604051630c9fd58160e01b815281151560048201525f516020613a995f395f51905f5290630c9fd581906024015f6040518083038186803b158015611aa0575f5ffd5b505afa158015611697573d5f5f3e3d5ffd5b60405163260a5b1560e21b815260048101839052602481018290525f516020613a995f395f51905f52906398296c54906044015f6040518083038186803b158015611afb575f5ffd5b505afa158015611b0d573d5f5f3e3d5ffd5b505050505050565b611a5a815f6a636f6e736f6c652e6c6f6790505f5f835160208501845afa505050565b610a108061211183390190565b61034880612b2183390190565b610c3080612e6983390190565b602080825282518282018190525f918401906040840190835b81811015611b9f5783516001600160a01b0316835260209384019390920191600101611b78565b509095945050505050565b5f81518084525f5b81811015611bce57602081850181015186830182015201611bb2565b505f602082860101526020601f19601f83011685010191505092915050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f19878603018452815180516001600160a01b03168652602090810151604082880181905281519088018190529101906060600582901b8801810191908801905f5b81811015611c9057605f198a8503018352611c7a848651611baa565b6020958601959094509290920191600101611c5e565b509197505050602094850194929092019150600101611c13565b50929695505050505050565b5f8151808452602084019350602083015f5b82811015611cf05781516001600160e01b031916865260209586019590910190600101611cc8565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f198786030184528151805160408752611d466040880182611baa565b9050602082015191508681036020880152611d618183611cb6565b965050506020938401939190910190600101611d20565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57603f19878603018452611dba858351611baa565b94506020938401939190910190600101611d9e565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015611caa57868503603f19018452815180516001600160a01b03168652602090810151604091870182905290611e3090870182611cb6565b9550506020938401939190910190600101611df5565b600181811c90821680611e5a57607f821691505b602082108103611e7857634e487b7160e01b5f52602260045260245ffd5b50919050565b6001600160a01b039390931683526020830191909152604082015260600190565b5f60208284031215611eaf575f5ffd5b5051919050565b5f5f5f60608486031215611ec8575f5ffd5b835160ff81168114611ed8575f5ffd5b602085015160409095015190969495509392505050565b5f60208284031215611eff575f5ffd5b81516001600160a01b0381168114611f15575f5ffd5b9392505050565b6001600160a01b03841681526060602082018190525f90611f3f90830185611baa565b8281036040840152611f518185611baa565b9695505050505050565b602081525f825160406020840152611f766060840182611baa565b90506020840151601f19848303016040850152611f938282611baa565b95945050505050565b5f60033d11156118e35760045f5f3e505f5160e01c90565b601f8201601f1916810167ffffffffffffffff81118282101715611fe657634e487b7160e01b5f52604160045260245ffd5b6040525050565b5f60443d1015611ffa5790565b6040513d600319016004823e80513d602482011167ffffffffffffffff8211171561202457505090565b808201805167ffffffffffffffff811115612040575050505090565b3d840160031901828201602001111561205a575050505090565b61206960208285010185611fb4565b509392505050565b5f60208284031215612081575f5ffd5b81518015158114611f15575f5ffd5b604081525f6120a26040830185611baa565b90508260208301529392505050565b604081525f6120c36040830185611baa565b905060018060a01b03831660208301529392505050565b604081525f6120ec6040830185611baa565b8281036020840152611f938185611baa565b602081525f611f156020830184611baa56fe608060405234801561000f575f5ffd5b50604051610a10380380610a1083398101604081905261002e91610105565b8181600361003c83826101ee565b50600461004982826101ee565b50505050506102a8565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112610076575f5ffd5b81516001600160401b0381111561008f5761008f610053565b604051601f8201601f19908116603f011681016001600160401b03811182821017156100bd576100bd610053565b6040528181528382016020018510156100d4575f5ffd5b5f5b828110156100f2576020818601810151838301820152016100d6565b505f918101602001919091529392505050565b5f5f60408385031215610116575f5ffd5b82516001600160401b0381111561012b575f5ffd5b61013785828601610067565b602085015190935090506001600160401b03811115610154575f5ffd5b61016085828601610067565b9150509250929050565b600181811c9082168061017e57607f821691505b60208210810361019c57634e487b7160e01b5f52602260045260245ffd5b50919050565b601f8211156101e957805f5260205f20601f840160051c810160208510156101c75750805b601f840160051c820191505b818110156101e6575f81556001016101d3565b50505b505050565b81516001600160401b0381111561020757610207610053565b61021b81610215845461016a565b846101a2565b6020601f82116001811461024d575f83156102365750848201515b5f19600385901b1c1916600184901b1784556101e6565b5f84815260208120601f198516915b8281101561027c578785015182556020948501946001909201910161025c565b508482101561029957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b61075b806102b55f395ff3fe608060405234801561000f575f5ffd5b506004361061009b575f3560e01c806340c10f191161006357806340c10f191461011457806370a082311461012957806395d89b4114610151578063a9059cbb14610159578063dd62ed3e1461016c575f5ffd5b806306fdde031461009f578063095ea7b3146100bd57806318160ddd146100e057806323b872dd146100f2578063313ce56714610105575b5f5ffd5b6100a76101a4565b6040516100b491906105b5565b60405180910390f35b6100d06100cb36600461061b565b610234565b60405190151581526020016100b4565b6002545b6040519081526020016100b4565b6100d0610100366004610643565b61024d565b604051601281526020016100b4565b61012761012236600461061b565b610270565b005b6100e461013736600461067d565b6001600160a01b03165f9081526020819052604090205490565b6100a761027e565b6100d061016736600461061b565b61028d565b6100e461017a36600461069d565b6001600160a01b039182165f90815260016020908152604080832093909416825291909152205490565b6060600380546101b3906106ce565b80601f01602080910402602001604051908101604052809291908181526020018280546101df906106ce565b801561022a5780601f106102015761010080835404028352916020019161022a565b820191905f5260205f20905b81548152906001019060200180831161020d57829003601f168201915b5050505050905090565b5f3361024181858561029a565b60019150505b92915050565b5f3361025a8582856102ac565b61026585858561032c565b506001949350505050565b61027a8282610389565b5050565b6060600480546101b3906106ce565b5f3361024181858561032c565b6102a783838360016103bd565b505050565b6001600160a01b038381165f908152600160209081526040808320938616835292905220545f198114610326578181101561031857604051637dc7a0d960e11b81526001600160a01b038416600482015260248101829052604481018390526064015b60405180910390fd5b61032684848484035f6103bd565b50505050565b6001600160a01b03831661035557604051634b637e8f60e11b81525f600482015260240161030f565b6001600160a01b03821661037e5760405163ec442f0560e01b81525f600482015260240161030f565b6102a783838361048f565b6001600160a01b0382166103b25760405163ec442f0560e01b81525f600482015260240161030f565b61027a5f838361048f565b6001600160a01b0384166103e65760405163e602df0560e01b81525f600482015260240161030f565b6001600160a01b03831661040f57604051634a1406b160e11b81525f600482015260240161030f565b6001600160a01b038085165f908152600160209081526040808320938716835292905220829055801561032657826001600160a01b0316846001600160a01b03167f8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b9258460405161048191815260200190565b60405180910390a350505050565b6001600160a01b0383166104b9578060025f8282546104ae9190610706565b909155506105299050565b6001600160a01b0383165f908152602081905260409020548181101561050b5760405163391434e360e21b81526001600160a01b0385166004820152602481018290526044810183905260640161030f565b6001600160a01b0384165f9081526020819052604090209082900390555b6001600160a01b03821661054557600280548290039055610563565b6001600160a01b0382165f9081526020819052604090208054820190555b816001600160a01b0316836001600160a01b03167fddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef836040516105a891815260200190565b60405180910390a3505050565b602081525f82518060208401525f5b818110156105e157602081860181015160408684010152016105c4565b505f604082850101526040601f19601f83011684010191505092915050565b80356001600160a01b0381168114610616575f5ffd5b919050565b5f5f6040838503121561062c575f5ffd5b61063583610600565b946020939093013593505050565b5f5f5f60608486031215610655575f5ffd5b61065e84610600565b925061066c60208501610600565b929592945050506040919091013590565b5f6020828403121561068d575f5ffd5b61069682610600565b9392505050565b5f5f604083850312156106ae575f5ffd5b6106b783610600565b91506106c560208401610600565b90509250929050565b600181811c908216806106e257607f821691505b60208210810361070057634e487b7160e01b5f52602260045260245ffd5b50919050565b8082018082111561024757634e487b7160e01b5f52601160045260245ffdfea2646970667358221220fe1dfb04b261515d783c42c37727c6a547db0da7c9b24141248edff106da93be64736f6c634300081c00336080604052348015600e575f5ffd5b5061032c8061001c5f395ff3fe608060405234801561000f575f5ffd5b506004361061004a575f3560e01c806301b7037c1461004e578063852c6ae214610065578063c49298ac146100d3578063d96ee754146100e6575b5f5ffd5b61006361005c3660046101e4565b5050505050565b005b6100c1610073366004610247565b6040516bffffffffffffffffffffffff19606085901b16602082015260348101839052605481018290525f906074016040516020818303038152906040528051906020012090509392505050565b60405190815260200160405180910390f35b6100636100e1366004610277565b6100f9565b6100636100f4366004610247565b610138565b827ffbd3494747d87a8762583f3697021d2eae7789c823492b672da727715c3cb367838360405161012b9291906102bf565b60405180910390a2505050565b81836001600160a01b03167f97404ccd19300d892cb8813ac95c441f995c0db13e2b71b3661afabf9385bc818360405161017491815260200190565b60405180910390a3505050565b80356001600160a01b0381168114610197575f5ffd5b919050565b5f5f83601f8401126101ac575f5ffd5b50813567ffffffffffffffff8111156101c3575f5ffd5b6020830191508360208260051b85010111156101dd575f5ffd5b9250929050565b5f5f5f5f5f608086880312156101f8575f5ffd5b61020186610181565b94506020860135935060408601359250606086013567ffffffffffffffff81111561022a575f5ffd5b6102368882890161019c565b969995985093965092949392505050565b5f5f5f60608486031215610259575f5ffd5b61026284610181565b95602085013595506040909401359392505050565b5f5f5f60408486031215610289575f5ffd5b83359250602084013567ffffffffffffffff8111156102a6575f5ffd5b6102b28682870161019c565b9497909650939450505050565b602080825281018290525f6001600160fb1b038311156102dd575f5ffd5b8260051b8085604085013791909101604001939250505056fea264697066735822122064d4850aebfe59e4c2100896cf62961476b078f4a33a9b2316864eb020ccd4f964736f6c634300081c003360c060405234801561000f575f5ffd5b50604051610c30380380610c3083398101604081905261002e91610060565b6001600160a01b039182166080521660a052610091565b80516001600160a01b038116811461005b575f5ffd5b919050565b5f5f60408385031215610071575f5ffd5b61007a83610045565b915061008860208401610045565b90509250929050565b60805160a051610b636100cd5f395f818160d60152818161031a015281816103b1015261067201525f8181610128015261024e0152610b635ff3fe608060405234801561000f575f5ffd5b5060043610610090575f3560e01c8063761de19f11610063578063761de19f146101235780638c93344f1461014a5780639aa9fda51461016c578063a71f8da01461017f578063ce1f456714610192575f5ffd5b8063216a3e9a146100945780634c305710146100bc5780635bd9e299146100d157806363710c0514610110575b5f5ffd5b6100a76100a236600461077f565b6101bf565b60405190151581526020015b60405180910390f35b6100cf6100ca3660046107bd565b6102f5565b005b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100b3565b6100cf61011e3660046107fa565b610442565b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6100a761015836600461086b565b5f6020819052908152604090205460ff1681565b6100cf61017a36600461077f565b6104c6565b6100a761018d3660046107fa565b6104fa565b6101b16101a036600461086b565b60016020525f908152604090205481565b6040519081526020016100b3565b5f806101cb8380610882565b6040516101d99291906108cc565b604051809103902090505f61023a826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836102816020890189610882565b6040518463ffffffff1660e01b815260040161029f93929190610903565b602060405180830381865afa1580156102ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102de9190610925565b6001600160e01b0319918216911614949350505050565b604051634296357160e11b815230600482015260248101839052604481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063852c6ae290606401602060405180830381865afa158015610367573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038b919061094c565b60405163365bb9d560e21b815230600482015260248101859052604481018490529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d96ee754906064015f604051808303815f87803b1580156103fa575f5ffd5b505af115801561040c573d5f5f3e3d5ffd5b50506040518392508591507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a350505050565b818161044e82826104fa565b61046b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156104bf576104ad85858363ffffffff1681811061049657610496610963565b90506020028101906104a89190610977565b6105c1565b806104b781610995565b91505061046d565b5050505050565b806104d0816101bf565b6104ed57604051638baa579f60e01b815260040160405180910390fd5b6104f6826105c1565b5050565b5f805b63ffffffff81168311156105b5573063216a3e9a858563ffffffff851681811061052957610529610963565b905060200281019061053b9190610977565b6040518263ffffffff1660e01b81526004016105579190610a07565b602060405180830381865afa158015610572573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105969190610a5c565b6105a3575f9150506105bb565b806105ad81610995565b9150506104fd565b50600190505b92915050565b5f6105cc8280610882565b8101906105d99190610a7b565b80515f9081526020819052604090205490915060ff161561060d5760405163aa43cb2d60e01b815260040160405180910390fd5b6001816040015111156106335760405163c74a206d60e01b815260040160405180910390fd5b80515f90815260208181526040808320805460ff1916600190811790915581850180518651865291909352922091909155815190516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163c49298ac916106a290610736565b6040518363ffffffff1660e01b81526004016106bf929190610ae0565b5f604051808303815f87803b1580156106d6575f5ffd5b505af11580156106e8573d5f5f3e3d5ffd5b505050508060200151815f01517f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d1836040015160405161072a91815260200190565b60405180910390a35050565b60408051600280825260608083018452925f929190602083019080368337019050509050600181848151811061076e5761076e610963565b602090810291909101015292915050565b5f6020828403121561078f575f5ffd5b813567ffffffffffffffff8111156107a5575f5ffd5b8201604081850312156107b6575f5ffd5b9392505050565b5f5f5f606084860312156107cf575f5ffd5b83356001600160a01b03811681146107e5575f5ffd5b95602085013595506040909401359392505050565b5f5f6020838503121561080b575f5ffd5b823567ffffffffffffffff811115610821575f5ffd5b8301601f81018513610831575f5ffd5b803567ffffffffffffffff811115610847575f5ffd5b8560208260051b840101111561085b575f5ffd5b6020919091019590945092505050565b5f6020828403121561087b575f5ffd5b5035919050565b5f5f8335601e19843603018112610897575f5ffd5b83018035915067ffffffffffffffff8211156108b1575f5ffd5b6020019150368190038213156108c5575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61091c6040830184866108db565b95945050505050565b5f60208284031215610935575f5ffd5b81516001600160e01b0319811681146107b6575f5ffd5b5f6020828403121561095c575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261098b575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff81036109bc57634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e198436030181126109da575f5ffd5b830160208101925035905067ffffffffffffffff8111156109f9575f5ffd5b8036038213156108c5575f5ffd5b602081525f610a1683846109c5565b60406020850152610a2b6060850182846108db565b915050610a3b60208501856109c5565b848303601f19016040860152610a528382846108db565b9695505050505050565b5f60208284031215610a6c575f5ffd5b815180151581146107b6575f5ffd5b5f6060828403128015610a8c575f5ffd5b506040516060810167ffffffffffffffff81118282101715610abc57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b5f60408201848352604060208401528084518083526060850191506020860192505f5b81811015610b21578351835260209384019390920191600101610b03565b5090969550505050505056fea26469706673582212207a5dc298a257036afb5a67be155c7a7ab06a20bc1df591976b60bfd43121f3da64736f6c634300081c00330000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12d591eed45d83ee01c18f61af559cbb698bc6d377a62748889af9cd67b0e69e2696164645061796c6f6164206661696c65642077697468206c6f772d6c6576656c206572726f72a2646970667358221220ad4297ee0ee6eeca5361dd2af6e09be93f1f187d8b6e9643678239805c582dd364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01HW_5`\xE0\x1C\x80cz\x89j\x93\x11a\0\xBFW\x80c\xB0FO\xDC\x11a\0yW\x80c\xB0FO\xDC\x14a\x02mW\x80c\xB5P\x8A\xA9\x14a\x02uW\x80c\xBAAO\xA6\x14a\x02}W\x80c\xD8\xDF\xEBE\x14a\x02\x95W\x80c\xE2\x0C\x9Fq\x14a\x02\xA8W\x80c\xFAv&\xD4\x14a\x02\xB0W__\xFD[\x80cz\x89j\x93\x14a\x02\x13W\x80c\x80\xF5V\x05\x14a\x02\x1BW\x80c\x85\"l\x81\x14a\x023W\x80c\x91j\x17\xC6\x14a\x02HW\x80c\x9Fw\xAEP\x14a\x02]W\x80c\xA6<&\xBF\x14a\x02eW__\xFD[\x80c?r\x86\xF4\x11a\x01\x10W\x80c?r\x86\xF4\x14a\x01\x99W\x80c[\xD9\xE2\x99\x14a\x01\xA1W\x80cd\xCDC\xAF\x14a\x01\xCCW\x80cf\xD9\xA9\xA0\x14a\x01\xD4W\x80cg\x9ET\xE8\x14a\x01\xE9W\x80cv\x1D\xE1\x9F\x14a\x02\x0BW__\xFD[\x80c\n\x92T\xE4\x14a\x01LW\x80c\x1E\xD7\x83\x1C\x14a\x01VW\x80c*\xDE8\x80\x14a\x01tW\x80c3Y\\F\x14a\x01\x89W\x80c>^<#\x14a\x01\x91W[__\xFD[a\x01Ta\x02\xBDV[\0[a\x01^a\x03\xEEV[`@Qa\x01k\x91\x90a\x1B_V[`@Q\x80\x91\x03\x90\xF3[a\x01|a\x04NV[`@Qa\x01k\x91\x90a\x1B\xEDV[a\x01Ta\x05\x8AV[a\x01^a\r\xEAV[a\x01^a\x0EHV[`!Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01kV[a\x01Ta\x0E\xA6V[a\x01\xDCa\x10\x8AV[`@Qa\x01k\x91\x90a\x1C\xFAV[a\x01\xFD_Q` a:\xB9_9_Q\x90_R\x81V[`@Q\x90\x81R` \x01a\x01kV[a\x01\xB4`\x01\x81V[a\x01Ta\x11\xEEV[`\x1FTa\x01\xB4\x90a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02;a\x13@V[`@Qa\x01k\x91\x90a\x1DxV[a\x02Pa\x14\x0BV[`@Qa\x01k\x91\x90a\x1D\xCFV[a\x01\xFD`\x02\x81V[a\x01Ta\x14\xECV[a\x02Pa\x16\x9EV[a\x02;a\x17\x7FV[a\x02\x85a\x18JV[`@Q\x90\x15\x15\x81R` \x01a\x01kV[` Ta\x01\xB4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01^a\x18\xE6V[`\x1FTa\x02\x85\x90`\xFF\x16\x81V[`@Qa\x02\xC9\x90a\x1B8V[`@\x80\x82R`\n\x90\x82\x01Ri*2\xB9\xBA\x10*7\xB5\xB2\xB7`\xB1\x1B``\x82\x01R`\x80` \x82\x01\x81\x90R`\x04\x90\x82\x01Rc\x15\x11T\xD5`\xE2\x1B`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03\x1FW=__>=_\xFD[P` \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`@Qa\x03L\x90a\x1BEV[`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03eW=__>=_\xFD[P`!\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x90U`@Q`\x01\x91\x90a\x03\x95\x90a\x1BRV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15a\x03\xC5W=__>=_\xFD[P`\x1F`\x01a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&W[PPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x05jW\x83\x82\x90_R` _ \x01\x80Ta\x04\xDF\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05\x0B\x90a\x1EFV[\x80\x15a\x05VW\x80`\x1F\x10a\x05-Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05VV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x059W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x04\xC2V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x04qV[PPPP\x90P\x90V[`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x05\xD5\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xECW__\xFD[PZ\xF1\x15\x80\x15a\x05\xFEW=__>=_\xFD[PP`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x85,j\xE2\x92a\x06N\x92a\x01\0\x90\x04\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x8D\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x01\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82\x01\x90\x91R`\x0C\x82Rk(\xBA\xB2\xB9\xBA4\xB7\xB7\x10$\xA2\x1D`\xA1\x1B` \x83\x01R\x91Pa\x07\x1C\x90_Q` a:\xB9_9_Q\x90_Ra\x19DV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81Rl!\xB7\xB724\xBA4\xB7\xB7\x10$\xA2\x1D`\x99\x1B` \x82\x01Ra\x07J\x90\x84a\x19DV[a\x07x`@Q\x80`@\x01`@R\x80`\x08\x81R` \x01g'\xBA\xBA1\xB7\xB6\xB2\x9D`\xC1\x1B\x81RP\x83`@\x01Qa\x19DV[_\x81\x80Q\x90` \x01 \x90P_\x81`@Q` \x01a\x07\xC1\x91\x90\x7F\x19Ethereum Signed Message:\n32\0\0\0\0\x81R`\x1C\x81\x01\x91\x90\x91R`<\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x83\x83\x01\x90\x92R`\r\x83Rl&\xB2\xB9\xB9\xB0\xB3\xB2\x90$0\xB9\xB4\x1D`\x99\x1B\x90\x83\x01R\x91Pa\x08\x04\x90\x83a\x19DV[`@\x80Q\x80\x82\x01\x90\x91R`\x18\x81R\x7FEth Signed Message Hash:\0\0\0\0\0\0\0\0` \x82\x01Ra\x08B\x90\x82a\x19DV[`@Qc8\xD0z\xA9`\xE2\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x82\x90R_\x90\x81\x90\x81\x90_Q` a:\x99_9_Q\x90_R\x90c\xE3A\xEA\xA4\x90`D\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x96W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBA\x91\x90a\x1E\xB6V[`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R`\x01`\x01`\xF8\x1B\x03\x19`\xF8\x85\x90\x1B\x16``\x82\x01R\x92\x95P\x90\x93P\x91P_\x90`a\x01`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x82\x82\x01\x82R\x89\x83R` \x83\x01\x81\x90R\x90Q`\x01b^y\xB7`\xE0\x1B\x03\x19\x81R`\x01`\x04\x82\x01R\x90\x92P_\x90_Q` a:\x99_9_Q\x90_R\x90c\xFF\xA1\x86I\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tz\x91\x90a\x1E\xEFV[\x90Pa\t\xAD`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n)\xB4\xB3\xB72\xB9\x100\xB2292\xB9\xB9\x9D`\x89\x1B\x81RP\x82a\x19\x8DV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_Q` a:\x99_9_Q\x90_R\x90c\xB9b\x13\xE4\x90`\x01\x90`D\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x80\x83\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16co\x1E\x853`\xE0\x1B\x17\x90R\x81Q`\x01\x91\x81\x01\x91\x90\x91R\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n>\x93\x92\x91\x90a\x1F\x1CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\nUW__\xFD[PZ\xF1\x15\x80\x15a\ngW=__>=_\xFD[PP`@Qc\xCAf\x9F\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R_Q` a:\x99_9_Q\x90_R\x92Pc\xCAf\x9F\xA7\x91P`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xB6W__\xFD[PZ\xF1\x15\x80\x15a\n\xC8W=__>=_\xFD[PP`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R_`D\x83\x01R`d\x82\x01R_Q` a:\x99_9_Q\x90_R\x92PcI\x1C\xC7\xC2\x91P`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B#W__\xFD[PZ\xF1\x15\x80\x15a\x0B5W=__>=_\xFD[PPPP\x8A_Q` a:\xB9_9_Q\x90_R\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1`\x01`@Qa\x0Bz\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x1FT`@Qc\x9A\xA9\xFD\xA5`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x9A\xA9\xFD\xA5\x90a\x0B\xB6\x90\x85\x90`\x04\x01a\x1F[V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xCDW__\xFD[PZ\xF1\x92PPP\x80\x15a\x0B\xDEWP`\x01[a\x0C\xA0Wa\x0B\xEAa\x1F\x9CV[\x80c\x08\xC3y\xA0\x03a\x0CNWPa\x0B\xFEa\x1F\xEDV[\x80a\x0C\tWPa\x0CPV[a\x0CH`@Q\x80`@\x01`@R\x80`\x1E\x81R` \x01\x7FaddPayload failed with reason:\0\0\x81RP\x82a\x19\xD2V[Pa\x0C\xD5V[P[=\x80\x80\x15a\x0CyW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C~V[``\x91P[Pa\x0CH`@Q\x80``\x01`@R\x80`&\x81R` \x01a:\xD9`&\x919a\x1A\x17V[a\x0C\xD5`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x18Y\x19\x14\x18^[\x1B\xD8Y\x08\x1C\xDDX\xD8\xD9YY\x19Y`b\x1B\x81RPa\x1A\x17V[`\x1FT`@Qc\x8C\x934O`\xE0\x1B\x81R_Q` a:\xB9_9_Q\x90_R`\x04\x82\x01Ra\rX\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\x8C\x934O\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r/W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rS\x91\x90a qV[a\x1A]V[`\x1FT`@Qc\xCE\x1FEg`\xE0\x1B\x81R_Q` a:\xB9_9_Q\x90_R`\x04\x82\x01Ra\r\xDD\x91a\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x1FEg\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD6\x91\x90a\x1E\x9FV[`\x01a\x1A\xB2V[PPPPPPPPPPPV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[a\x0E\xAEa\x05\x8AV[`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\x85,j\xE2\x92a\x0E\xFD\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x18W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F<\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x01\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R`A\x80\x84R`\x80\x84\x01\x90\x92R\x92P_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP`@\x80Q\x80\x82\x01\x82R\x84\x81R` \x81\x01\x83\x90R\x90Qc\xCAf\x9F\xA7`\xE0\x1B\x81Ra\x01#`\x04\x82\x01R\x91\x92P\x90_Q` a:\x99_9_Q\x90_R\x90c\xCAf\x9F\xA7\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\x0CW__\xFD[PZ\xF1\x15\x80\x15a\x10\x1EW=__>=_\xFD[PP`\x1FT`@Qc\x9A\xA9\xFD\xA5`\xE0\x1B\x81Ra\x01\0\x90\x91\x04`\x01`\x01`\xA0\x1B\x03\x16\x92Pc\x9A\xA9\xFD\xA5\x91Pa\x10V\x90\x84\x90`\x04\x01a\x1F[V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10mW__\xFD[PZ\xF1\x15\x80\x15a\x10\x7FW=__>=_\xFD[PPPPPPPPPV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x10\xDD\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\t\x90a\x1EFV[\x80\x15a\x11TW\x80`\x1F\x10a\x11+Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11TV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x117W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x11\xD6W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11\x98W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x10\xADV[`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x129\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12PW__\xFD[PZ\xF1\x15\x80\x15a\x12bW=__>=_\xFD[PP`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x94P`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x93Pc\x85,j\xE2\x92a\x12\xB2\x92a\x01\0\x90\x04\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xCDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xF1\x91\x90a\x1E\x9FV[`@\x80Q``\x80\x82\x01\x83R_Q` a:\xB9_9_Q\x90_R\x80\x83R` \x80\x84\x01\x86\x81R`\x02\x85\x87\x01\x90\x81R\x86Q\x92\x83\x01\x93\x90\x93RQ\x94\x81\x01\x94\x90\x94RQ\x90\x83\x01R\x91\x92P_\x90`\x80\x01a\x0F\x87V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x01\x80Ta\x13\x80\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xAC\x90a\x1EFV[\x80\x15a\x13\xF7W\x80`\x1F\x10a\x13\xCEWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xF7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xDAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x13cV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xD4W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\x96W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14.V[`!T`\x1FT`@QcB\x965q`\xE1\x1B\x81R_\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92c\x85,j\xE2\x92a\x15;\x92a\x01\0\x90\x92\x04\x90\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15z\x91\x90a\x1E\x9FV[`@Qc$\x8Ec\xE1`\xE1\x1B\x81R`\x01`\x04\x82\x01\x81\x90R`$\x82\x01\x81\x90R_`D\x83\x01R`d\x82\x01R\x90\x91P_Q` a:\x99_9_Q\x90_R\x90cI\x1C\xC7\xC2\x90`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xD4W__\xFD[PZ\xF1\x15\x80\x15a\x15\xE6W=__>=_\xFD[PP`@Q\x83\x92P_Q` a:\xB9_9_Q\x90_R\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3`\x1FT` T`@Qc\x04\xC3\x05q`\xE4\x1B\x81R`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x93\x04\x83\x16\x92cL0W\x10\x92a\x16n\x92\x91\x16\x90_Q` a:\xB9_9_Q\x90_R\x90`\x02\x90`\x04\x01a\x1E~V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x16\x85W__\xFD[PZ\xF1\x15\x80\x15a\x16\x97W=__>=_\xFD[PPPPPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x17gW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x17)W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x16\xC1V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\x81W\x83\x82\x90_R` _ \x01\x80Ta\x17\xBF\x90a\x1EFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17\xEB\x90a\x1EFV[\x80\x15a\x186W\x80`\x1F\x10a\x18\rWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x186V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x17\xA2V[`\x08T_\x90`\xFF\x16\x15a\x18aWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_Q` a:\x99_9_Q\x90_R`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xDC\x91\x90a\x1E\x9FV[\x14\x15\x90P\x90V[\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04DW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04&WPPPPP\x90P\x90V[a\x19\x89\x82\x82`@Q`$\x01a\x19Z\x92\x91\x90a \x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c-\x83\x9C\xB3`\xE2\x1B\x17\x90Ra\x1B\x15V[PPV[a\x19\x89\x82\x82`@Q`$\x01a\x19\xA3\x92\x91\x90a \xB1V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra\x1B\x15V[a\x19\x89\x82\x82`@Q`$\x01a\x19\xE8\x92\x91\x90a \xDAV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra\x1B\x15V[a\x1AZ\x81`@Q`$\x01a\x1A+\x91\x90a \xFEV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra\x1B\x15V[PV[`@Qc\x0C\x9F\xD5\x81`\xE0\x1B\x81R\x81\x15\x15`\x04\x82\x01R_Q` a:\x99_9_Q\x90_R\x90c\x0C\x9F\xD5\x81\x90`$\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xA0W__\xFD[PZ\xFA\x15\x80\x15a\x16\x97W=__>=_\xFD[`@Qc&\n[\x15`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R_Q` a:\x99_9_Q\x90_R\x90c\x98)lT\x90`D\x01_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1A\xFBW__\xFD[PZ\xFA\x15\x80\x15a\x1B\rW=__>=_\xFD[PPPPPPV[a\x1AZ\x81_jconsole.log\x90P__\x83Q` \x85\x01\x84Z\xFAPPPV[a\n\x10\x80a!\x11\x839\x01\x90V[a\x03H\x80a+!\x839\x01\x90V[a\x0C0\x80a.i\x839\x01\x90V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1B\x9FW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1BxV[P\x90\x95\x94PPPPPV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x1B\xCEW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1B\xB2V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90```\x05\x82\x90\x1B\x88\x01\x81\x01\x91\x90\x88\x01\x90_[\x81\x81\x10\x15a\x1C\x90W`_\x19\x8A\x85\x03\x01\x83Ra\x1Cz\x84\x86Qa\x1B\xAAV[` \x95\x86\x01\x95\x90\x94P\x92\x90\x92\x01\x91`\x01\x01a\x1C^V[P\x91\x97PPP` \x94\x85\x01\x94\x92\x90\x92\x01\x91P`\x01\x01a\x1C\x13V[P\x92\x96\x95PPPPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x1C\xF0W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x1C\xC8V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra\x1DF`@\x88\x01\x82a\x1B\xAAV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra\x1Da\x81\x83a\x1C\xB6V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW`?\x19\x87\x86\x03\x01\x84Ra\x1D\xBA\x85\x83Qa\x1B\xAAV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D\x9EV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x1C\xAAW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\x1E0\x90\x87\x01\x82a\x1C\xB6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x1D\xF5V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1EZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1ExWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a\x1E\xAFW__\xFD[PQ\x91\x90PV[___``\x84\x86\x03\x12\x15a\x1E\xC8W__\xFD[\x83Q`\xFF\x81\x16\x81\x14a\x1E\xD8W__\xFD[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x1E\xFFW__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1F\x15W__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R``` \x82\x01\x81\x90R_\x90a\x1F?\x90\x83\x01\x85a\x1B\xAAV[\x82\x81\x03`@\x84\x01Ra\x1FQ\x81\x85a\x1B\xAAV[\x96\x95PPPPPPV[` \x81R_\x82Q`@` \x84\x01Ra\x1Fv``\x84\x01\x82a\x1B\xAAV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra\x1F\x93\x82\x82a\x1B\xAAV[\x95\x94PPPPPV[_`\x03=\x11\x15a\x18\xE3W`\x04__>P_Q`\xE0\x1C\x90V[`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F\xE6WcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@RPPV[_`D=\x10\x15a\x1F\xFAW\x90V[`@Q=`\x03\x19\x01`\x04\x82>\x80Q=`$\x82\x01\x11g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a $WPP\x90V[\x80\x82\x01\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a @WPPPP\x90V[=\x84\x01`\x03\x19\x01\x82\x82\x01` \x01\x11\x15a ZWPPPP\x90V[a i` \x82\x85\x01\x01\x85a\x1F\xB4V[P\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \x81W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1F\x15W__\xFD[`@\x81R_a \xA2`@\x83\x01\x85a\x1B\xAAV[\x90P\x82` \x83\x01R\x93\x92PPPV[`@\x81R_a \xC3`@\x83\x01\x85a\x1B\xAAV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@\x81R_a \xEC`@\x83\x01\x85a\x1B\xAAV[\x82\x81\x03` \x84\x01Ra\x1F\x93\x81\x85a\x1B\xAAV[` \x81R_a\x1F\x15` \x83\x01\x84a\x1B\xAAV\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\n\x108\x03\x80a\n\x10\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x05V[\x81\x81`\x03a\0<\x83\x82a\x01\xEEV[P`\x04a\0I\x82\x82a\x01\xEEV[PPPPPa\x02\xA8V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0vW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\x8FWa\0\x8Fa\0SV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xBDWa\0\xBDa\0SV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xD4W__\xFD[_[\x82\x81\x10\x15a\0\xF2W` \x81\x86\x01\x81\x01Q\x83\x83\x01\x82\x01R\x01a\0\xD6V[P_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x01\x16W__\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01+W__\xFD[a\x017\x85\x82\x86\x01a\0gV[` \x85\x01Q\x90\x93P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01TW__\xFD[a\x01`\x85\x82\x86\x01a\0gV[\x91PP\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01~W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\x9CWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x01\xE9W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xC7WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xE6W_\x81U`\x01\x01a\x01\xD3V[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x07Wa\x02\x07a\0SV[a\x02\x1B\x81a\x02\x15\x84Ta\x01jV[\x84a\x01\xA2V[` `\x1F\x82\x11`\x01\x81\x14a\x02MW_\x83\x15a\x026WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x01\xE6V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02\\V[P\x84\x82\x10\x15a\x02\x99W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[a\x07[\x80a\x02\xB5_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x9BW_5`\xE0\x1C\x80c@\xC1\x0F\x19\x11a\0cW\x80c@\xC1\x0F\x19\x14a\x01\x14W\x80cp\xA0\x821\x14a\x01)W\x80c\x95\xD8\x9BA\x14a\x01QW\x80c\xA9\x05\x9C\xBB\x14a\x01YW\x80c\xDDb\xED>\x14a\x01lW__\xFD[\x80c\x06\xFD\xDE\x03\x14a\0\x9FW\x80c\t^\xA7\xB3\x14a\0\xBDW\x80c\x18\x16\r\xDD\x14a\0\xE0W\x80c#\xB8r\xDD\x14a\0\xF2W\x80c1<\xE5g\x14a\x01\x05W[__\xFD[a\0\xA7a\x01\xA4V[`@Qa\0\xB4\x91\x90a\x05\xB5V[`@Q\x80\x91\x03\x90\xF3[a\0\xD0a\0\xCB6`\x04a\x06\x1BV[a\x024V[`@Q\x90\x15\x15\x81R` \x01a\0\xB4V[`\x02T[`@Q\x90\x81R` \x01a\0\xB4V[a\0\xD0a\x01\x006`\x04a\x06CV[a\x02MV[`@Q`\x12\x81R` \x01a\0\xB4V[a\x01'a\x01\"6`\x04a\x06\x1BV[a\x02pV[\0[a\0\xE4a\x0176`\x04a\x06}V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[a\0\xA7a\x02~V[a\0\xD0a\x01g6`\x04a\x06\x1BV[a\x02\x8DV[a\0\xE4a\x01z6`\x04a\x06\x9DV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T\x90V[```\x03\x80Ta\x01\xB3\x90a\x06\xCEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x01\xDF\x90a\x06\xCEV[\x80\x15a\x02*W\x80`\x1F\x10a\x02\x01Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02*V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_3a\x02A\x81\x85\x85a\x02\x9AV[`\x01\x91PP[\x92\x91PPV[_3a\x02Z\x85\x82\x85a\x02\xACV[a\x02e\x85\x85\x85a\x03,V[P`\x01\x94\x93PPPPV[a\x02z\x82\x82a\x03\x89V[PPV[```\x04\x80Ta\x01\xB3\x90a\x06\xCEV[_3a\x02A\x81\x85\x85a\x03,V[a\x02\xA7\x83\x83\x83`\x01a\x03\xBDV[PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R T_\x19\x81\x14a\x03&W\x81\x81\x10\x15a\x03\x18W`@Qc}\xC7\xA0\xD9`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03&\x84\x84\x84\x84\x03_a\x03\xBDV[PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x03UW`@QcKc~\x8F`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03~W`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[a\x02\xA7\x83\x83\x83a\x04\x8FV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x03\xB2W`@Qc\xECD/\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[a\x02z_\x83\x83a\x04\x8FV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x03\xE6W`@Qc\xE6\x02\xDF\x05`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\x0FW`@QcJ\x14\x06\xB1`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R \x82\x90U\x80\x15a\x03&W\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84`@Qa\x04\x81\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04\xB9W\x80`\x02_\x82\x82Ta\x04\xAE\x91\x90a\x07\x06V[\x90\x91UPa\x05)\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 T\x81\x81\x10\x15a\x05\x0BW`@Qc9\x144\xE3`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x01a\x03\x0FV[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05EW`\x02\x80T\x82\x90\x03\x90Ua\x05cV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R` \x81\x90R`@\x90 \x80T\x82\x01\x90U[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x83`@Qa\x05\xA8\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[` \x81R_\x82Q\x80` \x84\x01R_[\x81\x81\x10\x15a\x05\xE1W` \x81\x86\x01\x81\x01Q`@\x86\x84\x01\x01R\x01a\x05\xC4V[P_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x16W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x06,W__\xFD[a\x065\x83a\x06\0V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a\x06UW__\xFD[a\x06^\x84a\x06\0V[\x92Pa\x06l` \x85\x01a\x06\0V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x06\x8DW__\xFD[a\x06\x96\x82a\x06\0V[\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a\x06\xAEW__\xFD[a\x06\xB7\x83a\x06\0V[\x91Pa\x06\xC5` \x84\x01a\x06\0V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x06\xE2W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\0WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x02GWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \xFE\x1D\xFB\x04\xB2aQ]x<B\xC3w'\xC6\xA5G\xDB\r\xA7\xC9\xB2AA$\x8E\xDF\xF1\x06\xDA\x93\xBEdsolcC\0\x08\x1C\x003`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x03,\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0JW_5`\xE0\x1C\x80c\x01\xB7\x03|\x14a\0NW\x80c\x85,j\xE2\x14a\0eW\x80c\xC4\x92\x98\xAC\x14a\0\xD3W\x80c\xD9n\xE7T\x14a\0\xE6W[__\xFD[a\0ca\0\\6`\x04a\x01\xE4V[PPPPPV[\0[a\0\xC1a\0s6`\x04a\x02GV[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19``\x85\x90\x1B\x16` \x82\x01R`4\x81\x01\x83\x90R`T\x81\x01\x82\x90R_\x90`t\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0ca\0\xE16`\x04a\x02wV[a\0\xF9V[a\0ca\0\xF46`\x04a\x02GV[a\x018V[\x82\x7F\xFB\xD3IGG\xD8z\x87bX?6\x97\x02\x1D.\xAEw\x89\xC8#I+g-\xA7'q\\<\xB3g\x83\x83`@Qa\x01+\x92\x91\x90a\x02\xBFV[`@Q\x80\x91\x03\x90\xA2PPPV[\x81\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\x97@L\xCD\x190\r\x89,\xB8\x81:\xC9\\D\x1F\x99\\\r\xB1>+q\xB3f\x1A\xFA\xBF\x93\x85\xBC\x81\x83`@Qa\x01t\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x97W__\xFD[\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x01\xACW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\xC3W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x01\xDDW__\xFD[\x92P\x92\x90PV[_____`\x80\x86\x88\x03\x12\x15a\x01\xF8W__\xFD[a\x02\x01\x86a\x01\x81V[\x94P` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02*W__\xFD[a\x026\x88\x82\x89\x01a\x01\x9CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x02YW__\xFD[a\x02b\x84a\x01\x81V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[___`@\x84\x86\x03\x12\x15a\x02\x89W__\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xA6W__\xFD[a\x02\xB2\x86\x82\x87\x01a\x01\x9CV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x81\x01\x82\x90R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x02\xDDW__\xFD[\x82`\x05\x1B\x80\x85`@\x85\x017\x91\x90\x91\x01`@\x01\x93\x92PPPV\xFE\xA2dipfsX\"\x12 d\xD4\x85\n\xEB\xFEY\xE4\xC2\x10\x08\x96\xCFb\x96\x14v\xB0x\xF4\xA3:\x9B#\x16\x86N\xB0 \xCC\xD4\xF9dsolcC\0\x08\x1C\x003`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0C08\x03\x80a\x0C0\x839\x81\x01`@\x81\x90Ra\0.\x91a\0`V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x91V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0qW__\xFD[a\0z\x83a\0EV[\x91Pa\0\x88` \x84\x01a\0EV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0Bca\0\xCD_9_\x81\x81`\xD6\x01R\x81\x81a\x03\x1A\x01R\x81\x81a\x03\xB1\x01Ra\x06r\x01R_\x81\x81a\x01(\x01Ra\x02N\x01Ra\x0Bc_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x90W_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0cW\x80cv\x1D\xE1\x9F\x14a\x01#W\x80c\x8C\x934O\x14a\x01JW\x80c\x9A\xA9\xFD\xA5\x14a\x01lW\x80c\xA7\x1F\x8D\xA0\x14a\x01\x7FW\x80c\xCE\x1FEg\x14a\x01\x92W__\xFD[\x80c!j>\x9A\x14a\0\x94W\x80cL0W\x10\x14a\0\xBCW\x80c[\xD9\xE2\x99\x14a\0\xD1W\x80ccq\x0C\x05\x14a\x01\x10W[__\xFD[a\0\xA7a\0\xA26`\x04a\x07\x7FV[a\x01\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a\x07\xBDV[a\x02\xF5V[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xCFa\x01\x1E6`\x04a\x07\xFAV[a\x04BV[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA7a\x01X6`\x04a\x08kV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xCFa\x01z6`\x04a\x07\x7FV[a\x04\xC6V[a\0\xA7a\x01\x8D6`\x04a\x07\xFAV[a\x04\xFAV[a\x01\xB1a\x01\xA06`\x04a\x08kV[`\x01` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB3V[_\x80a\x01\xCB\x83\x80a\x08\x82V[`@Qa\x01\xD9\x92\x91\x90a\x08\xCCV[`@Q\x80\x91\x03\x90 \x90P_a\x02:\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x81` \x89\x01\x89a\x08\x82V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x9F\x93\x92\x91\x90a\t\x03V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDE\x91\x90a\t%V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[`@QcB\x965q`\xE1\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85,j\xE2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03gW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8B\x91\x90a\tLV[`@Qc6[\xB9\xD5`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD9n\xE7T\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xFAW__\xFD[PZ\xF1\x15\x80\x15a\x04\x0CW=__>=_\xFD[PP`@Q\x83\x92P\x85\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3PPPPV[\x81\x81a\x04N\x82\x82a\x04\xFAV[a\x04kW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04\xBFWa\x04\xAD\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04\x96Wa\x04\x96a\tcV[\x90P` \x02\x81\x01\x90a\x04\xA8\x91\x90a\twV[a\x05\xC1V[\x80a\x04\xB7\x81a\t\x95V[\x91PPa\x04mV[PPPPPV[\x80a\x04\xD0\x81a\x01\xBFV[a\x04\xEDW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xF6\x82a\x05\xC1V[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x05\xB5W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x05)Wa\x05)a\tcV[\x90P` \x02\x81\x01\x90a\x05;\x91\x90a\twV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05W\x91\x90a\n\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\n\\V[a\x05\xA3W_\x91PPa\x05\xBBV[\x80a\x05\xAD\x81a\t\x95V[\x91PPa\x04\xFDV[P`\x01\x90P[\x92\x91PPV[_a\x05\xCC\x82\x80a\x08\x82V[\x81\x01\x90a\x05\xD9\x91\x90a\n{V[\x80Q_\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x15a\x06\rW`@Qc\xAAC\xCB-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`@\x01Q\x11\x15a\x063W`@Qc\xC7J m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q_\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x81\x85\x01\x80Q\x86Q\x86R\x91\x90\x93R\x92 \x91\x90\x91U\x81Q\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xC4\x92\x98\xAC\x91a\x06\xA2\x90a\x076V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBF\x92\x91\x90a\n\xE0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD6W__\xFD[PZ\xF1\x15\x80\x15a\x06\xE8W=__>=_\xFD[PPPP\x80` \x01Q\x81_\x01Q\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1\x83`@\x01Q`@Qa\x07*\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92_\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P`\x01\x81\x84\x81Q\x81\x10a\x07nWa\x07na\tcV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\x8FW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA5W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x07\xB6W__\xFD[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x07\xCFW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE5W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x08\x0BW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08!W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x081W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08GW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x08[W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x08{W__\xFD[P5\x91\x90PV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x97W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB1W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xC5W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\t\x1C`@\x83\x01\x84\x86a\x08\xDBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\t5W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xB6W__\xFD[_` \x82\x84\x03\x12\x15a\t\\W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\t\x8BW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\t\xBCWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xDAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF9W__\xFD[\x806\x03\x82\x13\x15a\x08\xC5W__\xFD[` \x81R_a\n\x16\x83\x84a\t\xC5V[`@` \x85\x01Ra\n+``\x85\x01\x82\x84a\x08\xDBV[\x91PPa\n;` \x85\x01\x85a\t\xC5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\nR\x83\x82\x84a\x08\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\nlW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xB6W__\xFD[_``\x82\x84\x03\x12\x80\x15a\n\x8CW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBCWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a\x0B!W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x03V[P\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 z]\xC2\x98\xA2W\x03j\xFBZg\xBE\x15\\zz\xB0j \xBC\x1D\xF5\x91\x97k`\xBF\xD41!\xF3\xDAdsolcC\0\x08\x1C\x003\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-Y\x1E\xEDE\xD8>\xE0\x1C\x18\xF6\x1A\xF5Y\xCB\xB6\x98\xBCm7zbt\x88\x89\xAF\x9C\xD6{\x0Ei\xE2iaddPayload failed with low-level error\xA2dipfsX\"\x12 \xADB\x97\xEE\x0E\xE6\xEE\xCASa\xDD*\xF6\xE0\x9B\xE9?\x1F\x18}\x8Bn\x96Cg\x829\x80\\X-\xD3dsolcC\0\x08\x1C\x003",
    );
    /**Event with signature `MarketCreated(bytes32,bytes32)` and selector `0xbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db`.
```solidity
event MarketCreated(bytes32 indexed questionId, bytes32 indexed conditionId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MarketCreated {
        #[allow(missing_docs)]
        pub questionId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub conditionId: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for MarketCreated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "MarketCreated(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                187u8,
                131u8,
                204u8,
                42u8,
                19u8,
                211u8,
                171u8,
                175u8,
                42u8,
                198u8,
                188u8,
                177u8,
                185u8,
                110u8,
                16u8,
                143u8,
                108u8,
                231u8,
                93u8,
                200u8,
                235u8,
                219u8,
                96u8,
                134u8,
                248u8,
                5u8,
                234u8,
                176u8,
                155u8,
                213u8,
                52u8,
                219u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    questionId: topics.1,
                    conditionId: topics.2,
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
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.questionId.clone(),
                    self.conditionId.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.questionId);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.conditionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MarketCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MarketCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MarketCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `MarketResolved(bytes32,bytes32,uint256)` and selector `0x4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d1`.
```solidity
event MarketResolved(bytes32 indexed questionId, bytes32 indexed conditionId, uint256 outcome);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MarketResolved {
        #[allow(missing_docs)]
        pub questionId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub conditionId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub outcome: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MarketResolved {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "MarketResolved(bytes32,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                76u8,
                10u8,
                226u8,
                56u8,
                246u8,
                123u8,
                33u8,
                18u8,
                121u8,
                29u8,
                149u8,
                155u8,
                177u8,
                2u8,
                160u8,
                176u8,
                96u8,
                12u8,
                58u8,
                83u8,
                131u8,
                124u8,
                61u8,
                121u8,
                49u8,
                149u8,
                141u8,
                86u8,
                42u8,
                99u8,
                185u8,
                209u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    questionId: topics.1,
                    conditionId: topics.2,
                    outcome: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.outcome),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.questionId.clone(),
                    self.conditionId.clone(),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.questionId);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.conditionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MarketResolved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MarketResolved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MarketResolved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
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
    /**Function with signature `OUTCOME_SLOT_COUNT()` and selector `0x9f77ae50`.
```solidity
function OUTCOME_SLOT_COUNT() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OUTCOME_SLOT_COUNTCall {}
    ///Container type for the return parameters of the [`OUTCOME_SLOT_COUNT()`](OUTCOME_SLOT_COUNTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OUTCOME_SLOT_COUNTReturn {
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
            impl ::core::convert::From<OUTCOME_SLOT_COUNTCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OUTCOME_SLOT_COUNTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OUTCOME_SLOT_COUNTCall {
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
            impl ::core::convert::From<OUTCOME_SLOT_COUNTReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OUTCOME_SLOT_COUNTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OUTCOME_SLOT_COUNTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OUTCOME_SLOT_COUNTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OUTCOME_SLOT_COUNTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OUTCOME_SLOT_COUNT()";
            const SELECTOR: [u8; 4] = [159u8, 119u8, 174u8, 80u8];
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
    /**Function with signature `QUESTION_ID()` and selector `0x679e54e8`.
```solidity
function QUESTION_ID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUESTION_IDCall {}
    ///Container type for the return parameters of the [`QUESTION_ID()`](QUESTION_IDCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QUESTION_IDReturn {
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
            impl ::core::convert::From<QUESTION_IDCall> for UnderlyingRustTuple<'_> {
                fn from(value: QUESTION_IDCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUESTION_IDCall {
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
            impl ::core::convert::From<QUESTION_IDReturn> for UnderlyingRustTuple<'_> {
                fn from(value: QUESTION_IDReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for QUESTION_IDReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for QUESTION_IDCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = QUESTION_IDReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "QUESTION_ID()";
            const SELECTOR: [u8; 4] = [103u8, 158u8, 84u8, 232u8];
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
    /**Function with signature `collateral()` and selector `0xd8dfeb45`.
```solidity
function collateral() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collateralCall {}
    ///Container type for the return parameters of the [`collateral()`](collateralCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct collateralReturn {
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
            impl ::core::convert::From<collateralCall> for UnderlyingRustTuple<'_> {
                fn from(value: collateralCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralCall {
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
            impl ::core::convert::From<collateralReturn> for UnderlyingRustTuple<'_> {
                fn from(value: collateralReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for collateralReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collateralCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collateralReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collateral()";
            const SELECTOR: [u8; 4] = [216u8, 223u8, 235u8, 69u8];
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
    /**Function with signature `conditionalTokens()` and selector `0x5bd9e299`.
```solidity
function conditionalTokens() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct conditionalTokensCall {}
    ///Container type for the return parameters of the [`conditionalTokens()`](conditionalTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct conditionalTokensReturn {
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
            impl ::core::convert::From<conditionalTokensCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: conditionalTokensCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for conditionalTokensCall {
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
            impl ::core::convert::From<conditionalTokensReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: conditionalTokensReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for conditionalTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for conditionalTokensCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = conditionalTokensReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "conditionalTokens()";
            const SELECTOR: [u8; 4] = [91u8, 217u8, 226u8, 153u8];
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
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
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
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        pub excludedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
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
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
```solidity
function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall {}
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        pub excludedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
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
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        pub excludedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
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
    /**Function with signature `market()` and selector `0x80f55605`.
```solidity
function market() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketCall {}
    ///Container type for the return parameters of the [`market()`](marketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketReturn {
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
            impl ::core::convert::From<marketCall> for UnderlyingRustTuple<'_> {
                fn from(value: marketCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for marketCall {
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
            impl ::core::convert::From<marketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: marketReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for marketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for marketCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = marketReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "market()";
            const SELECTOR: [u8; 4] = [128u8, 245u8, 86u8, 5u8];
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
    /**Function with signature `setUp()` and selector `0x0a9254e4`.
```solidity
function setUp() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpCall {}
    ///Container type for the return parameters of the [`setUp()`](setUpCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setUpReturn {}
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
            impl ::core::convert::From<setUpCall> for UnderlyingRustTuple<'_> {
                fn from(value: setUpCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpCall {
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
            impl ::core::convert::From<setUpReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setUpReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setUpReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setUpCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setUpReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setUp()";
            const SELECTOR: [u8; 4] = [10u8, 146u8, 84u8, 228u8];
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
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
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
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
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        pub targetedContracts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
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
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
```solidity
function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
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
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
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
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        pub targetedSenders_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
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
    /**Function with signature `testFail_InvalidOutcome()` and selector `0x7a896a93`.
```solidity
function testFail_InvalidOutcome() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFail_InvalidOutcomeCall {}
    ///Container type for the return parameters of the [`testFail_InvalidOutcome()`](testFail_InvalidOutcomeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFail_InvalidOutcomeReturn {}
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
            impl ::core::convert::From<testFail_InvalidOutcomeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFail_InvalidOutcomeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFail_InvalidOutcomeCall {
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
            impl ::core::convert::From<testFail_InvalidOutcomeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFail_InvalidOutcomeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFail_InvalidOutcomeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFail_InvalidOutcomeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFail_InvalidOutcomeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFail_InvalidOutcome()";
            const SELECTOR: [u8; 4] = [122u8, 137u8, 106u8, 147u8];
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
    /**Function with signature `testFail_ResolveMarketTwice()` and selector `0x64cd43af`.
```solidity
function testFail_ResolveMarketTwice() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFail_ResolveMarketTwiceCall {}
    ///Container type for the return parameters of the [`testFail_ResolveMarketTwice()`](testFail_ResolveMarketTwiceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct testFail_ResolveMarketTwiceReturn {}
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
            impl ::core::convert::From<testFail_ResolveMarketTwiceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFail_ResolveMarketTwiceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFail_ResolveMarketTwiceCall {
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
            impl ::core::convert::From<testFail_ResolveMarketTwiceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: testFail_ResolveMarketTwiceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for testFail_ResolveMarketTwiceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for testFail_ResolveMarketTwiceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = testFail_ResolveMarketTwiceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "testFail_ResolveMarketTwice()";
            const SELECTOR: [u8; 4] = [100u8, 205u8, 67u8, 175u8];
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
    /**Function with signature `test_CreateMarket()` and selector `0xa63c26bf`.
```solidity
function test_CreateMarket() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CreateMarketCall {}
    ///Container type for the return parameters of the [`test_CreateMarket()`](test_CreateMarketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_CreateMarketReturn {}
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
            impl ::core::convert::From<test_CreateMarketCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CreateMarketCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CreateMarketCall {
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
            impl ::core::convert::From<test_CreateMarketReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_CreateMarketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_CreateMarketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_CreateMarketCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_CreateMarketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_CreateMarket()";
            const SELECTOR: [u8; 4] = [166u8, 60u8, 38u8, 191u8];
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
    /**Function with signature `test_ResolveMarket()` and selector `0x33595c46`.
```solidity
function test_ResolveMarket() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ResolveMarketCall {}
    ///Container type for the return parameters of the [`test_ResolveMarket()`](test_ResolveMarketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct test_ResolveMarketReturn {}
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
            impl ::core::convert::From<test_ResolveMarketCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ResolveMarketCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ResolveMarketCall {
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
            impl ::core::convert::From<test_ResolveMarketReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: test_ResolveMarketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for test_ResolveMarketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for test_ResolveMarketCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = test_ResolveMarketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "test_ResolveMarket()";
            const SELECTOR: [u8; 4] = [51u8, 89u8, 92u8, 70u8];
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
    ///Container for all the [`WavsConditionalMarketTest`](self) function calls.
    pub enum WavsConditionalMarketTestCalls {
        IS_TEST(IS_TESTCall),
        OUTCOME_SLOT_COUNT(OUTCOME_SLOT_COUNTCall),
        QUESTION_ID(QUESTION_IDCall),
        STAKE_REGISTRY(STAKE_REGISTRYCall),
        collateral(collateralCall),
        conditionalTokens(conditionalTokensCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSelectors(excludeSelectorsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        market(marketCall),
        setUp(setUpCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        testFail_InvalidOutcome(testFail_InvalidOutcomeCall),
        testFail_ResolveMarketTwice(testFail_ResolveMarketTwiceCall),
        test_CreateMarket(test_CreateMarketCall),
        test_ResolveMarket(test_ResolveMarketCall),
    }
    #[automatically_derived]
    impl WavsConditionalMarketTestCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [10u8, 146u8, 84u8, 228u8],
            [30u8, 215u8, 131u8, 28u8],
            [42u8, 222u8, 56u8, 128u8],
            [51u8, 89u8, 92u8, 70u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [91u8, 217u8, 226u8, 153u8],
            [100u8, 205u8, 67u8, 175u8],
            [102u8, 217u8, 169u8, 160u8],
            [103u8, 158u8, 84u8, 232u8],
            [118u8, 29u8, 225u8, 159u8],
            [122u8, 137u8, 106u8, 147u8],
            [128u8, 245u8, 86u8, 5u8],
            [133u8, 34u8, 108u8, 129u8],
            [145u8, 106u8, 23u8, 198u8],
            [159u8, 119u8, 174u8, 80u8],
            [166u8, 60u8, 38u8, 191u8],
            [176u8, 70u8, 79u8, 220u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [216u8, 223u8, 235u8, 69u8],
            [226u8, 12u8, 159u8, 113u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsConditionalMarketTestCalls {
        const NAME: &'static str = "WavsConditionalMarketTestCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::OUTCOME_SLOT_COUNT(_) => {
                    <OUTCOME_SLOT_COUNTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::QUESTION_ID(_) => {
                    <QUESTION_IDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::STAKE_REGISTRY(_) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::collateral(_) => {
                    <collateralCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::conditionalTokens(_) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::market(_) => <marketCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setUp(_) => <setUpCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFail_InvalidOutcome(_) => {
                    <testFail_InvalidOutcomeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::testFail_ResolveMarketTwice(_) => {
                    <testFail_ResolveMarketTwiceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_CreateMarket(_) => {
                    <test_CreateMarketCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::test_ResolveMarket(_) => {
                    <test_ResolveMarketCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls>] = &[
                {
                    fn setUp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <setUpCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::setUp)
                    }
                    setUp
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn test_ResolveMarket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <test_ResolveMarketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::test_ResolveMarket)
                    }
                    test_ResolveMarket
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn conditionalTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <conditionalTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::conditionalTokens)
                    }
                    conditionalTokens
                },
                {
                    fn testFail_ResolveMarketTwice(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <testFail_ResolveMarketTwiceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                WavsConditionalMarketTestCalls::testFail_ResolveMarketTwice,
                            )
                    }
                    testFail_ResolveMarketTwice
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn QUESTION_ID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <QUESTION_IDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::QUESTION_ID)
                    }
                    QUESTION_ID
                },
                {
                    fn STAKE_REGISTRY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::STAKE_REGISTRY)
                    }
                    STAKE_REGISTRY
                },
                {
                    fn testFail_InvalidOutcome(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <testFail_InvalidOutcomeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::testFail_InvalidOutcome)
                    }
                    testFail_InvalidOutcome
                },
                {
                    fn market(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <marketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::market)
                    }
                    market
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn OUTCOME_SLOT_COUNT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <OUTCOME_SLOT_COUNTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::OUTCOME_SLOT_COUNT)
                    }
                    OUTCOME_SLOT_COUNT
                },
                {
                    fn test_CreateMarket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <test_CreateMarketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::test_CreateMarket)
                    }
                    test_CreateMarket
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::failed)
                    }
                    failed
                },
                {
                    fn collateral(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <collateralCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::collateral)
                    }
                    collateral
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketTestCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketTestCalls::IS_TEST)
                    }
                    IS_TEST
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::OUTCOME_SLOT_COUNT(inner) => {
                    <OUTCOME_SLOT_COUNTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::QUESTION_ID(inner) => {
                    <QUESTION_IDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::collateral(inner) => {
                    <collateralCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::conditionalTokens(inner) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::market(inner) => {
                    <marketCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFail_InvalidOutcome(inner) => {
                    <testFail_InvalidOutcomeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::testFail_ResolveMarketTwice(inner) => {
                    <testFail_ResolveMarketTwiceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_CreateMarket(inner) => {
                    <test_CreateMarketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::test_ResolveMarket(inner) => {
                    <test_ResolveMarketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::OUTCOME_SLOT_COUNT(inner) => {
                    <OUTCOME_SLOT_COUNTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::QUESTION_ID(inner) => {
                    <QUESTION_IDCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::STAKE_REGISTRY(inner) => {
                    <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::collateral(inner) => {
                    <collateralCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::conditionalTokens(inner) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::market(inner) => {
                    <marketCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setUp(inner) => {
                    <setUpCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFail_InvalidOutcome(inner) => {
                    <testFail_InvalidOutcomeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::testFail_ResolveMarketTwice(inner) => {
                    <testFail_ResolveMarketTwiceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_CreateMarket(inner) => {
                    <test_CreateMarketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::test_ResolveMarket(inner) => {
                    <test_ResolveMarketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`WavsConditionalMarketTest`](self) events.
    pub enum WavsConditionalMarketTestEvents {
        MarketCreated(MarketCreated),
        MarketResolved(MarketResolved),
        log(log),
        log_address(log_address),
        log_array_0(log_array_0),
        log_array_1(log_array_1),
        log_array_2(log_array_2),
        log_bytes(log_bytes),
        log_bytes32(log_bytes32),
        log_int(log_int),
        log_named_address(log_named_address),
        log_named_array_0(log_named_array_0),
        log_named_array_1(log_named_array_1),
        log_named_array_2(log_named_array_2),
        log_named_bytes(log_named_bytes),
        log_named_bytes32(log_named_bytes32),
        log_named_decimal_int(log_named_decimal_int),
        log_named_decimal_uint(log_named_decimal_uint),
        log_named_int(log_named_int),
        log_named_string(log_named_string),
        log_named_uint(log_named_uint),
        log_string(log_string),
        log_uint(log_uint),
        logs(logs),
    }
    #[automatically_derived]
    impl WavsConditionalMarketTestEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ],
            [
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ],
            [
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ],
            [
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ],
            [
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ],
            [
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ],
            [
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ],
            [
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ],
            [
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ],
            [
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ],
            [
                76u8,
                10u8,
                226u8,
                56u8,
                246u8,
                123u8,
                33u8,
                18u8,
                121u8,
                29u8,
                149u8,
                155u8,
                177u8,
                2u8,
                160u8,
                176u8,
                96u8,
                12u8,
                58u8,
                83u8,
                131u8,
                124u8,
                61u8,
                121u8,
                49u8,
                149u8,
                141u8,
                86u8,
                42u8,
                99u8,
                185u8,
                209u8,
            ],
            [
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ],
            [
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ],
            [
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ],
            [
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ],
            [
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ],
            [
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ],
            [
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ],
            [
                187u8,
                131u8,
                204u8,
                42u8,
                19u8,
                211u8,
                171u8,
                175u8,
                42u8,
                198u8,
                188u8,
                177u8,
                185u8,
                110u8,
                16u8,
                143u8,
                108u8,
                231u8,
                93u8,
                200u8,
                235u8,
                219u8,
                96u8,
                134u8,
                248u8,
                5u8,
                234u8,
                176u8,
                155u8,
                213u8,
                52u8,
                219u8,
            ],
            [
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ],
            [
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ],
            [
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ],
            [
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ],
            [
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for WavsConditionalMarketTestEvents {
        const NAME: &'static str = "WavsConditionalMarketTestEvents";
        const COUNT: usize = 24usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<MarketCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MarketCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MarketCreated)
                }
                Some(<MarketResolved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MarketResolved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MarketResolved)
                }
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for WavsConditionalMarketTestEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MarketCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MarketResolved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MarketCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MarketResolved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`WavsConditionalMarketTest`](self) contract instance.

See the [wrapper's documentation](`WavsConditionalMarketTestInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WavsConditionalMarketTestInstance<T, P, N> {
        WavsConditionalMarketTestInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<WavsConditionalMarketTestInstance<T, P, N>>,
    > {
        WavsConditionalMarketTestInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        WavsConditionalMarketTestInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`WavsConditionalMarketTest`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`WavsConditionalMarketTest`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WavsConditionalMarketTestInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WavsConditionalMarketTestInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WavsConditionalMarketTestInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsConditionalMarketTestInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`WavsConditionalMarketTest`](self) contract instance.

See the [wrapper's documentation](`WavsConditionalMarketTestInstance`) for more details.*/
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
        ) -> alloy_contract::Result<WavsConditionalMarketTestInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> WavsConditionalMarketTestInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WavsConditionalMarketTestInstance<T, P, N> {
            WavsConditionalMarketTestInstance {
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
    > WavsConditionalMarketTestInstance<T, P, N> {
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
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`OUTCOME_SLOT_COUNT`] function.
        pub fn OUTCOME_SLOT_COUNT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, OUTCOME_SLOT_COUNTCall, N> {
            self.call_builder(&OUTCOME_SLOT_COUNTCall {})
        }
        ///Creates a new call builder for the [`QUESTION_ID`] function.
        pub fn QUESTION_ID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, QUESTION_IDCall, N> {
            self.call_builder(&QUESTION_IDCall {})
        }
        ///Creates a new call builder for the [`STAKE_REGISTRY`] function.
        pub fn STAKE_REGISTRY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, STAKE_REGISTRYCall, N> {
            self.call_builder(&STAKE_REGISTRYCall {})
        }
        ///Creates a new call builder for the [`collateral`] function.
        pub fn collateral(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, collateralCall, N> {
            self.call_builder(&collateralCall {})
        }
        ///Creates a new call builder for the [`conditionalTokens`] function.
        pub fn conditionalTokens(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, conditionalTokensCall, N> {
            self.call_builder(&conditionalTokensCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`market`] function.
        pub fn market(&self) -> alloy_contract::SolCallBuilder<T, &P, marketCall, N> {
            self.call_builder(&marketCall {})
        }
        ///Creates a new call builder for the [`setUp`] function.
        pub fn setUp(&self) -> alloy_contract::SolCallBuilder<T, &P, setUpCall, N> {
            self.call_builder(&setUpCall {})
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`testFail_InvalidOutcome`] function.
        pub fn testFail_InvalidOutcome(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testFail_InvalidOutcomeCall, N> {
            self.call_builder(&testFail_InvalidOutcomeCall {})
        }
        ///Creates a new call builder for the [`testFail_ResolveMarketTwice`] function.
        pub fn testFail_ResolveMarketTwice(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, testFail_ResolveMarketTwiceCall, N> {
            self.call_builder(&testFail_ResolveMarketTwiceCall {})
        }
        ///Creates a new call builder for the [`test_CreateMarket`] function.
        pub fn test_CreateMarket(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, test_CreateMarketCall, N> {
            self.call_builder(&test_CreateMarketCall {})
        }
        ///Creates a new call builder for the [`test_ResolveMarket`] function.
        pub fn test_ResolveMarket(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, test_ResolveMarketCall, N> {
            self.call_builder(&test_ResolveMarketCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsConditionalMarketTestInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`MarketCreated`] event.
        pub fn MarketCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MarketCreated, N> {
            self.event_filter::<MarketCreated>()
        }
        ///Creates a new event filter for the [`MarketResolved`] event.
        pub fn MarketResolved_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MarketResolved, N> {
            self.event_filter::<MarketResolved>()
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
