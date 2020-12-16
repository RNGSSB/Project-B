#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;
mod mk;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
    mk::install();
}
