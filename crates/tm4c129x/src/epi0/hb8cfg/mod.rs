#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB8CFG {
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
    MUX,
    #[doc = "ADNONMUX - D[7:0]"]
    NMUX,
    #[doc = "Continuous Read - D[7:0]"]
    SRAM,
    #[doc = "XFIFO - D[7:0]"]
    FIFO,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::MUX => 0,
            MODER::NMUX => 1,
            MODER::SRAM => 2,
            MODER::FIFO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::MUX,
            1 => MODER::NMUX,
            2 => MODER::SRAM,
            3 => MODER::FIFO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline]
    pub fn is_mux(&self) -> bool {
        *self == MODER::MUX
    }
    #[doc = "Checks if the value of the field is `NMUX`"]
    #[inline]
    pub fn is_nmux(&self) -> bool {
        *self == MODER::NMUX
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline]
    pub fn is_sram(&self) -> bool {
        *self == MODER::SRAM
    }
    #[doc = "Checks if the value of the field is `FIFO`"]
    #[inline]
    pub fn is_fifo(&self) -> bool {
        *self == MODER::FIFO
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
pub struct MAXWAITR {
    bits: u8,
}
impl MAXWAITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = r" Value of the field"]
pub struct XFEENR {
    bits: bool,
}
impl XFEENR {
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
pub struct XFFENR {
    bits: bool,
}
impl XFFENR {
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
pub struct IRDYINVR {
    bits: bool,
}
impl IRDYINVR {
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
pub struct RDYENR {
    bits: bool,
}
impl RDYENR {
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
pub struct CLKINVR {
    bits: bool,
}
impl CLKINVR {
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
pub struct CLKGATEIR {
    bits: bool,
}
impl CLKGATEIR {
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
pub struct CLKGATER {
    bits: bool,
}
impl CLKGATER {
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
    MUX,
    #[doc = "ADNONMUX - D[7:0]"]
    NMUX,
    #[doc = "Continuous Read - D[7:0]"]
    SRAM,
    #[doc = "XFIFO - D[7:0]"]
    FIFO,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::MUX => 0,
            MODEW::NMUX => 1,
            MODEW::SRAM => 2,
            MODEW::FIFO => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADMUX - AD[7:0]"]
    #[inline]
    pub fn mux(self) -> &'a mut W {
        self.variant(MODEW::MUX)
    }
    #[doc = "ADNONMUX - D[7:0]"]
    #[inline]
    pub fn nmux(self) -> &'a mut W {
        self.variant(MODEW::NMUX)
    }
    #[doc = "Continuous Read - D[7:0]"]
    #[inline]
    pub fn sram(self) -> &'a mut W {
        self.variant(MODEW::SRAM)
    }
    #[doc = "XFIFO - D[7:0]"]
    #[inline]
    pub fn fifo(self) -> &'a mut W {
        self.variant(MODEW::FIFO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
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
pub struct _MAXWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXWAITW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
#[doc = r" Proxy"]
pub struct _XFEENW<'a> {
    w: &'a mut W,
}
impl<'a> _XFEENW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XFFENW<'a> {
    w: &'a mut W,
}
impl<'a> _XFFENW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRDYINVW<'a> {
    w: &'a mut W,
}
impl<'a> _IRDYINVW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _RDYENW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKINVW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEIW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKGATEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKGATEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline]
    pub fn rdws(&self) -> RDWSR {
        RDWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline]
    pub fn wrws(&self) -> WRWSR {
        WRWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline]
    pub fn maxwait(&self) -> MAXWAITR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXWAITR { bits }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline]
    pub fn alehigh(&self) -> ALEHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALEHIGHR { bits }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline]
    pub fn rdhigh(&self) -> RDHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDHIGHR { bits }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline]
    pub fn wrhigh(&self) -> WRHIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRHIGHR { bits }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline]
    pub fn xfeen(&self) -> XFEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XFEENR { bits }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline]
    pub fn xffen(&self) -> XFFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        XFFENR { bits }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline]
    pub fn irdyinv(&self) -> IRDYINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRDYINVR { bits }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline]
    pub fn rdyen(&self) -> RDYENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDYENR { bits }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline]
    pub fn clkinv(&self) -> CLKINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKINVR { bits }
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline]
    pub fn clkgatei(&self) -> CLKGATEIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATEIR { bits }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline]
    pub fn clkgate(&self) -> CLKGATER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKGATER { bits }
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
    #[doc = "Bits 0:1 - Host Bus Sub-Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Read Wait States"]
    #[inline]
    pub fn rdws(&mut self) -> _RDWSW {
        _RDWSW { w: self }
    }
    #[doc = "Bits 6:7 - Write Wait States"]
    #[inline]
    pub fn wrws(&mut self) -> _WRWSW {
        _WRWSW { w: self }
    }
    #[doc = "Bits 8:15 - Maximum Wait"]
    #[inline]
    pub fn maxwait(&mut self) -> _MAXWAITW {
        _MAXWAITW { w: self }
    }
    #[doc = "Bit 19 - ALE Strobe Polarity"]
    #[inline]
    pub fn alehigh(&mut self) -> _ALEHIGHW {
        _ALEHIGHW { w: self }
    }
    #[doc = "Bit 20 - READ Strobe Polarity"]
    #[inline]
    pub fn rdhigh(&mut self) -> _RDHIGHW {
        _RDHIGHW { w: self }
    }
    #[doc = "Bit 21 - WRITE Strobe Polarity"]
    #[inline]
    pub fn wrhigh(&mut self) -> _WRHIGHW {
        _WRHIGHW { w: self }
    }
    #[doc = "Bit 22 - External FIFO EMPTY Enable"]
    #[inline]
    pub fn xfeen(&mut self) -> _XFEENW {
        _XFEENW { w: self }
    }
    #[doc = "Bit 23 - External FIFO FULL Enable"]
    #[inline]
    pub fn xffen(&mut self) -> _XFFENW {
        _XFFENW { w: self }
    }
    #[doc = "Bit 27 - Input Ready Invert"]
    #[inline]
    pub fn irdyinv(&mut self) -> _IRDYINVW {
        _IRDYINVW { w: self }
    }
    #[doc = "Bit 28 - Input Ready Enable"]
    #[inline]
    pub fn rdyen(&mut self) -> _RDYENW {
        _RDYENW { w: self }
    }
    #[doc = "Bit 29 - Invert Output Clock Enable"]
    #[inline]
    pub fn clkinv(&mut self) -> _CLKINVW {
        _CLKINVW { w: self }
    }
    #[doc = "Bit 30 - Clock Gated when Idle"]
    #[inline]
    pub fn clkgatei(&mut self) -> _CLKGATEIW {
        _CLKGATEIW { w: self }
    }
    #[doc = "Bit 31 - Clock Gated"]
    #[inline]
    pub fn clkgate(&mut self) -> _CLKGATEW {
        _CLKGATEW { w: self }
    }
}
