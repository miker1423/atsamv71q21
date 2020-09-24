#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 4usize],
    _reserved_1_mr: [u8; 4usize],
    _reserved_2_ier: [u8; 4usize],
    _reserved_3_idr: [u8; 4usize],
    _reserved_4_imr: [u8; 4usize],
    _reserved_5_csr: [u8; 4usize],
    #[doc = "0x18 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    _reserved_10_ttgr: [u8; 4usize],
    _reserved11: [u8; 36usize],
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: LINBRR,
    #[doc = "0x60 - LON Mode Register"]
    pub lonmr: LONMR,
    #[doc = "0x64 - LON Preamble Register"]
    pub lonpr: LONPR,
    #[doc = "0x68 - LON Data Length Register"]
    pub londl: LONDL,
    #[doc = "0x6c - LON L2HDR Register"]
    pub lonl2hdr: LONL2HDR,
    #[doc = "0x70 - LON Backlog Register"]
    pub lonbl: LONBL,
    #[doc = "0x74 - LON Beta1 Tx Register"]
    pub lonb1tx: LONB1TX,
    #[doc = "0x78 - LON Beta1 Rx Register"]
    pub lonb1rx: LONB1RX,
    #[doc = "0x7c - LON Priority Register"]
    pub lonprio: LONPRIO,
    #[doc = "0x80 - LON IDT Tx Register"]
    pub idttx: IDTTX,
    #[doc = "0x84 - LON IDT Rx Register"]
    pub idtrx: IDTRX,
    #[doc = "0x88 - IC DIFF Register"]
    pub icdiff: ICDIFF,
    _reserved26: [u8; 88usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi_mode(&self) -> &CR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR_SPI_MODE) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi_mode_mut(&self) -> &mut CR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR_SPI_MODE) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &CR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_mut(&self) -> &mut CR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi_mode(&self) -> &MR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR_SPI_MODE) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi_mode_mut(&self) -> &mut MR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR_SPI_MODE) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &MR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_mut(&self) -> &mut MR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lon_mode(&self) -> &IER_LON_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_LON_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lon_mode_mut(&self) -> &mut IER_LON_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_LON_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin_mode(&self) -> &IER_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_LIN_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin_mode_mut(&self) -> &mut IER_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_LIN_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi_mode(&self) -> &IER_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_SPI_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi_mode_mut(&self) -> &mut IER_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_SPI_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lon_mode(&self) -> &IDR_LON_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_LON_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lon_mode_mut(&self) -> &mut IDR_LON_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_LON_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin_mode(&self) -> &IDR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_LIN_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin_mode_mut(&self) -> &mut IDR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_LIN_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi_mode(&self) -> &IDR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_SPI_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi_mode_mut(&self) -> &mut IDR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_SPI_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &IDR {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_mut(&self) -> &mut IDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lon_mode(&self) -> &IMR_LON_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_LON_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lon_mode_mut(&self) -> &mut IMR_LON_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_LON_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin_mode(&self) -> &IMR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_LIN_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin_mode_mut(&self) -> &mut IMR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_LIN_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_spi_mode(&self) -> &IMR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_SPI_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_spi_mode_mut(&self) -> &mut IMR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_SPI_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &IMR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_mut(&self) -> &mut IMR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lon_mode(&self) -> &CSR_LON_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_LON_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lon_mode_mut(&self) -> &mut CSR_LON_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_LON_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin_mode(&self) -> &CSR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_LIN_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin_mode_mut(&self) -> &mut CSR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_LIN_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi_mode(&self) -> &CSR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_SPI_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi_mode_mut(&self) -> &mut CSR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_SPI_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &CSR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_mut(&self) -> &mut CSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR) }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn ttgr_lon_mode(&self) -> &TTGR_LON_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const TTGR_LON_MODE) }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn ttgr_lon_mode_mut(&self) -> &mut TTGR_LON_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut TTGR_LON_MODE) }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn ttgr(&self) -> &TTGR {
        unsafe { &*(((self as *const Self) as *const u8).add(40usize) as *const TTGR) }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn ttgr_mut(&self) -> &mut TTGR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(40usize) as *mut TTGR) }
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr_spi_mode](cr_spi_mode) module"]
pub type CR_SPI_MODE = crate::Reg<u32, _CR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [cr_spi_mode::W](cr_spi_mode::W) writer structure"]
impl crate::Writable for CR_SPI_MODE {}
#[doc = "Control Register"]
pub mod cr_spi_mode;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr_spi_mode](mr_spi_mode) module"]
pub type MR_SPI_MODE = crate::Reg<u32, _MR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR_SPI_MODE;
#[doc = "`read()` method returns [mr_spi_mode::R](mr_spi_mode::R) reader structure"]
impl crate::Readable for MR_SPI_MODE {}
#[doc = "`write(|w| ..)` method takes [mr_spi_mode::W](mr_spi_mode::W) writer structure"]
impl crate::Writable for MR_SPI_MODE {}
#[doc = "Mode Register"]
pub mod mr_spi_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_spi_mode](ier_spi_mode) module"]
pub type IER_SPI_MODE = crate::Reg<u32, _IER_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [ier_spi_mode::W](ier_spi_mode::W) writer structure"]
impl crate::Writable for IER_SPI_MODE {}
#[doc = "Interrupt Enable Register"]
pub mod ier_spi_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_lin_mode](ier_lin_mode) module"]
pub type IER_LIN_MODE = crate::Reg<u32, _IER_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_LIN_MODE;
#[doc = "`write(|w| ..)` method takes [ier_lin_mode::W](ier_lin_mode::W) writer structure"]
impl crate::Writable for IER_LIN_MODE {}
#[doc = "Interrupt Enable Register"]
pub mod ier_lin_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier_lon_mode](ier_lon_mode) module"]
pub type IER_LON_MODE = crate::Reg<u32, _IER_LON_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_LON_MODE;
#[doc = "`write(|w| ..)` method takes [ier_lon_mode::W](ier_lon_mode::W) writer structure"]
impl crate::Writable for IER_LON_MODE {}
#[doc = "Interrupt Enable Register"]
pub mod ier_lon_mode;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_spi_mode](idr_spi_mode) module"]
pub type IDR_SPI_MODE = crate::Reg<u32, _IDR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [idr_spi_mode::W](idr_spi_mode::W) writer structure"]
impl crate::Writable for IDR_SPI_MODE {}
#[doc = "Interrupt Disable Register"]
pub mod idr_spi_mode;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_lin_mode](idr_lin_mode) module"]
pub type IDR_LIN_MODE = crate::Reg<u32, _IDR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_LIN_MODE;
#[doc = "`write(|w| ..)` method takes [idr_lin_mode::W](idr_lin_mode::W) writer structure"]
impl crate::Writable for IDR_LIN_MODE {}
#[doc = "Interrupt Disable Register"]
pub mod idr_lin_mode;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr_lon_mode](idr_lon_mode) module"]
pub type IDR_LON_MODE = crate::Reg<u32, _IDR_LON_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_LON_MODE;
#[doc = "`write(|w| ..)` method takes [idr_lon_mode::W](idr_lon_mode::W) writer structure"]
impl crate::Writable for IDR_LON_MODE {}
#[doc = "Interrupt Disable Register"]
pub mod idr_lon_mode;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr_spi_mode](imr_spi_mode) module"]
pub type IMR_SPI_MODE = crate::Reg<u32, _IMR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_SPI_MODE;
#[doc = "`read()` method returns [imr_spi_mode::R](imr_spi_mode::R) reader structure"]
impl crate::Readable for IMR_SPI_MODE {}
#[doc = "Interrupt Mask Register"]
pub mod imr_spi_mode;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr_lin_mode](imr_lin_mode) module"]
pub type IMR_LIN_MODE = crate::Reg<u32, _IMR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_LIN_MODE;
#[doc = "`read()` method returns [imr_lin_mode::R](imr_lin_mode::R) reader structure"]
impl crate::Readable for IMR_LIN_MODE {}
#[doc = "Interrupt Mask Register"]
pub mod imr_lin_mode;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr_lon_mode](imr_lon_mode) module"]
pub type IMR_LON_MODE = crate::Reg<u32, _IMR_LON_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_LON_MODE;
#[doc = "`read()` method returns [imr_lon_mode::R](imr_lon_mode::R) reader structure"]
impl crate::Readable for IMR_LON_MODE {}
#[doc = "Interrupt Mask Register"]
pub mod imr_lon_mode;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_spi_mode](csr_spi_mode) module"]
pub type CSR_SPI_MODE = crate::Reg<u32, _CSR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_SPI_MODE;
#[doc = "`read()` method returns [csr_spi_mode::R](csr_spi_mode::R) reader structure"]
impl crate::Readable for CSR_SPI_MODE {}
#[doc = "Channel Status Register"]
pub mod csr_spi_mode;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_lin_mode](csr_lin_mode) module"]
pub type CSR_LIN_MODE = crate::Reg<u32, _CSR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_LIN_MODE;
#[doc = "`read()` method returns [csr_lin_mode::R](csr_lin_mode::R) reader structure"]
impl crate::Readable for CSR_LIN_MODE {}
#[doc = "Channel Status Register"]
pub mod csr_lin_mode;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr_lon_mode](csr_lon_mode) module"]
pub type CSR_LON_MODE = crate::Reg<u32, _CSR_LON_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_LON_MODE;
#[doc = "`read()` method returns [csr_lon_mode::R](csr_lon_mode::R) reader structure"]
impl crate::Readable for CSR_LON_MODE {}
#[doc = "Channel Status Register"]
pub mod csr_lon_mode;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](brgr) module"]
pub type BRGR = crate::Reg<u32, _BRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGR;
#[doc = "`read()` method returns [brgr::R](brgr::R) reader structure"]
impl crate::Readable for BRGR {}
#[doc = "`write(|w| ..)` method takes [brgr::W](brgr::W) writer structure"]
impl crate::Writable for BRGR {}
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](rtor) module"]
pub type RTOR = crate::Reg<u32, _RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTOR;
#[doc = "`read()` method returns [rtor::R](rtor::R) reader structure"]
impl crate::Readable for RTOR {}
#[doc = "`write(|w| ..)` method takes [rtor::W](rtor::W) writer structure"]
impl crate::Writable for RTOR {}
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgr](ttgr) module"]
pub type TTGR = crate::Reg<u32, _TTGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTGR;
#[doc = "`read()` method returns [ttgr::R](ttgr::R) reader structure"]
impl crate::Readable for TTGR {}
#[doc = "`write(|w| ..)` method takes [ttgr::W](ttgr::W) writer structure"]
impl crate::Writable for TTGR {}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttgr_lon_mode](ttgr_lon_mode) module"]
pub type TTGR_LON_MODE = crate::Reg<u32, _TTGR_LON_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTGR_LON_MODE;
#[doc = "`read()` method returns [ttgr_lon_mode::R](ttgr_lon_mode::R) reader structure"]
impl crate::Readable for TTGR_LON_MODE {}
#[doc = "`write(|w| ..)` method takes [ttgr_lon_mode::W](ttgr_lon_mode::W) writer structure"]
impl crate::Writable for TTGR_LON_MODE {}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr_lon_mode;
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "LIN Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linmr](linmr) module"]
pub type LINMR = crate::Reg<u32, _LINMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINMR;
#[doc = "`read()` method returns [linmr::R](linmr::R) reader structure"]
impl crate::Readable for LINMR {}
#[doc = "`write(|w| ..)` method takes [linmr::W](linmr::W) writer structure"]
impl crate::Writable for LINMR {}
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linir](linir) module"]
pub type LINIR = crate::Reg<u32, _LINIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINIR;
#[doc = "`read()` method returns [linir::R](linir::R) reader structure"]
impl crate::Readable for LINIR {}
#[doc = "`write(|w| ..)` method takes [linir::W](linir::W) writer structure"]
impl crate::Writable for LINIR {}
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbrr](linbrr) module"]
pub type LINBRR = crate::Reg<u32, _LINBRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINBRR;
#[doc = "`read()` method returns [linbrr::R](linbrr::R) reader structure"]
impl crate::Readable for LINBRR {}
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "LON Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonmr](lonmr) module"]
pub type LONMR = crate::Reg<u32, _LONMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONMR;
#[doc = "`read()` method returns [lonmr::R](lonmr::R) reader structure"]
impl crate::Readable for LONMR {}
#[doc = "`write(|w| ..)` method takes [lonmr::W](lonmr::W) writer structure"]
impl crate::Writable for LONMR {}
#[doc = "LON Mode Register"]
pub mod lonmr;
#[doc = "LON Preamble Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonpr](lonpr) module"]
pub type LONPR = crate::Reg<u32, _LONPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONPR;
#[doc = "`read()` method returns [lonpr::R](lonpr::R) reader structure"]
impl crate::Readable for LONPR {}
#[doc = "`write(|w| ..)` method takes [lonpr::W](lonpr::W) writer structure"]
impl crate::Writable for LONPR {}
#[doc = "LON Preamble Register"]
pub mod lonpr;
#[doc = "LON Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [londl](londl) module"]
pub type LONDL = crate::Reg<u32, _LONDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONDL;
#[doc = "`read()` method returns [londl::R](londl::R) reader structure"]
impl crate::Readable for LONDL {}
#[doc = "`write(|w| ..)` method takes [londl::W](londl::W) writer structure"]
impl crate::Writable for LONDL {}
#[doc = "LON Data Length Register"]
pub mod londl;
#[doc = "LON L2HDR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonl2hdr](lonl2hdr) module"]
pub type LONL2HDR = crate::Reg<u32, _LONL2HDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONL2HDR;
#[doc = "`read()` method returns [lonl2hdr::R](lonl2hdr::R) reader structure"]
impl crate::Readable for LONL2HDR {}
#[doc = "`write(|w| ..)` method takes [lonl2hdr::W](lonl2hdr::W) writer structure"]
impl crate::Writable for LONL2HDR {}
#[doc = "LON L2HDR Register"]
pub mod lonl2hdr;
#[doc = "LON Backlog Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonbl](lonbl) module"]
pub type LONBL = crate::Reg<u32, _LONBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONBL;
#[doc = "`read()` method returns [lonbl::R](lonbl::R) reader structure"]
impl crate::Readable for LONBL {}
#[doc = "LON Backlog Register"]
pub mod lonbl;
#[doc = "LON Beta1 Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonb1tx](lonb1tx) module"]
pub type LONB1TX = crate::Reg<u32, _LONB1TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONB1TX;
#[doc = "`read()` method returns [lonb1tx::R](lonb1tx::R) reader structure"]
impl crate::Readable for LONB1TX {}
#[doc = "`write(|w| ..)` method takes [lonb1tx::W](lonb1tx::W) writer structure"]
impl crate::Writable for LONB1TX {}
#[doc = "LON Beta1 Tx Register"]
pub mod lonb1tx;
#[doc = "LON Beta1 Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonb1rx](lonb1rx) module"]
pub type LONB1RX = crate::Reg<u32, _LONB1RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONB1RX;
#[doc = "`read()` method returns [lonb1rx::R](lonb1rx::R) reader structure"]
impl crate::Readable for LONB1RX {}
#[doc = "`write(|w| ..)` method takes [lonb1rx::W](lonb1rx::W) writer structure"]
impl crate::Writable for LONB1RX {}
#[doc = "LON Beta1 Rx Register"]
pub mod lonb1rx;
#[doc = "LON Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonprio](lonprio) module"]
pub type LONPRIO = crate::Reg<u32, _LONPRIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LONPRIO;
#[doc = "`read()` method returns [lonprio::R](lonprio::R) reader structure"]
impl crate::Readable for LONPRIO {}
#[doc = "`write(|w| ..)` method takes [lonprio::W](lonprio::W) writer structure"]
impl crate::Writable for LONPRIO {}
#[doc = "LON Priority Register"]
pub mod lonprio;
#[doc = "LON IDT Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idttx](idttx) module"]
pub type IDTTX = crate::Reg<u32, _IDTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDTTX;
#[doc = "`read()` method returns [idttx::R](idttx::R) reader structure"]
impl crate::Readable for IDTTX {}
#[doc = "`write(|w| ..)` method takes [idttx::W](idttx::W) writer structure"]
impl crate::Writable for IDTTX {}
#[doc = "LON IDT Tx Register"]
pub mod idttx;
#[doc = "LON IDT Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idtrx](idtrx) module"]
pub type IDTRX = crate::Reg<u32, _IDTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDTRX;
#[doc = "`read()` method returns [idtrx::R](idtrx::R) reader structure"]
impl crate::Readable for IDTRX {}
#[doc = "`write(|w| ..)` method takes [idtrx::W](idtrx::W) writer structure"]
impl crate::Writable for IDTRX {}
#[doc = "LON IDT Rx Register"]
pub mod idtrx;
#[doc = "IC DIFF Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icdiff](icdiff) module"]
pub type ICDIFF = crate::Reg<u32, _ICDIFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICDIFF;
#[doc = "`read()` method returns [icdiff::R](icdiff::R) reader structure"]
impl crate::Readable for ICDIFF {}
#[doc = "`write(|w| ..)` method takes [icdiff::W](icdiff::W) writer structure"]
impl crate::Writable for ICDIFF {}
#[doc = "IC DIFF Register"]
pub mod icdiff;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
