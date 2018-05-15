#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIIADDR {
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
pub struct MIIBR {
    bits: bool,
}
impl MIIBR {
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
pub struct MIIWR {
    bits: bool,
}
impl MIIWR {
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
#[doc = "Possible values of the field `CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRR {
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    _60_100,
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    _100_150,
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    _20_35,
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    _35_60,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRR::_60_100 => 0,
            CRR::_100_150 => 1,
            CRR::_20_35 => 2,
            CRR::_35_60 => 3,
            CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRR {
        match value {
            0 => CRR::_60_100,
            1 => CRR::_100_150,
            2 => CRR::_20_35,
            3 => CRR::_35_60,
            i => CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_60_100`"]
    #[inline]
    pub fn is_60_100(&self) -> bool {
        *self == CRR::_60_100
    }
    #[doc = "Checks if the value of the field is `_100_150`"]
    #[inline]
    pub fn is_100_150(&self) -> bool {
        *self == CRR::_100_150
    }
    #[doc = "Checks if the value of the field is `_20_35`"]
    #[inline]
    pub fn is_20_35(&self) -> bool {
        *self == CRR::_20_35
    }
    #[doc = "Checks if the value of the field is `_35_60`"]
    #[inline]
    pub fn is_35_60(&self) -> bool {
        *self == CRR::_35_60
    }
}
#[doc = r" Value of the field"]
pub struct MIIR {
    bits: u8,
}
impl MIIR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PLAR {
    bits: u8,
}
impl PLAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _MIIBW<'a> {
    w: &'a mut W,
}
impl<'a> _MIIBW<'a> {
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
pub struct _MIIWW<'a> {
    w: &'a mut W,
}
impl<'a> _MIIWW<'a> {
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
#[doc = "Values that can be written to the field `CR`"]
pub enum CRW {
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    _60_100,
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    _100_150,
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    _20_35,
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    _35_60,
}
impl CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRW::_60_100 => 0,
            CRW::_100_150 => 1,
            CRW::_20_35 => 2,
            CRW::_35_60 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRW<'a> {
    w: &'a mut W,
}
impl<'a> _CRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The frequency of the System Clock is 60 to 100 MHz providing a MDIO clock of SYSCLK/42"]
    #[inline]
    pub fn _60_100(self) -> &'a mut W {
        self.variant(CRW::_60_100)
    }
    #[doc = "The frequency of the System Clock is 100 to 150 MHz providing a MDIO clock of SYSCLK/62"]
    #[inline]
    pub fn _100_150(self) -> &'a mut W {
        self.variant(CRW::_100_150)
    }
    #[doc = "The frequency of the System Clock is 20-35 MHz providing a MDIO clock of System Clock/16"]
    #[inline]
    pub fn _20_35(self) -> &'a mut W {
        self.variant(CRW::_20_35)
    }
    #[doc = "The frequency of the System Clock is 35 to 60 MHz providing a MDIO clock of System Clock/26"]
    #[inline]
    pub fn _35_60(self) -> &'a mut W {
        self.variant(CRW::_35_60)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MIIW<'a> {
    w: &'a mut W,
}
impl<'a> _MIIW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLAW<'a> {
    w: &'a mut W,
}
impl<'a> _PLAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bit 0 - MII Busy"]
    #[inline]
    pub fn miib(&self) -> MIIBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIIBR { bits }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline]
    pub fn miiw(&self) -> MIIWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MIIWR { bits }
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline]
    pub fn cr(&self) -> CRR {
        CRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline]
    pub fn mii(&self) -> MIIR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MIIR { bits }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline]
    pub fn pla(&self) -> PLAR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLAR { bits }
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
    #[doc = "Bit 0 - MII Busy"]
    #[inline]
    pub fn miib(&mut self) -> _MIIBW {
        _MIIBW { w: self }
    }
    #[doc = "Bit 1 - MII Write"]
    #[inline]
    pub fn miiw(&mut self) -> _MIIWW {
        _MIIWW { w: self }
    }
    #[doc = "Bits 2:5 - Clock Reference Frequency Selection"]
    #[inline]
    pub fn cr(&mut self) -> _CRW {
        _CRW { w: self }
    }
    #[doc = "Bits 6:10 - MII Register"]
    #[inline]
    pub fn mii(&mut self) -> _MIIW {
        _MIIW { w: self }
    }
    #[doc = "Bits 11:15 - Physical Layer Address"]
    #[inline]
    pub fn pla(&mut self) -> _PLAW {
        _PLAW { w: self }
    }
}
