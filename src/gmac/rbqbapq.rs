#[doc = "Register `RBQBAPQ[%s]` reader"]
pub struct R(crate::R<RBQBAPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBQBAPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RBQBAPQ_SPEC>> for R {
    fn from(reader: crate::R<RBQBAPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RBQBAPQ[%s]` writer"]
pub struct W(crate::W<RBQBAPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBQBAPQ_SPEC>;
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
impl core::convert::From<crate::W<RBQBAPQ_SPEC>> for W {
    fn from(writer: crate::W<RBQBAPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBQBA` reader - Receive Buffer Queue Base Address"]
pub struct RXBQBA_R(crate::FieldReader<u8, u8>);
impl RXBQBA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBQBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBQBA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBQBA` writer - Receive Buffer Queue Base Address"]
pub struct RXBQBA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBQBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&self) -> RXBQBA_R {
        RXBQBA_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&mut self) -> RXBQBA_W {
        RXBQBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Buffer Queue Base Address Register Priority Queue (index = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqbapq](index.html) module"]
pub struct RBQBAPQ_SPEC;
impl crate::RegisterSpec for RBQBAPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbqbapq::R](R) reader structure"]
impl crate::Readable for RBQBAPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbqbapq::W](W) writer structure"]
impl crate::Writable for RBQBAPQ_SPEC {
    type Writer = W;
}
