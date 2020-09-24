#[doc = "Reader of register HCTL"]
pub type R = crate::R<u32, super::HCTL>;
#[doc = "Writer for register HCTL"]
pub type W = crate::W<u32, super::HCTL>;
#[doc = "Register HCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::HCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RST0`"]
pub type RST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST0`"]
pub struct RST0_W<'a> {
    w: &'a mut W,
}
impl<'a> RST0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RST1`"]
pub type RST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST1`"]
pub struct RST1_W<'a> {
    w: &'a mut W,
}
impl<'a> RST1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&mut self) -> RST0_W {
        RST0_W { w: self }
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&mut self) -> RST1_W {
        RST1_W { w: self }
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
