#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCFG {
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
#[doc = "Possible values of the field `DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIZER {
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    _4BIT,
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    _16BIT,
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    _24BIT,
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    _32BIT,
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSIZER::_4BIT => 0,
            DSIZER::_16BIT => 1,
            DSIZER::_24BIT => 2,
            DSIZER::_32BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSIZER {
        match value {
            0 => DSIZER::_4BIT,
            1 => DSIZER::_16BIT,
            2 => DSIZER::_24BIT,
            3 => DSIZER::_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline]
    pub fn is_4bit(&self) -> bool {
        *self == DSIZER::_4BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == DSIZER::_16BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline]
    pub fn is_24bit(&self) -> bool {
        *self == DSIZER::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline]
    pub fn is_32bit(&self) -> bool {
        *self == DSIZER::_32BIT
    }
}
#[doc = "Possible values of the field `ASIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASIZER {
    #[doc = "No address"]
    NONE,
    #[doc = "Up to 4 bits wide"]
    _4BIT,
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    _12BIT,
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    _20BIT,
}
impl ASIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASIZER::NONE => 0,
            ASIZER::_4BIT => 1,
            ASIZER::_12BIT => 2,
            ASIZER::_20BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASIZER {
        match value {
            0 => ASIZER::NONE,
            1 => ASIZER::_4BIT,
            2 => ASIZER::_12BIT,
            3 => ASIZER::_20BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ASIZER::NONE
    }
    #[doc = "Checks if the value of the field is `_4BIT`"]
    #[inline]
    pub fn is_4bit(&self) -> bool {
        *self == ASIZER::_4BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline]
    pub fn is_12bit(&self) -> bool {
        *self == ASIZER::_12BIT
    }
    #[doc = "Checks if the value of the field is `_20BIT`"]
    #[inline]
    pub fn is_20bit(&self) -> bool {
        *self == ASIZER::_20BIT
    }
}
#[doc = r" Value of the field"]
pub struct WR2CYCR {
    bits: bool,
}
impl WR2CYCR {
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
pub struct FRMCNTR {
    bits: u8,
}
impl FRMCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRM50R {
    bits: bool,
}
impl FRM50R {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
pub struct CLKPINR {
    bits: bool,
}
impl CLKPINR {
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
#[doc = "Values that can be written to the field `DSIZE`"]
pub enum DSIZEW {
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    _4BIT,
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    _16BIT,
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    _24BIT,
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    _32BIT,
}
impl DSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSIZEW::_4BIT => 0,
            DSIZEW::_16BIT => 1,
            DSIZEW::_24BIT => 2,
            DSIZEW::_32BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 Bits Wide (EPI0S0 to EPI0S7)"]
    #[inline]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(DSIZEW::_4BIT)
    }
    #[doc = "16 Bits Wide (EPI0S0 to EPI0S15)"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(DSIZEW::_16BIT)
    }
    #[doc = "24 Bits Wide (EPI0S0 to EPI0S23)"]
    #[inline]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(DSIZEW::_24BIT)
    }
    #[doc = "32 Bits Wide (EPI0S0 to EPI0S31)"]
    #[inline]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(DSIZEW::_32BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASIZE`"]
pub enum ASIZEW {
    #[doc = "No address"]
    NONE,
    #[doc = "Up to 4 bits wide"]
    _4BIT,
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    _12BIT,
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    _20BIT,
}
impl ASIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASIZEW::NONE => 0,
            ASIZEW::_4BIT => 1,
            ASIZEW::_12BIT => 2,
            ASIZEW::_20BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No address"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ASIZEW::NONE)
    }
    #[doc = "Up to 4 bits wide"]
    #[inline]
    pub fn _4bit(self) -> &'a mut W {
        self.variant(ASIZEW::_4BIT)
    }
    #[doc = "Up to 12 bits wide. This size cannot be used with 24-bit data"]
    #[inline]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(ASIZEW::_12BIT)
    }
    #[doc = "Up to 20 bits wide. This size cannot be used with data sizes other than 8"]
    #[inline]
    pub fn _20bit(self) -> &'a mut W {
        self.variant(ASIZEW::_20BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WR2CYCW<'a> {
    w: &'a mut W,
}
impl<'a> _WR2CYCW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRMCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FRMCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRM50W<'a> {
    w: &'a mut W,
}
impl<'a> _FRM50W<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKPINW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKPINW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        DSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline]
    pub fn asize(&self) -> ASIZER {
        ASIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline]
    pub fn wr2cyc(&self) -> WR2CYCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WR2CYCR { bits }
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline]
    pub fn frmcnt(&self) -> FRMCNTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FRMCNTR { bits }
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline]
    pub fn frm50(&self) -> FRM50R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRM50R { bits }
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline]
    pub fn clkpin(&self) -> CLKPINR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKPINR { bits }
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
    #[doc = "Bits 0:1 - Size of Data Bus"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bits 4:5 - Address Bus Size"]
    #[inline]
    pub fn asize(&mut self) -> _ASIZEW {
        _ASIZEW { w: self }
    }
    #[doc = "Bit 19 - 2-Cycle Writes"]
    #[inline]
    pub fn wr2cyc(&mut self) -> _WR2CYCW {
        _WR2CYCW { w: self }
    }
    #[doc = "Bits 22:25 - Frame Count"]
    #[inline]
    pub fn frmcnt(&mut self) -> _FRMCNTW {
        _FRMCNTW { w: self }
    }
    #[doc = "Bit 26 - 50/50 Frame"]
    #[inline]
    pub fn frm50(&mut self) -> _FRM50W {
        _FRM50W { w: self }
    }
    #[doc = "Bit 30 - Clock Gated"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
    #[doc = "Bit 31 - Clock Pin"]
    #[inline]
    pub fn clkpin(&mut self) -> _CLKPINW {
        _CLKPINW { w: self }
    }
}
