#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFLS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "TX FIFO &lt;= 1/8 full"]
    TX1_8,
    #[doc = "TX FIFO &lt;= 1/4 full"]
    TX2_8,
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    TX4_8,
    #[doc = "TX FIFO &lt;= 3/4 full"]
    TX6_8,
    #[doc = "TX FIFO &lt;= 7/8 full"]
    TX7_8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXR::TX1_8 => 0,
            TXR::TX2_8 => 1,
            TXR::TX4_8 => 2,
            TXR::TX6_8 => 3,
            TXR::TX7_8 => 4,
            TXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXR {
        match value {
            0 => TXR::TX1_8,
            1 => TXR::TX2_8,
            2 => TXR::TX4_8,
            3 => TXR::TX6_8,
            4 => TXR::TX7_8,
            i => TXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TX1_8`"]
    #[inline]
    pub fn is_tx1_8(&self) -> bool {
        *self == TXR::TX1_8
    }
    #[doc = "Checks if the value of the field is `TX2_8`"]
    #[inline]
    pub fn is_tx2_8(&self) -> bool {
        *self == TXR::TX2_8
    }
    #[doc = "Checks if the value of the field is `TX4_8`"]
    #[inline]
    pub fn is_tx4_8(&self) -> bool {
        *self == TXR::TX4_8
    }
    #[doc = "Checks if the value of the field is `TX6_8`"]
    #[inline]
    pub fn is_tx6_8(&self) -> bool {
        *self == TXR::TX6_8
    }
    #[doc = "Checks if the value of the field is `TX7_8`"]
    #[inline]
    pub fn is_tx7_8(&self) -> bool {
        *self == TXR::TX7_8
    }
}
#[doc = "Possible values of the field `RX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXR {
    #[doc = "RX FIFO >= 1/8 full"]
    RX1_8,
    #[doc = "RX FIFO >= 1/4 full"]
    RX2_8,
    #[doc = "RX FIFO >= 1/2 full (default)"]
    RX4_8,
    #[doc = "RX FIFO >= 3/4 full"]
    RX6_8,
    #[doc = "RX FIFO >= 7/8 full"]
    RX7_8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXR::RX1_8 => 0,
            RXR::RX2_8 => 1,
            RXR::RX4_8 => 2,
            RXR::RX6_8 => 3,
            RXR::RX7_8 => 4,
            RXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXR {
        match value {
            0 => RXR::RX1_8,
            1 => RXR::RX2_8,
            2 => RXR::RX4_8,
            3 => RXR::RX6_8,
            4 => RXR::RX7_8,
            i => RXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RX1_8`"]
    #[inline]
    pub fn is_rx1_8(&self) -> bool {
        *self == RXR::RX1_8
    }
    #[doc = "Checks if the value of the field is `RX2_8`"]
    #[inline]
    pub fn is_rx2_8(&self) -> bool {
        *self == RXR::RX2_8
    }
    #[doc = "Checks if the value of the field is `RX4_8`"]
    #[inline]
    pub fn is_rx4_8(&self) -> bool {
        *self == RXR::RX4_8
    }
    #[doc = "Checks if the value of the field is `RX6_8`"]
    #[inline]
    pub fn is_rx6_8(&self) -> bool {
        *self == RXR::RX6_8
    }
    #[doc = "Checks if the value of the field is `RX7_8`"]
    #[inline]
    pub fn is_rx7_8(&self) -> bool {
        *self == RXR::RX7_8
    }
}
#[doc = "Values that can be written to the field `TX`"]
pub enum TXW {
    #[doc = "TX FIFO &lt;= 1/8 full"]
    TX1_8,
    #[doc = "TX FIFO &lt;= 1/4 full"]
    TX2_8,
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    TX4_8,
    #[doc = "TX FIFO &lt;= 3/4 full"]
    TX6_8,
    #[doc = "TX FIFO &lt;= 7/8 full"]
    TX7_8,
}
impl TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXW::TX1_8 => 0,
            TXW::TX2_8 => 1,
            TXW::TX4_8 => 2,
            TXW::TX6_8 => 3,
            TXW::TX7_8 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXW<'a> {
    w: &'a mut W,
}
impl<'a> _TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TX FIFO &lt;= 1/8 full"]
    #[inline]
    pub fn tx1_8(self) -> &'a mut W {
        self.variant(TXW::TX1_8)
    }
    #[doc = "TX FIFO &lt;= 1/4 full"]
    #[inline]
    pub fn tx2_8(self) -> &'a mut W {
        self.variant(TXW::TX2_8)
    }
    #[doc = "TX FIFO &lt;= 1/2 full (default)"]
    #[inline]
    pub fn tx4_8(self) -> &'a mut W {
        self.variant(TXW::TX4_8)
    }
    #[doc = "TX FIFO &lt;= 3/4 full"]
    #[inline]
    pub fn tx6_8(self) -> &'a mut W {
        self.variant(TXW::TX6_8)
    }
    #[doc = "TX FIFO &lt;= 7/8 full"]
    #[inline]
    pub fn tx7_8(self) -> &'a mut W {
        self.variant(TXW::TX7_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RX`"]
pub enum RXW {
    #[doc = "RX FIFO >= 1/8 full"]
    RX1_8,
    #[doc = "RX FIFO >= 1/4 full"]
    RX2_8,
    #[doc = "RX FIFO >= 1/2 full (default)"]
    RX4_8,
    #[doc = "RX FIFO >= 3/4 full"]
    RX6_8,
    #[doc = "RX FIFO >= 7/8 full"]
    RX7_8,
}
impl RXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXW::RX1_8 => 0,
            RXW::RX2_8 => 1,
            RXW::RX4_8 => 2,
            RXW::RX6_8 => 3,
            RXW::RX7_8 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXW<'a> {
    w: &'a mut W,
}
impl<'a> _RXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "RX FIFO >= 1/8 full"]
    #[inline]
    pub fn rx1_8(self) -> &'a mut W {
        self.variant(RXW::RX1_8)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline]
    pub fn rx2_8(self) -> &'a mut W {
        self.variant(RXW::RX2_8)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline]
    pub fn rx4_8(self) -> &'a mut W {
        self.variant(RXW::RX4_8)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline]
    pub fn rx6_8(self) -> &'a mut W {
        self.variant(RXW::RX6_8)
    }
    #[doc = "RX FIFO >= 7/8 full"]
    #[inline]
    pub fn rx7_8(self) -> &'a mut W {
        self.variant(RXW::RX7_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline]
    pub fn rx(&self) -> RXR {
        RXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select"]
    #[inline]
    pub fn tx(&mut self) -> _TXW {
        _TXW { w: self }
    }
    #[doc = "Bits 3:5 - UART Receive Interrupt FIFO Level Select"]
    #[inline]
    pub fn rx(&mut self) -> _RXW {
        _RXW { w: self }
    }
}
