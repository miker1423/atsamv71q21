#![doc = "Peripheral access API for ATSAMV71Q21 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn PMC();
    fn EFC();
    fn UART0();
    fn UART1();
    fn PIOA();
    fn PIOB();
    fn PIOC();
    fn USART0();
    fn USART1();
    fn USART2();
    fn PIOD();
    fn PIOE();
    fn HSMCI();
    fn TWIHS0();
    fn TWIHS1();
    fn SPI0();
    fn SSC();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn AFEC0();
    fn DACC();
    fn PWM0();
    fn ICM();
    fn ACC();
    fn USBHS();
    fn MCAN0();
    fn MCAN1();
    fn GMAC();
    fn AFEC1();
    fn TWIHS2();
    fn SPI1();
    fn QSPI();
    fn UART2();
    fn UART3();
    fn UART4();
    fn TC6();
    fn TC7();
    fn TC8();
    fn TC9();
    fn TC10();
    fn TC11();
    fn MLB();
    fn AES();
    fn TRNG();
    fn XDMAC();
    fn ISI();
    fn PWM1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 61] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PMC },
    Vector { _handler: EFC },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _handler: PIOC },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: PIOD },
    Vector { _handler: PIOE },
    Vector { _handler: HSMCI },
    Vector { _handler: TWIHS0 },
    Vector { _handler: TWIHS1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SSC },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: AFEC0 },
    Vector { _handler: DACC },
    Vector { _handler: PWM0 },
    Vector { _handler: ICM },
    Vector { _handler: ACC },
    Vector { _handler: USBHS },
    Vector { _handler: MCAN0 },
    Vector { _reserved: 0 },
    Vector { _handler: MCAN1 },
    Vector { _reserved: 0 },
    Vector { _handler: GMAC },
    Vector { _handler: AFEC1 },
    Vector { _handler: TWIHS2 },
    Vector { _handler: SPI1 },
    Vector { _handler: QSPI },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: TC6 },
    Vector { _handler: TC7 },
    Vector { _handler: TC8 },
    Vector { _handler: TC9 },
    Vector { _handler: TC10 },
    Vector { _handler: TC11 },
    Vector { _handler: MLB },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: AES },
    Vector { _handler: TRNG },
    Vector { _handler: XDMAC },
    Vector { _handler: ISI },
    Vector { _handler: PWM1 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "5 - PMC"]
    PMC = 5,
    #[doc = "6 - EFC"]
    EFC = 6,
    #[doc = "7 - UART0"]
    UART0 = 7,
    #[doc = "8 - UART1"]
    UART1 = 8,
    #[doc = "10 - PIOA"]
    PIOA = 10,
    #[doc = "11 - PIOB"]
    PIOB = 11,
    #[doc = "12 - PIOC"]
    PIOC = 12,
    #[doc = "13 - USART0"]
    USART0 = 13,
    #[doc = "14 - USART1"]
    USART1 = 14,
    #[doc = "15 - USART2"]
    USART2 = 15,
    #[doc = "16 - PIOD"]
    PIOD = 16,
    #[doc = "17 - PIOE"]
    PIOE = 17,
    #[doc = "18 - HSMCI"]
    HSMCI = 18,
    #[doc = "19 - TWIHS0"]
    TWIHS0 = 19,
    #[doc = "20 - TWIHS1"]
    TWIHS1 = 20,
    #[doc = "21 - SPI0"]
    SPI0 = 21,
    #[doc = "22 - SSC"]
    SSC = 22,
    #[doc = "23 - TC0"]
    TC0 = 23,
    #[doc = "24 - TC1"]
    TC1 = 24,
    #[doc = "25 - TC2"]
    TC2 = 25,
    #[doc = "26 - TC3"]
    TC3 = 26,
    #[doc = "27 - TC4"]
    TC4 = 27,
    #[doc = "28 - TC5"]
    TC5 = 28,
    #[doc = "29 - AFEC0"]
    AFEC0 = 29,
    #[doc = "30 - DACC"]
    DACC = 30,
    #[doc = "31 - PWM0"]
    PWM0 = 31,
    #[doc = "32 - ICM"]
    ICM = 32,
    #[doc = "33 - ACC"]
    ACC = 33,
    #[doc = "34 - USBHS"]
    USBHS = 34,
    #[doc = "35 - MCAN0"]
    MCAN0 = 35,
    #[doc = "37 - MCAN1"]
    MCAN1 = 37,
    #[doc = "39 - GMAC"]
    GMAC = 39,
    #[doc = "40 - AFEC1"]
    AFEC1 = 40,
    #[doc = "41 - TWIHS2"]
    TWIHS2 = 41,
    #[doc = "42 - SPI1"]
    SPI1 = 42,
    #[doc = "43 - QSPI"]
    QSPI = 43,
    #[doc = "44 - UART2"]
    UART2 = 44,
    #[doc = "45 - UART3"]
    UART3 = 45,
    #[doc = "46 - UART4"]
    UART4 = 46,
    #[doc = "47 - TC6"]
    TC6 = 47,
    #[doc = "48 - TC7"]
    TC7 = 48,
    #[doc = "49 - TC8"]
    TC8 = 49,
    #[doc = "50 - TC9"]
    TC9 = 50,
    #[doc = "51 - TC10"]
    TC10 = 51,
    #[doc = "52 - TC11"]
    TC11 = 52,
    #[doc = "53 - MLB"]
    MLB = 53,
    #[doc = "56 - AES"]
    AES = 56,
    #[doc = "57 - TRNG"]
    TRNG = 57,
    #[doc = "58 - XDMAC"]
    XDMAC = 58,
    #[doc = "59 - ISI"]
    ISI = 59,
    #[doc = "60 - PWM1"]
    PWM1 = 60,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsmci::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSC::ptr() }
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Serial Peripheral Interface 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0"]
pub mod spi0;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc2::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Timer Counter 2"]
pub mod tc2;
#[doc = "Two-wire Interface High Speed 0"]
pub struct TWIHS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS0 {}
impl TWIHS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for TWIHS0 {
    type Target = twihs0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS0::ptr() }
    }
}
#[doc = "Two-wire Interface High Speed 0"]
pub mod twihs0;
#[doc = "Two-wire Interface High Speed 1"]
pub struct TWIHS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS1 {}
impl TWIHS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs1::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for TWIHS1 {
    type Target = twihs1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS1::ptr() }
    }
}
#[doc = "Two-wire Interface High Speed 1"]
pub mod twihs1;
#[doc = "Pulse Width Modulation Controller 0"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller 0"]
pub mod pwm0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart2::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub mod usart2;
#[doc = "Controller Area Network 0"]
pub struct MCAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCAN0 {}
impl MCAN0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcan0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for MCAN0 {
    type Target = mcan0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCAN0::ptr() }
    }
}
#[doc = "Controller Area Network 0"]
pub mod mcan0;
#[doc = "Controller Area Network 1"]
pub struct MCAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCAN1 {}
impl MCAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcan1::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for MCAN1 {
    type Target = mcan1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCAN1::ptr() }
    }
}
#[doc = "Controller Area Network 1"]
pub mod mcan1;
#[doc = "USB High-Speed Interface"]
pub struct USBHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHS {}
impl USBHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhs::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for USBHS {
    type Target = usbhs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHS::ptr() }
    }
}
#[doc = "USB High-Speed Interface"]
pub mod usbhs;
#[doc = "Analog Front-End Controller 0"]
pub struct AFEC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC0 {}
impl AFEC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec0::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for AFEC0 {
    type Target = afec0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC0::ptr() }
    }
}
#[doc = "Analog Front-End Controller 0"]
pub mod afec0;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Analog Comparator Controller"]
pub struct ACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACC {}
impl ACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acc::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for ACC {
    type Target = acc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACC::ptr() }
    }
}
#[doc = "Analog Comparator Controller"]
pub mod acc;
#[doc = "Integrity Check Monitor"]
pub struct ICM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICM {}
impl ICM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icm::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for ICM {
    type Target = icm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICM::ptr() }
    }
}
#[doc = "Integrity Check Monitor"]
pub mod icm;
#[doc = "Image Sensor Interface"]
pub struct ISI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ISI {}
impl ISI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const isi::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for ISI {
    type Target = isi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ISI::ptr() }
    }
}
#[doc = "Image Sensor Interface"]
pub mod isi;
#[doc = "Gigabit Ethernet MAC"]
pub struct GMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GMAC {}
impl GMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gmac::RegisterBlock {
        0x4005_0000 as *const _
    }
}
impl Deref for GMAC {
    type Target = gmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GMAC::ptr() }
    }
}
#[doc = "Gigabit Ethernet MAC"]
pub mod gmac;
#[doc = "Timer Counter 3"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc3::RegisterBlock {
        0x4005_4000 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "Timer Counter 3"]
