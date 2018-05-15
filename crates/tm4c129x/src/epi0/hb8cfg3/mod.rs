#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB8CFG3 {
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
    #[doc = "ADMUX - AD[7:0]"]
    ADMUX,
    #[doc = "ADNONMUX - D[7:0]"]
    AD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::ADMUX => 0,
            MODER::AD => 1,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::ADMUX,
            1 => MODER::AD,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADMUX`"]
    #[inline]
    pub fn is_admux(&self) -> bool {
        *self == MODER::ADMUX
    }
    #[doc = "Checks if the value of the field is `AD`"]
    #[inline]
    pub fn is_ad(&self) -> bool {
        *self == MODER::AD
    }
}
#[doc = "Possible values of the field `RDWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDWSR {
    #[doc = "Active RDn is 2 EPI clocks"]
    _2,
    #[doc = "Active RDn is 4 EPI clocks"]
    _4,
    #[doc = "Active RDn is 6 EPI clocks"]
    _6,
    #[doc = "Active RDn is 8 EPI clocks"]
    _8,
}
impl RDWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RDWSR::_2 => 0,
            RDWSR::_4 => 1,
            RDWSR::_6 => 2,
            RDWSR::_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RDWSR {
        match value {
            0 => RDWSR::_2,
            1 => RDWSR::_4,
            2 => RDWSR::_6,
            3 => RDWSR::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == RDWSR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == RDWSR::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == RDWSR::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == RDWSR::_8
    }
}
#[doc = "Possible values of the field `WRWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRWSR {
    #[doc = "Active WRn is 2 EPI clocks"]
    _2,
    #[doc = "Active WRn is 4 EPI clocks"]
    _4,
    #[doc = "Active WRn is 6 EPI clocks"]
    _6,
    #[doc = "Active WRn is 8 EPI clocks"]
    _8,
}
impl WRWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRWSR::_2 => 0,
            WRWSR::_4 => 1,
            WRWSR::_6 => 2,
            WRWSR::_8 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRWSR {
        match value {
            0 => WRWSR::_2,
            1 => WRWSR::_4,
            2 => WRWSR::_6,
            3 => WRWSR::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == WRWSR::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == WRWSR::_4
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline]
    pub fn is_6(&self) -> bool {
        *self == WRWSR::_6
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == WRWSR::_8
    }
}
#[doc = r" Value of the field"]
pub struct ALEHIGHR {
    bits: bool,
}
impl ALEHIGHR {
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
pub struct RDHIGHR {
    bits: bool,
}
impl RDHIGHR {
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
pub struct WRHIGHR {
    bits: bool,
}
impl WRHIGHR {
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
    #[doc = "ADMUX - AD[7:0]"]
    ADMUX,
    #[doc = "ADNONMUX - D[7:0]"]
    AD,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::ADMUX => 0,
            MODEW::AD => 1,
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
    #[doc = "ADMUX - AD[7:0]"]
    #[inline]
    pub fn admux(self) -> &'a mut W {
        self.variant(MODEW::ADMUX)
    }
    #[doc = "ADNONMUX - D[7:0]"]
    #[inline]
    pub fn ad(self) -> &'a mut W {
        self.variant(MODEW::AD)
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
#[doc = "Values that can be written to the field `RDWS`"]
pub enum RDWSW {
    #[doc = "Active RDn is 2 EPI clocks"]
    _2,
    #[doc = "Active RDn is 4 EPI clocks"]
    _4,
    #[doc = "Active RDn is 6 EPI clocks"]
    _6,
    #[doc = "Active RDn is 8 EPI clocks"]
    _8,
}
impl RDWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RDWSW::_2 => 0,
            RDWSW::_4 => 1,
            RDWSW::_6 => 2,
            RDWSW::_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDWSW<'a> {
    w: &'a mut W,
}
impl<'a> _RDWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active RDn is 2 EPI clocks"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(RDWSW::_2)
    }
    #[doc = "Active RDn is 4 EPI clocks"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(RDWSW::_4)
    }
    #[doc = "Active RDn is 6 EPI clocks"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(RDWSW::_6)
    }
    #[doc = "Active RDn is 8 EPI clocks"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(RDWSW::_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRWS`"]
pub enum WRWSW {
    #[doc = "Active WRn is 2 EPI clocks"]
    _2,
    #[doc = "Active WRn is 4 EPI clocks"]
    _4,
    #[doc = "Active WRn is 6 EPI clocks"]
    _6,
    #[doc = "Active WRn is 8 EPI clocks"]
    _8,
}
impl WRWSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRWSW::_2 => 0,
            WRWSW::_4 => 1,
            WRWSW::_6 => 2,
            WRWSW::_8 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRWSW<'a> {
    w: &'a mut W,
}
impl<'a> _WRWSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRWSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Active WRn is 2 EPI clocks"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(WRWSW::_2)
    }
    #[doc = "Active WRn is 4 EPI clocks"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(WRWSW::_4)
    }
    #[doc = "Active WRn is 6 EPI clocks"]
    #[inline]
    pub fn _6(self) -> &'a mut W {
        self.variant(WRWSW::_6)
    }
    #[doc = "Active WRn is 8 EPI clocks"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(WRWSW::_8)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALEHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _ALEHIGHW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _RDHIGHW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _WRHIGHW<'a> {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline]
    pub fn rdws(&self) -> RDWSR {
        RDWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline]
    pub fn wrws(&self) -> WRWSR {
        WRWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline]
    pub fn alehigh(&self) -> ALEHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline]
    pub fn rdhigh(&self) -> RDHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDHIGHR { bits }
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline]
    pub fn wrhigh(&self) -> WRHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRHIGHR { bits }
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
    #[doc = "Bits 0:1 - CS2n Host Bus Sub-Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:5 - CS2n Read Wait States"]
    #[inline]
    pub fn rdws(&mut self) -> _RDWSW {
        _RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - CS2n Write Wait States"]
    #[inline]
    pub fn wrws(&mut self) -> _WRWSW {
        _WRWSW { w: self }
    }
    #[doc = "Bit 19 - CS2n ALE Strobe Polarity"]
    #[inline]
    pub fn alehigh(&mut self) -> _ALEHIGHW {
        _ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - CS2n READ Strobe Polarity"]
    #[inline]
    pub fn rdhigh(&mut self) -> _RDHIGHW {
        _RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - CS2n WRITE Strobe Polarity"]
    #[inline]
    pub fn wrhigh(&mut self) -> _WRHIGHW {
        _WRHIGHW { w: self }
    }
}
