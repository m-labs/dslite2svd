#[doc = "Reader of register DC9"]
pub type R = crate::R<u32, super::DC9>;
#[doc = "Reader of field `ADC0DC0`"]
pub type ADC0DC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC1`"]
pub type ADC0DC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC2`"]
pub type ADC0DC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC3`"]
pub type ADC0DC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC4`"]
pub type ADC0DC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC5`"]
pub type ADC0DC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC6`"]
pub type ADC0DC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC0DC7`"]
pub type ADC0DC7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC0`"]
pub type ADC1DC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC1`"]
pub type ADC1DC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC2`"]
pub type ADC1DC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC3`"]
pub type ADC1DC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC4`"]
pub type ADC1DC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC5`"]
pub type ADC1DC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC6`"]
pub type ADC1DC6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADC1DC7`"]
pub type ADC1DC7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ADC0 DC0 Present"]
    #[inline(always)]
    pub fn adc0dc0(&self) -> ADC0DC0_R {
        ADC0DC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC0 DC1 Present"]
    #[inline(always)]
    pub fn adc0dc1(&self) -> ADC0DC1_R {
        ADC0DC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ADC0 DC2 Present"]
    #[inline(always)]
    pub fn adc0dc2(&self) -> ADC0DC2_R {
        ADC0DC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC0 DC3 Present"]
    #[inline(always)]
    pub fn adc0dc3(&self) -> ADC0DC3_R {
        ADC0DC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC0 DC4 Present"]
    #[inline(always)]
    pub fn adc0dc4(&self) -> ADC0DC4_R {
        ADC0DC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC0 DC5 Present"]
    #[inline(always)]
    pub fn adc0dc5(&self) -> ADC0DC5_R {
        ADC0DC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC0 DC6 Present"]
    #[inline(always)]
    pub fn adc0dc6(&self) -> ADC0DC6_R {
        ADC0DC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 DC7 Present"]
    #[inline(always)]
    pub fn adc0dc7(&self) -> ADC0DC7_R {
        ADC0DC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ADC1 DC0 Present"]
    #[inline(always)]
    pub fn adc1dc0(&self) -> ADC1DC0_R {
        ADC1DC0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC1 DC1 Present"]
    #[inline(always)]
    pub fn adc1dc1(&self) -> ADC1DC1_R {
        ADC1DC1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC1 DC2 Present"]
    #[inline(always)]
    pub fn adc1dc2(&self) -> ADC1DC2_R {
        ADC1DC2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC1 DC3 Present"]
    #[inline(always)]
    pub fn adc1dc3(&self) -> ADC1DC3_R {
        ADC1DC3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ADC1 DC4 Present"]
    #[inline(always)]
    pub fn adc1dc4(&self) -> ADC1DC4_R {
        ADC1DC4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ADC1 DC5 Present"]
    #[inline(always)]
    pub fn adc1dc5(&self) -> ADC1DC5_R {
        ADC1DC5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC1 DC6 Present"]
    #[inline(always)]
    pub fn adc1dc6(&self) -> ADC1DC6_R {
        ADC1DC6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC1 DC7 Present"]
    #[inline(always)]
    pub fn adc1dc7(&self) -> ADC1DC7_R {
        ADC1DC7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
