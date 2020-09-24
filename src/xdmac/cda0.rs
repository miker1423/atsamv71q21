#[doc = "Reader of register CDA0"]
pub type R = crate::R<u32, super::CDA0>;
#[doc = "Writer for register CDA0"]
pub type W = crate::W<u32, super::CDA0>;
#[doc = "Register CDA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CDA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DA`"]
pub type DA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DA`"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
}