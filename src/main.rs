use fluence::fce;
use fluence::module_manifest;

mod db;

module_manifest!();

pub fn main() {}

#[fce]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[fce]
#[derive(Debug)]
pub struct IFResult {
    pub success: bool,
    pub err_msg: String,
}

#[fce]
pub fn init_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::create_table(&conn);

    for curr_res in res.iter() {
        match curr_res {
            Ok(_v) => (),
            Err(e) => return IFResult {success: false, err_msg: e.to_string()}
        }
    }

    IFResult {success: true, err_msg: "".into()}
}

#[fce]
pub fn reset_service() -> IFResult {
    let conn = db::get_connection();
    let res = db::delete_tables(&conn);

    match res {
        Ok(_v) => return IFResult {success: true, err_msg: "".into()},
        Err(e) => return IFResult {success: false, err_msg: e.to_string()} 
    }
}

#[fce]
pub fn register_user(stellar_pk: String, name: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_user(&conn, stellar_pk, name);

    match res {
        Ok(_v) => return IFResult {success: true, err_msg: "".into()},
        Err(e) => return IFResult {success: false, err_msg: e.to_string()}
    }
}

#[fce]
pub fn list_users() -> Vec<String>  {
    let conn = db::get_connection();
    let users = db::get_users(&conn);

    users
}

#[fce]
pub fn post_item(user_id: String, name: String, pickup_location: String, price: u32, description: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_item(&conn, user_id, name, pickup_location, price, description);

    match res {
        Ok(_v) => return IFResult {success: true, err_msg: "".into()},
        Err(e) => return IFResult {success: false, err_msg: e.to_string()}
    }
}