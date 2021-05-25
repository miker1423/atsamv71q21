#[doc = "Register `ACSR[%s]` reader"]
pub struct R(crate::R<ACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACSR_SPEC>> for R {
    fn from(reader: crate::R<ACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACSR[%s]` writer"]
pub struct W(crate::W<ACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACSR_SPEC>;
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
impl core::convert::From<crate::W<ACSR_SPEC>> for W {
    fn from(writer: crate::W<ACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHS` reader - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub struct CHS_R(crate::FieldReader<u32, u32>);
impl CHS_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHS` writer - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub struct CHS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&mut self) -> CHS_W {
        CHS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acsr](index.html) module"]
pub struct ACSR_SPEC;
impl crate::RegisterSpec for ACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acsr::R](R) reader structure"]
impl crate::Readable for ACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acsr::W](W) writer structure"]
impl crate::Writable for ACSR_SPEC {
    type Writer = W;
}
