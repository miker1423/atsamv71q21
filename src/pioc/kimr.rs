#[doc = "Register `KIMR` reader"]
pub struct R(crate::R<KIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KIMR_SPEC>> for R {
    fn from(reader: crate::R<KIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KPR` reader - Key Press Interrupt Mask"]
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
#[doc = "Field `KRL` reader - Key Release Interrupt Mask"]
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
impl R {
    #[doc = "Bit 0 - Key Press Interrupt Mask"]
    #[inline(always)]
    pub fn kpr(&self) -> KPR_R {
        KPR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Release Interrupt Mask"]
    #[inline(always)]
    pub fn krl(&self) -> KRL_R {
        KRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Keypad Controller Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kimr](index.html) module"]
pub struct KIMR_SPEC;
impl crate::RegisterSpec for KIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kimr::R](R) reader structure"]
impl crate::Readable for KIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KIMR to value 0"]
impl crate::Resettable for KIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
