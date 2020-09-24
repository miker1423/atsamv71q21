#[doc = "Reader of register KKRR"]
pub type R = crate::R<u32, super::KKRR>;
#[doc = "Reader of field `KEY0ROW`"]
pub type KEY0ROW_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY0COL`"]
pub type KEY0COL_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY1ROW`"]
pub type KEY1ROW_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY1COL`"]
pub type KEY1COL_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY2ROW`"]
pub type KEY2ROW_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY2COL`"]
pub type KEY2COL_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY3ROW`"]
pub type KEY3ROW_R = crate::R<u8, u8>;
#[doc = "Reader of field `KEY3COL`"]
pub type KEY3COL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Row Index of the First Detected Key Release"]
    #[inline(always)]
    pub fn key0row(&self) -> KEY0ROW_R {
        KEY0ROW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Column Index of the First Detected Key Release"]
    #[inline(always)]
    pub fn key0col(&self) -> KEY0COL_R {
        KEY0COL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Row Index of the Second Detected Key Release"]
    #[inline(always)]
    pub fn key1row(&self) -> KEY1ROW_R {
        KEY1ROW_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Column Index of the Second Detected Key Release"]
    #[inline(always)]
    pub fn key1col(&self) -> KEY1COL_R {
        KEY1COL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Row Index of the Third Detected Key Release"]
    #[inline(always)]
    pub fn key2row(&self) -> KEY2ROW_R {
        KEY2ROW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Column Index of the Third Detected Key Release"]
    #[inline(always)]
    pub fn key2col(&self) -> KEY2COL_R {
        KEY2COL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Row Index of the Fourth Detected Key Release"]
    #[inline(always)]
    pub fn key3row(&self) -> KEY3ROW_R {
        KEY3ROW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Column Index of the Fourth Detected Key Release"]
    #[inline(always)]
    pub fn key3col(&self) -> KEY3COL_R {
        KEY3COL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
