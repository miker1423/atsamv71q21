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
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x1c - Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: crate::Reg<brgr::BRGR_SPEC>,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    _reserved_10_ttgr: [u8; 4usize],
    _reserved11: [u8; 36usize],
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: crate::Reg<man::MAN_SPEC>,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: crate::Reg<linmr::LINMR_SPEC>,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: crate::Reg<linir::LINIR_SPEC>,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: crate::Reg<linbrr::LINBRR_SPEC>,
    #[doc = "0x60 - LON Mode Register"]
    pub lonmr: crate::Reg<lonmr::LONMR_SPEC>,
    #[doc = "0x64 - LON Preamble Register"]
    pub lonpr: crate::Reg<lonpr::LONPR_SPEC>,
    #[doc = "0x68 - LON Data Length Register"]
    pub londl: crate::Reg<londl::LONDL_SPEC>,
    #[doc = "0x6c - LON L2HDR Register"]
    pub lonl2hdr: crate::Reg<lonl2hdr::LONL2HDR_SPEC>,
    #[doc = "0x70 - LON Backlog Register"]
    pub lonbl: crate::Reg<lonbl::LONBL_SPEC>,
    #[doc = "0x74 - LON Beta1 Tx Register"]
    pub lonb1tx: crate::Reg<lonb1tx::LONB1TX_SPEC>,
    #[doc = "0x78 - LON Beta1 Rx Register"]
    pub lonb1rx: crate::Reg<lonb1rx::LONB1RX_SPEC>,
    #[doc = "0x7c - LON Priority Register"]
    pub lonprio: crate::Reg<lonprio::LONPRIO_SPEC>,
    #[doc = "0x80 - LON IDT Tx Register"]
    pub idttx: crate::Reg<idttx::IDTTX_SPEC>,
    #[doc = "0x84 - LON IDT Rx Register"]
    pub idtrx: crate::Reg<idtrx::IDTRX_SPEC>,
    #[doc = "0x88 - IC DIFF Register"]
    pub icdiff: crate::Reg<icdiff::ICDIFF_SPEC>,
    _reserved26: [u8; 88usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn spi_mode_cr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &crate::Reg<cr::CR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<cr::CR_SPEC>)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn spi_mode_mr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &crate::Reg<mr::MR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<mr::MR_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn lon_mode_ier_lon_mode(
        &self,
    ) -> &crate::Reg<lon_mode_ier_lon_mode::LON_MODE_IER_LON_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<lon_mode_ier_lon_mode::LON_MODE_IER_LON_MODE_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn lin_mode_ier_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn spi_mode_ier_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn lon_mode_idr_lon_mode(
        &self,
    ) -> &crate::Reg<lon_mode_idr_lon_mode::LON_MODE_IDR_LON_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<lon_mode_idr_lon_mode::LON_MODE_IDR_LON_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn lin_mode_idr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn spi_mode_idr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &crate::Reg<idr::IDR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<idr::IDR_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn lon_mode_imr_lon_mode(
        &self,
    ) -> &crate::Reg<lon_mode_imr_lon_mode::LON_MODE_IMR_LON_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<lon_mode_imr_lon_mode::LON_MODE_IMR_LON_MODE_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn lin_mode_imr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn spi_mode_imr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &crate::Reg<imr::IMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<imr::IMR_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn lon_mode_csr_lon_mode(
        &self,
    ) -> &crate::Reg<lon_mode_csr_lon_mode::LON_MODE_CSR_LON_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<lon_mode_csr_lon_mode::LON_MODE_CSR_LON_MODE_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn lin_mode_csr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn spi_mode_csr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &crate::Reg<csr::CSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<csr::CSR_SPEC>)
        }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn lon_mode_ttgr_lon_mode(
        &self,
    ) -> &crate::Reg<lon_mode_ttgr_lon_mode::LON_MODE_TTGR_LON_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<lon_mode_ttgr_lon_mode::LON_MODE_TTGR_LON_MODE_SPEC>)
        }
    }
    #[doc = "0x28 - Transmitter Timeguard Register"]
    #[inline(always)]
    pub fn ttgr(&self) -> &crate::Reg<ttgr::TTGR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<ttgr::TTGR_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SPI_MODE_CR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_CR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CR_SPI_MODE = crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>;
