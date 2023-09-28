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

struct Producto{
    name: String,
    bar_code: BarCode,
    quantity: i32,
    price: f32,
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
    items_list: Vec<Producto>,
    total: f32,
    date: String,
}

impl Ticket {
    pub fn generate_new_ticket(){
        let mut list_products: Vec<Producto> = vec![];
        //crear otra funcion para agregar un producto a la lista.
    }
    pub fn add_product_at_list(item: &Producto){

    }
}

//structs--------------------------------------------------------------