#[doc = "Register `GCMHR[%s]` reader"]
pub struct R(crate::R<GCMHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCMHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GCMHR_SPEC>> for R {
    fn from(reader: crate::R<GCMHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCMHR[%s]` writer"]
pub struct W(crate::W<GCMHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCMHR_SPEC>;
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
impl core::convert::From<crate::W<GCMHR_SPEC>> for W {
    fn from(writer: crate::W<GCMHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H` reader - GCM H Word x"]
pub struct H_R(crate::FieldReader<u32, u32>);
impl H_R {
    pub(crate) fn new(bits: u32) -> Self {
        H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `H` writer - GCM H Word x"]
pub struct H_W<'a> {
    w: &'a mut W,
}
impl<'a> H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GCM H Word x"]
    #[inline(always)]
    pub fn h(&mut self) -> H_W {
        H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GCM H Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcmhr](index.html) module"]
pub struct GCMHR_SPEC;
impl crate::RegisterSpec for GCMHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcmhr::R](R) reader structure"]
impl crate::Readable for GCMHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcmhr::W](W) writer structure"]
impl crate::Writable for GCMHR_SPEC {
    type Writer = W;
}
