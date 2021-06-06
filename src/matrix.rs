#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>; 12],
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>; 9],
    _reserved2: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>,
    #[doc = "0x84 - Priority Register B for Slave 0"]
    pub matrix_prbs0: crate::Reg<matrix_prbs0::MATRIX_PRBS0_SPEC>,
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>,
    #[doc = "0x8c - Priority Register B for Slave 1"]
    pub matrix_prbs1: crate::Reg<matrix_prbs1::MATRIX_PRBS1_SPEC>,
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>,
    #[doc = "0x94 - Priority Register B for Slave 2"]
    pub matrix_prbs2: crate::Reg<matrix_prbs2::MATRIX_PRBS2_SPEC>,
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>,
    #[doc = "0x9c - Priority Register B for Slave 3"]
    pub matrix_prbs3: crate::Reg<matrix_prbs3::MATRIX_PRBS3_SPEC>,
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>,
    #[doc = "0xa4 - Priority Register B for Slave 4"]
    pub matrix_prbs4: crate::Reg<matrix_prbs4::MATRIX_PRBS4_SPEC>,
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub matrix_pras5: crate::Reg<matrix_pras5::MATRIX_PRAS5_SPEC>,
    #[doc = "0xac - Priority Register B for Slave 5"]
    pub matrix_prbs5: crate::Reg<matrix_prbs5::MATRIX_PRBS5_SPEC>,
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub matrix_pras6: crate::Reg<matrix_pras6::MATRIX_PRAS6_SPEC>,
    #[doc = "0xb4 - Priority Register B for Slave 6"]
    pub matrix_prbs6: crate::Reg<matrix_prbs6::MATRIX_PRBS6_SPEC>,
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub matrix_pras7: crate::Reg<matrix_pras7::MATRIX_PRAS7_SPEC>,
    #[doc = "0xbc - Priority Register B for Slave 7"]
    pub matrix_prbs7: crate::Reg<matrix_prbs7::MATRIX_PRBS7_SPEC>,
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub matrix_pras8: crate::Reg<matrix_pras8::MATRIX_PRAS8_SPEC>,
    #[doc = "0xc4 - Priority Register B for Slave 8"]
    pub matrix_prbs8: crate::Reg<matrix_prbs8::MATRIX_PRBS8_SPEC>,
    _reserved20: [u8; 56usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>,
    _reserved21: [u8; 12usize],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    _reserved23: [u8; 12usize],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>,
    _reserved24: [u8; 188usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>,
}
#[doc = "MATRIX_MCFG register accessor: an alias for `Reg<MATRIX_MCFG_SPEC>`"]
pub type MATRIX_MCFG = crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG register accessor: an alias for `Reg<MATRIX_SCFG_SPEC>`"]
pub type MATRIX_SCFG = crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 register accessor: an alias for `Reg<MATRIX_PRAS0_SPEC>`"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRBS0 register accessor: an alias for `Reg<MATRIX_PRBS0_SPEC>`"]
pub type MATRIX_PRBS0 = crate::Reg<matrix_prbs0::MATRIX_PRBS0_SPEC>;
#[doc = "Priority Register B for Slave 0"]
pub mod matrix_prbs0;
#[doc = "MATRIX_PRAS1 register accessor: an alias for `Reg<MATRIX_PRAS1_SPEC>`"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRBS1 register accessor: an alias for `Reg<MATRIX_PRBS1_SPEC>`"]
pub type MATRIX_PRBS1 = crate::Reg<matrix_prbs1::MATRIX_PRBS1_SPEC>;
#[doc = "Priority Register B for Slave 1"]
pub mod matrix_prbs1;
#[doc = "MATRIX_PRAS2 register accessor: an alias for `Reg<MATRIX_PRAS2_SPEC>`"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRBS2 register accessor: an alias for `Reg<MATRIX_PRBS2_SPEC>`"]
pub type MATRIX_PRBS2 = crate::Reg<matrix_prbs2::MATRIX_PRBS2_SPEC>;
#[doc = "Priority Register B for Slave 2"]
pub mod matrix_prbs2;
#[doc = "MATRIX_PRAS3 register accessor: an alias for `Reg<MATRIX_PRAS3_SPEC>`"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "MATRIX_PRBS3 register accessor: an alias for `Reg<MATRIX_PRBS3_SPEC>`"]
pub type MATRIX_PRBS3 = crate::Reg<matrix_prbs3::MATRIX_PRBS3_SPEC>;
#[doc = "Priority Register B for Slave 3"]
pub mod matrix_prbs3;
#[doc = "MATRIX_PRAS4 register accessor: an alias for `Reg<MATRIX_PRAS4_SPEC>`"]
pub type MATRIX_PRAS4 = crate::Reg<matrix_pras4::MATRIX_PRAS4_SPEC>;
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "MATRIX_PRBS4 register accessor: an alias for `Reg<MATRIX_PRBS4_SPEC>`"]
pub type MATRIX_PRBS4 = crate::Reg<matrix_prbs4::MATRIX_PRBS4_SPEC>;
#[doc = "Priority Register B for Slave 4"]
pub mod matrix_prbs4;
#[doc = "MATRIX_PRAS5 register accessor: an alias for `Reg<MATRIX_PRAS5_SPEC>`"]
pub type MATRIX_PRAS5 = crate::Reg<matrix_pras5::MATRIX_PRAS5_SPEC>;
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "MATRIX_PRBS5 register accessor: an alias for `Reg<MATRIX_PRBS5_SPEC>`"]
pub type MATRIX_PRBS5 = crate::Reg<matrix_prbs5::MATRIX_PRBS5_SPEC>;
#[doc = "Priority Register B for Slave 5"]
pub mod matrix_prbs5;
#[doc = "MATRIX_PRAS6 register accessor: an alias for `Reg<MATRIX_PRAS6_SPEC>`"]
pub type MATRIX_PRAS6 = crate::Reg<matrix_pras6::MATRIX_PRAS6_SPEC>;
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "MATRIX_PRBS6 register accessor: an alias for `Reg<MATRIX_PRBS6_SPEC>`"]
pub type MATRIX_PRBS6 = crate::Reg<matrix_prbs6::MATRIX_PRBS6_SPEC>;
#[doc = "Priority Register B for Slave 6"]
pub mod matrix_prbs6;
#[doc = "MATRIX_PRAS7 register accessor: an alias for `Reg<MATRIX_PRAS7_SPEC>`"]
pub type MATRIX_PRAS7 = crate::Reg<matrix_pras7::MATRIX_PRAS7_SPEC>;
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "MATRIX_PRBS7 register accessor: an alias for `Reg<MATRIX_PRBS7_SPEC>`"]
pub type MATRIX_PRBS7 = crate::Reg<matrix_prbs7::MATRIX_PRBS7_SPEC>;
#[doc = "Priority Register B for Slave 7"]
pub mod matrix_prbs7;
#[doc = "MATRIX_PRAS8 register accessor: an alias for `Reg<MATRIX_PRAS8_SPEC>`"]
pub type MATRIX_PRAS8 = crate::Reg<matrix_pras8::MATRIX_PRAS8_SPEC>;
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "MATRIX_PRBS8 register accessor: an alias for `Reg<MATRIX_PRBS8_SPEC>`"]
pub type MATRIX_PRBS8 = crate::Reg<matrix_prbs8::MATRIX_PRBS8_SPEC>;
#[doc = "Priority Register B for Slave 8"]
pub mod matrix_prbs8;
#[doc = "MATRIX_MRCR register accessor: an alias for `Reg<MATRIX_MRCR_SPEC>`"]
pub type MATRIX_MRCR = crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CCFG_CAN0 register accessor: an alias for `Reg<CCFG_CAN0_SPEC>`"]
pub type CCFG_CAN0 = crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>;
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS register accessor: an alias for `Reg<CCFG_SMCNFCS_SPEC>`"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "MATRIX_WPMR register accessor: an alias for `Reg<MATRIX_WPMR_SPEC>`"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR register accessor: an alias for `Reg<MATRIX_WPSR_SPEC>`"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
