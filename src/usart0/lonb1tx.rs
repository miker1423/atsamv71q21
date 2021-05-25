#[doc = "Register `LONB1TX` reader"]
pub struct R(crate::R<LONB1TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LONB1TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LONB1TX_SPEC>> for R {
    fn from(reader: crate::R<LONB1TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LONB1TX` writer"]
pub struct W(crate::W<LONB1TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LONB1TX_SPEC>;
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
impl core::convert::From<crate::W<LONB1TX_SPEC>> for W {
    fn from(writer: crate::W<LONB1TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BETA1TX` reader - LON Beta1 Length after Transmission"]
pub struct BETA1TX_R(crate::FieldReader<u32, u32>);
impl BETA1TX_R {
    pub(crate) fn new(bits: u32) -> Self {
        BETA1TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BETA1TX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BETA1TX` writer - LON Beta1 Length after Transmission"]
pub struct BETA1TX_W<'a> {
    w: &'a mut W,
}
impl<'a> BETA1TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    pub fn beta1tx(&self) -> BETA1TX_R {
        BETA1TX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    pub fn beta1tx(&mut self) -> BETA1TX_W {
        BETA1TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LON Beta1 Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lonb1tx](index.html) module"]
pub struct LONB1TX_SPEC;
impl crate::RegisterSpec for LONB1TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lonb1tx::R](R) reader structure"]
impl crate::Readable for LONB1TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lonb1tx::W](W) writer structure"]
impl crate::Writable for LONB1TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LONB1TX to value 0"]
impl crate::Resettable for LONB1TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
