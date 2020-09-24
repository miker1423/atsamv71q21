#[doc = "Reader of register MCTL"]
pub type R = crate::R<u32, super::MCTL>;
#[doc = "Writer for register MCTL"]
pub type W = crate::W<u32, super::MCTL>;
#[doc = "Register MCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::MCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XCMP`"]
pub type XCMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XCMP`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
