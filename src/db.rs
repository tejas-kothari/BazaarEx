use fluence::fce;
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
            uuid TEXT not null primary key , 
            name TEXT not null
        );
        "
    );

    let res2 = conn.execute(
        "
        create table if not exists items (
            uuid INTEGER not null primary key AUTOINCREMENT, 
            name TEXT not null,
            pickup_location TEXT not null,
            price INTEGER not null,
            description TEXT,
            seller_id TEXT not null,
            buyer_id TEXT default null,
            deliverer_id TEXT default null,
            dropoff_location TEXT default null,
            FOREIGN KEY (seller_id)
                REFERENCES users (uuid),
            FOREIGN KEY (buyer_id)
                REFERENCES users (uuid),
            FOREIGN KEY (deliverer_id)
                REFERENCES users (uuid)
        );
        "
    );

    vec![res, res2]
}

pub fn delete_tables(conn: &Connection) -> Result<(), Error> {
    let res = conn.execute(
        "
        drop table if exists items;
        drop table if exists users;
        "
    );

    res
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

pub fn add_item(conn: &Connection, seller_id: String, name: String, pickup_location: String, price: f64, description: String) -> Result<(), Error> {
    let res = conn.execute(format!(
            "
            insert into items (name, pickup_location, price, description, seller_id)
            values ('{}', '{}', {}, '{}', '{}');
            ",
            name,
            pickup_location,
            price,
            description,
            seller_id
        )
    );

    res
}

#[fce]
#[derive(Debug)]
pub struct Item {
    pub uuid: i64,
    pub name: String,
    pub pickup_location: String,
    pub price: f64,
    pub description: String
} 

pub fn get_items(conn: &Connection) ->Vec<Item>  {
    let mut cursor = conn.prepare(
        "
        select uuid, name, pickup_location, price, description from items;
        "
    ).unwrap().cursor();

    let mut items = Vec::new();
    while let Some(row) = cursor.next().unwrap() {
        items.push(Item {uuid: row[0].as_integer().unwrap().into(), name: row[1].as_string().unwrap().into(), pickup_location: row[2].as_string().unwrap().into(), price: row[3].as_float().unwrap().into(), description: row[4].as_string().unwrap().into()})
    }

    items
}

pub fn add_buying_info(conn: &Connection, buyer_id: String, item_id: i64, dropoff_location: String) -> Result<(), Error> {
    let res = conn.execute(format!(
            "
            update items
            SET buyer_id = '{}',
                dropoff_location = '{}'
            where 
                uuid = {};
            ",
            buyer_id,
            dropoff_location,
            item_id
        )
    );

    res
}

pub fn add_deliver_info(conn: &Connection, deliverer_id: String, item_id: i64) -> Result<(), Error> {
    let res = conn.execute(format!(
            "
            update items
            SET deliverer_id = '{}'
            where 
                uuid = {};
            ",
            deliverer_id,
            item_id
        )
    );

    res
}