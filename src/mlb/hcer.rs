#[doc = "Reader of register HCER[%s]"]
pub type R = crate::R<u32, super::HCER>;
#[doc = "Reader of field `CERR`"]
pub type CERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
