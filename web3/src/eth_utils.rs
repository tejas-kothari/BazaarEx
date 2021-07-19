use crate::fce_results::JsonRpcResult;
use sha3::{Digest, Keccak256};
use std::sync::atomic::{AtomicUsize, Ordering};

pub const BLOCK_NUMBER_TAGS: [&'static str; 3] = ["latest", "earliest", "pending"];
pub static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn get_nonce() -> u64 {
    NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}

pub fn check_response_string(response: String, id: &u64, result_is_hex: bool) -> JsonRpcResult {
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"$V\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = err_msg.replace("$ID", &id.to_string());
        return JsonRpcResult::from_res(Result::from(Err(err_msg)), result_is_hex);
    }

    match response.contains("error") {
        true => JsonRpcResult::from_res(Result::from(Err(response)), result_is_hex),
        false => JsonRpcResult::from_res(Result::from(Ok(response)), result_is_hex),
    }
}

pub fn wei_to_eth(amount: &u128) -> f64 {
    *amount as f64 / (1_000_000_000.0 * 1_000_000_000.0)
}

pub fn pk_to_add(pk_string: String) -> String {
    let pk_bytes = hex::decode(pk_string.clone()).unwrap();

    let mut hasher = Keccak256::new();
    hasher.update(pk_bytes.clone());
    let pk_hash = hex::encode(hasher.finalize());
    let add = (&pk_hash[24..]).to_string();
    let mut add_string = "0x".to_string();
    add_string.push_str(&add);

    add_string
}
