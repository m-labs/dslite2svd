#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VLNINCREP {
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
pub struct VLTR {
    bits: u16,
}
impl VLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `VLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLCR {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NONE,
    #[doc = "VLAN tag deletion"]
    TAGDEL,
    #[doc = "VLAN tag insertion"]
    TAGINS,
    #[doc = "VLAN tag replacement"]
    TAGREP,
}
impl VLCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VLCR::NONE => 0,
            VLCR::TAGDEL => 1,
            VLCR::TAGINS => 2,
            VLCR::TAGREP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VLCR {
        match value {
            0 => VLCR::NONE,
            1 => VLCR::TAGDEL,
            2 => VLCR::TAGINS,
            3 => VLCR::TAGREP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VLCR::NONE
    }
    #[doc = "Checks if the value of the field is `TAGDEL`"]
    #[inline]
    pub fn is_tagdel(&self) -> bool {
        *self == VLCR::TAGDEL
    }
    #[doc = "Checks if the value of the field is `TAGINS`"]
    #[inline]
    pub fn is_tagins(&self) -> bool {
        *self == VLCR::TAGINS
    }
    #[doc = "Checks if the value of the field is `TAGREP`"]
    #[inline]
    pub fn is_tagrep(&self) -> bool {
        *self == VLCR::TAGREP
    }
}
#[doc = r" Value of the field"]
pub struct VLPR {
    bits: bool,
}
impl VLPR {
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
pub struct CSVLR {
    bits: bool,
}
impl CSVLR {
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
pub struct _VLTW<'a> {
    w: &'a mut W,
}
impl<'a> _VLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VLC`"]
pub enum VLCW {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    NONE,
    #[doc = "VLAN tag deletion"]
    TAGDEL,
    #[doc = "VLAN tag insertion"]
    TAGINS,
    #[doc = "VLAN tag replacement"]
    TAGREP,
}
impl VLCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VLCW::NONE => 0,
            VLCW::TAGDEL => 1,
            VLCW::TAGINS => 2,
            VLCW::TAGREP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VLCW<'a> {
    w: &'a mut W,
}
impl<'a> _VLCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VLCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(VLCW::NONE)
    }
    #[doc = "VLAN tag deletion"]
    #[inline]
    pub fn tagdel(self) -> &'a mut W {
        self.variant(VLCW::TAGDEL)
    }
    #[doc = "VLAN tag insertion"]
    #[inline]
    pub fn tagins(self) -> &'a mut W {
        self.variant(VLCW::TAGINS)
    }
    #[doc = "VLAN tag replacement"]
    #[inline]
    pub fn tagrep(self) -> &'a mut W {
        self.variant(VLCW::TAGREP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VLPW<'a> {
    w: &'a mut W,
}
impl<'a> _VLPW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSVLW<'a> {
    w: &'a mut W,
}
impl<'a> _CSVLW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline]
    pub fn vlt(&self) -> VLTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        VLTR { bits }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline]
    pub fn vlc(&self) -> VLCR {
        VLCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline]
    pub fn vlp(&self) -> VLPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VLPR { bits }
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline]
    pub fn csvl(&self) -> CSVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSVLR { bits }
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
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline]
    pub fn vlt(&mut self) -> _VLTW {
        _VLTW { w: self }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline]
    pub fn vlc(&mut self) -> _VLCW {
        _VLCW { w: self }
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline]
    pub fn vlp(&mut self) -> _VLPW {
        _VLPW { w: self }
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline]
    pub fn csvl(&mut self) -> _CSVLW {
        _CSVLW { w: self }
    }
}
