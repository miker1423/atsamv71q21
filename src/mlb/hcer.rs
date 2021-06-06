#[doc = "Register `HCER[%s]` reader"]
pub struct R(crate::R<HCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCER_SPEC>> for R {
    fn from(reader: crate::R<HCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CERR` reader - Bitwise Channel Error Bit \\[31:0\\]"]
pub struct CERR_R(crate::FieldReader<u32, u32>);
impl CERR_R {
    pub(crate) fn new(bits: u32) -> Self {
        CERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "HBI Channel Error 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcer](index.html) module"]
pub struct HCER_SPEC;
impl crate::RegisterSpec for HCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcer::R](R) reader structure"]
impl crate::Readable for HCER_SPEC {
    type Reader = R;
}
