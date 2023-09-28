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

pub fn insert_new_product(conn: &Connection, producto: Product)-> Result<()>{

    {
        let mut stmt = conn.prepare_cached("INSERT INTO Products (name, bar_code, quantity, price_cost, price_sell, departament) VALUES (?1, ?2, ?3, ?4, ?5, ?5)")?;
        stmt.execute(params![
            producto.name,
            match producto.bar_code {
                BarCode::BarCodeStr(s) => s,
                BarCode::BarCodeInt(i) => i.to_string(),
            },
            producto.quantity,
            producto.price_cost,
            producto.price_sell,
            producto.departament,
        ])?;
    }
    Ok(())
}
//structs--------------------------------------------------------------