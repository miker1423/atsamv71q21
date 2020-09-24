#[doc = "Reader of register CSR_LON_MODE"]
pub type R = crate::R<u32, super::CSR_LON_MODE>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LSFE`"]
pub type LSFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCRCE`"]
pub type LCRCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNRE`"]
pub type UNRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LTXD`"]
pub type LTXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<bool, bool>;
#[doc = "Reader of field `LFET`"]
pub type LFET_R = crate::R<bool, bool>;
#[doc = "Reader of field `LRXD`"]
pub type LRXD_R = crate::R<bool, bool>;
#[doc = "Reader of field `LBLOVFE`"]
pub type LBLOVFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lsfe(&self) -> LSFE_R {
        LSFE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lcrce(&self) -> LCRCE_R {
        LCRCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Underrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LON Transmission End Flag (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ltxd(&self) -> LTXD_R {
        LTXD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LON Collision Detected Flag (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lfet(&self) -> LFET_R {
        LFET_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LON Reception End Flag (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lrxd(&self) -> LRXD_R {
        LRXD_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LBLOVFE_R {
        LBLOVFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
