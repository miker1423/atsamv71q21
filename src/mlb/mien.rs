#[doc = "Reader of register MIEN"]
pub type R = crate::R<u32, super::MIEN>;
#[doc = "Writer for register MIEN"]
pub type W = crate::W<u32, super::MIEN>;
#[doc = "Register MIEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MIEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISOC_PE`"]
pub type ISOC_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOC_PE`"]
pub struct ISOC_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOC_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ISOC_BUFO`"]
pub type ISOC_BUFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ISOC_BUFO`"]
pub struct ISOC_BUFO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOC_BUFO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SYNC_PE`"]
pub type SYNC_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC_PE`"]
pub struct SYNC_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ARX_DONE`"]
pub type ARX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARX_DONE`"]
pub struct ARX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ARX_PE`"]
pub type ARX_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARX_PE`"]
pub struct ARX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ARX_BREAK`"]
pub type ARX_BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARX_BREAK`"]
pub struct ARX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_BREAK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ATX_DONE`"]
pub type ATX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATX_DONE`"]
pub struct ATX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ATX_PE`"]
pub type ATX_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATX_PE`"]
pub struct ATX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `ATX_BREAK`"]
pub type ATX_BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATX_BREAK`"]
pub struct ATX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_BREAK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `CRX_DONE`"]
pub type CRX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRX_DONE`"]
pub struct CRX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `CRX_PE`"]
pub type CRX_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRX_PE`"]
pub struct CRX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `CRX_BREAK`"]
pub type CRX_BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRX_BREAK`"]
pub struct CRX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_BREAK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CTX_DONE`"]
pub type CTX_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTX_DONE`"]
pub struct CTX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `CTX_PE`"]
pub type CTX_PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTX_PE`"]
pub struct CTX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `CTX_BREAK`"]
pub type CTX_BREAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTX_BREAK`"]
pub struct CTX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_BREAK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&self) -> ISOC_PE_R {
        ISOC_PE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&self) -> ISOC_BUFO_R {
        ISOC_BUFO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&self) -> SYNC_PE_R {
        SYNC_PE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&self) -> ARX_DONE_R {
        ARX_DONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&self) -> ARX_PE_R {
        ARX_PE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&self) -> ARX_BREAK_R {
        ARX_BREAK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&self) -> ATX_DONE_R {
        ATX_DONE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&self) -> ATX_PE_R {
        ATX_PE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&self) -> ATX_BREAK_R {
        ATX_BREAK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&self) -> CRX_DONE_R {
        CRX_DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&self) -> CRX_PE_R {
        CRX_PE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&self) -> CRX_BREAK_R {
        CRX_BREAK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&self) -> CTX_DONE_R {
        CTX_DONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&self) -> CTX_PE_R {
        CTX_PE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&self) -> CTX_BREAK_R {
        CTX_BREAK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&mut self) -> ISOC_PE_W {
        ISOC_PE_W { w: self }
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&mut self) -> ISOC_BUFO_W {
        ISOC_BUFO_W { w: self }
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&mut self) -> SYNC_PE_W {
        SYNC_PE_W { w: self }
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&mut self) -> ARX_DONE_W {
        ARX_DONE_W { w: self }
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&mut self) -> ARX_PE_W {
        ARX_PE_W { w: self }
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&mut self) -> ARX_BREAK_W {
        ARX_BREAK_W { w: self }
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&mut self) -> ATX_DONE_W {
        ATX_DONE_W { w: self }
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&mut self) -> ATX_PE_W {
        ATX_PE_W { w: self }
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&mut self) -> ATX_BREAK_W {
        ATX_BREAK_W { w: self }
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&mut self) -> CRX_DONE_W {
        CRX_DONE_W { w: self }
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&mut self) -> CRX_PE_W {
        CRX_PE_W { w: self }
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&mut self) -> CRX_BREAK_W {
        CRX_BREAK_W { w: self }
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&mut self) -> CTX_DONE_W {
        CTX_DONE_W { w: self }
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&mut self) -> CTX_PE_W {
        CTX_PE_W { w: self }
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&mut self) -> CTX_BREAK_W {
        CTX_BREAK_W { w: self }
    }
}
