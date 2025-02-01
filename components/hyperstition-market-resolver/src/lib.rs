#[allow(warnings)]
mod bindings;
use alloy_sol_types::{sol, SolValue};
use bindings::{
    export,
    lay3r::avs::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};

sol! {
    #[derive(Debug)]
    event ResolveMarket(
        uint64 indexed triggerId,
        address indexed creator,
        bytes data
    );

    #[derive(Debug)]
    struct TriggerInfoData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    #[derive(Debug)]
    struct ReturnData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }
}

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        match trigger_action.data {
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                let ResolveMarket { data, .. } = layer_wasi::ethereum::decode_event_log_data(
                    layer_wasi::bindings::compat::EthEventLogData {
                        topics: log.topics,
                        data: log.data,
                    },
                )
                .map_err(|e| format!("Failed to decode event log data: {}", e))?;

                let data = <TriggerInfoData as SolValue>::abi_decode(&data, true)
                    .map_err(|e| format!("Failed to decode trigger data: {}", e))?;

                Ok(ReturnData {
                    lmsrMarketMaker: data.lmsrMarketMaker,
                    conditionalTokens: data.conditionalTokens,
                    result: data.result,
                }
                .abi_encode())

                // let prompt = trigger_info.prompt;
                // let creator = trigger_info.creator;

                // block_on(|reactor| async move {
                //     // Query Ollama
                //     let response = query_ollama(&reactor, &prompt).await?;

                //     // Create NFT metadata
                //     let metadata = NFTMetadata {
                //         name: "AI Generated NFT".to_string(),
                //         description: response,
                //         image: "ipfs://placeholder".to_string(),
                //         attributes: vec![Attribute {
                //             trait_type: "Prompt".to_string(),
                //             value: prompt,
                //         }],
                //     };

                //     // Serialize to JSON and convert to data URI
                //     let json = serde_json::to_string(&metadata)
                //         .map_err(|e| format!("JSON serialization error: {}", e))?;
                //     let data_uri = format!(
                //         "data:application/json;base64,{}",
                //         base64::engine::general_purpose::STANDARD.encode(json)
                //     );

                //     // Create the return data
                //     let return_data = ReturnData {
                //         creator,
                //         triggerId: trigger_info.triggerId,
                //         dataUri: data_uri,
                //     };

                //     // Encode using abi.encode format
                //     Ok(return_data.abi_encode())
                // })
            }
            _ => Err("Unsupported trigger data".to_string()),
        }
    }
}

export!(Component with_types_in bindings);