pub mod tc3;
#[doc = "Serial Peripheral Interface 1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0x4005_8000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 1"]
pub mod spi1;
#[doc = "Pulse Width Modulation Controller 1"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller 1"]
pub mod pwm1;
#[doc = "Two-wire Interface High Speed 2"]
pub struct TWIHS2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS2 {}
impl TWIHS2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs2::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for TWIHS2 {
    type Target = twihs2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS2::ptr() }
    }
}
#[doc = "Two-wire Interface High Speed 2"]
pub mod twihs2;
#[doc = "Analog Front-End Controller 1"]
pub struct AFEC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC1 {}
impl AFEC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec1::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for AFEC1 {
    type Target = afec1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC1::ptr() }
    }
}
#[doc = "Analog Front-End Controller 1"]
pub mod afec1;
#[doc = "Media LB"]
pub struct MLB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MLB {}
impl MLB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mlb::RegisterBlock {
        0x4006_8000 as *const _
    }
}
impl Deref for MLB {
    type Target = mlb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MLB::ptr() }
    }
}
#[doc = "Media LB"]
pub mod mlb;
#[doc = "Advanced Encryption Standard"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aes;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Extensible DMA Controller"]
pub struct XDMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XDMAC {}
impl XDMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xdmac::RegisterBlock {
        0x4007_8000 as *const _
    }
}
impl Deref for XDMAC {
    type Target = xdmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*XDMAC::ptr() }
    }
}
#[doc = "Extensible DMA Controller"]
pub mod xdmac;
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI {}
impl QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for QSPI {
    type Target = qspi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub mod qspi;
#[doc = "Static Memory Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "Static Memory Controller"]
pub mod smc;
#[doc = "SDRAM Controller"]
pub struct SDRAMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDRAMC {}
impl SDRAMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sdramc::RegisterBlock {
        0x4008_4000 as *const _
    }
}
impl Deref for SDRAMC {
    type Target = sdramc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SDRAMC::ptr() }
    }
}
#[doc = "SDRAM Controller"]
pub mod sdramc;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "USB Transmitter Interface Macrocell"]
pub struct UTMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UTMI {}
impl UTMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const utmi::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for UTMI {
    type Target = utmi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UTMI::ptr() }
    }
}
#[doc = "USB Transmitter Interface Macrocell"]
pub mod utmi;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x400e_0600 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_0800 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 0"]
pub mod uart0;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0940 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x400e_0a00 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 1"]
pub mod uart1;
#[doc = "Embedded Flash Controller"]
pub struct EFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC {}
impl EFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc::RegisterBlock {
        0x400e_0c00 as *const _
    }
}
impl Deref for EFC {
    type Target = efc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC::ptr() }
    }
}
#[doc = "Embedded Flash Controller"]
pub mod efc;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_0e00 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piob::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioc::RegisterBlock {
        0x400e_1200 as *const _
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOC::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Parallel Input/Output Controller D"]
pub struct PIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOD {}
impl PIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const piod::RegisterBlock {
        0x400e_1400 as *const _
    }
}
impl Deref for PIOD {
    type Target = piod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOD::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller D"]
pub mod piod;
#[doc = "Parallel Input/Output Controller E"]
pub struct PIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOE {}
impl PIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioe::RegisterBlock {
        0x400e_1600 as *const _
    }
}
impl Deref for PIOE {
    type Target = pioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOE::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller E"]
pub mod pioe;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x400e_1800 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x400e_1810 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        0x400e_1830 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400e_1850 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x400e_1860 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Registers"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        0x400e_1890 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[doc = "Reinforced Safety Watchdog Timer"]
pub struct RSWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSWDT {}
impl RSWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rswdt::RegisterBlock {
        0x400e_1900 as *const _
    }
}
impl Deref for RSWDT {
    type Target = rswdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSWDT::ptr() }
    }
}
#[doc = "Reinforced Safety Watchdog Timer"]
pub mod rswdt;
#[doc = "Universal Asynchronous Receiver Transmitter 2"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart2::RegisterBlock {
        0x400e_1a00 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 2"]
