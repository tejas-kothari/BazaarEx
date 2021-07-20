pub use ethereum_types::{H160, H256, U256};
use libsecp256k1::{sign, Message, SecretKey};
use num_traits::int;
use rlp::RlpStream;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

/// Description of a Transaction, pending or in the chain.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct RawTransaction {
    /// Nonce
    pub nonce: U256,
    /// Recipient (None when contract creation)
    pub to: Option<H160>,
    /// Transfered value
    pub value: U256,
    /// Gas Price
    #[serde(rename = "gasPrice")]
    pub gas_price: U256,
    /// Gas amount
    pub gas: U256,
    /// Input data
    pub data: Vec<u8>,
}

impl RawTransaction {
    /// Signs and returns the RLP-encoded transaction
    pub fn sign<T: int::PrimInt>(&self, private_key: &H256, chain_id: &T) -> Vec<u8> {
        let chain_id_u64: u64 = chain_id.to_u64().unwrap();
        let hash = self.hash(chain_id_u64);
        let sig = ecdsa_sign(&hash, &private_key.0, &chain_id_u64);
        let mut r_n = sig.r;
        let mut s_n = sig.s;
        while r_n[0] == 0 {
            r_n.remove(0);
        }
        while s_n[0] == 0 {
            s_n.remove(0);
        }
        let mut tx = RlpStream::new();
        tx.begin_unbounded_list();
        self.encode(&mut tx);
        tx.append(&sig.v);
        tx.append(&r_n);
        tx.append(&s_n);
        tx.finalize_unbounded_list();
        tx.out()
    }

    fn hash(&self, chain_id: u64) -> Vec<u8> {
        let mut hash = RlpStream::new();
        hash.begin_unbounded_list();
        self.encode(&mut hash);
        hash.append(&chain_id.clone());
        hash.append(&U256::zero());
        hash.append(&U256::zero());
        hash.finalize_unbounded_list();
        keccak256_hash(&hash.out())
    }

    fn encode(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        if let Some(ref t) = self.to {
            s.append(t);
        } else {
            s.append(&vec![]);
        }
        s.append(&self.value);
        s.append(&self.data);
    }
}

fn keccak256_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    let mut resp: [u8; 32] = Default::default();
    hasher.finalize(&mut resp);
    resp.iter().cloned().collect()
}

fn ecdsa_sign(hash: &[u8], private_key: &[u8; 32], chain_id: &u64) -> EcdsaSig {
    let msg = Message::parse_slice(hash).unwrap();
    let key = SecretKey::parse(private_key).unwrap();
    let (sig, rec) = sign(&msg, &key);
    // let (v, sig_bytes) = s.sign_recoverable(&msg, &key).serialize_compact();

    EcdsaSig {
        v: rec.serialize() as i32 as u64 + chain_id * 2 + 35,
        r: sig.r.b32().to_vec(),
        s: sig.s.b32().to_vec(),
    }
}

pub struct EcdsaSig {
    v: u64,
    r: Vec<u8>,
    s: Vec<u8>,
}

mod test {

    #[test]
    fn test_signs_transaction_eth() {
        use crate::RawTransaction;
        use ethereum_types::*;
        use serde::Deserialize;
        use serde_json;
        use std::fs::File;
        use std::io::Read;

        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256,
        }

        let mut file = File::open("./test/test_txs.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 1 as u64;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }

    #[test]
    fn test_signs_transaction_ropsten() {
        use crate::RawTransaction;
        use ethereum_types::*;
        use serde::Deserialize;
        use serde_json;
        use std::fs::File;
        use std::io::Read;
        #[derive(Deserialize)]
        struct Signing {
            signed: Vec<u8>,
            private_key: H256,
        }

        let mut file = File::open("./test/test_txs_ropsten.json").unwrap();
        let mut f_string = String::new();
        file.read_to_string(&mut f_string).unwrap();
        let txs: Vec<(RawTransaction, Signing)> = serde_json::from_str(&f_string).unwrap();
        let chain_id = 3 as i32;
        for (tx, signed) in txs.into_iter() {
            assert_eq!(signed.signed, tx.sign(&signed.private_key, &chain_id));
        }
    }
}
