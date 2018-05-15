#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSLPCLKCFG {
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
pub struct PIOSCPDR {
    bits: bool,
}
impl PIOSCPDR {
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
#[doc = "Possible values of the field `O`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OR {
    #[doc = "MOSC"]
    IGN,
    #[doc = "PIOSC"]
    IO,
    #[doc = "LFIOSC"]
    _30,
    #[doc = "32.768 kHz"]
    _32,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OR::IGN => 0,
            OR::IO => 1,
            OR::_30 => 3,
            OR::_32 => 7,
            OR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OR {
        match value {
            0 => OR::IGN,
            1 => OR::IO,
            3 => OR::_30,
            7 => OR::_32,
            i => OR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IGN`"]
    #[inline]
    pub fn is_ign(&self) -> bool {
        *self == OR::IGN
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline]
    pub fn is_io(&self) -> bool {
        *self == OR::IO
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline]
    pub fn is_30(&self) -> bool {
        *self == OR::_30
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline]
    pub fn is_32(&self) -> bool {
        *self == OR::_32
    }
}
#[doc = r" Value of the field"]
pub struct DR {
    bits: u8,
}
impl DR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PIOSCPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIOSCPDW<'a> {
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
#[doc = "Values that can be written to the field `O`"]
pub enum OW {
    #[doc = "MOSC"]
    IGN,
    #[doc = "PIOSC"]
    IO,
    #[doc = "LFIOSC"]
    _30,
    #[doc = "32.768 kHz"]
    _32,
}
impl OW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OW::IGN => 0,
            OW::IO => 1,
            OW::_30 => 3,
            OW::_32 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OW<'a> {
    w: &'a mut W,
}
impl<'a> _OW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MOSC"]
    #[inline]
    pub fn ign(self) -> &'a mut W {
        self.variant(OW::IGN)
    }
    #[doc = "PIOSC"]
    #[inline]
    pub fn io(self) -> &'a mut W {
        self.variant(OW::IO)
    }
    #[doc = "LFIOSC"]
    #[inline]
    pub fn _30(self) -> &'a mut W {
        self.variant(OW::_30)
    }
    #[doc = "32.768 kHz"]
    #[inline]
    pub fn _32(self) -> &'a mut W {
        self.variant(OW::_32)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DW<'a> {
    w: &'a mut W,
}
impl<'a> _DW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 1 - PIOSC Power Down Request"]
    #[inline]
    pub fn pioscpd(&self) -> PIOSCPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PIOSCPDR { bits }
    }
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline]
    pub fn o(&self) -> OR {
        OR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline]
    pub fn d(&self) -> DR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DR { bits }
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
    #[doc = "Bit 1 - PIOSC Power Down Request"]
    #[inline]
    pub fn pioscpd(&mut self) -> _PIOSCPDW {
        _PIOSCPDW { w: self }
    }
    #[doc = "Bits 4:6 - Clock Source"]
    #[inline]
    pub fn o(&mut self) -> _OW {
        _OW { w: self }
    }
    #[doc = "Bits 23:28 - Divider Field Override"]
    #[inline]
    pub fn d(&mut self) -> _DW {
        _DW { w: self }
    }
}
