#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EPC {
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
#[doc = "Possible values of the field `EPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPENR {
    #[doc = "Power Enable Active Low"]
    LOW,
    #[doc = "Power Enable Active High"]
    HIGH,
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    VBLOW,
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    VBHIGH,
}
impl EPENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPENR::LOW => 0,
            EPENR::HIGH => 1,
            EPENR::VBLOW => 2,
            EPENR::VBHIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPENR {
        match value {
            0 => EPENR::LOW,
            1 => EPENR::HIGH,
            2 => EPENR::VBLOW,
            3 => EPENR::VBHIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == EPENR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == EPENR::HIGH
    }
    #[doc = "Checks if the value of the field is `VBLOW`"]
    #[inline]
    pub fn is_vblow(&self) -> bool {
        *self == EPENR::VBLOW
    }
    #[doc = "Checks if the value of the field is `VBHIGH`"]
    #[inline]
    pub fn is_vbhigh(&self) -> bool {
        *self == EPENR::VBHIGH
    }
}
#[doc = r" Value of the field"]
pub struct EPENDER {
    bits: bool,
}
impl EPENDER {
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
pub struct PFLTENR {
    bits: bool,
}
impl PFLTENR {
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
pub struct PFLTSEN_HIGHR {
    bits: bool,
}
impl PFLTSEN_HIGHR {
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
pub struct PFLTAENR {
    bits: bool,
}
impl PFLTAENR {
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
#[doc = "Possible values of the field `PFLTACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFLTACTR {
    #[doc = "Unchanged"]
    UNCHG,
    #[doc = "Tristate"]
    TRIS,
    #[doc = "Low"]
    LOW,
    #[doc = "High"]
    HIGH,
}
impl PFLTACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PFLTACTR::UNCHG => 0,
            PFLTACTR::TRIS => 1,
            PFLTACTR::LOW => 2,
            PFLTACTR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PFLTACTR {
        match value {
            0 => PFLTACTR::UNCHG,
            1 => PFLTACTR::TRIS,
            2 => PFLTACTR::LOW,
            3 => PFLTACTR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UNCHG`"]
    #[inline]
    pub fn is_unchg(&self) -> bool {
        *self == PFLTACTR::UNCHG
    }
    #[doc = "Checks if the value of the field is `TRIS`"]
    #[inline]
    pub fn is_tris(&self) -> bool {
        *self == PFLTACTR::TRIS
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PFLTACTR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PFLTACTR::HIGH
    }
}
#[doc = "Values that can be written to the field `EPEN`"]
pub enum EPENW {
    #[doc = "Power Enable Active Low"]
    LOW,
    #[doc = "Power Enable Active High"]
    HIGH,
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    VBLOW,
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    VBHIGH,
}
impl EPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPENW::LOW => 0,
            EPENW::HIGH => 1,
            EPENW::VBLOW => 2,
            EPENW::VBHIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Power Enable Active Low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(EPENW::LOW)
    }
    #[doc = "Power Enable Active High"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(EPENW::HIGH)
    }
    #[doc = "Power Enable High if VBUS Low (OTG only)"]
    #[inline]
    pub fn vblow(self) -> &'a mut W {
        self.variant(EPENW::VBLOW)
    }
    #[doc = "Power Enable High if VBUS High (OTG only)"]
    #[inline]
    pub fn vbhigh(self) -> &'a mut W {
        self.variant(EPENW::VBHIGH)
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
#[doc = r" Proxy"]
pub struct _EPENDEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPENDEW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFLTENW<'a> {
    w: &'a mut W,
}
impl<'a> _PFLTENW<'a> {
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
pub struct _PFLTSEN_HIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _PFLTSEN_HIGHW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PFLTAENW<'a> {
    w: &'a mut W,
}
impl<'a> _PFLTAENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PFLTACT`"]
pub enum PFLTACTW {
    #[doc = "Unchanged"]
    UNCHG,
    #[doc = "Tristate"]
    TRIS,
    #[doc = "Low"]
    LOW,
    #[doc = "High"]
    HIGH,
}
impl PFLTACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PFLTACTW::UNCHG => 0,
            PFLTACTW::TRIS => 1,
            PFLTACTW::LOW => 2,
            PFLTACTW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFLTACTW<'a> {
    w: &'a mut W,
}
impl<'a> _PFLTACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFLTACTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Unchanged"]
    #[inline]
    pub fn unchg(self) -> &'a mut W {
        self.variant(PFLTACTW::UNCHG)
    }
    #[doc = "Tristate"]
    #[inline]
    pub fn tris(self) -> &'a mut W {
        self.variant(PFLTACTW::TRIS)
    }
    #[doc = "Low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PFLTACTW::LOW)
    }
    #[doc = "High"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PFLTACTW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline]
    pub fn epen(&self) -> EPENR {
        EPENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline]
    pub fn epende(&self) -> EPENDER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPENDER { bits }
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline]
    pub fn pflten(&self) -> PFLTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFLTENR { bits }
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline]
    pub fn pfltsen_high(&self) -> PFLTSEN_HIGHR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFLTSEN_HIGHR { bits }
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline]
    pub fn pfltaen(&self) -> PFLTAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PFLTAENR { bits }
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline]
    pub fn pfltact(&self) -> PFLTACTR {
        PFLTACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - External Power Supply Enable Configuration"]
    #[inline]
    pub fn epen(&mut self) -> _EPENW {
        _EPENW { w: self }
    }
    #[doc = "Bit 2 - EPEN Drive Enable"]
    #[inline]
    pub fn epende(&mut self) -> _EPENDEW {
        _EPENDEW { w: self }
    }
    #[doc = "Bit 4 - Power Fault Input Enable"]
    #[inline]
    pub fn pflten(&mut self) -> _PFLTENW {
        _PFLTENW { w: self }
    }
    #[doc = "Bit 5 - Power Fault Sense"]
    #[inline]
    pub fn pfltsen_high(&mut self) -> _PFLTSEN_HIGHW {
        _PFLTSEN_HIGHW { w: self }
    }
    #[doc = "Bit 6 - Power Fault Action Enable"]
    #[inline]
    pub fn pfltaen(&mut self) -> _PFLTAENW {
        _PFLTAENW { w: self }
    }
    #[doc = "Bits 8:9 - Power Fault Action"]
    #[inline]
    pub fn pfltact(&mut self) -> _PFLTACTW {
        _PFLTACTW { w: self }
    }
}
