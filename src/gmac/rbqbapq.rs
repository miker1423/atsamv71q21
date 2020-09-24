#[doc = "Reader of register RBQBAPQ[%s]"]
pub type R = crate::R<u32, super::RBQBAPQ>;
#[doc = "Writer for register RBQBAPQ[%s]"]
pub type W = crate::W<u32, super::RBQBAPQ>;
#[doc = "Reader of field `RXBQBA`"]
pub type RXBQBA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBQBA`"]
pub struct RXBQBA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBQBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&self) -> RXBQBA_R {
        RXBQBA_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&mut self) -> RXBQBA_W {
        RXBQBA_W { w: self }
    }
}
