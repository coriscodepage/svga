#![no_main]
#![no_std]

use core::arch::asm;

use log::info;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello world!");
    let mut x: u64 = 4;
unsafe {
    asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
}
    info!("generalnie to chyba dziala to cale asm : {} | normalne mnozenie: {}", x, 4 * 6);
    boot::stall(10_000_000);
    Status::SUCCESS
}