#[doc = "Register `KER` reader"]
pub struct R(crate::R<KER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KER_SPEC>> for R {
    fn from(reader: crate::R<KER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KER` writer"]
pub struct W(crate::W<KER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KER_SPEC>;
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
impl core::convert::From<crate::W<KER_SPEC>> for W {
    fn from(writer: crate::W<KER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KCE` reader - Keypad Controller Enable"]
pub struct KCE_R(crate::FieldReader<bool, bool>);
impl KCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        KCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KCE` writer - Keypad Controller Enable"]
pub struct KCE_W<'a> {
    w: &'a mut W,
}
impl<'a> KCE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Keypad Controller Enable"]
    #[inline(always)]
    pub fn kce(&self) -> KCE_R {
        KCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keypad Controller Enable"]
    #[inline(always)]
    pub fn kce(&mut self) -> KCE_W {
        KCE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Controller Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ker](index.html) module"]
pub struct KER_SPEC;
impl crate::RegisterSpec for KER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ker::R](R) reader structure"]
impl crate::Readable for KER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ker::W](W) writer structure"]
impl crate::Writable for KER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KER to value 0"]
impl crate::Resettable for KER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
