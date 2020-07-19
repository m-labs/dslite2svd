#[doc = "Reader of register DC6"]
pub type R = crate::R<u32, super::DC6>;
#[doc = "USB Module 0 Present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USB0_A {
    #[doc = "1: USB0 is Device Only"]
    DEV = 1,
    #[doc = "2: USB is Device or Host"]
    HOSTDEV = 2,
    #[doc = "3: USB0 is OTG"]
    OTG = 3,
}
impl From<USB0_A> for u8 {
    #[inline(always)]
    fn from(variant: USB0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USB0`"]
pub type USB0_R = crate::R<u8, USB0_A>;
impl USB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USB0_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(USB0_A::DEV),
            2 => Val(USB0_A::HOSTDEV),
            3 => Val(USB0_A::OTG),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEV`"]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == USB0_A::DEV
    }
    #[doc = "Checks if the value of the field is `HOSTDEV`"]
    #[inline(always)]
    pub fn is_hostdev(&self) -> bool {
        *self == USB0_A::HOSTDEV
    }
    #[doc = "Checks if the value of the field is `OTG`"]
    #[inline(always)]
    pub fn is_otg(&self) -> bool {
        *self == USB0_A::OTG
    }
}
#[doc = "Reader of field `USB0PHY`"]
pub type USB0PHY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - USB Module 0 Present"]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - USB Module 0 PHY Present"]
    #[inline(always)]
    pub fn usb0phy(&self) -> USB0PHY_R {
        USB0PHY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
