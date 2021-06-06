#[doc = "Register `KIER` writer"]
pub struct W(crate::W<KIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KIER_SPEC>;
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
impl core::convert::From<crate::W<KIER_SPEC>> for W {
    fn from(writer: crate::W<KIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KPR` writer - Key Press Interrupt Enable"]
pub struct KPR_W<'a> {
    w: &'a mut W,
}
impl<'a> KPR_W<'a> {
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
#[doc = "Field `KRL` writer - Key Release Interrupt Enable"]
pub struct KRL_W<'a> {
    w: &'a mut W,
}
impl<'a> KRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Key Press Interrupt Enable"]
    #[inline(always)]
    pub fn kpr(&mut self) -> KPR_W {
        KPR_W { w: self }
    }
    #[doc = "Bit 1 - Key Release Interrupt Enable"]
    #[inline(always)]
    pub fn krl(&mut self) -> KRL_W {
        KRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Controller Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kier](index.html) module"]
pub struct KIER_SPEC;
impl crate::RegisterSpec for KIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [kier::W](W) writer structure"]
impl crate::Writable for KIER_SPEC {
    type Writer = W;
}
