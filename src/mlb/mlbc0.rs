#[doc = "Reader of register MLBC0"]
pub type R = crate::R<u32, super::MLBC0>;
#[doc = "Writer for register MLBC0"]
pub type W = crate::W<u32, super::MLBC0>;
#[doc = "Register MLBC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MLBC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MLBEN`"]
pub type MLBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MLBEN`"]
pub struct MLBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "MLB_CLK (MediaLB clock) speed select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MLBCLK_A {
    #[doc = "0: 256xFs (for MLBPEN = 0)"]
    _256_FS = 0,
    #[doc = "1: 512xFs (for MLBPEN = 0)"]
    _512_FS = 1,
    #[doc = "2: 1024xFs (for MLBPEN = 0)"]
    _1024_FS = 2,
}
impl From<MLBCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: MLBCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MLBCLK`"]
pub type MLBCLK_R = crate::R<u8, MLBCLK_A>;
impl MLBCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MLBCLK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MLBCLK_A::_256_FS),
            1 => Val(MLBCLK_A::_512_FS),
            2 => Val(MLBCLK_A::_1024_FS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256_FS`"]
    #[inline(always)]
    pub fn is_256_fs(&self) -> bool {
        *self == MLBCLK_A::_256_FS
    }
    #[doc = "Checks if the value of the field is `_512_FS`"]
    #[inline(always)]
    pub fn is_512_fs(&self) -> bool {
        *self == MLBCLK_A::_512_FS
    }
    #[doc = "Checks if the value of the field is `_1024_FS`"]
    #[inline(always)]
    pub fn is_1024_fs(&self) -> bool {
        *self == MLBCLK_A::_1024_FS
    }
}
#[doc = "Write proxy for field `MLBCLK`"]
pub struct MLBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLBCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _256_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_256_FS)
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _512_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_512_FS)
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _1024_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_1024_FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `ZERO`"]
pub type ZERO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ZERO`"]
pub struct ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> ZERO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MLBLK`"]
pub type MLBLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MLBLK`"]
pub struct MLBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `ASYRETRY`"]
pub type ASYRETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYRETRY`"]
pub struct ASYRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYRETRY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CTLRETRY`"]
pub type CTLRETRY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTLRETRY`"]
pub struct CTLRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLRETRY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "The number of frames per sub-buffer for synchronous channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCNT_A {
    #[doc = "0: 1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    _1_FRAME = 0,
    #[doc = "1: 2 frames per sub-buffer"]
    _2_FRAMES = 1,
    #[doc = "2: 4 frames per sub-buffer"]
    _4_FRAMES = 2,
    #[doc = "3: 8 frames per sub-buffer"]
    _8_FRAMES = 3,
    #[doc = "4: 16 frames per sub-buffer"]
    _16_FRAMES = 4,
    #[doc = "5: 32 frames per sub-buffer"]
    _32_FRAMES = 5,
    #[doc = "6: 64 frames per sub-buffer"]
    _64_FRAMES = 6,
}
impl From<FCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FCNT`"]
pub type FCNT_R = crate::R<u8, FCNT_A>;
impl FCNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FCNT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FCNT_A::_1_FRAME),
            1 => Val(FCNT_A::_2_FRAMES),
            2 => Val(FCNT_A::_4_FRAMES),
            3 => Val(FCNT_A::_8_FRAMES),
            4 => Val(FCNT_A::_16_FRAMES),
            5 => Val(FCNT_A::_32_FRAMES),
            6 => Val(FCNT_A::_64_FRAMES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_FRAME`"]
    #[inline(always)]
    pub fn is_1_frame(&self) -> bool {
        *self == FCNT_A::_1_FRAME
    }
    #[doc = "Checks if the value of the field is `_2_FRAMES`"]
    #[inline(always)]
    pub fn is_2_frames(&self) -> bool {
        *self == FCNT_A::_2_FRAMES
    }
    #[doc = "Checks if the value of the field is `_4_FRAMES`"]
    #[inline(always)]
    pub fn is_4_frames(&self) -> bool {
        *self == FCNT_A::_4_FRAMES
    }
    #[doc = "Checks if the value of the field is `_8_FRAMES`"]
    #[inline(always)]
    pub fn is_8_frames(&self) -> bool {
        *self == FCNT_A::_8_FRAMES
    }
    #[doc = "Checks if the value of the field is `_16_FRAMES`"]
    #[inline(always)]
    pub fn is_16_frames(&self) -> bool {
        *self == FCNT_A::_16_FRAMES
    }
    #[doc = "Checks if the value of the field is `_32_FRAMES`"]
    #[inline(always)]
    pub fn is_32_frames(&self) -> bool {
        *self == FCNT_A::_32_FRAMES
    }
    #[doc = "Checks if the value of the field is `_64_FRAMES`"]
    #[inline(always)]
    pub fn is_64_frames(&self) -> bool {
        *self == FCNT_A::_64_FRAMES
    }
}
#[doc = "Write proxy for field `FCNT`"]
pub struct FCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn _1_frame(self) -> &'a mut W {
        self.variant(FCNT_A::_1_FRAME)
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn _2_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_2_FRAMES)
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn _4_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_4_FRAMES)
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn _8_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_8_FRAMES)
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn _16_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_16_FRAMES)
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn _32_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_32_FRAMES)
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn _64_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_64_FRAMES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&self) -> MLBEN_R {
        MLBEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - MLB_CLK (MediaLB clock) speed select"]
    #[inline(always)]
    pub fn mlbclk(&self) -> MLBCLK_R {
        MLBCLK_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&self) -> MLBLK_R {
        MLBLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&self) -> ASYRETRY_R {
        ASYRETRY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&self) -> CTLRETRY_R {
        CTLRETRY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new(((self.bits >> 15) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&mut self) -> MLBEN_W {
        MLBEN_W { w: self }
    }
    #[doc = "Bits 2:4 - MLB_CLK (MediaLB clock) speed select"]
    #[inline(always)]
    pub fn mlbclk(&mut self) -> MLBCLK_W {
        MLBCLK_W { w: self }
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&mut self) -> ZERO_W {
        ZERO_W { w: self }
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&mut self) -> MLBLK_W {
        MLBLK_W { w: self }
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&mut self) -> ASYRETRY_W {
        ASYRETRY_W { w: self }
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&mut self) -> CTLRETRY_W {
        CTLRETRY_W { w: self }
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&mut self) -> FCNT_W {
        FCNT_W { w: self }
    }
}
