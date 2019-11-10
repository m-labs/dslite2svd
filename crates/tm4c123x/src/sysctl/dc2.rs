#[doc = "Reader of register DC2"]
pub type R = crate::R<u32, super::DC2>;
#[doc = "Reader of field `UART0`"]
pub type UART0_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART1`"]
pub type UART1_R = crate::R<bool, bool>;
#[doc = "Reader of field `UART2`"]
pub type UART2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSI0`"]
pub type SSI0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSI1`"]
pub type SSI1_R = crate::R<bool, bool>;
#[doc = "Reader of field `QEI0`"]
pub type QEI0_R = crate::R<bool, bool>;
#[doc = "Reader of field `QEI1`"]
pub type QEI1_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C0`"]
pub type I2C0_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C0HS`"]
pub type I2C0HS_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C1`"]
pub type I2C1_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2C1HS`"]
pub type I2C1HS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER0`"]
pub type TIMER0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER1`"]
pub type TIMER1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER2`"]
pub type TIMER2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMER3`"]
pub type TIMER3_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP0`"]
pub type COMP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP1`"]
pub type COMP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP2`"]
pub type COMP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `I2S0`"]
pub type I2S0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPI0`"]
pub type EPI0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - UART Module 0 Present"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART Module 1 Present"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART Module 2 Present"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SSI Module 0 Present"]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SSI Module 1 Present"]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - QEI Module 0 Present"]
    #[inline(always)]
    pub fn qei0(&self) -> QEI0_R {
        QEI0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - QEI Module 1 Present"]
    #[inline(always)]
    pub fn qei1(&self) -> QEI1_R {
        QEI1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - I2C Module 0 Speed"]
    #[inline(always)]
    pub fn i2c0hs(&self) -> I2C0HS_R {
        I2C0HS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - I2C Module 1 Speed"]
    #[inline(always)]
    pub fn i2c1hs(&self) -> I2C1HS_R {
        I2C1HS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timer Module 0 Present"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Timer Module 1 Present"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timer Module 2 Present"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Timer Module 3 Present"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Analog Comparator 0 Present"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Analog Comparator 1 Present"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Analog Comparator 2 Present"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - I2S Module 0 Present"]
    #[inline(always)]
    pub fn i2s0(&self) -> I2S0_R {
        I2S0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 30 - EPI Module 0 Present"]
    #[inline(always)]
    pub fn epi0(&self) -> EPI0_R {
        EPI0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
