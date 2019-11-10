#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Controller Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE_A {
    #[doc = "0: The first-generation USB controller"]
    _0,
    #[doc = "1: Second-generation USB controller.The controller implemented in post Icestorm devices that use the 3.0 version of the Mentor controller"]
    _1,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        match variant {
            TYPE_A::_0 => 0,
            TYPE_A::_1 => 1,
        }
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::R<u8, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPE_A::_0),
            1 => Val(TYPE_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TYPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TYPE_A::_1
    }
}
#[doc = "Reader of field `PHY`"]
pub type PHY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ULPI`"]
pub type ULPI_R = crate::R<bool, bool>;
#[doc = "USB Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "1: DEVICE"]
    DEVICE,
    #[doc = "2: HOST"]
    HOSTDEVICE,
    #[doc = "3: OTG"]
    OTG,
}
impl From<USB_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        match variant {
            USB_A::DEVICE => 1,
            USB_A::HOSTDEVICE => 2,
            USB_A::OTG => 3,
        }
    }
}
#[doc = "Reader of field `USB`"]
pub type USB_R = crate::R<u8, USB_A>;
impl USB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USB_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(USB_A::DEVICE),
            2 => Val(USB_A::HOSTDEVICE),
            3 => Val(USB_A::OTG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == USB_A::DEVICE
    }
    #[doc = "Checks if the value of the field is `HOSTDEVICE`"]
    #[inline(always)]
    pub fn is_hostdevice(&self) -> bool {
        *self == USB_A::HOSTDEVICE
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline(always)]
    pub fn is_otg(&self) -> bool {
        *self == USB_A::OTG
    }
}
#[doc = "Reader of field `ECNT`"]
pub type ECNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Controller Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - PHY Present"]
    #[inline(always)]
    pub fn phy(&self) -> PHY_R {
        PHY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ULPI Present"]
    #[inline(always)]
    pub fn ulpi(&self) -> ULPI_R {
        ULPI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - USB Capability"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Endpoint Count"]
    #[inline(always)]
    pub fn ecnt(&self) -> ECNT_R {
        ECNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
