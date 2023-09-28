extern crate rusqlite;

use rusqlite::{params, Connection, Result};

//enums---------------------------------------------------------------
enum BarCode{
    BarCodeStr(String),
    BarCodeInt(i64),
}

enum EventType{
    EntryCash(String),
    OutputCash(String),
    ProvetorEntry(String),
    TicketGenerate(),
}

struct Product{
    name: String,
    bar_code: BarCode,
    quantity: i32,
    price_cost: f32,
    price_sell: f32,
    departament: String,
}
struct Usuarios{
    username: String,
    password: String,
    name: String,
    last_name: String,
    direction: String,
    number: String
}
struct Eventos{
    date: String,
    username: String,
    event_type: EventType,
    description: String,
}
struct Ticket{
    id: i32,
    creator: String,
    items_list: Vec<Product>,
    total: f32,
    date: String,
}

pub fn insert_new_product(conn: &Connection)-> Result<()>{
    
    {
        let mut stmt = conn.prepare_cached("INSERT INTO Products (name) VALUES (?1)")?;
        stmt.execute(["Miguel Ramos"])?;
    }

    {
        let mut stmt = conn.prepare_cached(("INSERT INTO Products (name) VALUES (?1)"))?;
        stmt.execute(["Valeria Ramos"])?;
    }
    Ok(())
}
//structs--------------------------------------------------------------