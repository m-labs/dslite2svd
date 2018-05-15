#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSSEL {
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
#[doc = "Possible values of the field `PS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS0R {
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PS0R::_0 => 0,
            PS0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PS0R {
        match value {
            0 => PS0R::_0,
            i => PS0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PS0R::_0
    }
}
#[doc = "Possible values of the field `PS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS1R {
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PS1R::_0 => 0,
            PS1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PS1R {
        match value {
            0 => PS1R::_0,
            i => PS1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PS1R::_0
    }
}
#[doc = "Possible values of the field `PS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS2R {
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PS2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PS2R::_0 => 0,
            PS2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PS2R {
        match value {
            0 => PS2R::_0,
            i => PS2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PS2R::_0
    }
}
#[doc = "Possible values of the field `PS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS3R {
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    _0,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PS3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PS3R::_0 => 0,
            PS3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PS3R {
        match value {
            0 => PS3R::_0,
            i => PS3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PS3R::_0
    }
}
#[doc = "Values that can be written to the field `PS0`"]
pub enum PS0W {
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    _0,
}
impl PS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PS0W::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PS0W<'a> {
    w: &'a mut W,
}
impl<'a> _PS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PS0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS0W::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PS1`"]
pub enum PS1W {
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    _0,
}
impl PS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PS1W::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PS1W<'a> {
    w: &'a mut W,
}
impl<'a> _PS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PS1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS1W::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PS2`"]
pub enum PS2W {
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    _0,
}
impl PS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PS2W::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PS2W<'a> {
    w: &'a mut W,
}
impl<'a> _PS2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PS2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS2W::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PS3`"]
pub enum PS3W {
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    _0,
}
impl PS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PS3W::_0 => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PS3W<'a> {
    w: &'a mut W,
}
impl<'a> _PS3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PS3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS3W::_0)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline]
    pub fn ps0(&self) -> PS0R {
        PS0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline]
    pub fn ps1(&self) -> PS1R {
        PS1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline]
    pub fn ps2(&self) -> PS2R {
        PS2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline]
    pub fn ps3(&self) -> PS3R {
        PS3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline]
    pub fn ps0(&mut self) -> _PS0W {
        _PS0W { w: self }
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline]
    pub fn ps1(&mut self) -> _PS1W {
        _PS1W { w: self }
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline]
    pub fn ps2(&mut self) -> _PS2W {
        _PS2W { w: self }
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline]
    pub fn ps3(&mut self) -> _PS3W {
        _PS3W { w: self }
    }
}
