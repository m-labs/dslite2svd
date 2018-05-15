#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RXTYPE5 {
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
pub struct TEPR {
    bits: u8,
}
impl TEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PROTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTOR {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISOC,
    #[doc = "Bulk"]
    BULK,
    #[doc = "Interrupt"]
    INT,
}
impl PROTOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PROTOR::CTRL => 0,
            PROTOR::ISOC => 1,
            PROTOR::BULK => 2,
            PROTOR::INT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PROTOR {
        match value {
            0 => PROTOR::CTRL,
            1 => PROTOR::ISOC,
            2 => PROTOR::BULK,
            3 => PROTOR::INT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline]
    pub fn is_ctrl(&self) -> bool {
        *self == PROTOR::CTRL
    }
    #[doc = "Checks if the value of the field is `ISOC`"]
    #[inline]
    pub fn is_isoc(&self) -> bool {
        *self == PROTOR::ISOC
    }
    #[doc = "Checks if the value of the field is `BULK`"]
    #[inline]
    pub fn is_bulk(&self) -> bool {
        *self == PROTOR::BULK
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline]
    pub fn is_int(&self) -> bool {
        *self == PROTOR::INT
    }
}
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Default"]
    DFLT,
    #[doc = "High"]
    HIGH,
    #[doc = "Full"]
    FULL,
    #[doc = "Low"]
    LOW,
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::DFLT => 0,
            SPEEDR::HIGH => 1,
            SPEEDR::FULL => 2,
            SPEEDR::LOW => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::DFLT,
            1 => SPEEDR::HIGH,
            2 => SPEEDR::FULL,
            3 => SPEEDR::LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DFLT`"]
    #[inline]
    pub fn is_dflt(&self) -> bool {
        *self == SPEEDR::DFLT
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SPEEDR::HIGH
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline]
    pub fn is_full(&self) -> bool {
        *self == SPEEDR::FULL
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SPEEDR::LOW
    }
}
#[doc = r" Proxy"]
pub struct _TEPW<'a> {
    w: &'a mut W,
}
impl<'a> _TEPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROTO`"]
pub enum PROTOW {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISOC,
    #[doc = "Bulk"]
    BULK,
    #[doc = "Interrupt"]
    INT,
}
impl PROTOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PROTOW::CTRL => 0,
            PROTOW::ISOC => 1,
            PROTOW::BULK => 2,
            PROTOW::INT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTOW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTOW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PROTOW::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn isoc(self) -> &'a mut W {
        self.variant(PROTOW::ISOC)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn bulk(self) -> &'a mut W {
        self.variant(PROTOW::BULK)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn int(self) -> &'a mut W {
        self.variant(PROTOW::INT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPEED`"]
pub enum SPEEDW {
    #[doc = "Default"]
    DFLT,
    #[doc = "High"]
    HIGH,
    #[doc = "Full"]
    FULL,
    #[doc = "Low"]
    LOW,
}
impl SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPEEDW::DFLT => 0,
            SPEEDW::HIGH => 1,
            SPEEDW::FULL => 2,
            SPEEDW::LOW => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPEEDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Default"]
    #[inline]
    pub fn dflt(self) -> &'a mut W {
        self.variant(SPEEDW::DFLT)
    }
    #[doc = "High"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SPEEDW::HIGH)
    }
    #[doc = "Full"]
    #[inline]
    pub fn full(self) -> &'a mut W {
        self.variant(SPEEDW::FULL)
    }
    #[doc = "Low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SPEEDW::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline]
    pub fn tep(&self) -> TEPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        };
        TEPR { bits }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline]
    pub fn proto(&self) -> PROTOR {
        PROTOR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) as u8
        })
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) as u8
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Target Endpoint Number"]
    #[inline]
    pub fn tep(&mut self) -> _TEPW {
        _TEPW { w: self }
    }
    #[doc = "Bits 4:5 - Protocol"]
    #[inline]
    pub fn proto(&mut self) -> _PROTOW {
        _PROTOW { w: self }
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
}
