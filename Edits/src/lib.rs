#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod bowser;
mod snake;
mod gannon;
mod mk;
mod falco;
mod wolf;
mod marth;
mod fox;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    bowser::install();
    snake::install();
    gannon::install();
    mk::install();
    falco::install();
    wolf::install();
    marth::install();
    fox::install();
}
