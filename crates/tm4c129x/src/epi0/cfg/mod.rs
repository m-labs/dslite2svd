#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "General Purpose"]
    NONE,
    #[doc = "SDRAM"]
    SDRAM,
    #[doc = "8-Bit Host-Bus (HB8)"]
    HB8,
    #[doc = "16-Bit Host-Bus (HB16)"]
    HB16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NONE => 0,
            MODER::SDRAM => 1,
            MODER::HB8 => 2,
            MODER::HB16 => 3,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NONE,
            1 => MODER::SDRAM,
            2 => MODER::HB8,
            3 => MODER::HB16,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == MODER::NONE
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline]
    pub fn is_sdram(&self) -> bool {
        *self == MODER::SDRAM
    }
    #[doc = "Checks if the value of the field is `HB8`"]
    #[inline]
    pub fn is_hb8(&self) -> bool {
        *self == MODER::HB8
    }
    #[doc = "Checks if the value of the field is `HB16`"]
    #[inline]
    pub fn is_hb16(&self) -> bool {
        *self == MODER::HB16
    }
}
#[doc = r" Value of the field"]
pub struct BLKENR {
    bits: bool,
}
impl BLKENR {
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
pub struct INTDIVR {
    bits: bool,
}
impl INTDIVR {
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "General Purpose"]
    NONE,
    #[doc = "SDRAM"]
    SDRAM,
    #[doc = "8-Bit Host-Bus (HB8)"]
    HB8,
    #[doc = "16-Bit Host-Bus (HB16)"]
    HB16,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NONE => 0,
            MODEW::SDRAM => 1,
            MODEW::HB8 => 2,
            MODEW::HB16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "General Purpose"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(MODEW::NONE)
    }
    #[doc = "SDRAM"]
    #[inline]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MODEW::SDRAM)
    }
    #[doc = "8-Bit Host-Bus (HB8)"]
    #[inline]
    pub fn hb8(self) -> &'a mut W {
        self.variant(MODEW::HB8)
    }
    #[doc = "16-Bit Host-Bus (HB16)"]
    #[inline]
    pub fn hb16(self) -> &'a mut W {
        self.variant(MODEW::HB16)
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
#[doc = r" Proxy"]
pub struct _BLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKENW<'a> {
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
pub struct _INTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _INTDIVW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline]
    pub fn blken(&self) -> BLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLKENR { bits }
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline]
    pub fn intdiv(&self) -> INTDIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INTDIVR { bits }
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
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline]
    pub fn blken(&mut self) -> _BLKENW {
        _BLKENW { w: self }
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline]
    pub fn intdiv(&mut self) -> _INTDIVW {
        _INTDIVW { w: self }
    }
}
