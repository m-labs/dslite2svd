#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSLPPWRCFG {
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
#[doc = "Possible values of the field `SRAMPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMPMR {
    #[doc = "Active Mode"]
    NRM,
    #[doc = "Standby Mode"]
    SBY,
    #[doc = "Low Power Mode"]
    LP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRAMPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMPMR::NRM => 0,
            SRAMPMR::SBY => 1,
            SRAMPMR::LP => 3,
            SRAMPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMPMR {
        match value {
            0 => SRAMPMR::NRM,
            1 => SRAMPMR::SBY,
            3 => SRAMPMR::LP,
            i => SRAMPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NRM`"]
    #[inline]
    pub fn is_nrm(&self) -> bool {
        *self == SRAMPMR::NRM
    }
    #[doc = "Checks if the value of the field is `SBY`"]
    #[inline]
    pub fn is_sby(&self) -> bool {
        *self == SRAMPMR::SBY
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline]
    pub fn is_lp(&self) -> bool {
        *self == SRAMPMR::LP
    }
}
#[doc = "Possible values of the field `FLASHPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHPMR {
    #[doc = "Active Mode"]
    NRM,
    #[doc = "Low Power Mode"]
    SLP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASHPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASHPMR::NRM => 0,
            FLASHPMR::SLP => 2,
            FLASHPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASHPMR {
        match value {
            0 => FLASHPMR::NRM,
            2 => FLASHPMR::SLP,
            i => FLASHPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NRM`"]
    #[inline]
    pub fn is_nrm(&self) -> bool {
        *self == FLASHPMR::NRM
    }
    #[doc = "Checks if the value of the field is `SLP`"]
    #[inline]
    pub fn is_slp(&self) -> bool {
        *self == FLASHPMR::SLP
    }
}
#[doc = "Values that can be written to the field `SRAMPM`"]
pub enum SRAMPMW {
    #[doc = "Active Mode"]
    NRM,
    #[doc = "Standby Mode"]
    SBY,
    #[doc = "Low Power Mode"]
    LP,
}
impl SRAMPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRAMPMW::NRM => 0,
            SRAMPMW::SBY => 1,
            SRAMPMW::LP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRAMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAMPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAMPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline]
    pub fn nrm(self) -> &'a mut W {
        self.variant(SRAMPMW::NRM)
    }
    #[doc = "Standby Mode"]
    #[inline]
    pub fn sby(self) -> &'a mut W {
        self.variant(SRAMPMW::SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline]
    pub fn lp(self) -> &'a mut W {
        self.variant(SRAMPMW::LP)
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
#[doc = "Values that can be written to the field `FLASHPM`"]
pub enum FLASHPMW {
    #[doc = "Active Mode"]
    NRM,
    #[doc = "Low Power Mode"]
    SLP,
}
impl FLASHPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASHPMW::NRM => 0,
            FLASHPMW::SLP => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASHPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASHPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASHPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline]
    pub fn nrm(self) -> &'a mut W {
        self.variant(FLASHPMW::NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline]
    pub fn slp(self) -> &'a mut W {
        self.variant(FLASHPMW::SLP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline]
    pub fn srampm(&self) -> SRAMPMR {
        SRAMPMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline]
    pub fn flashpm(&self) -> FLASHPMR {
        FLASHPMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline]
    pub fn srampm(&mut self) -> _SRAMPMW {
        _SRAMPMW { w: self }
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline]
    pub fn flashpm(&mut self) -> _FLASHPMW {
        _FLASHPMW { w: self }
    }
}
