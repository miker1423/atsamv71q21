#[doc = "Reader of register ACTL"]
pub type R = crate::R<u32, super::ACTL>;
#[doc = "Writer for register ACTL"]
pub type W = crate::W<u32, super::ACTL>;
#[doc = "Register ACTL `reset()`'s with value 0"]
impl crate::ResetValue for super::ACTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCE`"]
pub type SCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCE`"]
pub struct SCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_W<'a> {
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
#[doc = "Reader of field `SMX`"]
pub type SMX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMX`"]
pub struct SMX_W<'a> {
    w: &'a mut W,
}
impl<'a> SMX_W<'a> {
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
#[doc = "Reader of field `DMA_MODE`"]
pub type DMA_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA_MODE`"]
pub struct DMA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Packet Buffering Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPB_A {
    #[doc = "0: Single-packet mode"]
    SINGLE_PACKET = 0,
    #[doc = "1: Multiple-packet mode"]
    MULTIPLE_PACKET = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MPB`"]
pub type MPB_R = crate::R<bool, MPB_A>;
impl MPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::SINGLE_PACKET,
            true => MPB_A::MULTIPLE_PACKET,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_PACKET`"]
    #[inline(always)]
    pub fn is_single_packet(&self) -> bool {
        *self == MPB_A::SINGLE_PACKET
    }
    #[doc = "Checks if the value of the field is `MULTIPLE_PACKET`"]
    #[inline(always)]
    pub fn is_multiple_packet(&self) -> bool {
        *self == MPB_A::MULTIPLE_PACKET
    }
}
#[doc = "Write proxy for field `MPB`"]
pub struct MPB_W<'a> {
    w: &'a mut W,
}
impl<'a> MPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn single_packet(self) -> &'a mut W {
        self.variant(MPB_A::SINGLE_PACKET)
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn multiple_packet(self) -> &'a mut W {
        self.variant(MPB_A::MULTIPLE_PACKET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&self) -> SMX_R {
        SMX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DMA_MODE_R {
        DMA_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&mut self) -> SCE_W {
        SCE_W { w: self }
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&mut self) -> SMX_W {
        SMX_W { w: self }
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DMA_MODE_W {
        DMA_MODE_W { w: self }
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&mut self) -> MPB_W {
        MPB_W { w: self }
    }
}
