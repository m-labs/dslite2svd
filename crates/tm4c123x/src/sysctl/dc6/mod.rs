#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DC6 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `USB0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB0R {
    #[doc = "USB0 is Device Only"]
    DEV,
    #[doc = "USB is Device or Host"]
    HOSTDEV,
    #[doc = "USB0 is OTG"]
    OTG,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USB0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USB0R::DEV => 1,
            USB0R::HOSTDEV => 2,
            USB0R::OTG => 3,
            USB0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USB0R {
        match value {
            1 => USB0R::DEV,
            2 => USB0R::HOSTDEV,
            3 => USB0R::OTG,
            i => USB0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEV`"]
    #[inline]
    pub fn is_dev(&self) -> bool {
        *self == USB0R::DEV
    }
    #[doc = "Checks if the value of the field is `HOSTDEV`"]
    #[inline]
    pub fn is_hostdev(&self) -> bool {
        *self == USB0R::HOSTDEV
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline]
    pub fn is_otg(&self) -> bool {
        *self == USB0R::OTG
    }
}
#[doc = r" Value of the field"]
pub struct USB0PHYR {
    bits: bool,
}
impl USB0PHYR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - USB Module 0 Present"]
    #[inline]
    pub fn usb0(&self) -> USB0R {
        USB0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - USB Module 0 PHY Present"]
    #[inline]
    pub fn usb0phy(&self) -> USB0PHYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USB0PHYR { bits }
    }
}
