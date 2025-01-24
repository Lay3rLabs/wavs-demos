use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractCall {
    pub function: String,
    pub args: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafeTransaction {
    pub to: String,
    pub value: String, // Using string to handle large numbers safely
    #[serde(default = "default_contract_call")]
    pub contract_call: Option<ContractCall>, // JSON representation of the call to encode
    pub data: String,  // Will be populated after encoding
    pub description: String, // LLM's explanation of the transaction
}

pub fn default_contract_call() -> Option<ContractCall> {
    None
}

/// Represents a smart contract that the DAO can interact with
#[derive(Debug, Clone)]
pub struct Contract {
    pub name: String,
    pub address: String,
    pub abi: String, // JSON ABI string
}

/// Represents a token balance
#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub token_address: String,
    pub symbol: String,
    pub balance: String,
    pub decimals: u8,
}

/// Context for the DAO agent's decision making
#[derive(Debug)]
pub struct DaoContext {
    pub safe_address: String,
    pub eth_balance: TokenBalance,
    pub token_balances: Vec<TokenBalance>,
    pub allowed_addresses: Vec<String>,
    pub dao_description: String,
    pub contracts: Vec<Contract>,
}
