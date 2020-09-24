#[doc = "Reader of register KRCR"]
pub type R = crate::R<u32, super::KRCR>;
#[doc = "Writer for register KRCR"]
pub type W = crate::W<u32, super::KRCR>;
#[doc = "Register KRCR `reset()`'s with value 0"]
impl crate::ResetValue for super::KRCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBR`"]
pub type NBR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBR`"]
pub struct NBR_W<'a> {
    w: &'a mut W,
}
impl<'a> NBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `NBC`"]
pub type NBC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBC`"]
pub struct NBC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of Columns of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbr(&self) -> NBR_R {
        NBR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Number of Rows of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of Columns of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbr(&mut self) -> NBR_W {
        NBR_W { w: self }
    }
    #[doc = "Bits 8:10 - Number of Rows of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W {
        NBC_W { w: self }
    }
}
