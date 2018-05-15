#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESBEHAVCTL {
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
#[doc = "Possible values of the field `EXTRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRESR {
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTRESR::SYSRST => 2,
            EXTRESR::POR => 3,
            EXTRESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTRESR {
        match value {
            2 => EXTRESR::SYSRST,
            3 => EXTRESR::POR,
            i => EXTRESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline]
    pub fn is_sysrst(&self) -> bool {
        *self == EXTRESR::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == EXTRESR::POR
    }
}
#[doc = "Possible values of the field `BOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BORR {
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    POR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BORR::SYSRST => 2,
            BORR::POR => 3,
            BORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BORR {
        match value {
            2 => BORR::SYSRST,
            3 => BORR::POR,
            i => BORR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline]
    pub fn is_sysrst(&self) -> bool {
        *self == BORR::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == BORR::POR
    }
}
#[doc = "Possible values of the field `WDOG0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG0R {
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDOG0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDOG0R::SYSRST => 2,
            WDOG0R::POR => 3,
            WDOG0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDOG0R {
        match value {
            2 => WDOG0R::SYSRST,
            3 => WDOG0R::POR,
            i => WDOG0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline]
    pub fn is_sysrst(&self) -> bool {
        *self == WDOG0R::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == WDOG0R::POR
    }
}
#[doc = "Possible values of the field `WDOG1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOG1R {
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDOG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDOG1R::SYSRST => 2,
            WDOG1R::POR => 3,
            WDOG1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDOG1R {
        match value {
            2 => WDOG1R::SYSRST,
            3 => WDOG1R::POR,
            i => WDOG1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSRST`"]
    #[inline]
    pub fn is_sysrst(&self) -> bool {
        *self == WDOG1R::SYSRST
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline]
    pub fn is_por(&self) -> bool {
        *self == WDOG1R::POR
    }
}
#[doc = "Values that can be written to the field `EXTRES`"]
pub enum EXTRESW {
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl EXTRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTRESW::SYSRST => 2,
            EXTRESW::POR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTRESW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "External RST assertion issues a system reset. The application starts within 10 us"]
    #[inline]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(EXTRESW::SYSRST)
    }
    #[doc = "External RST assertion issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline]
    pub fn por(self) -> &'a mut W {
        self.variant(EXTRESW::POR)
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
#[doc = "Values that can be written to the field `BOR`"]
pub enum BORW {
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl BORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BORW::SYSRST => 2,
            BORW::POR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BORW<'a> {
    w: &'a mut W,
}
impl<'a> _BORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BORW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Brown Out Reset issues system reset. The application starts within 10 us"]
    #[inline]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(BORW::SYSRST)
    }
    #[doc = "Brown Out Reset issues a simulated POR sequence. The application starts less than 500 us after deassertion (Default)"]
    #[inline]
    pub fn por(self) -> &'a mut W {
        self.variant(BORW::POR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WDOG0`"]
pub enum WDOG0W {
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl WDOG0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDOG0W::SYSRST => 2,
            WDOG0W::POR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG0W<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Watchdog 0 issues a system reset. The application starts within 10 us"]
    #[inline]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(WDOG0W::SYSRST)
    }
    #[doc = "Watchdog 0 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline]
    pub fn por(self) -> &'a mut W {
        self.variant(WDOG0W::POR)
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
#[doc = "Values that can be written to the field `WDOG1`"]
pub enum WDOG1W {
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    SYSRST,
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    POR,
}
impl WDOG1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WDOG1W::SYSRST => 2,
            WDOG1W::POR => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOG1W<'a> {
    w: &'a mut W,
}
impl<'a> _WDOG1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOG1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Watchdog 1 issues a system reset. The application starts within 10 us"]
    #[inline]
    pub fn sysrst(self) -> &'a mut W {
        self.variant(WDOG1W::SYSRST)
    }
    #[doc = "Watchdog 1 issues a simulated POR sequence. Application starts less than 500 us after deassertion (Default)"]
    #[inline]
    pub fn por(self) -> &'a mut W {
        self.variant(WDOG1W::POR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline]
    pub fn extres(&self) -> EXTRESR {
        EXTRESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline]
    pub fn bor(&self) -> BORR {
        BORR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline]
    pub fn wdog0(&self) -> WDOG0R {
        WDOG0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline]
    pub fn wdog1(&self) -> WDOG1R {
        WDOG1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - External RST Pin Operation"]
    #[inline]
    pub fn extres(&mut self) -> _EXTRESW {
        _EXTRESW { w: self }
    }
    #[doc = "Bits 2:3 - BOR Reset operation"]
    #[inline]
    pub fn bor(&mut self) -> _BORW {
        _BORW { w: self }
    }
    #[doc = "Bits 4:5 - Watchdog 0 Reset Operation"]
    #[inline]
    pub fn wdog0(&mut self) -> _WDOG0W {
        _WDOG0W { w: self }
    }
    #[doc = "Bits 6:7 - Watchdog 1 Reset Operation"]
    #[inline]
    pub fn wdog1(&mut self) -> _WDOG1W {
        _WDOG1W { w: self }
    }
}
