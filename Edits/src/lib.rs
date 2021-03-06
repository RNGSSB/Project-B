#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod bowser;
mod dedede;
mod falco;
mod fox;
mod gannon;
mod luigi;
mod mario;
mod marth;
mod mk;
mod ness;
mod puff;
mod samus;
mod snake;
mod sonic;
mod wario;
mod wolf;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    bowser::install();
    dedede::install();
    falco::install();
    fox::install();
    gannon::install();
    luigi::install();
    mario::install();
    marth::install();
    mk::install();
    ness::install();
    puff::install();
    samus::install();
    snake::install();
    sonic::install();
    wario::install();
    wolf::install();
}