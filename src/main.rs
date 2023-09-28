mod db_controllers;
use db_controllers::*;
use rusqlite::Connection;

fn main() {
    let conn = Connection::open("pos_db.db").expect("No se pudo conectar a la base de datos, llame al tecnico");

    //llama a la funcion para insertar nuesvos productos
    match insert_new_product(&conn) {
        Ok(_) => println!("Producto Creado"),
        Err(err) => eprintln!("Error: {:?}", err),
    }
    println!("Hello, world!");
}
