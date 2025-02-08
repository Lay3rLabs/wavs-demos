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
    event ApproveHash(bytes32 indexed approvedHash, address indexed owner);

    #[derive(Debug)]
    struct ValidationPayload {
        bytes32 approvedHash;
        bool approved;
    }
}

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        match trigger_action.data {
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                // Decode event
                let ApproveHash { approvedHash, .. } = layer_wasi::ethereum::decode_event_log_data(
                    layer_wasi::bindings::compat::EthEventLogData {
                        topics: log.topics,
                        data: log.data,
                    },
                )
                .map_err(|e| format!("Failed to decode event log data: {}", e))?;

                // Return true. Normally you would like to have some other logic in the component
                // to decide if the transaction should be approved or not.
                Ok(ValidationPayload { approvedHash, approved: true }.abi_encode())
            }
            _ => Err("Unsupported trigger data".to_string()),
        }
    }
}

export!(Component with_types_in bindings);
