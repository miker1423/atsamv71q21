#[doc = "Reader of register MSS"]
pub type R = crate::R<u32, super::MSS>;
#[doc = "Writer for register MSS"]
pub type W = crate::W<u32, super::MSS>;
#[doc = "Register MSS `reset()`'s with value 0"]
impl crate::ResetValue for super::MSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSTSYSCMD`"]
pub type RSTSYSCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTSYSCMD`"]
pub struct RSTSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSYSCMD_W<'a> {
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
#[doc = "Reader of field `LKSYSCMD`"]
pub type LKSYSCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LKSYSCMD`"]
pub struct LKSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> LKSYSCMD_W<'a> {
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
#[doc = "Reader of field `ULKSYSCMD`"]
pub type ULKSYSCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ULKSYSCMD`"]
pub struct ULKSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ULKSYSCMD_W<'a> {
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
#[doc = "Reader of field `CSSYSCMD`"]
pub type CSSYSCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSSYSCMD`"]
pub struct CSSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSYSCMD_W<'a> {
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
#[doc = "Reader of field `SWSYSCMD`"]
pub type SWSYSCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWSYSCMD`"]
pub struct SWSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYSCMD_W<'a> {
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
#[doc = "Reader of field `SERVREQ`"]
pub type SERVREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SERVREQ`"]
pub struct SERVREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SERVREQ_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&self) -> RSTSYSCMD_R {
        RSTSYSCMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&self) -> LKSYSCMD_R {
        LKSYSCMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&self) -> ULKSYSCMD_R {
        ULKSYSCMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&self) -> CSSYSCMD_R {
        CSSYSCMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&self) -> SWSYSCMD_R {
        SWSYSCMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&self) -> SERVREQ_R {
        SERVREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&mut self) -> RSTSYSCMD_W {
        RSTSYSCMD_W { w: self }
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&mut self) -> LKSYSCMD_W {
        LKSYSCMD_W { w: self }
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&mut self) -> ULKSYSCMD_W {
        ULKSYSCMD_W { w: self }
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&mut self) -> CSSYSCMD_W {
        CSSYSCMD_W { w: self }
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&mut self) -> SWSYSCMD_W {
        SWSYSCMD_W { w: self }
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&mut self) -> SERVREQ_W {
        SERVREQ_W { w: self }
    }
}
