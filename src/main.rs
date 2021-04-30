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
        if curr_res.is_err() {
            return IFResult {success: false, err_msg: "Failure to create tables".into()};
        }
    }

    IFResult {success: true, err_msg: "".into()}
}

#[fce]
pub fn register_user(stellar_pk: String, name: String) -> IFResult {
    let conn = db::get_connection();
    let res = db::add_user(&conn, stellar_pk, name);
    if res.is_err() {
        return IFResult {success: false, err_msg: "Failure to add user".into()};
    }

    IFResult {success: true, err_msg: "".into()}
}

#[fce]
pub fn list_users() -> Vec<String>  {
    let conn = db::get_connection();
    let users = db::get_users(&conn);

    users
}