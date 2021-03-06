#![no_std]
#![no_main]

/*
#[macro_use]
extern crate lazy_static;
*/

use panic_halt as _;
use cortex_m_rt::entry;
use atsamv71q21::Peripherals;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    let watchdog_timer = p.WDT;
    watchdog_timer.mr.write(|w| w.wddis().set_bit());


    let port_a = &p.PIOA;
    unsafe {
        port_a.idr.write_with_zero(|w| w.p23().set_bit());
        port_a.pudr.write_with_zero(|w| w.p23().set_bit());
    }
    port_a.abcdsr.first().unwrap().modify(|_, w| w.p23().set_bit());
    port_a.abcdsr.last().unwrap().modify(|_, w| w.p23().set_bit());

    let mut is_set = true;

    loop {
        let value = port_a.odsr.read().bits();
        if is_set {
            unsafe { port_a.codr.write_with_zero(|w| w.p23().set_bit()); }
        } else {
            unsafe { port_a.sodr.write_with_zero(|w| w.bits(value & (1 << 23))); }
        }
        delay();
        is_set = !is_set;
    }
}

fn delay(){
    for _ in 0..10_000_00 {
        cortex_m::asm::nop();
    }
}