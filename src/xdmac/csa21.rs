#[doc = "Register `CSA21` reader"]
pub struct R(crate::R<CSA21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSA21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSA21_SPEC>> for R {
    fn from(reader: crate::R<CSA21_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSA21` writer"]
pub struct W(crate::W<CSA21_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSA21_SPEC>;
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
impl core::convert::From<crate::W<CSA21_SPEC>> for W {
    fn from(writer: crate::W<CSA21_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SA` reader - Channel x Source Address"]
pub struct SA_R(crate::FieldReader<u32, u32>);
impl SA_R {
    pub(crate) fn new(bits: u32) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` writer - Channel x Source Address"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Source Address Register (chid = 21)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csa21](index.html) module"]
pub struct CSA21_SPEC;
impl crate::RegisterSpec for CSA21_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csa21::R](R) reader structure"]
impl crate::Readable for CSA21_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csa21::W](W) writer structure"]
impl crate::Writable for CSA21_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSA21 to value 0"]
impl crate::Resettable for CSA21_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
