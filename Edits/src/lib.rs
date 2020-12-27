#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod doug;
mod frog;
mod gannon;
mod mk;
mod sonic;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    doug::install();
    frog::install();
    gannon::install();
    mk::install();
    sonic::install();
}
