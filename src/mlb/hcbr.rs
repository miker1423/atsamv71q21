#[doc = "Reader of register HCBR[%s]"]
pub type R = crate::R<u32, super::HCBR>;
#[doc = "Reader of field `CHB`"]
pub type CHB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Busy Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
