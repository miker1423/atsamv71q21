#[doc = "Register `ARGR` reader"]
pub struct R(crate::R<ARGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ARGR_SPEC>> for R {
    fn from(reader: crate::R<ARGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARGR` writer"]
pub struct W(crate::W<ARGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARGR_SPEC>;
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
impl core::convert::From<crate::W<ARGR_SPEC>> for W {
    fn from(writer: crate::W<ARGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARG` reader - Command Argument"]
pub struct ARG_R(crate::FieldReader<u32, u32>);
impl ARG_R {
    pub(crate) fn new(bits: u32) -> Self {
        ARG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARG` writer - Command Argument"]
pub struct ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W {
        ARG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [argr](index.html) module"]
pub struct ARGR_SPEC;
impl crate::RegisterSpec for ARGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [argr::R](R) reader structure"]
impl crate::Readable for ARGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [argr::W](W) writer structure"]
impl crate::Writable for ARGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ARGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
