#[doc = "Register `LONBL` reader"]
pub struct R(crate::R<LONBL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LONBL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LONBL_SPEC>> for R {
    fn from(reader: crate::R<LONBL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LONBL` reader - LON Node Backlog Value"]
pub struct LONBL_R(crate::FieldReader<u8, u8>);
impl LONBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LONBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LONBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:5 - LON Node Backlog Value"]
    #[inline(always)]
    pub fn lonbl(&self) -> LONBL_R {
        LONBL_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "LON Backlog Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonbl](index.html) module"]
pub struct LONBL_SPEC;
impl crate::RegisterSpec for LONBL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lonbl::R](R) reader structure"]
impl crate::Readable for LONBL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LONBL to value 0"]
impl crate::Resettable for LONBL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
