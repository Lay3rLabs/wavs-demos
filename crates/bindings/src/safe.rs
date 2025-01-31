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

interface Safe {
    event AddedOwner(address indexed owner);
    event ApproveHash(bytes32 indexed approvedHash, address indexed owner);
    event ChangedFallbackHandler(address indexed handler);
    event ChangedGuard(address indexed guard);
    event ChangedThreshold(uint256 threshold);
    event DisabledModule(address indexed module);
    event EnabledModule(address indexed module);
    event ExecutionFailure(bytes32 indexed txHash, uint256 payment);
    event ExecutionFromModuleFailure(address indexed module);
    event ExecutionFromModuleSuccess(address indexed module);
    event ExecutionSuccess(bytes32 indexed txHash, uint256 payment);
    event RemovedOwner(address indexed owner);
    event SafeReceived(address indexed sender, uint256 value);
    event SafeSetup(address indexed initiator, address[] owners, uint256 threshold, address initializer, address fallbackHandler);
    event SignMsg(bytes32 indexed msgHash);

    constructor();

    fallback() external;

    receive() external payable;

    function VERSION() external view returns (string memory);
    function addOwnerWithThreshold(address owner, uint256 _threshold) external;
    function approveHash(bytes32 hashToApprove) external;
    function approvedHashes(address, bytes32) external view returns (uint256);
    function changeThreshold(uint256 _threshold) external;
    function checkNSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures, uint256 requiredSignatures) external view;
    function checkSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures) external view;
    function disableModule(address prevModule, address module) external;
    function domainSeparator() external view returns (bytes32);
    function enableModule(address module) external;
    function encodeTransactionData(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes memory);
    function execTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address payable refundReceiver, bytes memory signatures) external payable returns (bool success);
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success, bytes memory returnData);
    function getChainId() external view returns (uint256);
    function getModulesPaginated(address start, uint256 pageSize) external view returns (address[] memory array, address next);
    function getOwners() external view returns (address[] memory);
    function getStorageAt(uint256 offset, uint256 length) external view returns (bytes memory);
    function getThreshold() external view returns (uint256);
    function getTransactionHash(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes32);
    function isModuleEnabled(address module) external view returns (bool);
    function isOwner(address owner) external view returns (bool);
    function nonce() external view returns (uint256);
    function removeOwner(address prevOwner, address owner, uint256 _threshold) external;
    function setFallbackHandler(address handler) external;
    function setGuard(address guard) external;
    function setup(address[] memory _owners, uint256 _threshold, address to, bytes memory data, address fallbackHandler, address paymentToken, uint256 payment, address payable paymentReceiver) external;
    function signedMessages(bytes32) external view returns (uint256);
    function simulateAndRevert(address targetContract, bytes memory calldataPayload) external;
    function swapOwner(address prevOwner, address oldOwner, address newOwner) external;
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
    "type": "fallback",
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "VERSION",
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
    "name": "addOwnerWithThreshold",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "approveHash",
    "inputs": [
      {
        "name": "hashToApprove",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "approvedHashes",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "changeThreshold",
    "inputs": [
      {
        "name": "_threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "checkNSignatures",
    "inputs": [
      {
        "name": "dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signatures",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "requiredSignatures",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signatures",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "disableModule",
    "inputs": [
      {
        "name": "prevModule",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "module",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "domainSeparator",
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
    "name": "enableModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "encodeTransactionData",
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
        "internalType": "address"
      },
      {
        "name": "_nonce",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execTransaction",
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
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "execTransactionFromModule",
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
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "execTransactionFromModuleReturnData",
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
      }
    ],
    "outputs": [
      {
        "name": "success",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "returnData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getChainId",
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
    "name": "getModulesPaginated",
    "inputs": [
      {
        "name": "start",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "pageSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "array",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "next",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOwners",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStorageAt",
    "inputs": [
      {
        "name": "offset",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getThreshold",
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
    "name": "getTransactionHash",
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
        "internalType": "address"
      },
      {
        "name": "_nonce",
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
    "name": "isModuleEnabled",
    "inputs": [
      {
        "name": "module",
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
    "name": "isOwner",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "nonce",
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
    "name": "removeOwner",
    "inputs": [
      {
        "name": "prevOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_threshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setFallbackHandler",
    "inputs": [
      {
        "name": "handler",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setGuard",
    "inputs": [
      {
        "name": "guard",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setup",
    "inputs": [
      {
        "name": "_owners",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "_threshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "fallbackHandler",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "paymentToken",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "payment",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "paymentReceiver",
        "type": "address",
        "internalType": "address payable"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "signedMessages",
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
    "name": "simulateAndRevert",
    "inputs": [
      {
        "name": "targetContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "calldataPayload",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swapOwner",
    "inputs": [
      {
        "name": "prevOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "oldOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AddedOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ApproveHash",
    "inputs": [
      {
        "name": "approvedHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChangedFallbackHandler",
    "inputs": [
      {
        "name": "handler",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChangedGuard",
    "inputs": [
      {
        "name": "guard",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChangedThreshold",
    "inputs": [
      {
        "name": "threshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DisabledModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EnabledModule",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionFailure",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "payment",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionFromModuleFailure",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionFromModuleSuccess",
    "inputs": [
      {
        "name": "module",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExecutionSuccess",
    "inputs": [
      {
        "name": "txHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "payment",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RemovedOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SafeReceived",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "value",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SafeSetup",
    "inputs": [
      {
        "name": "initiator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "owners",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "threshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "initializer",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "fallbackHandler",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SignMsg",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
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
pub mod Safe {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600e575f5ffd5b506001600455612fd2806100215f395ff3fe6080604052600436106101d0575f3560e01c8063affed0e0116100f6578063e19a9dd911610094578063f08a032311610063578063f08a0323146105d2578063f698da25146105f1578063f8dc5dd914610605578063ffa1ad74146106245761020c565b8063e19a9dd914610561578063e318b52b14610580578063e75235b81461059f578063e86637db146105b35761020c565b8063cc2f8452116100d0578063cc2f8452146104d7578063d4d9bdcd14610504578063d8d11f7814610523578063e009cfde146105425761020c565b8063affed0e014610484578063b4faba0914610499578063b63e800d146104b85761020c565b80635624b25b1161016e5780636a7612021161013d5780636a761202146103fb5780637d8329741461040e578063934f3a1114610444578063a0e67e2b146104635761020c565b80635624b25b146103665780635ae6bd3714610392578063610b5925146103bd578063694e80c3146103dc5761020c565b80632f54bf6e116101aa5780632f54bf6e146102df5780633408e470146102fe578063468721a71461031a5780635229073f146103395761020c565b80630d582f131461026b57806312fb68e01461028c5780632d9ad53d146102ab5761020c565b3661020c5760405134815233907f3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d9060200160405180910390a2005b348015610217575f5ffd5b507f6c9a6c4a39284e37ed1cf53d337577d14212a4870fb976a4366c693b939918d580548061024257005b365f5f373360601b36525f5f601436015f5f855af190503d5f5f3e80610266573d5ffd5b503d5ff35b348015610276575f5ffd5b5061028a610285366004612504565b610654565b005b348015610297575f5ffd5b5061028a6102a63660046125cb565b6107a9565b3480156102b6575f5ffd5b506102ca6102c536600461263e565b610c3a565b60405190151581526020015b60405180910390f35b3480156102ea575f5ffd5b506102ca6102f936600461263e565b610c73565b348015610309575f5ffd5b50465b6040519081526020016102d6565b348015610325575f5ffd5b506102ca610334366004612667565b610ca9565b348015610344575f5ffd5b50610358610353366004612667565b610d7d565b6040516102d692919061270f565b348015610371575f5ffd5b50610385610380366004612729565b610db1565b6040516102d69190612749565b34801561039d575f5ffd5b5061030c6103ac36600461275b565b60076020525f908152604090205481565b3480156103c8575f5ffd5b5061028a6103d736600461263e565b610e2a565b3480156103e7575f5ffd5b5061028a6103f636600461275b565b610f61565b6102ca6104093660046127b6565b610fff565b348015610419575f5ffd5b5061030c610428366004612504565b600860209081525f928352604080842090915290825290205481565b34801561044f575f5ffd5b5061028a61045e366004612886565b611338565b34801561046e575f5ffd5b50610477611382565b6040516102d69190612934565b34801561048f575f5ffd5b5061030c60055481565b3480156104a4575f5ffd5b5061028a6104b3366004612946565b61146f565b3480156104c3575f5ffd5b5061028a6104d2366004612992565b61148e565b3480156104e2575f5ffd5b506104f66104f1366004612504565b61158d565b6040516102d6929190612a81565b34801561050f575f5ffd5b5061028a61051e36600461275b565b611744565b34801561052e575f5ffd5b5061030c61053d366004612aaa565b6117d7565b34801561054d575f5ffd5b5061028a61055c366004612b67565b611803565b34801561056c575f5ffd5b5061028a61057b36600461263e565b611923565b34801561058b575f5ffd5b5061028a61059a366004612b9e565b611a36565b3480156105aa575f5ffd5b5060045461030c565b3480156105be575f5ffd5b506103856105cd366004612aaa565b611c0d565b3480156105dd575f5ffd5b5061028a6105ec36600461263e565b611ce4565b3480156105fc575f5ffd5b5061030c611d2b565b348015610610575f5ffd5b5061028a61061f366004612be6565b611d81565b34801561062f575f5ffd5b5061038560405180604001604052806005815260200164312e342e3160d81b81525081565b61065c611ee9565b6001600160a01b0382161580159061067e57506001600160a01b038216600114155b801561069357506001600160a01b0382163014155b6106b85760405162461bcd60e51b81526004016106af90612c24565b60405180910390fd5b6001600160a01b038281165f9081526002602052604090205416156106ef5760405162461bcd60e51b81526004016106af90612c43565b60026020527fe90b7bceb6e7df5418fb78d8ee546e97c83a08bbccc01a0644d599ccd2a7c2e080546001600160a01b038481165f818152604081208054939094166001600160a01b03199384161790935560018352835490911617909155600380549161075b83612c76565b90915550506040516001600160a01b038316907f9465fa0c962cc76958e6373a993326400c1c94f8be2fe3a952adfa7f60b2ea26905f90a280600454146107a5576107a581610f61565b5050565b6107b4816041611f22565b825110156107ec5760405162461bcd60e51b8152602060048201526005602482015264047533032360dc1b60448201526064016106af565b5f80808080805b86811015610c2e576041818102890160208101516040820151919092015160ff16955090935091505f8490036109fe57885160208a01208a146108605760405162461bcd60e51b8152602060048201526005602482015264475330323760d81b60448201526064016106af565b9193508391610870876041611f22565b8210156108a75760405162461bcd60e51b8152602060048201526005602482015264475330323160d81b60448201526064016106af565b87516108b4836020611f59565b11156108ea5760405162461bcd60e51b815260206004820152600560248201526423a998191960d91b60448201526064016106af565b60208289018101518951909161090d908390610907908790611f59565b90611f59565b11156109435760405162461bcd60e51b8152602060048201526005602482015264475330323360d81b60448201526064016106af565b6040516320c13b0b60e01b8082528a8501602001916001600160a01b038916906320c13b0b90610979908f908690600401612c8e565b602060405180830381865afa158015610994573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109b89190612cb2565b6001600160e01b031916146109f75760405162461bcd60e51b815260206004820152600560248201526411d4cc0c8d60da1b60448201526064016106af565b5050610b9e565b8360ff16600103610a7f579193508391336001600160a01b0384161480610a4657506001600160a01b0385165f9081526008602090815260408083208d845290915290205415155b610a7a5760405162461bcd60e51b8152602060048201526005602482015264475330323560d81b60448201526064016106af565b610b9e565b601e8460ff161115610b41576040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018b9052600190605c0160405160208183030381529060405280519060200120600486610ae49190612cd9565b604080515f8152602081018083529390935260ff90911690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015610b30573d5f5f3e3d5ffd5b505050602060405103519450610b9e565b604080515f8152602081018083528c905260ff861691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa158015610b91573d5f5f3e3d5ffd5b5050506020604051035194505b856001600160a01b0316856001600160a01b0316118015610bd757506001600160a01b038581165f908152600260205260409020541615155b8015610bed57506001600160a01b038516600114155b610c215760405162461bcd60e51b815260206004820152600560248201526423a998191b60d91b60448201526064016106af565b93945084936001016107f3565b50505050505050505050565b5f60016001600160a01b03831614801590610c6d57506001600160a01b038281165f908152600160205260409020541615155b92915050565b5f6001600160a01b038216600114801590610c6d5750506001600160a01b039081165f9081526002602052604090205416151590565b5f33600114801590610cd15750335f908152600160205260409020546001600160a01b031615155b610d055760405162461bcd60e51b815260206004820152600560248201526411d4cc4c0d60da1b60448201526064016106af565b610d13858585855f19611f73565b90508015610d4a5760405133907f6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb8905f90a2610d75565b60405133907facd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd375905f90a25b949350505050565b5f6060610d8c86868686610ca9565b915060405160203d0181016040523d81523d5f602083013e8091505094509492505050565b60605f610dbf836020612cf2565b6001600160401b03811115610dd657610dd661252e565b6040519080825280601f01601f191660200182016040528015610e00576020820181803683370190505b5090505f5b83811015610e225784810154602080830284010152600101610e05565b509392505050565b610e32611ee9565b6001600160a01b03811615801590610e5457506001600160a01b038116600114155b610e885760405162461bcd60e51b8152602060048201526005602482015264475331303160d81b60448201526064016106af565b6001600160a01b038181165f908152600160205260409020541615610ed75760405162461bcd60e51b815260206004820152600560248201526423a998981960d91b60448201526064016106af565b600160208190527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f80546001600160a01b038481165f81815260408082208054949095166001600160a01b031994851617909455948552835490911681179092555190917fecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f844091a250565b610f69611ee9565b600354811115610f8b5760405162461bcd60e51b81526004016106af90612d09565b6001811015610fc45760405162461bcd60e51b815260206004820152600560248201526423a999181960d91b60448201526064016106af565b60048190556040518181527f610f7ff2b304ae8903c3de74c60c6ab1f7d6226b3f52c5161905bb5ad4039c939060200160405180910390a150565b5f5f5f6110178e8e8e8e8e8e8e8e8e8e600554611c0d565b600580549192505f61102883612c76565b9091555050805160208201209150611041828286611338565b505f61106b7f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c85490565b90506001600160a01b038116156110ec57806001600160a01b03166375f0bb528f8f8f8f8f8f8f8f8f8f8f336040518d63ffffffff1660e01b81526004016110be9c9b9a99989796959493929190612d5c565b5f604051808303815f87803b1580156110d5575f5ffd5b505af11580156110e7573d5f5f3e3d5ffd5b505050505b6111186110fb8a6109c4612e23565b603f6111088c6040612cf2565b6111129190612e36565b90611fb7565b611124906101f4612e23565b5a101561115b5760405162461bcd60e51b8152602060048201526005602482015264047533031360dc1b60448201526064016106af565b5f5a90506111c98f8f8f8f8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508e8c5f146111b6578e611f73565b6109c45a6111c49190612e55565b611f73565b93506111d65a8290611fcd565b905083806111e357508915155b806111ed57508715155b6112215760405162461bcd60e51b8152602060048201526005602482015264475330313360d81b60448201526064016106af565b5f881561123857611235828b8b8b8b611fe5565b90505b841561127d57837f442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e8260405161127091815260200190565b60405180910390a26112b8565b837f23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23826040516112af91815260200190565b60405180910390a25b50506001600160a01b0381161561132757604051631264e26d60e31b81526004810183905283151560248201526001600160a01b038216906393271368906044015f604051808303815f87803b158015611310575f5ffd5b505af1158015611322573d5f5f3e3d5ffd5b505050505b50509b9a5050505050505050505050565b600454806113705760405162461bcd60e51b8152602060048201526005602482015264475330303160d81b60448201526064016106af565b61137c848484846107a9565b50505050565b60605f6003546001600160401b0381111561139f5761139f61252e565b6040519080825280602002602001820160405280156113c8578160200160208202803683370190505b5060015f90815260026020527fe90b7bceb6e7df5418fb78d8ee546e97c83a08bbccc01a0644d599ccd2a7c2e054919250906001600160a01b03165b6001600160a01b038116600114611467578083838151811061142857611428612e68565b6001600160a01b039283166020918202929092018101919091529181165f9081526002909252604090912054168161145f81612c76565b925050611404565b509092915050565b5f5f825160208401855af4805f52503d6020523d5f60403e60403d015ffd5b6114cb8a8a808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508c92506120e9915050565b6001600160a01b038416156114e3576114e3846122bf565b6115228787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061232392505050565b811561153857611536825f60018685611fe5565b505b336001600160a01b03167f141df868a6331af528e38c83b7aa03edc19be66e37ae67f9285bf4f8e3c6a1a88b8b8b8b89604051611579959493929190612e7c565b60405180910390a250505050505050505050565b60605f6001600160a01b038416600114806115ac57506115ac84610c3a565b6115e05760405162461bcd60e51b8152602060048201526005602482015264475331303560d81b60448201526064016106af565b5f83116116175760405162461bcd60e51b815260206004820152600560248201526423a998981b60d91b60448201526064016106af565b826001600160401b0381111561162f5761162f61252e565b604051908082528060200260200182016040528015611658578160200160208202803683370190505b506001600160a01b038086165f90815260016020526040812054929450911691505b6001600160a01b0382161580159061169c57506001600160a01b038216600114155b80156116a757508381105b1561170157818382815181106116bf576116bf612e68565b6001600160a01b039283166020918202929092018101919091529281165f908152600190935260409092205490911690806116f981612c76565b91505061167a565b6001600160a01b038216600114611739578261171e600183612e55565b8151811061172e5761172e612e68565b602002602001015191505b808352509250929050565b335f908152600260205260409020546001600160a01b03166117905760405162461bcd60e51b8152602060048201526005602482015264047533033360dc1b60448201526064016106af565b335f818152600860209081526040808320858452909152808220600190555183917ff2a0eb156472d1440255b0d7c1e19cc07115d1051fe605b0dce69acfec884d9c91a350565b5f6117eb8c8c8c8c8c8c8c8c8c8c8c611c0d565b8051906020012090509b9a5050505050505050505050565b61180b611ee9565b6001600160a01b0381161580159061182d57506001600160a01b038116600114155b6118615760405162461bcd60e51b8152602060048201526005602482015264475331303160d81b60448201526064016106af565b6001600160a01b038281165f908152600160205260409020548116908216146118b45760405162461bcd60e51b8152602060048201526005602482015264475331303360d81b60448201526064016106af565b6001600160a01b038181165f81815260016020526040808220805487861684528284208054919096166001600160a01b0319918216179095558383528054909416909355915190917faab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace405427691a25050565b61192b611ee9565b6001600160a01b038116156119db576040516301ffc9a760e01b815263736bd41d60e11b60048201526001600160a01b038216906301ffc9a790602401602060405180830381865afa158015611983573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a79190612ee7565b6119db5760405162461bcd60e51b8152602060048201526005602482015264047533330360dc1b60448201526064016106af565b7f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c88181556040516001600160a01b038316907f1151116914515bc0891ff9047a6cb32cf902546f83066499bcf8ba33d2353fa2905f90a25050565b611a3e611ee9565b6001600160a01b03811615801590611a6057506001600160a01b038116600114155b8015611a7557506001600160a01b0381163014155b611a915760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038181165f908152600260205260409020541615611ac85760405162461bcd60e51b81526004016106af90612c43565b6001600160a01b03821615801590611aea57506001600160a01b038216600114155b611b065760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038381165f90815260026020526040902054811690831614611b595760405162461bcd60e51b8152602060048201526005602482015264475332303560d81b60448201526064016106af565b6001600160a01b038281165f81815260026020526040808220805486861680855283852080549288166001600160a01b03199384161790559589168452828420805482169096179095558383528054909416909355915190917ff8d49fc529812e9a7c5c50e69c20f0dccc0db8fa95c98bc58cc9a4f1c1299eaf91a26040516001600160a01b038216907f9465fa0c962cc76958e6373a993326400c1c94f8be2fe3a952adfa7f60b2ea26905f90a2505050565b60605f7fbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d85f1b8d8d8d8d604051611c45929190612f06565b604051908190038120611c6b949392918e908e908e908e908e908e908e90602001612f15565b60408051601f1981840301815291905280516020909101209050601960f81b600160f81b611c97611d2b565b6040516001600160f81b031993841660208201529290911660218301526022820152604281018290526062016040516020818303038152906040529150509b9a5050505050505050505050565b611cec611ee9565b611cf5816122bf565b6040516001600160a01b038216907f5ac6c46c93c8d0e53714ba3b53db3e7c046da994313d7ed0d192028bc7c228b0905f90a250565b5f7f47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a794692184660408051602081019390935282015230606082015260800160405160208183030381529060405280519060200120905090565b611d89611ee9565b806001600354611d999190612e55565b1015611db75760405162461bcd60e51b81526004016106af90612d09565b6001600160a01b03821615801590611dd957506001600160a01b038216600114155b611df55760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038381165f90815260026020526040902054811690831614611e485760405162461bcd60e51b8152602060048201526005602482015264475332303560d81b60448201526064016106af565b6001600160a01b038281165f81815260026020526040808220805488861684529183208054929095166001600160a01b03199283161790945591815282549091169091556003805491611e9a83612f87565b90915550506040516001600160a01b038316907ff8d49fc529812e9a7c5c50e69c20f0dccc0db8fa95c98bc58cc9a4f1c1299eaf905f90a28060045414611ee457611ee481610f61565b505050565b333014611f205760405162461bcd60e51b8152602060048201526005602482015264475330333160d81b60448201526064016106af565b565b5f825f03611f3157505f610c6d565b5f611f3c8385612cf2565b905082611f498583612e36565b14611f52575f5ffd5b9392505050565b5f80611f658385612e23565b905083811015611f52575f5ffd5b5f6001836001811115611f8857611f88612d28565b03611f9f575f5f8551602087018986f49050611fae565b5f5f855160208701888a87f190505b95945050505050565b5f81831015611fc65781611f52565b5090919050565b5f82821115611fda575f5ffd5b5f610d758385612e55565b5f806001600160a01b03831615611ffc5782611ffe565b325b90506001600160a01b038416612090576120303a861061201e573a612020565b855b61202a8989611f59565b90611f22565b6040519092506001600160a01b0382169083156108fc029084905f818181858888f1935050505061208b5760405162461bcd60e51b8152602060048201526005602482015264475330313160d81b60448201526064016106af565b6120df565b61209e8561202a8989611f59565b91506120ab848284612451565b6120df5760405162461bcd60e51b815260206004820152600560248201526423a998189960d91b60448201526064016106af565b5095945050505050565b600454156121215760405162461bcd60e51b8152602060048201526005602482015264047533230360dc1b60448201526064016106af565b81518111156121425760405162461bcd60e51b81526004016106af90612d09565b600181101561217b5760405162461bcd60e51b815260206004820152600560248201526423a999181960d91b60448201526064016106af565b60015f5b835181101561228d575f84828151811061219b5761219b612e68565b602002602001015190505f6001600160a01b0316816001600160a01b0316141580156121d157506001600160a01b038116600114155b80156121e657506001600160a01b0381163014155b80156122045750806001600160a01b0316836001600160a01b031614155b6122205760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038181165f9081526002602052604090205416156122575760405162461bcd60e51b81526004016106af90612c43565b6001600160a01b039283165f90815260026020526040902080546001600160a01b0319169382169390931790925560010161217f565b506001600160a01b03165f90815260026020526040902080546001600160a01b03191660011790559051600355600455565b306001600160a01b038216036122ff5760405162461bcd60e51b8152602060048201526005602482015264047533430360dc1b60448201526064016106af565b7f6c9a6c4a39284e37ed1cf53d337577d14212a4870fb976a4366c693b939918d555565b60015f8190526020527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f546001600160a01b03161561238c5760405162461bcd60e51b8152602060048201526005602482015264047533130360dc1b60448201526064016106af565b60015f81905260208190527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f80546001600160a01b03191690911790556001600160a01b038216156107a557813b61240e5760405162461bcd60e51b815260206004820152600560248201526423a998181960d91b60448201526064016106af565b61241d825f8360015f19611f73565b6107a55760405162461bcd60e51b8152602060048201526005602482015264047533030360dc1b60448201526064016106af565b604080516001600160a01b03841660248201526044808201849052825180830390910181526064909101909152602080820180516001600160e01b031663a9059cbb60e01b17815282515f93929184919082896127105a03f13d80156124c157602081146124c9575f93506124d3565b8193506124d3565b5f51158215171593505b5050509392505050565b6001600160a01b03811681146124f1575f5ffd5b50565b80356124ff816124dd565b919050565b5f5f60408385031215612515575f5ffd5b8235612520816124dd565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112612551575f5ffd5b81356001600160401b0381111561256a5761256a61252e565b604051601f8201601f19908116603f011681016001600160401b03811182821017156125985761259861252e565b6040528181528382016020018510156125af575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f5f5f5f608085870312156125de575f5ffd5b8435935060208501356001600160401b038111156125fa575f5ffd5b61260687828801612542565b93505060408501356001600160401b03811115612621575f5ffd5b61262d87828801612542565b949793965093946060013593505050565b5f6020828403121561264e575f5ffd5b8135611f52816124dd565b8035600281106124ff575f5ffd5b5f5f5f5f6080858703121561267a575f5ffd5b8435612685816124dd565b93506020850135925060408501356001600160401b038111156126a6575f5ffd5b6126b287828801612542565b9250506126c160608601612659565b905092959194509250565b5f81518084525f5b818110156126f0576020818501810151868301820152016126d4565b505f602082860101526020601f19601f83011685010191505092915050565b8215158152604060208201525f610d7560408301846126cc565b5f5f6040838503121561273a575f5ffd5b50508035926020909101359150565b602081525f611f5260208301846126cc565b5f6020828403121561276b575f5ffd5b5035919050565b5f5f83601f840112612782575f5ffd5b5081356001600160401b03811115612798575f5ffd5b6020830191508360208285010111156127af575f5ffd5b9250929050565b5f5f5f5f5f5f5f5f5f5f5f6101408c8e0312156127d1575f5ffd5b6127da8c6124f4565b9a5060208c0135995060408c01356001600160401b038111156127fb575f5ffd5b6128078e828f01612772565b909a50985061281a905060608d01612659565b965060808c0135955060a08c0135945060c08c0135935061283d60e08d016124f4565b925061284c6101008d016124f4565b91506101208c01356001600160401b03811115612867575f5ffd5b6128738e828f01612542565b9150509295989b509295989b9093969950565b5f5f5f60608486031215612898575f5ffd5b8335925060208401356001600160401b038111156128b4575f5ffd5b6128c086828701612542565b92505060408401356001600160401b038111156128db575f5ffd5b6128e786828701612542565b9150509250925092565b5f8151808452602084019350602083015f5b8281101561292a5781516001600160a01b0316865260209586019590910190600101612903565b5093949350505050565b602081525f611f5260208301846128f1565b5f5f60408385031215612957575f5ffd5b8235612962816124dd565b915060208301356001600160401b0381111561297c575f5ffd5b61298885828601612542565b9150509250929050565b5f5f5f5f5f5f5f5f5f5f6101008b8d0312156129ac575f5ffd5b8a356001600160401b038111156129c1575f5ffd5b8b01601f81018d136129d1575f5ffd5b80356001600160401b038111156129e6575f5ffd5b8d60208260051b84010111156129fa575f5ffd5b60209182019b5099508b01359750612a1460408c016124f4565b965060608b01356001600160401b03811115612a2e575f5ffd5b612a3a8d828e01612772565b9097509550612a4d905060808c016124f4565b9350612a5b60a08c016124f4565b925060c08b01359150612a7060e08c016124f4565b90509295989b9194979a5092959850565b604081525f612a9360408301856128f1565b905060018060a01b03831660208301529392505050565b5f5f5f5f5f5f5f5f5f5f5f6101408c8e031215612ac5575f5ffd5b8b35612ad0816124dd565b9a5060208c0135995060408c01356001600160401b03811115612af1575f5ffd5b612afd8e828f01612772565b909a509850612b10905060608d01612659565b965060808c0135955060a08c0135945060c08c0135935060e08c0135612b35816124dd565b92506101008c0135612b46816124dd565b809250505f6101208d01359050809150509295989b509295989b9093969950565b5f5f60408385031215612b78575f5ffd5b8235612b83816124dd565b91506020830135612b93816124dd565b809150509250929050565b5f5f5f60608486031215612bb0575f5ffd5b8335612bbb816124dd565b92506020840135612bcb816124dd565b91506040840135612bdb816124dd565b809150509250925092565b5f5f5f60608486031215612bf8575f5ffd5b8335612c03816124dd565b92506020840135612c13816124dd565b929592945050506040919091013590565b602080825260059082015264475332303360d81b604082015260600190565b60208082526005908201526411d4cc8c0d60da1b604082015260600190565b634e487b7160e01b5f52601160045260245ffd5b5f60018201612c8757612c87612c62565b5060010190565b604081525f612ca060408301856126cc565b8281036020840152611fae81856126cc565b5f60208284031215612cc2575f5ffd5b81516001600160e01b031981168114611f52575f5ffd5b60ff8281168282160390811115610c6d57610c6d612c62565b8082028115828204841417610c6d57610c6d612c62565b602080825260059082015264475332303160d81b604082015260600190565b634e487b7160e01b5f52602160045260245ffd5b60028110612d5857634e487b7160e01b5f52602160045260245ffd5b9052565b6001600160a01b038d168152602081018c90526101606040820181905281018a9052898b6101808301375f6101808b830101525f601f19601f8c01168201612da7606084018c612d3c565b8960808401528860a08401528760c0840152612dce60e08401886001600160a01b03169052565b6001600160a01b03861661010084015261018083820301610120840152612df96101808201866126cc565b915050612e126101408301846001600160a01b03169052565b9d9c50505050505050505050505050565b80820180821115610c6d57610c6d612c62565b5f82612e5057634e487b7160e01b5f52601260045260245ffd5b500490565b81810381811115610c6d57610c6d612c62565b634e487b7160e01b5f52603260045260245ffd5b608080825281018590525f8660a08301825b88811015612ebe578235612ea1816124dd565b6001600160a01b0316825260209283019290910190600101612e8e565b50602084019690965250506001600160a01b039283166040820152911660609091015292915050565b5f60208284031215612ef7575f5ffd5b81518015158114611f52575f5ffd5b818382375f9101908152919050565b8b81526001600160a01b038b166020820152604081018a9052606081018990526101608101612f47608083018a612d3c565b60a082019790975260c081019590955260e08501939093526001600160a01b03918216610100850152166101208301526101409091015295945050505050565b5f81612f9557612f95612c62565b505f19019056fea2646970667358221220ce6a6c459c521969c7de69701ca90937a8837cb28f38c0ba179005a2741cf32264736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x01`\x04Ua/\xD2\x80a\0!_9_\xF3\xFE`\x80`@R`\x046\x10a\x01\xD0W_5`\xE0\x1C\x80c\xAF\xFE\xD0\xE0\x11a\0\xF6W\x80c\xE1\x9A\x9D\xD9\x11a\0\x94W\x80c\xF0\x8A\x03#\x11a\0cW\x80c\xF0\x8A\x03#\x14a\x05\xD2W\x80c\xF6\x98\xDA%\x14a\x05\xF1W\x80c\xF8\xDC]\xD9\x14a\x06\x05W\x80c\xFF\xA1\xADt\x14a\x06$Wa\x02\x0CV[\x80c\xE1\x9A\x9D\xD9\x14a\x05aW\x80c\xE3\x18\xB5+\x14a\x05\x80W\x80c\xE7R5\xB8\x14a\x05\x9FW\x80c\xE8f7\xDB\x14a\x05\xB3Wa\x02\x0CV[\x80c\xCC/\x84R\x11a\0\xD0W\x80c\xCC/\x84R\x14a\x04\xD7W\x80c\xD4\xD9\xBD\xCD\x14a\x05\x04W\x80c\xD8\xD1\x1Fx\x14a\x05#W\x80c\xE0\t\xCF\xDE\x14a\x05BWa\x02\x0CV[\x80c\xAF\xFE\xD0\xE0\x14a\x04\x84W\x80c\xB4\xFA\xBA\t\x14a\x04\x99W\x80c\xB6>\x80\r\x14a\x04\xB8Wa\x02\x0CV[\x80cV$\xB2[\x11a\x01nW\x80cjv\x12\x02\x11a\x01=W\x80cjv\x12\x02\x14a\x03\xFBW\x80c}\x83)t\x14a\x04\x0EW\x80c\x93O:\x11\x14a\x04DW\x80c\xA0\xE6~+\x14a\x04cWa\x02\x0CV[\x80cV$\xB2[\x14a\x03fW\x80cZ\xE6\xBD7\x14a\x03\x92W\x80ca\x0BY%\x14a\x03\xBDW\x80ciN\x80\xC3\x14a\x03\xDCWa\x02\x0CV[\x80c/T\xBFn\x11a\x01\xAAW\x80c/T\xBFn\x14a\x02\xDFW\x80c4\x08\xE4p\x14a\x02\xFEW\x80cF\x87!\xA7\x14a\x03\x1AW\x80cR)\x07?\x14a\x039Wa\x02\x0CV[\x80c\rX/\x13\x14a\x02kW\x80c\x12\xFBh\xE0\x14a\x02\x8CW\x80c-\x9A\xD5=\x14a\x02\xABWa\x02\x0CV[6a\x02\x0CW`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[4\x80\x15a\x02\x17W__\xFD[P\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\x02BW\0[6__73``\x1B6R__`\x146\x01__\x85Z\xF1\x90P=__>\x80a\x02fW=_\xFD[P=_\xF3[4\x80\x15a\x02vW__\xFD[Pa\x02\x8Aa\x02\x856`\x04a%\x04V[a\x06TV[\0[4\x80\x15a\x02\x97W__\xFD[Pa\x02\x8Aa\x02\xA66`\x04a%\xCBV[a\x07\xA9V[4\x80\x15a\x02\xB6W__\xFD[Pa\x02\xCAa\x02\xC56`\x04a&>V[a\x0C:V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEAW__\xFD[Pa\x02\xCAa\x02\xF96`\x04a&>V[a\x0CsV[4\x80\x15a\x03\tW__\xFD[PF[`@Q\x90\x81R` \x01a\x02\xD6V[4\x80\x15a\x03%W__\xFD[Pa\x02\xCAa\x0346`\x04a&gV[a\x0C\xA9V[4\x80\x15a\x03DW__\xFD[Pa\x03Xa\x03S6`\x04a&gV[a\r}V[`@Qa\x02\xD6\x92\x91\x90a'\x0FV[4\x80\x15a\x03qW__\xFD[Pa\x03\x85a\x03\x806`\x04a')V[a\r\xB1V[`@Qa\x02\xD6\x91\x90a'IV[4\x80\x15a\x03\x9DW__\xFD[Pa\x03\x0Ca\x03\xAC6`\x04a'[V[`\x07` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xC8W__\xFD[Pa\x02\x8Aa\x03\xD76`\x04a&>V[a\x0E*V[4\x80\x15a\x03\xE7W__\xFD[Pa\x02\x8Aa\x03\xF66`\x04a'[V[a\x0FaV[a\x02\xCAa\x04\t6`\x04a'\xB6V[a\x0F\xFFV[4\x80\x15a\x04\x19W__\xFD[Pa\x03\x0Ca\x04(6`\x04a%\x04V[`\x08` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04OW__\xFD[Pa\x02\x8Aa\x04^6`\x04a(\x86V[a\x138V[4\x80\x15a\x04nW__\xFD[Pa\x04wa\x13\x82V[`@Qa\x02\xD6\x91\x90a)4V[4\x80\x15a\x04\x8FW__\xFD[Pa\x03\x0C`\x05T\x81V[4\x80\x15a\x04\xA4W__\xFD[Pa\x02\x8Aa\x04\xB36`\x04a)FV[a\x14oV[4\x80\x15a\x04\xC3W__\xFD[Pa\x02\x8Aa\x04\xD26`\x04a)\x92V[a\x14\x8EV[4\x80\x15a\x04\xE2W__\xFD[Pa\x04\xF6a\x04\xF16`\x04a%\x04V[a\x15\x8DV[`@Qa\x02\xD6\x92\x91\x90a*\x81V[4\x80\x15a\x05\x0FW__\xFD[Pa\x02\x8Aa\x05\x1E6`\x04a'[V[a\x17DV[4\x80\x15a\x05.W__\xFD[Pa\x03\x0Ca\x05=6`\x04a*\xAAV[a\x17\xD7V[4\x80\x15a\x05MW__\xFD[Pa\x02\x8Aa\x05\\6`\x04a+gV[a\x18\x03V[4\x80\x15a\x05lW__\xFD[Pa\x02\x8Aa\x05{6`\x04a&>V[a\x19#V[4\x80\x15a\x05\x8BW__\xFD[Pa\x02\x8Aa\x05\x9A6`\x04a+\x9EV[a\x1A6V[4\x80\x15a\x05\xAAW__\xFD[P`\x04Ta\x03\x0CV[4\x80\x15a\x05\xBEW__\xFD[Pa\x03\x85a\x05\xCD6`\x04a*\xAAV[a\x1C\rV[4\x80\x15a\x05\xDDW__\xFD[Pa\x02\x8Aa\x05\xEC6`\x04a&>V[a\x1C\xE4V[4\x80\x15a\x05\xFCW__\xFD[Pa\x03\x0Ca\x1D+V[4\x80\x15a\x06\x10W__\xFD[Pa\x02\x8Aa\x06\x1F6`\x04a+\xE6V[a\x1D\x81V[4\x80\x15a\x06/W__\xFD[Pa\x03\x85`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d1.4.1`\xD8\x1B\x81RP\x81V[a\x06\\a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06~WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x06\x93WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x06\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\x06\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x81\x81R`@\x81 \x80T\x93\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90\x93U`\x01\x83R\x83T\x90\x91\x16\x17\x90\x91U`\x03\x80T\x91a\x07[\x83a,vV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90_\x90\xA2\x80`\x04T\x14a\x07\xA5Wa\x07\xA5\x81a\x0FaV[PPV[a\x07\xB4\x81`Aa\x1F\"V[\x82Q\x10\x15a\x07\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x80\x80\x80\x80\x80[\x86\x81\x10\x15a\x0C.W`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P_\x84\x90\x03a\t\xFEW\x88Q` \x8A\x01 \x8A\x14a\x08`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS027`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x91\x93P\x83\x91a\x08p\x87`Aa\x1F\"V[\x82\x10\x15a\x08\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x87Qa\x08\xB4\x83` a\x1FYV[\x11\x15a\x08\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[` \x82\x89\x01\x81\x01Q\x89Q\x90\x91a\t\r\x90\x83\x90a\t\x07\x90\x87\x90a\x1FYV[\x90a\x1FYV[\x11\x15a\tCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`@Qc \xC1;\x0B`\xE0\x1B\x80\x82R\x8A\x85\x01` \x01\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c \xC1;\x0B\x90a\ty\x90\x8F\x90\x86\x90`\x04\x01a,\x8EV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB8\x91\x90a,\xB2V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\t\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x01a\x06\xAFV[PPa\x0B\x9EV[\x83`\xFF\x16`\x01\x03a\n\x7FW\x91\x93P\x83\x913`\x01`\x01`\xA0\x1B\x03\x84\x16\x14\x80a\nFWP`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x8D\x84R\x90\x91R\x90 T\x15\x15[a\nzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\x0B\x9EV[`\x1E\x84`\xFF\x16\x11\x15a\x0BAW`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8B\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\n\xE4\x91\x90a,\xD9V[`@\x80Q_\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B0W=__>=_\xFD[PPP` `@Q\x03Q\x94Pa\x0B\x9EV[`@\x80Q_\x81R` \x81\x01\x80\x83R\x8C\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\x91W=__>=_\xFD[PPP` `@Q\x03Q\x94P[\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\x0B\xD7WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15\x15[\x80\x15a\x0B\xEDWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\x01\x14\x15[a\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x93\x94P\x84\x93`\x01\x01a\x07\xF3V[PPPPPPPPPPV[_`\x01`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80\x15\x90a\x0CmWP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x01` R`@\x90 T\x16\x15\x15[\x92\x91PPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x0CmWPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[_3`\x01\x14\x80\x15\x90a\x0C\xD1WP3_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[a\r\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\r\x13\x85\x85\x85\x85_\x19a\x1FsV[\x90P\x80\x15a\rJW`@Q3\x90\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8\x90_\x90\xA2a\ruV[`@Q3\x90\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u\x90_\x90\xA2[\x94\x93PPPPV[_``a\r\x8C\x86\x86\x86\x86a\x0C\xA9V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=_` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[``_a\r\xBF\x83` a,\xF2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xD6Wa\r\xD6a%.V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0E\"W\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R`\x01\x01a\x0E\x05V[P\x93\x92PPPV[a\x0E2a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0ETWP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x0E\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x01` R`@\x90 T\x16\x15a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x81\x81R`@\x80\x82 \x80T\x94\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90\x94U\x94\x85R\x83T\x90\x91\x16\x81\x17\x90\x92UQ\x90\x91\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x91\xA2PV[a\x0Fia\x1E\xE9V[`\x03T\x81\x11\x15a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01\x81\x10\x15a\x0F\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x04\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[___a\x10\x17\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x05Ta\x1C\rV[`\x05\x80T\x91\x92P_a\x10(\x83a,vV[\x90\x91UPP\x80Q` \x82\x01 \x91Pa\x10A\x82\x82\x86a\x138V[P_a\x10k\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\xECW\x80`\x01`\x01`\xA0\x1B\x03\x16cu\xF0\xBBR\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F3`@Q\x8Dc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xBE\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x10\xE7W=__>=_\xFD[PPPP[a\x11\x18a\x10\xFB\x8Aa\t\xC4a.#V[`?a\x11\x08\x8C`@a,\xF2V[a\x11\x12\x91\x90a.6V[\x90a\x1F\xB7V[a\x11$\x90a\x01\xF4a.#V[Z\x10\x15a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[_Z\x90Pa\x11\xC9\x8F\x8F\x8F\x8F\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8E\x8C_\x14a\x11\xB6W\x8Ea\x1FsV[a\t\xC4Za\x11\xC4\x91\x90a.UV[a\x1FsV[\x93Pa\x11\xD6Z\x82\x90a\x1F\xCDV[\x90P\x83\x80a\x11\xE3WP\x89\x15\x15[\x80a\x11\xEDWP\x87\x15\x15[a\x12!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x88\x15a\x128Wa\x125\x82\x8B\x8B\x8B\x8Ba\x1F\xE5V[\x90P[\x84\x15a\x12}W\x83\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x82`@Qa\x12p\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x12\xB8V[\x83\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x82`@Qa\x12\xAF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13'W`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x83\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x93'\x13h\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13\x10W__\xFD[PZ\xF1\x15\x80\x15a\x13\"W=__>=_\xFD[PPPP[PP\x9B\x9APPPPPPPPPPPV[`\x04T\x80a\x13pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\x13|\x84\x84\x84\x84a\x07\xA9V[PPPPV[``_`\x03T`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x9FWa\x13\x9Fa%.V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01_\x90\x81R`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x14gW\x80\x83\x83\x81Q\x81\x10a\x14(Wa\x14(a.hV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16_\x90\x81R`\x02\x90\x92R`@\x90\x91 T\x16\x81a\x14_\x81a,vV[\x92PPa\x14\x04V[P\x90\x92\x91PPV[__\x82Q` \x84\x01\x85Z\xF4\x80_RP=` R=_`@>`@=\x01_\xFD[a\x14\xCB\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8C\x92Pa \xE9\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x14\xE3Wa\x14\xE3\x84a\"\xBFV[a\x15\"\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa##\x92PPPV[\x81\x15a\x158Wa\x156\x82_`\x01\x86\x85a\x1F\xE5V[P[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x8B\x8B\x8B\x8B\x89`@Qa\x15y\x95\x94\x93\x92\x91\x90a.|V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[``_`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01\x14\x80a\x15\xACWPa\x15\xAC\x84a\x0C:V[a\x15\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS105`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x83\x11a\x16\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16/Wa\x16/a%.V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16XW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x01` R`@\x81 T\x92\x94P\x91\x16\x91P[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x16\x9CWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x16\xA7WP\x83\x81\x10[\x15a\x17\x01W\x81\x83\x82\x81Q\x81\x10a\x16\xBFWa\x16\xBFa.hV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x92\x81\x16_\x90\x81R`\x01\x90\x93R`@\x90\x92 T\x90\x91\x16\x90\x80a\x16\xF9\x81a,vV[\x91PPa\x16zV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14a\x179W\x82a\x17\x1E`\x01\x83a.UV[\x81Q\x81\x10a\x17.Wa\x17.a.hV[` \x02` \x01\x01Q\x91P[\x80\x83RP\x92P\x92\x90PV[3_\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x17\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[3_\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 `\x01\x90UQ\x83\x91\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x91\xA3PV[_a\x17\xEB\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x1C\rV[\x80Q\x90` \x01 \x90P\x9B\x9APPPPPPPPPPPV[a\x18\x0Ba\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x18-WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x18aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x01` R`@\x90 T\x81\x16\x90\x82\x16\x14a\x18\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x01` R`@\x80\x82 \x80T\x87\x86\x16\x84R\x82\x84 \x80T\x91\x90\x96\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x95U\x83\x83R\x80T\x90\x94\x16\x90\x93U\x91Q\x90\x91\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x91\xA2PPV[a\x19+a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x19\xDBW`@Qc\x01\xFF\xC9\xA7`\xE0\x1B\x81Rcsk\xD4\x1D`\xE1\x1B`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x01\xFF\xC9\xA7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA7\x91\x90a.\xE7V[a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u33\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8\x81\x81U`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2\x90_\x90\xA2PPV[a\x1A>a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1A`WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x1AuWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x1A\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\x1A\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1A\xEAWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1B\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1BYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x02` R`@\x80\x82 \x80T\x86\x86\x16\x80\x85R\x83\x85 \x80T\x92\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x95\x89\x16\x84R\x82\x84 \x80T\x82\x16\x90\x96\x17\x90\x95U\x83\x83R\x80T\x90\x94\x16\x90\x93U\x91Q\x90\x91\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\xA2`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90_\x90\xA2PPPV[``_\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8_\x1B\x8D\x8D\x8D\x8D`@Qa\x1CE\x92\x91\x90a/\x06V[`@Q\x90\x81\x90\x03\x81 a\x1Ck\x94\x93\x92\x91\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90` \x01a/\x15V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x19`\xF8\x1B`\x01`\xF8\x1Ba\x1C\x97a\x1D+V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16` \x82\x01R\x92\x90\x91\x16`!\x83\x01R`\"\x82\x01R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x9B\x9APPPPPPPPPPPV[a\x1C\xECa\x1E\xE9V[a\x1C\xF5\x81a\"\xBFV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90_\x90\xA2PV[_\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18F`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R0``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x1D\x89a\x1E\xE9V[\x80`\x01`\x03Ta\x1D\x99\x91\x90a.UV[\x10\x15a\x1D\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1D\xD9WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1D\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x02` R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x03\x80T\x91a\x1E\x9A\x83a/\x87V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90_\x90\xA2\x80`\x04T\x14a\x1E\xE4Wa\x1E\xE4\x81a\x0FaV[PPPV[30\x14a\x1F W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[V[_\x82_\x03a\x1F1WP_a\x0CmV[_a\x1F<\x83\x85a,\xF2V[\x90P\x82a\x1FI\x85\x83a.6V[\x14a\x1FRW__\xFD[\x93\x92PPPV[_\x80a\x1Fe\x83\x85a.#V[\x90P\x83\x81\x10\x15a\x1FRW__\xFD[_`\x01\x83`\x01\x81\x11\x15a\x1F\x88Wa\x1F\x88a-(V[\x03a\x1F\x9FW__\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x1F\xAEV[__\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[_\x81\x83\x10\x15a\x1F\xC6W\x81a\x1FRV[P\x90\x91\x90PV[_\x82\x82\x11\x15a\x1F\xDAW__\xFD[_a\ru\x83\x85a.UV[_\x80`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1F\xFCW\x82a\x1F\xFEV[2[\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a \x90Wa 0:\x86\x10a \x1EW:a  V[\x85[a *\x89\x89a\x1FYV[\x90a\x1F\"V[`@Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa \x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a \xDFV[a \x9E\x85a *\x89\x89a\x1FYV[\x91Pa \xAB\x84\x82\x84a$QV[a \xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[P\x95\x94PPPPPV[`\x04T\x15a!!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x81Q\x81\x11\x15a!BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01\x81\x10\x15a!{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01_[\x83Q\x81\x10\x15a\"\x8DW_\x84\x82\x81Q\x81\x10a!\x9BWa!\x9Ba.hV[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!\xD1WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a!\xE6WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[\x80\x15a\"\x04WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\" W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\"WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\x01\x01a!\x7FV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01\x17\x90U\x90Q`\x03U`\x04UV[0`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3C\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`\x01_\x81\x90R` R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/T`\x01`\x01`\xA0\x1B\x03\x16\x15a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01_\x81\x90R` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x07\xA5W\x81;a$\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[a$\x1D\x82_\x83`\x01_\x19a\x1FsV[a\x07\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Q_\x93\x92\x91\x84\x91\x90\x82\x89a'\x10Z\x03\xF1=\x80\x15a$\xC1W` \x81\x14a$\xC9W_\x93Pa$\xD3V[\x81\x93Pa$\xD3V[_Q\x15\x82\x15\x17\x15\x93P[PPP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xF1W__\xFD[PV[\x805a$\xFF\x81a$\xDDV[\x91\x90PV[__`@\x83\x85\x03\x12\x15a%\x15W__\xFD[\x825a% \x81a$\xDDV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a%QW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a%jWa%ja%.V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\x98Wa%\x98a%.V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a%\xAFW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a%\xDEW__\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xFAW__\xFD[a&\x06\x87\x82\x88\x01a%BV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&!W__\xFD[a&-\x87\x82\x88\x01a%BV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[_` \x82\x84\x03\x12\x15a&NW__\xFD[\x815a\x1FR\x81a$\xDDV[\x805`\x02\x81\x10a$\xFFW__\xFD[____`\x80\x85\x87\x03\x12\x15a&zW__\xFD[\x845a&\x85\x81a$\xDDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA6W__\xFD[a&\xB2\x87\x82\x88\x01a%BV[\x92PPa&\xC1``\x86\x01a&YV[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a&\xF0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&\xD4V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R_a\ru`@\x83\x01\x84a&\xCCV[__`@\x83\x85\x03\x12\x15a':W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x81R_a\x1FR` \x83\x01\x84a&\xCCV[_` \x82\x84\x03\x12\x15a'kW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a'\x82W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x98W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a'\xAFW__\xFD[\x92P\x92\x90PV[___________a\x01@\x8C\x8E\x03\x12\x15a'\xD1W__\xFD[a'\xDA\x8Ca$\xF4V[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xFBW__\xFD[a(\x07\x8E\x82\x8F\x01a'rV[\x90\x9AP\x98Pa(\x1A\x90P``\x8D\x01a&YV[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93Pa(=`\xE0\x8D\x01a$\xF4V[\x92Pa(La\x01\0\x8D\x01a$\xF4V[\x91Pa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(gW__\xFD[a(s\x8E\x82\x8F\x01a%BV[\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[___``\x84\x86\x03\x12\x15a(\x98W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB4W__\xFD[a(\xC0\x86\x82\x87\x01a%BV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDBW__\xFD[a(\xE7\x86\x82\x87\x01a%BV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a)*W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a)\x03V[P\x93\x94\x93PPPPV[` \x81R_a\x1FR` \x83\x01\x84a(\xF1V[__`@\x83\x85\x03\x12\x15a)WW__\xFD[\x825a)b\x81a$\xDDV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)|W__\xFD[a)\x88\x85\x82\x86\x01a%BV[\x91PP\x92P\x92\x90PV[__________a\x01\0\x8B\x8D\x03\x12\x15a)\xACW__\xFD[\x8A5`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xC1W__\xFD[\x8B\x01`\x1F\x81\x01\x8D\x13a)\xD1W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE6W__\xFD[\x8D` \x82`\x05\x1B\x84\x01\x01\x11\x15a)\xFAW__\xFD[` \x91\x82\x01\x9BP\x99P\x8B\x015\x97Pa*\x14`@\x8C\x01a$\xF4V[\x96P``\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*.W__\xFD[a*:\x8D\x82\x8E\x01a'rV[\x90\x97P\x95Pa*M\x90P`\x80\x8C\x01a$\xF4V[\x93Pa*[`\xA0\x8C\x01a$\xF4V[\x92P`\xC0\x8B\x015\x91Pa*p`\xE0\x8C\x01a$\xF4V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`@\x81R_a*\x93`@\x83\x01\x85a(\xF1V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[___________a\x01@\x8C\x8E\x03\x12\x15a*\xC5W__\xFD[\x8B5a*\xD0\x81a$\xDDV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xF1W__\xFD[a*\xFD\x8E\x82\x8F\x01a'rV[\x90\x9AP\x98Pa+\x10\x90P``\x8D\x01a&YV[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93P`\xE0\x8C\x015a+5\x81a$\xDDV[\x92Pa\x01\0\x8C\x015a+F\x81a$\xDDV[\x80\x92PP_a\x01 \x8D\x015\x90P\x80\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[__`@\x83\x85\x03\x12\x15a+xW__\xFD[\x825a+\x83\x81a$\xDDV[\x91P` \x83\x015a+\x93\x81a$\xDDV[\x80\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a+\xB0W__\xFD[\x835a+\xBB\x81a$\xDDV[\x92P` \x84\x015a+\xCB\x81a$\xDDV[\x91P`@\x84\x015a+\xDB\x81a$\xDDV[\x80\x91PP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a+\xF8W__\xFD[\x835a,\x03\x81a$\xDDV[\x92P` \x84\x015a,\x13\x81a$\xDDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x05\x90\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a,\x87Wa,\x87a,bV[P`\x01\x01\x90V[`@\x81R_a,\xA0`@\x83\x01\x85a&\xCCV[\x82\x81\x03` \x84\x01Ra\x1F\xAE\x81\x85a&\xCCV[_` \x82\x84\x03\x12\x15a,\xC2W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1FRW__\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0CmWa\x0Cma,bV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0CmWa\x0Cma,bV[` \x80\x82R`\x05\x90\x82\x01RdGS201`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a-XWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x81R` \x81\x01\x8C\x90Ra\x01``@\x82\x01\x81\x90R\x81\x01\x8A\x90R\x89\x8Ba\x01\x80\x83\x017_a\x01\x80\x8B\x83\x01\x01R_`\x1F\x19`\x1F\x8C\x01\x16\x82\x01a-\xA7``\x84\x01\x8Ca-<V[\x89`\x80\x84\x01R\x88`\xA0\x84\x01R\x87`\xC0\x84\x01Ra-\xCE`\xE0\x84\x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x01\0\x84\x01Ra\x01\x80\x83\x82\x03\x01a\x01 \x84\x01Ra-\xF9a\x01\x80\x82\x01\x86a&\xCCV[\x91PPa.\x12a\x01@\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x9D\x9CPPPPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0CmWa\x0Cma,bV[_\x82a.PWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0CmWa\x0Cma,bV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x80\x80\x82R\x81\x01\x85\x90R_\x86`\xA0\x83\x01\x82[\x88\x81\x10\x15a.\xBEW\x825a.\xA1\x81a$\xDDV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\x8EV[P` \x84\x01\x96\x90\x96RPP`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`@\x82\x01R\x91\x16``\x90\x91\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\xF7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1FRW__\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16` \x82\x01R`@\x81\x01\x8A\x90R``\x81\x01\x89\x90Ra\x01`\x81\x01a/G`\x80\x83\x01\x8Aa-<V[`\xA0\x82\x01\x97\x90\x97R`\xC0\x81\x01\x95\x90\x95R`\xE0\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16a\x01\0\x85\x01R\x16a\x01 \x83\x01Ra\x01@\x90\x91\x01R\x95\x94PPPPPV[_\x81a/\x95Wa/\x95a,bV[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xCEjlE\x9CR\x19i\xC7\xDEip\x1C\xA9\t7\xA8\x83|\xB2\x8F8\xC0\xBA\x17\x90\x05\xA2t\x1C\xF3\"dsolcC\0\x08\x1C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101d0575f3560e01c8063affed0e0116100f6578063e19a9dd911610094578063f08a032311610063578063f08a0323146105d2578063f698da25146105f1578063f8dc5dd914610605578063ffa1ad74146106245761020c565b8063e19a9dd914610561578063e318b52b14610580578063e75235b81461059f578063e86637db146105b35761020c565b8063cc2f8452116100d0578063cc2f8452146104d7578063d4d9bdcd14610504578063d8d11f7814610523578063e009cfde146105425761020c565b8063affed0e014610484578063b4faba0914610499578063b63e800d146104b85761020c565b80635624b25b1161016e5780636a7612021161013d5780636a761202146103fb5780637d8329741461040e578063934f3a1114610444578063a0e67e2b146104635761020c565b80635624b25b146103665780635ae6bd3714610392578063610b5925146103bd578063694e80c3146103dc5761020c565b80632f54bf6e116101aa5780632f54bf6e146102df5780633408e470146102fe578063468721a71461031a5780635229073f146103395761020c565b80630d582f131461026b57806312fb68e01461028c5780632d9ad53d146102ab5761020c565b3661020c5760405134815233907f3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d9060200160405180910390a2005b348015610217575f5ffd5b507f6c9a6c4a39284e37ed1cf53d337577d14212a4870fb976a4366c693b939918d580548061024257005b365f5f373360601b36525f5f601436015f5f855af190503d5f5f3e80610266573d5ffd5b503d5ff35b348015610276575f5ffd5b5061028a610285366004612504565b610654565b005b348015610297575f5ffd5b5061028a6102a63660046125cb565b6107a9565b3480156102b6575f5ffd5b506102ca6102c536600461263e565b610c3a565b60405190151581526020015b60405180910390f35b3480156102ea575f5ffd5b506102ca6102f936600461263e565b610c73565b348015610309575f5ffd5b50465b6040519081526020016102d6565b348015610325575f5ffd5b506102ca610334366004612667565b610ca9565b348015610344575f5ffd5b50610358610353366004612667565b610d7d565b6040516102d692919061270f565b348015610371575f5ffd5b50610385610380366004612729565b610db1565b6040516102d69190612749565b34801561039d575f5ffd5b5061030c6103ac36600461275b565b60076020525f908152604090205481565b3480156103c8575f5ffd5b5061028a6103d736600461263e565b610e2a565b3480156103e7575f5ffd5b5061028a6103f636600461275b565b610f61565b6102ca6104093660046127b6565b610fff565b348015610419575f5ffd5b5061030c610428366004612504565b600860209081525f928352604080842090915290825290205481565b34801561044f575f5ffd5b5061028a61045e366004612886565b611338565b34801561046e575f5ffd5b50610477611382565b6040516102d69190612934565b34801561048f575f5ffd5b5061030c60055481565b3480156104a4575f5ffd5b5061028a6104b3366004612946565b61146f565b3480156104c3575f5ffd5b5061028a6104d2366004612992565b61148e565b3480156104e2575f5ffd5b506104f66104f1366004612504565b61158d565b6040516102d6929190612a81565b34801561050f575f5ffd5b5061028a61051e36600461275b565b611744565b34801561052e575f5ffd5b5061030c61053d366004612aaa565b6117d7565b34801561054d575f5ffd5b5061028a61055c366004612b67565b611803565b34801561056c575f5ffd5b5061028a61057b36600461263e565b611923565b34801561058b575f5ffd5b5061028a61059a366004612b9e565b611a36565b3480156105aa575f5ffd5b5060045461030c565b3480156105be575f5ffd5b506103856105cd366004612aaa565b611c0d565b3480156105dd575f5ffd5b5061028a6105ec36600461263e565b611ce4565b3480156105fc575f5ffd5b5061030c611d2b565b348015610610575f5ffd5b5061028a61061f366004612be6565b611d81565b34801561062f575f5ffd5b5061038560405180604001604052806005815260200164312e342e3160d81b81525081565b61065c611ee9565b6001600160a01b0382161580159061067e57506001600160a01b038216600114155b801561069357506001600160a01b0382163014155b6106b85760405162461bcd60e51b81526004016106af90612c24565b60405180910390fd5b6001600160a01b038281165f9081526002602052604090205416156106ef5760405162461bcd60e51b81526004016106af90612c43565b60026020527fe90b7bceb6e7df5418fb78d8ee546e97c83a08bbccc01a0644d599ccd2a7c2e080546001600160a01b038481165f818152604081208054939094166001600160a01b03199384161790935560018352835490911617909155600380549161075b83612c76565b90915550506040516001600160a01b038316907f9465fa0c962cc76958e6373a993326400c1c94f8be2fe3a952adfa7f60b2ea26905f90a280600454146107a5576107a581610f61565b5050565b6107b4816041611f22565b825110156107ec5760405162461bcd60e51b8152602060048201526005602482015264047533032360dc1b60448201526064016106af565b5f80808080805b86811015610c2e576041818102890160208101516040820151919092015160ff16955090935091505f8490036109fe57885160208a01208a146108605760405162461bcd60e51b8152602060048201526005602482015264475330323760d81b60448201526064016106af565b9193508391610870876041611f22565b8210156108a75760405162461bcd60e51b8152602060048201526005602482015264475330323160d81b60448201526064016106af565b87516108b4836020611f59565b11156108ea5760405162461bcd60e51b815260206004820152600560248201526423a998191960d91b60448201526064016106af565b60208289018101518951909161090d908390610907908790611f59565b90611f59565b11156109435760405162461bcd60e51b8152602060048201526005602482015264475330323360d81b60448201526064016106af565b6040516320c13b0b60e01b8082528a8501602001916001600160a01b038916906320c13b0b90610979908f908690600401612c8e565b602060405180830381865afa158015610994573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109b89190612cb2565b6001600160e01b031916146109f75760405162461bcd60e51b815260206004820152600560248201526411d4cc0c8d60da1b60448201526064016106af565b5050610b9e565b8360ff16600103610a7f579193508391336001600160a01b0384161480610a4657506001600160a01b0385165f9081526008602090815260408083208d845290915290205415155b610a7a5760405162461bcd60e51b8152602060048201526005602482015264475330323560d81b60448201526064016106af565b610b9e565b601e8460ff161115610b41576040517f19457468657265756d205369676e6564204d6573736167653a0a3332000000006020820152603c81018b9052600190605c0160405160208183030381529060405280519060200120600486610ae49190612cd9565b604080515f8152602081018083529390935260ff90911690820152606081018590526080810184905260a0016020604051602081039080840390855afa158015610b30573d5f5f3e3d5ffd5b505050602060405103519450610b9e565b604080515f8152602081018083528c905260ff861691810191909152606081018490526080810183905260019060a0016020604051602081039080840390855afa158015610b91573d5f5f3e3d5ffd5b5050506020604051035194505b856001600160a01b0316856001600160a01b0316118015610bd757506001600160a01b038581165f908152600260205260409020541615155b8015610bed57506001600160a01b038516600114155b610c215760405162461bcd60e51b815260206004820152600560248201526423a998191b60d91b60448201526064016106af565b93945084936001016107f3565b50505050505050505050565b5f60016001600160a01b03831614801590610c6d57506001600160a01b038281165f908152600160205260409020541615155b92915050565b5f6001600160a01b038216600114801590610c6d5750506001600160a01b039081165f9081526002602052604090205416151590565b5f33600114801590610cd15750335f908152600160205260409020546001600160a01b031615155b610d055760405162461bcd60e51b815260206004820152600560248201526411d4cc4c0d60da1b60448201526064016106af565b610d13858585855f19611f73565b90508015610d4a5760405133907f6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb8905f90a2610d75565b60405133907facd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd375905f90a25b949350505050565b5f6060610d8c86868686610ca9565b915060405160203d0181016040523d81523d5f602083013e8091505094509492505050565b60605f610dbf836020612cf2565b6001600160401b03811115610dd657610dd661252e565b6040519080825280601f01601f191660200182016040528015610e00576020820181803683370190505b5090505f5b83811015610e225784810154602080830284010152600101610e05565b509392505050565b610e32611ee9565b6001600160a01b03811615801590610e5457506001600160a01b038116600114155b610e885760405162461bcd60e51b8152602060048201526005602482015264475331303160d81b60448201526064016106af565b6001600160a01b038181165f908152600160205260409020541615610ed75760405162461bcd60e51b815260206004820152600560248201526423a998981960d91b60448201526064016106af565b600160208190527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f80546001600160a01b038481165f81815260408082208054949095166001600160a01b031994851617909455948552835490911681179092555190917fecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f844091a250565b610f69611ee9565b600354811115610f8b5760405162461bcd60e51b81526004016106af90612d09565b6001811015610fc45760405162461bcd60e51b815260206004820152600560248201526423a999181960d91b60448201526064016106af565b60048190556040518181527f610f7ff2b304ae8903c3de74c60c6ab1f7d6226b3f52c5161905bb5ad4039c939060200160405180910390a150565b5f5f5f6110178e8e8e8e8e8e8e8e8e8e600554611c0d565b600580549192505f61102883612c76565b9091555050805160208201209150611041828286611338565b505f61106b7f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c85490565b90506001600160a01b038116156110ec57806001600160a01b03166375f0bb528f8f8f8f8f8f8f8f8f8f8f336040518d63ffffffff1660e01b81526004016110be9c9b9a99989796959493929190612d5c565b5f604051808303815f87803b1580156110d5575f5ffd5b505af11580156110e7573d5f5f3e3d5ffd5b505050505b6111186110fb8a6109c4612e23565b603f6111088c6040612cf2565b6111129190612e36565b90611fb7565b611124906101f4612e23565b5a101561115b5760405162461bcd60e51b8152602060048201526005602482015264047533031360dc1b60448201526064016106af565b5f5a90506111c98f8f8f8f8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050508e8c5f146111b6578e611f73565b6109c45a6111c49190612e55565b611f73565b93506111d65a8290611fcd565b905083806111e357508915155b806111ed57508715155b6112215760405162461bcd60e51b8152602060048201526005602482015264475330313360d81b60448201526064016106af565b5f881561123857611235828b8b8b8b611fe5565b90505b841561127d57837f442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e8260405161127091815260200190565b60405180910390a26112b8565b837f23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23826040516112af91815260200190565b60405180910390a25b50506001600160a01b0381161561132757604051631264e26d60e31b81526004810183905283151560248201526001600160a01b038216906393271368906044015f604051808303815f87803b158015611310575f5ffd5b505af1158015611322573d5f5f3e3d5ffd5b505050505b50509b9a5050505050505050505050565b600454806113705760405162461bcd60e51b8152602060048201526005602482015264475330303160d81b60448201526064016106af565b61137c848484846107a9565b50505050565b60605f6003546001600160401b0381111561139f5761139f61252e565b6040519080825280602002602001820160405280156113c8578160200160208202803683370190505b5060015f90815260026020527fe90b7bceb6e7df5418fb78d8ee546e97c83a08bbccc01a0644d599ccd2a7c2e054919250906001600160a01b03165b6001600160a01b038116600114611467578083838151811061142857611428612e68565b6001600160a01b039283166020918202929092018101919091529181165f9081526002909252604090912054168161145f81612c76565b925050611404565b509092915050565b5f5f825160208401855af4805f52503d6020523d5f60403e60403d015ffd5b6114cb8a8a808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508c92506120e9915050565b6001600160a01b038416156114e3576114e3846122bf565b6115228787878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061232392505050565b811561153857611536825f60018685611fe5565b505b336001600160a01b03167f141df868a6331af528e38c83b7aa03edc19be66e37ae67f9285bf4f8e3c6a1a88b8b8b8b89604051611579959493929190612e7c565b60405180910390a250505050505050505050565b60605f6001600160a01b038416600114806115ac57506115ac84610c3a565b6115e05760405162461bcd60e51b8152602060048201526005602482015264475331303560d81b60448201526064016106af565b5f83116116175760405162461bcd60e51b815260206004820152600560248201526423a998981b60d91b60448201526064016106af565b826001600160401b0381111561162f5761162f61252e565b604051908082528060200260200182016040528015611658578160200160208202803683370190505b506001600160a01b038086165f90815260016020526040812054929450911691505b6001600160a01b0382161580159061169c57506001600160a01b038216600114155b80156116a757508381105b1561170157818382815181106116bf576116bf612e68565b6001600160a01b039283166020918202929092018101919091529281165f908152600190935260409092205490911690806116f981612c76565b91505061167a565b6001600160a01b038216600114611739578261171e600183612e55565b8151811061172e5761172e612e68565b602002602001015191505b808352509250929050565b335f908152600260205260409020546001600160a01b03166117905760405162461bcd60e51b8152602060048201526005602482015264047533033360dc1b60448201526064016106af565b335f818152600860209081526040808320858452909152808220600190555183917ff2a0eb156472d1440255b0d7c1e19cc07115d1051fe605b0dce69acfec884d9c91a350565b5f6117eb8c8c8c8c8c8c8c8c8c8c8c611c0d565b8051906020012090509b9a5050505050505050505050565b61180b611ee9565b6001600160a01b0381161580159061182d57506001600160a01b038116600114155b6118615760405162461bcd60e51b8152602060048201526005602482015264475331303160d81b60448201526064016106af565b6001600160a01b038281165f908152600160205260409020548116908216146118b45760405162461bcd60e51b8152602060048201526005602482015264475331303360d81b60448201526064016106af565b6001600160a01b038181165f81815260016020526040808220805487861684528284208054919096166001600160a01b0319918216179095558383528054909416909355915190917faab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace405427691a25050565b61192b611ee9565b6001600160a01b038116156119db576040516301ffc9a760e01b815263736bd41d60e11b60048201526001600160a01b038216906301ffc9a790602401602060405180830381865afa158015611983573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119a79190612ee7565b6119db5760405162461bcd60e51b8152602060048201526005602482015264047533330360dc1b60448201526064016106af565b7f4a204f620c8c5ccdca3fd54d003badd85ba500436a431f0cbda4f558c93c34c88181556040516001600160a01b038316907f1151116914515bc0891ff9047a6cb32cf902546f83066499bcf8ba33d2353fa2905f90a25050565b611a3e611ee9565b6001600160a01b03811615801590611a6057506001600160a01b038116600114155b8015611a7557506001600160a01b0381163014155b611a915760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038181165f908152600260205260409020541615611ac85760405162461bcd60e51b81526004016106af90612c43565b6001600160a01b03821615801590611aea57506001600160a01b038216600114155b611b065760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038381165f90815260026020526040902054811690831614611b595760405162461bcd60e51b8152602060048201526005602482015264475332303560d81b60448201526064016106af565b6001600160a01b038281165f81815260026020526040808220805486861680855283852080549288166001600160a01b03199384161790559589168452828420805482169096179095558383528054909416909355915190917ff8d49fc529812e9a7c5c50e69c20f0dccc0db8fa95c98bc58cc9a4f1c1299eaf91a26040516001600160a01b038216907f9465fa0c962cc76958e6373a993326400c1c94f8be2fe3a952adfa7f60b2ea26905f90a2505050565b60605f7fbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d85f1b8d8d8d8d604051611c45929190612f06565b604051908190038120611c6b949392918e908e908e908e908e908e908e90602001612f15565b60408051601f1981840301815291905280516020909101209050601960f81b600160f81b611c97611d2b565b6040516001600160f81b031993841660208201529290911660218301526022820152604281018290526062016040516020818303038152906040529150509b9a5050505050505050505050565b611cec611ee9565b611cf5816122bf565b6040516001600160a01b038216907f5ac6c46c93c8d0e53714ba3b53db3e7c046da994313d7ed0d192028bc7c228b0905f90a250565b5f7f47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a794692184660408051602081019390935282015230606082015260800160405160208183030381529060405280519060200120905090565b611d89611ee9565b806001600354611d999190612e55565b1015611db75760405162461bcd60e51b81526004016106af90612d09565b6001600160a01b03821615801590611dd957506001600160a01b038216600114155b611df55760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038381165f90815260026020526040902054811690831614611e485760405162461bcd60e51b8152602060048201526005602482015264475332303560d81b60448201526064016106af565b6001600160a01b038281165f81815260026020526040808220805488861684529183208054929095166001600160a01b03199283161790945591815282549091169091556003805491611e9a83612f87565b90915550506040516001600160a01b038316907ff8d49fc529812e9a7c5c50e69c20f0dccc0db8fa95c98bc58cc9a4f1c1299eaf905f90a28060045414611ee457611ee481610f61565b505050565b333014611f205760405162461bcd60e51b8152602060048201526005602482015264475330333160d81b60448201526064016106af565b565b5f825f03611f3157505f610c6d565b5f611f3c8385612cf2565b905082611f498583612e36565b14611f52575f5ffd5b9392505050565b5f80611f658385612e23565b905083811015611f52575f5ffd5b5f6001836001811115611f8857611f88612d28565b03611f9f575f5f8551602087018986f49050611fae565b5f5f855160208701888a87f190505b95945050505050565b5f81831015611fc65781611f52565b5090919050565b5f82821115611fda575f5ffd5b5f610d758385612e55565b5f806001600160a01b03831615611ffc5782611ffe565b325b90506001600160a01b038416612090576120303a861061201e573a612020565b855b61202a8989611f59565b90611f22565b6040519092506001600160a01b0382169083156108fc029084905f818181858888f1935050505061208b5760405162461bcd60e51b8152602060048201526005602482015264475330313160d81b60448201526064016106af565b6120df565b61209e8561202a8989611f59565b91506120ab848284612451565b6120df5760405162461bcd60e51b815260206004820152600560248201526423a998189960d91b60448201526064016106af565b5095945050505050565b600454156121215760405162461bcd60e51b8152602060048201526005602482015264047533230360dc1b60448201526064016106af565b81518111156121425760405162461bcd60e51b81526004016106af90612d09565b600181101561217b5760405162461bcd60e51b815260206004820152600560248201526423a999181960d91b60448201526064016106af565b60015f5b835181101561228d575f84828151811061219b5761219b612e68565b602002602001015190505f6001600160a01b0316816001600160a01b0316141580156121d157506001600160a01b038116600114155b80156121e657506001600160a01b0381163014155b80156122045750806001600160a01b0316836001600160a01b031614155b6122205760405162461bcd60e51b81526004016106af90612c24565b6001600160a01b038181165f9081526002602052604090205416156122575760405162461bcd60e51b81526004016106af90612c43565b6001600160a01b039283165f90815260026020526040902080546001600160a01b0319169382169390931790925560010161217f565b506001600160a01b03165f90815260026020526040902080546001600160a01b03191660011790559051600355600455565b306001600160a01b038216036122ff5760405162461bcd60e51b8152602060048201526005602482015264047533430360dc1b60448201526064016106af565b7f6c9a6c4a39284e37ed1cf53d337577d14212a4870fb976a4366c693b939918d555565b60015f8190526020527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f546001600160a01b03161561238c5760405162461bcd60e51b8152602060048201526005602482015264047533130360dc1b60448201526064016106af565b60015f81905260208190527fcc69885fda6bcc1a4ace058b4a62bf5e179ea78fd58a1ccd71c22cc9b688792f80546001600160a01b03191690911790556001600160a01b038216156107a557813b61240e5760405162461bcd60e51b815260206004820152600560248201526423a998181960d91b60448201526064016106af565b61241d825f8360015f19611f73565b6107a55760405162461bcd60e51b8152602060048201526005602482015264047533030360dc1b60448201526064016106af565b604080516001600160a01b03841660248201526044808201849052825180830390910181526064909101909152602080820180516001600160e01b031663a9059cbb60e01b17815282515f93929184919082896127105a03f13d80156124c157602081146124c9575f93506124d3565b8193506124d3565b5f51158215171593505b5050509392505050565b6001600160a01b03811681146124f1575f5ffd5b50565b80356124ff816124dd565b919050565b5f5f60408385031215612515575f5ffd5b8235612520816124dd565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b5f82601f830112612551575f5ffd5b81356001600160401b0381111561256a5761256a61252e565b604051601f8201601f19908116603f011681016001600160401b03811182821017156125985761259861252e565b6040528181528382016020018510156125af575f5ffd5b816020850160208301375f918101602001919091529392505050565b5f5f5f5f608085870312156125de575f5ffd5b8435935060208501356001600160401b038111156125fa575f5ffd5b61260687828801612542565b93505060408501356001600160401b03811115612621575f5ffd5b61262d87828801612542565b949793965093946060013593505050565b5f6020828403121561264e575f5ffd5b8135611f52816124dd565b8035600281106124ff575f5ffd5b5f5f5f5f6080858703121561267a575f5ffd5b8435612685816124dd565b93506020850135925060408501356001600160401b038111156126a6575f5ffd5b6126b287828801612542565b9250506126c160608601612659565b905092959194509250565b5f81518084525f5b818110156126f0576020818501810151868301820152016126d4565b505f602082860101526020601f19601f83011685010191505092915050565b8215158152604060208201525f610d7560408301846126cc565b5f5f6040838503121561273a575f5ffd5b50508035926020909101359150565b602081525f611f5260208301846126cc565b5f6020828403121561276b575f5ffd5b5035919050565b5f5f83601f840112612782575f5ffd5b5081356001600160401b03811115612798575f5ffd5b6020830191508360208285010111156127af575f5ffd5b9250929050565b5f5f5f5f5f5f5f5f5f5f5f6101408c8e0312156127d1575f5ffd5b6127da8c6124f4565b9a5060208c0135995060408c01356001600160401b038111156127fb575f5ffd5b6128078e828f01612772565b909a50985061281a905060608d01612659565b965060808c0135955060a08c0135945060c08c0135935061283d60e08d016124f4565b925061284c6101008d016124f4565b91506101208c01356001600160401b03811115612867575f5ffd5b6128738e828f01612542565b9150509295989b509295989b9093969950565b5f5f5f60608486031215612898575f5ffd5b8335925060208401356001600160401b038111156128b4575f5ffd5b6128c086828701612542565b92505060408401356001600160401b038111156128db575f5ffd5b6128e786828701612542565b9150509250925092565b5f8151808452602084019350602083015f5b8281101561292a5781516001600160a01b0316865260209586019590910190600101612903565b5093949350505050565b602081525f611f5260208301846128f1565b5f5f60408385031215612957575f5ffd5b8235612962816124dd565b915060208301356001600160401b0381111561297c575f5ffd5b61298885828601612542565b9150509250929050565b5f5f5f5f5f5f5f5f5f5f6101008b8d0312156129ac575f5ffd5b8a356001600160401b038111156129c1575f5ffd5b8b01601f81018d136129d1575f5ffd5b80356001600160401b038111156129e6575f5ffd5b8d60208260051b84010111156129fa575f5ffd5b60209182019b5099508b01359750612a1460408c016124f4565b965060608b01356001600160401b03811115612a2e575f5ffd5b612a3a8d828e01612772565b9097509550612a4d905060808c016124f4565b9350612a5b60a08c016124f4565b925060c08b01359150612a7060e08c016124f4565b90509295989b9194979a5092959850565b604081525f612a9360408301856128f1565b905060018060a01b03831660208301529392505050565b5f5f5f5f5f5f5f5f5f5f5f6101408c8e031215612ac5575f5ffd5b8b35612ad0816124dd565b9a5060208c0135995060408c01356001600160401b03811115612af1575f5ffd5b612afd8e828f01612772565b909a509850612b10905060608d01612659565b965060808c0135955060a08c0135945060c08c0135935060e08c0135612b35816124dd565b92506101008c0135612b46816124dd565b809250505f6101208d01359050809150509295989b509295989b9093969950565b5f5f60408385031215612b78575f5ffd5b8235612b83816124dd565b91506020830135612b93816124dd565b809150509250929050565b5f5f5f60608486031215612bb0575f5ffd5b8335612bbb816124dd565b92506020840135612bcb816124dd565b91506040840135612bdb816124dd565b809150509250925092565b5f5f5f60608486031215612bf8575f5ffd5b8335612c03816124dd565b92506020840135612c13816124dd565b929592945050506040919091013590565b602080825260059082015264475332303360d81b604082015260600190565b60208082526005908201526411d4cc8c0d60da1b604082015260600190565b634e487b7160e01b5f52601160045260245ffd5b5f60018201612c8757612c87612c62565b5060010190565b604081525f612ca060408301856126cc565b8281036020840152611fae81856126cc565b5f60208284031215612cc2575f5ffd5b81516001600160e01b031981168114611f52575f5ffd5b60ff8281168282160390811115610c6d57610c6d612c62565b8082028115828204841417610c6d57610c6d612c62565b602080825260059082015264475332303160d81b604082015260600190565b634e487b7160e01b5f52602160045260245ffd5b60028110612d5857634e487b7160e01b5f52602160045260245ffd5b9052565b6001600160a01b038d168152602081018c90526101606040820181905281018a9052898b6101808301375f6101808b830101525f601f19601f8c01168201612da7606084018c612d3c565b8960808401528860a08401528760c0840152612dce60e08401886001600160a01b03169052565b6001600160a01b03861661010084015261018083820301610120840152612df96101808201866126cc565b915050612e126101408301846001600160a01b03169052565b9d9c50505050505050505050505050565b80820180821115610c6d57610c6d612c62565b5f82612e5057634e487b7160e01b5f52601260045260245ffd5b500490565b81810381811115610c6d57610c6d612c62565b634e487b7160e01b5f52603260045260245ffd5b608080825281018590525f8660a08301825b88811015612ebe578235612ea1816124dd565b6001600160a01b0316825260209283019290910190600101612e8e565b50602084019690965250506001600160a01b039283166040820152911660609091015292915050565b5f60208284031215612ef7575f5ffd5b81518015158114611f52575f5ffd5b818382375f9101908152919050565b8b81526001600160a01b038b166020820152604081018a9052606081018990526101608101612f47608083018a612d3c565b60a082019790975260c081019590955260e08501939093526001600160a01b03918216610100850152166101208301526101409091015295945050505050565b5f81612f9557612f95612c62565b505f19019056fea2646970667358221220ce6a6c459c521969c7de69701ca90937a8837cb28f38c0ba179005a2741cf32264736f6c634300081c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xD0W_5`\xE0\x1C\x80c\xAF\xFE\xD0\xE0\x11a\0\xF6W\x80c\xE1\x9A\x9D\xD9\x11a\0\x94W\x80c\xF0\x8A\x03#\x11a\0cW\x80c\xF0\x8A\x03#\x14a\x05\xD2W\x80c\xF6\x98\xDA%\x14a\x05\xF1W\x80c\xF8\xDC]\xD9\x14a\x06\x05W\x80c\xFF\xA1\xADt\x14a\x06$Wa\x02\x0CV[\x80c\xE1\x9A\x9D\xD9\x14a\x05aW\x80c\xE3\x18\xB5+\x14a\x05\x80W\x80c\xE7R5\xB8\x14a\x05\x9FW\x80c\xE8f7\xDB\x14a\x05\xB3Wa\x02\x0CV[\x80c\xCC/\x84R\x11a\0\xD0W\x80c\xCC/\x84R\x14a\x04\xD7W\x80c\xD4\xD9\xBD\xCD\x14a\x05\x04W\x80c\xD8\xD1\x1Fx\x14a\x05#W\x80c\xE0\t\xCF\xDE\x14a\x05BWa\x02\x0CV[\x80c\xAF\xFE\xD0\xE0\x14a\x04\x84W\x80c\xB4\xFA\xBA\t\x14a\x04\x99W\x80c\xB6>\x80\r\x14a\x04\xB8Wa\x02\x0CV[\x80cV$\xB2[\x11a\x01nW\x80cjv\x12\x02\x11a\x01=W\x80cjv\x12\x02\x14a\x03\xFBW\x80c}\x83)t\x14a\x04\x0EW\x80c\x93O:\x11\x14a\x04DW\x80c\xA0\xE6~+\x14a\x04cWa\x02\x0CV[\x80cV$\xB2[\x14a\x03fW\x80cZ\xE6\xBD7\x14a\x03\x92W\x80ca\x0BY%\x14a\x03\xBDW\x80ciN\x80\xC3\x14a\x03\xDCWa\x02\x0CV[\x80c/T\xBFn\x11a\x01\xAAW\x80c/T\xBFn\x14a\x02\xDFW\x80c4\x08\xE4p\x14a\x02\xFEW\x80cF\x87!\xA7\x14a\x03\x1AW\x80cR)\x07?\x14a\x039Wa\x02\x0CV[\x80c\rX/\x13\x14a\x02kW\x80c\x12\xFBh\xE0\x14a\x02\x8CW\x80c-\x9A\xD5=\x14a\x02\xABWa\x02\x0CV[6a\x02\x0CW`@Q4\x81R3\x90\x7F=\x0C\xE9\xBF\xC3\xED}hb\xDB\xB2\x8B-\xEA\x94V\x1F\xE7\x14\xA1\xB4\xD0\x19\xAA\x8A\xF3\x970\xD1\xAD|=\x90` \x01`@Q\x80\x91\x03\x90\xA2\0[4\x80\x15a\x02\x17W__\xFD[P\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5\x80T\x80a\x02BW\0[6__73``\x1B6R__`\x146\x01__\x85Z\xF1\x90P=__>\x80a\x02fW=_\xFD[P=_\xF3[4\x80\x15a\x02vW__\xFD[Pa\x02\x8Aa\x02\x856`\x04a%\x04V[a\x06TV[\0[4\x80\x15a\x02\x97W__\xFD[Pa\x02\x8Aa\x02\xA66`\x04a%\xCBV[a\x07\xA9V[4\x80\x15a\x02\xB6W__\xFD[Pa\x02\xCAa\x02\xC56`\x04a&>V[a\x0C:V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xEAW__\xFD[Pa\x02\xCAa\x02\xF96`\x04a&>V[a\x0CsV[4\x80\x15a\x03\tW__\xFD[PF[`@Q\x90\x81R` \x01a\x02\xD6V[4\x80\x15a\x03%W__\xFD[Pa\x02\xCAa\x0346`\x04a&gV[a\x0C\xA9V[4\x80\x15a\x03DW__\xFD[Pa\x03Xa\x03S6`\x04a&gV[a\r}V[`@Qa\x02\xD6\x92\x91\x90a'\x0FV[4\x80\x15a\x03qW__\xFD[Pa\x03\x85a\x03\x806`\x04a')V[a\r\xB1V[`@Qa\x02\xD6\x91\x90a'IV[4\x80\x15a\x03\x9DW__\xFD[Pa\x03\x0Ca\x03\xAC6`\x04a'[V[`\x07` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x03\xC8W__\xFD[Pa\x02\x8Aa\x03\xD76`\x04a&>V[a\x0E*V[4\x80\x15a\x03\xE7W__\xFD[Pa\x02\x8Aa\x03\xF66`\x04a'[V[a\x0FaV[a\x02\xCAa\x04\t6`\x04a'\xB6V[a\x0F\xFFV[4\x80\x15a\x04\x19W__\xFD[Pa\x03\x0Ca\x04(6`\x04a%\x04V[`\x08` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[4\x80\x15a\x04OW__\xFD[Pa\x02\x8Aa\x04^6`\x04a(\x86V[a\x138V[4\x80\x15a\x04nW__\xFD[Pa\x04wa\x13\x82V[`@Qa\x02\xD6\x91\x90a)4V[4\x80\x15a\x04\x8FW__\xFD[Pa\x03\x0C`\x05T\x81V[4\x80\x15a\x04\xA4W__\xFD[Pa\x02\x8Aa\x04\xB36`\x04a)FV[a\x14oV[4\x80\x15a\x04\xC3W__\xFD[Pa\x02\x8Aa\x04\xD26`\x04a)\x92V[a\x14\x8EV[4\x80\x15a\x04\xE2W__\xFD[Pa\x04\xF6a\x04\xF16`\x04a%\x04V[a\x15\x8DV[`@Qa\x02\xD6\x92\x91\x90a*\x81V[4\x80\x15a\x05\x0FW__\xFD[Pa\x02\x8Aa\x05\x1E6`\x04a'[V[a\x17DV[4\x80\x15a\x05.W__\xFD[Pa\x03\x0Ca\x05=6`\x04a*\xAAV[a\x17\xD7V[4\x80\x15a\x05MW__\xFD[Pa\x02\x8Aa\x05\\6`\x04a+gV[a\x18\x03V[4\x80\x15a\x05lW__\xFD[Pa\x02\x8Aa\x05{6`\x04a&>V[a\x19#V[4\x80\x15a\x05\x8BW__\xFD[Pa\x02\x8Aa\x05\x9A6`\x04a+\x9EV[a\x1A6V[4\x80\x15a\x05\xAAW__\xFD[P`\x04Ta\x03\x0CV[4\x80\x15a\x05\xBEW__\xFD[Pa\x03\x85a\x05\xCD6`\x04a*\xAAV[a\x1C\rV[4\x80\x15a\x05\xDDW__\xFD[Pa\x02\x8Aa\x05\xEC6`\x04a&>V[a\x1C\xE4V[4\x80\x15a\x05\xFCW__\xFD[Pa\x03\x0Ca\x1D+V[4\x80\x15a\x06\x10W__\xFD[Pa\x02\x8Aa\x06\x1F6`\x04a+\xE6V[a\x1D\x81V[4\x80\x15a\x06/W__\xFD[Pa\x03\x85`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d1.4.1`\xD8\x1B\x81RP\x81V[a\x06\\a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x06~WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x06\x93WP`\x01`\x01`\xA0\x1B\x03\x82\x160\x14\x15[a\x06\xB8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\x06\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x81\x81R`@\x81 \x80T\x93\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90\x93U`\x01\x83R\x83T\x90\x91\x16\x17\x90\x91U`\x03\x80T\x91a\x07[\x83a,vV[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90_\x90\xA2\x80`\x04T\x14a\x07\xA5Wa\x07\xA5\x81a\x0FaV[PPV[a\x07\xB4\x81`Aa\x1F\"V[\x82Q\x10\x15a\x07\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03#`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x80\x80\x80\x80\x80[\x86\x81\x10\x15a\x0C.W`A\x81\x81\x02\x89\x01` \x81\x01Q`@\x82\x01Q\x91\x90\x92\x01Q`\xFF\x16\x95P\x90\x93P\x91P_\x84\x90\x03a\t\xFEW\x88Q` \x8A\x01 \x8A\x14a\x08`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS027`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x91\x93P\x83\x91a\x08p\x87`Aa\x1F\"V[\x82\x10\x15a\x08\xA7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS021`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x87Qa\x08\xB4\x83` a\x1FYV[\x11\x15a\x08\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[` \x82\x89\x01\x81\x01Q\x89Q\x90\x91a\t\r\x90\x83\x90a\t\x07\x90\x87\x90a\x1FYV[\x90a\x1FYV[\x11\x15a\tCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS023`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`@Qc \xC1;\x0B`\xE0\x1B\x80\x82R\x8A\x85\x01` \x01\x91`\x01`\x01`\xA0\x1B\x03\x89\x16\x90c \xC1;\x0B\x90a\ty\x90\x8F\x90\x86\x90`\x04\x01a,\x8EV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x94W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB8\x91\x90a,\xB2V[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14a\t\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCC\x0C\x8D`\xDA\x1B`D\x82\x01R`d\x01a\x06\xAFV[PPa\x0B\x9EV[\x83`\xFF\x16`\x01\x03a\n\x7FW\x91\x93P\x83\x913`\x01`\x01`\xA0\x1B\x03\x84\x16\x14\x80a\nFWP`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x08` \x90\x81R`@\x80\x83 \x8D\x84R\x90\x91R\x90 T\x15\x15[a\nzW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS025`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\x0B\x9EV[`\x1E\x84`\xFF\x16\x11\x15a\x0BAW`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x8B\x90R`\x01\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\x04\x86a\n\xE4\x91\x90a,\xD9V[`@\x80Q_\x81R` \x81\x01\x80\x83R\x93\x90\x93R`\xFF\x90\x91\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B0W=__>=_\xFD[PPP` `@Q\x03Q\x94Pa\x0B\x9EV[`@\x80Q_\x81R` \x81\x01\x80\x83R\x8C\x90R`\xFF\x86\x16\x91\x81\x01\x91\x90\x91R``\x81\x01\x84\x90R`\x80\x81\x01\x83\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x0B\x91W=__>=_\xFD[PPP` `@Q\x03Q\x94P[\x85`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x11\x80\x15a\x0B\xD7WP`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15\x15[\x80\x15a\x0B\xEDWP`\x01`\x01`\xA0\x1B\x03\x85\x16`\x01\x14\x15[a\x0C!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x19\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x93\x94P\x84\x93`\x01\x01a\x07\xF3V[PPPPPPPPPPV[_`\x01`\x01`\x01`\xA0\x1B\x03\x83\x16\x14\x80\x15\x90a\x0CmWP`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x01` R`@\x90 T\x16\x15\x15[\x92\x91PPV[_`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x80\x15\x90a\x0CmWPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15\x15\x90V[_3`\x01\x14\x80\x15\x90a\x0C\xD1WP3_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x15\x15[a\r\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x11\xD4\xCCL\r`\xDA\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\r\x13\x85\x85\x85\x85_\x19a\x1FsV[\x90P\x80\x15a\rJW`@Q3\x90\x7Fh\x95\xC16d\xAAOg(\x8B%\xD7\xA2\x1Dz\xAA4\x91n5_\xB9\xB6\xFA\xE0\xA19\xA9\x08[\xEC\xB8\x90_\x90\xA2a\ruV[`@Q3\x90\x7F\xAC\xD2\xC8p(\x04\x12\x8F\xDB\r\xB2\xBBI\xF6\xD1'\xDD\x01\x81\xC1?\xD4]\xBF\xE1m\xE0\x93\x0E+\xD3u\x90_\x90\xA2[\x94\x93PPPPV[_``a\r\x8C\x86\x86\x86\x86a\x0C\xA9V[\x91P`@Q` =\x01\x81\x01`@R=\x81R=_` \x83\x01>\x80\x91PP\x94P\x94\x92PPPV[``_a\r\xBF\x83` a,\xF2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xD6Wa\r\xD6a%.V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0E\0W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0E\"W\x84\x81\x01T` \x80\x83\x02\x84\x01\x01R`\x01\x01a\x0E\x05V[P\x93\x92PPPV[a\x0E2a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x0ETWP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x0E\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x01` R`@\x90 T\x16\x15a\x0E\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x84\x81\x16_\x81\x81R`@\x80\x82 \x80T\x94\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x94\x85\x16\x17\x90\x94U\x94\x85R\x83T\x90\x91\x16\x81\x17\x90\x92UQ\x90\x91\x7F\xEC\xDF:>\xFF\xEAW\x83\xA3\xC4\xC2\x14\x0Eguwfd(\xD4N\xD9\xD4t\xA0\xB3\xA4\xC9\x94?\x84@\x91\xA2PV[a\x0Fia\x1E\xE9V[`\x03T\x81\x11\x15a\x0F\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01\x81\x10\x15a\x0F\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x04\x81\x90U`@Q\x81\x81R\x7Fa\x0F\x7F\xF2\xB3\x04\xAE\x89\x03\xC3\xDEt\xC6\x0Cj\xB1\xF7\xD6\"k?R\xC5\x16\x19\x05\xBBZ\xD4\x03\x9C\x93\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[___a\x10\x17\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E\x8E`\x05Ta\x1C\rV[`\x05\x80T\x91\x92P_a\x10(\x83a,vV[\x90\x91UPP\x80Q` \x82\x01 \x91Pa\x10A\x82\x82\x86a\x138V[P_a\x10k\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x10\xECW\x80`\x01`\x01`\xA0\x1B\x03\x16cu\xF0\xBBR\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F\x8F3`@Q\x8Dc\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xBE\x9C\x9B\x9A\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a-\\V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10\xD5W__\xFD[PZ\xF1\x15\x80\x15a\x10\xE7W=__>=_\xFD[PPPP[a\x11\x18a\x10\xFB\x8Aa\t\xC4a.#V[`?a\x11\x08\x8C`@a,\xF2V[a\x11\x12\x91\x90a.6V[\x90a\x1F\xB7V[a\x11$\x90a\x01\xF4a.#V[Z\x10\x15a\x11[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x13`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[_Z\x90Pa\x11\xC9\x8F\x8F\x8F\x8F\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8E\x8C_\x14a\x11\xB6W\x8Ea\x1FsV[a\t\xC4Za\x11\xC4\x91\x90a.UV[a\x1FsV[\x93Pa\x11\xD6Z\x82\x90a\x1F\xCDV[\x90P\x83\x80a\x11\xE3WP\x89\x15\x15[\x80a\x11\xEDWP\x87\x15\x15[a\x12!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS013`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x88\x15a\x128Wa\x125\x82\x8B\x8B\x8B\x8Ba\x1F\xE5V[\x90P[\x84\x15a\x12}W\x83\x7FD.q_bcF\xE8\xC5C\x81\0-\xA6\x14\xF6+\xEE\x8D'8e5\xB2R\x1E\xC8T\x08\x98Un\x82`@Qa\x12p\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x12\xB8V[\x83\x7F#B\x8B\x18\xAC\xFB>\xA6K\x08\xDC\x0C\x1D)n\xA9\xC0\x97\x02\xC0\x90\x83\xCARr\xE6M\x11[h}#\x82`@Qa\x12\xAF\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2[PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x13'W`@Qc\x12d\xE2m`\xE3\x1B\x81R`\x04\x81\x01\x83\x90R\x83\x15\x15`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x93'\x13h\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13\x10W__\xFD[PZ\xF1\x15\x80\x15a\x13\"W=__>=_\xFD[PPPP[PP\x9B\x9APPPPPPPPPPPV[`\x04T\x80a\x13pW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS001`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a\x13|\x84\x84\x84\x84a\x07\xA9V[PPPPV[``_`\x03T`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x9FWa\x13\x9Fa%.V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01_\x90\x81R`\x02` R\x7F\xE9\x0B{\xCE\xB6\xE7\xDFT\x18\xFBx\xD8\xEETn\x97\xC8:\x08\xBB\xCC\xC0\x1A\x06D\xD5\x99\xCC\xD2\xA7\xC2\xE0T\x91\x92P\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14a\x14gW\x80\x83\x83\x81Q\x81\x10a\x14(Wa\x14(a.hV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x91\x81\x16_\x90\x81R`\x02\x90\x92R`@\x90\x91 T\x16\x81a\x14_\x81a,vV[\x92PPa\x14\x04V[P\x90\x92\x91PPV[__\x82Q` \x84\x01\x85Z\xF4\x80_RP=` R=_`@>`@=\x01_\xFD[a\x14\xCB\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8C\x92Pa \xE9\x91PPV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x14\xE3Wa\x14\xE3\x84a\"\xBFV[a\x15\"\x87\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa##\x92PPPV[\x81\x15a\x158Wa\x156\x82_`\x01\x86\x85a\x1F\xE5V[P[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\x14\x1D\xF8h\xA63\x1A\xF5(\xE3\x8C\x83\xB7\xAA\x03\xED\xC1\x9B\xE6n7\xAEg\xF9([\xF4\xF8\xE3\xC6\xA1\xA8\x8B\x8B\x8B\x8B\x89`@Qa\x15y\x95\x94\x93\x92\x91\x90a.|V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPV[``_`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01\x14\x80a\x15\xACWPa\x15\xAC\x84a\x0C:V[a\x15\xE0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS105`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[_\x83\x11a\x16\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x98\x1B`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16/Wa\x16/a%.V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16XW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x01` R`@\x81 T\x92\x94P\x91\x16\x91P[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x16\x9CWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[\x80\x15a\x16\xA7WP\x83\x81\x10[\x15a\x17\x01W\x81\x83\x82\x81Q\x81\x10a\x16\xBFWa\x16\xBFa.hV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16` \x91\x82\x02\x92\x90\x92\x01\x81\x01\x91\x90\x91R\x92\x81\x16_\x90\x81R`\x01\x90\x93R`@\x90\x92 T\x90\x91\x16\x90\x80a\x16\xF9\x81a,vV[\x91PPa\x16zV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14a\x179W\x82a\x17\x1E`\x01\x83a.UV[\x81Q\x81\x10a\x17.Wa\x17.a.hV[` \x02` \x01\x01Q\x91P[\x80\x83RP\x92P\x92\x90PV[3_\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16a\x17\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x033`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[3_\x81\x81R`\x08` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x80\x82 `\x01\x90UQ\x83\x91\x7F\xF2\xA0\xEB\x15dr\xD1D\x02U\xB0\xD7\xC1\xE1\x9C\xC0q\x15\xD1\x05\x1F\xE6\x05\xB0\xDC\xE6\x9A\xCF\xEC\x88M\x9C\x91\xA3PV[_a\x17\xEB\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8C\x8Ca\x1C\rV[\x80Q\x90` \x01 \x90P\x9B\x9APPPPPPPPPPPV[a\x18\x0Ba\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x18-WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[a\x18aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS101`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x90\x81R`\x01` R`@\x90 T\x81\x16\x90\x82\x16\x14a\x18\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS103`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x01` R`@\x80\x82 \x80T\x87\x86\x16\x84R\x82\x84 \x80T\x91\x90\x96\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x95U\x83\x83R\x80T\x90\x94\x16\x90\x93U\x91Q\x90\x91\x7F\xAA\xB4\xFA+F?X\x1B+2\xCB;~;pK\x9C\xE3|\xC2\t\xB5\xFBMw\xE5\x93\xAC\xE4\x05Bv\x91\xA2PPV[a\x19+a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x19\xDBW`@Qc\x01\xFF\xC9\xA7`\xE0\x1B\x81Rcsk\xD4\x1D`\xE1\x1B`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x01\xFF\xC9\xA7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x83W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA7\x91\x90a.\xE7V[a\x19\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u33\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x7FJ Ob\x0C\x8C\\\xCD\xCA?\xD5M\0;\xAD\xD8[\xA5\0CjC\x1F\x0C\xBD\xA4\xF5X\xC9<4\xC8\x81\x81U`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\x11Q\x11i\x14Q[\xC0\x89\x1F\xF9\x04zl\xB3,\xF9\x02To\x83\x06d\x99\xBC\xF8\xBA3\xD25?\xA2\x90_\x90\xA2PPV[a\x1A>a\x1E\xE9V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x1A`WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a\x1AuWP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[a\x1A\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\x1A\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1A\xEAWP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1B\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1BYW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x02` R`@\x80\x82 \x80T\x86\x86\x16\x80\x85R\x83\x85 \x80T\x92\x88\x16`\x01`\x01`\xA0\x1B\x03\x19\x93\x84\x16\x17\x90U\x95\x89\x16\x84R\x82\x84 \x80T\x82\x16\x90\x96\x17\x90\x95U\x83\x83R\x80T\x90\x94\x16\x90\x93U\x91Q\x90\x91\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x91\xA2`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\x94e\xFA\x0C\x96,\xC7iX\xE67:\x993&@\x0C\x1C\x94\xF8\xBE/\xE3\xA9R\xAD\xFA\x7F`\xB2\xEA&\x90_\x90\xA2PPPV[``_\x7F\xBB\x83\x10\xD4\x866\x8D\xB6\xBDo\x84\x94\x02\xFD\xD7:\xD5=1kZK&D\xADn\xFE\x0F\x94\x12\x86\xD8_\x1B\x8D\x8D\x8D\x8D`@Qa\x1CE\x92\x91\x90a/\x06V[`@Q\x90\x81\x90\x03\x81 a\x1Ck\x94\x93\x92\x91\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90` \x01a/\x15V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x90P`\x19`\xF8\x1B`\x01`\xF8\x1Ba\x1C\x97a\x1D+V[`@Q`\x01`\x01`\xF8\x1B\x03\x19\x93\x84\x16` \x82\x01R\x92\x90\x91\x16`!\x83\x01R`\"\x82\x01R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x9B\x9APPPPPPPPPPPV[a\x1C\xECa\x1E\xE9V[a\x1C\xF5\x81a\"\xBFV[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7FZ\xC6\xC4l\x93\xC8\xD0\xE57\x14\xBA;S\xDB>|\x04m\xA9\x941=~\xD0\xD1\x92\x02\x8B\xC7\xC2(\xB0\x90_\x90\xA2PV[_\x7FG\xE7\x954\xA2E\x95.\x8B\x16\x89:3k\x85\xA3\xD9\xEA\x9F\xA8\xC5s\xF3\xD8\x03\xAF\xB9*yF\x92\x18F`@\x80Q` \x81\x01\x93\x90\x93R\x82\x01R0``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[a\x1D\x89a\x1E\xE9V[\x80`\x01`\x03Ta\x1D\x99\x91\x90a.UV[\x10\x15a\x1D\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x80\x15\x90a\x1D\xD9WP`\x01`\x01`\xA0\x1B\x03\x82\x16`\x01\x14\x15[a\x1D\xF5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x02` R`@\x90 T\x81\x16\x90\x83\x16\x14a\x1EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS205`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x02` R`@\x80\x82 \x80T\x88\x86\x16\x84R\x91\x83 \x80T\x92\x90\x95\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x94U\x91\x81R\x82T\x90\x91\x16\x90\x91U`\x03\x80T\x91a\x1E\x9A\x83a/\x87V[\x90\x91UPP`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xF8\xD4\x9F\xC5)\x81.\x9A|\\P\xE6\x9C \xF0\xDC\xCC\r\xB8\xFA\x95\xC9\x8B\xC5\x8C\xC9\xA4\xF1\xC1)\x9E\xAF\x90_\x90\xA2\x80`\x04T\x14a\x1E\xE4Wa\x1E\xE4\x81a\x0FaV[PPPV[30\x14a\x1F W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS031`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[V[_\x82_\x03a\x1F1WP_a\x0CmV[_a\x1F<\x83\x85a,\xF2V[\x90P\x82a\x1FI\x85\x83a.6V[\x14a\x1FRW__\xFD[\x93\x92PPPV[_\x80a\x1Fe\x83\x85a.#V[\x90P\x83\x81\x10\x15a\x1FRW__\xFD[_`\x01\x83`\x01\x81\x11\x15a\x1F\x88Wa\x1F\x88a-(V[\x03a\x1F\x9FW__\x85Q` \x87\x01\x89\x86\xF4\x90Pa\x1F\xAEV[__\x85Q` \x87\x01\x88\x8A\x87\xF1\x90P[\x95\x94PPPPPV[_\x81\x83\x10\x15a\x1F\xC6W\x81a\x1FRV[P\x90\x91\x90PV[_\x82\x82\x11\x15a\x1F\xDAW__\xFD[_a\ru\x83\x85a.UV[_\x80`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1F\xFCW\x82a\x1F\xFEV[2[\x90P`\x01`\x01`\xA0\x1B\x03\x84\x16a \x90Wa 0:\x86\x10a \x1EW:a  V[\x85[a *\x89\x89a\x1FYV[\x90a\x1F\"V[`@Q\x90\x92P`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPPa \x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01RdGS011`\xD8\x1B`D\x82\x01R`d\x01a\x06\xAFV[a \xDFV[a \x9E\x85a *\x89\x89a\x1FYV[\x91Pa \xAB\x84\x82\x84a$QV[a \xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x99`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[P\x95\x94PPPPPV[`\x04T\x15a!!W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3#\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x81Q\x81\x11\x15a!BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a-\tV[`\x01\x81\x10\x15a!{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x99\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01_[\x83Q\x81\x10\x15a\"\x8DW_\x84\x82\x81Q\x81\x10a!\x9BWa!\x9Ba.hV[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a!\xD1WP`\x01`\x01`\xA0\x1B\x03\x81\x16`\x01\x14\x15[\x80\x15a!\xE6WP`\x01`\x01`\xA0\x1B\x03\x81\x160\x14\x15[\x80\x15a\"\x04WP\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[a\" W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,$V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x02` R`@\x90 T\x16\x15a\"WW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xAF\x90a,CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\x01\x01a!\x7FV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x02` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01\x17\x90U\x90Q`\x03U`\x04UV[0`\x01`\x01`\xA0\x1B\x03\x82\x16\x03a\"\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3C\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[\x7Fl\x9AlJ9(N7\xED\x1C\xF5=3uw\xD1B\x12\xA4\x87\x0F\xB9v\xA46li;\x93\x99\x18\xD5UV[`\x01_\x81\x90R` R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/T`\x01`\x01`\xA0\x1B\x03\x16\x15a#\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x13\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[`\x01_\x81\x90R` \x81\x90R\x7F\xCCi\x88_\xDAk\xCC\x1AJ\xCE\x05\x8BJb\xBF^\x17\x9E\xA7\x8F\xD5\x8A\x1C\xCDq\xC2,\xC9\xB6\x88y/\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x91\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x07\xA5W\x81;a$\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd#\xA9\x98\x18\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06\xAFV[a$\x1D\x82_\x83`\x01_\x19a\x1FsV[a\x07\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04u3\x03\x03`\xDC\x1B`D\x82\x01R`d\x01a\x06\xAFV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x80\x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x81R\x82Q_\x93\x92\x91\x84\x91\x90\x82\x89a'\x10Z\x03\xF1=\x80\x15a$\xC1W` \x81\x14a$\xC9W_\x93Pa$\xD3V[\x81\x93Pa$\xD3V[_Q\x15\x82\x15\x17\x15\x93P[PPP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xF1W__\xFD[PV[\x805a$\xFF\x81a$\xDDV[\x91\x90PV[__`@\x83\x85\x03\x12\x15a%\x15W__\xFD[\x825a% \x81a$\xDDV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a%QW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a%jWa%ja%.V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a%\x98Wa%\x98a%.V[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a%\xAFW__\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[____`\x80\x85\x87\x03\x12\x15a%\xDEW__\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xFAW__\xFD[a&\x06\x87\x82\x88\x01a%BV[\x93PP`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&!W__\xFD[a&-\x87\x82\x88\x01a%BV[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[_` \x82\x84\x03\x12\x15a&NW__\xFD[\x815a\x1FR\x81a$\xDDV[\x805`\x02\x81\x10a$\xFFW__\xFD[____`\x80\x85\x87\x03\x12\x15a&zW__\xFD[\x845a&\x85\x81a$\xDDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a&\xA6W__\xFD[a&\xB2\x87\x82\x88\x01a%BV[\x92PPa&\xC1``\x86\x01a&YV[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a&\xF0W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&\xD4V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x82\x15\x15\x81R`@` \x82\x01R_a\ru`@\x83\x01\x84a&\xCCV[__`@\x83\x85\x03\x12\x15a':W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x81R_a\x1FR` \x83\x01\x84a&\xCCV[_` \x82\x84\x03\x12\x15a'kW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12a'\x82W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x98W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a'\xAFW__\xFD[\x92P\x92\x90PV[___________a\x01@\x8C\x8E\x03\x12\x15a'\xD1W__\xFD[a'\xDA\x8Ca$\xF4V[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xFBW__\xFD[a(\x07\x8E\x82\x8F\x01a'rV[\x90\x9AP\x98Pa(\x1A\x90P``\x8D\x01a&YV[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93Pa(=`\xE0\x8D\x01a$\xF4V[\x92Pa(La\x01\0\x8D\x01a$\xF4V[\x91Pa\x01 \x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(gW__\xFD[a(s\x8E\x82\x8F\x01a%BV[\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[___``\x84\x86\x03\x12\x15a(\x98W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB4W__\xFD[a(\xC0\x86\x82\x87\x01a%BV[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDBW__\xFD[a(\xE7\x86\x82\x87\x01a%BV[\x91PP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a)*W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a)\x03V[P\x93\x94\x93PPPPV[` \x81R_a\x1FR` \x83\x01\x84a(\xF1V[__`@\x83\x85\x03\x12\x15a)WW__\xFD[\x825a)b\x81a$\xDDV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)|W__\xFD[a)\x88\x85\x82\x86\x01a%BV[\x91PP\x92P\x92\x90PV[__________a\x01\0\x8B\x8D\x03\x12\x15a)\xACW__\xFD[\x8A5`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xC1W__\xFD[\x8B\x01`\x1F\x81\x01\x8D\x13a)\xD1W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE6W__\xFD[\x8D` \x82`\x05\x1B\x84\x01\x01\x11\x15a)\xFAW__\xFD[` \x91\x82\x01\x9BP\x99P\x8B\x015\x97Pa*\x14`@\x8C\x01a$\xF4V[\x96P``\x8B\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*.W__\xFD[a*:\x8D\x82\x8E\x01a'rV[\x90\x97P\x95Pa*M\x90P`\x80\x8C\x01a$\xF4V[\x93Pa*[`\xA0\x8C\x01a$\xF4V[\x92P`\xC0\x8B\x015\x91Pa*p`\xE0\x8C\x01a$\xF4V[\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`@\x81R_a*\x93`@\x83\x01\x85a(\xF1V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[___________a\x01@\x8C\x8E\x03\x12\x15a*\xC5W__\xFD[\x8B5a*\xD0\x81a$\xDDV[\x9AP` \x8C\x015\x99P`@\x8C\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a*\xF1W__\xFD[a*\xFD\x8E\x82\x8F\x01a'rV[\x90\x9AP\x98Pa+\x10\x90P``\x8D\x01a&YV[\x96P`\x80\x8C\x015\x95P`\xA0\x8C\x015\x94P`\xC0\x8C\x015\x93P`\xE0\x8C\x015a+5\x81a$\xDDV[\x92Pa\x01\0\x8C\x015a+F\x81a$\xDDV[\x80\x92PP_a\x01 \x8D\x015\x90P\x80\x91PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[__`@\x83\x85\x03\x12\x15a+xW__\xFD[\x825a+\x83\x81a$\xDDV[\x91P` \x83\x015a+\x93\x81a$\xDDV[\x80\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a+\xB0W__\xFD[\x835a+\xBB\x81a$\xDDV[\x92P` \x84\x015a+\xCB\x81a$\xDDV[\x91P`@\x84\x015a+\xDB\x81a$\xDDV[\x80\x91PP\x92P\x92P\x92V[___``\x84\x86\x03\x12\x15a+\xF8W__\xFD[\x835a,\x03\x81a$\xDDV[\x92P` \x84\x015a,\x13\x81a$\xDDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[` \x80\x82R`\x05\x90\x82\x01RdGS203`\xD8\x1B`@\x82\x01R``\x01\x90V[` \x80\x82R`\x05\x90\x82\x01Rd\x11\xD4\xCC\x8C\r`\xDA\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a,\x87Wa,\x87a,bV[P`\x01\x01\x90V[`@\x81R_a,\xA0`@\x83\x01\x85a&\xCCV[\x82\x81\x03` \x84\x01Ra\x1F\xAE\x81\x85a&\xCCV[_` \x82\x84\x03\x12\x15a,\xC2W__\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x1FRW__\xFD[`\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0CmWa\x0Cma,bV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0CmWa\x0Cma,bV[` \x80\x82R`\x05\x90\x82\x01RdGS201`\xD8\x1B`@\x82\x01R``\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a-XWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[`\x01`\x01`\xA0\x1B\x03\x8D\x16\x81R` \x81\x01\x8C\x90Ra\x01``@\x82\x01\x81\x90R\x81\x01\x8A\x90R\x89\x8Ba\x01\x80\x83\x017_a\x01\x80\x8B\x83\x01\x01R_`\x1F\x19`\x1F\x8C\x01\x16\x82\x01a-\xA7``\x84\x01\x8Ca-<V[\x89`\x80\x84\x01R\x88`\xA0\x84\x01R\x87`\xC0\x84\x01Ra-\xCE`\xE0\x84\x01\x88`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x86\x16a\x01\0\x84\x01Ra\x01\x80\x83\x82\x03\x01a\x01 \x84\x01Ra-\xF9a\x01\x80\x82\x01\x86a&\xCCV[\x91PPa.\x12a\x01@\x83\x01\x84`\x01`\x01`\xA0\x1B\x03\x16\x90RV[\x9D\x9CPPPPPPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x0CmWa\x0Cma,bV[_\x82a.PWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0CmWa\x0Cma,bV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x80\x80\x82R\x81\x01\x85\x90R_\x86`\xA0\x83\x01\x82[\x88\x81\x10\x15a.\xBEW\x825a.\xA1\x81a$\xDDV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\x8EV[P` \x84\x01\x96\x90\x96RPP`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`@\x82\x01R\x91\x16``\x90\x91\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a.\xF7W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x1FRW__\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x8B\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x16` \x82\x01R`@\x81\x01\x8A\x90R``\x81\x01\x89\x90Ra\x01`\x81\x01a/G`\x80\x83\x01\x8Aa-<V[`\xA0\x82\x01\x97\x90\x97R`\xC0\x81\x01\x95\x90\x95R`\xE0\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16a\x01\0\x85\x01R\x16a\x01 \x83\x01Ra\x01@\x90\x91\x01R\x95\x94PPPPPV[_\x81a/\x95Wa/\x95a,bV[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 \xCEjlE\x9CR\x19i\xC7\xDEip\x1C\xA9\t7\xA8\x83|\xB2\x8F8\xC0\xBA\x17\x90\x05\xA2t\x1C\xF3\"dsolcC\0\x08\x1C\x003",
    );
    /**Event with signature `AddedOwner(address)` and selector `0x9465fa0c962cc76958e6373a993326400c1c94f8be2fe3a952adfa7f60b2ea26`.
    ```solidity
    event AddedOwner(address indexed owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct AddedOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AddedOwner {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "AddedOwner(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    148u8, 101u8, 250u8, 12u8, 150u8, 44u8, 199u8, 105u8, 88u8, 230u8, 55u8, 58u8,
                    153u8, 51u8, 38u8, 64u8, 12u8, 28u8, 148u8, 248u8, 190u8, 47u8, 227u8, 169u8,
                    82u8, 173u8, 250u8, 127u8, 96u8, 178u8, 234u8, 38u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { owner: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.owner.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AddedOwner {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AddedOwner> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AddedOwner) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ApproveHash(bytes32,address)` and selector `0xf2a0eb156472d1440255b0d7c1e19cc07115d1051fe605b0dce69acfec884d9c`.
    ```solidity
    event ApproveHash(bytes32 indexed approvedHash, address indexed owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ApproveHash {
        #[allow(missing_docs)]
        pub approvedHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ApproveHash {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ApproveHash(bytes32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    242u8, 160u8, 235u8, 21u8, 100u8, 114u8, 209u8, 68u8, 2u8, 85u8, 176u8, 215u8,
                    193u8, 225u8, 156u8, 192u8, 113u8, 21u8, 209u8, 5u8, 31u8, 230u8, 5u8, 176u8,
                    220u8, 230u8, 154u8, 207u8, 236u8, 136u8, 77u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { approvedHash: topics.1, owner: topics.2 }
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
                (Self::SIGNATURE_HASH.into(), self.approvedHash.clone(), self.owner.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ApproveHash {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ApproveHash> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ApproveHash) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ChangedFallbackHandler(address)` and selector `0x5ac6c46c93c8d0e53714ba3b53db3e7c046da994313d7ed0d192028bc7c228b0`.
    ```solidity
    event ChangedFallbackHandler(address indexed handler);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ChangedFallbackHandler {
        #[allow(missing_docs)]
        pub handler: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ChangedFallbackHandler {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "ChangedFallbackHandler(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    90u8, 198u8, 196u8, 108u8, 147u8, 200u8, 208u8, 229u8, 55u8, 20u8, 186u8, 59u8,
                    83u8, 219u8, 62u8, 124u8, 4u8, 109u8, 169u8, 148u8, 49u8, 61u8, 126u8, 208u8,
                    209u8, 146u8, 2u8, 139u8, 199u8, 194u8, 40u8, 176u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { handler: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.handler.clone())
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
                    &self.handler,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChangedFallbackHandler {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChangedFallbackHandler> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChangedFallbackHandler) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ChangedGuard(address)` and selector `0x1151116914515bc0891ff9047a6cb32cf902546f83066499bcf8ba33d2353fa2`.
    ```solidity
    event ChangedGuard(address indexed guard);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ChangedGuard {
        #[allow(missing_docs)]
        pub guard: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ChangedGuard {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "ChangedGuard(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    17u8, 81u8, 17u8, 105u8, 20u8, 81u8, 91u8, 192u8, 137u8, 31u8, 249u8, 4u8,
                    122u8, 108u8, 179u8, 44u8, 249u8, 2u8, 84u8, 111u8, 131u8, 6u8, 100u8, 153u8,
                    188u8, 248u8, 186u8, 51u8, 210u8, 53u8, 63u8, 162u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { guard: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.guard.clone())
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
                    &self.guard,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ChangedGuard {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChangedGuard> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChangedGuard) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ChangedThreshold(uint256)` and selector `0x610f7ff2b304ae8903c3de74c60c6ab1f7d6226b3f52c5161905bb5ad4039c93`.
    ```solidity
    event ChangedThreshold(uint256 threshold);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ChangedThreshold {
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ChangedThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChangedThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    97u8, 15u8, 127u8, 242u8, 179u8, 4u8, 174u8, 137u8, 3u8, 195u8, 222u8, 116u8,
                    198u8, 12u8, 106u8, 177u8, 247u8, 214u8, 34u8, 107u8, 63u8, 82u8, 197u8, 22u8,
                    25u8, 5u8, 187u8, 90u8, 212u8, 3u8, 156u8, 147u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { threshold: data.0 }
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
                    &self.threshold,
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
        impl alloy_sol_types::private::IntoLogData for ChangedThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChangedThreshold> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChangedThreshold) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DisabledModule(address)` and selector `0xaab4fa2b463f581b2b32cb3b7e3b704b9ce37cc209b5fb4d77e593ace4054276`.
    ```solidity
    event DisabledModule(address indexed module);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct DisabledModule {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for DisabledModule {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "DisabledModule(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    170u8, 180u8, 250u8, 43u8, 70u8, 63u8, 88u8, 27u8, 43u8, 50u8, 203u8, 59u8,
                    126u8, 59u8, 112u8, 75u8, 156u8, 227u8, 124u8, 194u8, 9u8, 181u8, 251u8, 77u8,
                    119u8, 229u8, 147u8, 172u8, 228u8, 5u8, 66u8, 118u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DisabledModule {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DisabledModule> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DisabledModule) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EnabledModule(address)` and selector `0xecdf3a3effea5783a3c4c2140e677577666428d44ed9d474a0b3a4c9943f8440`.
    ```solidity
    event EnabledModule(address indexed module);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct EnabledModule {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EnabledModule {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "EnabledModule(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    236u8, 223u8, 58u8, 62u8, 255u8, 234u8, 87u8, 131u8, 163u8, 196u8, 194u8, 20u8,
                    14u8, 103u8, 117u8, 119u8, 102u8, 100u8, 40u8, 212u8, 78u8, 217u8, 212u8,
                    116u8, 160u8, 179u8, 164u8, 201u8, 148u8, 63u8, 132u8, 64u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EnabledModule {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EnabledModule> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EnabledModule) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ExecutionFailure(bytes32,uint256)` and selector `0x23428b18acfb3ea64b08dc0c1d296ea9c09702c09083ca5272e64d115b687d23`.
    ```solidity
    event ExecutionFailure(bytes32 indexed txHash, uint256 payment);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ExecutionFailure {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub payment: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionFailure {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ExecutionFailure(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 66u8, 139u8, 24u8, 172u8, 251u8, 62u8, 166u8, 75u8, 8u8, 220u8, 12u8,
                    29u8, 41u8, 110u8, 169u8, 192u8, 151u8, 2u8, 192u8, 144u8, 131u8, 202u8, 82u8,
                    114u8, 230u8, 77u8, 17u8, 91u8, 104u8, 125u8, 35u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { txHash: topics.1, payment: data.0 }
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
                    &self.payment,
                ),)
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
        impl alloy_sol_types::private::IntoLogData for ExecutionFailure {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFailure> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionFailure) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ExecutionFromModuleFailure(address)` and selector `0xacd2c8702804128fdb0db2bb49f6d127dd0181c13fd45dbfe16de0930e2bd375`.
    ```solidity
    event ExecutionFromModuleFailure(address indexed module);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ExecutionFromModuleFailure {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionFromModuleFailure {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "ExecutionFromModuleFailure(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    172u8, 210u8, 200u8, 112u8, 40u8, 4u8, 18u8, 143u8, 219u8, 13u8, 178u8, 187u8,
                    73u8, 246u8, 209u8, 39u8, 221u8, 1u8, 129u8, 193u8, 63u8, 212u8, 93u8, 191u8,
                    225u8, 109u8, 224u8, 147u8, 14u8, 43u8, 211u8, 117u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExecutionFromModuleFailure {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFromModuleFailure> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionFromModuleFailure) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ExecutionFromModuleSuccess(address)` and selector `0x6895c13664aa4f67288b25d7a21d7aaa34916e355fb9b6fae0a139a9085becb8`.
    ```solidity
    event ExecutionFromModuleSuccess(address indexed module);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ExecutionFromModuleSuccess {
        #[allow(missing_docs)]
        pub module: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionFromModuleSuccess {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "ExecutionFromModuleSuccess(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    104u8, 149u8, 193u8, 54u8, 100u8, 170u8, 79u8, 103u8, 40u8, 139u8, 37u8, 215u8,
                    162u8, 29u8, 122u8, 170u8, 52u8, 145u8, 110u8, 53u8, 95u8, 185u8, 182u8, 250u8,
                    224u8, 161u8, 57u8, 169u8, 8u8, 91u8, 236u8, 184u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { module: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.module.clone())
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
                    &self.module,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExecutionFromModuleSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionFromModuleSuccess> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionFromModuleSuccess) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ExecutionSuccess(bytes32,uint256)` and selector `0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e`.
    ```solidity
    event ExecutionSuccess(bytes32 indexed txHash, uint256 payment);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct ExecutionSuccess {
        #[allow(missing_docs)]
        pub txHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub payment: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExecutionSuccess {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ExecutionSuccess(bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    68u8, 46u8, 113u8, 95u8, 98u8, 99u8, 70u8, 232u8, 197u8, 67u8, 129u8, 0u8,
                    45u8, 166u8, 20u8, 246u8, 43u8, 238u8, 141u8, 39u8, 56u8, 101u8, 53u8, 178u8,
                    82u8, 30u8, 200u8, 84u8, 8u8, 152u8, 85u8, 110u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { txHash: topics.1, payment: data.0 }
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
                    &self.payment,
                ),)
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
        impl alloy_sol_types::private::IntoLogData for ExecutionSuccess {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExecutionSuccess> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExecutionSuccess) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RemovedOwner(address)` and selector `0xf8d49fc529812e9a7c5c50e69c20f0dccc0db8fa95c98bc58cc9a4f1c1299eaf`.
    ```solidity
    event RemovedOwner(address indexed owner);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct RemovedOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RemovedOwner {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "RemovedOwner(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    248u8, 212u8, 159u8, 197u8, 41u8, 129u8, 46u8, 154u8, 124u8, 92u8, 80u8, 230u8,
                    156u8, 32u8, 240u8, 220u8, 204u8, 13u8, 184u8, 250u8, 149u8, 201u8, 139u8,
                    197u8, 140u8, 201u8, 164u8, 241u8, 193u8, 41u8, 158u8, 175u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { owner: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.owner.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RemovedOwner {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RemovedOwner> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RemovedOwner) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SafeReceived(address,uint256)` and selector `0x3d0ce9bfc3ed7d6862dbb28b2dea94561fe714a1b4d019aa8af39730d1ad7c3d`.
    ```solidity
    event SafeReceived(address indexed sender, uint256 value);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct SafeReceived {
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SafeReceived {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "SafeReceived(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    61u8, 12u8, 233u8, 191u8, 195u8, 237u8, 125u8, 104u8, 98u8, 219u8, 178u8,
                    139u8, 45u8, 234u8, 148u8, 86u8, 31u8, 231u8, 20u8, 161u8, 180u8, 208u8, 25u8,
                    170u8, 138u8, 243u8, 151u8, 48u8, 209u8, 173u8, 124u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { sender: topics.1, value: data.0 }
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
                    &self.value,
                ),)
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.sender.clone())
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
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SafeReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SafeReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SafeReceived) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SafeSetup(address,address[],uint256,address,address)` and selector `0x141df868a6331af528e38c83b7aa03edc19be66e37ae67f9285bf4f8e3c6a1a8`.
    ```solidity
    event SafeSetup(address indexed initiator, address[] owners, uint256 threshold, address initializer, address fallbackHandler);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct SafeSetup {
        #[allow(missing_docs)]
        pub initiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub owners: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub initializer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub fallbackHandler: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SafeSetup {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList =
                (alloy_sol_types::sol_data::FixedBytes<32>, alloy::sol_types::sol_data::Address);
            const SIGNATURE: &'static str = "SafeSetup(address,address[],uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    20u8, 29u8, 248u8, 104u8, 166u8, 51u8, 26u8, 245u8, 40u8, 227u8, 140u8, 131u8,
                    183u8, 170u8, 3u8, 237u8, 193u8, 155u8, 230u8, 110u8, 55u8, 174u8, 103u8,
                    249u8, 40u8, 91u8, 244u8, 248u8, 227u8, 198u8, 161u8, 168u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    initiator: topics.1,
                    owners: data.0,
                    threshold: data.1,
                    initializer: data.2,
                    fallbackHandler: data.3,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.owners),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.initializer,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.fallbackHandler,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.initiator.clone())
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
                    &self.initiator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SafeSetup {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SafeSetup> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SafeSetup) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SignMsg(bytes32)` and selector `0xe7f4675038f4f6034dfcbbb24c4dc08e4ebf10eb9d257d3d02c0f38d122ac6e4`.
    ```solidity
    event SignMsg(bytes32 indexed msgHash);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    #[derive(Clone)]
    pub struct SignMsg {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SignMsg {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "SignMsg(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    231u8, 244u8, 103u8, 80u8, 56u8, 244u8, 246u8, 3u8, 77u8, 252u8, 187u8, 178u8,
                    76u8, 77u8, 192u8, 142u8, 78u8, 191u8, 16u8, 235u8, 157u8, 37u8, 125u8, 61u8,
                    2u8, 192u8, 243u8, 141u8, 18u8, 42u8, 198u8, 228u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { msgHash: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.msgHash.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.msgHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SignMsg {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SignMsg> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SignMsg) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `VERSION()` and selector `0xffa1ad74`.
    ```solidity
    function VERSION() external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONCall {}
    ///Container type for the return parameters of the [`VERSION()`](VERSIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VERSIONReturn {
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
            impl ::core::convert::From<VERSIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONCall {
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
            impl ::core::convert::From<VERSIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: VERSIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for VERSIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for VERSIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = VERSIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VERSION()";
            const SELECTOR: [u8; 4] = [255u8, 161u8, 173u8, 116u8];
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
    /**Function with signature `addOwnerWithThreshold(address,uint256)` and selector `0x0d582f13`.
    ```solidity
    function addOwnerWithThreshold(address owner, uint256 _threshold) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOwnerWithThresholdCall {
        pub owner: alloy::sol_types::private::Address,
        pub _threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`addOwnerWithThreshold(address,uint256)`](addOwnerWithThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addOwnerWithThresholdReturn {}
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
            impl ::core::convert::From<addOwnerWithThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: addOwnerWithThresholdCall) -> Self {
                    (value.owner, value._threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addOwnerWithThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0, _threshold: tuple.1 }
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
            impl ::core::convert::From<addOwnerWithThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addOwnerWithThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addOwnerWithThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addOwnerWithThresholdCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addOwnerWithThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addOwnerWithThreshold(address,uint256)";
            const SELECTOR: [u8; 4] = [13u8, 88u8, 47u8, 19u8];
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
                        &self._threshold,
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
    /**Function with signature `approveHash(bytes32)` and selector `0xd4d9bdcd`.
    ```solidity
    function approveHash(bytes32 hashToApprove) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveHashCall {
        pub hashToApprove: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`approveHash(bytes32)`](approveHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approveHashReturn {}
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
            impl ::core::convert::From<approveHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveHashCall) -> Self {
                    (value.hashToApprove,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { hashToApprove: tuple.0 }
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
            impl ::core::convert::From<approveHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveHashReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveHashReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approveHash(bytes32)";
            const SELECTOR: [u8; 4] = [212u8, 217u8, 189u8, 205u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.hashToApprove),
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
    /**Function with signature `approvedHashes(address,bytes32)` and selector `0x7d832974`.
    ```solidity
    function approvedHashes(address, bytes32) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approvedHashesCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`approvedHashes(address,bytes32)`](approvedHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct approvedHashesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
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
            impl ::core::convert::From<approvedHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: approvedHashesCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approvedHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<approvedHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approvedHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approvedHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approvedHashesCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::FixedBytes<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = approvedHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approvedHashes(address,bytes32)";
            const SELECTOR: [u8; 4] = [125u8, 131u8, 41u8, 116u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
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
    /**Function with signature `changeThreshold(uint256)` and selector `0x694e80c3`.
    ```solidity
    function changeThreshold(uint256 _threshold) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeThresholdCall {
        pub _threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`changeThreshold(uint256)`](changeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct changeThresholdReturn {}
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
            impl ::core::convert::From<changeThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: changeThresholdCall) -> Self {
                    (value._threshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for changeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _threshold: tuple.0 }
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
            impl ::core::convert::From<changeThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: changeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for changeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for changeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = changeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "changeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [105u8, 78u8, 128u8, 195u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                    &self._threshold,
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
    /**Function with signature `checkNSignatures(bytes32,bytes,bytes,uint256)` and selector `0x12fb68e0`.
    ```solidity
    function checkNSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures, uint256 requiredSignatures) external view;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkNSignaturesCall {
        pub dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub data: alloy::sol_types::private::Bytes,
        pub signatures: alloy::sol_types::private::Bytes,
        pub requiredSignatures: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`checkNSignatures(bytes32,bytes,bytes,uint256)`](checkNSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkNSignaturesReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<checkNSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkNSignaturesCall) -> Self {
                    (value.dataHash, value.data, value.signatures, value.requiredSignatures)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkNSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        dataHash: tuple.0,
                        data: tuple.1,
                        signatures: tuple.2,
                        requiredSignatures: tuple.3,
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
            impl ::core::convert::From<checkNSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkNSignaturesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkNSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkNSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkNSignaturesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkNSignatures(bytes32,bytes,bytes,uint256)";
            const SELECTOR: [u8; 4] = [18u8, 251u8, 104u8, 224u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signatures,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requiredSignatures),
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
    /**Function with signature `checkSignatures(bytes32,bytes,bytes)` and selector `0x934f3a11`.
    ```solidity
    function checkSignatures(bytes32 dataHash, bytes memory data, bytes memory signatures) external view;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        pub dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub data: alloy::sol_types::private::Bytes,
        pub signatures: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,bytes)`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (value.dataHash, value.data, value.signatures)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { dataHash: tuple.0, data: tuple.1, signatures: tuple.2 }
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
            impl ::core::convert::From<checkSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,bytes)";
            const SELECTOR: [u8; 4] = [147u8, 79u8, 58u8, 17u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signatures,
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
    /**Function with signature `disableModule(address,address)` and selector `0xe009cfde`.
    ```solidity
    function disableModule(address prevModule, address module) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableModuleCall {
        pub prevModule: alloy::sol_types::private::Address,
        pub module: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`disableModule(address,address)`](disableModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableModuleReturn {}
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
            impl ::core::convert::From<disableModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableModuleCall) -> Self {
                    (value.prevModule, value.module)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prevModule: tuple.0, module: tuple.1 }
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
            impl ::core::convert::From<disableModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableModuleCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableModule(address,address)";
            const SELECTOR: [u8; 4] = [224u8, 9u8, 207u8, 222u8];
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
                        &self.prevModule,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.module,
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
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
    ```solidity
    function domainSeparator() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall {}
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
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
            impl ::core::convert::From<domainSeparatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
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
    /**Function with signature `enableModule(address)` and selector `0x610b5925`.
    ```solidity
    function enableModule(address module) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableModuleCall {
        pub module: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`enableModule(address)`](enableModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct enableModuleReturn {}
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
            impl ::core::convert::From<enableModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: enableModuleCall) -> Self {
                    (value.module,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { module: tuple.0 }
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
            impl ::core::convert::From<enableModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: enableModuleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for enableModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for enableModuleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = enableModuleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "enableModule(address)";
            const SELECTOR: [u8; 4] = [97u8, 11u8, 89u8, 37u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.module,
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
    /**Function with signature `encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xe86637db`.
    ```solidity
    function encodeTransactionData(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeTransactionDataCall {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        pub safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
        pub baseGas: alloy::sol_types::private::primitives::aliases::U256,
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub gasToken: alloy::sol_types::private::Address,
        pub refundReceiver: alloy::sol_types::private::Address,
        pub _nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)`](encodeTransactionDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeTransactionDataReturn {
        pub _0: alloy::sol_types::private::Bytes,
    }
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
                alloy::sol_types::sol_data::Uint<256>,
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
            impl ::core::convert::From<encodeTransactionDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: encodeTransactionDataCall) -> Self {
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
                        value._nonce,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encodeTransactionDataCall {
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
                        _nonce: tuple.9,
                    }
                }
            }
        }
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
            impl ::core::convert::From<encodeTransactionDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: encodeTransactionDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encodeTransactionDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for encodeTransactionDataCall {
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = encodeTransactionDataReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "encodeTransactionData(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)";
            const SELECTOR: [u8; 4] = [232u8, 102u8, 55u8, 219u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._nonce,
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
    /**Function with signature `execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)` and selector `0x6a761202`.
    ```solidity
    function execTransaction(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, bytes memory signatures) external payable returns (bool success);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionCall {
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
    }
    ///Container type for the return parameters of the [`execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)`](execTransactionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionReturn {
        pub success: bool,
    }
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
            impl ::core::convert::From<execTransactionCall> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionCall) -> Self {
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
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionCall {
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
                    }
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
            impl ::core::convert::From<execTransactionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionReturn) -> Self {
                    (value.success,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { success: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionCall {
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
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = execTransactionReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)";
            const SELECTOR: [u8; 4] = [106u8, 118u8, 18u8, 2u8];
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
    /**Function with signature `execTransactionFromModule(address,uint256,bytes,uint8)` and selector `0x468721a7`.
    ```solidity
    function execTransactionFromModule(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleCall {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`execTransactionFromModule(address,uint256,bytes,uint8)`](execTransactionFromModuleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturn {
        pub success: bool,
    }
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleCall> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionFromModuleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0, value: tuple.1, data: tuple.2, operation: tuple.3 }
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
            impl ::core::convert::From<execTransactionFromModuleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturn) -> Self {
                    (value.success,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionFromModuleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { success: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = execTransactionFromModuleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "execTransactionFromModule(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [70u8, 135u8, 33u8, 167u8];
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
    /**Function with signature `execTransactionFromModuleReturnData(address,uint256,bytes,uint8)` and selector `0x5229073f`.
    ```solidity
    function execTransactionFromModuleReturnData(address to, uint256 value, bytes memory data, Enum.Operation operation) external returns (bool success, bytes memory returnData);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataCall {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`execTransactionFromModuleReturnData(address,uint256,bytes,uint8)`](execTransactionFromModuleReturnDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct execTransactionFromModuleReturnDataReturn {
        pub success: bool,
        pub returnData: alloy::sol_types::private::Bytes,
    }
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
                <Enum::Operation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<execTransactionFromModuleReturnDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataCall) -> Self {
                    (value.to, value.value, value.data, value.operation)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionFromModuleReturnDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { to: tuple.0, value: tuple.1, data: tuple.2, operation: tuple.3 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Bool, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<execTransactionFromModuleReturnDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: execTransactionFromModuleReturnDataReturn) -> Self {
                    (value.success, value.returnData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for execTransactionFromModuleReturnDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { success: tuple.0, returnData: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for execTransactionFromModuleReturnDataCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                Enum::Operation,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = execTransactionFromModuleReturnDataReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Bool, alloy::sol_types::sol_data::Bytes);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "execTransactionFromModuleReturnData(address,uint256,bytes,uint8)";
            const SELECTOR: [u8; 4] = [82u8, 41u8, 7u8, 63u8];
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
    /**Function with signature `getChainId()` and selector `0x3408e470`.
    ```solidity
    function getChainId() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChainIdCall {}
    ///Container type for the return parameters of the [`getChainId()`](getChainIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getChainIdReturn {
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
            impl ::core::convert::From<getChainIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getChainIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getChainIdCall {
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
            impl ::core::convert::From<getChainIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getChainIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getChainIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getChainIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getChainIdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getChainId()";
            const SELECTOR: [u8; 4] = [52u8, 8u8, 228u8, 112u8];
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
    /**Function with signature `getModulesPaginated(address,uint256)` and selector `0xcc2f8452`.
    ```solidity
    function getModulesPaginated(address start, uint256 pageSize) external view returns (address[] memory array, address next);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getModulesPaginatedCall {
        pub start: alloy::sol_types::private::Address,
        pub pageSize: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getModulesPaginated(address,uint256)`](getModulesPaginatedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getModulesPaginatedReturn {
        pub array: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub next: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<getModulesPaginatedCall> for UnderlyingRustTuple<'_> {
                fn from(value: getModulesPaginatedCall) -> Self {
                    (value.start, value.pageSize)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getModulesPaginatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { start: tuple.0, pageSize: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getModulesPaginatedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getModulesPaginatedReturn) -> Self {
                    (value.array, value.next)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getModulesPaginatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { array: tuple.0, next: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getModulesPaginatedCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getModulesPaginatedReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getModulesPaginated(address,uint256)";
            const SELECTOR: [u8; 4] = [204u8, 47u8, 132u8, 82u8];
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
                        &self.start,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.pageSize,
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
    /**Function with signature `getOwners()` and selector `0xa0e67e2b`.
    ```solidity
    function getOwners() external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOwnersCall {}
    ///Container type for the return parameters of the [`getOwners()`](getOwnersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOwnersReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getOwnersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOwnersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOwnersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOwnersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOwnersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOwnersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOwnersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOwnersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOwners()";
            const SELECTOR: [u8; 4] = [160u8, 230u8, 126u8, 43u8];
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
    /**Function with signature `getStorageAt(uint256,uint256)` and selector `0x5624b25b`.
    ```solidity
    function getStorageAt(uint256 offset, uint256 length) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStorageAtCall {
        pub offset: alloy::sol_types::private::primitives::aliases::U256,
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getStorageAt(uint256,uint256)`](getStorageAtCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStorageAtReturn {
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getStorageAtCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStorageAtCall) -> Self {
                    (value.offset, value.length)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStorageAtCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { offset: tuple.0, length: tuple.1 }
                }
            }
        }
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
            impl ::core::convert::From<getStorageAtReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStorageAtReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStorageAtReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStorageAtCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Uint<256>, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStorageAtReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStorageAt(uint256,uint256)";
            const SELECTOR: [u8; 4] = [86u8, 36u8, 178u8, 91u8];
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
                        &self.offset,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.length,
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
    /**Function with signature `getThreshold()` and selector `0xe75235b8`.
    ```solidity
    function getThreshold() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getThresholdCall {}
    ///Container type for the return parameters of the [`getThreshold()`](getThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getThresholdReturn {
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
            impl ::core::convert::From<getThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getThresholdCall {
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
            impl ::core::convert::From<getThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getThresholdReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getThreshold()";
            const SELECTOR: [u8; 4] = [231u8, 82u8, 53u8, 184u8];
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
    /**Function with signature `getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)` and selector `0xd8d11f78`.
    ```solidity
    function getTransactionHash(address to, uint256 value, bytes memory data, Enum.Operation operation, uint256 safeTxGas, uint256 baseGas, uint256 gasPrice, address gasToken, address refundReceiver, uint256 _nonce) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransactionHashCall {
        pub to: alloy::sol_types::private::Address,
        pub value: alloy::sol_types::private::primitives::aliases::U256,
        pub data: alloy::sol_types::private::Bytes,
        pub operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        pub safeTxGas: alloy::sol_types::private::primitives::aliases::U256,
        pub baseGas: alloy::sol_types::private::primitives::aliases::U256,
        pub gasPrice: alloy::sol_types::private::primitives::aliases::U256,
        pub gasToken: alloy::sol_types::private::Address,
        pub refundReceiver: alloy::sol_types::private::Address,
        pub _nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)`](getTransactionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransactionHashReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
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
                alloy::sol_types::sol_data::Uint<256>,
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
            impl ::core::convert::From<getTransactionHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionHashCall) -> Self {
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
                        value._nonce,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransactionHashCall {
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
                        _nonce: tuple.9,
                    }
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
            impl ::core::convert::From<getTransactionHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTransactionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransactionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTransactionHashCall {
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTransactionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTransactionHash(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,uint256)";
            const SELECTOR: [u8; 4] = [216u8, 209u8, 31u8, 120u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._nonce,
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
    /**Function with signature `isModuleEnabled(address)` and selector `0x2d9ad53d`.
    ```solidity
    function isModuleEnabled(address module) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isModuleEnabledCall {
        pub module: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isModuleEnabled(address)`](isModuleEnabledCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isModuleEnabledReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isModuleEnabledCall> for UnderlyingRustTuple<'_> {
                fn from(value: isModuleEnabledCall) -> Self {
                    (value.module,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isModuleEnabledCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { module: tuple.0 }
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
            impl ::core::convert::From<isModuleEnabledReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isModuleEnabledReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isModuleEnabledReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isModuleEnabledCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isModuleEnabledReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isModuleEnabled(address)";
            const SELECTOR: [u8; 4] = [45u8, 154u8, 213u8, 61u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.module,
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
    /**Function with signature `isOwner(address)` and selector `0x2f54bf6e`.
    ```solidity
    function isOwner(address owner) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOwnerCall {
        pub owner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOwner(address)`](isOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOwnerReturn {
        pub _0: bool,
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
            impl ::core::convert::From<isOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOwnerCall) -> Self {
                    (value.owner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { owner: tuple.0 }
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
            impl ::core::convert::From<isOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOwnerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOwnerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOwner(address)";
            const SELECTOR: [u8; 4] = [47u8, 84u8, 191u8, 110u8];
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
    /**Function with signature `nonce()` and selector `0xaffed0e0`.
    ```solidity
    function nonce() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceCall {}
    ///Container type for the return parameters of the [`nonce()`](nonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonceReturn {
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
            impl ::core::convert::From<nonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: nonceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonceCall {
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
            impl ::core::convert::From<nonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonce()";
            const SELECTOR: [u8; 4] = [175u8, 254u8, 208u8, 224u8];
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
    /**Function with signature `removeOwner(address,address,uint256)` and selector `0xf8dc5dd9`.
    ```solidity
    function removeOwner(address prevOwner, address owner, uint256 _threshold) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeOwnerCall {
        pub prevOwner: alloy::sol_types::private::Address,
        pub owner: alloy::sol_types::private::Address,
        pub _threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeOwner(address,address,uint256)`](removeOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeOwnerReturn {}
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
            impl ::core::convert::From<removeOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeOwnerCall) -> Self {
                    (value.prevOwner, value.owner, value._threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prevOwner: tuple.0, owner: tuple.1, _threshold: tuple.2 }
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
            impl ::core::convert::From<removeOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeOwnerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeOwner(address,address,uint256)";
            const SELECTOR: [u8; 4] = [248u8, 220u8, 93u8, 217u8];
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
                        &self.prevOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._threshold,
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
    /**Function with signature `setFallbackHandler(address)` and selector `0xf08a0323`.
    ```solidity
    function setFallbackHandler(address handler) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFallbackHandlerCall {
        pub handler: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setFallbackHandler(address)`](setFallbackHandlerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setFallbackHandlerReturn {}
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
            impl ::core::convert::From<setFallbackHandlerCall> for UnderlyingRustTuple<'_> {
                fn from(value: setFallbackHandlerCall) -> Self {
                    (value.handler,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFallbackHandlerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { handler: tuple.0 }
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
            impl ::core::convert::From<setFallbackHandlerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setFallbackHandlerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setFallbackHandlerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setFallbackHandlerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setFallbackHandlerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setFallbackHandler(address)";
            const SELECTOR: [u8; 4] = [240u8, 138u8, 3u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.handler,
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
    /**Function with signature `setGuard(address)` and selector `0xe19a9dd9`.
    ```solidity
    function setGuard(address guard) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGuardCall {
        pub guard: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setGuard(address)`](setGuardCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGuardReturn {}
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
            impl ::core::convert::From<setGuardCall> for UnderlyingRustTuple<'_> {
                fn from(value: setGuardCall) -> Self {
                    (value.guard,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGuardCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { guard: tuple.0 }
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
            impl ::core::convert::From<setGuardReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setGuardReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGuardReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGuardCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGuardReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGuard(address)";
            const SELECTOR: [u8; 4] = [225u8, 154u8, 157u8, 217u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                    &self.guard,
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
    /**Function with signature `setup(address[],uint256,address,bytes,address,address,uint256,address)` and selector `0xb63e800d`.
    ```solidity
    function setup(address[] memory _owners, uint256 _threshold, address to, bytes memory data, address fallbackHandler, address paymentToken, uint256 payment, address paymentReceiver) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setupCall {
        pub _owners: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub _threshold: alloy::sol_types::private::primitives::aliases::U256,
        pub to: alloy::sol_types::private::Address,
        pub data: alloy::sol_types::private::Bytes,
        pub fallbackHandler: alloy::sol_types::private::Address,
        pub paymentToken: alloy::sol_types::private::Address,
        pub payment: alloy::sol_types::private::primitives::aliases::U256,
        pub paymentReceiver: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setup(address[],uint256,address,bytes,address,address,uint256,address)`](setupCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setupReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<setupCall> for UnderlyingRustTuple<'_> {
                fn from(value: setupCall) -> Self {
                    (
                        value._owners,
                        value._threshold,
                        value.to,
                        value.data,
                        value.fallbackHandler,
                        value.paymentToken,
                        value.payment,
                        value.paymentReceiver,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setupCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _owners: tuple.0,
                        _threshold: tuple.1,
                        to: tuple.2,
                        data: tuple.3,
                        fallbackHandler: tuple.4,
                        paymentToken: tuple.5,
                        payment: tuple.6,
                        paymentReceiver: tuple.7,
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
            impl ::core::convert::From<setupReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setupReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setupReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setupCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setupReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "setup(address[],uint256,address,bytes,address,address,uint256,address)";
            const SELECTOR: [u8; 4] = [182u8, 62u8, 128u8, 13u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._owners),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._threshold),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.fallbackHandler,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentToken,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.payment),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.paymentReceiver,
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
    /**Function with signature `signedMessages(bytes32)` and selector `0x5ae6bd37`.
    ```solidity
    function signedMessages(bytes32) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedMessagesCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`signedMessages(bytes32)`](signedMessagesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedMessagesReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<signedMessagesCall> for UnderlyingRustTuple<'_> {
                fn from(value: signedMessagesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for signedMessagesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<signedMessagesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: signedMessagesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for signedMessagesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signedMessagesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = signedMessagesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signedMessages(bytes32)";
            const SELECTOR: [u8; 4] = [90u8, 230u8, 189u8, 55u8];
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
    /**Function with signature `simulateAndRevert(address,bytes)` and selector `0xb4faba09`.
    ```solidity
    function simulateAndRevert(address targetContract, bytes memory calldataPayload) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateAndRevertCall {
        pub targetContract: alloy::sol_types::private::Address,
        pub calldataPayload: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`simulateAndRevert(address,bytes)`](simulateAndRevertCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct simulateAndRevertReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<simulateAndRevertCall> for UnderlyingRustTuple<'_> {
                fn from(value: simulateAndRevertCall) -> Self {
                    (value.targetContract, value.calldataPayload)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for simulateAndRevertCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetContract: tuple.0, calldataPayload: tuple.1 }
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
            impl ::core::convert::From<simulateAndRevertReturn> for UnderlyingRustTuple<'_> {
                fn from(value: simulateAndRevertReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for simulateAndRevertReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for simulateAndRevertCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Address, alloy::sol_types::sol_data::Bytes);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = simulateAndRevertReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "simulateAndRevert(address,bytes)";
            const SELECTOR: [u8; 4] = [180u8, 250u8, 186u8, 9u8];
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
                        &self.targetContract,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.calldataPayload,
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
    /**Function with signature `swapOwner(address,address,address)` and selector `0xe318b52b`.
    ```solidity
    function swapOwner(address prevOwner, address oldOwner, address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapOwnerCall {
        pub prevOwner: alloy::sol_types::private::Address,
        pub oldOwner: alloy::sol_types::private::Address,
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`swapOwner(address,address,address)`](swapOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct swapOwnerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<swapOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapOwnerCall) -> Self {
                    (value.prevOwner, value.oldOwner, value.newOwner)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prevOwner: tuple.0, oldOwner: tuple.1, newOwner: tuple.2 }
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
            impl ::core::convert::From<swapOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: swapOwnerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapOwnerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapOwnerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swapOwner(address,address,address)";
            const SELECTOR: [u8; 4] = [227u8, 24u8, 181u8, 43u8];
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
                        &self.prevOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.oldOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
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
    ///Container for all the [`Safe`](self) function calls.
    pub enum SafeCalls {
        VERSION(VERSIONCall),
        addOwnerWithThreshold(addOwnerWithThresholdCall),
        approveHash(approveHashCall),
        approvedHashes(approvedHashesCall),
        changeThreshold(changeThresholdCall),
        checkNSignatures(checkNSignaturesCall),
        checkSignatures(checkSignaturesCall),
        disableModule(disableModuleCall),
        domainSeparator(domainSeparatorCall),
        enableModule(enableModuleCall),
        encodeTransactionData(encodeTransactionDataCall),
        execTransaction(execTransactionCall),
        execTransactionFromModule(execTransactionFromModuleCall),
        execTransactionFromModuleReturnData(execTransactionFromModuleReturnDataCall),
        getChainId(getChainIdCall),
        getModulesPaginated(getModulesPaginatedCall),
        getOwners(getOwnersCall),
        getStorageAt(getStorageAtCall),
        getThreshold(getThresholdCall),
        getTransactionHash(getTransactionHashCall),
        isModuleEnabled(isModuleEnabledCall),
        isOwner(isOwnerCall),
        nonce(nonceCall),
        removeOwner(removeOwnerCall),
        setFallbackHandler(setFallbackHandlerCall),
        setGuard(setGuardCall),
        setup(setupCall),
        signedMessages(signedMessagesCall),
        simulateAndRevert(simulateAndRevertCall),
        swapOwner(swapOwnerCall),
    }
    #[automatically_derived]
    impl SafeCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 88u8, 47u8, 19u8],
            [18u8, 251u8, 104u8, 224u8],
            [45u8, 154u8, 213u8, 61u8],
            [47u8, 84u8, 191u8, 110u8],
            [52u8, 8u8, 228u8, 112u8],
            [70u8, 135u8, 33u8, 167u8],
            [82u8, 41u8, 7u8, 63u8],
            [86u8, 36u8, 178u8, 91u8],
            [90u8, 230u8, 189u8, 55u8],
            [97u8, 11u8, 89u8, 37u8],
            [105u8, 78u8, 128u8, 195u8],
            [106u8, 118u8, 18u8, 2u8],
            [125u8, 131u8, 41u8, 116u8],
            [147u8, 79u8, 58u8, 17u8],
            [160u8, 230u8, 126u8, 43u8],
            [175u8, 254u8, 208u8, 224u8],
            [180u8, 250u8, 186u8, 9u8],
            [182u8, 62u8, 128u8, 13u8],
            [204u8, 47u8, 132u8, 82u8],
            [212u8, 217u8, 189u8, 205u8],
            [216u8, 209u8, 31u8, 120u8],
            [224u8, 9u8, 207u8, 222u8],
            [225u8, 154u8, 157u8, 217u8],
            [227u8, 24u8, 181u8, 43u8],
            [231u8, 82u8, 53u8, 184u8],
            [232u8, 102u8, 55u8, 219u8],
            [240u8, 138u8, 3u8, 35u8],
            [246u8, 152u8, 218u8, 37u8],
            [248u8, 220u8, 93u8, 217u8],
            [255u8, 161u8, 173u8, 116u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for SafeCalls {
        const NAME: &'static str = "SafeCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::VERSION(_) => <VERSIONCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::addOwnerWithThreshold(_) => {
                    <addOwnerWithThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approveHash(_) => <approveHashCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::approvedHashes(_) => {
                    <approvedHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::changeThreshold(_) => {
                    <changeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkNSignatures(_) => {
                    <checkNSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableModule(_) => <disableModuleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::enableModule(_) => <enableModuleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::encodeTransactionData(_) => {
                    <encodeTransactionDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransaction(_) => {
                    <execTransactionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModule(_) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execTransactionFromModuleReturnData(_) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getChainId(_) => <getChainIdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getModulesPaginated(_) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOwners(_) => <getOwnersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getStorageAt(_) => <getStorageAtCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getThreshold(_) => <getThresholdCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getTransactionHash(_) => {
                    <getTransactionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isModuleEnabled(_) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOwner(_) => <isOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nonce(_) => <nonceCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::removeOwner(_) => <removeOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setFallbackHandler(_) => {
                    <setFallbackHandlerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGuard(_) => <setGuardCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::setup(_) => <setupCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::signedMessages(_) => {
                    <signedMessagesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::simulateAndRevert(_) => {
                    <simulateAndRevertCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swapOwner(_) => <swapOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<SafeCalls>] = &[
                {
                    fn addOwnerWithThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <addOwnerWithThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::addOwnerWithThreshold)
                    }
                    addOwnerWithThreshold
                },
                {
                    fn checkNSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <checkNSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::checkNSignatures)
                    }
                    checkNSignatures
                },
                {
                    fn isModuleEnabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::isModuleEnabled)
                    }
                    isModuleEnabled
                },
                {
                    fn isOwner(data: &[u8], validate: bool) -> alloy_sol_types::Result<SafeCalls> {
                        <isOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::isOwner)
                    }
                    isOwner
                },
                {
                    fn getChainId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getChainIdCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::getChainId)
                    }
                    getChainId
                },
                {
                    fn execTransactionFromModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::execTransactionFromModule)
                    }
                    execTransactionFromModule
                },
                {
                    fn execTransactionFromModuleReturnData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(SafeCalls::execTransactionFromModuleReturnData)
                    }
                    execTransactionFromModuleReturnData
                },
                {
                    fn getStorageAt(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getStorageAtCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::getStorageAt)
                    }
                    getStorageAt
                },
                {
                    fn signedMessages(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <signedMessagesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::signedMessages)
                    }
                    signedMessages
                },
                {
                    fn enableModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <enableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::enableModule)
                    }
                    enableModule
                },
                {
                    fn changeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <changeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::changeThreshold)
                    }
                    changeThreshold
                },
                {
                    fn execTransaction(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <execTransactionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::execTransaction)
                    }
                    execTransaction
                },
                {
                    fn approvedHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <approvedHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::approvedHashes)
                    }
                    approvedHashes
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn getOwners(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getOwnersCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::getOwners)
                    }
                    getOwners
                },
                {
                    fn nonce(data: &[u8], validate: bool) -> alloy_sol_types::Result<SafeCalls> {
                        <nonceCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::nonce)
                    }
                    nonce
                },
                {
                    fn simulateAndRevert(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <simulateAndRevertCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::simulateAndRevert)
                    }
                    simulateAndRevert
                },
                {
                    fn setup(data: &[u8], validate: bool) -> alloy_sol_types::Result<SafeCalls> {
                        <setupCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::setup)
                    }
                    setup
                },
                {
                    fn getModulesPaginated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::getModulesPaginated)
                    }
                    getModulesPaginated
                },
                {
                    fn approveHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <approveHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::approveHash)
                    }
                    approveHash
                },
                {
                    fn getTransactionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getTransactionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::getTransactionHash)
                    }
                    getTransactionHash
                },
                {
                    fn disableModule(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <disableModuleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::disableModule)
                    }
                    disableModule
                },
                {
                    fn setGuard(data: &[u8], validate: bool) -> alloy_sol_types::Result<SafeCalls> {
                        <setGuardCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::setGuard)
                    }
                    setGuard
                },
                {
                    fn swapOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <swapOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::swapOwner)
                    }
                    swapOwner
                },
                {
                    fn getThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <getThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::getThreshold)
                    }
                    getThreshold
                },
                {
                    fn encodeTransactionData(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <encodeTransactionDataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::encodeTransactionData)
                    }
                    encodeTransactionData
                },
                {
                    fn setFallbackHandler(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <setFallbackHandlerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::setFallbackHandler)
                    }
                    setFallbackHandler
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn removeOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<SafeCalls> {
                        <removeOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(SafeCalls::removeOwner)
                    }
                    removeOwner
                },
                {
                    fn VERSION(data: &[u8], validate: bool) -> alloy_sol_types::Result<SafeCalls> {
                        <VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(SafeCalls::VERSION)
                    }
                    VERSION
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
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addOwnerWithThreshold(inner) => {
                    <addOwnerWithThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::approveHash(inner) => {
                    <approveHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::approvedHashes(inner) => {
                    <approvedHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::changeThreshold(inner) => {
                    <changeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkNSignatures(inner) => {
                    <checkNSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableModule(inner) => {
                    <disableModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::enableModule(inner) => {
                    <enableModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::encodeTransactionData(inner) => {
                    <encodeTransactionDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransaction(inner) => {
                    <execTransactionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getChainId(inner) => {
                    <getChainIdCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getModulesPaginated(inner) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOwners(inner) => {
                    <getOwnersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getStorageAt(inner) => {
                    <getStorageAtCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getThreshold(inner) => {
                    <getThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTransactionHash(inner) => {
                    <getTransactionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isModuleEnabled(inner) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOwner(inner) => {
                    <isOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nonce(inner) => {
                    <nonceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::removeOwner(inner) => {
                    <removeOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setFallbackHandler(inner) => {
                    <setFallbackHandlerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setGuard(inner) => {
                    <setGuardCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::setup(inner) => {
                    <setupCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::signedMessages(inner) => {
                    <signedMessagesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::simulateAndRevert(inner) => {
                    <simulateAndRevertCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swapOwner(inner) => {
                    <swapOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::VERSION(inner) => {
                    <VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::addOwnerWithThreshold(inner) => {
                    <addOwnerWithThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approveHash(inner) => {
                    <approveHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approvedHashes(inner) => {
                    <approvedHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::changeThreshold(inner) => {
                    <changeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkNSignatures(inner) => {
                    <checkNSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableModule(inner) => {
                    <disableModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::enableModule(inner) => {
                    <enableModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::encodeTransactionData(inner) => {
                    <encodeTransactionDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransaction(inner) => {
                    <execTransactionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModule(inner) => {
                    <execTransactionFromModuleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execTransactionFromModuleReturnData(inner) => {
                    <execTransactionFromModuleReturnDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getChainId(inner) => {
                    <getChainIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getModulesPaginated(inner) => {
                    <getModulesPaginatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOwners(inner) => {
                    <getOwnersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStorageAt(inner) => {
                    <getStorageAtCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getThreshold(inner) => {
                    <getThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTransactionHash(inner) => {
                    <getTransactionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isModuleEnabled(inner) => {
                    <isModuleEnabledCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOwner(inner) => {
                    <isOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nonce(inner) => {
                    <nonceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::removeOwner(inner) => {
                    <removeOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setFallbackHandler(inner) => {
                    <setFallbackHandlerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setGuard(inner) => {
                    <setGuardCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setup(inner) => {
                    <setupCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::signedMessages(inner) => {
                    <signedMessagesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::simulateAndRevert(inner) => {
                    <simulateAndRevertCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swapOwner(inner) => {
                    <swapOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Safe`](self) events.
    pub enum SafeEvents {
        AddedOwner(AddedOwner),
        ApproveHash(ApproveHash),
        ChangedFallbackHandler(ChangedFallbackHandler),
        ChangedGuard(ChangedGuard),
        ChangedThreshold(ChangedThreshold),
        DisabledModule(DisabledModule),
        EnabledModule(EnabledModule),
        ExecutionFailure(ExecutionFailure),
        ExecutionFromModuleFailure(ExecutionFromModuleFailure),
        ExecutionFromModuleSuccess(ExecutionFromModuleSuccess),
        ExecutionSuccess(ExecutionSuccess),
        RemovedOwner(RemovedOwner),
        SafeReceived(SafeReceived),
        SafeSetup(SafeSetup),
        SignMsg(SignMsg),
    }
    #[automatically_derived]
    impl SafeEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                17u8, 81u8, 17u8, 105u8, 20u8, 81u8, 91u8, 192u8, 137u8, 31u8, 249u8, 4u8, 122u8,
                108u8, 179u8, 44u8, 249u8, 2u8, 84u8, 111u8, 131u8, 6u8, 100u8, 153u8, 188u8,
                248u8, 186u8, 51u8, 210u8, 53u8, 63u8, 162u8,
            ],
            [
                20u8, 29u8, 248u8, 104u8, 166u8, 51u8, 26u8, 245u8, 40u8, 227u8, 140u8, 131u8,
                183u8, 170u8, 3u8, 237u8, 193u8, 155u8, 230u8, 110u8, 55u8, 174u8, 103u8, 249u8,
                40u8, 91u8, 244u8, 248u8, 227u8, 198u8, 161u8, 168u8,
            ],
            [
                35u8, 66u8, 139u8, 24u8, 172u8, 251u8, 62u8, 166u8, 75u8, 8u8, 220u8, 12u8, 29u8,
                41u8, 110u8, 169u8, 192u8, 151u8, 2u8, 192u8, 144u8, 131u8, 202u8, 82u8, 114u8,
                230u8, 77u8, 17u8, 91u8, 104u8, 125u8, 35u8,
            ],
            [
                61u8, 12u8, 233u8, 191u8, 195u8, 237u8, 125u8, 104u8, 98u8, 219u8, 178u8, 139u8,
                45u8, 234u8, 148u8, 86u8, 31u8, 231u8, 20u8, 161u8, 180u8, 208u8, 25u8, 170u8,
                138u8, 243u8, 151u8, 48u8, 209u8, 173u8, 124u8, 61u8,
            ],
            [
                68u8, 46u8, 113u8, 95u8, 98u8, 99u8, 70u8, 232u8, 197u8, 67u8, 129u8, 0u8, 45u8,
                166u8, 20u8, 246u8, 43u8, 238u8, 141u8, 39u8, 56u8, 101u8, 53u8, 178u8, 82u8, 30u8,
                200u8, 84u8, 8u8, 152u8, 85u8, 110u8,
            ],
            [
                90u8, 198u8, 196u8, 108u8, 147u8, 200u8, 208u8, 229u8, 55u8, 20u8, 186u8, 59u8,
                83u8, 219u8, 62u8, 124u8, 4u8, 109u8, 169u8, 148u8, 49u8, 61u8, 126u8, 208u8,
                209u8, 146u8, 2u8, 139u8, 199u8, 194u8, 40u8, 176u8,
            ],
            [
                97u8, 15u8, 127u8, 242u8, 179u8, 4u8, 174u8, 137u8, 3u8, 195u8, 222u8, 116u8,
                198u8, 12u8, 106u8, 177u8, 247u8, 214u8, 34u8, 107u8, 63u8, 82u8, 197u8, 22u8,
                25u8, 5u8, 187u8, 90u8, 212u8, 3u8, 156u8, 147u8,
            ],
            [
                104u8, 149u8, 193u8, 54u8, 100u8, 170u8, 79u8, 103u8, 40u8, 139u8, 37u8, 215u8,
                162u8, 29u8, 122u8, 170u8, 52u8, 145u8, 110u8, 53u8, 95u8, 185u8, 182u8, 250u8,
                224u8, 161u8, 57u8, 169u8, 8u8, 91u8, 236u8, 184u8,
            ],
            [
                148u8, 101u8, 250u8, 12u8, 150u8, 44u8, 199u8, 105u8, 88u8, 230u8, 55u8, 58u8,
                153u8, 51u8, 38u8, 64u8, 12u8, 28u8, 148u8, 248u8, 190u8, 47u8, 227u8, 169u8, 82u8,
                173u8, 250u8, 127u8, 96u8, 178u8, 234u8, 38u8,
            ],
            [
                170u8, 180u8, 250u8, 43u8, 70u8, 63u8, 88u8, 27u8, 43u8, 50u8, 203u8, 59u8, 126u8,
                59u8, 112u8, 75u8, 156u8, 227u8, 124u8, 194u8, 9u8, 181u8, 251u8, 77u8, 119u8,
                229u8, 147u8, 172u8, 228u8, 5u8, 66u8, 118u8,
            ],
            [
                172u8, 210u8, 200u8, 112u8, 40u8, 4u8, 18u8, 143u8, 219u8, 13u8, 178u8, 187u8,
                73u8, 246u8, 209u8, 39u8, 221u8, 1u8, 129u8, 193u8, 63u8, 212u8, 93u8, 191u8,
                225u8, 109u8, 224u8, 147u8, 14u8, 43u8, 211u8, 117u8,
            ],
            [
                231u8, 244u8, 103u8, 80u8, 56u8, 244u8, 246u8, 3u8, 77u8, 252u8, 187u8, 178u8,
                76u8, 77u8, 192u8, 142u8, 78u8, 191u8, 16u8, 235u8, 157u8, 37u8, 125u8, 61u8, 2u8,
                192u8, 243u8, 141u8, 18u8, 42u8, 198u8, 228u8,
            ],
            [
                236u8, 223u8, 58u8, 62u8, 255u8, 234u8, 87u8, 131u8, 163u8, 196u8, 194u8, 20u8,
                14u8, 103u8, 117u8, 119u8, 102u8, 100u8, 40u8, 212u8, 78u8, 217u8, 212u8, 116u8,
                160u8, 179u8, 164u8, 201u8, 148u8, 63u8, 132u8, 64u8,
            ],
            [
                242u8, 160u8, 235u8, 21u8, 100u8, 114u8, 209u8, 68u8, 2u8, 85u8, 176u8, 215u8,
                193u8, 225u8, 156u8, 192u8, 113u8, 21u8, 209u8, 5u8, 31u8, 230u8, 5u8, 176u8,
                220u8, 230u8, 154u8, 207u8, 236u8, 136u8, 77u8, 156u8,
            ],
            [
                248u8, 212u8, 159u8, 197u8, 41u8, 129u8, 46u8, 154u8, 124u8, 92u8, 80u8, 230u8,
                156u8, 32u8, 240u8, 220u8, 204u8, 13u8, 184u8, 250u8, 149u8, 201u8, 139u8, 197u8,
                140u8, 201u8, 164u8, 241u8, 193u8, 41u8, 158u8, 175u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for SafeEvents {
        const NAME: &'static str = "SafeEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AddedOwner as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AddedOwner as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AddedOwner)
                }
                Some(<ApproveHash as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ApproveHash as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ApproveHash)
                }
                Some(<ChangedFallbackHandler as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChangedFallbackHandler as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChangedFallbackHandler)
                }
                Some(<ChangedGuard as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChangedGuard as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChangedGuard)
                }
                Some(<ChangedThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChangedThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ChangedThreshold)
                }
                Some(<DisabledModule as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <DisabledModule as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::DisabledModule)
                }
                Some(<EnabledModule as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EnabledModule as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EnabledModule)
                }
                Some(<ExecutionFailure as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionFailure as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ExecutionFailure)
                }
                Some(<ExecutionFromModuleFailure as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionFromModuleFailure as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ExecutionFromModuleFailure)
                }
                Some(<ExecutionFromModuleSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionFromModuleSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ExecutionFromModuleSuccess)
                }
                Some(<ExecutionSuccess as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExecutionSuccess as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ExecutionSuccess)
                }
                Some(<RemovedOwner as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RemovedOwner as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RemovedOwner)
                }
                Some(<SafeReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SafeReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::SafeReceived)
                }
                Some(<SafeSetup as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SafeSetup as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::SafeSetup)
                }
                Some(<SignMsg as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SignMsg as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::SignMsg)
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
    impl alloy_sol_types::private::IntoLogData for SafeEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddedOwner(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ApproveHash(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChangedFallbackHandler(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChangedGuard(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChangedThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DisabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EnabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionFromModuleFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionFromModuleSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExecutionSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RemovedOwner(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SafeReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SafeSetup(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::SignMsg(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddedOwner(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ApproveHash(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChangedFallbackHandler(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChangedGuard(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChangedThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DisabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EnabledModule(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionFromModuleFailure(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionFromModuleSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExecutionSuccess(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RemovedOwner(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SafeReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SafeSetup(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SignMsg(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Safe`](self) contract instance.

    See the [wrapper's documentation](`SafeInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> SafeInstance<T, P, N> {
        SafeInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<SafeInstance<T, P, N>>> {
        SafeInstance::<T, P, N>::deploy(provider)
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
        SafeInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Safe`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`Safe`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct SafeInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for SafeInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("SafeInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SafeInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`Safe`](self) contract instance.

        See the [wrapper's documentation](`SafeInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self { address, provider, _network_transport: ::core::marker::PhantomData }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(provider: P) -> alloy_contract::Result<SafeInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> SafeInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> SafeInstance<T, P, N> {
            SafeInstance {
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
        > SafeInstance<T, P, N>
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
        ///Creates a new call builder for the [`VERSION`] function.
        pub fn VERSION(&self) -> alloy_contract::SolCallBuilder<T, &P, VERSIONCall, N> {
            self.call_builder(&VERSIONCall {})
        }
        ///Creates a new call builder for the [`addOwnerWithThreshold`] function.
        pub fn addOwnerWithThreshold(
            &self,
            owner: alloy::sol_types::private::Address,
            _threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, addOwnerWithThresholdCall, N> {
            self.call_builder(&addOwnerWithThresholdCall { owner, _threshold })
        }
        ///Creates a new call builder for the [`approveHash`] function.
        pub fn approveHash(
            &self,
            hashToApprove: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveHashCall, N> {
            self.call_builder(&approveHashCall { hashToApprove })
        }
        ///Creates a new call builder for the [`approvedHashes`] function.
        pub fn approvedHashes(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, approvedHashesCall, N> {
            self.call_builder(&approvedHashesCall { _0, _1 })
        }
        ///Creates a new call builder for the [`changeThreshold`] function.
        pub fn changeThreshold(
            &self,
            _threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, changeThresholdCall, N> {
            self.call_builder(&changeThresholdCall { _threshold })
        }
        ///Creates a new call builder for the [`checkNSignatures`] function.
        pub fn checkNSignatures(
            &self,
            dataHash: alloy::sol_types::private::FixedBytes<32>,
            data: alloy::sol_types::private::Bytes,
            signatures: alloy::sol_types::private::Bytes,
            requiredSignatures: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkNSignaturesCall, N> {
            self.call_builder(&checkNSignaturesCall {
                dataHash,
                data,
                signatures,
                requiredSignatures,
            })
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            dataHash: alloy::sol_types::private::FixedBytes<32>,
            data: alloy::sol_types::private::Bytes,
            signatures: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall { dataHash, data, signatures })
        }
        ///Creates a new call builder for the [`disableModule`] function.
        pub fn disableModule(
            &self,
            prevModule: alloy::sol_types::private::Address,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, disableModuleCall, N> {
            self.call_builder(&disableModuleCall { prevModule, module })
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`enableModule`] function.
        pub fn enableModule(
            &self,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, enableModuleCall, N> {
            self.call_builder(&enableModuleCall { module })
        }
        ///Creates a new call builder for the [`encodeTransactionData`] function.
        pub fn encodeTransactionData(
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
            _nonce: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, encodeTransactionDataCall, N> {
            self.call_builder(&encodeTransactionDataCall {
                to,
                value,
                data,
                operation,
                safeTxGas,
                baseGas,
                gasPrice,
                gasToken,
                refundReceiver,
                _nonce,
            })
        }
        ///Creates a new call builder for the [`execTransaction`] function.
        pub fn execTransaction(
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
        ) -> alloy_contract::SolCallBuilder<T, &P, execTransactionCall, N> {
            self.call_builder(&execTransactionCall {
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
            })
        }
        ///Creates a new call builder for the [`execTransactionFromModule`] function.
        pub fn execTransactionFromModule(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, execTransactionFromModuleCall, N> {
            self.call_builder(&execTransactionFromModuleCall { to, value, data, operation })
        }
        ///Creates a new call builder for the [`execTransactionFromModuleReturnData`] function.
        pub fn execTransactionFromModuleReturnData(
            &self,
            to: alloy::sol_types::private::Address,
            value: alloy::sol_types::private::primitives::aliases::U256,
            data: alloy::sol_types::private::Bytes,
            operation: <Enum::Operation as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, execTransactionFromModuleReturnDataCall, N>
        {
            self.call_builder(&execTransactionFromModuleReturnDataCall {
                to,
                value,
                data,
                operation,
            })
        }
        ///Creates a new call builder for the [`getChainId`] function.
        pub fn getChainId(&self) -> alloy_contract::SolCallBuilder<T, &P, getChainIdCall, N> {
            self.call_builder(&getChainIdCall {})
        }
        ///Creates a new call builder for the [`getModulesPaginated`] function.
        pub fn getModulesPaginated(
            &self,
            start: alloy::sol_types::private::Address,
            pageSize: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getModulesPaginatedCall, N> {
            self.call_builder(&getModulesPaginatedCall { start, pageSize })
        }
        ///Creates a new call builder for the [`getOwners`] function.
        pub fn getOwners(&self) -> alloy_contract::SolCallBuilder<T, &P, getOwnersCall, N> {
            self.call_builder(&getOwnersCall {})
        }
        ///Creates a new call builder for the [`getStorageAt`] function.
        pub fn getStorageAt(
            &self,
            offset: alloy::sol_types::private::primitives::aliases::U256,
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStorageAtCall, N> {
            self.call_builder(&getStorageAtCall { offset, length })
        }
        ///Creates a new call builder for the [`getThreshold`] function.
        pub fn getThreshold(&self) -> alloy_contract::SolCallBuilder<T, &P, getThresholdCall, N> {
            self.call_builder(&getThresholdCall {})
        }
        ///Creates a new call builder for the [`getTransactionHash`] function.
        pub fn getTransactionHash(
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
            _nonce: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTransactionHashCall, N> {
            self.call_builder(&getTransactionHashCall {
                to,
                value,
                data,
                operation,
                safeTxGas,
                baseGas,
                gasPrice,
                gasToken,
                refundReceiver,
                _nonce,
            })
        }
        ///Creates a new call builder for the [`isModuleEnabled`] function.
        pub fn isModuleEnabled(
            &self,
            module: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isModuleEnabledCall, N> {
            self.call_builder(&isModuleEnabledCall { module })
        }
        ///Creates a new call builder for the [`isOwner`] function.
        pub fn isOwner(
            &self,
            owner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOwnerCall, N> {
            self.call_builder(&isOwnerCall { owner })
        }
        ///Creates a new call builder for the [`nonce`] function.
        pub fn nonce(&self) -> alloy_contract::SolCallBuilder<T, &P, nonceCall, N> {
            self.call_builder(&nonceCall {})
        }
        ///Creates a new call builder for the [`removeOwner`] function.
        pub fn removeOwner(
            &self,
            prevOwner: alloy::sol_types::private::Address,
            owner: alloy::sol_types::private::Address,
            _threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeOwnerCall, N> {
            self.call_builder(&removeOwnerCall { prevOwner, owner, _threshold })
        }
        ///Creates a new call builder for the [`setFallbackHandler`] function.
        pub fn setFallbackHandler(
            &self,
            handler: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setFallbackHandlerCall, N> {
            self.call_builder(&setFallbackHandlerCall { handler })
        }
        ///Creates a new call builder for the [`setGuard`] function.
        pub fn setGuard(
            &self,
            guard: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setGuardCall, N> {
            self.call_builder(&setGuardCall { guard })
        }
        ///Creates a new call builder for the [`setup`] function.
        pub fn setup(
            &self,
            _owners: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            _threshold: alloy::sol_types::private::primitives::aliases::U256,
            to: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
            fallbackHandler: alloy::sol_types::private::Address,
            paymentToken: alloy::sol_types::private::Address,
            payment: alloy::sol_types::private::primitives::aliases::U256,
            paymentReceiver: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setupCall, N> {
            self.call_builder(&setupCall {
                _owners,
                _threshold,
                to,
                data,
                fallbackHandler,
                paymentToken,
                payment,
                paymentReceiver,
            })
        }
        ///Creates a new call builder for the [`signedMessages`] function.
        pub fn signedMessages(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, signedMessagesCall, N> {
            self.call_builder(&signedMessagesCall { _0 })
        }
        ///Creates a new call builder for the [`simulateAndRevert`] function.
        pub fn simulateAndRevert(
            &self,
            targetContract: alloy::sol_types::private::Address,
            calldataPayload: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, simulateAndRevertCall, N> {
            self.call_builder(&simulateAndRevertCall { targetContract, calldataPayload })
        }
        ///Creates a new call builder for the [`swapOwner`] function.
        pub fn swapOwner(
            &self,
            prevOwner: alloy::sol_types::private::Address,
            oldOwner: alloy::sol_types::private::Address,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapOwnerCall, N> {
            self.call_builder(&swapOwnerCall { prevOwner, oldOwner, newOwner })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > SafeInstance<T, P, N>
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
        ///Creates a new event filter for the [`AddedOwner`] event.
        pub fn AddedOwner_filter(&self) -> alloy_contract::Event<T, &P, AddedOwner, N> {
            self.event_filter::<AddedOwner>()
        }
        ///Creates a new event filter for the [`ApproveHash`] event.
        pub fn ApproveHash_filter(&self) -> alloy_contract::Event<T, &P, ApproveHash, N> {
            self.event_filter::<ApproveHash>()
        }
        ///Creates a new event filter for the [`ChangedFallbackHandler`] event.
        pub fn ChangedFallbackHandler_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ChangedFallbackHandler, N> {
            self.event_filter::<ChangedFallbackHandler>()
        }
        ///Creates a new event filter for the [`ChangedGuard`] event.
        pub fn ChangedGuard_filter(&self) -> alloy_contract::Event<T, &P, ChangedGuard, N> {
            self.event_filter::<ChangedGuard>()
        }
        ///Creates a new event filter for the [`ChangedThreshold`] event.
        pub fn ChangedThreshold_filter(&self) -> alloy_contract::Event<T, &P, ChangedThreshold, N> {
            self.event_filter::<ChangedThreshold>()
        }
        ///Creates a new event filter for the [`DisabledModule`] event.
        pub fn DisabledModule_filter(&self) -> alloy_contract::Event<T, &P, DisabledModule, N> {
            self.event_filter::<DisabledModule>()
        }
        ///Creates a new event filter for the [`EnabledModule`] event.
        pub fn EnabledModule_filter(&self) -> alloy_contract::Event<T, &P, EnabledModule, N> {
            self.event_filter::<EnabledModule>()
        }
        ///Creates a new event filter for the [`ExecutionFailure`] event.
        pub fn ExecutionFailure_filter(&self) -> alloy_contract::Event<T, &P, ExecutionFailure, N> {
            self.event_filter::<ExecutionFailure>()
        }
        ///Creates a new event filter for the [`ExecutionFromModuleFailure`] event.
        pub fn ExecutionFromModuleFailure_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ExecutionFromModuleFailure, N> {
            self.event_filter::<ExecutionFromModuleFailure>()
        }
        ///Creates a new event filter for the [`ExecutionFromModuleSuccess`] event.
        pub fn ExecutionFromModuleSuccess_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ExecutionFromModuleSuccess, N> {
            self.event_filter::<ExecutionFromModuleSuccess>()
        }
        ///Creates a new event filter for the [`ExecutionSuccess`] event.
        pub fn ExecutionSuccess_filter(&self) -> alloy_contract::Event<T, &P, ExecutionSuccess, N> {
            self.event_filter::<ExecutionSuccess>()
        }
        ///Creates a new event filter for the [`RemovedOwner`] event.
        pub fn RemovedOwner_filter(&self) -> alloy_contract::Event<T, &P, RemovedOwner, N> {
            self.event_filter::<RemovedOwner>()
        }
        ///Creates a new event filter for the [`SafeReceived`] event.
        pub fn SafeReceived_filter(&self) -> alloy_contract::Event<T, &P, SafeReceived, N> {
            self.event_filter::<SafeReceived>()
        }
        ///Creates a new event filter for the [`SafeSetup`] event.
        pub fn SafeSetup_filter(&self) -> alloy_contract::Event<T, &P, SafeSetup, N> {
            self.event_filter::<SafeSetup>()
        }
        ///Creates a new event filter for the [`SignMsg`] event.
        pub fn SignMsg_filter(&self) -> alloy_contract::Event<T, &P, SignMsg, N> {
            self.event_filter::<SignMsg>()
        }
    }
}
