mod bindings;

use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use anyhow::Result;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use std::{fs::File, io::Write, path::Path};
use wavs_wasi_chain::http::{fetch_bytes, http_request_get};
use wstd::runtime::block_on;

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        match trigger_action.data {
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                let event: Download =
                    wavs_wasi_chain::decode_event_log_data!(log).map_err(|e| e.to_string())?;

                let res = block_on(async move {
                    match download_file(&event.from, &event.file_name).await {
                        Ok(resp) => Ok(()),
                        Err(e) => Err(e),
                    }
                });

                let message = Response { downloaded: true };
                Ok(message.abi_encode())
            }
            TriggerData::CosmosContractEvent(_) => {
                Err("expected eth event, got cosmos".to_string())
            }
            TriggerData::Raw(_) => Err("expected eth event, got raw".to_string()),
        }
    }
}

/// downloads a file from a given URL and saves it to the specified local path.
pub async fn download_file(url: &str, output_path: &str) -> Result<()> {
    let request = http_request_get(url)?;

    let file_bytes = fetch_bytes(request).await?;

    let path = Path::new(output_path);
    let mut file = File::create(path)?;
    file.write_all(&file_bytes)?;

    println!("File downloaded successfully to {}", output_path);
    Ok(())
}

sol! {
    event Download(string from, string file_name);

    struct Response {
        bool downloaded;
    }
}

export!(Component with_types_in bindings);
