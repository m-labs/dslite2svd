#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPC {
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
#[doc = "Possible values of the field `PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASER {
    #[doc = "ADC sample lags by 0.0"]
    _0,
    #[doc = "ADC sample lags by 22.5"]
    _22_5,
    #[doc = "ADC sample lags by 45.0"]
    _45,
    #[doc = "ADC sample lags by 67.5"]
    _67_5,
    #[doc = "ADC sample lags by 90.0"]
    _90,
    #[doc = "ADC sample lags by 112.5"]
    _112_5,
    #[doc = "ADC sample lags by 135.0"]
    _135,
    #[doc = "ADC sample lags by 157.5"]
    _157_5,
    #[doc = "ADC sample lags by 180.0"]
    _180,
    #[doc = "ADC sample lags by 202.5"]
    _202_5,
    #[doc = "ADC sample lags by 225.0"]
    _225,
    #[doc = "ADC sample lags by 247.5"]
    _247_5,
    #[doc = "ADC sample lags by 270.0"]
    _270,
    #[doc = "ADC sample lags by 292.5"]
    _292_5,
    #[doc = "ADC sample lags by 315.0"]
    _315,
    #[doc = "ADC sample lags by 337.5"]
    _337_5,
}
impl PHASER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PHASER::_0 => 0,
            PHASER::_22_5 => 1,
            PHASER::_45 => 2,
            PHASER::_67_5 => 3,
            PHASER::_90 => 4,
            PHASER::_112_5 => 5,
            PHASER::_135 => 6,
            PHASER::_157_5 => 7,
            PHASER::_180 => 8,
            PHASER::_202_5 => 9,
            PHASER::_225 => 10,
            PHASER::_247_5 => 11,
            PHASER::_270 => 12,
            PHASER::_292_5 => 13,
            PHASER::_315 => 14,
            PHASER::_337_5 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PHASER {
        match value {
            0 => PHASER::_0,
            1 => PHASER::_22_5,
            2 => PHASER::_45,
            3 => PHASER::_67_5,
            4 => PHASER::_90,
            5 => PHASER::_112_5,
            6 => PHASER::_135,
            7 => PHASER::_157_5,
            8 => PHASER::_180,
            9 => PHASER::_202_5,
            10 => PHASER::_225,
            11 => PHASER::_247_5,
            12 => PHASER::_270,
            13 => PHASER::_292_5,
            14 => PHASER::_315,
            15 => PHASER::_337_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PHASER::_0
    }
    #[doc = "Checks if the value of the field is `_22_5`"]
    #[inline]
    pub fn is_22_5(&self) -> bool {
        *self == PHASER::_22_5
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline]
    pub fn is_45(&self) -> bool {
        *self == PHASER::_45
    }
    #[doc = "Checks if the value of the field is `_67_5`"]
    #[inline]
    pub fn is_67_5(&self) -> bool {
        *self == PHASER::_67_5
    }
    #[doc = "Checks if the value of the field is `_90`"]
    #[inline]
    pub fn is_90(&self) -> bool {
        *self == PHASER::_90
    }
    #[doc = "Checks if the value of the field is `_112_5`"]
    #[inline]
    pub fn is_112_5(&self) -> bool {
        *self == PHASER::_112_5
    }
    #[doc = "Checks if the value of the field is `_135`"]
    #[inline]
    pub fn is_135(&self) -> bool {
        *self == PHASER::_135
    }
    #[doc = "Checks if the value of the field is `_157_5`"]
    #[inline]
    pub fn is_157_5(&self) -> bool {
        *self == PHASER::_157_5
    }
    #[doc = "Checks if the value of the field is `_180`"]
    #[inline]
    pub fn is_180(&self) -> bool {
        *self == PHASER::_180
    }
    #[doc = "Checks if the value of the field is `_202_5`"]
    #[inline]
    pub fn is_202_5(&self) -> bool {
        *self == PHASER::_202_5
    }
    #[doc = "Checks if the value of the field is `_225`"]
    #[inline]
    pub fn is_225(&self) -> bool {
        *self == PHASER::_225
    }
    #[doc = "Checks if the value of the field is `_247_5`"]
    #[inline]
    pub fn is_247_5(&self) -> bool {
        *self == PHASER::_247_5
    }
    #[doc = "Checks if the value of the field is `_270`"]
    #[inline]
    pub fn is_270(&self) -> bool {
        *self == PHASER::_270
    }
    #[doc = "Checks if the value of the field is `_292_5`"]
    #[inline]
    pub fn is_292_5(&self) -> bool {
        *self == PHASER::_292_5
    }
    #[doc = "Checks if the value of the field is `_315`"]
    #[inline]
    pub fn is_315(&self) -> bool {
        *self == PHASER::_315
    }
    #[doc = "Checks if the value of the field is `_337_5`"]
    #[inline]
    pub fn is_337_5(&self) -> bool {
        *self == PHASER::_337_5
    }
}
#[doc = "Values that can be written to the field `PHASE`"]
pub enum PHASEW {
    #[doc = "ADC sample lags by 0.0"]
    _0,
    #[doc = "ADC sample lags by 22.5"]
    _22_5,
    #[doc = "ADC sample lags by 45.0"]
    _45,
    #[doc = "ADC sample lags by 67.5"]
    _67_5,
    #[doc = "ADC sample lags by 90.0"]
    _90,
    #[doc = "ADC sample lags by 112.5"]
    _112_5,
    #[doc = "ADC sample lags by 135.0"]
    _135,
    #[doc = "ADC sample lags by 157.5"]
    _157_5,
    #[doc = "ADC sample lags by 180.0"]
    _180,
    #[doc = "ADC sample lags by 202.5"]
    _202_5,
    #[doc = "ADC sample lags by 225.0"]
    _225,
    #[doc = "ADC sample lags by 247.5"]
    _247_5,
    #[doc = "ADC sample lags by 270.0"]
    _270,
    #[doc = "ADC sample lags by 292.5"]
    _292_5,
    #[doc = "ADC sample lags by 315.0"]
    _315,
    #[doc = "ADC sample lags by 337.5"]
    _337_5,
}
impl PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PHASEW::_0 => 0,
            PHASEW::_22_5 => 1,
            PHASEW::_45 => 2,
            PHASEW::_67_5 => 3,
            PHASEW::_90 => 4,
            PHASEW::_112_5 => 5,
            PHASEW::_135 => 6,
            PHASEW::_157_5 => 7,
            PHASEW::_180 => 8,
            PHASEW::_202_5 => 9,
            PHASEW::_225 => 10,
            PHASEW::_247_5 => 11,
            PHASEW::_270 => 12,
            PHASEW::_292_5 => 13,
            PHASEW::_315 => 14,
            PHASEW::_337_5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PHASEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC sample lags by 0.0"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHASEW::_0)
    }
    #[doc = "ADC sample lags by 22.5"]
    #[inline]
    pub fn _22_5(self) -> &'a mut W {
        self.variant(PHASEW::_22_5)
    }
    #[doc = "ADC sample lags by 45.0"]
    #[inline]
    pub fn _45(self) -> &'a mut W {
        self.variant(PHASEW::_45)
    }
    #[doc = "ADC sample lags by 67.5"]
    #[inline]
    pub fn _67_5(self) -> &'a mut W {
        self.variant(PHASEW::_67_5)
    }
    #[doc = "ADC sample lags by 90.0"]
    #[inline]
    pub fn _90(self) -> &'a mut W {
        self.variant(PHASEW::_90)
    }
    #[doc = "ADC sample lags by 112.5"]
    #[inline]
    pub fn _112_5(self) -> &'a mut W {
        self.variant(PHASEW::_112_5)
    }
    #[doc = "ADC sample lags by 135.0"]
    #[inline]
    pub fn _135(self) -> &'a mut W {
        self.variant(PHASEW::_135)
    }
    #[doc = "ADC sample lags by 157.5"]
    #[inline]
    pub fn _157_5(self) -> &'a mut W {
        self.variant(PHASEW::_157_5)
    }
    #[doc = "ADC sample lags by 180.0"]
    #[inline]
    pub fn _180(self) -> &'a mut W {
        self.variant(PHASEW::_180)
    }
    #[doc = "ADC sample lags by 202.5"]
    #[inline]
    pub fn _202_5(self) -> &'a mut W {
        self.variant(PHASEW::_202_5)
    }
    #[doc = "ADC sample lags by 225.0"]
    #[inline]
    pub fn _225(self) -> &'a mut W {
        self.variant(PHASEW::_225)
    }
    #[doc = "ADC sample lags by 247.5"]
    #[inline]
    pub fn _247_5(self) -> &'a mut W {
        self.variant(PHASEW::_247_5)
    }
    #[doc = "ADC sample lags by 270.0"]
    #[inline]
    pub fn _270(self) -> &'a mut W {
        self.variant(PHASEW::_270)
    }
    #[doc = "ADC sample lags by 292.5"]
    #[inline]
    pub fn _292_5(self) -> &'a mut W {
        self.variant(PHASEW::_292_5)
    }
    #[doc = "ADC sample lags by 315.0"]
    #[inline]
    pub fn _315(self) -> &'a mut W {
        self.variant(PHASEW::_315)
    }
    #[doc = "ADC sample lags by 337.5"]
    #[inline]
    pub fn _337_5(self) -> &'a mut W {
        self.variant(PHASEW::_337_5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline]
    pub fn phase(&self) -> PHASER {
        PHASER::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline]
    pub fn phase(&mut self) -> _PHASEW {
        _PHASEW { w: self }
    }
}
