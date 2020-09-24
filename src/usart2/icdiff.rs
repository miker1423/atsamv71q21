#[doc = "Reader of register ICDIFF"]
pub type R = crate::R<u32, super::ICDIFF>;
#[doc = "Writer for register ICDIFF"]
pub type W = crate::W<u32, super::ICDIFF>;
#[doc = "Register ICDIFF `reset()`'s with value 0"]
impl crate::ResetValue for super::ICDIFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICDIFF`"]
pub type ICDIFF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ICDIFF`"]
pub struct ICDIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ICDIFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&self) -> ICDIFF_R {
        ICDIFF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IC Differentiator Number"]
    #[inline(always)]
    pub fn icdiff(&mut self) -> ICDIFF_W {
        ICDIFF_W { w: self }
    }
}
