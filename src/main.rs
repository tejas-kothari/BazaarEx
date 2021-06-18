use fce_sqlite_connector::Result;
use fluence::fce;
use fluence::module_manifest;
mod db;
use db::Item;

module_manifest!();

pub fn main() {}

#[fce]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[fce]
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

#[fce]
pub fn init_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::create_tables(&conn);

    IFResult::from_res(res)
}

#[fce]
pub fn reset_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::delete_tables(&conn);

    IFResult::from_res(res)
}

#[fce]
pub fn register_user(stellar_pk: String, user_name: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_user(&conn, stellar_pk, user_name);

    IFResult::from_res(res)
}

#[fce]
pub fn list_all_users() -> Vec<String> {
    let conn = db::get_connection();
    let users = db::get_users(&conn);

    users.unwrap_or_default()
}

#[fce]
pub fn post_item_for_sale(
    user_id: String,
    item_name: String,
    pickup_location: String,
    price: f64,
    description: String,
) -> Item {
    let conn = db::get_connection();
    let item = db::add_item(
        &conn,
        user_id,
        item_name,
        pickup_location,
        price,
        description,
    );

    Item::from_res(item)
}

#[fce]
pub fn list_all_items() -> Vec<Item> {
    let conn = db::get_connection();
    let items = db::get_items(&conn);

    items.unwrap_or_default()
}

#[fce]
pub fn list_item(item_id: i64) -> Item {
    let conn = db::get_connection();
    let item = db::get_item(&conn, item_id);

    Item::from_res(item)
}

#[fce]
pub fn buy_item(user_id: String, item_id: i64, dropoff_location: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_buying_info(&conn, user_id, item_id, dropoff_location);

    IFResult::from_res(res)
}

#[fce]
pub fn accept_delivery(user_id: String, item_id: i64) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_delivery_info(&conn, user_id, item_id);

    IFResult::from_res(res)
}
