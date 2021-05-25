#[doc = "Register `CDS_MSP3` reader"]
pub struct R(crate::R<CDS_MSP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDS_MSP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CDS_MSP3_SPEC>> for R {
    fn from(reader: crate::R<CDS_MSP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDS_MSP3` writer"]
pub struct W(crate::W<CDS_MSP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDS_MSP3_SPEC>;
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
impl core::convert::From<crate::W<CDS_MSP3_SPEC>> for W {
    fn from(writer: crate::W<CDS_MSP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDS_MSP` reader - Channel x Source Data stride or Memory Set Pattern"]
pub struct SDS_MSP_R(crate::FieldReader<u16, u16>);
impl SDS_MSP_R {
    pub(crate) fn new(bits: u16) -> Self {
        SDS_MSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDS_MSP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDS_MSP` writer - Channel x Source Data stride or Memory Set Pattern"]
pub struct SDS_MSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDS_MSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `DDS_MSP` reader - Channel x Destination Data Stride or Memory Set Pattern"]
pub struct DDS_MSP_R(crate::FieldReader<u16, u16>);
impl DDS_MSP_R {
    pub(crate) fn new(bits: u16) -> Self {
        DDS_MSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDS_MSP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDS_MSP` writer - Channel x Destination Data Stride or Memory Set Pattern"]
pub struct DDS_MSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_MSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&self) -> SDS_MSP_R {
        SDS_MSP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&self) -> DDS_MSP_R {
        DDS_MSP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&mut self) -> SDS_MSP_W {
        SDS_MSP_W { w: self }
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&mut self) -> DDS_MSP_W {
        DDS_MSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cds_msp3](index.html) module"]
pub struct CDS_MSP3_SPEC;
impl crate::RegisterSpec for CDS_MSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cds_msp3::R](R) reader structure"]
impl crate::Readable for CDS_MSP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cds_msp3::W](W) writer structure"]
impl crate::Writable for CDS_MSP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDS_MSP3 to value 0"]
impl crate::Resettable for CDS_MSP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
