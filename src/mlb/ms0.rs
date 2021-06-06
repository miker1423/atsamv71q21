#[doc = "Register `MS0` reader"]
pub struct R(crate::R<MS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MS0_SPEC>> for R {
    fn from(reader: crate::R<MS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MS0` writer"]
pub struct W(crate::W<MS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MS0_SPEC>;
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
impl core::convert::From<crate::W<MS0_SPEC>> for W {
    fn from(writer: crate::W<MS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub struct MCS_R(crate::FieldReader<u32, u32>);
impl MCS_R {
    pub(crate) fn new(bits: u32) -> Self {
        MCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub struct MCS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> MCS_R {
        MCS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&mut self) -> MCS_W {
        MCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ms0](index.html) module"]
pub struct MS0_SPEC;
impl crate::RegisterSpec for MS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ms0::R](R) reader structure"]
impl crate::Readable for MS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ms0::W](W) writer structure"]
impl crate::Writable for MS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MS0 to value 0"]
impl crate::Resettable for MS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
