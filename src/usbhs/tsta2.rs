#[doc = "Reader of register TSTA2"]
pub type R = crate::R<u32, super::TSTA2>;
#[doc = "Writer for register TSTA2"]
pub type W = crate::W<u32, super::TSTA2>;
#[doc = "Register TSTA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TSTA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FullDetachEn`"]
pub type FULLDETACHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FullDetachEn`"]
pub struct FULLDETACHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDETACHEN_W<'a> {
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
#[doc = "Reader of field `HSSerialMode`"]
pub type HSSERIALMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSSerialMode`"]
pub struct HSSERIALMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSSERIALMODE_W<'a> {
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
#[doc = "Reader of field `LoopBackMode`"]
pub type LOOPBACKMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LoopBackMode`"]
pub struct LOOPBACKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACKMODE_W<'a> {
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
#[doc = "Reader of field `DisableGatedClock`"]
pub type DISABLEGATEDCLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DisableGatedClock`"]
pub struct DISABLEGATEDCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLEGATEDCLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `ForceSuspendMTo1`"]
pub type FORCESUSPENDMTO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ForceSuspendMTo1`"]
pub struct FORCESUSPENDMTO1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCESUSPENDMTO1_W<'a> {
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
#[doc = "Reader of field `ByPassDpll`"]
pub type BYPASSDPLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ByPassDpll`"]
pub struct BYPASSDPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSDPLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `HostHSDisconnectDisable`"]
pub type HOSTHSDISCONNECTDISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HostHSDisconnectDisable`"]
pub struct HOSTHSDISCONNECTDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTHSDISCONNECTDISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ForceHSRst_50ms`"]
pub type FORCEHSRST_50MS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ForceHSRst_50ms`"]
pub struct FORCEHSRST_50MS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEHSRST_50MS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RemovePUWhenTX`"]
pub type REMOVEPUWHENTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RemovePUWhenTX`"]
pub struct REMOVEPUWHENTX_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVEPUWHENTX_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&self) -> FULLDETACHEN_R {
        FULLDETACHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&self) -> HSSERIALMODE_R {
        HSSERIALMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&self) -> LOOPBACKMODE_R {
        LOOPBACKMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&self) -> DISABLEGATEDCLOCK_R {
        DISABLEGATEDCLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&self) -> FORCESUSPENDMTO1_R {
        FORCESUSPENDMTO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&self) -> BYPASSDPLL_R {
        BYPASSDPLL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&self) -> HOSTHSDISCONNECTDISABLE_R {
        HOSTHSDISCONNECTDISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&self) -> FORCEHSRST_50MS_R {
        FORCEHSRST_50MS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&self) -> REMOVEPUWHENTX_R {
        REMOVEPUWHENTX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&mut self) -> FULLDETACHEN_W {
        FULLDETACHEN_W { w: self }
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&mut self) -> HSSERIALMODE_W {
        HSSERIALMODE_W { w: self }
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&mut self) -> LOOPBACKMODE_W {
        LOOPBACKMODE_W { w: self }
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&mut self) -> DISABLEGATEDCLOCK_W {
        DISABLEGATEDCLOCK_W { w: self }
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&mut self) -> FORCESUSPENDMTO1_W {
        FORCESUSPENDMTO1_W { w: self }
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&mut self) -> BYPASSDPLL_W {
        BYPASSDPLL_W { w: self }
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&mut self) -> HOSTHSDISCONNECTDISABLE_W {
        HOSTHSDISCONNECTDISABLE_W { w: self }
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&mut self) -> FORCEHSRST_50MS_W {
        FORCEHSRST_50MS_W { w: self }
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&mut self) -> REMOVEPUWHENTX_W {
        REMOVEPUWHENTX_W { w: self }
    }
}
