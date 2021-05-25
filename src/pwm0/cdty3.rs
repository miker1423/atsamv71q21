#[doc = "Register `CDTY3` reader"]
pub struct R(crate::R<CDTY3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDTY3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CDTY3_SPEC>> for R {
    fn from(reader: crate::R<CDTY3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDTY3` writer"]
pub struct W(crate::W<CDTY3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDTY3_SPEC>;
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
impl core::convert::From<crate::W<CDTY3_SPEC>> for W {
    fn from(writer: crate::W<CDTY3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTY` reader - Channel Duty-Cycle"]
pub struct CDTY_R(crate::FieldReader<u32, u32>);
impl CDTY_R {
    pub(crate) fn new(bits: u32) -> Self {
        CDTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDTY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDTY` writer - Channel Duty-Cycle"]
pub struct CDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&self) -> CDTY_R {
        CDTY_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle"]
    #[inline(always)]
    pub fn cdty(&mut self) -> CDTY_W {
        CDTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdty3](index.html) module"]
pub struct CDTY3_SPEC;
impl crate::RegisterSpec for CDTY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdty3::R](R) reader structure"]
impl crate::Readable for CDTY3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdty3::W](W) writer structure"]
impl crate::Writable for CDTY3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDTY3 to value 0"]
impl crate::Resettable for CDTY3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
