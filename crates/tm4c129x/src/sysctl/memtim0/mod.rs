#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MEMTIM0 {
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
pub struct FWSR {
    bits: u8,
}
impl FWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FBCER {
    bits: bool,
}
impl FBCER {
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
#[doc = "Possible values of the field `FBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FBCHTR {
    #[doc = "1/2 system clock period"]
    _0_5,
    #[doc = "1 system clock period"]
    _1,
    #[doc = "1.5 system clock periods"]
    _1_5,
    #[doc = "2 system clock periods"]
    _2,
    #[doc = "2.5 system clock periods"]
    _2_5,
    #[doc = "3 system clock periods"]
    _3,
    #[doc = "3.5 system clock periods"]
    _3_5,
    #[doc = "4 system clock periods"]
    _4,
    #[doc = "4.5 system clock periods"]
    _4_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FBCHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FBCHTR::_0_5 => 0,
            FBCHTR::_1 => 1,
            FBCHTR::_1_5 => 2,
            FBCHTR::_2 => 3,
            FBCHTR::_2_5 => 4,
            FBCHTR::_3 => 5,
            FBCHTR::_3_5 => 6,
            FBCHTR::_4 => 7,
            FBCHTR::_4_5 => 8,
            FBCHTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FBCHTR {
        match value {
            0 => FBCHTR::_0_5,
            1 => FBCHTR::_1,
            2 => FBCHTR::_1_5,
            3 => FBCHTR::_2,
            4 => FBCHTR::_2_5,
            5 => FBCHTR::_3,
            6 => FBCHTR::_3_5,
            7 => FBCHTR::_4,
            8 => FBCHTR::_4_5,
            i => FBCHTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5`"]
    #[inline]
    pub fn is_0_5(&self) -> bool {
        *self == FBCHTR::_0_5
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FBCHTR::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline]
    pub fn is_1_5(&self) -> bool {
        *self == FBCHTR::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == FBCHTR::_2
    }
    #[doc = "Checks if the value of the field is `_2_5`"]
    #[inline]
    pub fn is_2_5(&self) -> bool {
        *self == FBCHTR::_2_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == FBCHTR::_3
    }
    #[doc = "Checks if the value of the field is `_3_5`"]
    #[inline]
    pub fn is_3_5(&self) -> bool {
        *self == FBCHTR::_3_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == FBCHTR::_4
    }
    #[doc = "Checks if the value of the field is `_4_5`"]
    #[inline]
    pub fn is_4_5(&self) -> bool {
        *self == FBCHTR::_4_5
    }
}
#[doc = r" Value of the field"]
pub struct EWSR {
    bits: u8,
}
impl EWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EBCER {
    bits: bool,
}
impl EBCER {
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
#[doc = "Possible values of the field `EBCHT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBCHTR {
    #[doc = "1/2 system clock period"]
    _0_5,
    #[doc = "1 system clock period"]
    _1,
    #[doc = "1.5 system clock periods"]
    _1_5,
    #[doc = "2 system clock periods"]
    _2,
    #[doc = "2.5 system clock periods"]
    _2_5,
    #[doc = "3 system clock periods"]
    _3,
    #[doc = "3.5 system clock periods"]
    _3_5,
    #[doc = "4 system clock periods"]
    _4,
    #[doc = "4.5 system clock periods"]
    _4_5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EBCHTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EBCHTR::_0_5 => 0,
            EBCHTR::_1 => 1,
            EBCHTR::_1_5 => 2,
            EBCHTR::_2 => 3,
            EBCHTR::_2_5 => 4,
            EBCHTR::_3 => 5,
            EBCHTR::_3_5 => 6,
            EBCHTR::_4 => 7,
            EBCHTR::_4_5 => 8,
            EBCHTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EBCHTR {
        match value {
            0 => EBCHTR::_0_5,
            1 => EBCHTR::_1,
            2 => EBCHTR::_1_5,
            3 => EBCHTR::_2,
            4 => EBCHTR::_2_5,
            5 => EBCHTR::_3,
            6 => EBCHTR::_3_5,
            7 => EBCHTR::_4,
            8 => EBCHTR::_4_5,
            i => EBCHTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_5`"]
    #[inline]
    pub fn is_0_5(&self) -> bool {
        *self == EBCHTR::_0_5
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EBCHTR::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline]
    pub fn is_1_5(&self) -> bool {
        *self == EBCHTR::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == EBCHTR::_2
    }
    #[doc = "Checks if the value of the field is `_2_5`"]
    #[inline]
    pub fn is_2_5(&self) -> bool {
        *self == EBCHTR::_2_5
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline]
    pub fn is_3(&self) -> bool {
        *self == EBCHTR::_3
    }
    #[doc = "Checks if the value of the field is `_3_5`"]
    #[inline]
    pub fn is_3_5(&self) -> bool {
        *self == EBCHTR::_3_5
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == EBCHTR::_4
    }
    #[doc = "Checks if the value of the field is `_4_5`"]
    #[inline]
    pub fn is_4_5(&self) -> bool {
        *self == EBCHTR::_4_5
    }
}
#[doc = r" Proxy"]
pub struct _FWSW<'a> {
    w: &'a mut W,
}
impl<'a> _FWSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FBCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCEW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FBCHT`"]
pub enum FBCHTW {
    #[doc = "1/2 system clock period"]
    _0_5,
    #[doc = "1 system clock period"]
    _1,
    #[doc = "1.5 system clock periods"]
    _1_5,
    #[doc = "2 system clock periods"]
    _2,
    #[doc = "2.5 system clock periods"]
    _2_5,
    #[doc = "3 system clock periods"]
    _3,
    #[doc = "3.5 system clock periods"]
    _3_5,
    #[doc = "4 system clock periods"]
    _4,
    #[doc = "4.5 system clock periods"]
    _4_5,
}
impl FBCHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FBCHTW::_0_5 => 0,
            FBCHTW::_1 => 1,
            FBCHTW::_1_5 => 2,
            FBCHTW::_2 => 3,
            FBCHTW::_2_5 => 4,
            FBCHTW::_3 => 5,
            FBCHTW::_3_5 => 6,
            FBCHTW::_4 => 7,
            FBCHTW::_4_5 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FBCHTW<'a> {
    w: &'a mut W,
}
impl<'a> _FBCHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FBCHTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline]
    pub fn _0_5(self) -> &'a mut W {
        self.variant(FBCHTW::_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FBCHTW::_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(FBCHTW::_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(FBCHTW::_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline]
    pub fn _2_5(self) -> &'a mut W {
        self.variant(FBCHTW::_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(FBCHTW::_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline]
    pub fn _3_5(self) -> &'a mut W {
        self.variant(FBCHTW::_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(FBCHTW::_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline]
    pub fn _4_5(self) -> &'a mut W {
        self.variant(FBCHTW::_4_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EWSW<'a> {
    w: &'a mut W,
}
impl<'a> _EWSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EBCEW<'a> {
    w: &'a mut W,
}
impl<'a> _EBCEW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EBCHT`"]
pub enum EBCHTW {
    #[doc = "1/2 system clock period"]
    _0_5,
    #[doc = "1 system clock period"]
    _1,
    #[doc = "1.5 system clock periods"]
    _1_5,
    #[doc = "2 system clock periods"]
    _2,
    #[doc = "2.5 system clock periods"]
    _2_5,
    #[doc = "3 system clock periods"]
    _3,
    #[doc = "3.5 system clock periods"]
    _3_5,
    #[doc = "4 system clock periods"]
    _4,
    #[doc = "4.5 system clock periods"]
    _4_5,
}
impl EBCHTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EBCHTW::_0_5 => 0,
            EBCHTW::_1 => 1,
            EBCHTW::_1_5 => 2,
            EBCHTW::_2 => 3,
            EBCHTW::_2_5 => 4,
            EBCHTW::_3 => 5,
            EBCHTW::_3_5 => 6,
            EBCHTW::_4 => 7,
            EBCHTW::_4_5 => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EBCHTW<'a> {
    w: &'a mut W,
}
impl<'a> _EBCHTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EBCHTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1/2 system clock period"]
    #[inline]
    pub fn _0_5(self) -> &'a mut W {
        self.variant(EBCHTW::_0_5)
    }
    #[doc = "1 system clock period"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EBCHTW::_1)
    }
    #[doc = "1.5 system clock periods"]
    #[inline]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(EBCHTW::_1_5)
    }
    #[doc = "2 system clock periods"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(EBCHTW::_2)
    }
    #[doc = "2.5 system clock periods"]
    #[inline]
    pub fn _2_5(self) -> &'a mut W {
        self.variant(EBCHTW::_2_5)
    }
    #[doc = "3 system clock periods"]
    #[inline]
    pub fn _3(self) -> &'a mut W {
        self.variant(EBCHTW::_3)
    }
    #[doc = "3.5 system clock periods"]
    #[inline]
    pub fn _3_5(self) -> &'a mut W {
        self.variant(EBCHTW::_3_5)
    }
    #[doc = "4 system clock periods"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(EBCHTW::_4)
    }
    #[doc = "4.5 system clock periods"]
    #[inline]
    pub fn _4_5(self) -> &'a mut W {
        self.variant(EBCHTW::_4_5)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline]
    pub fn fws(&self) -> FWSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FWSR { bits }
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline]
    pub fn fbce(&self) -> FBCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FBCER { bits }
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline]
    pub fn fbcht(&self) -> FBCHTR {
        FBCHTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline]
    pub fn ews(&self) -> EWSR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        EWSR { bits }
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline]
    pub fn ebce(&self) -> EBCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EBCER { bits }
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline]
    pub fn ebcht(&self) -> EBCHTR {
        EBCHTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 22;
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
    #[doc = "Bits 0:3 - Flash Wait State"]
    #[inline]
    pub fn fws(&mut self) -> _FWSW {
        _FWSW { w: self }
    }
    #[doc = "Bit 5 - Flash Bank Clock Edge"]
    #[inline]
    pub fn fbce(&mut self) -> _FBCEW {
        _FBCEW { w: self }
    }
    #[doc = "Bits 6:9 - Flash Bank Clock High Time"]
    #[inline]
    pub fn fbcht(&mut self) -> _FBCHTW {
        _FBCHTW { w: self }
    }
    #[doc = "Bits 16:19 - EEPROM Wait States"]
    #[inline]
    pub fn ews(&mut self) -> _EWSW {
        _EWSW { w: self }
    }
    #[doc = "Bit 21 - EEPROM Bank Clock Edge"]
    #[inline]
    pub fn ebce(&mut self) -> _EBCEW {
        _EBCEW { w: self }
    }
    #[doc = "Bits 22:25 - EEPROM Clock High Time"]
    #[inline]
    pub fn ebcht(&mut self) -> _EBCHTW {
        _EBCHTW { w: self }
    }
}
