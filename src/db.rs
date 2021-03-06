use marine_rs_sdk::marine;
use marine_sqlite_connector::{Connection, Error, Result, Value};

const DB_PATH: &str = "/tmp/fluence_service_db.sqlite";

pub fn get_none_error() -> Error {
    Error {
        code: None,
        message: Some("Value doesn't exist".to_string()),
    }
}

pub fn get_connection() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        create table if not exists users (
            uuid TEXT not null primary key, 
            name TEXT not null,
            public_key TEXT not null,
            secret_key TEXT not null
        ) without rowid;
        ",
    )?;

    conn.execute(
        "
        create table if not exists items (
            uuid INTEGER not null primary key AUTOINCREMENT, 
            name TEXT not null,
            pickup_location TEXT not null,
            dropoff_location TEXT default null,
            price INTEGER not null,
            description TEXT default null,
            seller_id TEXT not null,
            buyer_id TEXT default null,
            deliverer_id TEXT default null,
            token_id INTEGER not null,
            FOREIGN KEY (seller_id)
                REFERENCES users (uuid),
            FOREIGN KEY (buyer_id)
                REFERENCES users (uuid),
            FOREIGN KEY (deliverer_id)
                REFERENCES users (uuid)
        );
        ",
    )?;

    Ok(())
}

pub fn delete_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        drop table if exists items;
        drop table if exists users;
        ",
    )?;

    Ok(())
}

pub fn add_user(
    conn: &Connection,
    peer_id: String,
    name: String,
    public_key: String,
    secret_key: String,
) -> Result<User> {
    conn.execute(format!(
        "
        insert into users (uuid, name, public_key, secret_key)
        values ('{}', '{}', '{}', '{}');
        ",
        peer_id, name, public_key, secret_key
    ))?;

    get_user(conn, peer_id)
}

#[marine]
#[derive(Default)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub public_key: String,
    pub address: String,
    pub err_msg: String,
    pub success: bool,
}

impl User {
    pub fn from_row(row: &[Value]) -> Result<User> {
        let mut user = User {
            uuid: row[0].as_string().ok_or(get_none_error())?.to_string(),
            name: row[1].as_string().ok_or(get_none_error())?.to_string(),
            public_key: row[2].as_string().ok_or(get_none_error())?.to_string(),
            err_msg: "".to_string(),
            success: true,
            ..Default::default()
        };
        user.address = web3::eth_utils::pk_to_add(user.public_key.clone());

        Ok(user)
    }

    pub fn from_res(res: Result<User>) -> User {
        match res {
            Ok(v) => v,
            Err(e) => {
                let mut res_user: User = Default::default();
                res_user.err_msg = e.to_string();
                res_user.success = false;
                res_user
            }
        }
    }
}

pub fn get_users(conn: &Connection) -> Result<Vec<User>> {
    let mut cursor = conn.prepare("select * from users;")?.cursor();

    let mut users = Vec::new();
    while let Some(row) = cursor.next()? {
        users.push(User::from_row(row)?);
    }

    Ok(users)
}

pub fn get_user(conn: &Connection, peer_id: String) -> Result<User> {
    let mut cursor = conn
        .prepare(format!("select * from users where uuid = '{}';", peer_id))?
        .cursor();

    let row = cursor.next()?.ok_or(get_none_error())?;
    let found_user = User::from_row(row);
    Ok(found_user?)
}

#[marine]
#[derive(Default)]
pub struct Item {
    pub uuid: i64,
    pub item_name: String,
    pub pickup_location: String,
    pub dropoff_location: String,
    pub price: f64,
    pub description: String,
    pub seller_id: String,
    pub buyer_id: String,
    pub deliverer_id: String,
    pub token_id: i64,
    pub err_msg: String,
    pub success: bool,
}

