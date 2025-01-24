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
library IWavsSDK {
    struct SignedPayload {
        bytes data;
        bytes signature;
    }
}

interface SafeModule {
    error InvalidSignature();

    constructor(address _safe, address _stakeRegistry);

    function STAKE_REGISTRY() external view returns (address);
    function addPayload(IWavsSDK.SignedPayload memory signedPayload) external;
    function addPayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external;
    function owner() external view returns (address);
    function safe() external view returns (address);
    function validatePayload(IWavsSDK.SignedPayload memory signedPayload) external view returns (bool);
    function validatePayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external view returns (bool);
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
    "type": "error",
    "name": "InvalidSignature",
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
pub mod SafeModule {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405234801561000f575f5ffd5b50604051610a94380380610a9483398101604081905261002e916100d7565b6001600160a01b03808216608052821661008e5760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420736166652061646472657373000000000000000000000000604482015260640160405180910390fd5b505f80546001600160a01b039092166001600160a01b03199283161790556001805490911633179055610108565b80516001600160a01b03811681146100d2575f5ffd5b919050565b5f5f604083850312156100e8575f5ffd5b6100f1836100bc565b91506100ff602084016100bc565b90509250929050565b60805161096e6101265f395f818160ea01526101d4015261096e5ff3fe608060405234801561000f575f5ffd5b506004361061007a575f3560e01c8063761de19f11610058578063761de19f146100e55780638da5cb5b1461010c5780639aa9fda51461011f578063a71f8da014610132575f5ffd5b8063186f03541461007e578063216a3e9a146100ad57806363710c05146100d0575b5f5ffd5b5f54610090906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100c06100bb366004610534565b610145565b60405190151581526020016100a4565b6100e36100de366004610572565b61027b565b005b6100907f000000000000000000000000000000000000000000000000000000000000000081565b600154610090906001600160a01b031681565b6100e361012d366004610534565b6102ff565b6100c0610140366004610572565b610333565b5f8061015183806105e3565b60405161015f92919061062d565b604051809103902090505f6101c0826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e8361020760208901896105e3565b6040518463ffffffff1660e01b815260040161022593929190610664565b602060405180830381865afa158015610240573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190610686565b6001600160e01b0319918216911614949350505050565b81816102878282610333565b6102a457604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156102f8576102e685858363ffffffff168181106102cf576102cf6106ad565b90506020028101906102e191906106c1565b6103fa565b806102f0816106df565b9150506102a6565b5050505050565b8061030981610145565b61032657604051638baa579f60e01b815260040160405180910390fd5b61032f826103fa565b5050565b5f805b63ffffffff81168311156103ee573063216a3e9a858563ffffffff8516818110610362576103626106ad565b905060200281019061037491906106c1565b6040518263ffffffff1660e01b81526004016103909190610751565b602060405180830381865afa1580156103ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103cf91906107a6565b6103dc575f9150506103f4565b806103e6816106df565b915050610336565b50600190505b92915050565b5f808061040784806105e3565b81019061041491906107d9565b919450925090506001600160a01b03831661046f5760405162461bcd60e51b8152602060048201526016602482015275496e76616c696420746172676574206164647265737360501b60448201526064015b60405180910390fd5b5f805460405163468721a760e01b81526001600160a01b039091169063468721a7906104a59087908790879087906004016108d2565b6020604051808303815f875af11580156104c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104e591906107a6565b9050806102f85760405162461bcd60e51b815260206004820152601960248201527f4d6f64756c65207472616e73616374696f6e206661696c6564000000000000006044820152606401610466565b5f60208284031215610544575f5ffd5b813567ffffffffffffffff81111561055a575f5ffd5b82016040818503121561056b575f5ffd5b9392505050565b5f5f60208385031215610583575f5ffd5b823567ffffffffffffffff811115610599575f5ffd5b8301601f810185136105a9575f5ffd5b803567ffffffffffffffff8111156105bf575f5ffd5b8560208260051b84010111156105d3575f5ffd5b6020919091019590945092505050565b5f5f8335601e198436030181126105f8575f5ffd5b83018035915067ffffffffffffffff821115610612575f5ffd5b602001915036819003821315610626575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61067d60408301848661063c565b95945050505050565b5f60208284031215610696575f5ffd5b81516001600160e01b03198116811461056b575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126106d5575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff810361070657634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e19843603018112610724575f5ffd5b830160208101925035905067ffffffffffffffff811115610743575f5ffd5b803603821315610626575f5ffd5b602081525f610760838461070f565b6040602085015261077560608501828461063c565b915050610785602085018561070f565b848303601f1901604086015261079c83828461063c565b9695505050505050565b5f602082840312156107b6575f5ffd5b8151801515811461056b575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f5f606084860312156107eb575f5ffd5b83356001600160a01b0381168114610801575f5ffd5b925060208401359150604084013567ffffffffffffffff811115610823575f5ffd5b8401601f81018613610833575f5ffd5b803567ffffffffffffffff81111561084d5761084d6107c5565b604051601f8201601f19908116603f0116810167ffffffffffffffff8111828210171561087c5761087c6107c5565b604052818152828201602001881015610893575f5ffd5b816020840160208301375f602083830101528093505050509250925092565b600281106108ce57634e487b7160e01b5f52602160045260245ffd5b9052565b60018060a01b0385168152836020820152608060408201525f83518060808401525f5b8181101561091257602081870181015160a08684010152016108f5565b505f60a0828501015260a0601f19601f83011684010191505061067d60608301846108b256fea2646970667358221220609c4615ff27663e1ecd6b881b3e1ebd2aa8f44db9b93c910969590559d1796364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\n\x948\x03\x80a\n\x94\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\xD7V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\x80R\x82\x16a\0\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid safe address\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90U`\x01\x80T\x90\x91\x163\x17\x90Ua\x01\x08V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xD2W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0\xE8W__\xFD[a\0\xF1\x83a\0\xBCV[\x91Pa\0\xFF` \x84\x01a\0\xBCV[\x90P\x92P\x92\x90PV[`\x80Qa\tna\x01&_9_\x81\x81`\xEA\x01Ra\x01\xD4\x01Ra\tn_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0XW\x80cv\x1D\xE1\x9F\x14a\0\xE5W\x80c\x8D\xA5\xCB[\x14a\x01\x0CW\x80c\x9A\xA9\xFD\xA5\x14a\x01\x1FW\x80c\xA7\x1F\x8D\xA0\x14a\x012W__\xFD[\x80c\x18o\x03T\x14a\0~W\x80c!j>\x9A\x14a\0\xADW\x80ccq\x0C\x05\x14a\0\xD0W[__\xFD[_Ta\0\x90\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC0a\0\xBB6`\x04a\x054V[a\x01EV[`@Q\x90\x15\x15\x81R` \x01a\0\xA4V[a\0\xE3a\0\xDE6`\x04a\x05rV[a\x02{V[\0[a\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\0\x90\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xE3a\x01-6`\x04a\x054V[a\x02\xFFV[a\0\xC0a\x01@6`\x04a\x05rV[a\x033V[_\x80a\x01Q\x83\x80a\x05\xE3V[`@Qa\x01_\x92\x91\x90a\x06-V[`@Q\x80\x91\x03\x90 \x90P_a\x01\xC0\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x07` \x89\x01\x89a\x05\xE3V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02%\x93\x92\x91\x90a\x06dV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90a\x06\x86V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[\x81\x81a\x02\x87\x82\x82a\x033V[a\x02\xA4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x02\xF8Wa\x02\xE6\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x02\xCFWa\x02\xCFa\x06\xADV[\x90P` \x02\x81\x01\x90a\x02\xE1\x91\x90a\x06\xC1V[a\x03\xFAV[\x80a\x02\xF0\x81a\x06\xDFV[\x91PPa\x02\xA6V[PPPPPV[\x80a\x03\t\x81a\x01EV[a\x03&W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03/\x82a\x03\xFAV[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x03\xEEW0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x03bWa\x03ba\x06\xADV[\x90P` \x02\x81\x01\x90a\x03t\x91\x90a\x06\xC1V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x90\x91\x90a\x07QV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a\x07\xA6V[a\x03\xDCW_\x91PPa\x03\xF4V[\x80a\x03\xE6\x81a\x06\xDFV[\x91PPa\x036V[P`\x01\x90P[\x92\x91PPV[_\x80\x80a\x04\x07\x84\x80a\x05\xE3V[\x81\x01\x90a\x04\x14\x91\x90a\x07\xD9V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid target address`P\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`@QcF\x87!\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\x87!\xA7\x90a\x04\xA5\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x08\xD2V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE5\x91\x90a\x07\xA6V[\x90P\x80a\x02\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FModule transaction failed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04fV[_` \x82\x84\x03\x12\x15a\x05DW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05ZW__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x05kW__\xFD[\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x05\x83W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x99W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x05\xA9W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xBFW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x05\xD3W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x05\xF8W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\x12W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06&W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\x06}`@\x83\x01\x84\x86a\x06<V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x06\x96W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05kW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x06\xD5W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x07\x06WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x07$W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07CW__\xFD[\x806\x03\x82\x13\x15a\x06&W__\xFD[` \x81R_a\x07`\x83\x84a\x07\x0FV[`@` \x85\x01Ra\x07u``\x85\x01\x82\x84a\x06<V[\x91PPa\x07\x85` \x85\x01\x85a\x07\x0FV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x07\x9C\x83\x82\x84a\x06<V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x07\xB6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x05kW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x07\xEBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x01W__\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08#W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x083W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08MWa\x08Ma\x07\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08|Wa\x08|a\x07\xC5V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x08\x93W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\x02\x81\x10a\x08\xCEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_\x83Q\x80`\x80\x84\x01R_[\x81\x81\x10\x15a\t\x12W` \x81\x87\x01\x81\x01Q`\xA0\x86\x84\x01\x01R\x01a\x08\xF5V[P_`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x06}``\x83\x01\x84a\x08\xB2V\xFE\xA2dipfsX\"\x12 `\x9CF\x15\xFF'f>\x1E\xCDk\x88\x1B>\x1E\xBD*\xA8\xF4M\xB9\xB9<\x91\tiY\x05Y\xD1ycdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061007a575f3560e01c8063761de19f11610058578063761de19f146100e55780638da5cb5b1461010c5780639aa9fda51461011f578063a71f8da014610132575f5ffd5b8063186f03541461007e578063216a3e9a146100ad57806363710c05146100d0575b5f5ffd5b5f54610090906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b6100c06100bb366004610534565b610145565b60405190151581526020016100a4565b6100e36100de366004610572565b61027b565b005b6100907f000000000000000000000000000000000000000000000000000000000000000081565b600154610090906001600160a01b031681565b6100e361012d366004610534565b6102ff565b6100c0610140366004610572565b610333565b5f8061015183806105e3565b60405161015f92919061062d565b604051809103902090505f6101c0826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e8361020760208901896105e3565b6040518463ffffffff1660e01b815260040161022593929190610664565b602060405180830381865afa158015610240573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102649190610686565b6001600160e01b0319918216911614949350505050565b81816102878282610333565b6102a457604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156102f8576102e685858363ffffffff168181106102cf576102cf6106ad565b90506020028101906102e191906106c1565b6103fa565b806102f0816106df565b9150506102a6565b5050505050565b8061030981610145565b61032657604051638baa579f60e01b815260040160405180910390fd5b61032f826103fa565b5050565b5f805b63ffffffff81168311156103ee573063216a3e9a858563ffffffff8516818110610362576103626106ad565b905060200281019061037491906106c1565b6040518263ffffffff1660e01b81526004016103909190610751565b602060405180830381865afa1580156103ab573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103cf91906107a6565b6103dc575f9150506103f4565b806103e6816106df565b915050610336565b50600190505b92915050565b5f808061040784806105e3565b81019061041491906107d9565b919450925090506001600160a01b03831661046f5760405162461bcd60e51b8152602060048201526016602482015275496e76616c696420746172676574206164647265737360501b60448201526064015b60405180910390fd5b5f805460405163468721a760e01b81526001600160a01b039091169063468721a7906104a59087908790879087906004016108d2565b6020604051808303815f875af11580156104c1573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104e591906107a6565b9050806102f85760405162461bcd60e51b815260206004820152601960248201527f4d6f64756c65207472616e73616374696f6e206661696c6564000000000000006044820152606401610466565b5f60208284031215610544575f5ffd5b813567ffffffffffffffff81111561055a575f5ffd5b82016040818503121561056b575f5ffd5b9392505050565b5f5f60208385031215610583575f5ffd5b823567ffffffffffffffff811115610599575f5ffd5b8301601f810185136105a9575f5ffd5b803567ffffffffffffffff8111156105bf575f5ffd5b8560208260051b84010111156105d3575f5ffd5b6020919091019590945092505050565b5f5f8335601e198436030181126105f8575f5ffd5b83018035915067ffffffffffffffff821115610612575f5ffd5b602001915036819003821315610626575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61067d60408301848661063c565b95945050505050565b5f60208284031215610696575f5ffd5b81516001600160e01b03198116811461056b575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126106d5575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff810361070657634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e19843603018112610724575f5ffd5b830160208101925035905067ffffffffffffffff811115610743575f5ffd5b803603821315610626575f5ffd5b602081525f610760838461070f565b6040602085015261077560608501828461063c565b915050610785602085018561070f565b848303601f1901604086015261079c83828461063c565b9695505050505050565b5f602082840312156107b6575f5ffd5b8151801515811461056b575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b5f5f5f606084860312156107eb575f5ffd5b83356001600160a01b0381168114610801575f5ffd5b925060208401359150604084013567ffffffffffffffff811115610823575f5ffd5b8401601f81018613610833575f5ffd5b803567ffffffffffffffff81111561084d5761084d6107c5565b604051601f8201601f19908116603f0116810167ffffffffffffffff8111828210171561087c5761087c6107c5565b604052818152828201602001881015610893575f5ffd5b816020840160208301375f602083830101528093505050509250925092565b600281106108ce57634e487b7160e01b5f52602160045260245ffd5b9052565b60018060a01b0385168152836020820152608060408201525f83518060808401525f5b8181101561091257602081870181015160a08684010152016108f5565b505f60a0828501015260a0601f19601f83011684010191505061067d60608301846108b256fea2646970667358221220609c4615ff27663e1ecd6b881b3e1ebd2aa8f44db9b93c910969590559d1796364736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0XW\x80cv\x1D\xE1\x9F\x14a\0\xE5W\x80c\x8D\xA5\xCB[\x14a\x01\x0CW\x80c\x9A\xA9\xFD\xA5\x14a\x01\x1FW\x80c\xA7\x1F\x8D\xA0\x14a\x012W__\xFD[\x80c\x18o\x03T\x14a\0~W\x80c!j>\x9A\x14a\0\xADW\x80ccq\x0C\x05\x14a\0\xD0W[__\xFD[_Ta\0\x90\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xC0a\0\xBB6`\x04a\x054V[a\x01EV[`@Q\x90\x15\x15\x81R` \x01a\0\xA4V[a\0\xE3a\0\xDE6`\x04a\x05rV[a\x02{V[\0[a\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\0\x90\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xE3a\x01-6`\x04a\x054V[a\x02\xFFV[a\0\xC0a\x01@6`\x04a\x05rV[a\x033V[_\x80a\x01Q\x83\x80a\x05\xE3V[`@Qa\x01_\x92\x91\x90a\x06-V[`@Q\x80\x91\x03\x90 \x90P_a\x01\xC0\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x07` \x89\x01\x89a\x05\xE3V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02%\x93\x92\x91\x90a\x06dV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02@W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02d\x91\x90a\x06\x86V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[\x81\x81a\x02\x87\x82\x82a\x033V[a\x02\xA4W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x02\xF8Wa\x02\xE6\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x02\xCFWa\x02\xCFa\x06\xADV[\x90P` \x02\x81\x01\x90a\x02\xE1\x91\x90a\x06\xC1V[a\x03\xFAV[\x80a\x02\xF0\x81a\x06\xDFV[\x91PPa\x02\xA6V[PPPPPV[\x80a\x03\t\x81a\x01EV[a\x03&W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03/\x82a\x03\xFAV[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x03\xEEW0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x03bWa\x03ba\x06\xADV[\x90P` \x02\x81\x01\x90a\x03t\x91\x90a\x06\xC1V[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\x90\x91\x90a\x07QV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xABW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCF\x91\x90a\x07\xA6V[a\x03\xDCW_\x91PPa\x03\xF4V[\x80a\x03\xE6\x81a\x06\xDFV[\x91PPa\x036V[P`\x01\x90P[\x92\x91PPV[_\x80\x80a\x04\x07\x84\x80a\x05\xE3V[\x81\x01\x90a\x04\x14\x91\x90a\x07\xD9V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16a\x04oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuInvalid target address`P\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`@QcF\x87!\xA7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\x87!\xA7\x90a\x04\xA5\x90\x87\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x08\xD2V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xC1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xE5\x91\x90a\x07\xA6V[\x90P\x80a\x02\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FModule transaction failed\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04fV[_` \x82\x84\x03\x12\x15a\x05DW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05ZW__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x05kW__\xFD[\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x05\x83W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\x99W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x05\xA9W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xBFW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x05\xD3W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x05\xF8W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x06\x12W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x06&W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\x06}`@\x83\x01\x84\x86a\x06<V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x06\x96W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x05kW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x06\xD5W__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x07\x06WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x07$W__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07CW__\xFD[\x806\x03\x82\x13\x15a\x06&W__\xFD[` \x81R_a\x07`\x83\x84a\x07\x0FV[`@` \x85\x01Ra\x07u``\x85\x01\x82\x84a\x06<V[\x91PPa\x07\x85` \x85\x01\x85a\x07\x0FV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x07\x9C\x83\x82\x84a\x06<V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x07\xB6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x05kW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x07\xEBW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\x01W__\xFD[\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08#W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x083W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08MWa\x08Ma\x07\xC5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x08|Wa\x08|a\x07\xC5V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x08\x93W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[`\x02\x81\x10a\x08\xCEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[`\x01\x80`\xA0\x1B\x03\x85\x16\x81R\x83` \x82\x01R`\x80`@\x82\x01R_\x83Q\x80`\x80\x84\x01R_[\x81\x81\x10\x15a\t\x12W` \x81\x87\x01\x81\x01Q`\xA0\x86\x84\x01\x01R\x01a\x08\xF5V[P_`\xA0\x82\x85\x01\x01R`\xA0`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x06}``\x83\x01\x84a\x08\xB2V\xFE\xA2dipfsX\"\x12 `\x9CF\x15\xFF'f>\x1E\xCDk\x88\x1B>\x1E\xBD*\xA8\xF4M\xB9\xB9<\x91\tiY\x05Y\xD1ycdsolcC\0\x08\x1C\x003",
    );
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    ///Container for all the [`SafeModule`](self) function calls.
    pub enum SafeModuleCalls {
        STAKE_REGISTRY(STAKE_REGISTRYCall),
        addPayload(addPayloadCall),
        addPayloadMulti(addPayloadMultiCall),
        owner(ownerCall),
        safe(safeCall),
        validatePayload(validatePayloadCall),
        validatePayloadMulti(validatePayloadMultiCall),
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
            [24u8, 111u8, 3u8, 84u8],
            [33u8, 106u8, 62u8, 154u8],
            [99u8, 113u8, 12u8, 5u8],
            [118u8, 29u8, 225u8, 159u8],
            [141u8, 165u8, 203u8, 91u8],
            [154u8, 169u8, 253u8, 165u8],
            [167u8, 31u8, 141u8, 160u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeModuleCalls {
        const NAME: &'static str = "SafeModuleCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 7usize;
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
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::validatePayload(_) => {
                    <validatePayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatePayloadMulti(_) => {
                    <validatePayloadMultiCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<SafeModuleCalls>] = &[
                {
                    fn safe(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <safeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::safe)
                    }
                    safe
                },
                {
                    fn validatePayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <validatePayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::validatePayload)
                    }
                    validatePayload
                },
                {
                    fn addPayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <addPayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::addPayloadMulti)
                    }
                    addPayloadMulti
                },
                {
                    fn STAKE_REGISTRY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::STAKE_REGISTRY)
                    }
                    STAKE_REGISTRY
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::owner)
                    }
                    owner
                },
                {
                    fn addPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <addPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::addPayload)
                    }
                    addPayload
                },
                {
                    fn validatePayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleCalls> {
                        <validatePayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleCalls::validatePayloadMulti)
                    }
                    validatePayloadMulti
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
            }
        }
    }
    ///Container for all the [`SafeModule`](self) custom errors.
    pub enum SafeModuleErrors {
        InvalidSignature(InvalidSignature),
    }
    #[automatically_derived]
    impl SafeModuleErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[139u8, 170u8, 87u8, 159u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeModuleErrors {
        const NAME: &'static str = "SafeModuleErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<SafeModuleErrors>] = &[
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeModuleErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeModuleErrors::InvalidSignature)
                    }
                    InvalidSignature
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
        _stakeRegistry: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<SafeModuleInstance<T, P, N>>,
    > {
        SafeModuleInstance::<T, P, N>::deploy(provider, _safe, _stakeRegistry)
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
        SafeModuleInstance::<T, P, N>::deploy_builder(provider, _safe, _stakeRegistry)
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
    > SafeModuleInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`SafeModule`](self) contract instance.

See the [wrapper's documentation](`SafeModuleInstance`) for more details.*/
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
        ) -> alloy_contract::Result<SafeModuleInstance<T, P, N>> {
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
    > SafeModuleInstance<T, P, N> {
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
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`safe`] function.
        pub fn safe(&self) -> alloy_contract::SolCallBuilder<T, &P, safeCall, N> {
            self.call_builder(&safeCall {})
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > SafeModuleInstance<T, P, N> {
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
