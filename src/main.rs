use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::Result;
mod db;
use db::{Item, User};
use rand::rngs::OsRng;
mod auth;
use auth::{am_i_owner, get_init_peer_id};
mod nft_contract_adapter;
use nft_contract_adapter::{fund_acct, mint, transfer};

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

    pub fn from_err_str(e: &str) -> IFResult {
        IFResult {
            success: false,
            err_msg: e.to_string(),
        }
    }
}

#[marine]
pub fn init_service() -> IFResult {
    if !am_i_owner() {
        return IFResult::from_err_str("You are not the owner!");
    }

    let conn = db::get_connection();
    let res = db::create_tables(&conn);
    IFResult::from_res(res)
}

#[marine]
pub fn reset_service() -> IFResult {
    if !am_i_owner() {
        return IFResult::from_err_str("You are not the owner!");
    }

    let conn = db::get_connection();
    let res = db::delete_tables(&conn);
    IFResult::from_res(res)
}

#[marine]
pub fn register_user(peer_id: String, name: String) -> User {
    let mut csprng = OsRng {};
    let sk = libsecp256k1::SecretKey::random(&mut csprng);
    let pk = libsecp256k1::PublicKey::from_secret_key(&sk);
    let secret_key = hex::encode(sk.serialize());
    let public_key = hex::encode(pk.serialize());

    // For demo use only! (Transfers 1 ETH to user account)
    let add_string = web3::eth_utils::pk_to_add(public_key.clone());
    fund_acct(add_string);

    let conn = db::get_connection();
    let res = db::add_user(&conn, peer_id, name, public_key.clone(), secret_key);
    User::from_res(res)
}

#[marine]
pub fn list_all_users() -> Vec<User> {
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
    let add_string = web3::eth_utils::pk_to_add(pk_string);
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

#[marine]
pub fn pickup_item(peer_id: String, item_id: i64) -> IFResult {
    let conn = db::get_connection();
    let item = db::get_item(&conn, item_id).unwrap();

    let from_sk_string = db::peer_id_2_sk(&conn, item.seller_id.clone()).unwrap();
    let from_pk_string = db::peer_id_2_pk(&conn, item.seller_id.clone()).unwrap();
    let from_add_string = web3::eth_utils::pk_to_add(from_pk_string);
    println!("from_add_string: {}", from_add_string);

    let to_pk_string = db::peer_id_2_pk(&conn, peer_id.clone()).unwrap();
    let to_add_string = web3::eth_utils::pk_to_add(to_pk_string);

    println!(
        "{}",
        transfer(
            from_sk_string,
            from_add_string,
            to_add_string,
            item.token_id,
        )
    );

    IFResult::from_res(Ok(()))
}

#[marine]
pub fn deliver_item(peer_id: String, item_id: i64) -> IFResult {
    let conn = db::get_connection();
    let item = db::get_item(&conn, item_id).unwrap();

    let from_sk_string = db::peer_id_2_sk(&conn, peer_id.clone()).unwrap();
    let from_pk_string = db::peer_id_2_pk(&conn, peer_id.clone()).unwrap();
    let from_add_string = web3::eth_utils::pk_to_add(from_pk_string);

    let to_pk_string = db::peer_id_2_pk(&conn, item.buyer_id).unwrap();
    let to_add_string = web3::eth_utils::pk_to_add(to_pk_string);

    transfer(
        from_sk_string,
        from_add_string,
        to_add_string,
        item.token_id,
    );

    IFResult::from_res(Ok(()))
}
