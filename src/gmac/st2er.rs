#[doc = "Reader of register ST2ER[%s]"]
pub type R = crate::R<u32, super::ST2ER>;
#[doc = "Writer for register ST2ER[%s]"]
pub type W = crate::W<u32, super::ST2ER>;
#[doc = "Reader of field `COMPVAL`"]
pub type COMPVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPVAL`"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
}
