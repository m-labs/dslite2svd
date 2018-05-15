#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPCTL {
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
pub struct TPENR {
    bits: bool,
}
impl TPENR {
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
pub struct TPCLRR {
    bits: bool,
}
impl TPCLRR {
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
#[doc = "Possible values of the field `MEMCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMCLRR {
    #[doc = "Do not Clear HIB memory on tamper event"]
    NONE,
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    LOW32,
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    HIGH32,
    #[doc = "Clear all HIB memory on tamper event"]
    ALL,
}
impl MEMCLRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MEMCLRR::NONE => 0,
            MEMCLRR::LOW32 => 1,
            MEMCLRR::HIGH32 => 2,
            MEMCLRR::ALL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MEMCLRR {
        match value {
            0 => MEMCLRR::NONE,
            1 => MEMCLRR::LOW32,
            2 => MEMCLRR::HIGH32,
            3 => MEMCLRR::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == MEMCLRR::NONE
    }
    #[doc = "Checks if the value of the field is `LOW32`"]
    #[inline]
    pub fn is_low32(&self) -> bool {
        *self == MEMCLRR::LOW32
    }
    #[doc = "Checks if the value of the field is `HIGH32`"]
    #[inline]
    pub fn is_high32(&self) -> bool {
        *self == MEMCLRR::HIGH32
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == MEMCLRR::ALL
    }
}
#[doc = r" Value of the field"]
pub struct WAKER {
    bits: bool,
}
impl WAKER {
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
pub struct _TPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TPENW<'a> {
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
pub struct _TPCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _TPCLRW<'a> {
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
#[doc = "Values that can be written to the field `MEMCLR`"]
pub enum MEMCLRW {
    #[doc = "Do not Clear HIB memory on tamper event"]
    NONE,
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    LOW32,
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    HIGH32,
    #[doc = "Clear all HIB memory on tamper event"]
    ALL,
}
impl MEMCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MEMCLRW::NONE => 0,
            MEMCLRW::LOW32 => 1,
            MEMCLRW::HIGH32 => 2,
            MEMCLRW::ALL => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMCLRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not Clear HIB memory on tamper event"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(MEMCLRW::NONE)
    }
    #[doc = "Clear Lower 32 Bytes of HIB memory on tamper event"]
    #[inline]
    pub fn low32(self) -> &'a mut W {
        self.variant(MEMCLRW::LOW32)
    }
    #[doc = "Clear upper 32 Bytes of HIB memory on tamper event"]
    #[inline]
    pub fn high32(self) -> &'a mut W {
        self.variant(MEMCLRW::HIGH32)
    }
    #[doc = "Clear all HIB memory on tamper event"]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(MEMCLRW::ALL)
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
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
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
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline]
    pub fn tpen(&self) -> TPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPENR { bits }
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline]
    pub fn tpclr(&self) -> TPCLRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TPCLRR { bits }
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline]
    pub fn memclr(&self) -> MEMCLRR {
        MEMCLRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAKER { bits }
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
    #[doc = "Bit 0 - Tamper Module Enable"]
    #[inline]
    pub fn tpen(&mut self) -> _TPENW {
        _TPENW { w: self }
    }
    #[doc = "Bit 4 - Tamper Event Clear"]
    #[inline]
    pub fn tpclr(&mut self) -> _TPCLRW {
        _TPCLRW { w: self }
    }
    #[doc = "Bits 8:9 - HIB Memory Clear on Tamper Event"]
    #[inline]
    pub fn memclr(&mut self) -> _MEMCLRW {
        _MEMCLRW { w: self }
    }
    #[doc = "Bit 11 - Wake from Hibernate on a Tamper Event"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
}
