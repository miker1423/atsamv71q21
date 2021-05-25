#[doc = "Register `MADR` reader"]
pub struct R(crate::R<MADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MADR_SPEC>> for R {
    fn from(reader: crate::R<MADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MADR` writer"]
pub struct W(crate::W<MADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MADR_SPEC>;
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
impl core::convert::From<crate::W<MADR_SPEC>> for W {
    fn from(writer: crate::W<MADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - CTR or DBR Address"]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - CTR or DBR Address"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Target Location Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TB_A {
    #[doc = "0: Selects CTR"]
    CTR = 0,
    #[doc = "1: Selects DBR"]
    DBR = 1,
}
impl From<TB_A> for bool {
    #[inline(always)]
    fn from(variant: TB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TB` reader - Target Location Bit"]
pub struct TB_R(crate::FieldReader<bool, TB_A>);
impl TB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TB_A {
        match self.bits {
            false => TB_A::CTR,
            true => TB_A::DBR,
        }
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        **self == TB_A::CTR
    }
    #[doc = "Checks if the value of the field is `DBR`"]
    #[inline(always)]
    pub fn is_dbr(&self) -> bool {
        **self == TB_A::DBR
    }
}
impl core::ops::Deref for TB_R {
    type Target = crate::FieldReader<bool, TB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TB` writer - Target Location Bit"]
pub struct TB_W<'a> {
    w: &'a mut W,
}
impl<'a> TB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects CTR"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(TB_A::CTR)
    }
    #[doc = "Selects DBR"]
    #[inline(always)]
    pub fn dbr(self) -> &'a mut W {
        self.variant(TB_A::DBR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WNR` reader - Write-Not-Read Selection"]
pub struct WNR_R(crate::FieldReader<bool, bool>);
impl WNR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WNR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WNR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WNR` writer - Write-Not-Read Selection"]
pub struct WNR_W<'a> {
    w: &'a mut W,
}
impl<'a> WNR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    pub fn wnr(&self) -> WNR_R {
        WNR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - CTR or DBR Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bit 30 - Target Location Bit"]
    #[inline(always)]
    pub fn tb(&mut self) -> TB_W {
        TB_W { w: self }
    }
    #[doc = "Bit 31 - Write-Not-Read Selection"]
    #[inline(always)]
    pub fn wnr(&mut self) -> WNR_W {
        WNR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [madr](index.html) module"]
pub struct MADR_SPEC;
impl crate::RegisterSpec for MADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [madr::R](R) reader structure"]
impl crate::Readable for MADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [madr::W](W) writer structure"]
impl crate::Writable for MADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MADR to value 0"]
impl crate::Resettable for MADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
