#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 12],
    _reserved1: [u8; 16usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 9],
    _reserved2: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    #[doc = "0x84 - Priority Register B for Slave 0"]
    pub matrix_prbs0: MATRIX_PRBS0,
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    #[doc = "0x8c - Priority Register B for Slave 1"]
    pub matrix_prbs1: MATRIX_PRBS1,
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    #[doc = "0x94 - Priority Register B for Slave 2"]
    pub matrix_prbs2: MATRIX_PRBS2,
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    #[doc = "0x9c - Priority Register B for Slave 3"]
    pub matrix_prbs3: MATRIX_PRBS3,
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    #[doc = "0xa4 - Priority Register B for Slave 4"]
    pub matrix_prbs4: MATRIX_PRBS4,
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub matrix_pras5: MATRIX_PRAS5,
    #[doc = "0xac - Priority Register B for Slave 5"]
    pub matrix_prbs5: MATRIX_PRBS5,
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub matrix_pras6: MATRIX_PRAS6,
    #[doc = "0xb4 - Priority Register B for Slave 6"]
    pub matrix_prbs6: MATRIX_PRBS6,
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub matrix_pras7: MATRIX_PRAS7,
    #[doc = "0xbc - Priority Register B for Slave 7"]
    pub matrix_prbs7: MATRIX_PRBS7,
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub matrix_pras8: MATRIX_PRAS8,
    #[doc = "0xc4 - Priority Register B for Slave 8"]
    pub matrix_prbs8: MATRIX_PRBS8,
    _reserved20: [u8; 56usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: MATRIX_MRCR,
    _reserved21: [u8; 12usize],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: CCFG_CAN0,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved23: [u8; 12usize],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved24: [u8; 188usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](matrix_mcfg) module"]
pub type MATRIX_MCFG = crate::Reg<u32, _MATRIX_MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_MCFG;
#[doc = "`read()` method returns [matrix_mcfg::R](matrix_mcfg::R) reader structure"]
impl crate::Readable for MATRIX_MCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](matrix_mcfg::W) writer structure"]
impl crate::Writable for MATRIX_MCFG {}
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](matrix_scfg) module"]
pub type MATRIX_SCFG = crate::Reg<u32, _MATRIX_SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_SCFG;
#[doc = "`read()` method returns [matrix_scfg::R](matrix_scfg::R) reader structure"]
impl crate::Readable for MATRIX_SCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](matrix_scfg::W) writer structure"]
impl crate::Writable for MATRIX_SCFG {}
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "Priority Register A for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras0](matrix_pras0) module"]
pub type MATRIX_PRAS0 = crate::Reg<u32, _MATRIX_PRAS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS0;
#[doc = "`read()` method returns [matrix_pras0::R](matrix_pras0::R) reader structure"]
impl crate::Readable for MATRIX_PRAS0 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras0::W](matrix_pras0::W) writer structure"]
impl crate::Writable for MATRIX_PRAS0 {}
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "Priority Register B for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs0](matrix_prbs0) module"]
pub type MATRIX_PRBS0 = crate::Reg<u32, _MATRIX_PRBS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS0;
#[doc = "`read()` method returns [matrix_prbs0::R](matrix_prbs0::R) reader structure"]
impl crate::Readable for MATRIX_PRBS0 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs0::W](matrix_prbs0::W) writer structure"]
impl crate::Writable for MATRIX_PRBS0 {}
#[doc = "Priority Register B for Slave 0"]
pub mod matrix_prbs0;
#[doc = "Priority Register A for Slave 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras1](matrix_pras1) module"]
pub type MATRIX_PRAS1 = crate::Reg<u32, _MATRIX_PRAS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS1;
#[doc = "`read()` method returns [matrix_pras1::R](matrix_pras1::R) reader structure"]
impl crate::Readable for MATRIX_PRAS1 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras1::W](matrix_pras1::W) writer structure"]
impl crate::Writable for MATRIX_PRAS1 {}
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "Priority Register B for Slave 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs1](matrix_prbs1) module"]
pub type MATRIX_PRBS1 = crate::Reg<u32, _MATRIX_PRBS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS1;
#[doc = "`read()` method returns [matrix_prbs1::R](matrix_prbs1::R) reader structure"]
impl crate::Readable for MATRIX_PRBS1 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs1::W](matrix_prbs1::W) writer structure"]
impl crate::Writable for MATRIX_PRBS1 {}
#[doc = "Priority Register B for Slave 1"]
pub mod matrix_prbs1;
#[doc = "Priority Register A for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras2](matrix_pras2) module"]
pub type MATRIX_PRAS2 = crate::Reg<u32, _MATRIX_PRAS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS2;
#[doc = "`read()` method returns [matrix_pras2::R](matrix_pras2::R) reader structure"]
impl crate::Readable for MATRIX_PRAS2 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras2::W](matrix_pras2::W) writer structure"]
impl crate::Writable for MATRIX_PRAS2 {}
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "Priority Register B for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs2](matrix_prbs2) module"]
pub type MATRIX_PRBS2 = crate::Reg<u32, _MATRIX_PRBS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS2;
#[doc = "`read()` method returns [matrix_prbs2::R](matrix_prbs2::R) reader structure"]
impl crate::Readable for MATRIX_PRBS2 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs2::W](matrix_prbs2::W) writer structure"]
impl crate::Writable for MATRIX_PRBS2 {}
#[doc = "Priority Register B for Slave 2"]
pub mod matrix_prbs2;
#[doc = "Priority Register A for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras3](matrix_pras3) module"]
pub type MATRIX_PRAS3 = crate::Reg<u32, _MATRIX_PRAS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS3;
#[doc = "`read()` method returns [matrix_pras3::R](matrix_pras3::R) reader structure"]
impl crate::Readable for MATRIX_PRAS3 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras3::W](matrix_pras3::W) writer structure"]
impl crate::Writable for MATRIX_PRAS3 {}
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "Priority Register B for Slave 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs3](matrix_prbs3) module"]
pub type MATRIX_PRBS3 = crate::Reg<u32, _MATRIX_PRBS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS3;
#[doc = "`read()` method returns [matrix_prbs3::R](matrix_prbs3::R) reader structure"]
impl crate::Readable for MATRIX_PRBS3 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs3::W](matrix_prbs3::W) writer structure"]
impl crate::Writable for MATRIX_PRBS3 {}
#[doc = "Priority Register B for Slave 3"]
pub mod matrix_prbs3;
#[doc = "Priority Register A for Slave 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras4](matrix_pras4) module"]
pub type MATRIX_PRAS4 = crate::Reg<u32, _MATRIX_PRAS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS4;
#[doc = "`read()` method returns [matrix_pras4::R](matrix_pras4::R) reader structure"]
impl crate::Readable for MATRIX_PRAS4 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras4::W](matrix_pras4::W) writer structure"]
impl crate::Writable for MATRIX_PRAS4 {}
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "Priority Register B for Slave 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs4](matrix_prbs4) module"]
pub type MATRIX_PRBS4 = crate::Reg<u32, _MATRIX_PRBS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS4;
#[doc = "`read()` method returns [matrix_prbs4::R](matrix_prbs4::R) reader structure"]
impl crate::Readable for MATRIX_PRBS4 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs4::W](matrix_prbs4::W) writer structure"]
impl crate::Writable for MATRIX_PRBS4 {}
#[doc = "Priority Register B for Slave 4"]
pub mod matrix_prbs4;
#[doc = "Priority Register A for Slave 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras5](matrix_pras5) module"]
pub type MATRIX_PRAS5 = crate::Reg<u32, _MATRIX_PRAS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS5;
#[doc = "`read()` method returns [matrix_pras5::R](matrix_pras5::R) reader structure"]
impl crate::Readable for MATRIX_PRAS5 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras5::W](matrix_pras5::W) writer structure"]
impl crate::Writable for MATRIX_PRAS5 {}
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "Priority Register B for Slave 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs5](matrix_prbs5) module"]
pub type MATRIX_PRBS5 = crate::Reg<u32, _MATRIX_PRBS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS5;
#[doc = "`read()` method returns [matrix_prbs5::R](matrix_prbs5::R) reader structure"]
impl crate::Readable for MATRIX_PRBS5 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs5::W](matrix_prbs5::W) writer structure"]
impl crate::Writable for MATRIX_PRBS5 {}
#[doc = "Priority Register B for Slave 5"]
pub mod matrix_prbs5;
#[doc = "Priority Register A for Slave 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras6](matrix_pras6) module"]
pub type MATRIX_PRAS6 = crate::Reg<u32, _MATRIX_PRAS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS6;
#[doc = "`read()` method returns [matrix_pras6::R](matrix_pras6::R) reader structure"]
impl crate::Readable for MATRIX_PRAS6 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras6::W](matrix_pras6::W) writer structure"]
impl crate::Writable for MATRIX_PRAS6 {}
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "Priority Register B for Slave 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs6](matrix_prbs6) module"]
pub type MATRIX_PRBS6 = crate::Reg<u32, _MATRIX_PRBS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS6;
#[doc = "`read()` method returns [matrix_prbs6::R](matrix_prbs6::R) reader structure"]
impl crate::Readable for MATRIX_PRBS6 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs6::W](matrix_prbs6::W) writer structure"]
impl crate::Writable for MATRIX_PRBS6 {}
#[doc = "Priority Register B for Slave 6"]
pub mod matrix_prbs6;
#[doc = "Priority Register A for Slave 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras7](matrix_pras7) module"]
pub type MATRIX_PRAS7 = crate::Reg<u32, _MATRIX_PRAS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS7;
#[doc = "`read()` method returns [matrix_pras7::R](matrix_pras7::R) reader structure"]
impl crate::Readable for MATRIX_PRAS7 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras7::W](matrix_pras7::W) writer structure"]
impl crate::Writable for MATRIX_PRAS7 {}
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "Priority Register B for Slave 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs7](matrix_prbs7) module"]
pub type MATRIX_PRBS7 = crate::Reg<u32, _MATRIX_PRBS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS7;
#[doc = "`read()` method returns [matrix_prbs7::R](matrix_prbs7::R) reader structure"]
impl crate::Readable for MATRIX_PRBS7 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs7::W](matrix_prbs7::W) writer structure"]
impl crate::Writable for MATRIX_PRBS7 {}
#[doc = "Priority Register B for Slave 7"]
pub mod matrix_prbs7;
#[doc = "Priority Register A for Slave 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras8](matrix_pras8) module"]
pub type MATRIX_PRAS8 = crate::Reg<u32, _MATRIX_PRAS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS8;
#[doc = "`read()` method returns [matrix_pras8::R](matrix_pras8::R) reader structure"]
impl crate::Readable for MATRIX_PRAS8 {}
#[doc = "`write(|w| ..)` method takes [matrix_pras8::W](matrix_pras8::W) writer structure"]
impl crate::Writable for MATRIX_PRAS8 {}
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "Priority Register B for Slave 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs8](matrix_prbs8) module"]
pub type MATRIX_PRBS8 = crate::Reg<u32, _MATRIX_PRBS8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS8;
#[doc = "`read()` method returns [matrix_prbs8::R](matrix_prbs8::R) reader structure"]
impl crate::Readable for MATRIX_PRBS8 {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs8::W](matrix_prbs8::W) writer structure"]
impl crate::Writable for MATRIX_PRBS8 {}
#[doc = "Priority Register B for Slave 8"]
pub mod matrix_prbs8;
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mrcr](matrix_mrcr) module"]
pub type MATRIX_MRCR = crate::Reg<u32, _MATRIX_MRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_MRCR;
#[doc = "`read()` method returns [matrix_mrcr::R](matrix_mrcr::R) reader structure"]
impl crate::Readable for MATRIX_MRCR {}
#[doc = "`write(|w| ..)` method takes [matrix_mrcr::W](matrix_mrcr::W) writer structure"]
impl crate::Writable for MATRIX_MRCR {}
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CAN0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_can0](ccfg_can0) module"]
pub type CCFG_CAN0 = crate::Reg<u32, _CCFG_CAN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_CAN0;
#[doc = "`read()` method returns [ccfg_can0::R](ccfg_can0::R) reader structure"]
impl crate::Readable for CCFG_CAN0 {}
#[doc = "`write(|w| ..)` method takes [ccfg_can0::W](ccfg_can0::W) writer structure"]
impl crate::Writable for CCFG_CAN0 {}
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "System I/O and CAN1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](ccfg_sysio) module"]
pub type CCFG_SYSIO = crate::Reg<u32, _CCFG_SYSIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SYSIO;
#[doc = "`read()` method returns [ccfg_sysio::R](ccfg_sysio::R) reader structure"]
impl crate::Readable for CCFG_SYSIO {}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](ccfg_sysio::W) writer structure"]
impl crate::Writable for CCFG_SYSIO {}
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](ccfg_smcnfcs) module"]
pub type CCFG_SMCNFCS = crate::Reg<u32, _CCFG_SMCNFCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SMCNFCS;
#[doc = "`read()` method returns [ccfg_smcnfcs::R](ccfg_smcnfcs::R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS {}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](ccfg_smcnfcs::W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS {}
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](matrix_wpmr) module"]
pub type MATRIX_WPMR = crate::Reg<u32, _MATRIX_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPMR;
#[doc = "`read()` method returns [matrix_wpmr::R](matrix_wpmr::R) reader structure"]
impl crate::Readable for MATRIX_WPMR {}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](matrix_wpmr::W) writer structure"]
impl crate::Writable for MATRIX_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpsr](matrix_wpsr) module"]
pub type MATRIX_WPSR = crate::Reg<u32, _MATRIX_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPSR;
#[doc = "`read()` method returns [matrix_wpsr::R](matrix_wpsr::R) reader structure"]
impl crate::Readable for MATRIX_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
