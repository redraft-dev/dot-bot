#![feature(proc_macro_hygiene, decl_macro)]
extern crate rocket; 

mod server;

use server::start;

fn main() {
    println!("Starting server...");
    start();
}
