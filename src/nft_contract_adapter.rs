use ethabi::{Function, Param, ParamType, StateMutability, Token};
use ethereum_types::{H160, H256, U256};
use serde_json::Value;
use std::str::FromStr;
use web3::eth_calls::{eth_call, eth_getTransactionReceipt, eth_sendTransaction, TxCall};

const CON_OWNER: &str = "0x8f7eF7aC4eE253aE02319018ea7c7F1aBd1320F6";
const CON_ADD: &str = "0xb251d7EDF1A503643F8617593179aC547D4d9dc2";
const URL: &str = "http://127.0.0.1:9545/";

pub fn get_name() -> String {
    let output = Param {
        name: "name".to_string(),
        kind: ParamType::String,
    };

    let func = Function {
        name: "name".to_string(),
        inputs: vec![],
        outputs: vec![output],
        constant: false,
        state_mutability: StateMutability::View,
    };

    let params = TxCall {
        to: Some(H160::from_str(CON_ADD).unwrap()),
        data: Some(func.encode_input(&[]).unwrap().into()),
        ..Default::default()
    };

    let res_bytes = eth_call(URL.to_string(), params, "latest".into()).result;
    let res_decoded = func.decode_output(&res_bytes).unwrap();
    for res in res_decoded {
        return res.into_string().unwrap();
    }

    "".to_string()
}

pub fn mint(to_add: String) -> i64 {
    let input = Param {
        name: "to".to_string(),
        kind: ParamType::Address,
    };

    let func = Function {
        name: "mint".to_string(),
        inputs: vec![input],
        outputs: vec![],
        constant: false,
        state_mutability: StateMutability::NonPayable,
    };

    let to = H160::from_str(&to_add).unwrap();

    let params = TxCall {
        gas: Some(U256::from_dec_str("4600000").unwrap()),
        gas_price: Some(U256::from_dec_str("20000000000").unwrap()),
        from: Some(H160::from_str(CON_OWNER).unwrap()),
        to: Some(H160::from_str(CON_ADD).unwrap()),
        data: Some(func.encode_input(&[Token::Address(to)]).unwrap().into()),
        ..Default::default()
    };

    let res_bytes = eth_sendTransaction(URL.to_string(), params).result;
    let res_hash = H256::from_slice(&res_bytes);

    let rep_u8 = eth_getTransactionReceipt(URL.to_string(), res_hash).result;
    let rep_string = std::str::from_utf8(&rep_u8).unwrap();
    let rep: Value = serde_json::from_str(rep_string).unwrap();

    let mut token_id_hex_string: String =
        serde_json::from_value(rep["logs"][0]["topics"][3].clone()).unwrap();
    token_id_hex_string = (&token_id_hex_string[2..]).to_string();
    let token_id = i64::from_str_radix(&token_id_hex_string.clone(), 16).unwrap();

    token_id
}
