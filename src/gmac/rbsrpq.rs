#[doc = "Reader of register RBSRPQ[%s]"]
pub type R = crate::R<u32, super::RBSRPQ>;
#[doc = "Writer for register RBSRPQ[%s]"]
pub type W = crate::W<u32, super::RBSRPQ>;
#[doc = "Reader of field `RBS`"]
pub type RBS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RBS`"]
pub struct RBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W {
        RBS_W { w: self }
    }
}
