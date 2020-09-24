#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Write proxy for field `RDERRIC`"]
pub struct RDERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIC_W<'a> {
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
#[doc = "Write proxy for field `VBUSRQC`"]
pub struct VBUSRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Clear"]
    #[inline(always)]
    pub fn rderric(&mut self) -> RDERRIC_W {
        RDERRIC_W { w: self }
    }
    #[doc = "Bit 9 - VBus Request Clear"]
    #[inline(always)]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W {
        VBUSRQC_W { w: self }
    }
}
