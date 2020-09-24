#[doc = "Reader of register TBQBAPQ[%s]"]
pub type R = crate::R<u32, super::TBQBAPQ>;
#[doc = "Writer for register TBQBAPQ[%s]"]
pub type W = crate::W<u32, super::TBQBAPQ>;
#[doc = "Reader of field `TXBQBA`"]
pub type TXBQBA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXBQBA`"]
pub struct TXBQBA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBQBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&self) -> TXBQBA_R {
        TXBQBA_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Transmit Buffer Queue Base Address"]
    #[inline(always)]
    pub fn txbqba(&mut self) -> TXBQBA_W {
        TXBQBA_W { w: self }
    }
}
