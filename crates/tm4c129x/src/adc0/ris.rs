#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Reader of field `INR0`"]
pub type INR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR1`"]
pub type INR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR2`"]
pub type INR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `INR3`"]
pub type INR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAINR0`"]
pub type DMAINR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAINR1`"]
pub type DMAINR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAINR2`"]
pub type DMAINR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMAINR3`"]
pub type DMAINR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `INRDC`"]
pub type INRDC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn inr0(&self) -> INR0_R {
        INR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn inr1(&self) -> INR1_R {
        INR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn inr2(&self) -> INR2_R {
        INR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn inr3(&self) -> INR3_R {
        INR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SS0 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmainr0(&self) -> DMAINR0_R {
        DMAINR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SS1 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmainr1(&self) -> DMAINR1_R {
        DMAINR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SS2 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmainr2(&self) -> DMAINR2_R {
        DMAINR2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SS3 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn dmainr3(&self) -> DMAINR3_R {
        DMAINR3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn inrdc(&self) -> INRDC_R {
        INRDC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
