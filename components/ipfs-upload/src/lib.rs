mod bindings;

use alloy_sol_macro::sol;
use alloy_sol_types::SolValue;
use anyhow::Result;
use bindings::{
    export,
    wavs::worker::layer_types::{TriggerData, TriggerDataEthContractEvent},
    Guest, TriggerAction,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use wavs_wasi_chain::http::{fetch_bytes, http_request_get};
use wstd::http::{body::IncomingBody, Body, Client, IntoBody, Request};
use wstd::io::empty;
use wstd::runtime::block_on;

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        match trigger_action.data {
            TriggerData::EthContractEvent(TriggerDataEthContractEvent { log, .. }) => {
                let event: Upload =
                    wavs_wasi_chain::decode_event_log_data!(log).map_err(|e| e.to_string())?;

                let res = block_on(async move {
                    let file_path = download_file(&event.data_uri, &event.file_name).await?;

                    let res = upload_to_ipfs(&file_path, &event.ipfs_url, &event.api_key).await?;

                    Ok(res)
                });

                match res {
                    Ok(uploaded_file_path) => {
                        let message = Response { uri: uploaded_file_path };
                        Ok(message.abi_encode())
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            TriggerData::CosmosContractEvent(_) => {
                Err("expected eth event, got cosmos".to_string())
            }
            TriggerData::Raw(_) => Err("expected eth event, got raw".to_string()),
        }
    }
}

/// downloads a file from a given URL and saves it to the specified local path
pub async fn download_file(url: &str, file_name: &str) -> Result<String> {
    let request = http_request_get(url)?;
    let file_bytes = fetch_bytes(request).await?;

    let full_path = format!("/tmp/{}", file_name);
    let path = Path::new(&full_path);

    let mut file = File::create(path)?;
    file.write_all(&file_bytes)?;

    println!("File downloaded successfully to {}", full_path);
    Ok(full_path)
}

/// Uploads a file using multipart/form-data to IPFS
pub async fn upload_to_ipfs(
    file_path: &str,
    ipfs_url: &str,
    api_key: &str,
) -> Result<IncomingBody> {
    let mut file = File::open(file_path)?;
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)?;

    // define multipart request boundary
    let boundary = "----RustBoundary";

    // construct the body
    let body = format!(
        "--{}\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n\
        Content-Type: application/octet-stream\r\n\r\n",
        boundary, file_path
    );

    let mut request_body = body.into_bytes();
    request_body.extend_from_slice(&file_bytes);
    request_body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

    let request = Request::post(ipfs_url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Content-Type", &format!("multipart/form-data; boundary={}", boundary))
        .body(request_body.into_body())?;

    let response = wstd::http::Client::new().send(request).await?;

    if response.status().is_success() {
        println!("File successfully uploaded to IPFS: {:?}", response);
        Ok(response)
    } else {
        Err(anyhow::anyhow!("Failed to upload to IPFS. Status: {:?}", response.status()))
    }
}

sol! {
    event Upload(string data_uri, string file_name, string ipfs_url, string api_key);

    struct Response {
        string uri;
    }
}

export!(Component with_types_in bindings);
