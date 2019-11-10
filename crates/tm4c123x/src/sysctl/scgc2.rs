#[doc = "Reader of register SCGC2"]
pub type R = crate::R<u32, super::SCGC2>;
#[doc = "Reader of field `GPIOA`"]
pub type GPIOA_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOB`"]
pub type GPIOB_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOC`"]
pub type GPIOC_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOD`"]
pub type GPIOD_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOE`"]
pub type GPIOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOF`"]
pub type GPIOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `UDMA`"]
pub type UDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `USB0`"]
pub type USB0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Port A Clock Gating Control"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port B Clock Gating Control"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port C Clock Gating Control"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port D Clock Gating Control"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port E Clock Gating Control"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port F Clock Gating Control"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Micro-DMA Clock Gating Control"]
    #[inline(always)]
    pub fn udma(&self) -> UDMA_R {
        UDMA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USB0 Clock Gating Control"]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
