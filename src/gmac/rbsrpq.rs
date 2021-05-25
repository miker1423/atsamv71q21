#[doc = "Register `RBSRPQ[%s]` reader"]
pub struct R(crate::R<RBSRPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBSRPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBSRPQ_SPEC>> for R {
    fn from(reader: crate::R<RBSRPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBSRPQ[%s]` writer"]
pub struct W(crate::W<RBSRPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBSRPQ_SPEC>;
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
impl core::convert::From<crate::W<RBSRPQ_SPEC>> for W {
    fn from(writer: crate::W<RBSRPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBS` reader - Receive Buffer Size"]
pub struct RBS_R(crate::FieldReader<u16, u16>);
impl RBS_R {
    pub(crate) fn new(bits: u16) -> Self {
        RBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBS` writer - Receive Buffer Size"]
pub struct RBS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&self) -> RBS_R {
        RBS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Buffer Size"]
    #[inline(always)]
    pub fn rbs(&mut self) -> RBS_W {
        RBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Size Register Priority Queue (index = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbsrpq](index.html) module"]
pub struct RBSRPQ_SPEC;
impl crate::RegisterSpec for RBSRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbsrpq::R](R) reader structure"]
impl crate::Readable for RBSRPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbsrpq::W](W) writer structure"]
impl crate::Writable for RBSRPQ_SPEC {
    type Writer = W;
}
