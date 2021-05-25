#[doc = "Register `RB0` reader"]
pub struct R(crate::R<RB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RB0_SPEC>> for R {
    fn from(reader: crate::R<RB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RB0` writer"]
pub struct W(crate::W<RB0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RB0_SPEC>;
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
impl core::convert::From<crate::W<RB0_SPEC>> for W {
    fn from(writer: crate::W<RB0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - Register B"]
pub struct RB_R(crate::FieldReader<u32, u32>);
impl RB_R {
    pub(crate) fn new(bits: u32) -> Self {
        RB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB` writer - Register B"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register B (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb0](index.html) module"]
pub struct RB0_SPEC;
impl crate::RegisterSpec for RB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rb0::R](R) reader structure"]
impl crate::Readable for RB0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rb0::W](W) writer structure"]
impl crate::Writable for RB0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RB0 to value 0"]
impl crate::Resettable for RB0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
