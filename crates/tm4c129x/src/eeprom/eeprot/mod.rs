#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEPROT {
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
#[doc = "Possible values of the field `PROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTR {
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    RWNPW,
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    RWPW,
    #[doc = "If there is no password, the block is readable, not writable"]
    RONPW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROTR::RWNPW => 0,
            PROTR::RWPW => 1,
            PROTR::RONPW => 2,
            PROTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROTR {
        match value {
            0 => PROTR::RWNPW,
            1 => PROTR::RWPW,
            2 => PROTR::RONPW,
            i => PROTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RWNPW`"]
    #[inline]
    pub fn is_rwnpw(&self) -> bool {
        *self == PROTR::RWNPW
    }
    #[doc = "Checks if the value of the field is `RWPW`"]
    #[inline]
    pub fn is_rwpw(&self) -> bool {
        *self == PROTR::RWPW
    }
    #[doc = "Checks if the value of the field is `RONPW`"]
    #[inline]
    pub fn is_ronpw(&self) -> bool {
        *self == PROTR::RONPW
    }
}
#[doc = r" Value of the field"]
pub struct ACCR {
    bits: bool,
}
impl ACCR {
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
#[doc = "Values that can be written to the field `PROT`"]
pub enum PROTW {
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    RWNPW,
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    RWPW,
    #[doc = "If there is no password, the block is readable, not writable"]
    RONPW,
}
impl PROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROTW::RWNPW => 0,
            PROTW::RWPW => 1,
            PROTW::RONPW => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "This setting is the default. If there is no password, the block is not protected and is readable and writable"]
    #[inline]
    pub fn rwnpw(self) -> &'a mut W {
        self.variant(PROTW::RWNPW)
    }
    #[doc = "If there is a password, the block is readable or writable only when unlocked"]
    #[inline]
    pub fn rwpw(self) -> &'a mut W {
        self.variant(PROTW::RWPW)
    }
    #[doc = "If there is no password, the block is readable, not writable"]
    #[inline]
    pub fn ronpw(self) -> &'a mut W {
        self.variant(PROTW::RONPW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACCW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCW<'a> {
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline]
    pub fn prot(&self) -> PROTR {
        PROTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline]
    pub fn acc(&self) -> ACCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACCR { bits }
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
    #[doc = "Bits 0:2 - Protection Control"]
    #[inline]
    pub fn prot(&mut self) -> _PROTW {
        _PROTW { w: self }
    }
    #[doc = "Bit 3 - Access Control"]
    #[inline]
    pub fn acc(&mut self) -> _ACCW {
        _ACCW { w: self }
    }
}
