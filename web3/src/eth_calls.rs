use crate::bytes_type::Bytes;
use crate::curl_request_res;
use crate::eth_utils::{check_response_string, get_nonce};
use crate::fce_results::JsonRpcResult;
use crate::jsonrpc_helpers::Request;
use ethereum_types::{H160, H256, U256};
use jsonrpc_core as rpc;
use serde::Serialize;
use serde_json::json;

pub fn serialize<T: serde::Serialize>(t: &T) -> rpc::Value {
    serde_json::to_value(t).expect("Types never fail to serialize.")
}

#[derive(Default, Serialize)]
pub struct TxCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}

pub fn eth_call(url: String, tx: TxCall, tag: String) -> JsonRpcResult {
    let method = "eth_call".to_string();

    let tx_call_serial = serialize(&tx);
    let tag_serial = serialize(&tag);
    let params: rpc::Value = json!(vec![tx_call_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id, true)
}

pub fn eth_sendTransaction(url: String, tx: TxCall) -> JsonRpcResult {
    let method = "eth_sendTransaction".to_string();

    let tx_call_serial = serialize(&tx);
    let params: rpc::Value = json!(vec![tx_call_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id, true)
}

pub fn eth_getTransactionReceipt(url: String, trans_hash: H256) -> JsonRpcResult {
    let method = "eth_getTransactionReceipt".to_string();

    let trans_hash_serial = serialize(&trans_hash);
    let params: rpc::Value = json!(vec![trans_hash_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id, false)
}

pub fn eth_sendRawTransaction(url: String, signedTx: String) -> JsonRpcResult {
    let method = "eth_sendRawTransaction".to_string();

    let signed_tx_serial = serialize(&signedTx);
    let params: rpc::Value = json!(vec![signed_tx_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id, true)
}

pub fn eth_getBalance(url: String, add: String) -> JsonRpcResult {
    let method = "eth_getBalance".to_string();

    let add_serial = serialize(&add);
    let tag_serial = serialize(&"latest".to_string());
    let params: rpc::Value = json!(vec![add_serial, tag_serial]);

    let id = get_nonce();

    let curl_args = Request::new(method, params, id).as_sys_string(&url);
    let response = curl_request_res(curl_args).unwrap();

    check_response_string(response, &id, true)
}
