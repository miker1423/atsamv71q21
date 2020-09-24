#[doc = "Reader of register KSR"]
pub type R = crate::R<u32, super::KSR>;
#[doc = "Reader of field `KPR`"]
pub type KPR_R = crate::R<bool, bool>;
#[doc = "Reader of field `KRL`"]
pub type KRL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBKPR`"]
pub type NBKPR_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBKRL`"]
pub type NBKRL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Key Press Status"]
    #[inline(always)]
    pub fn kpr(&self) -> KPR_R {
        KPR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Release Status"]
    #[inline(always)]
    pub fn krl(&self) -> KRL_R {
        KRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Number of Simultaneous Key Presses"]
    #[inline(always)]
    pub fn nbkpr(&self) -> NBKPR_R {
        NBKPR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Number of Simultaneous Key Releases"]
    #[inline(always)]
    pub fn nbkrl(&self) -> NBKRL_R {
        NBKRL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
