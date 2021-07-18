use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::Result;
mod db;
use db::Item;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
mod auth;
use auth::get_init_peer_id;
mod nft_contract_adapter;
use nft_contract_adapter::{get_name, mint};
use sha3::{Digest, Keccak256};

module_manifest!();

pub fn main() {}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[marine]
pub struct IFResult {
    pub success: bool,
    pub err_msg: String,
}

impl IFResult {
    pub fn from_res(res: Result<()>) -> IFResult {
        match res {
            Ok(_v) => IFResult {
                success: true,
                err_msg: "".into(),
            },
            Err(e) => IFResult {
                success: false,
                err_msg: e.to_string(),
            },
        }
    }
}

#[marine]
pub fn init_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::create_tables(&conn);

    IFResult::from_res(res)
}

#[marine]
pub fn reset_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::delete_tables(&conn);

    IFResult::from_res(res)
}

#[marine]
pub fn register_user(peer_id: String, name: String) -> IFResult {
    let conn = db::get_connection();

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = hex::encode(keypair.public.as_bytes());
    let secret_key = hex::encode(keypair.secret.as_bytes());

    let res = db::add_user(&conn, peer_id, name, public_key, secret_key);

    IFResult::from_res(res)
}

#[marine]
pub fn list_all_users() -> Vec<String> {
    let conn = db::get_connection();
    let users = db::get_users(&conn);

    users.unwrap_or_default()
}

#[marine]
pub fn post_item_for_sale(
    peer_id: String,
    item_name: String,
    pickup_location: String,
    price: f64,
    description: String,
) -> Item {
    let conn = db::get_connection();

    let pk_string = db::peer_id_2_pk(&conn, peer_id.clone()).unwrap();
    let pk_bytes = hex::decode(pk_string.clone()).unwrap();

    let mut hasher = Keccak256::new();
    hasher.update(pk_bytes.clone());
    let pk_hash = hex::encode(hasher.finalize());
    let add = (&pk_hash[24..]).to_string();
    let mut add_string = "0x".to_string();
    add_string.push_str(&add);

    let token_id: i64 = mint(add_string);

    let item = db::add_item(
        &conn,
        peer_id,
        item_name,
        pickup_location,
        price,
        description,
        token_id,
    );

    Item::from_res(item)
}

#[marine]
pub fn list_all_items() -> Vec<Item> {
    let conn = db::get_connection();
    let items = db::get_items(&conn);

    items.unwrap_or_default()
}

#[marine]
pub fn list_item(item_id: i64) -> Item {
    let conn = db::get_connection();
    let item = db::get_item(&conn, item_id);

    Item::from_res(item)
}

#[marine]
pub fn buy_item(peer_id: String, item_id: i64, dropoff_location: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_buying_info(&conn, peer_id, item_id, dropoff_location);

    IFResult::from_res(res)
}

#[marine]
pub fn accept_delivery(peer_id: String, item_id: i64) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_delivery_info(&conn, peer_id, item_id);

    IFResult::from_res(res)
}
