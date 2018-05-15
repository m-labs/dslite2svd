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
}
