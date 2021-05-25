#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRDY` reader - Command Ready (cleared by writing in HSMCI_CMDR)"]
pub struct CMDRDY_R(crate::FieldReader<bool, bool>);
impl CMDRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading HSMCI_RDR)"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready (cleared by writing in HSMCI_TDR)"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKE` reader - Data Block Ended (cleared on read)"]
pub struct BLKE_R(crate::FieldReader<bool, bool>);
impl BLKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIP` reader - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
pub struct DTIP_R(crate::FieldReader<bool, bool>);
impl DTIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOTBUSY` reader - HSMCI Not Busy"]
pub struct NOTBUSY_R(crate::FieldReader<bool, bool>);
impl NOTBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOTBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOTBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOIRQA` reader - SDIO Interrupt for Slot A (cleared on read)"]
pub struct SDIOIRQA_R(crate::FieldReader<bool, bool>);
impl SDIOIRQA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOIRQA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOIRQA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDIOWAIT` reader - SDIO Read Wait Operation Status"]
pub struct SDIOWAIT_R(crate::FieldReader<bool, bool>);
impl SDIOWAIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDIOWAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIOWAIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRCV` reader - CE-ATA Completion Signal Received (cleared on read)"]
pub struct CSRCV_R(crate::FieldReader<bool, bool>);
impl CSRCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RINDE` reader - Response Index Error (cleared by writing in HSMCI_CMDR)"]
pub struct RINDE_R(crate::FieldReader<bool, bool>);
impl RINDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RINDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RINDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDIRE` reader - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
pub struct RDIRE_R(crate::FieldReader<bool, bool>);
impl RDIRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDIRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDIRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCRCE` reader - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
pub struct RCRCE_R(crate::FieldReader<bool, bool>);
impl RCRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RENDE` reader - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
pub struct RENDE_R(crate::FieldReader<bool, bool>);
impl RENDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RENDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RENDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTOE` reader - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
pub struct RTOE_R(crate::FieldReader<bool, bool>);
impl RTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCE` reader - Data CRC Error (cleared on read)"]
pub struct DCRCE_R(crate::FieldReader<bool, bool>);
impl DCRCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCRCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOE` reader - Data Time-out Error (cleared on read)"]
pub struct DTOE_R(crate::FieldReader<bool, bool>);
impl DTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSTOE` reader - Completion Signal Time-out Error (cleared on read)"]
pub struct CSTOE_R(crate::FieldReader<bool, bool>);
impl CSTOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSTOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSTOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLKOVRE` reader - DMA Block Overrun Error (cleared on read)"]
pub struct BLKOVRE_R(crate::FieldReader<bool, bool>);
impl BLKOVRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLKOVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLKOVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOEMPTY` reader - FIFO empty flag"]
pub struct FIFOEMPTY_R(crate::FieldReader<bool, bool>);
impl FIFOEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFRDONE` reader - Transfer Done flag"]
pub struct XFRDONE_R(crate::FieldReader<bool, bool>);
impl XFRDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        XFRDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFRDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKRCV` reader - Boot Operation Acknowledge Received (cleared on read)"]
pub struct ACKRCV_R(crate::FieldReader<bool, bool>);
impl ACKRCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKRCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKRCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKRCVE` reader - Boot Operation Acknowledge Error (cleared on read)"]
pub struct ACKRCVE_R(crate::FieldReader<bool, bool>);
impl ACKRCVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKRCVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKRCVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNRE` reader - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub struct UNRE_R(crate::FieldReader<bool, bool>);
impl UNRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Command Ready (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready (cleared by reading HSMCI_RDR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready (cleared by writing in HSMCI_TDR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended (cleared on read)"]
    #[inline(always)]
    pub fn blke(&self) -> BLKE_R {
        BLKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
    #[inline(always)]
    pub fn dtip(&self) -> DTIP_R {
        DTIP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSMCI Not Busy"]
    #[inline(always)]
    pub fn notbusy(&self) -> NOTBUSY_R {
        NOTBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A (cleared on read)"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SDIOIRQA_R {
        SDIOIRQA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SDIOWAIT_R {
        SDIOWAIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CE-ATA Completion Signal Received (cleared on read)"]
    #[inline(always)]
    pub fn csrcv(&self) -> CSRCV_R {
        CSRCV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Response Index Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rinde(&self) -> RINDE_R {
        RINDE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rdire(&self) -> RDIRE_R {
        RDIRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rcrce(&self) -> RCRCE_R {
        RCRCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rende(&self) -> RENDE_R {
        RENDE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rtoe(&self) -> RTOE_R {
        RTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error (cleared on read)"]
    #[inline(always)]
    pub fn dcrce(&self) -> DCRCE_R {
        DCRCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn cstoe(&self) -> CSTOE_R {
        CSTOE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn blkovre(&self) -> BLKOVRE_R {
        BLKOVRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FIFO empty flag"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transfer Done flag"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received (cleared on read)"]
    #[inline(always)]
    pub fn ackrcv(&self) -> ACKRCV_R {
        ACKRCV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error (cleared on read)"]
    #[inline(always)]
    pub fn ackrcve(&self) -> ACKRCVE_R {
        ACKRCVE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0xc0e5"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0e5
    }
}
