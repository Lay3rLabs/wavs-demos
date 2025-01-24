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

interface WavsConditionalMarket {
    error InvalidCollateralToken();
    error InvalidOutcome();
    error InvalidSignature();
    error MarketAlreadyResolved();

    event MarketCreated(bytes32 indexed questionId, bytes32 indexed conditionId);
    event MarketResolved(bytes32 indexed questionId, bytes32 indexed conditionId, uint256 outcome);

    constructor(address _stakeRegistry, address _conditionalTokens);

    function STAKE_REGISTRY() external view returns (address);
    function addPayload(IWavsSDK.SignedPayload memory signedPayload) external;
    function addPayloadMulti(IWavsSDK.SignedPayload[] memory signedPayloads) external;
    function conditionalTokens() external view returns (address);
    function createMarket(address collateralToken, bytes32 questionId, uint256 outcomeSlotCount) external;
    function marketOutcome(bytes32) external view returns (uint256);
    function marketResolved(bytes32) external view returns (bool);
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
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_conditionalTokens",
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
    "name": "conditionalTokens",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IConditionalTokens"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createMarket",
    "inputs": [
      {
        "name": "collateralToken",
        "type": "address",
        "internalType": "contract IERC20"
      },
      {
        "name": "questionId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "outcomeSlotCount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "marketOutcome",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "marketResolved",
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
    "type": "error",
    "name": "InvalidCollateralToken",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOutcome",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MarketAlreadyResolved",
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
pub mod WavsConditionalMarket {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405234801561000f575f5ffd5b50604051610c30380380610c3083398101604081905261002e91610060565b6001600160a01b039182166080521660a052610091565b80516001600160a01b038116811461005b575f5ffd5b919050565b5f5f60408385031215610071575f5ffd5b61007a83610045565b915061008860208401610045565b90509250929050565b60805160a051610b636100cd5f395f818160d60152818161031a015281816103b1015261067201525f8181610128015261024e0152610b635ff3fe608060405234801561000f575f5ffd5b5060043610610090575f3560e01c8063761de19f11610063578063761de19f146101235780638c93344f1461014a5780639aa9fda51461016c578063a71f8da01461017f578063ce1f456714610192575f5ffd5b8063216a3e9a146100945780634c305710146100bc5780635bd9e299146100d157806363710c0514610110575b5f5ffd5b6100a76100a236600461077f565b6101bf565b60405190151581526020015b60405180910390f35b6100cf6100ca3660046107bd565b6102f5565b005b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100b3565b6100cf61011e3660046107fa565b610442565b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6100a761015836600461086b565b5f6020819052908152604090205460ff1681565b6100cf61017a36600461077f565b6104c6565b6100a761018d3660046107fa565b6104fa565b6101b16101a036600461086b565b60016020525f908152604090205481565b6040519081526020016100b3565b5f806101cb8380610882565b6040516101d99291906108cc565b604051809103902090505f61023a826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836102816020890189610882565b6040518463ffffffff1660e01b815260040161029f93929190610903565b602060405180830381865afa1580156102ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102de9190610925565b6001600160e01b0319918216911614949350505050565b604051634296357160e11b815230600482015260248101839052604481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063852c6ae290606401602060405180830381865afa158015610367573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038b919061094c565b60405163365bb9d560e21b815230600482015260248101859052604481018490529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d96ee754906064015f604051808303815f87803b1580156103fa575f5ffd5b505af115801561040c573d5f5f3e3d5ffd5b50506040518392508591507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a350505050565b818161044e82826104fa565b61046b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156104bf576104ad85858363ffffffff1681811061049657610496610963565b90506020028101906104a89190610977565b6105c1565b806104b781610995565b91505061046d565b5050505050565b806104d0816101bf565b6104ed57604051638baa579f60e01b815260040160405180910390fd5b6104f6826105c1565b5050565b5f805b63ffffffff81168311156105b5573063216a3e9a858563ffffffff851681811061052957610529610963565b905060200281019061053b9190610977565b6040518263ffffffff1660e01b81526004016105579190610a07565b602060405180830381865afa158015610572573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105969190610a5c565b6105a3575f9150506105bb565b806105ad81610995565b9150506104fd565b50600190505b92915050565b5f6105cc8280610882565b8101906105d99190610a7b565b80515f9081526020819052604090205490915060ff161561060d5760405163aa43cb2d60e01b815260040160405180910390fd5b6001816040015111156106335760405163c74a206d60e01b815260040160405180910390fd5b80515f90815260208181526040808320805460ff1916600190811790915581850180518651865291909352922091909155815190516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163c49298ac916106a290610736565b6040518363ffffffff1660e01b81526004016106bf929190610ae0565b5f604051808303815f87803b1580156106d6575f5ffd5b505af11580156106e8573d5f5f3e3d5ffd5b505050508060200151815f01517f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d1836040015160405161072a91815260200190565b60405180910390a35050565b60408051600280825260608083018452925f929190602083019080368337019050509050600181848151811061076e5761076e610963565b602090810291909101015292915050565b5f6020828403121561078f575f5ffd5b813567ffffffffffffffff8111156107a5575f5ffd5b8201604081850312156107b6575f5ffd5b9392505050565b5f5f5f606084860312156107cf575f5ffd5b83356001600160a01b03811681146107e5575f5ffd5b95602085013595506040909401359392505050565b5f5f6020838503121561080b575f5ffd5b823567ffffffffffffffff811115610821575f5ffd5b8301601f81018513610831575f5ffd5b803567ffffffffffffffff811115610847575f5ffd5b8560208260051b840101111561085b575f5ffd5b6020919091019590945092505050565b5f6020828403121561087b575f5ffd5b5035919050565b5f5f8335601e19843603018112610897575f5ffd5b83018035915067ffffffffffffffff8211156108b1575f5ffd5b6020019150368190038213156108c5575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61091c6040830184866108db565b95945050505050565b5f60208284031215610935575f5ffd5b81516001600160e01b0319811681146107b6575f5ffd5b5f6020828403121561095c575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261098b575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff81036109bc57634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e198436030181126109da575f5ffd5b830160208101925035905067ffffffffffffffff8111156109f9575f5ffd5b8036038213156108c5575f5ffd5b602081525f610a1683846109c5565b60406020850152610a2b6060850182846108db565b915050610a3b60208501856109c5565b848303601f19016040860152610a528382846108db565b9695505050505050565b5f60208284031215610a6c575f5ffd5b815180151581146107b6575f5ffd5b5f6060828403128015610a8c575f5ffd5b506040516060810167ffffffffffffffff81118282101715610abc57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b5f60408201848352604060208401528084518083526060850191506020860192505f5b81811015610b21578351835260209384019390920191600101610b03565b5090969550505050505056fea26469706673582212207a5dc298a257036afb5a67be155c7a7ab06a20bc1df591976b60bfd43121f3da64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\x0C08\x03\x80a\x0C0\x839\x81\x01`@\x81\x90Ra\0.\x91a\0`V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Ra\0\x91V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0[W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\0qW__\xFD[a\0z\x83a\0EV[\x91Pa\0\x88` \x84\x01a\0EV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Qa\x0Bca\0\xCD_9_\x81\x81`\xD6\x01R\x81\x81a\x03\x1A\x01R\x81\x81a\x03\xB1\x01Ra\x06r\x01R_\x81\x81a\x01(\x01Ra\x02N\x01Ra\x0Bc_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x90W_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0cW\x80cv\x1D\xE1\x9F\x14a\x01#W\x80c\x8C\x934O\x14a\x01JW\x80c\x9A\xA9\xFD\xA5\x14a\x01lW\x80c\xA7\x1F\x8D\xA0\x14a\x01\x7FW\x80c\xCE\x1FEg\x14a\x01\x92W__\xFD[\x80c!j>\x9A\x14a\0\x94W\x80cL0W\x10\x14a\0\xBCW\x80c[\xD9\xE2\x99\x14a\0\xD1W\x80ccq\x0C\x05\x14a\x01\x10W[__\xFD[a\0\xA7a\0\xA26`\x04a\x07\x7FV[a\x01\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a\x07\xBDV[a\x02\xF5V[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xCFa\x01\x1E6`\x04a\x07\xFAV[a\x04BV[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA7a\x01X6`\x04a\x08kV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xCFa\x01z6`\x04a\x07\x7FV[a\x04\xC6V[a\0\xA7a\x01\x8D6`\x04a\x07\xFAV[a\x04\xFAV[a\x01\xB1a\x01\xA06`\x04a\x08kV[`\x01` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB3V[_\x80a\x01\xCB\x83\x80a\x08\x82V[`@Qa\x01\xD9\x92\x91\x90a\x08\xCCV[`@Q\x80\x91\x03\x90 \x90P_a\x02:\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x81` \x89\x01\x89a\x08\x82V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x9F\x93\x92\x91\x90a\t\x03V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDE\x91\x90a\t%V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[`@QcB\x965q`\xE1\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85,j\xE2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03gW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8B\x91\x90a\tLV[`@Qc6[\xB9\xD5`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD9n\xE7T\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xFAW__\xFD[PZ\xF1\x15\x80\x15a\x04\x0CW=__>=_\xFD[PP`@Q\x83\x92P\x85\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3PPPPV[\x81\x81a\x04N\x82\x82a\x04\xFAV[a\x04kW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04\xBFWa\x04\xAD\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04\x96Wa\x04\x96a\tcV[\x90P` \x02\x81\x01\x90a\x04\xA8\x91\x90a\twV[a\x05\xC1V[\x80a\x04\xB7\x81a\t\x95V[\x91PPa\x04mV[PPPPPV[\x80a\x04\xD0\x81a\x01\xBFV[a\x04\xEDW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xF6\x82a\x05\xC1V[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x05\xB5W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x05)Wa\x05)a\tcV[\x90P` \x02\x81\x01\x90a\x05;\x91\x90a\twV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05W\x91\x90a\n\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\n\\V[a\x05\xA3W_\x91PPa\x05\xBBV[\x80a\x05\xAD\x81a\t\x95V[\x91PPa\x04\xFDV[P`\x01\x90P[\x92\x91PPV[_a\x05\xCC\x82\x80a\x08\x82V[\x81\x01\x90a\x05\xD9\x91\x90a\n{V[\x80Q_\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x15a\x06\rW`@Qc\xAAC\xCB-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`@\x01Q\x11\x15a\x063W`@Qc\xC7J m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q_\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x81\x85\x01\x80Q\x86Q\x86R\x91\x90\x93R\x92 \x91\x90\x91U\x81Q\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xC4\x92\x98\xAC\x91a\x06\xA2\x90a\x076V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBF\x92\x91\x90a\n\xE0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD6W__\xFD[PZ\xF1\x15\x80\x15a\x06\xE8W=__>=_\xFD[PPPP\x80` \x01Q\x81_\x01Q\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1\x83`@\x01Q`@Qa\x07*\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92_\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P`\x01\x81\x84\x81Q\x81\x10a\x07nWa\x07na\tcV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\x8FW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA5W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x07\xB6W__\xFD[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x07\xCFW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE5W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x08\x0BW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08!W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x081W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08GW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x08[W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x08{W__\xFD[P5\x91\x90PV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x97W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB1W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xC5W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\t\x1C`@\x83\x01\x84\x86a\x08\xDBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\t5W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xB6W__\xFD[_` \x82\x84\x03\x12\x15a\t\\W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\t\x8BW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\t\xBCWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xDAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF9W__\xFD[\x806\x03\x82\x13\x15a\x08\xC5W__\xFD[` \x81R_a\n\x16\x83\x84a\t\xC5V[`@` \x85\x01Ra\n+``\x85\x01\x82\x84a\x08\xDBV[\x91PPa\n;` \x85\x01\x85a\t\xC5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\nR\x83\x82\x84a\x08\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\nlW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xB6W__\xFD[_``\x82\x84\x03\x12\x80\x15a\n\x8CW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBCWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a\x0B!W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x03V[P\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 z]\xC2\x98\xA2W\x03j\xFBZg\xBE\x15\\zz\xB0j \xBC\x1D\xF5\x91\x97k`\xBF\xD41!\xF3\xDAdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610090575f3560e01c8063761de19f11610063578063761de19f146101235780638c93344f1461014a5780639aa9fda51461016c578063a71f8da01461017f578063ce1f456714610192575f5ffd5b8063216a3e9a146100945780634c305710146100bc5780635bd9e299146100d157806363710c0514610110575b5f5ffd5b6100a76100a236600461077f565b6101bf565b60405190151581526020015b60405180910390f35b6100cf6100ca3660046107bd565b6102f5565b005b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100b3565b6100cf61011e3660046107fa565b610442565b6100f87f000000000000000000000000000000000000000000000000000000000000000081565b6100a761015836600461086b565b5f6020819052908152604090205460ff1681565b6100cf61017a36600461077f565b6104c6565b6100a761018d3660046107fa565b6104fa565b6101b16101a036600461086b565b60016020525f908152604090205481565b6040519081526020016100b3565b5f806101cb8380610882565b6040516101d99291906108cc565b604051809103902090505f61023a826040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836102816020890189610882565b6040518463ffffffff1660e01b815260040161029f93929190610903565b602060405180830381865afa1580156102ba573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102de9190610925565b6001600160e01b0319918216911614949350505050565b604051634296357160e11b815230600482015260248101839052604481018290525f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063852c6ae290606401602060405180830381865afa158015610367573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061038b919061094c565b60405163365bb9d560e21b815230600482015260248101859052604481018490529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d96ee754906064015f604051808303815f87803b1580156103fa575f5ffd5b505af115801561040c573d5f5f3e3d5ffd5b50506040518392508591507fbb83cc2a13d3abaf2ac6bcb1b96e108f6ce75dc8ebdb6086f805eab09bd534db905f90a350505050565b818161044e82826104fa565b61046b57604051638baa579f60e01b815260040160405180910390fd5b5f5b63ffffffff81168411156104bf576104ad85858363ffffffff1681811061049657610496610963565b90506020028101906104a89190610977565b6105c1565b806104b781610995565b91505061046d565b5050505050565b806104d0816101bf565b6104ed57604051638baa579f60e01b815260040160405180910390fd5b6104f6826105c1565b5050565b5f805b63ffffffff81168311156105b5573063216a3e9a858563ffffffff851681811061052957610529610963565b905060200281019061053b9190610977565b6040518263ffffffff1660e01b81526004016105579190610a07565b602060405180830381865afa158015610572573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105969190610a5c565b6105a3575f9150506105bb565b806105ad81610995565b9150506104fd565b50600190505b92915050565b5f6105cc8280610882565b8101906105d99190610a7b565b80515f9081526020819052604090205490915060ff161561060d5760405163aa43cb2d60e01b815260040160405180910390fd5b6001816040015111156106335760405163c74a206d60e01b815260040160405180910390fd5b80515f90815260208181526040808320805460ff1916600190811790915581850180518651865291909352922091909155815190516001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169163c49298ac916106a290610736565b6040518363ffffffff1660e01b81526004016106bf929190610ae0565b5f604051808303815f87803b1580156106d6575f5ffd5b505af11580156106e8573d5f5f3e3d5ffd5b505050508060200151815f01517f4c0ae238f67b2112791d959bb102a0b0600c3a53837c3d7931958d562a63b9d1836040015160405161072a91815260200190565b60405180910390a35050565b60408051600280825260608083018452925f929190602083019080368337019050509050600181848151811061076e5761076e610963565b602090810291909101015292915050565b5f6020828403121561078f575f5ffd5b813567ffffffffffffffff8111156107a5575f5ffd5b8201604081850312156107b6575f5ffd5b9392505050565b5f5f5f606084860312156107cf575f5ffd5b83356001600160a01b03811681146107e5575f5ffd5b95602085013595506040909401359392505050565b5f5f6020838503121561080b575f5ffd5b823567ffffffffffffffff811115610821575f5ffd5b8301601f81018513610831575f5ffd5b803567ffffffffffffffff811115610847575f5ffd5b8560208260051b840101111561085b575f5ffd5b6020919091019590945092505050565b5f6020828403121561087b575f5ffd5b5035919050565b5f5f8335601e19843603018112610897575f5ffd5b83018035915067ffffffffffffffff8211156108b1575f5ffd5b6020019150368190038213156108c5575f5ffd5b9250929050565b818382375f9101908152919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b838152604060208201525f61091c6040830184866108db565b95945050505050565b5f60208284031215610935575f5ffd5b81516001600160e01b0319811681146107b6575f5ffd5b5f6020828403121561095c575f5ffd5b5051919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261098b575f5ffd5b9190910192915050565b5f63ffffffff821663ffffffff81036109bc57634e487b7160e01b5f52601160045260245ffd5b60010192915050565b5f5f8335601e198436030181126109da575f5ffd5b830160208101925035905067ffffffffffffffff8111156109f9575f5ffd5b8036038213156108c5575f5ffd5b602081525f610a1683846109c5565b60406020850152610a2b6060850182846108db565b915050610a3b60208501856109c5565b848303601f19016040860152610a528382846108db565b9695505050505050565b5f60208284031215610a6c575f5ffd5b815180151581146107b6575f5ffd5b5f6060828403128015610a8c575f5ffd5b506040516060810167ffffffffffffffff81118282101715610abc57634e487b7160e01b5f52604160045260245ffd5b60409081528335825260208085013590830152928301359281019290925250919050565b5f60408201848352604060208401528084518083526060850191506020860192505f5b81811015610b21578351835260209384019390920191600101610b03565b5090969550505050505056fea26469706673582212207a5dc298a257036afb5a67be155c7a7ab06a20bc1df591976b60bfd43121f3da64736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\x90W_5`\xE0\x1C\x80cv\x1D\xE1\x9F\x11a\0cW\x80cv\x1D\xE1\x9F\x14a\x01#W\x80c\x8C\x934O\x14a\x01JW\x80c\x9A\xA9\xFD\xA5\x14a\x01lW\x80c\xA7\x1F\x8D\xA0\x14a\x01\x7FW\x80c\xCE\x1FEg\x14a\x01\x92W__\xFD[\x80c!j>\x9A\x14a\0\x94W\x80cL0W\x10\x14a\0\xBCW\x80c[\xD9\xE2\x99\x14a\0\xD1W\x80ccq\x0C\x05\x14a\x01\x10W[__\xFD[a\0\xA7a\0\xA26`\x04a\x07\x7FV[a\x01\xBFV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCFa\0\xCA6`\x04a\x07\xBDV[a\x02\xF5V[\0[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xB3V[a\0\xCFa\x01\x1E6`\x04a\x07\xFAV[a\x04BV[a\0\xF8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\0\xA7a\x01X6`\x04a\x08kV[_` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[a\0\xCFa\x01z6`\x04a\x07\x7FV[a\x04\xC6V[a\0\xA7a\x01\x8D6`\x04a\x07\xFAV[a\x04\xFAV[a\x01\xB1a\x01\xA06`\x04a\x08kV[`\x01` R_\x90\x81R`@\x90 T\x81V[`@Q\x90\x81R` \x01a\0\xB3V[_\x80a\x01\xCB\x83\x80a\x08\x82V[`@Qa\x01\xD9\x92\x91\x90a\x08\xCCV[`@Q\x80\x91\x03\x90 \x90P_a\x02:\x82`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x02\x81` \x89\x01\x89a\x08\x82V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x9F\x93\x92\x91\x90a\t\x03V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xBAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDE\x91\x90a\t%V[`\x01`\x01`\xE0\x1B\x03\x19\x91\x82\x16\x91\x16\x14\x94\x93PPPPV[`@QcB\x965q`\xE1\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R_\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x85,j\xE2\x90`d\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03gW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\x8B\x91\x90a\tLV[`@Qc6[\xB9\xD5`\xE2\x1B\x81R0`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x81\x01\x84\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD9n\xE7T\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x03\xFAW__\xFD[PZ\xF1\x15\x80\x15a\x04\x0CW=__>=_\xFD[PP`@Q\x83\x92P\x85\x91P\x7F\xBB\x83\xCC*\x13\xD3\xAB\xAF*\xC6\xBC\xB1\xB9n\x10\x8Fl\xE7]\xC8\xEB\xDB`\x86\xF8\x05\xEA\xB0\x9B\xD54\xDB\x90_\x90\xA3PPPPV[\x81\x81a\x04N\x82\x82a\x04\xFAV[a\x04kW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[c\xFF\xFF\xFF\xFF\x81\x16\x84\x11\x15a\x04\xBFWa\x04\xAD\x85\x85\x83c\xFF\xFF\xFF\xFF\x16\x81\x81\x10a\x04\x96Wa\x04\x96a\tcV[\x90P` \x02\x81\x01\x90a\x04\xA8\x91\x90a\twV[a\x05\xC1V[\x80a\x04\xB7\x81a\t\x95V[\x91PPa\x04mV[PPPPPV[\x80a\x04\xD0\x81a\x01\xBFV[a\x04\xEDW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xF6\x82a\x05\xC1V[PPV[_\x80[c\xFF\xFF\xFF\xFF\x81\x16\x83\x11\x15a\x05\xB5W0c!j>\x9A\x85\x85c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x05)Wa\x05)a\tcV[\x90P` \x02\x81\x01\x90a\x05;\x91\x90a\twV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05W\x91\x90a\n\x07V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x96\x91\x90a\n\\V[a\x05\xA3W_\x91PPa\x05\xBBV[\x80a\x05\xAD\x81a\t\x95V[\x91PPa\x04\xFDV[P`\x01\x90P[\x92\x91PPV[_a\x05\xCC\x82\x80a\x08\x82V[\x81\x01\x90a\x05\xD9\x91\x90a\n{V[\x80Q_\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16\x15a\x06\rW`@Qc\xAAC\xCB-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`@\x01Q\x11\x15a\x063W`@Qc\xC7J m`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q_\x90\x81R` \x81\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x81\x85\x01\x80Q\x86Q\x86R\x91\x90\x93R\x92 \x91\x90\x91U\x81Q\x90Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xC4\x92\x98\xAC\x91a\x06\xA2\x90a\x076V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06\xBF\x92\x91\x90a\n\xE0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD6W__\xFD[PZ\xF1\x15\x80\x15a\x06\xE8W=__>=_\xFD[PPPP\x80` \x01Q\x81_\x01Q\x7FL\n\xE28\xF6{!\x12y\x1D\x95\x9B\xB1\x02\xA0\xB0`\x0C:S\x83|=y1\x95\x8DV*c\xB9\xD1\x83`@\x01Q`@Qa\x07*\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3PPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92_\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P`\x01\x81\x84\x81Q\x81\x10a\x07nWa\x07na\tcV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x07\x8FW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA5W__\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x07\xB6W__\xFD[\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x07\xCFW__\xFD[\x835`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07\xE5W__\xFD[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[__` \x83\x85\x03\x12\x15a\x08\x0BW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08!W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x081W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08GW__\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a\x08[W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a\x08{W__\xFD[P5\x91\x90PV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x08\x97W__\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xB1W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x08\xC5W__\xFD[\x92P\x92\x90PV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x83\x81R`@` \x82\x01R_a\t\x1C`@\x83\x01\x84\x86a\x08\xDBV[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\t5W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x07\xB6W__\xFD[_` \x82\x84\x03\x12\x15a\t\\W__\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\t\x8BW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\t\xBCWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\t\xDAW__\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xF9W__\xFD[\x806\x03\x82\x13\x15a\x08\xC5W__\xFD[` \x81R_a\n\x16\x83\x84a\t\xC5V[`@` \x85\x01Ra\n+``\x85\x01\x82\x84a\x08\xDBV[\x91PPa\n;` \x85\x01\x85a\t\xC5V[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\nR\x83\x82\x84a\x08\xDBV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\nlW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07\xB6W__\xFD[_``\x82\x84\x03\x12\x80\x15a\n\x8CW__\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\n\xBCWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x01\x84\x83R`@` \x84\x01R\x80\x84Q\x80\x83R``\x85\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a\x0B!W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\x03V[P\x90\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 z]\xC2\x98\xA2W\x03j\xFBZg\xBE\x15\\zz\xB0j \xBC\x1D\xF5\x91\x97k`\xBF\xD41!\xF3\xDAdsolcC\0\x08\x1C\x003",
    );
    /**Custom error with signature `InvalidCollateralToken()` and selector `0x74003c0a`.
```solidity
error InvalidCollateralToken();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCollateralToken {}
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
        impl ::core::convert::From<InvalidCollateralToken> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCollateralToken) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCollateralToken {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCollateralToken {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCollateralToken()";
            const SELECTOR: [u8; 4] = [116u8, 0u8, 60u8, 10u8];
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
    /**Custom error with signature `InvalidOutcome()` and selector `0xc74a206d`.
```solidity
error InvalidOutcome();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOutcome {}
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
        impl ::core::convert::From<InvalidOutcome> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOutcome) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOutcome {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOutcome {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOutcome()";
            const SELECTOR: [u8; 4] = [199u8, 74u8, 32u8, 109u8];
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
    /**Custom error with signature `MarketAlreadyResolved()` and selector `0xaa43cb2d`.
```solidity
error MarketAlreadyResolved();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MarketAlreadyResolved {}
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
        impl ::core::convert::From<MarketAlreadyResolved> for UnderlyingRustTuple<'_> {
            fn from(value: MarketAlreadyResolved) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MarketAlreadyResolved {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MarketAlreadyResolved {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MarketAlreadyResolved()";
            const SELECTOR: [u8; 4] = [170u8, 67u8, 203u8, 45u8];
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
    /**Constructor`.
```solidity
constructor(address _stakeRegistry, address _conditionalTokens);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _stakeRegistry: alloy::sol_types::private::Address,
        pub _conditionalTokens: alloy::sol_types::private::Address,
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
                    (value._stakeRegistry, value._conditionalTokens)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _stakeRegistry: tuple.0,
                        _conditionalTokens: tuple.1,
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
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._conditionalTokens,
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
    /**Function with signature `createMarket(address,bytes32,uint256)` and selector `0x4c305710`.
```solidity
function createMarket(address collateralToken, bytes32 questionId, uint256 outcomeSlotCount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createMarketCall {
        pub collateralToken: alloy::sol_types::private::Address,
        pub questionId: alloy::sol_types::private::FixedBytes<32>,
        pub outcomeSlotCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`createMarket(address,bytes32,uint256)`](createMarketCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createMarketReturn {}
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<createMarketCall> for UnderlyingRustTuple<'_> {
                fn from(value: createMarketCall) -> Self {
                    (value.collateralToken, value.questionId, value.outcomeSlotCount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createMarketCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        collateralToken: tuple.0,
                        questionId: tuple.1,
                        outcomeSlotCount: tuple.2,
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
            impl ::core::convert::From<createMarketReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createMarketReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createMarketReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createMarketCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createMarketReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createMarket(address,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [76u8, 48u8, 87u8, 16u8];
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
                        &self.collateralToken,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.questionId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.outcomeSlotCount),
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
    /**Function with signature `marketOutcome(bytes32)` and selector `0xce1f4567`.
```solidity
function marketOutcome(bytes32) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketOutcomeCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`marketOutcome(bytes32)`](marketOutcomeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketOutcomeReturn {
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
            impl ::core::convert::From<marketOutcomeCall> for UnderlyingRustTuple<'_> {
                fn from(value: marketOutcomeCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for marketOutcomeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<marketOutcomeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: marketOutcomeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for marketOutcomeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for marketOutcomeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = marketOutcomeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "marketOutcome(bytes32)";
            const SELECTOR: [u8; 4] = [206u8, 31u8, 69u8, 103u8];
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
    /**Function with signature `marketResolved(bytes32)` and selector `0x8c93344f`.
```solidity
function marketResolved(bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketResolvedCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`marketResolved(bytes32)`](marketResolvedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct marketResolvedReturn {
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
            impl ::core::convert::From<marketResolvedCall> for UnderlyingRustTuple<'_> {
                fn from(value: marketResolvedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for marketResolvedCall {
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
            impl ::core::convert::From<marketResolvedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: marketResolvedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for marketResolvedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for marketResolvedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = marketResolvedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "marketResolved(bytes32)";
            const SELECTOR: [u8; 4] = [140u8, 147u8, 52u8, 79u8];
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
    ///Container for all the [`WavsConditionalMarket`](self) function calls.
    pub enum WavsConditionalMarketCalls {
        STAKE_REGISTRY(STAKE_REGISTRYCall),
        addPayload(addPayloadCall),
        addPayloadMulti(addPayloadMultiCall),
        conditionalTokens(conditionalTokensCall),
        createMarket(createMarketCall),
        marketOutcome(marketOutcomeCall),
        marketResolved(marketResolvedCall),
        validatePayload(validatePayloadCall),
        validatePayloadMulti(validatePayloadMultiCall),
    }
    #[automatically_derived]
    impl WavsConditionalMarketCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [33u8, 106u8, 62u8, 154u8],
            [76u8, 48u8, 87u8, 16u8],
            [91u8, 217u8, 226u8, 153u8],
            [99u8, 113u8, 12u8, 5u8],
            [118u8, 29u8, 225u8, 159u8],
            [140u8, 147u8, 52u8, 79u8],
            [154u8, 169u8, 253u8, 165u8],
            [167u8, 31u8, 141u8, 160u8],
            [206u8, 31u8, 69u8, 103u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsConditionalMarketCalls {
        const NAME: &'static str = "WavsConditionalMarketCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 9usize;
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
                Self::conditionalTokens(_) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createMarket(_) => {
                    <createMarketCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::marketOutcome(_) => {
                    <marketOutcomeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::marketResolved(_) => {
                    <marketResolvedCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<WavsConditionalMarketCalls>] = &[
                {
                    fn validatePayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <validatePayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::validatePayload)
                    }
                    validatePayload
                },
                {
                    fn createMarket(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <createMarketCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::createMarket)
                    }
                    createMarket
                },
                {
                    fn conditionalTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <conditionalTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::conditionalTokens)
                    }
                    conditionalTokens
                },
                {
                    fn addPayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <addPayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::addPayloadMulti)
                    }
                    addPayloadMulti
                },
                {
                    fn STAKE_REGISTRY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <STAKE_REGISTRYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::STAKE_REGISTRY)
                    }
                    STAKE_REGISTRY
                },
                {
                    fn marketResolved(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <marketResolvedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::marketResolved)
                    }
                    marketResolved
                },
                {
                    fn addPayload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <addPayloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::addPayload)
                    }
                    addPayload
                },
                {
                    fn validatePayloadMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <validatePayloadMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::validatePayloadMulti)
                    }
                    validatePayloadMulti
                },
                {
                    fn marketOutcome(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketCalls> {
                        <marketOutcomeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketCalls::marketOutcome)
                    }
                    marketOutcome
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
                Self::conditionalTokens(inner) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createMarket(inner) => {
                    <createMarketCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::marketOutcome(inner) => {
                    <marketOutcomeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::marketResolved(inner) => {
                    <marketResolvedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::conditionalTokens(inner) => {
                    <conditionalTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createMarket(inner) => {
                    <createMarketCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::marketOutcome(inner) => {
                    <marketOutcomeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::marketResolved(inner) => {
                    <marketResolvedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`WavsConditionalMarket`](self) custom errors.
    pub enum WavsConditionalMarketErrors {
        InvalidCollateralToken(InvalidCollateralToken),
        InvalidOutcome(InvalidOutcome),
        InvalidSignature(InvalidSignature),
        MarketAlreadyResolved(MarketAlreadyResolved),
    }
    #[automatically_derived]
    impl WavsConditionalMarketErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [116u8, 0u8, 60u8, 10u8],
            [139u8, 170u8, 87u8, 159u8],
            [170u8, 67u8, 203u8, 45u8],
            [199u8, 74u8, 32u8, 109u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsConditionalMarketErrors {
        const NAME: &'static str = "WavsConditionalMarketErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidCollateralToken(_) => {
                    <InvalidCollateralToken as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOutcome(_) => {
                    <InvalidOutcome as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MarketAlreadyResolved(_) => {
                    <MarketAlreadyResolved as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<WavsConditionalMarketErrors>] = &[
                {
                    fn InvalidCollateralToken(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketErrors> {
                        <InvalidCollateralToken as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketErrors::InvalidCollateralToken)
                    }
                    InvalidCollateralToken
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn MarketAlreadyResolved(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketErrors> {
                        <MarketAlreadyResolved as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketErrors::MarketAlreadyResolved)
                    }
                    MarketAlreadyResolved
                },
                {
                    fn InvalidOutcome(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsConditionalMarketErrors> {
                        <InvalidOutcome as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsConditionalMarketErrors::InvalidOutcome)
                    }
                    InvalidOutcome
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
                Self::InvalidCollateralToken(inner) => {
                    <InvalidCollateralToken as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOutcome(inner) => {
                    <InvalidOutcome as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MarketAlreadyResolved(inner) => {
                    <MarketAlreadyResolved as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidCollateralToken(inner) => {
                    <InvalidCollateralToken as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOutcome(inner) => {
                    <InvalidOutcome as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MarketAlreadyResolved(inner) => {
                    <MarketAlreadyResolved as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`WavsConditionalMarket`](self) events.
    pub enum WavsConditionalMarketEvents {
        MarketCreated(MarketCreated),
        MarketResolved(MarketResolved),
    }
    #[automatically_derived]
    impl WavsConditionalMarketEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for WavsConditionalMarketEvents {
        const NAME: &'static str = "WavsConditionalMarketEvents";
        const COUNT: usize = 2usize;
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
    impl alloy_sol_types::private::IntoLogData for WavsConditionalMarketEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MarketCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MarketResolved(inner) => {
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
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`WavsConditionalMarket`](self) contract instance.

See the [wrapper's documentation](`WavsConditionalMarketInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WavsConditionalMarketInstance<T, P, N> {
        WavsConditionalMarketInstance::<T, P, N>::new(address, provider)
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _conditionalTokens: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<WavsConditionalMarketInstance<T, P, N>>,
    > {
        WavsConditionalMarketInstance::<
            T,
            P,
            N,
        >::deploy(provider, _stakeRegistry, _conditionalTokens)
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
        _stakeRegistry: alloy::sol_types::private::Address,
        _conditionalTokens: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        WavsConditionalMarketInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _stakeRegistry, _conditionalTokens)
    }
    /**A [`WavsConditionalMarket`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`WavsConditionalMarket`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WavsConditionalMarketInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WavsConditionalMarketInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WavsConditionalMarketInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsConditionalMarketInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`WavsConditionalMarket`](self) contract instance.

See the [wrapper's documentation](`WavsConditionalMarketInstance`) for more details.*/
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _conditionalTokens: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<WavsConditionalMarketInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _stakeRegistry,
                _conditionalTokens,
            );
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
            _stakeRegistry: alloy::sol_types::private::Address,
            _conditionalTokens: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _stakeRegistry,
                            _conditionalTokens,
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
    impl<T, P: ::core::clone::Clone, N> WavsConditionalMarketInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WavsConditionalMarketInstance<T, P, N> {
            WavsConditionalMarketInstance {
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
    > WavsConditionalMarketInstance<T, P, N> {
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
        ///Creates a new call builder for the [`conditionalTokens`] function.
        pub fn conditionalTokens(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, conditionalTokensCall, N> {
            self.call_builder(&conditionalTokensCall {})
        }
        ///Creates a new call builder for the [`createMarket`] function.
        pub fn createMarket(
            &self,
            collateralToken: alloy::sol_types::private::Address,
            questionId: alloy::sol_types::private::FixedBytes<32>,
            outcomeSlotCount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, createMarketCall, N> {
            self.call_builder(
                &createMarketCall {
                    collateralToken,
                    questionId,
                    outcomeSlotCount,
                },
            )
        }
        ///Creates a new call builder for the [`marketOutcome`] function.
        pub fn marketOutcome(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, marketOutcomeCall, N> {
            self.call_builder(&marketOutcomeCall { _0 })
        }
        ///Creates a new call builder for the [`marketResolved`] function.
        pub fn marketResolved(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, marketResolvedCall, N> {
            self.call_builder(&marketResolvedCall { _0 })
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
    > WavsConditionalMarketInstance<T, P, N> {
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
    }
}
