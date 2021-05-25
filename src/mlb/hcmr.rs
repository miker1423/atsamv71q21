#[doc = "Register `HCMR[%s]` reader"]
pub struct R(crate::R<HCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCMR_SPEC>> for R {
    fn from(reader: crate::R<HCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCMR[%s]` writer"]
pub struct W(crate::W<HCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<HCMR_SPEC>> for W {
    fn from(writer: crate::W<HCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bit \\[31:0\\]"]
pub struct CHM_R(crate::FieldReader<u32, u32>);
impl CHM_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bit \\[31:0\\]"]
pub struct CHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&mut self) -> CHM_W {
        CHM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBI Channel Mask 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcmr](index.html) module"]
pub struct HCMR_SPEC;
impl crate::RegisterSpec for HCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcmr::R](R) reader structure"]
impl crate::Readable for HCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcmr::W](W) writer structure"]
impl crate::Writable for HCMR_SPEC {
    type Writer = W;
}
