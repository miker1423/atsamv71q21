#[doc = "Reader of register KER"]
pub type R = crate::R<u32, super::KER>;
#[doc = "Writer for register KER"]
pub type W = crate::W<u32, super::KER>;
#[doc = "Register KER `reset()`'s with value 0"]
impl crate::ResetValue for super::KER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KCE`"]
pub type KCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KCE`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
