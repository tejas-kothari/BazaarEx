use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::Result;
mod db;
use db::Item;
use ed25519_dalek::Keypair;
use rand::rngs::OsRng;
mod auth;
use auth::get_init_peer_id;

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
pub fn register_user(name: String) -> IFResult {
    let conn = db::get_connection();

    let mut csprng = OsRng {};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key = hex::encode(keypair.public.as_bytes());
    let secret_key = hex::encode(keypair.secret.as_bytes());

    let res = db::add_user(&conn, get_init_peer_id(), name, public_key, secret_key);

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
    item_name: String,
    pickup_location: String,
    price: f64,
    description: String,
) -> Item {
    let conn = db::get_connection();
    let item = db::add_item(
        &conn,
        get_init_peer_id(),
        item_name,
        pickup_location,
        price,
        description,
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
pub fn buy_item(item_id: i64, dropoff_location: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_buying_info(&conn, get_init_peer_id(), item_id, dropoff_location);

    IFResult::from_res(res)
}

#[marine]
pub fn accept_delivery(item_id: i64) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_delivery_info(&conn, get_init_peer_id(), item_id);

    IFResult::from_res(res)
}
