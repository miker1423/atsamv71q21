#[doc = "Reader of register CSA10"]
pub type R = crate::R<u32, super::CSA10>;
#[doc = "Writer for register CSA10"]
pub type W = crate::W<u32, super::CSA10>;
#[doc = "Register CSA10 `reset()`'s with value 0"]
impl crate::ResetValue for super::CSA10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
}
