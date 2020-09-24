#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `TXRDY0`"]
pub type TXRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY1`"]
pub type TXRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC0`"]
pub type EOC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDTX0`"]
pub type ENDTX0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDTX1`"]
pub type ENDTX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFE0`"]
pub type TXBUFE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFE1`"]
pub type TXBUFE1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Transmit Buffer Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn endtx0(&self) -> ENDTX0_R {
        ENDTX0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of Transmit Buffer Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn endtx1(&self) -> ENDTX1_R {
        ENDTX1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmit Buffer Empty Interrupt Mask of channel 0"]
    #[inline(always)]
    pub fn txbufe0(&self) -> TXBUFE0_R {
        TXBUFE0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Buffer Empty Interrupt Mask of channel 1"]
    #[inline(always)]
    pub fn txbufe1(&self) -> TXBUFE1_R {
        TXBUFE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
