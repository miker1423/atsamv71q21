#[doc = "Register `KKPR` reader"]
pub struct R(crate::R<KKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<KKPR_SPEC>> for R {
    fn from(reader: crate::R<KKPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY0ROW` reader - Row Index of the First Detected Key Press"]
pub struct KEY0ROW_R(crate::FieldReader<u8, u8>);
impl KEY0ROW_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY0ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0ROW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY0COL` reader - Column Index of the First Detected Key Press"]
pub struct KEY0COL_R(crate::FieldReader<u8, u8>);
impl KEY0COL_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY0COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY0COL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1ROW` reader - Row Index of the Second Detected Key Press"]
pub struct KEY1ROW_R(crate::FieldReader<u8, u8>);
impl KEY1ROW_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY1ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1ROW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY1COL` reader - Column Index of the Second Detected Key Press"]
pub struct KEY1COL_R(crate::FieldReader<u8, u8>);
impl KEY1COL_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY1COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY1COL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2ROW` reader - Row Index of the Third Detected Key Press"]
pub struct KEY2ROW_R(crate::FieldReader<u8, u8>);
impl KEY2ROW_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY2ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2ROW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY2COL` reader - Column Index of the Third Detected Key Press"]
pub struct KEY2COL_R(crate::FieldReader<u8, u8>);
impl KEY2COL_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY2COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY2COL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3ROW` reader - Row Index of the Fourth Detected Key Press"]
pub struct KEY3ROW_R(crate::FieldReader<u8, u8>);
impl KEY3ROW_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY3ROW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3ROW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY3COL` reader - Column Index of the Fourth Detected Key Press"]
pub struct KEY3COL_R(crate::FieldReader<u8, u8>);
impl KEY3COL_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY3COL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY3COL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Row Index of the First Detected Key Press"]
    #[inline(always)]
    pub fn key0row(&self) -> KEY0ROW_R {
        KEY0ROW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Column Index of the First Detected Key Press"]
    #[inline(always)]
    pub fn key0col(&self) -> KEY0COL_R {
        KEY0COL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Row Index of the Second Detected Key Press"]
    #[inline(always)]
    pub fn key1row(&self) -> KEY1ROW_R {
        KEY1ROW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Column Index of the Second Detected Key Press"]
    #[inline(always)]
    pub fn key1col(&self) -> KEY1COL_R {
        KEY1COL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Row Index of the Third Detected Key Press"]
    #[inline(always)]
    pub fn key2row(&self) -> KEY2ROW_R {
        KEY2ROW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Column Index of the Third Detected Key Press"]
    #[inline(always)]
    pub fn key2col(&self) -> KEY2COL_R {
        KEY2COL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Row Index of the Fourth Detected Key Press"]
    #[inline(always)]
    pub fn key3row(&self) -> KEY3ROW_R {
        KEY3ROW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Column Index of the Fourth Detected Key Press"]
    #[inline(always)]
    pub fn key3col(&self) -> KEY3COL_R {
        KEY3COL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
#[doc = "Keypad Controller Key Press Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kkpr](index.html) module"]
pub struct KKPR_SPEC;
impl crate::RegisterSpec for KKPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kkpr::R](R) reader structure"]
impl crate::Readable for KKPR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets KKPR to value 0"]
impl crate::Resettable for KKPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
