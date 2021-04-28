use fluence::fce;
use fce_sqlite_connector;
use fce_sqlite_connector::{Connection};

const DB_PATH: &str  = "/tmp/fluence_service_db.sqlite";

fn get_connection() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

#[fce]
#[derive(Debug)]
pub struct InitResult {
    pub success: bool,
    pub err_msg: String,
}

pub fn create_table(conn: &Connection) -> Vec<std::result::Result<(), fce_sqlite_connector::Error>> {
    let res = conn.execute(
        "
        create table if not exists users (
            uuid TEXT not null primary key, 
            name TEXT not null
        );
        "
    );

    let res2 = conn.execute(
        "
        create table if not exists items (
            uuid INTEGER not null primary key, 
            name TEXT not null,
            description TEXT,
            price INTEGER not null,
            seller_id TEXT not null,
            FOREIGN KEY (seller_id)
                REFERENCES users (uuid)
        );
        "
    );

    vec![res, res2]
}

#[fce]
pub fn init_service() -> InitResult {
    let conn = get_connection();
    let res = create_table(&conn);

    for curr_res in res.iter() {
        if curr_res.is_err() {
            return InitResult {success: false, err_msg: "Failure to create tables".into()};
        }
    }

    InitResult {success: true, err_msg: "".into()}
}