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
    struct TriggerInputData {
        address lmsrMarketMaker;
        address conditionalTokens;
        bool result;
    }

    #[derive(Debug)]
    struct AvsOutputData {
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

                let data = <TriggerInputData as SolValue>::abi_decode(&data, true)
                    .map_err(|e| format!("Failed to decode trigger data: {}", e))?;

                Ok(AvsOutputData {
                    lmsrMarketMaker: data.lmsrMarketMaker,
                    conditionalTokens: data.conditionalTokens,
                    result: data.result,
                }
                .abi_encode())
            }
            _ => Err("Unsupported trigger data".to_string()),
        }
    }
}

export!(Component with_types_in bindings);
