#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PP {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "The first-generation USB controller"]
    _0,
    #[doc = "Second-generation USB controller.The controller implemented in post Icestorm devices that use the 3.0 version of the Mentor controller"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPER::_0 => 0,
            TYPER::_1 => 1,
            TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPER {
        match value {
            0 => TYPER::_0,
            1 => TYPER::_1,
            i => TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TYPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TYPER::_1
    }
}
#[doc = r" Value of the field"]
pub struct PHYR {
    bits: bool,
}
impl PHYR {
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
pub struct ULPIR {
    bits: bool,
}
impl ULPIR {
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
#[doc = "Possible values of the field `USB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBR {
    #[doc = "DEVICE"]
    DEVICE,
    #[doc = "HOST"]
    HOSTDEVICE,
    #[doc = "OTG"]
    OTG,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USBR::DEVICE => 1,
            USBR::HOSTDEVICE => 2,
            USBR::OTG => 3,
            USBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USBR {
        match value {
            1 => USBR::DEVICE,
            2 => USBR::HOSTDEVICE,
            3 => USBR::OTG,
            i => USBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline]
    pub fn is_device(&self) -> bool {
        *self == USBR::DEVICE
    }
    #[doc = "Checks if the value of the field is `HOSTDEVICE`"]
    #[inline]
    pub fn is_hostdevice(&self) -> bool {
        *self == USBR::HOSTDEVICE
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline]
    pub fn is_otg(&self) -> bool {
        *self == USBR::OTG
    }
}
#[doc = r" Value of the field"]
pub struct ECNTR {
    bits: u8,
}
impl ECNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline]
    pub fn phy(&self) -> PHYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHYR { bits }
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline]
    pub fn ulpi(&self) -> ULPIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ULPIR { bits }
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline]
    pub fn usb(&self) -> USBR {
        USBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline]
    pub fn ecnt(&self) -> ECNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECNTR { bits }
    }
}
