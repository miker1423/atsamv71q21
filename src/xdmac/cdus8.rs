#[doc = "Register `CDUS8` reader"]
pub struct R(crate::R<CDUS8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDUS8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CDUS8_SPEC>> for R {
    fn from(reader: crate::R<CDUS8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDUS8` writer"]
pub struct W(crate::W<CDUS8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDUS8_SPEC>;
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
impl core::convert::From<crate::W<CDUS8_SPEC>> for W {
    fn from(writer: crate::W<CDUS8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUBS` reader - Channel x Destination Microblock Stride"]
pub struct DUBS_R(crate::FieldReader<u32, u32>);
impl DUBS_R {
    pub(crate) fn new(bits: u32) -> Self {
        DUBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUBS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUBS` writer - Channel x Destination Microblock Stride"]
pub struct DUBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DUBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&self) -> DUBS_R {
        DUBS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&mut self) -> DUBS_W {
        DUBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Destination Microblock Stride (chid = 8)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdus8](index.html) module"]
pub struct CDUS8_SPEC;
impl crate::RegisterSpec for CDUS8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdus8::R](R) reader structure"]
impl crate::Readable for CDUS8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdus8::W](W) writer structure"]
impl crate::Writable for CDUS8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDUS8 to value 0"]
impl crate::Resettable for CDUS8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
