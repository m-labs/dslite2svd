#[doc = "Reader of register IFLS"]
pub type R = crate::R<u32, super::IFLS>;
#[doc = "Writer for register IFLS"]
pub type W = crate::W<u32, super::IFLS>;
#[doc = "Register IFLS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART Transmit Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_A {
    #[doc = "0: TX FIFO &lt;= 1/8 full"]
    TX1_8,
    #[doc = "1: TX FIFO &lt;= 1/4 full"]
    TX2_8,
    #[doc = "2: TX FIFO &lt;= 1/2 full (default)"]
    TX4_8,
    #[doc = "3: TX FIFO &lt;= 3/4 full"]
    TX6_8,
    #[doc = "4: TX FIFO &lt;= 7/8 full"]
    TX7_8,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        match variant {
            TX_A::TX1_8 => 0,
            TX_A::TX2_8 => 1,
            TX_A::TX4_8 => 2,
            TX_A::TX6_8 => 3,
            TX_A::TX7_8 => 4,
        }
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TX_A::TX1_8),
            1 => Val(TX_A::TX2_8),
            2 => Val(TX_A::TX4_8),
            3 => Val(TX_A::TX6_8),
            4 => Val(TX_A::TX7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX1_8`"]
    #[inline(always)]
    pub fn is_tx1_8(&self) -> bool {
        *self == TX_A::TX1_8
    }
    #[doc = "Checks if the value of the field is `TX2_8`"]
    #[inline(always)]
    pub fn is_tx2_8(&self) -> bool {
        *self == TX_A::TX2_8
    }
    #[doc = "Checks if the value of the field is `TX4_8`"]
    #[inline(always)]
    pub fn is_tx4_8(&self) -> bool {
        *self == TX_A::TX4_8
    }
    #[doc = "Checks if the value of the field is `TX6_8`"]
    #[inline(always)]
    pub fn is_tx6_8(&self) -> bool {
        *self == TX_A::TX6_8
    }
    #[doc = "Checks if the value of the field is `TX7_8`"]
    #[inline(always)]
    pub fn is_tx7_8(&self) -> bool {
        *self == TX_A::TX7_8
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TX FIFO &lt;= 1/8 full"]
    #[inline(always)]
    pub fn tx1_8(self) -> &'a mut W {
        self.variant(TX_A::TX1_8)
    }
    #[doc = "TX FIFO &lt;= 1/4 full"]
    #[inline(always)]
    pub fn tx2_8(self) -> &'a mut W {
        self.variant(TX_A::TX2_8)
    }
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    #[inline(always)]
    pub fn tx4_8(self) -> &'a mut W {
        self.variant(TX_A::TX4_8)
    }
    #[doc = "TX FIFO &lt;= 3/4 full"]
    #[inline(always)]
    pub fn tx6_8(self) -> &'a mut W {
        self.variant(TX_A::TX6_8)
    }
    #[doc = "TX FIFO &lt;= 7/8 full"]
    #[inline(always)]
    pub fn tx7_8(self) -> &'a mut W {
        self.variant(TX_A::TX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "UART Receive Interrupt FIFO Level Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_A {
    #[doc = "0: RX FIFO >= 1/8 full"]
    RX1_8,
    #[doc = "1: RX FIFO >= 1/4 full"]
    RX2_8,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    RX4_8,
    #[doc = "3: RX FIFO >= 3/4 full"]
    RX6_8,
    #[doc = "4: RX FIFO >= 7/8 full"]
    RX7_8,
}
impl From<RX_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        match variant {
            RX_A::RX1_8 => 0,
            RX_A::RX2_8 => 1,
            RX_A::RX4_8 => 2,
            RX_A::RX6_8 => 3,
            RX_A::RX7_8 => 4,
        }
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<u8, RX_A>;
impl RX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RX_A::RX1_8),
            1 => Val(RX_A::RX2_8),
            2 => Val(RX_A::RX4_8),
            3 => Val(RX_A::RX6_8),
            4 => Val(RX_A::RX7_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX1_8`"]
    #[inline(always)]
    pub fn is_rx1_8(&self) -> bool {
        *self == RX_A::RX1_8
    }
    #[doc = "Checks if the value of the field is `RX2_8`"]
    #[inline(always)]
    pub fn is_rx2_8(&self) -> bool {
        *self == RX_A::RX2_8
    }
    #[doc = "Checks if the value of the field is `RX4_8`"]
    #[inline(always)]
    pub fn is_rx4_8(&self) -> bool {
        *self == RX_A::RX4_8
    }
    #[doc = "Checks if the value of the field is `RX6_8`"]
    #[inline(always)]
    pub fn is_rx6_8(&self) -> bool {
        *self == RX_A::RX6_8
    }
    #[doc = "Checks if the value of the field is `RX7_8`"]
    #[inline(always)]
    pub fn is_rx7_8(&self) -> bool {
        *self == RX_A::RX7_8
    }
}
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline(always)]
    pub fn rx1_8(self) -> &'a mut W {
        self.variant(RX_A::RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn rx2_8(self) -> &'a mut W {
        self.variant(RX_A::RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn rx4_8(self) -> &'a mut W {
        self.variant(RX_A::RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn rx6_8(self) -> &'a mut W {
        self.variant(RX_A::RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline(always)]
    pub fn rx7_8(self) -> &'a mut W {
        self.variant(RX_A::RX7_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
