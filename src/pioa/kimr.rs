#[doc = "Reader of register KIMR"]
pub type R = crate::R<u32, super::KIMR>;
#[doc = "Reader of field `KPR`"]
pub type KPR_R = crate::R<bool, bool>;
#[doc = "Reader of field `KRL`"]
pub type KRL_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Key Press Interrupt Mask"]
    #[inline(always)]
    pub fn kpr(&self) -> KPR_R {
        KPR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Release Interrupt Mask"]
    #[inline(always)]
    pub fn krl(&self) -> KRL_R {
        KRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
