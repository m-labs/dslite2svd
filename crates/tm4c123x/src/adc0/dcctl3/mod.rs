#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCCTL3 {
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
#[doc = "Possible values of the field `CIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIMR {
    #[doc = "Always"]
    ALWAYS,
    #[doc = "Once"]
    ONCE,
    #[doc = "Hysteresis Always"]
    HALWAYS,
    #[doc = "Hysteresis Once"]
    HONCE,
}
impl CIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CIMR::ALWAYS => 0,
            CIMR::ONCE => 1,
            CIMR::HALWAYS => 2,
            CIMR::HONCE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CIMR {
        match value {
            0 => CIMR::ALWAYS,
            1 => CIMR::ONCE,
            2 => CIMR::HALWAYS,
            3 => CIMR::HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == CIMR::ALWAYS
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline]
    pub fn is_once(&self) -> bool {
        *self == CIMR::ONCE
    }
    #[doc = "Checks if the value of the field is `HALWAYS`"]
    #[inline]
    pub fn is_halways(&self) -> bool {
        *self == CIMR::HALWAYS
    }
    #[doc = "Checks if the value of the field is `HONCE`"]
    #[inline]
    pub fn is_honce(&self) -> bool {
        *self == CIMR::HONCE
    }
}
#[doc = "Possible values of the field `CIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CICR {
    #[doc = "Low Band"]
    LOW,
    #[doc = "Mid Band"]
    MID,
    #[doc = "High Band"]
    HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CICR::LOW => 0,
            CICR::MID => 1,
            CICR::HIGH => 3,
            CICR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CICR {
        match value {
            0 => CICR::LOW,
            1 => CICR::MID,
            3 => CICR::HIGH,
            i => CICR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CICR::LOW
    }
    #[doc = "Checks if the value of the field is `MID`"]
    #[inline]
    pub fn is_mid(&self) -> bool {
        *self == CICR::MID
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CICR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct CIER {
    bits: bool,
}
impl CIER {
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
#[doc = "Possible values of the field `CTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMR {
    #[doc = "Always"]
    ALWAYS,
    #[doc = "Once"]
    ONCE,
    #[doc = "Hysteresis Always"]
    HALWAYS,
    #[doc = "Hysteresis Once"]
    HONCE,
}
impl CTMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTMR::ALWAYS => 0,
            CTMR::ONCE => 1,
            CTMR::HALWAYS => 2,
            CTMR::HONCE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTMR {
        match value {
            0 => CTMR::ALWAYS,
            1 => CTMR::ONCE,
            2 => CTMR::HALWAYS,
            3 => CTMR::HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS`"]
    #[inline]
    pub fn is_always(&self) -> bool {
        *self == CTMR::ALWAYS
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline]
    pub fn is_once(&self) -> bool {
        *self == CTMR::ONCE
    }
    #[doc = "Checks if the value of the field is `HALWAYS`"]
    #[inline]
    pub fn is_halways(&self) -> bool {
        *self == CTMR::HALWAYS
    }
    #[doc = "Checks if the value of the field is `HONCE`"]
    #[inline]
    pub fn is_honce(&self) -> bool {
        *self == CTMR::HONCE
    }
}
#[doc = "Possible values of the field `CTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTCR {
    #[doc = "Low Band"]
    LOW,
    #[doc = "Mid Band"]
    MID,
    #[doc = "High Band"]
    HIGH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CTCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CTCR::LOW => 0,
            CTCR::MID => 1,
            CTCR::HIGH => 3,
            CTCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CTCR {
        match value {
            0 => CTCR::LOW,
            1 => CTCR::MID,
            3 => CTCR::HIGH,
            i => CTCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CTCR::LOW
    }
    #[doc = "Checks if the value of the field is `MID`"]
    #[inline]
    pub fn is_mid(&self) -> bool {
        *self == CTCR::MID
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CTCR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct CTER {
    bits: bool,
}
impl CTER {
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
#[doc = "Values that can be written to the field `CIM`"]
pub enum CIMW {
    #[doc = "Always"]
    ALWAYS,
    #[doc = "Once"]
    ONCE,
    #[doc = "Hysteresis Always"]
    HALWAYS,
    #[doc = "Hysteresis Once"]
    HONCE,
}
impl CIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CIMW::ALWAYS => 0,
            CIMW::ONCE => 1,
            CIMW::HALWAYS => 2,
            CIMW::HONCE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Always"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(CIMW::ALWAYS)
    }
    #[doc = "Once"]
    #[inline]
    pub fn once(self) -> &'a mut W {
        self.variant(CIMW::ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline]
    pub fn halways(self) -> &'a mut W {
        self.variant(CIMW::HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline]
    pub fn honce(self) -> &'a mut W {
        self.variant(CIMW::HONCE)
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
#[doc = "Values that can be written to the field `CIC`"]
pub enum CICW {
    #[doc = "Low Band"]
    LOW,
    #[doc = "Mid Band"]
    MID,
    #[doc = "High Band"]
    HIGH,
}
impl CICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CICW::LOW => 0,
            CICW::MID => 1,
            CICW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CICW<'a> {
    w: &'a mut W,
}
impl<'a> _CICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CICW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Band"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CICW::LOW)
    }
    #[doc = "Mid Band"]
    #[inline]
    pub fn mid(self) -> &'a mut W {
        self.variant(CICW::MID)
    }
    #[doc = "High Band"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CICW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CIEW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTM`"]
pub enum CTMW {
    #[doc = "Always"]
    ALWAYS,
    #[doc = "Once"]
    ONCE,
    #[doc = "Hysteresis Always"]
    HALWAYS,
    #[doc = "Hysteresis Once"]
    HONCE,
}
impl CTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTMW::ALWAYS => 0,
            CTMW::ONCE => 1,
            CTMW::HALWAYS => 2,
            CTMW::HONCE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTMW<'a> {
    w: &'a mut W,
}
impl<'a> _CTMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Always"]
    #[inline]
    pub fn always(self) -> &'a mut W {
        self.variant(CTMW::ALWAYS)
    }
    #[doc = "Once"]
    #[inline]
    pub fn once(self) -> &'a mut W {
        self.variant(CTMW::ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline]
    pub fn halways(self) -> &'a mut W {
        self.variant(CTMW::HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline]
    pub fn honce(self) -> &'a mut W {
        self.variant(CTMW::HONCE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CTC`"]
pub enum CTCW {
    #[doc = "Low Band"]
    LOW,
    #[doc = "Mid Band"]
    MID,
    #[doc = "High Band"]
    HIGH,
}
impl CTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CTCW::LOW => 0,
            CTCW::MID => 1,
            CTCW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Band"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CTCW::LOW)
    }
    #[doc = "Mid Band"]
    #[inline]
    pub fn mid(self) -> &'a mut W {
        self.variant(CTCW::MID)
    }
    #[doc = "High Band"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CTCW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTEW<'a> {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline]
    pub fn cim(&self) -> CIMR {
        CIMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline]
    pub fn cic(&self) -> CICR {
        CICR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline]
    pub fn cie(&self) -> CIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CIER { bits }
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline]
    pub fn ctm(&self) -> CTMR {
        CTMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline]
    pub fn ctc(&self) -> CTCR {
        CTCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline]
    pub fn cte(&self) -> CTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTER { bits }
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
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline]
    pub fn cim(&mut self) -> _CIMW {
        _CIMW { w: self }
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline]
    pub fn cic(&mut self) -> _CICW {
        _CICW { w: self }
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline]
    pub fn cie(&mut self) -> _CIEW {
        _CIEW { w: self }
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline]
    pub fn ctm(&mut self) -> _CTMW {
        _CTMW { w: self }
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline]
    pub fn ctc(&mut self) -> _CTCW {
        _CTCW { w: self }
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline]
    pub fn cte(&mut self) -> _CTEW {
        _CTEW { w: self }
    }
}
