#[doc = "Reader of register DC7"]
pub type R = crate::R<u32, super::DC7>;
#[doc = "Reader of field `DMACH0`"]
pub type DMACH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH1`"]
pub type DMACH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH2`"]
pub type DMACH2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH3`"]
pub type DMACH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH4`"]
pub type DMACH4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH5`"]
pub type DMACH5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH6`"]
pub type DMACH6_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH7`"]
pub type DMACH7_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH8`"]
pub type DMACH8_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH9`"]
pub type DMACH9_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH10`"]
pub type DMACH10_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH11`"]
pub type DMACH11_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH12`"]
pub type DMACH12_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH13`"]
pub type DMACH13_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH14`"]
pub type DMACH14_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH15`"]
pub type DMACH15_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH16`"]
pub type DMACH16_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH17`"]
pub type DMACH17_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH18`"]
pub type DMACH18_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH19`"]
pub type DMACH19_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH20`"]
pub type DMACH20_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH21`"]
pub type DMACH21_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH22`"]
pub type DMACH22_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH23`"]
pub type DMACH23_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH24`"]
pub type DMACH24_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH25`"]
pub type DMACH25_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH26`"]
pub type DMACH26_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH27`"]
pub type DMACH27_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH28`"]
pub type DMACH28_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH29`"]
pub type DMACH29_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMACH30`"]
pub type DMACH30_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DMA Channel 0"]
    #[inline(always)]
    pub fn dmach0(&self) -> DMACH0_R {
        DMACH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1"]
    #[inline(always)]
    pub fn dmach1(&self) -> DMACH1_R {
        DMACH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2"]
    #[inline(always)]
    pub fn dmach2(&self) -> DMACH2_R {
        DMACH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3"]
    #[inline(always)]
    pub fn dmach3(&self) -> DMACH3_R {
        DMACH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4"]
    #[inline(always)]
    pub fn dmach4(&self) -> DMACH4_R {
        DMACH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5"]
    #[inline(always)]
    pub fn dmach5(&self) -> DMACH5_R {
        DMACH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6"]
    #[inline(always)]
    pub fn dmach6(&self) -> DMACH6_R {
        DMACH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Channel 7"]
    #[inline(always)]
    pub fn dmach7(&self) -> DMACH7_R {
        DMACH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA Channel 8"]
    #[inline(always)]
    pub fn dmach8(&self) -> DMACH8_R {
        DMACH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMA Channel 9"]
    #[inline(always)]
    pub fn dmach9(&self) -> DMACH9_R {
        DMACH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DMA Channel 10"]
    #[inline(always)]
    pub fn dmach10(&self) -> DMACH10_R {
        DMACH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DMA Channel 11"]
    #[inline(always)]
    pub fn dmach11(&self) -> DMACH11_R {
        DMACH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DMA Channel 12"]
    #[inline(always)]
    pub fn dmach12(&self) -> DMACH12_R {
        DMACH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DMA Channel 13"]
    #[inline(always)]
    pub fn dmach13(&self) -> DMACH13_R {
        DMACH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DMA Channel 14"]
    #[inline(always)]
    pub fn dmach14(&self) -> DMACH14_R {
        DMACH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DMA Channel 15"]
    #[inline(always)]
    pub fn dmach15(&self) -> DMACH15_R {
        DMACH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DMA Channel 16"]
    #[inline(always)]
    pub fn dmach16(&self) -> DMACH16_R {
        DMACH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DMA Channel 17"]
    #[inline(always)]
    pub fn dmach17(&self) -> DMACH17_R {
        DMACH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DMA Channel 18"]
    #[inline(always)]
    pub fn dmach18(&self) -> DMACH18_R {
        DMACH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DMA Channel 19"]
    #[inline(always)]
    pub fn dmach19(&self) -> DMACH19_R {
        DMACH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - DMA Channel 20"]
    #[inline(always)]
    pub fn dmach20(&self) -> DMACH20_R {
        DMACH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DMA Channel 21"]
    #[inline(always)]
    pub fn dmach21(&self) -> DMACH21_R {
        DMACH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DMA Channel 22"]
    #[inline(always)]
    pub fn dmach22(&self) -> DMACH22_R {
        DMACH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DMA Channel 23"]
    #[inline(always)]
    pub fn dmach23(&self) -> DMACH23_R {
        DMACH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Channel 24"]
    #[inline(always)]
    pub fn dmach24(&self) -> DMACH24_R {
        DMACH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 25"]
    #[inline(always)]
    pub fn dmach25(&self) -> DMACH25_R {
        DMACH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 26"]
    #[inline(always)]
    pub fn dmach26(&self) -> DMACH26_R {
        DMACH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 27"]
    #[inline(always)]
    pub fn dmach27(&self) -> DMACH27_R {
        DMACH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 28"]
    #[inline(always)]
    pub fn dmach28(&self) -> DMACH28_R {
        DMACH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 29"]
    #[inline(always)]
    pub fn dmach29(&self) -> DMACH29_R {
        DMACH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 30"]
    #[inline(always)]
    pub fn dmach30(&self) -> DMACH30_R {
        DMACH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
