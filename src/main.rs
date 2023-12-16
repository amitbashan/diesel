#![allow(non_snake_case)]

mod hook;
mod ql;
mod schedule;
mod ui;

fn main() {
    dioxus_web::launch(ui::UI);
}
