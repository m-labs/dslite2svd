#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TAMR {
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
#[doc = "Possible values of the field `TAMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMRR {
    #[doc = "One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "Periodic Timer mode"]
    PERIOD,
    #[doc = "Capture mode"]
    CAP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TAMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAMRR::_1_SHOT => 1,
            TAMRR::PERIOD => 2,
            TAMRR::CAP => 3,
            TAMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAMRR {
        match value {
            1 => TAMRR::_1_SHOT,
            2 => TAMRR::PERIOD,
            3 => TAMRR::CAP,
            i => TAMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SHOT`"]
    #[inline]
    pub fn is_1_shot(&self) -> bool {
        *self == TAMRR::_1_SHOT
    }
    #[doc = "Checks if the value of the field is `PERIOD`"]
    #[inline]
    pub fn is_period(&self) -> bool {
        *self == TAMRR::PERIOD
    }
    #[doc = "Checks if the value of the field is `CAP`"]
    #[inline]
    pub fn is_cap(&self) -> bool {
        *self == TAMRR::CAP
    }
}
#[doc = r" Value of the field"]
pub struct TACMRR {
    bits: bool,
}
impl TACMRR {
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
pub struct TAAMSR {
    bits: bool,
}
impl TAAMSR {
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
pub struct TACDIRR {
    bits: bool,
}
impl TACDIRR {
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
pub struct TAMIER {
    bits: bool,
}
impl TAMIER {
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
pub struct TAWOTR {
    bits: bool,
}
impl TAWOTR {
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
pub struct TASNAPSR {
    bits: bool,
}
impl TASNAPSR {
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
pub struct TAILDR {
    bits: bool,
}
impl TAILDR {
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
pub struct TAPWMIER {
    bits: bool,
}
impl TAPWMIER {
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
pub struct TAMRSUR {
    bits: bool,
}
impl TAMRSUR {
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
pub struct TAPLOR {
    bits: bool,
}
impl TAPLOR {
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
pub struct TACINTDR {
    bits: bool,
}
impl TACINTDR {
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
#[doc = "Possible values of the field `TCACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCACTR {
    #[doc = "Disable compare operations"]
    NONE,
    #[doc = "Toggle State on Time-Out"]
    TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    CLRTO,
    #[doc = "Set CCP on Time-Out"]
    SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    CLRSETTO,
}
impl TCACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCACTR::NONE => 0,
            TCACTR::TOGGLE => 1,
            TCACTR::CLRTO => 2,
            TCACTR::SETTO => 3,
            TCACTR::SETTOGTO => 4,
            TCACTR::CLRTOGTO => 5,
            TCACTR::SETCLRTO => 6,
            TCACTR::CLRSETTO => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCACTR {
        match value {
            0 => TCACTR::NONE,
            1 => TCACTR::TOGGLE,
            2 => TCACTR::CLRTO,
            3 => TCACTR::SETTO,
            4 => TCACTR::SETTOGTO,
            5 => TCACTR::CLRTOGTO,
            6 => TCACTR::SETCLRTO,
            7 => TCACTR::CLRSETTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == TCACTR::NONE
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == TCACTR::TOGGLE
    }
    #[doc = "Checks if the value of the field is `CLRTO`"]
    #[inline]
    pub fn is_clrto(&self) -> bool {
        *self == TCACTR::CLRTO
    }
    #[doc = "Checks if the value of the field is `SETTO`"]
    #[inline]
    pub fn is_setto(&self) -> bool {
        *self == TCACTR::SETTO
    }
    #[doc = "Checks if the value of the field is `SETTOGTO`"]
    #[inline]
    pub fn is_settogto(&self) -> bool {
        *self == TCACTR::SETTOGTO
    }
    #[doc = "Checks if the value of the field is `CLRTOGTO`"]
    #[inline]
    pub fn is_clrtogto(&self) -> bool {
        *self == TCACTR::CLRTOGTO
    }
    #[doc = "Checks if the value of the field is `SETCLRTO`"]
    #[inline]
    pub fn is_setclrto(&self) -> bool {
        *self == TCACTR::SETCLRTO
    }
    #[doc = "Checks if the value of the field is `CLRSETTO`"]
    #[inline]
    pub fn is_clrsetto(&self) -> bool {
        *self == TCACTR::CLRSETTO
    }
}
#[doc = "Values that can be written to the field `TAMR`"]
pub enum TAMRW {
    #[doc = "One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "Periodic Timer mode"]
    PERIOD,
    #[doc = "Capture mode"]
    CAP,
}
impl TAMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAMRW::_1_SHOT => 1,
            TAMRW::PERIOD => 2,
            TAMRW::CAP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline]
    pub fn _1_shot(self) -> &'a mut W {
        self.variant(TAMRW::_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline]
    pub fn period(self) -> &'a mut W {
        self.variant(TAMRW::PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline]
    pub fn cap(self) -> &'a mut W {
        self.variant(TAMRW::CAP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TACMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TACMRW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TAAMSW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TACDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TACDIRW<'a> {
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
#[doc = r" Proxy"]
pub struct _TAMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMIEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TAWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TAWOTW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TASNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TASNAPSW<'a> {
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
#[doc = r" Proxy"]
pub struct _TAILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TAILDW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPWMIEW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TAMRSUW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPLOW<'a> {
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
#[doc = r" Proxy"]
pub struct _TACINTDW<'a> {
    w: &'a mut W,
}
impl<'a> _TACINTDW<'a> {
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
#[doc = "Values that can be written to the field `TCACT`"]
pub enum TCACTW {
    #[doc = "Disable compare operations"]
    NONE,
    #[doc = "Toggle State on Time-Out"]
    TOGGLE,
    #[doc = "Clear CCP on Time-Out"]
    CLRTO,
    #[doc = "Set CCP on Time-Out"]
    SETTO,
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    SETTOGTO,
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    CLRTOGTO,
    #[doc = "Set CCP immediately and clear on Time-Out"]
    SETCLRTO,
    #[doc = "Clear CCP immediately and set on Time-Out"]
    CLRSETTO,
}
impl TCACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCACTW::NONE => 0,
            TCACTW::TOGGLE => 1,
            TCACTW::CLRTO => 2,
            TCACTW::SETTO => 3,
            TCACTW::SETTOGTO => 4,
            TCACTW::CLRTOGTO => 5,
            TCACTW::SETCLRTO => 6,
            TCACTW::CLRSETTO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCACTW<'a> {
    w: &'a mut W,
}
impl<'a> _TCACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable compare operations"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(TCACTW::NONE)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(TCACTW::TOGGLE)
    }
    #[doc = "Clear CCP on Time-Out"]
    #[inline]
    pub fn clrto(self) -> &'a mut W {
        self.variant(TCACTW::CLRTO)
    }
    #[doc = "Set CCP on Time-Out"]
    #[inline]
    pub fn setto(self) -> &'a mut W {
        self.variant(TCACTW::SETTO)
    }
    #[doc = "Set CCP immediately and toggle on Time-Out"]
    #[inline]
    pub fn settogto(self) -> &'a mut W {
        self.variant(TCACTW::SETTOGTO)
    }
    #[doc = "Clear CCP immediately and toggle on Time-Out"]
    #[inline]
    pub fn clrtogto(self) -> &'a mut W {
        self.variant(TCACTW::CLRTOGTO)
    }
    #[doc = "Set CCP immediately and clear on Time-Out"]
    #[inline]
    pub fn setclrto(self) -> &'a mut W {
        self.variant(TCACTW::SETCLRTO)
    }
    #[doc = "Clear CCP immediately and set on Time-Out"]
    #[inline]
    pub fn clrsetto(self) -> &'a mut W {
        self.variant(TCACTW::CLRSETTO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline]
    pub fn tamr(&self) -> TAMRR {
        TAMRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline]
    pub fn tacmr(&self) -> TACMRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TACMRR { bits }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline]
    pub fn taams(&self) -> TAAMSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAAMSR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline]
    pub fn tacdir(&self) -> TACDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TACDIRR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline]
    pub fn tamie(&self) -> TAMIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMIER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline]
    pub fn tawot(&self) -> TAWOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAWOTR { bits }
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline]
    pub fn tasnaps(&self) -> TASNAPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TASNAPSR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline]
    pub fn taild(&self) -> TAILDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAILDR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline]
    pub fn tapwmie(&self) -> TAPWMIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAPWMIER { bits }
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline]
    pub fn tamrsu(&self) -> TAMRSUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAMRSUR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline]
    pub fn taplo(&self) -> TAPLOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAPLOR { bits }
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline]
    pub fn tacintd(&self) -> TACINTDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TACINTDR { bits }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline]
    pub fn tcact(&self) -> TCACTR {
        TCACTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline]
    pub fn tamr(&mut self) -> _TAMRW {
        _TAMRW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline]
    pub fn tacmr(&mut self) -> _TACMRW {
        _TACMRW { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline]
    pub fn taams(&mut self) -> _TAAMSW {
        _TAAMSW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline]
    pub fn tacdir(&mut self) -> _TACDIRW {
        _TACDIRW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline]
    pub fn tamie(&mut self) -> _TAMIEW {
        _TAMIEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline]
    pub fn tawot(&mut self) -> _TAWOTW {
        _TAWOTW { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline]
    pub fn tasnaps(&mut self) -> _TASNAPSW {
        _TASNAPSW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline]
    pub fn taild(&mut self) -> _TAILDW {
        _TAILDW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline]
    pub fn tapwmie(&mut self) -> _TAPWMIEW {
        _TAPWMIEW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline]
    pub fn tamrsu(&mut self) -> _TAMRSUW {
        _TAMRSUW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline]
    pub fn taplo(&mut self) -> _TAPLOW {
        _TAPLOW { w: self }
    }
    #[doc = "Bit 12 - One-shot/Periodic Interrupt Disable"]
    #[inline]
    pub fn tacintd(&mut self) -> _TACINTDW {
        _TACINTDW { w: self }
    }
    #[doc = "Bits 13:15 - Timer Compare Action Select"]
    #[inline]
    pub fn tcact(&mut self) -> _TCACTW {
        _TCACTW { w: self }
    }
}
