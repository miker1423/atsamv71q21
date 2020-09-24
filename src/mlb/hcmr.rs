#[doc = "Reader of register HCMR[%s]"]
pub type R = crate::R<u32, super::HCMR>;
#[doc = "Writer for register HCMR[%s]"]
pub type W = crate::W<u32, super::HCMR>;
#[doc = "Reader of field `CHM`"]
pub type CHM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHM`"]
pub struct CHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chm(&mut self) -> CHM_W {
        CHM_W { w: self }
    }
}
