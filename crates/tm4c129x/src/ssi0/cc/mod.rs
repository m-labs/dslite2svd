#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSR {
    #[doc = "System clock (based on clock source and divisor factor)"]
    SYSPLL,
    #[doc = "PIOSC"]
    PIOSC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSR::SYSPLL => 0,
            CSR::PIOSC => 5,
            CSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSR {
        match value {
            0 => CSR::SYSPLL,
            5 => CSR::PIOSC,
            i => CSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSPLL`"]
    #[inline]
    pub fn is_syspll(&self) -> bool {
        *self == CSR::SYSPLL
    }
    #[doc = "Checks if the value of the field is `PIOSC`"]
    #[inline]
    pub fn is_piosc(&self) -> bool {
        *self == CSR::PIOSC
    }
}
#[doc = "Values that can be written to the field `CS`"]
pub enum CSW {
    #[doc = "System clock (based on clock source and divisor factor)"]
    SYSPLL,
    #[doc = "PIOSC"]
    PIOSC,
}
impl CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSW::SYSPLL => 0,
            CSW::PIOSC => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System clock (based on clock source and divisor factor)"]
    #[inline]
    pub fn syspll(self) -> &'a mut W {
        self.variant(CSW::SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline]
    pub fn piosc(self) -> &'a mut W {
        self.variant(CSW::PIOSC)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline]
    pub fn cs(&self) -> CSR {
        CSR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - SSI Baud Clock Source"]
    #[inline]
    pub fn cs(&mut self) -> _CSW {
        _CSW { w: self }
    }
}
