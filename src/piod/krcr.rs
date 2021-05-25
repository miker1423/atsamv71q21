#[doc = "Register `KRCR` reader"]
pub struct R(crate::R<KRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KRCR_SPEC>> for R {
    fn from(reader: crate::R<KRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KRCR` writer"]
pub struct W(crate::W<KRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRCR_SPEC>;
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
impl core::convert::From<crate::W<KRCR_SPEC>> for W {
    fn from(writer: crate::W<KRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NBR` reader - Number of Columns of the Keypad Matrix"]
pub struct NBR_R(crate::FieldReader<u8, u8>);
impl NBR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBR` writer - Number of Columns of the Keypad Matrix"]
pub struct NBR_W<'a> {
    w: &'a mut W,
}
impl<'a> NBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `NBC` reader - Number of Rows of the Keypad Matrix"]
pub struct NBC_R(crate::FieldReader<u8, u8>);
impl NBC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBC` writer - Number of Rows of the Keypad Matrix"]
pub struct NBC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of Columns of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbr(&self) -> NBR_R {
        NBR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Number of Rows of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of Columns of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbr(&mut self) -> NBR_W {
        NBR_W { w: self }
    }
    #[doc = "Bits 8:10 - Number of Rows of the Keypad Matrix"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W {
        NBC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Controller Row Column Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [krcr](index.html) module"]
pub struct KRCR_SPEC;
impl crate::RegisterSpec for KRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [krcr::R](R) reader structure"]
impl crate::Readable for KRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [krcr::W](W) writer structure"]
impl crate::Writable for KRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KRCR to value 0"]
impl crate::Resettable for KRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
