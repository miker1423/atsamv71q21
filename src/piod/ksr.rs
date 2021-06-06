#[doc = "Register `KSR` reader"]
pub struct R(crate::R<KSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KSR_SPEC>> for R {
    fn from(reader: crate::R<KSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KPR` reader - Key Press Status"]
pub struct KPR_R(crate::FieldReader<bool, bool>);
impl KPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        KPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KRL` reader - Key Release Status"]
pub struct KRL_R(crate::FieldReader<bool, bool>);
impl KRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        KRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBKPR` reader - Number of Simultaneous Key Presses"]
pub struct NBKPR_R(crate::FieldReader<u8, u8>);
impl NBKPR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBKPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBKPR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBKRL` reader - Number of Simultaneous Key Releases"]
pub struct NBKRL_R(crate::FieldReader<u8, u8>);
impl NBKRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBKRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBKRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Key Press Status"]
    #[inline(always)]
    pub fn kpr(&self) -> KPR_R {
        KPR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Release Status"]
    #[inline(always)]
    pub fn krl(&self) -> KRL_R {
        KRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Number of Simultaneous Key Presses"]
    #[inline(always)]
    pub fn nbkpr(&self) -> NBKPR_R {
        NBKPR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Number of Simultaneous Key Releases"]
    #[inline(always)]
    pub fn nbkrl(&self) -> NBKRL_R {
        NBKRL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
#[doc = "Keypad Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ksr](index.html) module"]
pub struct KSR_SPEC;
impl crate::RegisterSpec for KSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ksr::R](R) reader structure"]
impl crate::Readable for KSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KSR to value 0"]
impl crate::Resettable for KSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
