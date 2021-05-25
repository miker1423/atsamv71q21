#[doc = "Register `HSTPIPISR[%s]` reader"]
pub struct R(crate::R<HSTPIPISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIPISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HSTPIPISR_SPEC>> for R {
    fn from(reader: crate::R<HSTPIPISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub struct RXINI_R(crate::FieldReader<bool, bool>);
impl RXINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub struct TXOUTI_R(crate::FieldReader<bool, bool>);
impl TXOUTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOUTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOUTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTPI` reader - Transmitted SETUP Interrupt"]
pub struct TXSTPI_R(crate::FieldReader<bool, bool>);
impl TXSTPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub struct PERRI_R(crate::FieldReader<bool, bool>);
impl PERRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub struct NAKEDI_R(crate::FieldReader<bool, bool>);
impl NAKEDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKEDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKEDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub struct OVERFI_R(crate::FieldReader<bool, bool>);
impl OVERFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALLDI` reader - Received STALLed Interrupt"]
pub struct RXSTALLDI_R(crate::FieldReader<bool, bool>);
impl RXSTALLDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALLDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTALLDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHORTPACKETI` reader - Short Packet Interrupt"]
pub struct SHORTPACKETI_R(crate::FieldReader<bool, bool>);
impl SHORTPACKETI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHORTPACKETI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHORTPACKETI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data Toggle Sequence"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTSEQ_A {
    #[doc = "0: Data0 toggle sequence"]
    DATA0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    DATA1 = 1,
}
impl From<DTSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub struct DTSEQ_R(crate::FieldReader<u8, DTSEQ_A>);
impl DTSEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTSEQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTSEQ_A> {
        match self.bits {
            0 => Some(DTSEQ_A::DATA0),
            1 => Some(DTSEQ_A::DATA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        **self == DTSEQ_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        **self == DTSEQ_A::DATA1
    }
}
impl core::ops::Deref for DTSEQ_R {
    type Target = crate::FieldReader<u8, DTSEQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of Busy Banks"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBUSYBK_A {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0_BUSY = 0,
    #[doc = "1: 1 busy bank"]
    _1_BUSY = 1,
    #[doc = "2: 2 busy banks"]
    _2_BUSY = 2,
    #[doc = "3: 3 busy banks"]
    _3_BUSY = 3,
}
impl From<NBUSYBK_A> for u8 {
    #[inline(always)]
    fn from(variant: NBUSYBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub struct NBUSYBK_R(crate::FieldReader<u8, NBUSYBK_A>);
impl NBUSYBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBUSYBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBUSYBK_A {
        match self.bits {
            0 => NBUSYBK_A::_0_BUSY,
            1 => NBUSYBK_A::_1_BUSY,
            2 => NBUSYBK_A::_2_BUSY,
            3 => NBUSYBK_A::_3_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        **self == NBUSYBK_A::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        **self == NBUSYBK_A::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        **self == NBUSYBK_A::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        **self == NBUSYBK_A::_3_BUSY
    }
}
impl core::ops::Deref for NBUSYBK_R {
    type Target = crate::FieldReader<u8, NBUSYBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current Bank"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CURRBK_A {
    #[doc = "0: Current bank is bank0"]
    BANK0 = 0,
    #[doc = "1: Current bank is bank1"]
    BANK1 = 1,
    #[doc = "2: Current bank is bank2"]
    BANK2 = 2,
}
impl From<CURRBK_A> for u8 {
    #[inline(always)]
    fn from(variant: CURRBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub struct CURRBK_R(crate::FieldReader<u8, CURRBK_A>);
impl CURRBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURRBK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURRBK_A> {
        match self.bits {
            0 => Some(CURRBK_A::BANK0),
            1 => Some(CURRBK_A::BANK1),
            2 => Some(CURRBK_A::BANK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        **self == CURRBK_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        **self == CURRBK_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        **self == CURRBK_A::BANK2
    }
}
impl core::ops::Deref for CURRBK_R {
    type Target = crate::FieldReader<u8, CURRBK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RWALL` reader - Read/Write Allowed"]
pub struct RWALL_R(crate::FieldReader<bool, bool>);
impl RWALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RWALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub struct CFGOK_R(crate::FieldReader<bool, bool>);
impl CFGOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFGOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFGOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBYCT` reader - Pipe Byte Count"]
pub struct PBYCT_R(crate::FieldReader<u16, u16>);
impl PBYCT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PBYCT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBYCT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RXINI_R {
        RXINI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TXOUTI_R {
        TXOUTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt"]
    #[inline(always)]
    pub fn txstpi(&self) -> TXSTPI_R {
        TXSTPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PERRI_R {
        PERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NAKEDI_R {
        NAKEDI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt"]
    #[inline(always)]
    pub fn rxstalldi(&self) -> RXSTALLDI_R {
        RXSTALLDI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacketi(&self) -> SHORTPACKETI_R {
        SHORTPACKETI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:30 - Pipe Byte Count"]
    #[inline(always)]
    pub fn pbyct(&self) -> PBYCT_R {
        PBYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Host Pipe Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr](index.html) module"]
pub struct HSTPIPISR_SPEC;
impl crate::RegisterSpec for HSTPIPISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpipisr::R](R) reader structure"]
impl crate::Readable for HSTPIPISR_SPEC {
    type Reader = R;
}
