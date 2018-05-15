#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACCTL0 {
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
#[doc = r" Value of the field"]
pub struct CINVR {
    bits: bool,
}
impl CINVR {
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
#[doc = "Possible values of the field `ISEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISENR {
    #[doc = "Level sense, see ISLVAL"]
    LEVEL,
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
    #[doc = "Either edge"]
    BOTH,
}
impl ISENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ISENR::LEVEL => 0,
            ISENR::FALL => 1,
            ISENR::RISE => 2,
            ISENR::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ISENR {
        match value {
            0 => ISENR::LEVEL,
            1 => ISENR::FALL,
            2 => ISENR::RISE,
            3 => ISENR::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == ISENR::LEVEL
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == ISENR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == ISENR::RISE
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == ISENR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct ISLVALR {
    bits: bool,
}
impl ISLVALR {
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
#[doc = "Possible values of the field `TSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSENR {
    #[doc = "Level sense, see TSLVAL"]
    LEVEL,
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
    #[doc = "Either edge"]
    BOTH,
}
impl TSENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSENR::LEVEL => 0,
            TSENR::FALL => 1,
            TSENR::RISE => 2,
            TSENR::BOTH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSENR {
        match value {
            0 => TSENR::LEVEL,
            1 => TSENR::FALL,
            2 => TSENR::RISE,
            3 => TSENR::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline]
    pub fn is_level(&self) -> bool {
        *self == TSENR::LEVEL
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline]
    pub fn is_fall(&self) -> bool {
        *self == TSENR::FALL
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline]
    pub fn is_rise(&self) -> bool {
        *self == TSENR::RISE
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == TSENR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct TSLVALR {
    bits: bool,
}
impl TSLVALR {
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
#[doc = "Possible values of the field `ASRCP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASRCPR {
    #[doc = "Pin value of Cn+"]
    PIN,
    #[doc = "Pin value of C0+"]
    PIN0,
    #[doc = "Internal voltage reference"]
    REF,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ASRCPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASRCPR::PIN => 0,
            ASRCPR::PIN0 => 1,
            ASRCPR::REF => 2,
            ASRCPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASRCPR {
        match value {
            0 => ASRCPR::PIN,
            1 => ASRCPR::PIN0,
            2 => ASRCPR::REF,
            i => ASRCPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PIN`"]
    #[inline]
    pub fn is_pin(&self) -> bool {
        *self == ASRCPR::PIN
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline]
    pub fn is_pin0(&self) -> bool {
        *self == ASRCPR::PIN0
    }
    #[doc = "Checks if the value of the field is `REF`"]
    #[inline]
    pub fn is_ref_(&self) -> bool {
        *self == ASRCPR::REF
    }
}
#[doc = r" Value of the field"]
pub struct TOENR {
    bits: bool,
}
impl TOENR {
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
#[doc = r" Proxy"]
pub struct _CINVW<'a> {
    w: &'a mut W,
}
impl<'a> _CINVW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ISEN`"]
pub enum ISENW {
    #[doc = "Level sense, see ISLVAL"]
    LEVEL,
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
    #[doc = "Either edge"]
    BOTH,
}
impl ISENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ISENW::LEVEL => 0,
            ISENW::FALL => 1,
            ISENW::RISE => 2,
            ISENW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISENW<'a> {
    w: &'a mut W,
}
impl<'a> _ISENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level sense, see ISLVAL"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(ISENW::LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(ISENW::FALL)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(ISENW::RISE)
    }
    #[doc = "Either edge"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(ISENW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ISLVALW<'a> {
    w: &'a mut W,
}
impl<'a> _ISLVALW<'a> {
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
#[doc = "Values that can be written to the field `TSEN`"]
pub enum TSENW {
    #[doc = "Level sense, see TSLVAL"]
    LEVEL,
    #[doc = "Falling edge"]
    FALL,
    #[doc = "Rising edge"]
    RISE,
    #[doc = "Either edge"]
    BOTH,
}
impl TSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSENW::LEVEL => 0,
            TSENW::FALL => 1,
            TSENW::RISE => 2,
            TSENW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level sense, see TSLVAL"]
    #[inline]
    pub fn level(self) -> &'a mut W {
        self.variant(TSENW::LEVEL)
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn fall(self) -> &'a mut W {
        self.variant(TSENW::FALL)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rise(self) -> &'a mut W {
        self.variant(TSENW::RISE)
    }
    #[doc = "Either edge"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(TSENW::BOTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSLVALW<'a> {
    w: &'a mut W,
}
impl<'a> _TSLVALW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASRCP`"]
pub enum ASRCPW {
    #[doc = "Pin value of Cn+"]
    PIN,
    #[doc = "Pin value of C0+"]
    PIN0,
    #[doc = "Internal voltage reference"]
    REF,
}
impl ASRCPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASRCPW::PIN => 0,
            ASRCPW::PIN0 => 1,
            ASRCPW::REF => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASRCPW<'a> {
    w: &'a mut W,
}
impl<'a> _ASRCPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASRCPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Pin value of Cn+"]
    #[inline]
    pub fn pin(self) -> &'a mut W {
        self.variant(ASRCPW::PIN)
    }
    #[doc = "Pin value of C0+"]
    #[inline]
    pub fn pin0(self) -> &'a mut W {
        self.variant(ASRCPW::PIN0)
    }
    #[doc = "Internal voltage reference"]
    #[inline]
    pub fn ref_(self) -> &'a mut W {
        self.variant(ASRCPW::REF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TOENW<'a> {
    w: &'a mut W,
}
impl<'a> _TOENW<'a> {
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline]
    pub fn cinv(&self) -> CINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CINVR { bits }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline]
    pub fn isen(&self) -> ISENR {
        ISENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline]
    pub fn islval(&self) -> ISLVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ISLVALR { bits }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline]
    pub fn tsen(&self) -> TSENR {
        TSENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline]
    pub fn tslval(&self) -> TSLVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSLVALR { bits }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline]
    pub fn asrcp(&self) -> ASRCPR {
        ASRCPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline]
    pub fn toen(&self) -> TOENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TOENR { bits }
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
    #[doc = "Bit 1 - Comparator Output Invert"]
    #[inline]
    pub fn cinv(&mut self) -> _CINVW {
        _CINVW { w: self }
    }
    #[doc = "Bits 2:3 - Interrupt Sense"]
    #[inline]
    pub fn isen(&mut self) -> _ISENW {
        _ISENW { w: self }
    }
    #[doc = "Bit 4 - Interrupt Sense Level Value"]
    #[inline]
    pub fn islval(&mut self) -> _ISLVALW {
        _ISLVALW { w: self }
    }
    #[doc = "Bits 5:6 - Trigger Sense"]
    #[inline]
    pub fn tsen(&mut self) -> _TSENW {
        _TSENW { w: self }
    }
    #[doc = "Bit 7 - Trigger Sense Level Value"]
    #[inline]
    pub fn tslval(&mut self) -> _TSLVALW {
        _TSLVALW { w: self }
    }
    #[doc = "Bits 9:10 - Analog Source Positive"]
    #[inline]
    pub fn asrcp(&mut self) -> _ASRCPW {
        _ASRCPW { w: self }
    }
    #[doc = "Bit 11 - Trigger Output Enable"]
    #[inline]
    pub fn toen(&mut self) -> _TOENW {
        _TOENW { w: self }
    }
}
