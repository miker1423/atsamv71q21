#[doc = "Writer for register OCMS_KEY1"]
pub type W = crate::W<u32, super::OCMS_KEY1>;
#[doc = "Write proxy for field `KEY1`"]
pub struct KEY1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Off-chip Memory Scrambling (OCMS) Key Part 1"]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W {
        KEY1_W { w: self }
    }
}
