#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod mk;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    doug::install();
    frog::install();
    gannon::install();
    mk::install();
    sonic::install();
}
