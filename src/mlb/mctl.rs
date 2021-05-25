#[doc = "Register `MCTL` reader"]
pub struct R(crate::R<MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MCTL_SPEC>> for R {
    fn from(reader: crate::R<MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTL` writer"]
pub struct W(crate::W<MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTL_SPEC>;
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
impl core::convert::From<crate::W<MCTL_SPEC>> for W {
    fn from(writer: crate::W<MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XCMP` reader - Transfer Complete (Write 0 to Clear)"]
pub struct XCMP_R(crate::FieldReader<bool, bool>);
impl XCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        XCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XCMP` writer - Transfer Complete (Write 0 to Clear)"]
pub struct XCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> XCMP_W<'a> {
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
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&self) -> XCMP_R {
        XCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Complete (Write 0 to Clear)"]
    #[inline(always)]
    pub fn xcmp(&mut self) -> XCMP_W {
        XCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](index.html) module"]
pub struct MCTL_SPEC;
impl crate::RegisterSpec for MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctl::R](R) reader structure"]
impl crate::Readable for MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctl::W](W) writer structure"]
impl crate::Writable for MCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTL to value 0"]
impl crate::Resettable for MCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
