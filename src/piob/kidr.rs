#[doc = "Writer for register KIDR"]
pub type W = crate::W<u32, super::KIDR>;
#[doc = "Write proxy for field `KPR`"]
pub struct KPR_W<'a> {
    w: &'a mut W,
}
impl<'a> KPR_W<'a> {
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
#[doc = "Write proxy for field `KRL`"]
pub struct KRL_W<'a> {
    w: &'a mut W,
}
impl<'a> KRL_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Key Press Interrupt Disable"]
    #[inline(always)]
    pub fn kpr(&mut self) -> KPR_W {
        KPR_W { w: self }
    }
    #[doc = "Bit 1 - Key Release Interrupt Disable"]
    #[inline(always)]
    pub fn krl(&mut self) -> KRL_W {
        KRL_W { w: self }
    }
}
