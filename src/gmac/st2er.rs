#[doc = "Register `ST2ER[%s]` reader"]
pub struct R(crate::R<ST2ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ST2ER_SPEC>> for R {
    fn from(reader: crate::R<ST2ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2ER[%s]` writer"]
pub struct W(crate::W<ST2ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2ER_SPEC>;
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
impl core::convert::From<crate::W<ST2ER_SPEC>> for W {
    fn from(writer: crate::W<ST2ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPVAL` reader - Ethertype Compare Value"]
pub struct COMPVAL_R(crate::FieldReader<u16, u16>);
impl COMPVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        COMPVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPVAL` writer - Ethertype Compare Value"]
pub struct COMPVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Ethertype Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Ethertype Register (index = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2er](index.html) module"]
pub struct ST2ER_SPEC;
impl crate::RegisterSpec for ST2ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2er::R](R) reader structure"]
impl crate::Readable for ST2ER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2er::W](W) writer structure"]
impl crate::Writable for ST2ER_SPEC {
    type Writer = W;
}
