#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod custom;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    custom::install();
}