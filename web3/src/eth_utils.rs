use crate::fce_results::JsonRpcResult;
use std::sync::atomic::{AtomicUsize, Ordering};
use tiny_keccak::{Hasher, Keccak};

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
    let pk_bytes = hex::decode(pk_string[2..].to_string()).unwrap();
    let mut hasher = Keccak::v256();
    hasher.update(&pk_bytes);
    let mut add_bytes: [u8; 32] = Default::default();
    hasher.finalize(&mut add_bytes);
    let add_string_suf = hex::encode(&add_bytes[12..]);
    let mut add_string = "0x".to_string();
    add_string.push_str(&add_string_suf);
    add_string
}
