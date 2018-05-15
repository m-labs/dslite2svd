#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct TAENR {
    bits: bool,
}
impl TAENR {
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
pub struct TASTALLR {
    bits: bool,
}
impl TASTALLR {
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
#[doc = "Possible values of the field `TAEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAEVENTR {
    #[doc = "Positive edge"]
    POS,
    #[doc = "Negative edge"]
    NEG,
    #[doc = "Both edges"]
    BOTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TAEVENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TAEVENTR::POS => 0,
            TAEVENTR::NEG => 1,
            TAEVENTR::BOTH => 3,
            TAEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TAEVENTR {
        match value {
            0 => TAEVENTR::POS,
            1 => TAEVENTR::NEG,
            3 => TAEVENTR::BOTH,
            i => TAEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == TAEVENTR::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == TAEVENTR::NEG
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == TAEVENTR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct RTCENR {
    bits: bool,
}
impl RTCENR {
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
pub struct TAOTER {
    bits: bool,
}
impl TAOTER {
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
pub struct TAPWMLR {
    bits: bool,
}
impl TAPWMLR {
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
pub struct TBENR {
    bits: bool,
}
impl TBENR {
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
pub struct TBSTALLR {
    bits: bool,
}
impl TBSTALLR {
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
#[doc = "Possible values of the field `TBEVENT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TBEVENTR {
    #[doc = "Positive edge"]
    POS,
    #[doc = "Negative edge"]
    NEG,
    #[doc = "Both edges"]
    BOTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TBEVENTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TBEVENTR::POS => 0,
            TBEVENTR::NEG => 1,
            TBEVENTR::BOTH => 3,
            TBEVENTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TBEVENTR {
        match value {
            0 => TBEVENTR::POS,
            1 => TBEVENTR::NEG,
            3 => TBEVENTR::BOTH,
            i => TBEVENTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS`"]
    #[inline]
    pub fn is_pos(&self) -> bool {
        *self == TBEVENTR::POS
    }
    #[doc = "Checks if the value of the field is `NEG`"]
    #[inline]
    pub fn is_neg(&self) -> bool {
        *self == TBEVENTR::NEG
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == TBEVENTR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct TBOTER {
    bits: bool,
}
impl TBOTER {
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
pub struct TBPWMLR {
    bits: bool,
}
impl TBPWMLR {
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
pub struct _TAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TAENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TASTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TASTALLW<'a> {
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
#[doc = "Values that can be written to the field `TAEVENT`"]
pub enum TAEVENTW {
    #[doc = "Positive edge"]
    POS,
    #[doc = "Negative edge"]
    NEG,
    #[doc = "Both edges"]
    BOTH,
}
impl TAEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TAEVENTW::POS => 0,
            TAEVENTW::NEG => 1,
            TAEVENTW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TAEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Positive edge"]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(TAEVENTW::POS)
    }
    #[doc = "Negative edge"]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(TAEVENTW::NEG)
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(TAEVENTW::BOTH)
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
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
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
pub struct _TAOTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TAOTEW<'a> {
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
pub struct _TAPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TAPWMLW<'a> {
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
pub struct _TBENW<'a> {
    w: &'a mut W,
}
impl<'a> _TBENW<'a> {
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
pub struct _TBSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TBSTALLW<'a> {
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
#[doc = "Values that can be written to the field `TBEVENT`"]
pub enum TBEVENTW {
    #[doc = "Positive edge"]
    POS,
    #[doc = "Negative edge"]
    NEG,
    #[doc = "Both edges"]
    BOTH,
}
impl TBEVENTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TBEVENTW::POS => 0,
            TBEVENTW::NEG => 1,
            TBEVENTW::BOTH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TBEVENTW<'a> {
    w: &'a mut W,
}
impl<'a> _TBEVENTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TBEVENTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Positive edge"]
    #[inline]
    pub fn pos(self) -> &'a mut W {
        self.variant(TBEVENTW::POS)
    }
    #[doc = "Negative edge"]
    #[inline]
    pub fn neg(self) -> &'a mut W {
        self.variant(TBEVENTW::NEG)
    }
    #[doc = "Both edges"]
    #[inline]
    pub fn both(self) -> &'a mut W {
        self.variant(TBEVENTW::BOTH)
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
pub struct _TBOTEW<'a> {
    w: &'a mut W,
}
impl<'a> _TBOTEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TBPWMLW<'a> {
    w: &'a mut W,
}
impl<'a> _TBPWMLW<'a> {
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
        const OFFSET: u8 = 14;
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
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline]
    pub fn taen(&self) -> TAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAENR { bits }
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline]
    pub fn tastall(&self) -> TASTALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TASTALLR { bits }
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline]
    pub fn taevent(&self) -> TAEVENTR {
        TAEVENTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCENR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline]
    pub fn taote(&self) -> TAOTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAOTER { bits }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline]
    pub fn tapwml(&self) -> TAPWMLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAPWMLR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline]
    pub fn tben(&self) -> TBENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBENR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline]
    pub fn tbstall(&self) -> TBSTALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBSTALLR { bits }
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline]
    pub fn tbevent(&self) -> TBEVENTR {
        TBEVENTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline]
    pub fn tbote(&self) -> TBOTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBOTER { bits }
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline]
    pub fn tbpwml(&self) -> TBPWMLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TBPWMLR { bits }
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
    #[doc = "Bit 0 - GPTM Timer A Enable"]
    #[inline]
    pub fn taen(&mut self) -> _TAENW {
        _TAENW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Stall Enable"]
    #[inline]
    pub fn tastall(&mut self) -> _TASTALLW {
        _TASTALLW { w: self }
    }
    #[doc = "Bits 2:3 - GPTM Timer A Event Mode"]
    #[inline]
    pub fn taevent(&mut self) -> _TAEVENTW {
        _TAEVENTW { w: self }
    }
    #[doc = "Bit 4 - GPTM RTC Stall Enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A Output Trigger Enable"]
    #[inline]
    pub fn taote(&mut self) -> _TAOTEW {
        _TAOTEW { w: self }
    }
    #[doc = "Bit 6 - GPTM Timer A PWM Output Level"]
    #[inline]
    pub fn tapwml(&mut self) -> _TAPWMLW {
        _TAPWMLW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Enable"]
    #[inline]
    pub fn tben(&mut self) -> _TBENW {
        _TBENW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Stall Enable"]
    #[inline]
    pub fn tbstall(&mut self) -> _TBSTALLW {
        _TBSTALLW { w: self }
    }
    #[doc = "Bits 10:11 - GPTM Timer B Event Mode"]
    #[inline]
    pub fn tbevent(&mut self) -> _TBEVENTW {
        _TBEVENTW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B Output Trigger Enable"]
    #[inline]
    pub fn tbote(&mut self) -> _TBOTEW {
        _TBOTEW { w: self }
    }
    #[doc = "Bit 14 - GPTM Timer B PWM Output Level"]
    #[inline]
    pub fn tbpwml(&mut self) -> _TBPWMLW {
        _TBPWMLW { w: self }
    }
}
