#[doc = "Reader of register ST2CW019"]
pub type R = crate::R<u32, super::ST2CW019>;
#[doc = "Writer for register ST2CW019"]
pub type W = crate::W<u32, super::ST2CW019>;
#[doc = "Register ST2CW019 `reset()`'s with value 0"]
impl crate::ResetValue for super::ST2CW019 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MASKVAL`"]
pub type MASKVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MASKVAL`"]
pub struct MASKVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MASKVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `COMPVAL`"]
pub type COMPVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COMPVAL`"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W {
        MASKVAL_W { w: self }
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
}