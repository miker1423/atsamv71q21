#[doc = "Register `IADR` reader"]
pub struct R(crate::R<IADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IADR_SPEC>> for R {
    fn from(reader: crate::R<IADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IADR` writer"]
pub struct W(crate::W<IADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IADR_SPEC>;
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
impl core::convert::From<crate::W<IADR_SPEC>> for W {
    fn from(writer: crate::W<IADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IADR` reader - Internal Address"]
pub struct IADR_R(crate::FieldReader<u32, u32>);
impl IADR_R {
    pub(crate) fn new(bits: u32) -> Self {
        IADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IADR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IADR` writer - Internal Address"]
pub struct IADR_W<'a> {
    w: &'a mut W,
}
impl<'a> IADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&self) -> IADR_R {
        IADR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&mut self) -> IADR_W {
        IADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iadr](index.html) module"]
pub struct IADR_SPEC;
impl crate::RegisterSpec for IADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iadr::R](R) reader structure"]
impl crate::Readable for IADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iadr::W](W) writer structure"]
impl crate::Writable for IADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IADR to value 0"]
impl crate::Resettable for IADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
