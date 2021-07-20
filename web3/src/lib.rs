use marine_rs_sdk::marine;
use marine_rs_sdk::MountedBinaryResult;

mod bytes_type;
pub mod eth_calls;
pub mod eth_utils;
mod fce_results;
mod jsonrpc_helpers;

pub fn curl_request_res(curl_cmd: Vec<String>) -> Result<String, std::string::FromUtf8Error> {
    println!("curl cmd: {:?}", curl_cmd);
    let response = curl_request(curl_cmd);
    let res = String::from_utf8(response.stdout)?;
    Ok(res)
}

#[marine]
#[link(wasm_import_module = "curl_adapter")]
extern "C" {
    pub fn curl_request(curl_cmd: Vec<String>) -> MountedBinaryResult;
}
