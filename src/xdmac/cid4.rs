#[doc = "Register `CID4` writer"]
pub struct W(crate::W<CID4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CID4_SPEC>;
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
impl core::convert::From<crate::W<CID4_SPEC>> for W {
    fn from(writer: crate::W<CID4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BID` writer - End of Block Interrupt Disable Bit"]
pub struct BID_W<'a> {
    w: &'a mut W,
}
impl<'a> BID_W<'a> {
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
#[doc = "Field `LID` writer - End of Linked List Interrupt Disable Bit"]
pub struct LID_W<'a> {
    w: &'a mut W,
}
impl<'a> LID_W<'a> {
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
#[doc = "Field `DID` writer - End of Disable Interrupt Disable Bit"]
pub struct DID_W<'a> {
    w: &'a mut W,
}
impl<'a> DID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FID` writer - End of Flush Interrupt Disable Bit"]
pub struct FID_W<'a> {
    w: &'a mut W,
}
impl<'a> FID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RBEID` writer - Read Bus Error Interrupt Disable Bit"]
pub struct RBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEID_W<'a> {
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
#[doc = "Field `WBEID` writer - Write Bus Error Interrupt Disable Bit"]
pub struct WBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> WBEID_W<'a> {
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
#[doc = "Field `ROID` writer - Request Overflow Error Interrupt Disable Bit"]
pub struct ROID_W<'a> {
    w: &'a mut W,
}
impl<'a> ROID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    pub fn bid(&mut self) -> BID_W {
        BID_W { w: self }
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    pub fn lid(&mut self) -> LID_W {
        LID_W { w: self }
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    pub fn did(&mut self) -> DID_W {
        DID_W { w: self }
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    pub fn fid(&mut self) -> FID_W {
        FID_W { w: self }
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn rbeid(&mut self) -> RBEID_W {
        RBEID_W { w: self }
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn wbeid(&mut self) -> WBEID_W {
        WBEID_W { w: self }
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn roid(&mut self) -> ROID_W {
        ROID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Disable Register (chid = 4)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cid4](index.html) module"]
pub struct CID4_SPEC;
impl crate::RegisterSpec for CID4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cid4::W](W) writer structure"]
impl crate::Writable for CID4_SPEC {
    type Writer = W;
}
