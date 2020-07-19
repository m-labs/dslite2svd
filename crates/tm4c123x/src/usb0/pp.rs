#[doc = "Reader of register PP"]
pub type R = crate::R<u32, super::PP>;
#[doc = "Controller Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: The first-generation USB controller"]
    _0 = 0,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
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
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TYPE_A::_0
    }
}
#[doc = "Reader of field `PHY`"]
pub type PHY_R = crate::R<bool, bool>;
#[doc = "USB Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB_A {
    #[doc = "1: DEVICE"]
    DEVICE = 1,
    #[doc = "2: HOST"]
    HOSTDEVICE = 2,
    #[doc = "3: OTG"]
    OTG = 3,
}
impl From<USB_A> for u8 {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as _
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
