#[doc = "Reader of register TSTA1"]
pub type R = crate::R<u32, super::TSTA1>;
#[doc = "Writer for register TSTA1"]
pub type W = crate::W<u32, super::TSTA1>;
#[doc = "Register TSTA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSTA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CounterA`"]
pub type COUNTERA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CounterA`"]
pub struct COUNTERA_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `LoadCntA`"]
pub type LOADCNTA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LoadCntA`"]
pub struct LOADCNTA_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADCNTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CounterB`"]
pub type COUNTERB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CounterB`"]
pub struct COUNTERB_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LoadCntB`"]
pub type LOADCNTB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LoadCntB`"]
pub struct LOADCNTB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADCNTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `SOFCntMa1`"]
pub type SOFCNTMA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOFCntMa1`"]
pub struct SOFCNTMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFCNTMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
#[doc = "Reader of field `LoadSOFCnt`"]
pub type LOADSOFCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LoadSOFCnt`"]
pub struct LOADSOFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADSOFCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&self) -> COUNTERA_R {
        COUNTERA_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&self) -> LOADCNTA_R {
        LOADCNTA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&self) -> COUNTERB_R {
        COUNTERB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&self) -> LOADCNTB_R {
        LOADCNTB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&self) -> SOFCNTMA1_R {
        SOFCNTMA1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&self) -> LOADSOFCNT_R {
        LOADSOFCNT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&mut self) -> COUNTERA_W {
        COUNTERA_W { w: self }
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&mut self) -> LOADCNTA_W {
        LOADCNTA_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&mut self) -> COUNTERB_W {
        COUNTERB_W { w: self }
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&mut self) -> LOADCNTB_W {
        LOADCNTB_W { w: self }
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&mut self) -> SOFCNTMA1_W {
        SOFCNTMA1_W { w: self }
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&mut self) -> LOADSOFCNT_W {
        LOADSOFCNT_W { w: self }
    }
}
