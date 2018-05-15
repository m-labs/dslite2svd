#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPSCTRL {
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
pub struct PPSCTRLR {
    bits: u8,
}
impl PPSCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PPSEN0R {
    bits: bool,
}
impl PPSEN0R {
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
#[doc = "Possible values of the field `TRGMODS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGMODS0R {
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    INTONLY,
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    INTPPS0,
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    PPS0ONLY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGMODS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGMODS0R::INTONLY => 0,
            TRGMODS0R::INTPPS0 => 2,
            TRGMODS0R::PPS0ONLY => 3,
            TRGMODS0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGMODS0R {
        match value {
            0 => TRGMODS0R::INTONLY,
            2 => TRGMODS0R::INTPPS0,
            3 => TRGMODS0R::PPS0ONLY,
            i => TRGMODS0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INTONLY`"]
    #[inline]
    pub fn is_intonly(&self) -> bool {
        *self == TRGMODS0R::INTONLY
    }
    #[doc = "Checks if the value of the field is `INTPPS0`"]
    #[inline]
    pub fn is_intpps0(&self) -> bool {
        *self == TRGMODS0R::INTPPS0
    }
    #[doc = "Checks if the value of the field is `PPS0ONLY`"]
    #[inline]
    pub fn is_pps0only(&self) -> bool {
        *self == TRGMODS0R::PPS0ONLY
    }
}
#[doc = r" Proxy"]
pub struct _PPSCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _PPSCTRLW<'a> {
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
pub struct _PPSEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PPSEN0W<'a> {
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
#[doc = "Values that can be written to the field `TRGMODS0`"]
pub enum TRGMODS0W {
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    INTONLY,
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    INTPPS0,
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    PPS0ONLY,
}
impl TRGMODS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGMODS0W::INTONLY => 0,
            TRGMODS0W::INTPPS0 => 2,
            TRGMODS0W::PPS0ONLY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGMODS0W<'a> {
    w: &'a mut W,
}
impl<'a> _TRGMODS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGMODS0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    #[inline]
    pub fn intonly(self) -> &'a mut W {
        self.variant(TRGMODS0W::INTONLY)
    }
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    #[inline]
    pub fn intpps0(self) -> &'a mut W {
        self.variant(TRGMODS0W::INTPPS0)
    }
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    #[inline]
    pub fn pps0only(self) -> &'a mut W {
        self.variant(TRGMODS0W::PPS0ONLY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline]
    pub fn ppsctrl(&self) -> PPSCTRLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PPSCTRLR { bits }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline]
    pub fn ppsen0(&self) -> PPSEN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPSEN0R { bits }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline]
    pub fn trgmods0(&self) -> TRGMODS0R {
        TRGMODS0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline]
    pub fn ppsctrl(&mut self) -> _PPSCTRLW {
        _PPSCTRLW { w: self }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline]
    pub fn ppsen0(&mut self) -> _PPSEN0W {
        _PPSEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline]
    pub fn trgmods0(&mut self) -> _TRGMODS0W {
        _TRGMODS0W { w: self }
    }
}
