#[doc = "Register `RC0R` reader"]
pub struct R(crate::R<RC0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RC0R_SPEC>> for R {
    fn from(reader: crate::R<RC0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC0R` writer"]
pub struct W(crate::W<RC0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC0R_SPEC>;
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
impl core::convert::From<crate::W<RC0R_SPEC>> for W {
    fn from(writer: crate::W<RC0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CP0` reader - Receive Compare Data 0"]
pub struct CP0_R(crate::FieldReader<u16, u16>);
impl CP0_R {
    pub(crate) fn new(bits: u16) -> Self {
        CP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CP0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CP0` writer - Receive Compare Data 0"]
pub struct CP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&mut self) -> CP0_W {
        CP0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Compare 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc0r](index.html) module"]
pub struct RC0R_SPEC;
impl crate::RegisterSpec for RC0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc0r::R](R) reader structure"]
impl crate::Readable for RC0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc0r::W](W) writer structure"]
impl crate::Writable for RC0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RC0R to value 0"]
impl crate::Resettable for RC0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
