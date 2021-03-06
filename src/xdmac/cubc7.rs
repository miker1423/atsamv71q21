#[doc = "Register `CUBC7` reader"]
pub struct R(crate::R<CUBC7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUBC7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CUBC7_SPEC>> for R {
    fn from(reader: crate::R<CUBC7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUBC7` writer"]
pub struct W(crate::W<CUBC7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUBC7_SPEC>;
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
impl core::convert::From<crate::W<CUBC7_SPEC>> for W {
    fn from(writer: crate::W<CUBC7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UBLEN` reader - Channel x Microblock Length"]
pub struct UBLEN_R(crate::FieldReader<u32, u32>);
impl UBLEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        UBLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UBLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UBLEN` writer - Channel x Microblock Length"]
pub struct UBLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UBLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UBLEN_R {
        UBLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&mut self) -> UBLEN_W {
        UBLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Microblock Control Register (chid = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cubc7](index.html) module"]
pub struct CUBC7_SPEC;
impl crate::RegisterSpec for CUBC7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cubc7::R](R) reader structure"]
impl crate::Readable for CUBC7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cubc7::W](W) writer structure"]
impl crate::Writable for CUBC7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CUBC7 to value 0"]
impl crate::Resettable for CUBC7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
