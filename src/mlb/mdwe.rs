#[doc = "Register `MDWE[%s]` reader"]
pub struct R(crate::R<MDWE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDWE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MDWE_SPEC>> for R {
    fn from(reader: crate::R<MDWE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDWE[%s]` writer"]
pub struct W(crate::W<MDWE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDWE_SPEC>;
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
impl core::convert::From<crate::W<MDWE_SPEC>> for W {
    fn from(writer: crate::W<MDWE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Bitwise write enable for CTR data - bits\\[31:0\\]"]
pub struct MASK_R(crate::FieldReader<u32, u32>);
impl MASK_R {
    pub(crate) fn new(bits: u32) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Bitwise write enable for CTR data - bits\\[31:0\\]"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise write enable for CTR data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise write enable for CTR data - bits\\[31:0\\]"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIF Data Write Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdwe](index.html) module"]
pub struct MDWE_SPEC;
impl crate::RegisterSpec for MDWE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdwe::R](R) reader structure"]
impl crate::Readable for MDWE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdwe::W](W) writer structure"]
impl crate::Writable for MDWE_SPEC {
    type Writer = W;
}
