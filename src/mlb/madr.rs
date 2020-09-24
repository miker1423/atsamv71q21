#[doc = "Reader of register MADR"]
pub type R = crate::R<u32, super::MADR>;
#[doc = "Writer for register MADR"]
pub type W = crate::W<u32, super::MADR>;
#[doc = "Register MADR `reset()`'s with value 0"]
impl crate::ResetValue for super::MADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
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
#[doc = "Reader of field `TB`"]
pub type TB_R = crate::R<bool, TB_A>;
impl TB_R {
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
        *self == TB_A::CTR
    }
    #[doc = "Checks if the value of the field is `DBR`"]
    #[inline(always)]
    pub fn is_dbr(&self) -> bool {
        *self == TB_A::DBR
    }
}
#[doc = "Write proxy for field `TB`"]
pub struct TB_W<'a> {
    w: &'a mut W,
}
impl<'a> TB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WNR`"]
pub type WNR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WNR`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
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
}
