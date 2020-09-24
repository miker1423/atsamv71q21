#[doc = "Reader of register KDR"]
pub type R = crate::R<u32, super::KDR>;
#[doc = "Writer for register KDR"]
pub type W = crate::W<u32, super::KDR>;
#[doc = "Register KDR `reset()`'s with value 0"]
impl crate::ResetValue for super::KDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBC`"]
pub type DBC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DBC`"]
pub struct DBC_W<'a> {
    w: &'a mut W,
}
impl<'a> DBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Debouncing Value"]
    #[inline(always)]
    pub fn dbc(&self) -> DBC_R {
        DBC_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Debouncing Value"]
    #[inline(always)]
    pub fn dbc(&mut self) -> DBC_W {
        DBC_W { w: self }
    }
}
