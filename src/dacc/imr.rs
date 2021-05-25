#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMR_SPEC>> for R {
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRDY0` reader - Transmit Ready Interrupt Mask of channel 0"]
pub struct TXRDY0_R(crate::FieldReader<bool, bool>);
impl TXRDY0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY1` reader - Transmit Ready Interrupt Mask of channel 1"]
pub struct TXRDY1_R(crate::FieldReader<bool, bool>);
impl TXRDY1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Mask of channel 0"]
pub struct EOC0_R(crate::FieldReader<bool, bool>);
impl EOC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Mask of channel 1"]
pub struct EOC1_R(crate::FieldReader<bool, bool>);
impl EOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDTX0` reader - End of Transmit Buffer Interrupt Mask of channel 0"]
pub struct ENDTX0_R(crate::FieldReader<bool, bool>);
impl ENDTX0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDTX0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDTX0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENDTX1` reader - End of Transmit Buffer Interrupt Mask of channel 1"]
pub struct ENDTX1_R(crate::FieldReader<bool, bool>);
impl ENDTX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDTX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENDTX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUFE0` reader - Transmit Buffer Empty Interrupt Mask of channel 0"]
pub struct TXBUFE0_R(crate::FieldReader<bool, bool>);
impl TXBUFE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBUFE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUFE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUFE1` reader - Transmit Buffer Empty Interrupt Mask of channel 1"]
pub struct TXBUFE1_R(crate::FieldReader<bool, bool>);
impl TXBUFE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBUFE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUFE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Transmit Buffer Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn endtx0(&self) -> ENDTX0_R {
        ENDTX0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of Transmit Buffer Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn endtx1(&self) -> ENDTX1_R {
        ENDTX1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Empty Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn txbufe0(&self) -> TXBUFE0_R {
        TXBUFE0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Empty Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn txbufe1(&self) -> TXBUFE1_R {
        TXBUFE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
