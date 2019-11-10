#[doc = "Reader of register DC8"]
pub type R = crate::R<u32, super::DC8>;
#[doc = "Reader of field `ADC0AIN0`"]
pub type ADC0AIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN1`"]
pub type ADC0AIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN2`"]
pub type ADC0AIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN3`"]
pub type ADC0AIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN4`"]
pub type ADC0AIN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN5`"]
pub type ADC0AIN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN6`"]
pub type ADC0AIN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN7`"]
pub type ADC0AIN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN8`"]
pub type ADC0AIN8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN9`"]
pub type ADC0AIN9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN10`"]
pub type ADC0AIN10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN11`"]
pub type ADC0AIN11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN12`"]
pub type ADC0AIN12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN13`"]
pub type ADC0AIN13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN14`"]
pub type ADC0AIN14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0AIN15`"]
pub type ADC0AIN15_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN0`"]
pub type ADC1AIN0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN1`"]
pub type ADC1AIN1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN2`"]
pub type ADC1AIN2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN3`"]
pub type ADC1AIN3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN4`"]
pub type ADC1AIN4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN5`"]
pub type ADC1AIN5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN6`"]
pub type ADC1AIN6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN7`"]
pub type ADC1AIN7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN8`"]
pub type ADC1AIN8_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN9`"]
pub type ADC1AIN9_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN10`"]
pub type ADC1AIN10_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN11`"]
pub type ADC1AIN11_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN12`"]
pub type ADC1AIN12_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN13`"]
pub type ADC1AIN13_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN14`"]
pub type ADC1AIN14_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1AIN15`"]
pub type ADC1AIN15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 AIN0 Pin Present"]
    #[inline(always)]
    pub fn adc0ain0(&self) -> ADC0AIN0_R {
        ADC0AIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC Module 0 AIN1 Pin Present"]
    #[inline(always)]
    pub fn adc0ain1(&self) -> ADC0AIN1_R {
        ADC0AIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC Module 0 AIN2 Pin Present"]
    #[inline(always)]
    pub fn adc0ain2(&self) -> ADC0AIN2_R {
        ADC0AIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC Module 0 AIN3 Pin Present"]
    #[inline(always)]
    pub fn adc0ain3(&self) -> ADC0AIN3_R {
        ADC0AIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC Module 0 AIN4 Pin Present"]
    #[inline(always)]
    pub fn adc0ain4(&self) -> ADC0AIN4_R {
        ADC0AIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC Module 0 AIN5 Pin Present"]
    #[inline(always)]
    pub fn adc0ain5(&self) -> ADC0AIN5_R {
        ADC0AIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC Module 0 AIN6 Pin Present"]
    #[inline(always)]
    pub fn adc0ain6(&self) -> ADC0AIN6_R {
        ADC0AIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC Module 0 AIN7 Pin Present"]
    #[inline(always)]
    pub fn adc0ain7(&self) -> ADC0AIN7_R {
        ADC0AIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ADC Module 0 AIN8 Pin Present"]
    #[inline(always)]
    pub fn adc0ain8(&self) -> ADC0AIN8_R {
        ADC0AIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC Module 0 AIN9 Pin Present"]
    #[inline(always)]
    pub fn adc0ain9(&self) -> ADC0AIN9_R {
        ADC0AIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ADC Module 0 AIN10 Pin Present"]
    #[inline(always)]
    pub fn adc0ain10(&self) -> ADC0AIN10_R {
        ADC0AIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ADC Module 0 AIN11 Pin Present"]
    #[inline(always)]
    pub fn adc0ain11(&self) -> ADC0AIN11_R {
        ADC0AIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADC Module 0 AIN12 Pin Present"]
    #[inline(always)]
    pub fn adc0ain12(&self) -> ADC0AIN12_R {
        ADC0AIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADC Module 0 AIN13 Pin Present"]
    #[inline(always)]
    pub fn adc0ain13(&self) -> ADC0AIN13_R {
        ADC0AIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ADC Module 0 AIN14 Pin Present"]
    #[inline(always)]
    pub fn adc0ain14(&self) -> ADC0AIN14_R {
        ADC0AIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC Module 0 AIN15 Pin Present"]
    #[inline(always)]
    pub fn adc0ain15(&self) -> ADC0AIN15_R {
        ADC0AIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC Module 1 AIN0 Pin Present"]
    #[inline(always)]
    pub fn adc1ain0(&self) -> ADC1AIN0_R {
        ADC1AIN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC Module 1 AIN1 Pin Present"]
    #[inline(always)]
    pub fn adc1ain1(&self) -> ADC1AIN1_R {
        ADC1AIN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC Module 1 AIN2 Pin Present"]
    #[inline(always)]
    pub fn adc1ain2(&self) -> ADC1AIN2_R {
        ADC1AIN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC Module 1 AIN3 Pin Present"]
    #[inline(always)]
    pub fn adc1ain3(&self) -> ADC1AIN3_R {
        ADC1AIN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC Module 1 AIN4 Pin Present"]
    #[inline(always)]
    pub fn adc1ain4(&self) -> ADC1AIN4_R {
        ADC1AIN4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC Module 1 AIN5 Pin Present"]
    #[inline(always)]
    pub fn adc1ain5(&self) -> ADC1AIN5_R {
        ADC1AIN5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC Module 1 AIN6 Pin Present"]
    #[inline(always)]
    pub fn adc1ain6(&self) -> ADC1AIN6_R {
        ADC1AIN6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC Module 1 AIN7 Pin Present"]
    #[inline(always)]
    pub fn adc1ain7(&self) -> ADC1AIN7_R {
        ADC1AIN7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC Module 1 AIN8 Pin Present"]
    #[inline(always)]
    pub fn adc1ain8(&self) -> ADC1AIN8_R {
        ADC1AIN8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ADC Module 1 AIN9 Pin Present"]
    #[inline(always)]
    pub fn adc1ain9(&self) -> ADC1AIN9_R {
        ADC1AIN9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ADC Module 1 AIN10 Pin Present"]
    #[inline(always)]
    pub fn adc1ain10(&self) -> ADC1AIN10_R {
        ADC1AIN10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ADC Module 1 AIN11 Pin Present"]
    #[inline(always)]
    pub fn adc1ain11(&self) -> ADC1AIN11_R {
        ADC1AIN11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - ADC Module 1 AIN12 Pin Present"]
    #[inline(always)]
    pub fn adc1ain12(&self) -> ADC1AIN12_R {
        ADC1AIN12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ADC Module 1 AIN13 Pin Present"]
    #[inline(always)]
    pub fn adc1ain13(&self) -> ADC1AIN13_R {
        ADC1AIN13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ADC Module 1 AIN14 Pin Present"]
    #[inline(always)]
    pub fn adc1ain14(&self) -> ADC1AIN14_R {
        ADC1AIN14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - ADC Module 1 AIN15 Pin Present"]
    #[inline(always)]
    pub fn adc1ain15(&self) -> ADC1AIN15_R {
        ADC1AIN15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
