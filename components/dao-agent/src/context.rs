use crate::models::{Contract, DaoContext, TokenBalance};

// TODO make it so we don't have to hardcode these, fetch them from an IPFS node
impl Default for DaoContext {
    fn default() -> Self {
        Self {
            safe_address: "0x742d35Cc6634C0532925a3b844Bc454e4438f44e".to_string(),
            eth_balance: TokenBalance {
                token_address: "0x0000000000000000000000000000000000000000".to_string(),
                symbol: "ETH".to_string(),
                balance: "100000000000000000000".to_string(), // 100 ETH in wei
                decimals: 18,
            },
            token_balances: vec![
                TokenBalance {
                    token_address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                    symbol: "USDC".to_string(),
                    balance: "1000000000".to_string(), // 1000 USDC (6 decimals)
                    decimals: 6,
                },
            ],
            allowed_addresses: vec![
                "0xDf3679681B87fAE75CE185e4f01d98b64Ddb64a3".to_string(),
            ],
            dao_description: "A DAO focused on funding public goods and environmental causes".to_string(),
            contracts: vec![
                Contract {
                    name: "USDC".to_string(),
                    address: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
                    abi: r#"[{
                        "constant": false,
                        "inputs": [{"name": "_to","type": "address"},{"name": "_value","type": "uint256"}],
                        "name": "transfer",
                        "outputs": [{"name": "","type": "bool"}],
                        "type": "function"
                    }]"#.to_string(),
                },
            ],
        }
    }
}

impl DaoContext {
    /// Check if the context has sufficient ETH balance for a transaction
    pub fn has_sufficient_eth(&self, amount_wei: &str) -> bool {
        let amount = amount_wei.parse::<u128>().unwrap_or(0);
        let balance = self.eth_balance.balance.parse::<u128>().unwrap_or(0);
        amount <= balance
    }

    /// Format balances for display in the prompt
    pub fn format_balances(&self) -> String {
        let mut result =
            vec![format!("ETH: {} ({})", self.eth_balance.balance, self.eth_balance.symbol)];

        for balance in &self.token_balances {
            result.push(format!(
                "{}: {} ({})",
                balance.symbol, balance.balance, balance.token_address
            ));
        }

        result.join("\n")
    }
}
