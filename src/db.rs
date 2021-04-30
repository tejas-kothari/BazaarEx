use fce_sqlite_connector;
use fce_sqlite_connector::{Connection, Error};
use std::result::Result;

const DB_PATH: &str  = "/tmp/fluence_service_db.sqlite";

pub fn get_connection() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn create_table(conn: &Connection) -> Vec<Result<(), Error>> {
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
            pickup_location TEXT not null,
            price INTEGER not null,
            description TEXT,
            seller_id TEXT not null,
            FOREIGN KEY (seller_id)
                REFERENCES users (uuid)
        );
        "
    );

    vec![res, res2]
}

pub fn add_user(conn: &Connection, stellar_pk: String, name: String) -> Result<(), Error> {
    let res = conn.execute(format!(
            "
            insert into users (uuid, name)
            values ('{}', '{}');
            ",
            stellar_pk,
            name
        )
    );

    res
}

pub fn get_users(conn: &Connection) -> Vec<String> {
    let mut cursor = conn.prepare(
        "
        select * from users;
        "
    ).unwrap().cursor();

    let mut names = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        names.push(row[0].as_string().unwrap().into())
    }

    names
}