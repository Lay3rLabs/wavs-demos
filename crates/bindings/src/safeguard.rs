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
/**

Generated by the following Solidity interface...
```solidity
library Enum {
    type Operation is uint8;
}

interface SafeGuard {
    type ValidationStatus is uint8;

    error AsyncValidationRequired();
    error TransactionExpired();

    event ValidationStatusUpdated(bytes32 indexed approvedHash, ValidationStatus status);

    constructor(address payable _safe);

    function VALIDATION_TIMEOUT() external view returns (uint256);
    function checkAfterExecution(bytes32 txHash, bool success) external;
    function checkTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address payable refundReceiver, bytes memory signatures, address initiator) external view;
    function getTransactionStatus(bytes32 txHash) external view returns (ValidationStatus status, uint256 remainingTime);
    function handleAddPayload(bytes memory validationData, bytes memory signature) external;
    function initialize(address _serviceProvider) external;
    function initialized() external view returns (bool);
    function safe() external view returns (address payable);
    function serviceProvider() external view returns (address);
    function supportsInterface(bytes4 interfaceId) external pure returns (bool);
    function txDetails(bytes32) external view returns (ValidationStatus status, uint256 validationExpiry);
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
        "internalType": "address payable"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "VALIDATION_TIMEOUT",
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
        "name": "initiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
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
        "name": "remainingTime",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "handleAddPayload",
    "inputs": [
      {
        "name": "validationData",
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
    "name": "safe",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address payable"
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
        "name": "status",
        "type": "uint8",
        "internalType": "enum SafeGuard.ValidationStatus"
      },
      {
        "name": "validationExpiry",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "ValidationStatusUpdated",
    "inputs": [
      {
        "name": "approvedHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "status",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum SafeGuard.ValidationStatus"
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
    ///0x60a060405234801561000f575f5ffd5b50604051610dcf380380610dcf83398101604081905261002e91610099565b6001600160a01b0381166100885760405162461bcd60e51b815260206004820152601460248201527f496e76616c696420736166652061646472657373000000000000000000000000604482015260640160405180910390fd5b6001600160a01b03166080526100c6565b5f602082840312156100a9575f5ffd5b81516001600160a01b03811681146100bf575f5ffd5b9392505050565b608051610cd56100fa5f395f818161010f01528181610363015281816103c20152818161044d015261060e0152610cd55ff3fe608060405234801561000f575f5ffd5b50600436106100a5575f3560e01c80637b4f33731161006e5780637b4f33731461015c5780638d69e95e1461019657806393271368146101a857806394407465146101bb578063c4d66de8146101ce578063ccb2c7a4146101e1575f5ffd5b806273e1d7146100a957806301ffc9a7146100be578063158ef93e146100f7578063186f03541461010a57806375f0bb5214610149575b5f5ffd5b6100bc6100b7366004610828565b6101f8565b005b6100e26100cc366004610894565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b5f546100e290600160a01b900460ff1681565b6101317f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b6100bc610157366004610996565b610358565b61018861016a366004610a73565b600160208190525f9182526040909120805491015460ff9091169082565b6040516100ee929190610ab2565b5f54610131906001600160a01b031681565b6100bc6101b6366004610adc565b610603565b6101886101c9366004610a73565b6106b0565b6100bc6101dc366004610b06565b610718565b6101ea610e1081565b6040519081526020016100ee565b5f546001600160a01b0316331461026b5760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f61027884860186610b21565b90505f816020015161028b57600361028e565b60025b905060405180604001604052808260048111156102ad576102ad610a8a565b815260200183602001516102c1575f6102cd565b6102cd610e1042610b89565b905282515f9081526001602081905260409091208251815491929091839160ff199091169083600481111561030457610304610a8a565b02179055506020919091015160019091015581516040517f96d83666be19b73e365fb9e5785e6c1848a741b550bedf84f742ce52f5ddf5dd90610348908490610ba2565b60405180910390a2505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146103bf5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b6044820152606401610262565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561041c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104409190610bb0565b90505f6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663d8d11f788e8e8e8e8e8e8e8e8e61048660018e610bc7565b6040518b63ffffffff1660e01b81526004016104ab9a99989796959493929190610bea565b602060405180830381865afa1580156104c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104ea9190610bb0565b5f818152600160205260408120919250815460ff16600481111561051057610510610a8a565b0361052e576040516336fc571360e01b815260040160405180910390fd5b6003815460ff16600481111561054657610546610a8a565b036105935760405162461bcd60e51b815260206004820152601860248201527f5472616e73616374696f6e207761732072656a656374656400000000000000006044820152606401610262565b6002815460ff1660048111156105ab576105ab610a8a565b036105dd5780600101544211156105d5576040516338e5e54b60e21b815260040160405180910390fd5b5050506105f6565b6040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461066a5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b6044820152606401610262565b806106ac5760405162461bcd60e51b8152602060048201526012602482015271151c985b9cd858dd1a5bdb8819985a5b195960721b6044820152606401610262565b5050565b5f818152600160205260408120819081815460ff1660048111156106d6576106d6610a8a565b036106e657505f93849350915050565b5f428260010154116106f8575f610708565b4282600101546107089190610bc7565b915460ff16959194509092505050565b5f54600160a01b900460ff16156107675760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610262565b6001600160a01b0381166107bd5760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f766964657220616464726573736044820152606401610262565b5f80546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b5f5f83601f8401126107f3575f5ffd5b50813567ffffffffffffffff81111561080a575f5ffd5b602083019150836020828501011115610821575f5ffd5b9250929050565b5f5f5f5f6040858703121561083b575f5ffd5b843567ffffffffffffffff811115610851575f5ffd5b61085d878288016107e3565b909550935050602085013567ffffffffffffffff81111561087c575f5ffd5b610888878288016107e3565b95989497509550505050565b5f602082840312156108a4575f5ffd5b81356001600160e01b0319811681146108bb575f5ffd5b9392505050565b6001600160a01b03811681146108d6575f5ffd5b50565b80356108e4816108c2565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261090c575f5ffd5b813567ffffffffffffffff811115610926576109266108e9565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715610955576109556108e9565b60405281815283820160200185101561096c575f5ffd5b816020850160208301375f918101602001919091529392505050565b8035600281106108e4575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e0312156109b1575f5ffd5b6109ba8c6108d9565b9a5060208c0135995060408c013567ffffffffffffffff8111156109dc575f5ffd5b6109e88e828f016108fd565b9950506109f760608d01610988565b975060808c0135965060a08c0135955060c08c01359450610a1a60e08d016108d9565b9350610a296101008d016108d9565b92506101208c013567ffffffffffffffff811115610a45575f5ffd5b610a518e828f016108fd565b925050610a616101408d016108d9565b90509295989b509295989b9093969950565b5f60208284031215610a83575f5ffd5b5035919050565b634e487b7160e01b5f52602160045260245ffd5b60058110610aae57610aae610a8a565b9052565b60408101610ac08285610a9e565b8260208301529392505050565b803580151581146108e4575f5ffd5b5f5f60408385031215610aed575f5ffd5b82359150610afd60208401610acd565b90509250929050565b5f60208284031215610b16575f5ffd5b81356108bb816108c2565b5f6040828403128015610b32575f5ffd5b506040805190810167ffffffffffffffff81118282101715610b5657610b566108e9565b60405282358152610b6960208401610acd565b60208201529392505050565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610b9c57610b9c610b75565b92915050565b60208101610b9c8284610a9e565b5f60208284031215610bc0575f5ffd5b5051919050565b81810381811115610b9c57610b9c610b75565b60028110610aae57610aae610a8a565b60018060a01b038b16815289602082015261014060408201525f8951806101408401525f5b81811015610c2d576020818d01810151610160868401015201610c0f565b505f6101608285010152610160601f19601f830116840101915050610c55606083018a610bda565b8760808301528660a08301528560c0830152610c7c60e08301866001600160a01b03169052565b6001600160a01b039390931661010082015261012001529897505050505050505056fea2646970667358221220f82b0787449afe4ebbb24d20afdaa2d03302211e11a5e58017810d88d3b03a4764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa\r\xCF8\x03\x80a\r\xCF\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\x99V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01R\x7FInvalid safe address\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0\xC6V[_` \x82\x84\x03\x12\x15a\0\xA9W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xBFW__\xFD[\x93\x92PPPV[`\x80Qa\x0C\xD5a\0\xFA_9_\x81\x81a\x01\x0F\x01R\x81\x81a\x03c\x01R\x81\x81a\x03\xC2\x01R\x81\x81a\x04M\x01Ra\x06\x0E\x01Ra\x0C\xD5_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA5W_5`\xE0\x1C\x80c{O3s\x11a\0nW\x80c{O3s\x14a\x01\\W\x80c\x8Di\xE9^\x14a\x01\x96W\x80c\x93'\x13h\x14a\x01\xA8W\x80c\x94@te\x14a\x01\xBBW\x80c\xC4\xD6m\xE8\x14a\x01\xCEW\x80c\xCC\xB2\xC7\xA4\x14a\x01\xE1W__\xFD[\x80bs\xE1\xD7\x14a\0\xA9W\x80c\x01\xFF\xC9\xA7\x14a\0\xBEW\x80c\x15\x8E\xF9>\x14a\0\xF7W\x80c\x18o\x03T\x14a\x01\nW\x80cu\xF0\xBBR\x14a\x01IW[__\xFD[a\0\xBCa\0\xB76`\x04a\x08(V[a\x01\xF8V[\0[a\0\xE2a\0\xCC6`\x04a\x08\x94V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[_Ta\0\xE2\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xBCa\x01W6`\x04a\t\x96V[a\x03XV[a\x01\x88a\x01j6`\x04a\nsV[`\x01` \x81\x90R_\x91\x82R`@\x90\x91 \x80T\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xEE\x92\x91\x90a\n\xB2V[_Ta\x011\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xBCa\x01\xB66`\x04a\n\xDCV[a\x06\x03V[a\x01\x88a\x01\xC96`\x04a\nsV[a\x06\xB0V[a\0\xBCa\x01\xDC6`\x04a\x0B\x06V[a\x07\x18V[a\x01\xEAa\x0E\x10\x81V[`@Q\x90\x81R` \x01a\0\xEEV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_a\x02x\x84\x86\x01\x86a\x0B!V[\x90P_\x81` \x01Qa\x02\x8BW`\x03a\x02\x8EV[`\x02[\x90P`@Q\x80`@\x01`@R\x80\x82`\x04\x81\x11\x15a\x02\xADWa\x02\xADa\n\x8AV[\x81R` \x01\x83` \x01Qa\x02\xC1W_a\x02\xCDV[a\x02\xCDa\x0E\x10Ba\x0B\x89V[\x90R\x82Q_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x82Q\x81T\x91\x92\x90\x91\x83\x91`\xFF\x19\x90\x91\x16\x90\x83`\x04\x81\x11\x15a\x03\x04Wa\x03\x04a\n\x8AV[\x02\x17\x90UP` \x91\x90\x91\x01Q`\x01\x90\x91\x01U\x81Q`@Q\x7F\x96\xD86f\xBE\x19\xB7>6_\xB9\xE5x^l\x18H\xA7A\xB5P\xBE\xDF\x84\xF7B\xCER\xF5\xDD\xF5\xDD\x90a\x03H\x90\x84\x90a\x0B\xA2V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x02bV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04@\x91\x90a\x0B\xB0V[\x90P_`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xD8\xD1\x1Fx\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8Ea\x04\x86`\x01\x8Ea\x0B\xC7V[`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xAB\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xEAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xEA\x91\x90a\x0B\xB0V[_\x81\x81R`\x01` R`@\x81 \x91\x92P\x81T`\xFF\x16`\x04\x81\x11\x15a\x05\x10Wa\x05\x10a\n\x8AV[\x03a\x05.W`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81T`\xFF\x16`\x04\x81\x11\x15a\x05FWa\x05Fa\n\x8AV[\x03a\x05\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FTransaction was rejected\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02bV[`\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\x05\xABWa\x05\xABa\n\x8AV[\x03a\x05\xDDW\x80`\x01\x01TB\x11\x15a\x05\xD5W`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x05\xF6V[`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x02bV[\x80a\x06\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x19\x98Z[\x19Y`r\x1B`D\x82\x01R`d\x01a\x02bV[PPV[_\x81\x81R`\x01` R`@\x81 \x81\x90\x81\x81T`\xFF\x16`\x04\x81\x11\x15a\x06\xD6Wa\x06\xD6a\n\x8AV[\x03a\x06\xE6WP_\x93\x84\x93P\x91PPV[_B\x82`\x01\x01T\x11a\x06\xF8W_a\x07\x08V[B\x82`\x01\x01Ta\x07\x08\x91\x90a\x0B\xC7V[\x91T`\xFF\x16\x95\x91\x94P\x90\x92PPPV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x07gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x02bV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x02bV[_\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[__\x83`\x1F\x84\x01\x12a\x07\xF3W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\nW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08!W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\x08;W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08QW__\xFD[a\x08]\x87\x82\x88\x01a\x07\xE3V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08|W__\xFD[a\x08\x88\x87\x82\x88\x01a\x07\xE3V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x08\xA4W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\xBBW__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xD6W__\xFD[PV[\x805a\x08\xE4\x81a\x08\xC2V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\t\x0CW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t&Wa\t&a\x08\xE9V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\tUWa\tUa\x08\xE9V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\tlW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x02\x81\x10a\x08\xE4W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\t\xB1W__\xFD[a\t\xBA\x8Ca\x08\xD9V[\x9AP` \x8C\x015\x99P`@\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDCW__\xFD[a\t\xE8\x8E\x82\x8F\x01a\x08\xFDV[\x99PPa\t\xF7``\x8D\x01a\t\x88V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\n\x1A`\xE0\x8D\x01a\x08\xD9V[\x93Pa\n)a\x01\0\x8D\x01a\x08\xD9V[\x92Pa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nEW__\xFD[a\nQ\x8E\x82\x8F\x01a\x08\xFDV[\x92PPa\naa\x01@\x8D\x01a\x08\xD9V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\n\x83W__\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x81\x10a\n\xAEWa\n\xAEa\n\x8AV[\x90RV[`@\x81\x01a\n\xC0\x82\x85a\n\x9EV[\x82` \x83\x01R\x93\x92PPPV[\x805\x80\x15\x15\x81\x14a\x08\xE4W__\xFD[__`@\x83\x85\x03\x12\x15a\n\xEDW__\xFD[\x825\x91Pa\n\xFD` \x84\x01a\n\xCDV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x16W__\xFD[\x815a\x08\xBB\x81a\x08\xC2V[_`@\x82\x84\x03\x12\x80\x15a\x0B2W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BVWa\x0BVa\x08\xE9V[`@R\x825\x81Ra\x0Bi` \x84\x01a\n\xCDV[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\x9CWa\x0B\x9Ca\x0BuV[\x92\x91PPV[` \x81\x01a\x0B\x9C\x82\x84a\n\x9EV[_` \x82\x84\x03\x12\x15a\x0B\xC0W__\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0B\x9CWa\x0B\x9Ca\x0BuV[`\x02\x81\x10a\n\xAEWa\n\xAEa\n\x8AV[`\x01\x80`\xA0\x1B\x03\x8B\x16\x81R\x89` \x82\x01Ra\x01@`@\x82\x01R_\x89Q\x80a\x01@\x84\x01R_[\x81\x81\x10\x15a\x0C-W` \x81\x8D\x01\x81\x01Qa\x01`\x86\x84\x01\x01R\x01a\x0C\x0FV[P_a\x01`\x82\x85\x01\x01Ra\x01``\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x0CU``\x83\x01\x8Aa\x0B\xDAV[\x87`\x80\x83\x01R\x86`\xA0\x83\x01R\x85`\xC0\x83\x01Ra\x0C|`\xE0\x83\x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16a\x01\0\x82\x01Ra\x01 \x01R\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \xF8+\x07\x87D\x9A\xFEN\xBB\xB2M \xAF\xDA\xA2\xD03\x02!\x1E\x11\xA5\xE5\x80\x17\x81\r\x88\xD3\xB0:GdsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100a5575f3560e01c80637b4f33731161006e5780637b4f33731461015c5780638d69e95e1461019657806393271368146101a857806394407465146101bb578063c4d66de8146101ce578063ccb2c7a4146101e1575f5ffd5b806273e1d7146100a957806301ffc9a7146100be578063158ef93e146100f7578063186f03541461010a57806375f0bb5214610149575b5f5ffd5b6100bc6100b7366004610828565b6101f8565b005b6100e26100cc366004610894565b6001600160e01b03191663736bd41d60e11b1490565b60405190151581526020015b60405180910390f35b5f546100e290600160a01b900460ff1681565b6101317f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b6100bc610157366004610996565b610358565b61018861016a366004610a73565b600160208190525f9182526040909120805491015460ff9091169082565b6040516100ee929190610ab2565b5f54610131906001600160a01b031681565b6100bc6101b6366004610adc565b610603565b6101886101c9366004610a73565b6106b0565b6100bc6101dc366004610b06565b610718565b6101ea610e1081565b6040519081526020016100ee565b5f546001600160a01b0316331461026b5760405162461bcd60e51b815260206004820152602c60248201527f4f6e6c7920736572766963652070726f76696465722063616e2063616c6c207460448201526b3434b990333ab731ba34b7b760a11b60648201526084015b60405180910390fd5b5f61027884860186610b21565b90505f816020015161028b57600361028e565b60025b905060405180604001604052808260048111156102ad576102ad610a8a565b815260200183602001516102c1575f6102cd565b6102cd610e1042610b89565b905282515f9081526001602081905260409091208251815491929091839160ff199091169083600481111561030457610304610a8a565b02179055506020919091015160019091015581516040517f96d83666be19b73e365fb9e5785e6c1848a741b550bedf84f742ce52f5ddf5dd90610348908490610ba2565b60405180910390a2505050505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146103bf5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b6044820152606401610262565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663affed0e06040518163ffffffff1660e01b8152600401602060405180830381865afa15801561041c573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104409190610bb0565b90505f6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663d8d11f788e8e8e8e8e8e8e8e8e61048660018e610bc7565b6040518b63ffffffff1660e01b81526004016104ab9a99989796959493929190610bea565b602060405180830381865afa1580156104c6573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104ea9190610bb0565b5f818152600160205260408120919250815460ff16600481111561051057610510610a8a565b0361052e576040516336fc571360e01b815260040160405180910390fd5b6003815460ff16600481111561054657610546610a8a565b036105935760405162461bcd60e51b815260206004820152601860248201527f5472616e73616374696f6e207761732072656a656374656400000000000000006044820152606401610262565b6002815460ff1660048111156105ab576105ab610a8a565b036105dd5780600101544211156105d5576040516338e5e54b60e21b815260040160405180910390fd5b5050506105f6565b6040516336fc571360e01b815260040160405180910390fd5b5050505050505050505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461066a5760405162461bcd60e51b815260206004820152600c60248201526b155b985d5d1a1bdc9a5e995960a21b6044820152606401610262565b806106ac5760405162461bcd60e51b8152602060048201526012602482015271151c985b9cd858dd1a5bdb8819985a5b195960721b6044820152606401610262565b5050565b5f818152600160205260408120819081815460ff1660048111156106d6576106d6610a8a565b036106e657505f93849350915050565b5f428260010154116106f8575f610708565b4282600101546107089190610bc7565b915460ff16959194509092505050565b5f54600160a01b900460ff16156107675760405162461bcd60e51b8152602060048201526013602482015272105b1c9958591e481a5b9a5d1a585b1a5e9959606a1b6044820152606401610262565b6001600160a01b0381166107bd5760405162461bcd60e51b815260206004820181905260248201527f496e76616c696420736572766963652070726f766964657220616464726573736044820152606401610262565b5f80546001600160a81b0319166001600160a01b0390921691909117600160a01b179055565b5f5f83601f8401126107f3575f5ffd5b50813567ffffffffffffffff81111561080a575f5ffd5b602083019150836020828501011115610821575f5ffd5b9250929050565b5f5f5f5f6040858703121561083b575f5ffd5b843567ffffffffffffffff811115610851575f5ffd5b61085d878288016107e3565b909550935050602085013567ffffffffffffffff81111561087c575f5ffd5b610888878288016107e3565b95989497509550505050565b5f602082840312156108a4575f5ffd5b81356001600160e01b0319811681146108bb575f5ffd5b9392505050565b6001600160a01b03811681146108d6575f5ffd5b50565b80356108e4816108c2565b919050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f83011261090c575f5ffd5b813567ffffffffffffffff811115610926576109266108e9565b604051601f8201601f19908116603f0116810167ffffffffffffffff81118282101715610955576109556108e9565b60405281815283820160200185101561096c575f5ffd5b816020850160208301375f918101602001919091529392505050565b8035600281106108e4575f5ffd5b5f5f5f5f5f5f5f5f5f5f5f6101608c8e0312156109b1575f5ffd5b6109ba8c6108d9565b9a5060208c0135995060408c013567ffffffffffffffff8111156109dc575f5ffd5b6109e88e828f016108fd565b9950506109f760608d01610988565b975060808c0135965060a08c0135955060c08c01359450610a1a60e08d016108d9565b9350610a296101008d016108d9565b92506101208c013567ffffffffffffffff811115610a45575f5ffd5b610a518e828f016108fd565b925050610a616101408d016108d9565b90509295989b509295989b9093969950565b5f60208284031215610a83575f5ffd5b5035919050565b634e487b7160e01b5f52602160045260245ffd5b60058110610aae57610aae610a8a565b9052565b60408101610ac08285610a9e565b8260208301529392505050565b803580151581146108e4575f5ffd5b5f5f60408385031215610aed575f5ffd5b82359150610afd60208401610acd565b90509250929050565b5f60208284031215610b16575f5ffd5b81356108bb816108c2565b5f6040828403128015610b32575f5ffd5b506040805190810167ffffffffffffffff81118282101715610b5657610b566108e9565b60405282358152610b6960208401610acd565b60208201529392505050565b634e487b7160e01b5f52601160045260245ffd5b80820180821115610b9c57610b9c610b75565b92915050565b60208101610b9c8284610a9e565b5f60208284031215610bc0575f5ffd5b5051919050565b81810381811115610b9c57610b9c610b75565b60028110610aae57610aae610a8a565b60018060a01b038b16815289602082015261014060408201525f8951806101408401525f5b81811015610c2d576020818d01810151610160868401015201610c0f565b505f6101608285010152610160601f19601f830116840101915050610c55606083018a610bda565b8760808301528660a08301528560c0830152610c7c60e08301866001600160a01b03169052565b6001600160a01b039390931661010082015261012001529897505050505050505056fea2646970667358221220f82b0787449afe4ebbb24d20afdaa2d03302211e11a5e58017810d88d3b03a4764736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xA5W_5`\xE0\x1C\x80c{O3s\x11a\0nW\x80c{O3s\x14a\x01\\W\x80c\x8Di\xE9^\x14a\x01\x96W\x80c\x93'\x13h\x14a\x01\xA8W\x80c\x94@te\x14a\x01\xBBW\x80c\xC4\xD6m\xE8\x14a\x01\xCEW\x80c\xCC\xB2\xC7\xA4\x14a\x01\xE1W__\xFD[\x80bs\xE1\xD7\x14a\0\xA9W\x80c\x01\xFF\xC9\xA7\x14a\0\xBEW\x80c\x15\x8E\xF9>\x14a\0\xF7W\x80c\x18o\x03T\x14a\x01\nW\x80cu\xF0\xBBR\x14a\x01IW[__\xFD[a\0\xBCa\0\xB76`\x04a\x08(V[a\x01\xF8V[\0[a\0\xE2a\0\xCC6`\x04a\x08\x94V[`\x01`\x01`\xE0\x1B\x03\x19\x16csk\xD4\x1D`\xE1\x1B\x14\x90V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[_Ta\0\xE2\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\0\xBCa\x01W6`\x04a\t\x96V[a\x03XV[a\x01\x88a\x01j6`\x04a\nsV[`\x01` \x81\x90R_\x91\x82R`@\x90\x91 \x80T\x91\x01T`\xFF\x90\x91\x16\x90\x82V[`@Qa\0\xEE\x92\x91\x90a\n\xB2V[_Ta\x011\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\0\xBCa\x01\xB66`\x04a\n\xDCV[a\x06\x03V[a\x01\x88a\x01\xC96`\x04a\nsV[a\x06\xB0V[a\0\xBCa\x01\xDC6`\x04a\x0B\x06V[a\x07\x18V[a\x01\xEAa\x0E\x10\x81V[`@Q\x90\x81R` \x01a\0\xEEV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x02kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FOnly service provider can call t`D\x82\x01Rk44\xB9\x903:\xB71\xBA4\xB7\xB7`\xA1\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_a\x02x\x84\x86\x01\x86a\x0B!V[\x90P_\x81` \x01Qa\x02\x8BW`\x03a\x02\x8EV[`\x02[\x90P`@Q\x80`@\x01`@R\x80\x82`\x04\x81\x11\x15a\x02\xADWa\x02\xADa\n\x8AV[\x81R` \x01\x83` \x01Qa\x02\xC1W_a\x02\xCDV[a\x02\xCDa\x0E\x10Ba\x0B\x89V[\x90R\x82Q_\x90\x81R`\x01` \x81\x90R`@\x90\x91 \x82Q\x81T\x91\x92\x90\x91\x83\x91`\xFF\x19\x90\x91\x16\x90\x83`\x04\x81\x11\x15a\x03\x04Wa\x03\x04a\n\x8AV[\x02\x17\x90UP` \x91\x90\x91\x01Q`\x01\x90\x91\x01U\x81Q`@Q\x7F\x96\xD86f\xBE\x19\xB7>6_\xB9\xE5x^l\x18H\xA7A\xB5P\xBE\xDF\x84\xF7B\xCER\xF5\xDD\xF5\xDD\x90a\x03H\x90\x84\x90a\x0B\xA2V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x02bV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xFE\xD0\xE0`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x1CW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04@\x91\x90a\x0B\xB0V[\x90P_`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xD8\xD1\x1Fx\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8Ea\x04\x86`\x01\x8Ea\x0B\xC7V[`@Q\x8Bc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xAB\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\x0B\xEAV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xC6W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xEA\x91\x90a\x0B\xB0V[_\x81\x81R`\x01` R`@\x81 \x91\x92P\x81T`\xFF\x16`\x04\x81\x11\x15a\x05\x10Wa\x05\x10a\n\x8AV[\x03a\x05.W`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81T`\xFF\x16`\x04\x81\x11\x15a\x05FWa\x05Fa\n\x8AV[\x03a\x05\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FTransaction was rejected\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x02bV[`\x02\x81T`\xFF\x16`\x04\x81\x11\x15a\x05\xABWa\x05\xABa\n\x8AV[\x03a\x05\xDDW\x80`\x01\x01TB\x11\x15a\x05\xD5W`@Qc8\xE5\xE5K`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x05\xF6V[`@Qc6\xFCW\x13`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06jW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01Rk\x15[\x98]]\x1A\x1B\xDC\x9A^\x99Y`\xA2\x1B`D\x82\x01R`d\x01a\x02bV[\x80a\x06\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01Rq\x15\x1C\x98[\x9C\xD8X\xDD\x1A[\xDB\x88\x19\x98Z[\x19Y`r\x1B`D\x82\x01R`d\x01a\x02bV[PPV[_\x81\x81R`\x01` R`@\x81 \x81\x90\x81\x81T`\xFF\x16`\x04\x81\x11\x15a\x06\xD6Wa\x06\xD6a\n\x8AV[\x03a\x06\xE6WP_\x93\x84\x93P\x91PPV[_B\x82`\x01\x01T\x11a\x06\xF8W_a\x07\x08V[B\x82`\x01\x01Ta\x07\x08\x91\x90a\x0B\xC7V[\x91T`\xFF\x16\x95\x91\x94P\x90\x92PPPV[_T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x07gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x10[\x1C\x99XY\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`j\x1B`D\x82\x01R`d\x01a\x02bV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x07\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FInvalid service provider address`D\x82\x01R`d\x01a\x02bV[_\x80T`\x01`\x01`\xA8\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17`\x01`\xA0\x1B\x17\x90UV[__\x83`\x1F\x84\x01\x12a\x07\xF3W__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\nW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08!W__\xFD[\x92P\x92\x90PV[____`@\x85\x87\x03\x12\x15a\x08;W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08QW__\xFD[a\x08]\x87\x82\x88\x01a\x07\xE3V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08|W__\xFD[a\x08\x88\x87\x82\x88\x01a\x07\xE3V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\x08\xA4W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x08\xBBW__\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xD6W__\xFD[PV[\x805a\x08\xE4\x81a\x08\xC2V[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\t\x0CW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t&Wa\t&a\x08\xE9V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\tUWa\tUa\x08\xE9V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\tlW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[\x805`\x02\x81\x10a\x08\xE4W__\xFD[___________a\x01`\x8C\x8E\x03\x12\x15a\t\xB1W__\xFD[a\t\xBA\x8Ca\x08\xD9V[\x9AP` \x8C\x015\x99P`@\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xDCW__\xFD[a\t\xE8\x8E\x82\x8F\x01a\x08\xFDV[\x99PPa\t\xF7``\x8D\x01a\t\x88V[\x97P`\x80\x8C\x015\x96P`\xA0\x8C\x015\x95P`\xC0\x8C\x015\x94Pa\n\x1A`\xE0\x8D\x01a\x08\xD9V[\x93Pa\n)a\x01\0\x8D\x01a\x08\xD9V[\x92Pa\x01 \x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nEW__\xFD[a\nQ\x8E\x82\x8F\x01a\x08\xFDV[\x92PPa\naa\x01@\x8D\x01a\x08\xD9V[\x90P\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_` \x82\x84\x03\x12\x15a\n\x83W__\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x05\x81\x10a\n\xAEWa\n\xAEa\n\x8AV[\x90RV[`@\x81\x01a\n\xC0\x82\x85a\n\x9EV[\x82` \x83\x01R\x93\x92PPPV[\x805\x80\x15\x15\x81\x14a\x08\xE4W__\xFD[__`@\x83\x85\x03\x12\x15a\n\xEDW__\xFD[\x825\x91Pa\n\xFD` \x84\x01a\n\xCDV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\x0B\x16W__\xFD[\x815a\x08\xBB\x81a\x08\xC2V[_`@\x82\x84\x03\x12\x80\x15a\x0B2W__\xFD[P`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0BVWa\x0BVa\x08\xE9V[`@R\x825\x81Ra\x0Bi` \x84\x01a\n\xCDV[` \x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0B\x9CWa\x0B\x9Ca\x0BuV[\x92\x91PPV[` \x81\x01a\x0B\x9C\x82\x84a\n\x9EV[_` \x82\x84\x03\x12\x15a\x0B\xC0W__\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x0B\x9CWa\x0B\x9Ca\x0BuV[`\x02\x81\x10a\n\xAEWa\n\xAEa\n\x8AV[`\x01\x80`\xA0\x1B\x03\x8B\x16\x81R\x89` \x82\x01Ra\x01@`@\x82\x01R_\x89Q\x80a\x01@\x84\x01R_[\x81\x81\x10\x15a\x0C-W` \x81\x8D\x01\x81\x01Qa\x01`\x86\x84\x01\x01R\x01a\x0C\x0FV[P_a\x01`\x82\x85\x01\x01Ra\x01``\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PPa\x0CU``\x83\x01\x8Aa\x0B\xDAV[\x87`\x80\x83\x01R\x86`\xA0\x83\x01R\x85`\xC0\x83\x01Ra\x0C|`\xE0\x83\x01\x86`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16a\x01\0\x82\x01Ra\x01 \x01R\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 \xF8+\x07\x87D\x9A\xFEN\xBB\xB2M \xAF\xDA\xA2\xD03\x02!\x1E\x11\xA5\xE5\x80\x17\x81\r\x88\xD3\xB0:GdsolcC\0\x08\x1C\x003",
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
    /**Event with signature `ValidationStatusUpdated(bytes32,uint8)` and selector `0x96d83666be19b73e365fb9e5785e6c1848a741b550bedf84f742ce52f5ddf5dd`.
    ```solidity
    event ValidationStatusUpdated(bytes32 indexed approvedHash, ValidationStatus status);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ValidationStatusUpdated {
        #[allow(missing_docs)]
        pub approvedHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidationStatusUpdated {
            type DataTuple<'a> = (ValidationStatus,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ValidationStatusUpdated(bytes32,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    150u8, 216u8, 54u8, 102u8, 190u8, 25u8, 183u8, 62u8, 54u8, 95u8, 185u8, 229u8,
                    120u8, 94u8, 108u8, 24u8, 72u8, 167u8, 65u8, 181u8, 80u8, 190u8, 223u8, 132u8,
                    247u8, 66u8, 206u8, 82u8, 245u8, 221u8, 245u8, 221u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { approvedHash: topics.1, status: data.0 }
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
                (<ValidationStatus as alloy_sol_types::SolType>::tokenize(&self.status),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.approvedHash.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.approvedHash);
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
    /**Function with signature `VALIDATION_TIMEOUT()` and selector `0xccb2c7a4`.
    ```solidity
    function VALIDATION_TIMEOUT() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VALIDATION_TIMEOUTCall {}
    ///Container type for the return parameters of the [`VALIDATION_TIMEOUT()`](VALIDATION_TIMEOUTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VALIDATION_TIMEOUTReturn {
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
            impl ::core::convert::From<VALIDATION_TIMEOUTCall> for UnderlyingRustTuple<'_> {
                fn from(value: VALIDATION_TIMEOUTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VALIDATION_TIMEOUTCall {
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
            impl ::core::convert::From<VALIDATION_TIMEOUTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VALIDATION_TIMEOUTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VALIDATION_TIMEOUTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VALIDATION_TIMEOUTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = VALIDATION_TIMEOUTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VALIDATION_TIMEOUT()";
            const SELECTOR: [u8; 4] = [204u8, 178u8, 199u8, 164u8];
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
    function checkTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, bytes memory signatures, address initiator) external view;
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
        pub initiator: alloy::sol_types::private::Address,
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
                        value.initiator,
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
                        initiator: tuple.10,
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
                        &self.initiator,
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
    /**Function with signature `getTransactionStatus(bytes32)` and selector `0x94407465`.
    ```solidity
    function getTransactionStatus(bytes32 txHash) external view returns (ValidationStatus status, uint256 remainingTime);
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
            type UnderlyingSolTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ValidationStatus as alloy::sol_types::SolType>::RustType,
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
                    (value.status, value.remainingTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransactionStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { status: tuple.0, remainingTime: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTransactionStatusCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTransactionStatusReturn;
            type ReturnTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::Uint<256>);
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
    /**Function with signature `handleAddPayload(bytes,bytes)` and selector `0x0073e1d7`.
    ```solidity
    function handleAddPayload(bytes memory validationData, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct handleAddPayloadCall {
        pub validationData: alloy::sol_types::private::Bytes,
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
                    (value.validationData, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for handleAddPayloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validationData: tuple.0, signature: tuple.1 }
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
                        &self.validationData,
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
    /**Function with signature `txDetails(bytes32)` and selector `0x7b4f3373`.
    ```solidity
    function txDetails(bytes32) external view returns (ValidationStatus status, uint256 validationExpiry);
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
        pub status: <ValidationStatus as alloy::sol_types::SolType>::RustType,
        pub validationExpiry: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ValidationStatus as alloy::sol_types::SolType>::RustType,
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
                    (value.status, value.validationExpiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for txDetailsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { status: tuple.0, validationExpiry: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for txDetailsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = txDetailsReturn;
            type ReturnTuple<'a> = (ValidationStatus, alloy::sol_types::sol_data::Uint<256>);
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
    ///Container for all the [`SafeGuard`](self) function calls.
    pub enum SafeGuardCalls {
        VALIDATION_TIMEOUT(VALIDATION_TIMEOUTCall),
        checkAfterExecution(checkAfterExecutionCall),
        checkTransaction(checkTransactionCall),
        getTransactionStatus(getTransactionStatusCall),
        handleAddPayload(handleAddPayloadCall),
        initialize(initializeCall),
        initialized(initializedCall),
        safe(safeCall),
        serviceProvider(serviceProviderCall),
        supportsInterface(supportsInterfaceCall),
        txDetails(txDetailsCall),
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
            [117u8, 240u8, 187u8, 82u8],
            [123u8, 79u8, 51u8, 115u8],
            [141u8, 105u8, 233u8, 94u8],
            [147u8, 39u8, 19u8, 104u8],
            [148u8, 64u8, 116u8, 101u8],
            [196u8, 214u8, 109u8, 232u8],
            [204u8, 178u8, 199u8, 164u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardCalls {
        const NAME: &'static str = "SafeGuardCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::VALIDATION_TIMEOUT(_) => {
                    <VALIDATION_TIMEOUTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkAfterExecution(_) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkTransaction(_) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTransactionStatus(_) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::handleAddPayload(_) => {
                    <handleAddPayloadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initialized(_) => <initializedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::safe(_) => <safeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceProvider(_) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::txDetails(_) => <txDetailsCall as alloy_sol_types::SolCall>::SELECTOR,
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
                    fn VALIDATION_TIMEOUT(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeGuardCalls> {
                        <VALIDATION_TIMEOUTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeGuardCalls::VALIDATION_TIMEOUT)
                    }
                    VALIDATION_TIMEOUT
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
                Self::VALIDATION_TIMEOUT(inner) => {
                    <VALIDATION_TIMEOUTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::VALIDATION_TIMEOUT(inner) => {
                    <VALIDATION_TIMEOUTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkAfterExecution(inner) => {
                    <checkAfterExecutionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::checkTransaction(inner) => {
                    <checkTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getTransactionStatus(inner) => {
                    <getTransactionStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::safe(inner) => {
                    <safeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceProvider(inner) => {
                    <serviceProviderCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::txDetails(inner) => {
                    <txDetailsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) custom errors.
    pub enum SafeGuardErrors {
        AsyncValidationRequired(AsyncValidationRequired),
        TransactionExpired(TransactionExpired),
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
            &[[54u8, 252u8, 87u8, 19u8], [227u8, 151u8, 149u8, 44u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeGuardErrors {
        const NAME: &'static str = "SafeGuardErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 2usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AsyncValidationRequired(_) => {
                    <AsyncValidationRequired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransactionExpired(_) => {
                    <TransactionExpired as alloy_sol_types::SolError>::SELECTOR
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
            }
        }
    }
    ///Container for all the [`SafeGuard`](self) events.
    pub enum SafeGuardEvents {
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
        pub const SELECTORS: &'static [[u8; 32usize]] = &[[
            150u8, 216u8, 54u8, 102u8, 190u8, 25u8, 183u8, 62u8, 54u8, 95u8, 185u8, 229u8, 120u8,
            94u8, 108u8, 24u8, 72u8, 167u8, 65u8, 181u8, 80u8, 190u8, 223u8, 132u8, 247u8, 66u8,
            206u8, 82u8, 245u8, 221u8, 245u8, 221u8,
        ]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SafeGuardEvents {
        const NAME: &'static str = "SafeGuardEvents";
        const COUNT: usize = 1usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
                Self::ValidationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
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
        ///Creates a new call builder for the [`VALIDATION_TIMEOUT`] function.
        pub fn VALIDATION_TIMEOUT(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, VALIDATION_TIMEOUTCall, N> {
            self.call_builder(&VALIDATION_TIMEOUTCall {})
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
            initiator: alloy::sol_types::private::Address,
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
                initiator,
            })
        }
        ///Creates a new call builder for the [`getTransactionStatus`] function.
        pub fn getTransactionStatus(
            &self,
            txHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTransactionStatusCall, N> {
            self.call_builder(&getTransactionStatusCall { txHash })
        }
        ///Creates a new call builder for the [`handleAddPayload`] function.
        pub fn handleAddPayload(
            &self,
            validationData: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, handleAddPayloadCall, N> {
            self.call_builder(&handleAddPayloadCall { validationData, signature })
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
        ///Creates a new call builder for the [`txDetails`] function.
        pub fn txDetails(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, txDetailsCall, N> {
            self.call_builder(&txDetailsCall { _0 })
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
        ///Creates a new event filter for the [`ValidationStatusUpdated`] event.
        pub fn ValidationStatusUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidationStatusUpdated, N> {
            self.event_filter::<ValidationStatusUpdated>()
        }
    }
}
