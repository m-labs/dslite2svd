#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TBMR {
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
#[doc = "Possible values of the field `TBMR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBMRR {
    #[doc = "One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "Periodic Timer mode"]
    PERIOD,
    #[doc = "Capture mode"]
    CAP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TBMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TBMRR::_1_SHOT => 1,
            TBMRR::PERIOD => 2,
            TBMRR::CAP => 3,
            TBMRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TBMRR {
        match value {
            1 => TBMRR::_1_SHOT,
            2 => TBMRR::PERIOD,
            3 => TBMRR::CAP,
            i => TBMRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_SHOT`"]
    #[inline]
    pub fn is_1_shot(&self) -> bool {
        *self == TBMRR::_1_SHOT
    }
    #[doc = "Checks if the value of the field is `PERIOD`"]
    #[inline]
    pub fn is_period(&self) -> bool {
        *self == TBMRR::PERIOD
    }
    #[doc = "Checks if the value of the field is `CAP`"]
    #[inline]
    pub fn is_cap(&self) -> bool {
        *self == TBMRR::CAP
    }
}
#[doc = r" Value of the field"]
pub struct TBCMRR {
    bits: bool,
}
impl TBCMRR {
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
pub struct TBAMSR {
    bits: bool,
}
impl TBAMSR {
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
pub struct TBCDIRR {
    bits: bool,
}
impl TBCDIRR {
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
pub struct TBMIER {
    bits: bool,
}
impl TBMIER {
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
pub struct TBWOTR {
    bits: bool,
}
impl TBWOTR {
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
pub struct TBSNAPSR {
    bits: bool,
}
impl TBSNAPSR {
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
pub struct TBILDR {
    bits: bool,
}
impl TBILDR {
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
pub struct TBPWMIER {
    bits: bool,
}
impl TBPWMIER {
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
pub struct TBMRSUR {
    bits: bool,
}
impl TBMRSUR {
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
pub struct TBPLOR {
    bits: bool,
}
impl TBPLOR {
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
#[doc = "Values that can be written to the field `TBMR`"]
pub enum TBMRW {
    #[doc = "One-Shot Timer mode"]
    _1_SHOT,
    #[doc = "Periodic Timer mode"]
    PERIOD,
    #[doc = "Capture mode"]
    CAP,
}
impl TBMRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TBMRW::_1_SHOT => 1,
            TBMRW::PERIOD => 2,
            TBMRW::CAP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBMRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline]
    pub fn _1_shot(self) -> &'a mut W {
        self.variant(TBMRW::_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline]
    pub fn period(self) -> &'a mut W {
        self.variant(TBMRW::PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline]
    pub fn cap(self) -> &'a mut W {
        self.variant(TBMRW::CAP)
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
pub struct _TBCMRW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCMRW<'a> {
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
pub struct _TBAMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TBAMSW<'a> {
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
pub struct _TBCDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TBCDIRW<'a> {
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
pub struct _TBMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMIEW<'a> {
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
pub struct _TBWOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TBWOTW<'a> {
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
pub struct _TBSNAPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TBSNAPSW<'a> {
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
pub struct _TBILDW<'a> {
    w: &'a mut W,
}
impl<'a> _TBILDW<'a> {
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
pub struct _TBPWMIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPWMIEW<'a> {
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
pub struct _TBMRSUW<'a> {
    w: &'a mut W,
}
impl<'a> _TBMRSUW<'a> {
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
pub struct _TBPLOW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPLOW<'a> {
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
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline]
    pub fn tbmr(&self) -> TBMRR {
        TBMRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline]
    pub fn tbcmr(&self) -> TBCMRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBCMRR { bits }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline]
    pub fn tbams(&self) -> TBAMSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBAMSR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline]
    pub fn tbcdir(&self) -> TBCDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBCDIRR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline]
    pub fn tbmie(&self) -> TBMIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBMIER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline]
    pub fn tbwot(&self) -> TBWOTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBWOTR { bits }
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline]
    pub fn tbsnaps(&self) -> TBSNAPSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBSNAPSR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline]
    pub fn tbild(&self) -> TBILDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBILDR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline]
    pub fn tbpwmie(&self) -> TBPWMIER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBPWMIER { bits }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline]
    pub fn tbmrsu(&self) -> TBMRSUR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBMRSUR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline]
    pub fn tbplo(&self) -> TBPLOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBPLOR { bits }
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
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline]
    pub fn tbmr(&mut self) -> _TBMRW {
        _TBMRW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline]
    pub fn tbcmr(&mut self) -> _TBCMRW {
        _TBCMRW { w: self }
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline]
    pub fn tbams(&mut self) -> _TBAMSW {
        _TBAMSW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline]
    pub fn tbcdir(&mut self) -> _TBCDIRW {
        _TBCDIRW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline]
    pub fn tbmie(&mut self) -> _TBMIEW {
        _TBMIEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline]
    pub fn tbwot(&mut self) -> _TBWOTW {
        _TBWOTW { w: self }
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline]
    pub fn tbsnaps(&mut self) -> _TBSNAPSW {
        _TBSNAPSW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline]
    pub fn tbild(&mut self) -> _TBILDW {
        _TBILDW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline]
    pub fn tbpwmie(&mut self) -> _TBPWMIEW {
        _TBPWMIEW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline]
    pub fn tbmrsu(&mut self) -> _TBMRSUW {
        _TBMRSUW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline]
    pub fn tbplo(&mut self) -> _TBPLOW {
        _TBPLOW { w: self }
    }
}
