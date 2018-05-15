#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GPCS {
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
#[doc = "Possible values of the field `DEVMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVMODR {
    #[doc = "Use USB0VBUS and USB0ID pin"]
    OTG,
    #[doc = "Force USB0VBUS and USB0ID low"]
    HOST,
    #[doc = "Force USB0VBUS and USB0ID high"]
    DEV,
    #[doc = "Use USB0VBUS and force USB0ID low"]
    HOSTVBUS,
    #[doc = "Use USB0VBUS and force USB0ID high"]
    DEVVBUS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEVMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEVMODR::OTG => 0,
            DEVMODR::HOST => 2,
            DEVMODR::DEV => 3,
            DEVMODR::HOSTVBUS => 4,
            DEVMODR::DEVVBUS => 5,
            DEVMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEVMODR {
        match value {
            0 => DEVMODR::OTG,
            2 => DEVMODR::HOST,
            3 => DEVMODR::DEV,
            4 => DEVMODR::HOSTVBUS,
            5 => DEVMODR::DEVVBUS,
            i => DEVMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline]
    pub fn is_otg(&self) -> bool {
        *self == DEVMODR::OTG
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == DEVMODR::HOST
    }
    #[doc = "Checks if the value of the field is `DEV`"]
    #[inline]
    pub fn is_dev(&self) -> bool {
        *self == DEVMODR::DEV
    }
    #[doc = "Checks if the value of the field is `HOSTVBUS`"]
    #[inline]
    pub fn is_hostvbus(&self) -> bool {
        *self == DEVMODR::HOSTVBUS
    }
    #[doc = "Checks if the value of the field is `DEVVBUS`"]
    #[inline]
    pub fn is_devvbus(&self) -> bool {
        *self == DEVMODR::DEVVBUS
    }
}
#[doc = "Values that can be written to the field `DEVMOD`"]
pub enum DEVMODW {
    #[doc = "Use USB0VBUS and USB0ID pin"]
    OTG,
    #[doc = "Force USB0VBUS and USB0ID low"]
    HOST,
    #[doc = "Force USB0VBUS and USB0ID high"]
    DEV,
    #[doc = "Use USB0VBUS and force USB0ID low"]
    HOSTVBUS,
    #[doc = "Use USB0VBUS and force USB0ID high"]
    DEVVBUS,
}
impl DEVMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DEVMODW::OTG => 0,
            DEVMODW::HOST => 2,
            DEVMODW::DEV => 3,
            DEVMODW::HOSTVBUS => 4,
            DEVMODW::DEVVBUS => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DEVMODW<'a> {
    w: &'a mut W,
}
impl<'a> _DEVMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEVMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use USB0VBUS and USB0ID pin"]
    #[inline]
    pub fn otg(self) -> &'a mut W {
        self.variant(DEVMODW::OTG)
    }
    #[doc = "Force USB0VBUS and USB0ID low"]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(DEVMODW::HOST)
    }
    #[doc = "Force USB0VBUS and USB0ID high"]
    #[inline]
    pub fn dev(self) -> &'a mut W {
        self.variant(DEVMODW::DEV)
    }
    #[doc = "Use USB0VBUS and force USB0ID low"]
    #[inline]
    pub fn hostvbus(self) -> &'a mut W {
        self.variant(DEVMODW::HOSTVBUS)
    }
    #[doc = "Use USB0VBUS and force USB0ID high"]
    #[inline]
    pub fn devvbus(self) -> &'a mut W {
        self.variant(DEVMODW::DEVVBUS)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline]
    pub fn devmod(&self) -> DEVMODR {
        DEVMODR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:2 - Device Mode"]
    #[inline]
    pub fn devmod(&mut self) -> _DEVMODW {
        _DEVMODW { w: self }
    }
}