#[doc = "Control Register"]
pub mod spi_mode_cr_spi_mode;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SPI_MODE_MR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_MR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_MR_SPI_MODE = crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "Mode Register"]
pub mod spi_mode_mr_spi_mode;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "SPI_MODE_IER_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IER_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IER_SPI_MODE = crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod spi_mode_ier_spi_mode;
#[doc = "LIN_MODE_IER_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IER_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IER_LIN_MODE = crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lin_mode_ier_lin_mode;
#[doc = "LON_MODE_IER_LON_MODE register accessor: an alias for `Reg<LON_MODE_IER_LON_MODE_SPEC>`"]
pub type LON_MODE_IER_LON_MODE = crate::Reg<lon_mode_ier_lon_mode::LON_MODE_IER_LON_MODE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod lon_mode_ier_lon_mode;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "SPI_MODE_IDR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IDR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IDR_SPI_MODE = crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod spi_mode_idr_spi_mode;
#[doc = "LIN_MODE_IDR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IDR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IDR_LIN_MODE = crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod lin_mode_idr_lin_mode;
#[doc = "LON_MODE_IDR_LON_MODE register accessor: an alias for `Reg<LON_MODE_IDR_LON_MODE_SPEC>`"]
pub type LON_MODE_IDR_LON_MODE = crate::Reg<lon_mode_idr_lon_mode::LON_MODE_IDR_LON_MODE_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod lon_mode_idr_lon_mode;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "SPI_MODE_IMR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IMR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IMR_SPI_MODE = crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod spi_mode_imr_spi_mode;
#[doc = "LIN_MODE_IMR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IMR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IMR_LIN_MODE = crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod lin_mode_imr_lin_mode;
#[doc = "LON_MODE_IMR_LON_MODE register accessor: an alias for `Reg<LON_MODE_IMR_LON_MODE_SPEC>`"]
pub type LON_MODE_IMR_LON_MODE = crate::Reg<lon_mode_imr_lon_mode::LON_MODE_IMR_LON_MODE_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod lon_mode_imr_lon_mode;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "SPI_MODE_CSR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_CSR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CSR_SPI_MODE = crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod spi_mode_csr_spi_mode;
#[doc = "LIN_MODE_CSR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_CSR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_CSR_LIN_MODE = crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod lin_mode_csr_lin_mode;
#[doc = "LON_MODE_CSR_LON_MODE register accessor: an alias for `Reg<LON_MODE_CSR_LON_MODE_SPEC>`"]
pub type LON_MODE_CSR_LON_MODE = crate::Reg<lon_mode_csr_lon_mode::LON_MODE_CSR_LON_MODE_SPEC>;
#[doc = "Channel Status Register"]
pub mod lon_mode_csr_lon_mode;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "BRGR register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "RTOR register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "TTGR register accessor: an alias for `Reg<TTGR_SPEC>`"]
pub type TTGR = crate::Reg<ttgr::TTGR_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "LON_MODE_TTGR_LON_MODE register accessor: an alias for `Reg<LON_MODE_TTGR_LON_MODE_SPEC>`"]
pub type LON_MODE_TTGR_LON_MODE = crate::Reg<lon_mode_ttgr_lon_mode::LON_MODE_TTGR_LON_MODE_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod lon_mode_ttgr_lon_mode;
#[doc = "MAN register accessor: an alias for `Reg<MAN_SPEC>`"]
pub type MAN = crate::Reg<man::MAN_SPEC>;
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "LINMR register accessor: an alias for `Reg<LINMR_SPEC>`"]
pub type LINMR = crate::Reg<linmr::LINMR_SPEC>;
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LINIR register accessor: an alias for `Reg<LINIR_SPEC>`"]
pub type LINIR = crate::Reg<linir::LINIR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LINBRR register accessor: an alias for `Reg<LINBRR_SPEC>`"]
pub type LINBRR = crate::Reg<linbrr::LINBRR_SPEC>;
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "LONMR register accessor: an alias for `Reg<LONMR_SPEC>`"]
pub type LONMR = crate::Reg<lonmr::LONMR_SPEC>;
#[doc = "LON Mode Register"]
pub mod lonmr;
#[doc = "LONPR register accessor: an alias for `Reg<LONPR_SPEC>`"]
pub type LONPR = crate::Reg<lonpr::LONPR_SPEC>;
#[doc = "LON Preamble Register"]
pub mod lonpr;
#[doc = "LONDL register accessor: an alias for `Reg<LONDL_SPEC>`"]
pub type LONDL = crate::Reg<londl::LONDL_SPEC>;
#[doc = "LON Data Length Register"]
pub mod londl;
#[doc = "LONL2HDR register accessor: an alias for `Reg<LONL2HDR_SPEC>`"]
pub type LONL2HDR = crate::Reg<lonl2hdr::LONL2HDR_SPEC>;
#[doc = "LON L2HDR Register"]
pub mod lonl2hdr;
#[doc = "LONBL register accessor: an alias for `Reg<LONBL_SPEC>`"]
pub type LONBL = crate::Reg<lonbl::LONBL_SPEC>;
#[doc = "LON Backlog Register"]
pub mod lonbl;
#[doc = "LONB1TX register accessor: an alias for `Reg<LONB1TX_SPEC>`"]
pub type LONB1TX = crate::Reg<lonb1tx::LONB1TX_SPEC>;
#[doc = "LON Beta1 Tx Register"]
pub mod lonb1tx;
#[doc = "LONB1RX register accessor: an alias for `Reg<LONB1RX_SPEC>`"]
pub type LONB1RX = crate::Reg<lonb1rx::LONB1RX_SPEC>;
#[doc = "LON Beta1 Rx Register"]
pub mod lonb1rx;
#[doc = "LONPRIO register accessor: an alias for `Reg<LONPRIO_SPEC>`"]
pub type LONPRIO = crate::Reg<lonprio::LONPRIO_SPEC>;
#[doc = "LON Priority Register"]
pub mod lonprio;
#[doc = "IDTTX register accessor: an alias for `Reg<IDTTX_SPEC>`"]
pub type IDTTX = crate::Reg<idttx::IDTTX_SPEC>;
#[doc = "LON IDT Tx Register"]
pub mod idttx;
#[doc = "IDTRX register accessor: an alias for `Reg<IDTRX_SPEC>`"]
pub type IDTRX = crate::Reg<idtrx::IDTRX_SPEC>;
#[doc = "LON IDT Rx Register"]
pub mod idtrx;
#[doc = "ICDIFF register accessor: an alias for `Reg<ICDIFF_SPEC>`"]
pub type ICDIFF = crate::Reg<icdiff::ICDIFF_SPEC>;
#[doc = "IC DIFF Register"]
pub mod icdiff;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
