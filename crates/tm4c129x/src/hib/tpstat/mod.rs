#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPSTAT {
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
pub struct XOSCFAILR {
    bits: bool,
}
impl XOSCFAILR {
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
pub struct XOSCSTR {
    bits: bool,
}
impl XOSCSTR {
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
#[doc = "Possible values of the field `STATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATER {
    #[doc = "Tamper disabled"]
    DISABLED,
    #[doc = "Tamper configured"]
    CONFIGED,
    #[doc = "Tamper pin event occurred"]
    ERROR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STATER::DISABLED => 0,
            STATER::CONFIGED => 1,
            STATER::ERROR => 2,
            STATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STATER {
        match value {
            0 => STATER::DISABLED,
            1 => STATER::CONFIGED,
            2 => STATER::ERROR,
            i => STATER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STATER::DISABLED
    }
    #[doc = "Checks if the value of the field is `CONFIGED`"]
    #[inline]
    pub fn is_configed(&self) -> bool {
        *self == STATER::CONFIGED
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline]
    pub fn is_error(&self) -> bool {
        *self == STATER::ERROR
    }
}
#[doc = r" Proxy"]
pub struct _XOSCFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCFAILW<'a> {
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
pub struct _XOSCSTW<'a> {
    w: &'a mut W,
}
impl<'a> _XOSCSTW<'a> {
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
#[doc = "Values that can be written to the field `STATE`"]
pub enum STATEW {
    #[doc = "Tamper disabled"]
    DISABLED,
    #[doc = "Tamper configured"]
    CONFIGED,
    #[doc = "Tamper pin event occurred"]
    ERROR,
}
impl STATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STATEW::DISABLED => 0,
            STATEW::CONFIGED => 1,
            STATEW::ERROR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STATEW<'a> {
    w: &'a mut W,
}
impl<'a> _STATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STATEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Tamper disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STATEW::DISABLED)
    }
    #[doc = "Tamper configured"]
    #[inline]
    pub fn configed(self) -> &'a mut W {
        self.variant(STATEW::CONFIGED)
    }
    #[doc = "Tamper pin event occurred"]
    #[inline]
    pub fn error(self) -> &'a mut W {
        self.variant(STATEW::ERROR)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline]
    pub fn xoscfail(&self) -> XOSCFAILR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSCFAILR { bits }
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline]
    pub fn xoscst(&self) -> XOSCSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XOSCSTR { bits }
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline]
    pub fn state(&self) -> STATER {
        STATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - External Oscillator Failure"]
    #[inline]
    pub fn xoscfail(&mut self) -> _XOSCFAILW {
        _XOSCFAILW { w: self }
    }
    #[doc = "Bit 1 - External Oscillator Status"]
    #[inline]
    pub fn xoscst(&mut self) -> _XOSCSTW {
        _XOSCSTW { w: self }
    }
    #[doc = "Bits 2:3 - Tamper Module Status"]
    #[inline]
    pub fn state(&mut self) -> _STATEW {
        _STATEW { w: self }
    }
}
