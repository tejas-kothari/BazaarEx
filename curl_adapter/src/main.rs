#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

module_manifest!();

fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
pub fn curl_request(curl_cmd: String) -> String {
    let response = curl(vec![curl_cmd]);
    String::from_utf8(response.stdout).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
