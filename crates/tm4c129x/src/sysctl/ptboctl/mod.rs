#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PTBOCTL {
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
#[doc = "Possible values of the field `VDD_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDD_UBORR {
    #[doc = "No Action"]
    NONE,
    #[doc = "System control interrupt"]
    SYSINT,
    #[doc = "NMI"]
    NMI,
    #[doc = "Reset"]
    RST,
}
impl VDD_UBORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDD_UBORR::NONE => 0,
            VDD_UBORR::SYSINT => 1,
            VDD_UBORR::NMI => 2,
            VDD_UBORR::RST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDD_UBORR {
        match value {
            0 => VDD_UBORR::NONE,
            1 => VDD_UBORR::SYSINT,
            2 => VDD_UBORR::NMI,
            3 => VDD_UBORR::RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VDD_UBORR::NONE
    }
    #[doc = "Checks if the value of the field is `SYSINT`"]
    #[inline]
    pub fn is_sysint(&self) -> bool {
        *self == VDD_UBORR::SYSINT
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline]
    pub fn is_nmi(&self) -> bool {
        *self == VDD_UBORR::NMI
    }
    #[doc = "Checks if the value of the field is `RST`"]
    #[inline]
    pub fn is_rst(&self) -> bool {
        *self == VDD_UBORR::RST
    }
}
#[doc = "Possible values of the field `VDDA_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_UBORR {
    #[doc = "No Action"]
    NONE,
    #[doc = "System control interrupt"]
    SYSINT,
    #[doc = "NMI"]
    NMI,
    #[doc = "Reset"]
    RST,
}
impl VDDA_UBORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VDDA_UBORR::NONE => 0,
            VDDA_UBORR::SYSINT => 1,
            VDDA_UBORR::NMI => 2,
            VDDA_UBORR::RST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VDDA_UBORR {
        match value {
            0 => VDDA_UBORR::NONE,
            1 => VDDA_UBORR::SYSINT,
            2 => VDDA_UBORR::NMI,
            3 => VDDA_UBORR::RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == VDDA_UBORR::NONE
    }
    #[doc = "Checks if the value of the field is `SYSINT`"]
    #[inline]
    pub fn is_sysint(&self) -> bool {
        *self == VDDA_UBORR::SYSINT
    }
    #[doc = "Checks if the value of the field is `NMI`"]
    #[inline]
    pub fn is_nmi(&self) -> bool {
        *self == VDDA_UBORR::NMI
    }
    #[doc = "Checks if the value of the field is `RST`"]
    #[inline]
    pub fn is_rst(&self) -> bool {
        *self == VDDA_UBORR::RST
    }
}
#[doc = "Values that can be written to the field `VDD_UBOR`"]
pub enum VDD_UBORW {
    #[doc = "No Action"]
    NONE,
    #[doc = "System control interrupt"]
    SYSINT,
    #[doc = "NMI"]
    NMI,
    #[doc = "Reset"]
    RST,
}
impl VDD_UBORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDD_UBORW::NONE => 0,
            VDD_UBORW::SYSINT => 1,
            VDD_UBORW::NMI => 2,
            VDD_UBORW::RST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDD_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _VDD_UBORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDD_UBORW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(VDD_UBORW::NONE)
    }
    #[doc = "System control interrupt"]
    #[inline]
    pub fn sysint(self) -> &'a mut W {
        self.variant(VDD_UBORW::SYSINT)
    }
    #[doc = "NMI"]
    #[inline]
    pub fn nmi(self) -> &'a mut W {
        self.variant(VDD_UBORW::NMI)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn rst(self) -> &'a mut W {
        self.variant(VDD_UBORW::RST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VDDA_UBOR`"]
pub enum VDDA_UBORW {
    #[doc = "No Action"]
    NONE,
    #[doc = "System control interrupt"]
    SYSINT,
    #[doc = "NMI"]
    NMI,
    #[doc = "Reset"]
    RST,
}
impl VDDA_UBORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            VDDA_UBORW::NONE => 0,
            VDDA_UBORW::SYSINT => 1,
            VDDA_UBORW::NMI => 2,
            VDDA_UBORW::RST => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDDA_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDA_UBORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDA_UBORW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(VDDA_UBORW::NONE)
    }
    #[doc = "System control interrupt"]
    #[inline]
    pub fn sysint(self) -> &'a mut W {
        self.variant(VDDA_UBORW::SYSINT)
    }
    #[doc = "NMI"]
    #[inline]
    pub fn nmi(self) -> &'a mut W {
        self.variant(VDDA_UBORW::NMI)
    }
    #[doc = "Reset"]
    #[inline]
    pub fn rst(self) -> &'a mut W {
        self.variant(VDDA_UBORW::RST)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline]
    pub fn vdd_ubor(&self) -> VDD_UBORR {
        VDD_UBORR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline]
    pub fn vdda_ubor(&self) -> VDDA_UBORR {
        VDDA_UBORR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline]
    pub fn vdd_ubor(&mut self) -> _VDD_UBORW {
        _VDD_UBORW { w: self }
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline]
    pub fn vdda_ubor(&mut self) -> _VDDA_UBORW {
        _VDDA_UBORW { w: self }
    }
}
