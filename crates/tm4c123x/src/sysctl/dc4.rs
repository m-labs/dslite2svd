#[doc = "Reader of register DC4"]
pub type R = crate::R<u32, super::DC4>;
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
#[doc = "Reader of field `GPIOG`"]
pub type GPIOG_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOH`"]
pub type GPIOH_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPIOJ`"]
pub type GPIOJ_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROM`"]
pub type ROM_R = crate::R<bool, bool>;
#[doc = "Reader of field `UDMA`"]
pub type UDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP6`"]
pub type CCP6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CCP7`"]
pub type CCP7_R = crate::R<bool, bool>;
#[doc = "Reader of field `PICAL`"]
pub type PICAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `E1588`"]
pub type E1588_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMAC0`"]
pub type EMAC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPHY0`"]
pub type EPHY0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - GPIO Port A Present"]
    #[inline(always)]
    pub fn gpioa(&self) -> GPIOA_R {
        GPIOA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPIO Port B Present"]
    #[inline(always)]
    pub fn gpiob(&self) -> GPIOB_R {
        GPIOB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPIO Port C Present"]
    #[inline(always)]
    pub fn gpioc(&self) -> GPIOC_R {
        GPIOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - GPIO Port D Present"]
    #[inline(always)]
    pub fn gpiod(&self) -> GPIOD_R {
        GPIOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO Port E Present"]
    #[inline(always)]
    pub fn gpioe(&self) -> GPIOE_R {
        GPIOE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO Port F Present"]
    #[inline(always)]
    pub fn gpiof(&self) -> GPIOF_R {
        GPIOF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO Port G Present"]
    #[inline(always)]
    pub fn gpiog(&self) -> GPIOG_R {
        GPIOG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO Port H Present"]
    #[inline(always)]
    pub fn gpioh(&self) -> GPIOH_R {
        GPIOH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPIO Port J Present"]
    #[inline(always)]
    pub fn gpioj(&self) -> GPIOJ_R {
        GPIOJ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Internal Code ROM Present"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Micro-DMA Module Present"]
    #[inline(always)]
    pub fn udma(&self) -> UDMA_R {
        UDMA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - T3CCP0 Pin Present"]
    #[inline(always)]
    pub fn ccp6(&self) -> CCP6_R {
        CCP6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - T3CCP1 Pin Present"]
    #[inline(always)]
    pub fn ccp7(&self) -> CCP7_R {
        CCP7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PIOSC Calibrate"]
    #[inline(always)]
    pub fn pical(&self) -> PICAL_R {
        PICAL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 1588 Capable"]
    #[inline(always)]
    pub fn e1588(&self) -> E1588_R {
        E1588_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Ethernet MAC Layer 0 Present"]
    #[inline(always)]
    pub fn emac0(&self) -> EMAC0_R {
        EMAC0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ethernet PHY Layer 0 Present"]
    #[inline(always)]
    pub fn ephy0(&self) -> EPHY0_R {
        EPHY0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
