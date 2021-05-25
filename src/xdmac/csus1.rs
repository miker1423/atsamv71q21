#[doc = "Register `CSUS1` reader"]
pub struct R(crate::R<CSUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSUS1_SPEC>> for R {
    fn from(reader: crate::R<CSUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSUS1` writer"]
pub struct W(crate::W<CSUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSUS1_SPEC>;
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
impl core::convert::From<crate::W<CSUS1_SPEC>> for W {
    fn from(writer: crate::W<CSUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBS` reader - Channel x Source Microblock Stride"]
pub struct SUBS_R(crate::FieldReader<u32, u32>);
impl SUBS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SUBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUBS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUBS` writer - Channel x Source Microblock Stride"]
pub struct SUBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&self) -> SUBS_R {
        SUBS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&mut self) -> SUBS_W {
        SUBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Source Microblock Stride (chid = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csus1](index.html) module"]
pub struct CSUS1_SPEC;
impl crate::RegisterSpec for CSUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csus1::R](R) reader structure"]
impl crate::Readable for CSUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csus1::W](W) writer structure"]
impl crate::Writable for CSUS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSUS1 to value 0"]
impl crate::Resettable for CSUS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
