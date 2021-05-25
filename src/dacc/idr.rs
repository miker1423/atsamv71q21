#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<IDR_SPEC>> for W {
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXRDY0` writer - Transmit Ready Interrupt Disable of channel 0"]
pub struct TXRDY0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TXRDY1` writer - Transmit Ready Interrupt Disable of channel 1"]
pub struct TXRDY1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRDY1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EOC0` writer - End of Conversion Interrupt Disable of channel 0"]
pub struct EOC0_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `EOC1` writer - End of Conversion Interrupt Disable of channel 1"]
pub struct EOC1_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ENDTX0` writer - End of Transmit Buffer Interrupt Disable of channel 0"]
pub struct ENDTX0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ENDTX1` writer - End of Transmit Buffer Interrupt Disable of channel 1"]
pub struct ENDTX1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TXBUFE0` writer - Transmit Buffer Empty Interrupt Disable of channel 0"]
pub struct TXBUFE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUFE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `TXBUFE1` writer - Transmit Buffer Empty Interrupt Disable of channel 1"]
pub struct TXBUFE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUFE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Ready Interrupt Disable of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&mut self) -> TXRDY0_W {
        TXRDY0_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Disable of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&mut self) -> TXRDY1_W {
        TXRDY1_W { w: self }
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Disable of channel 0"]
    #[inline(always)]
    pub fn eoc0(&mut self) -> EOC0_W {
        EOC0_W { w: self }
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Disable of channel 1"]
    #[inline(always)]
    pub fn eoc1(&mut self) -> EOC1_W {
        EOC1_W { w: self }
    }
    #[doc = "Bit 8 - End of Transmit Buffer Interrupt Disable of channel 0"]
    #[inline(always)]
    pub fn endtx0(&mut self) -> ENDTX0_W {
        ENDTX0_W { w: self }
    }
    #[doc = "Bit 9 - End of Transmit Buffer Interrupt Disable of channel 1"]
    #[inline(always)]
    pub fn endtx1(&mut self) -> ENDTX1_W {
        ENDTX1_W { w: self }
    }
    #[doc = "Bit 12 - Transmit Buffer Empty Interrupt Disable of channel 0"]
    #[inline(always)]
    pub fn txbufe0(&mut self) -> TXBUFE0_W {
        TXBUFE0_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Buffer Empty Interrupt Disable of channel 1"]
    #[inline(always)]
    pub fn txbufe1(&mut self) -> TXBUFE1_W {
        TXBUFE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
}
