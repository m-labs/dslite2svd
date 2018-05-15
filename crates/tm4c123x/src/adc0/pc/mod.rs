#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
#[doc = "Possible values of the field `SR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR {
    #[doc = "125 ksps"]
    _125K,
    #[doc = "250 ksps"]
    _250K,
    #[doc = "500 ksps"]
    _500K,
    #[doc = "1 Msps"]
    _1M,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRR::_125K => 1,
            SRR::_250K => 3,
            SRR::_500K => 5,
            SRR::_1M => 7,
            SRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRR {
        match value {
            1 => SRR::_125K,
            3 => SRR::_250K,
            5 => SRR::_500K,
            7 => SRR::_1M,
            i => SRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_125K`"]
    #[inline]
    pub fn is_125k(&self) -> bool {
        *self == SRR::_125K
    }
    #[doc = "Checks if the value of the field is `_250K`"]
    #[inline]
    pub fn is_250k(&self) -> bool {
        *self == SRR::_250K
    }
    #[doc = "Checks if the value of the field is `_500K`"]
    #[inline]
    pub fn is_500k(&self) -> bool {
        *self == SRR::_500K
    }
    #[doc = "Checks if the value of the field is `_1M`"]
    #[inline]
    pub fn is_1m(&self) -> bool {
        *self == SRR::_1M
    }
}
#[doc = "Values that can be written to the field `SR`"]
pub enum SRW {
    #[doc = "125 ksps"]
    _125K,
    #[doc = "250 ksps"]
    _250K,
    #[doc = "500 ksps"]
    _500K,
    #[doc = "1 Msps"]
    _1M,
}
impl SRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRW::_125K => 1,
            SRW::_250K => 3,
            SRW::_500K => 5,
            SRW::_1M => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "125 ksps"]
    #[inline]
    pub fn _125k(self) -> &'a mut W {
        self.variant(SRW::_125K)
    }
    #[doc = "250 ksps"]
    #[inline]
    pub fn _250k(self) -> &'a mut W {
        self.variant(SRW::_250K)
    }
    #[doc = "500 ksps"]
    #[inline]
    pub fn _500k(self) -> &'a mut W {
        self.variant(SRW::_500K)
    }
    #[doc = "1 Msps"]
    #[inline]
    pub fn _1m(self) -> &'a mut W {
        self.variant(SRW::_1M)
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
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline]
    pub fn sr(&self) -> SRR {
        SRR::_from({
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
    #[doc = "Bits 0:3 - ADC Sample Rate"]
    #[inline]
    pub fn sr(&mut self) -> _SRW {
        _SRW { w: self }
    }
}
