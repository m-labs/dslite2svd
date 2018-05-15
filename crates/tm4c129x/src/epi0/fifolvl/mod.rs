#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOLVL {
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
#[doc = "Possible values of the field `RDFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFIFOR {
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    _1,
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    _2,
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    _4,
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    _6,
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    _7,
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    _8,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RDFIFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDFIFOR::_1 => 1,
            RDFIFOR::_2 => 2,
            RDFIFOR::_4 => 3,
            RDFIFOR::_6 => 4,
            RDFIFOR::_7 => 5,
            RDFIFOR::_8 => 6,
            RDFIFOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDFIFOR {
        match value {
            1 => RDFIFOR::_1,
            2 => RDFIFOR::_2,
            3 => RDFIFOR::_4,
            4 => RDFIFOR::_6,
            5 => RDFIFOR::_7,
            6 => RDFIFOR::_8,
            i => RDFIFOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDFIFOR::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RDFIFOR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RDFIFOR::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == RDFIFOR::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline]
    pub fn is_7(&self) -> bool {
        *self == RDFIFOR::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == RDFIFOR::_8
    }
}
#[doc = "Possible values of the field `WRFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRFIFOR {
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    EMPT,
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    _2,
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    _1,
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    NFULL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRFIFOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRFIFOR::EMPT => 0,
            WRFIFOR::_2 => 2,
            WRFIFOR::_1 => 3,
            WRFIFOR::NFULL => 4,
            WRFIFOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRFIFOR {
        match value {
            0 => WRFIFOR::EMPT,
            2 => WRFIFOR::_2,
            3 => WRFIFOR::_1,
            4 => WRFIFOR::NFULL,
            i => WRFIFOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMPT`"]
    #[inline]
    pub fn is_empt(&self) -> bool {
        *self == WRFIFOR::EMPT
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == WRFIFOR::_2
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WRFIFOR::_1
    }
    #[doc = "Checks if the value of the field is `NFULL`"]
    #[inline]
    pub fn is_nfull(&self) -> bool {
        *self == WRFIFOR::NFULL
    }
}
#[doc = r" Value of the field"]
pub struct RSERRR {
    bits: bool,
}
impl RSERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WFERRR {
    bits: bool,
}
impl WFERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `RDFIFO`"]
pub enum RDFIFOW {
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    _1,
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    _2,
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    _4,
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    _6,
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    _7,
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    _8,
}
impl RDFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDFIFOW::_1 => 1,
            RDFIFOW::_2 => 2,
            RDFIFOW::_4 => 3,
            RDFIFOW::_6 => 4,
            RDFIFOW::_7 => 5,
            RDFIFOW::_8 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _RDFIFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDFIFOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDFIFOW::_1)
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RDFIFOW::_2)
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RDFIFOW::_4)
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(RDFIFOW::_6)
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    #[inline]
    pub fn _7(self) -> &'a mut W {
        self.variant(RDFIFOW::_7)
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(RDFIFOW::_8)
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
#[doc = "Values that can be written to the field `WRFIFO`"]
pub enum WRFIFOW {
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    EMPT,
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    _2,
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    _1,
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    NFULL,
}
impl WRFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRFIFOW::EMPT => 0,
            WRFIFOW::_2 => 2,
            WRFIFOW::_1 => 3,
            WRFIFOW::NFULL => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _WRFIFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRFIFOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    #[inline]
    pub fn empt(self) -> &'a mut W {
        self.variant(WRFIFOW::EMPT)
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(WRFIFOW::_2)
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRFIFOW::_1)
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    #[inline]
    pub fn nfull(self) -> &'a mut W {
        self.variant(WRFIFOW::NFULL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RSERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WFERRW<'a> {
    w: &'a mut W,
}
impl<'a> _WFERRW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline]
    pub fn rdfifo(&self) -> RDFIFOR {
        RDFIFOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline]
    pub fn wrfifo(&self) -> WRFIFOR {
        WRFIFOR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline]
    pub fn rserr(&self) -> RSERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSERRR { bits }
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline]
    pub fn wferr(&self) -> WFERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WFERRR { bits }
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
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline]
    pub fn rdfifo(&mut self) -> _RDFIFOW {
        _RDFIFOW { w: self }
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline]
    pub fn wrfifo(&mut self) -> _WRFIFOW {
        _WRFIFOW { w: self }
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline]
    pub fn rserr(&mut self) -> _RSERRW {
        _RSERRW { w: self }
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline]
    pub fn wferr(&mut self) -> _WFERRW {
        _WFERRW { w: self }
    }
}
