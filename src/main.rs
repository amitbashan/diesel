#![allow(non_snake_case)]

mod hook;
mod ql;
mod schedule;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dioxus_web::launch(ui::UI);
}