impl Item {
    pub fn from_row(row: &[Value]) -> Result<Item> {
        let row_item = Item {
            uuid: row[0].as_integer().ok_or(get_none_error())?,
            item_name: row[1].as_string().ok_or(get_none_error())?.to_string(),
            pickup_location: row[2].as_string().ok_or(get_none_error())?.to_string(),
            dropoff_location: row[3].as_string().unwrap_or_default().to_string(),
            price: row[4].as_float().ok_or(get_none_error())?,
            description: row[5].as_string().unwrap_or_default().to_string(),
            seller_id: row[6].as_string().ok_or(get_none_error())?.to_string(),
            buyer_id: row[7].as_string().unwrap_or_default().to_string(),
            deliverer_id: row[8].as_string().unwrap_or_default().to_string(),
            token_id: row[9].as_integer().ok_or(get_none_error())?,
            err_msg: "".to_string(),
            success: true,
        };

        Ok(row_item)
    }

    pub fn from_res(res: Result<Item>) -> Item {
        match res {
            Ok(v) => v,
            Err(e) => {
                let mut res_item: Item = Default::default();
                res_item.err_msg = e.to_string();
                res_item.success = false;
                res_item
            }
        }
    }
}

pub fn add_item(
    conn: &Connection,
    seller_id: String,
    item_name: String,
    pickup_location: String,
    price: f64,
    description: String,
    token_id: i64,
) -> Result<Item> {
    conn.execute(format!(
        "
        insert into items (name, pickup_location, price, description, seller_id, token_id)
        values ('{}', '{}', {}, '{}', '{}', '{}');
        ",
        item_name, pickup_location, price, description, seller_id, token_id
    ))?;

    let new_row_id = conn
        .prepare("select last_insert_rowid();")?
        .cursor()
        .next()?
        .ok_or(get_none_error())?[0]
        .as_integer()
        .ok_or(get_none_error())?;

    get_item(conn, new_row_id)
}

pub fn get_item(conn: &Connection, item_id: i64) -> Result<Item> {
    let mut cursor = conn
        .prepare(format!("select * from items where uuid = {};", item_id))?
        .cursor();

    let row = cursor.next()?.ok_or(get_none_error())?;

    let found_item = Item::from_row(row);

    Ok(found_item?)
}

pub fn get_items(conn: &Connection) -> Result<Vec<Item>> {
    let mut cursor = conn.prepare("select * from items;")?.cursor();

    let mut items = Vec::new();
    while let Some(row) = cursor.next()? {
        items.push(Item::from_row(row)?)
    }

    Ok(items)
}

pub fn add_buying_info(
    conn: &Connection,
    buyer_id: String,
    item_id: i64,
    dropoff_location: String,
) -> Result<()> {
    let res = conn.execute(format!(
        "
        update items
        SET buyer_id = '{}',
            dropoff_location = '{}'
        where 
            uuid = {};
        ",
        buyer_id, dropoff_location, item_id
    ));

    res
}

pub fn add_delivery_info(conn: &Connection, deliverer_id: String, item_id: i64) -> Result<()> {
    let res = conn.execute(format!(
        "
        update items
        SET deliverer_id = '{}'
        where 
            uuid = {};
        ",
        deliverer_id, item_id
    ));

    res
}

pub fn peer_id_2_pk(conn: &Connection, peer_id: String) -> Result<String> {
    let mut cursor = conn
        .prepare(format!(
            "select public_key from users where uuid = '{}';",
            peer_id
        ))?
        .cursor();

    let pk = cursor.next()?.ok_or(get_none_error())?[0]
        .as_string()
        .ok_or(get_none_error())?;

    Ok(pk.to_string())
}

pub fn peer_id_2_sk(conn: &Connection, peer_id: String) -> Result<String> {
    let mut cursor = conn
        .prepare(format!(
            "select secret_key from users where uuid = '{}';",
            peer_id
        ))?
        .cursor();

    let sk = cursor.next()?.ok_or(get_none_error())?[0]
        .as_string()
        .ok_or(get_none_error())?;

    Ok(sk.to_string())
}
