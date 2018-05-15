#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMACMPC {
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
#[doc = "Possible values of the field `PWRCTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRCTLR {
    #[doc = "Array OFF"]
    OFF,
    #[doc = "Array On"]
    ON,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWRCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWRCTLR::OFF => 0,
            PWRCTLR::ON => 3,
            PWRCTLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWRCTLR {
        match value {
            0 => PWRCTLR::OFF,
            3 => PWRCTLR::ON,
            i => PWRCTLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == PWRCTLR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == PWRCTLR::ON
    }
}
#[doc = "Values that can be written to the field `PWRCTL`"]
pub enum PWRCTLW {
    #[doc = "Array OFF"]
    OFF,
    #[doc = "Array On"]
    ON,
}
impl PWRCTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWRCTLW::OFF => 0,
            PWRCTLW::ON => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWRCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _PWRCTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWRCTLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Array OFF"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(PWRCTLW::OFF)
    }
    #[doc = "Array On"]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(PWRCTLW::ON)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline]
    pub fn pwrctl(&self) -> PWRCTLR {
        PWRCTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - Memory Array Power Control"]
    #[inline]
    pub fn pwrctl(&mut self) -> _PWRCTLW {
        _PWRCTLW { w: self }
    }
}
