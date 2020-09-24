#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MediaLB Control 0 Register"]
    pub mlbc0: MLBC0,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - MediaLB Channel Status 0 Register"]
    pub ms0: MS0,
    _reserved2: [u8; 4usize],
    #[doc = "0x14 - MediaLB Channel Status1 Register"]
    pub ms1: MS1,
    _reserved3: [u8; 8usize],
    #[doc = "0x20 - MediaLB System Status Register"]
    pub mss: MSS,
    #[doc = "0x24 - MediaLB System Data Register"]
    pub msd: MSD,
    _reserved5: [u8; 4usize],
    #[doc = "0x2c - MediaLB Interrupt Enable Register"]
    pub mien: MIEN,
    _reserved6: [u8; 12usize],
    #[doc = "0x3c - MediaLB Control 1 Register"]
    pub mlbc1: MLBC1,
    _reserved7: [u8; 64usize],
    #[doc = "0x80 - HBI Control Register"]
    pub hctl: HCTL,
    _reserved8: [u8; 4usize],
    #[doc = "0x88 - HBI Channel Mask 0 Register"]
    pub hcmr: [HCMR; 2],
    #[doc = "0x90 - HBI Channel Error 0 Register"]
    pub hcer: [HCER; 2],
    #[doc = "0x98 - HBI Channel Busy 0 Register"]
    pub hcbr: [HCBR; 2],
    _reserved11: [u8; 32usize],
    #[doc = "0xc0 - MIF Data 0 Register"]
    pub mdat: [MDAT; 4],
    #[doc = "0xd0 - MIF Data Write Enable 0 Register"]
    pub mdwe: [MDWE; 4],
    #[doc = "0xe0 - MIF Control Register"]
    pub mctl: MCTL,
    #[doc = "0xe4 - MIF Address Register"]
    pub madr: MADR,
    _reserved15: [u8; 728usize],
    #[doc = "0x3c0 - AHB Control Register"]
    pub actl: ACTL,
    _reserved16: [u8; 12usize],
    #[doc = "0x3d0 - AHB Channel Status 0 Register"]
    pub acsr: [ACSR; 2],
    #[doc = "0x3d8 - AHB Channel Mask 0 Register"]
    pub acmr: [ACMR; 2],
}
#[doc = "MediaLB Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlbc0](mlbc0) module"]
pub type MLBC0 = crate::Reg<u32, _MLBC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MLBC0;
#[doc = "`read()` method returns [mlbc0::R](mlbc0::R) reader structure"]
impl crate::Readable for MLBC0 {}
#[doc = "`write(|w| ..)` method takes [mlbc0::W](mlbc0::W) writer structure"]
impl crate::Writable for MLBC0 {}
#[doc = "MediaLB Control 0 Register"]
pub mod mlbc0;
#[doc = "MediaLB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms0](ms0) module"]
pub type MS0 = crate::Reg<u32, _MS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS0;
#[doc = "`read()` method returns [ms0::R](ms0::R) reader structure"]
impl crate::Readable for MS0 {}
#[doc = "`write(|w| ..)` method takes [ms0::W](ms0::W) writer structure"]
impl crate::Writable for MS0 {}
#[doc = "MediaLB Channel Status 0 Register"]
pub mod ms0;
#[doc = "MediaLB Channel Status1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms1](ms1) module"]
pub type MS1 = crate::Reg<u32, _MS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MS1;
#[doc = "`read()` method returns [ms1::R](ms1::R) reader structure"]
impl crate::Readable for MS1 {}
#[doc = "`write(|w| ..)` method takes [ms1::W](ms1::W) writer structure"]
impl crate::Writable for MS1 {}
#[doc = "MediaLB Channel Status1 Register"]
pub mod ms1;
#[doc = "MediaLB System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mss](mss) module"]
pub type MSS = crate::Reg<u32, _MSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSS;
#[doc = "`read()` method returns [mss::R](mss::R) reader structure"]
impl crate::Readable for MSS {}
#[doc = "`write(|w| ..)` method takes [mss::W](mss::W) writer structure"]
impl crate::Writable for MSS {}
#[doc = "MediaLB System Status Register"]
pub mod mss;
#[doc = "MediaLB System Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msd](msd) module"]
pub type MSD = crate::Reg<u32, _MSD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSD;
#[doc = "`read()` method returns [msd::R](msd::R) reader structure"]
impl crate::Readable for MSD {}
#[doc = "MediaLB System Data Register"]
pub mod msd;
#[doc = "MediaLB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mien](mien) module"]
pub type MIEN = crate::Reg<u32, _MIEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIEN;
#[doc = "`read()` method returns [mien::R](mien::R) reader structure"]
impl crate::Readable for MIEN {}
#[doc = "`write(|w| ..)` method takes [mien::W](mien::W) writer structure"]
impl crate::Writable for MIEN {}
#[doc = "MediaLB Interrupt Enable Register"]
pub mod mien;
#[doc = "MediaLB Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlbc1](mlbc1) module"]
pub type MLBC1 = crate::Reg<u32, _MLBC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MLBC1;
#[doc = "`read()` method returns [mlbc1::R](mlbc1::R) reader structure"]
impl crate::Readable for MLBC1 {}
#[doc = "`write(|w| ..)` method takes [mlbc1::W](mlbc1::W) writer structure"]
impl crate::Writable for MLBC1 {}
#[doc = "MediaLB Control 1 Register"]
pub mod mlbc1;
#[doc = "HBI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctl](hctl) module"]
pub type HCTL = crate::Reg<u32, _HCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCTL;
#[doc = "`read()` method returns [hctl::R](hctl::R) reader structure"]
impl crate::Readable for HCTL {}
#[doc = "`write(|w| ..)` method takes [hctl::W](hctl::W) writer structure"]
impl crate::Writable for HCTL {}
#[doc = "HBI Control Register"]
pub mod hctl;
#[doc = "HBI Channel Mask 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcmr](hcmr) module"]
pub type HCMR = crate::Reg<u32, _HCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCMR;
#[doc = "`read()` method returns [hcmr::R](hcmr::R) reader structure"]
impl crate::Readable for HCMR {}
#[doc = "`write(|w| ..)` method takes [hcmr::W](hcmr::W) writer structure"]
impl crate::Writable for HCMR {}
#[doc = "HBI Channel Mask 0 Register"]
pub mod hcmr;
#[doc = "HBI Channel Error 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcer](hcer) module"]
pub type HCER = crate::Reg<u32, _HCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCER;
#[doc = "`read()` method returns [hcer::R](hcer::R) reader structure"]
impl crate::Readable for HCER {}
#[doc = "HBI Channel Error 0 Register"]
pub mod hcer;
#[doc = "HBI Channel Busy 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcbr](hcbr) module"]
pub type HCBR = crate::Reg<u32, _HCBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCBR;
#[doc = "`read()` method returns [hcbr::R](hcbr::R) reader structure"]
impl crate::Readable for HCBR {}
#[doc = "HBI Channel Busy 0 Register"]
pub mod hcbr;
#[doc = "MIF Data 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdat](mdat) module"]
pub type MDAT = crate::Reg<u32, _MDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDAT;
#[doc = "`read()` method returns [mdat::R](mdat::R) reader structure"]
impl crate::Readable for MDAT {}
#[doc = "`write(|w| ..)` method takes [mdat::W](mdat::W) writer structure"]
impl crate::Writable for MDAT {}
#[doc = "MIF Data 0 Register"]
pub mod mdat;
#[doc = "MIF Data Write Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdwe](mdwe) module"]
pub type MDWE = crate::Reg<u32, _MDWE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDWE;
#[doc = "`read()` method returns [mdwe::R](mdwe::R) reader structure"]
impl crate::Readable for MDWE {}
#[doc = "`write(|w| ..)` method takes [mdwe::W](mdwe::W) writer structure"]
impl crate::Writable for MDWE {}
#[doc = "MIF Data Write Enable 0 Register"]
pub mod mdwe;
#[doc = "MIF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](mctl) module"]
pub type MCTL = crate::Reg<u32, _MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTL;
#[doc = "`read()` method returns [mctl::R](mctl::R) reader structure"]
impl crate::Readable for MCTL {}
#[doc = "`write(|w| ..)` method takes [mctl::W](mctl::W) writer structure"]
impl crate::Writable for MCTL {}
#[doc = "MIF Control Register"]
pub mod mctl;
#[doc = "MIF Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [madr](madr) module"]
pub type MADR = crate::Reg<u32, _MADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MADR;
#[doc = "`read()` method returns [madr::R](madr::R) reader structure"]
impl crate::Readable for MADR {}
#[doc = "`write(|w| ..)` method takes [madr::W](madr::W) writer structure"]
impl crate::Writable for MADR {}
#[doc = "MIF Address Register"]
pub mod madr;
#[doc = "AHB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actl](actl) module"]
pub type ACTL = crate::Reg<u32, _ACTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTL;
#[doc = "`read()` method returns [actl::R](actl::R) reader structure"]
impl crate::Readable for ACTL {}
#[doc = "`write(|w| ..)` method takes [actl::W](actl::W) writer structure"]
impl crate::Writable for ACTL {}
#[doc = "AHB Control Register"]
pub mod actl;
#[doc = "AHB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr](acsr) module"]
pub type ACSR = crate::Reg<u32, _ACSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACSR;
#[doc = "`read()` method returns [acsr::R](acsr::R) reader structure"]
impl crate::Readable for ACSR {}
#[doc = "`write(|w| ..)` method takes [acsr::W](acsr::W) writer structure"]
impl crate::Writable for ACSR {}
#[doc = "AHB Channel Status 0 Register"]
pub mod acsr;
#[doc = "AHB Channel Mask 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmr](acmr) module"]
pub type ACMR = crate::Reg<u32, _ACMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACMR;
#[doc = "`read()` method returns [acmr::R](acmr::R) reader structure"]
impl crate::Readable for ACMR {}
#[doc = "`write(|w| ..)` method takes [acmr::W](acmr::W) writer structure"]
impl crate::Writable for ACMR {}
#[doc = "AHB Channel Mask 0 Register"]
pub mod acmr;