pub mod uart2;
#[doc = "Universal Asynchronous Receiver Transmitter 3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart3::RegisterBlock {
        0x400e_1c00 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 3"]
pub mod uart3;
#[doc = "Universal Asynchronous Receiver Transmitter 4"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart4::RegisterBlock {
        0x400e_1e00 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter 4"]
pub mod uart4;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TWIHS0"]
    pub TWIHS0: TWIHS0,
    #[doc = "TWIHS1"]
    pub TWIHS1: TWIHS1,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "MCAN0"]
    pub MCAN0: MCAN0,
    #[doc = "MCAN1"]
    pub MCAN1: MCAN1,
    #[doc = "USBHS"]
    pub USBHS: USBHS,
    #[doc = "AFEC0"]
    pub AFEC0: AFEC0,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "ACC"]
    pub ACC: ACC,
    #[doc = "ICM"]
    pub ICM: ICM,
    #[doc = "ISI"]
    pub ISI: ISI,
    #[doc = "GMAC"]
    pub GMAC: GMAC,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "TWIHS2"]
    pub TWIHS2: TWIHS2,
    #[doc = "AFEC1"]
    pub AFEC1: AFEC1,
    #[doc = "MLB"]
    pub MLB: MLB,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "XDMAC"]
    pub XDMAC: XDMAC,
    #[doc = "QSPI"]
    pub QSPI: QSPI,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "SDRAMC"]
    pub SDRAMC: SDRAMC,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "UTMI"]
    pub UTMI: UTMI,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "EFC"]
    pub EFC: EFC,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
    #[doc = "PIOD"]
    pub PIOD: PIOD,
    #[doc = "PIOE"]
    pub PIOE: PIOE,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
    #[doc = "RSWDT"]
    pub RSWDT: RSWDT,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TWIHS0: TWIHS0 {
                _marker: PhantomData,
            },
            TWIHS1: TWIHS1 {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            MCAN0: MCAN0 {
                _marker: PhantomData,
            },
            MCAN1: MCAN1 {
                _marker: PhantomData,
            },
            USBHS: USBHS {
                _marker: PhantomData,
            },
            AFEC0: AFEC0 {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            ACC: ACC {
                _marker: PhantomData,
            },
            ICM: ICM {
                _marker: PhantomData,
            },
            ISI: ISI {
                _marker: PhantomData,
            },
            GMAC: GMAC {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            TWIHS2: TWIHS2 {
                _marker: PhantomData,
            },
            AFEC1: AFEC1 {
                _marker: PhantomData,
            },
            MLB: MLB {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            XDMAC: XDMAC {
                _marker: PhantomData,
            },
            QSPI: QSPI {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            SDRAMC: SDRAMC {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            UTMI: UTMI {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            EFC: EFC {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
                _marker: PhantomData,
            },
            PIOD: PIOD {
                _marker: PhantomData,
            },
            PIOE: PIOE {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
            RSWDT: RSWDT {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
        }
    }
}
