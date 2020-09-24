#[doc = "Reader of register MLBC1"]
pub type R = crate::R<u32, super::MLBC1>;
#[doc = "Writer for register MLBC1"]
pub type W = crate::W<u32, super::MLBC1>;
#[doc = "Register MLBC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::MLBC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CLKM`"]
pub type CLKM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKM`"]
pub struct CLKM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `NDA`"]
pub type NDA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDA`"]
pub struct NDA_W<'a> {
    w: &'a mut W,
}
impl<'a> NDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&self) -> CLKM_R {
        CLKM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&mut self) -> CLKM_W {
        CLKM_W { w: self }
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NDA_W {
        NDA_W { w: self }
    }
}
