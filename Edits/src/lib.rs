#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod doug;
mod frog;
mod gannon;
mod mk;
mod falco;
mod wolf;
mod marth;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    doug::install();
    frog::install();
    gannon::install();
    mk::install();
    falco::install();
    wolf::install();
    marth::install();
}
