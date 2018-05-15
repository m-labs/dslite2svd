#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::DEVCTL {
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
pub struct SESSIONR {
    bits: bool,
}
impl SESSIONR {
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
pub struct HOSTREQR {
    bits: bool,
}
impl HOSTREQR {
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
pub struct HOSTR {
    bits: bool,
}
impl HOSTR {
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
#[doc = "Possible values of the field `VBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VBUSR {
    #[doc = "Below SessionEnd"]
    NONE,
    #[doc = "Above SessionEnd, below AValid"]
    SEND,
    #[doc = "Above AValid, below VBUSValid"]
    AVALID,
    #[doc = "Above VBUSValid"]
    VALID,
}
impl VBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VBUSR::NONE => 0,
            VBUSR::SEND => 1,
            VBUSR::AVALID => 2,
            VBUSR::VALID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VBUSR {
        match value {
            0 => VBUSR::NONE,
            1 => VBUSR::SEND,
            2 => VBUSR::AVALID,
            3 => VBUSR::VALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VBUSR::NONE
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline]
    pub fn is_send(&self) -> bool {
        *self == VBUSR::SEND
    }
    #[doc = "Checks if the value of the field is `AVALID`"]
    #[inline]
    pub fn is_avalid(&self) -> bool {
        *self == VBUSR::AVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self == VBUSR::VALID
    }
}
#[doc = r" Value of the field"]
pub struct LSDEVR {
    bits: bool,
}
impl LSDEVR {
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
pub struct FSDEVR {
    bits: bool,
}
impl FSDEVR {
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
pub struct DEVR {
    bits: bool,
}
impl DEVR {
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
#[doc = r" Proxy"]
pub struct _SESSIONW<'a> {
    w: &'a mut W,
}
impl<'a> _SESSIONW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOSTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTREQW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HOSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VBUS`"]
pub enum VBUSW {
    #[doc = "Below SessionEnd"]
    NONE,
    #[doc = "Above SessionEnd, below AValid"]
    SEND,
    #[doc = "Above AValid, below VBUSValid"]
    AVALID,
    #[doc = "Above VBUSValid"]
    VALID,
}
impl VBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VBUSW::NONE => 0,
            VBUSW::SEND => 1,
            VBUSW::AVALID => 2,
            VBUSW::VALID => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VBUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Below SessionEnd"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(VBUSW::NONE)
    }
    #[doc = "Above SessionEnd, below AValid"]
    #[inline]
    pub fn send(self) -> &'a mut W {
        self.variant(VBUSW::SEND)
    }
    #[doc = "Above AValid, below VBUSValid"]
    #[inline]
    pub fn avalid(self) -> &'a mut W {
        self.variant(VBUSW::AVALID)
    }
    #[doc = "Above VBUSValid"]
    #[inline]
    pub fn valid(self) -> &'a mut W {
        self.variant(VBUSW::VALID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LSDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _LSDEVW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSDEVW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDEVW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEVW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline]
    pub fn session(&self) -> SESSIONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        SESSIONR { bits }
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline]
    pub fn hostreq(&self) -> HOSTREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        HOSTREQR { bits }
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline]
    pub fn host(&self) -> HOSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        HOSTR { bits }
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline]
    pub fn vbus(&self) -> VBUSR {
        VBUSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline]
    pub fn lsdev(&self) -> LSDEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        LSDEVR { bits }
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline]
    pub fn fsdev(&self) -> FSDEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        FSDEVR { bits }
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline]
    pub fn dev(&self) -> DEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        DEVR { bits }
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Session Start/End (OTG only)"]
    #[inline]
    pub fn session(&mut self) -> _SESSIONW {
        _SESSIONW { w: self }
    }
    #[doc = "Bit 1 - Host Request (OTG only)"]
    #[inline]
    pub fn hostreq(&mut self) -> _HOSTREQW {
        _HOSTREQW { w: self }
    }
    #[doc = "Bit 2 - Host Mode"]
    #[inline]
    pub fn host(&mut self) -> _HOSTW {
        _HOSTW { w: self }
    }
    #[doc = "Bits 3:4 - VBUS Level (OTG only)"]
    #[inline]
    pub fn vbus(&mut self) -> _VBUSW {
        _VBUSW { w: self }
    }
    #[doc = "Bit 5 - Low-Speed Device Detected"]
    #[inline]
    pub fn lsdev(&mut self) -> _LSDEVW {
        _LSDEVW { w: self }
    }
    #[doc = "Bit 6 - Full-Speed Device Detected"]
    #[inline]
    pub fn fsdev(&mut self) -> _FSDEVW {
        _FSDEVW { w: self }
    }
    #[doc = "Bit 7 - Device Mode (OTG only)"]
    #[inline]
    pub fn dev(&mut self) -> _DEVW {
        _DEVW { w: self }
    }
}
