#[doc = "Reader of register MSD"]
pub type R = crate::R<u32, super::MSD>;
#[doc = "Reader of field `SD0`"]
pub type SD0_R = crate::R<u8, u8>;
#[doc = "Reader of field `SD1`"]
pub type SD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `SD2`"]
pub type SD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `SD3`"]
pub type SD3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - System Data (Byte 0)"]
    #[inline(always)]
    pub fn sd0(&self) -> SD0_R {
        SD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System Data (Byte 1)"]
    #[inline(always)]
    pub fn sd1(&self) -> SD1_R {
        SD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System Data (Byte 2)"]
    #[inline(always)]
    pub fn sd2(&self) -> SD2_R {
        SD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System Data (Byte 3)"]
    #[inline(always)]
    pub fn sd3(&self) -> SD3_R {
        SD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
