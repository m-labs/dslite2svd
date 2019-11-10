#[doc = "Reader of register SPC"]
pub type R = crate::R<u32, super::SPC>;
#[doc = "Writer for register SPC"]
pub type W = crate::W<u32, super::SPC>;
#[doc = "Register SPC `reset()`'s with value 0"]
impl crate::ResetValue for super::SPC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Phase Difference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_A {
    #[doc = "0: ADC sample lags by 0.0"]
    _0,
    #[doc = "1: ADC sample lags by 22.5"]
    _22_5,
    #[doc = "2: ADC sample lags by 45.0"]
    _45,
    #[doc = "3: ADC sample lags by 67.5"]
    _67_5,
    #[doc = "4: ADC sample lags by 90.0"]
    _90,
    #[doc = "5: ADC sample lags by 112.5"]
    _112_5,
    #[doc = "6: ADC sample lags by 135.0"]
    _135,
    #[doc = "7: ADC sample lags by 157.5"]
    _157_5,
    #[doc = "8: ADC sample lags by 180.0"]
    _180,
    #[doc = "9: ADC sample lags by 202.5"]
    _202_5,
    #[doc = "10: ADC sample lags by 225.0"]
    _225,
    #[doc = "11: ADC sample lags by 247.5"]
    _247_5,
    #[doc = "12: ADC sample lags by 270.0"]
    _270,
    #[doc = "13: ADC sample lags by 292.5"]
    _292_5,
    #[doc = "14: ADC sample lags by 315.0"]
    _315,
    #[doc = "15: ADC sample lags by 337.5"]
    _337_5,
}
impl From<PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: PHASE_A) -> Self {
        match variant {
            PHASE_A::_0 => 0,
            PHASE_A::_22_5 => 1,
            PHASE_A::_45 => 2,
            PHASE_A::_67_5 => 3,
            PHASE_A::_90 => 4,
            PHASE_A::_112_5 => 5,
            PHASE_A::_135 => 6,
            PHASE_A::_157_5 => 7,
            PHASE_A::_180 => 8,
            PHASE_A::_202_5 => 9,
            PHASE_A::_225 => 10,
            PHASE_A::_247_5 => 11,
            PHASE_A::_270 => 12,
            PHASE_A::_292_5 => 13,
            PHASE_A::_315 => 14,
            PHASE_A::_337_5 => 15,
        }
    }
}
#[doc = "Reader of field `PHASE`"]
pub type PHASE_R = crate::R<u8, PHASE_A>;
impl PHASE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHASE_A {
        match self.bits {
            0 => PHASE_A::_0,
            1 => PHASE_A::_22_5,
            2 => PHASE_A::_45,
            3 => PHASE_A::_67_5,
            4 => PHASE_A::_90,
            5 => PHASE_A::_112_5,
            6 => PHASE_A::_135,
            7 => PHASE_A::_157_5,
            8 => PHASE_A::_180,
            9 => PHASE_A::_202_5,
            10 => PHASE_A::_225,
            11 => PHASE_A::_247_5,
            12 => PHASE_A::_270,
            13 => PHASE_A::_292_5,
            14 => PHASE_A::_315,
            15 => PHASE_A::_337_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PHASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_22_5`"]
    #[inline(always)]
    pub fn is_22_5(&self) -> bool {
        *self == PHASE_A::_22_5
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        *self == PHASE_A::_45
    }
    #[doc = "Checks if the value of the field is `_67_5`"]
    #[inline(always)]
    pub fn is_67_5(&self) -> bool {
        *self == PHASE_A::_67_5
    }
    #[doc = "Checks if the value of the field is `_90`"]
    #[inline(always)]
    pub fn is_90(&self) -> bool {
        *self == PHASE_A::_90
    }
    #[doc = "Checks if the value of the field is `_112_5`"]
    #[inline(always)]
    pub fn is_112_5(&self) -> bool {
        *self == PHASE_A::_112_5
    }
    #[doc = "Checks if the value of the field is `_135`"]
    #[inline(always)]
    pub fn is_135(&self) -> bool {
        *self == PHASE_A::_135
    }
    #[doc = "Checks if the value of the field is `_157_5`"]
    #[inline(always)]
    pub fn is_157_5(&self) -> bool {
        *self == PHASE_A::_157_5
    }
    #[doc = "Checks if the value of the field is `_180`"]
    #[inline(always)]
    pub fn is_180(&self) -> bool {
        *self == PHASE_A::_180
    }
    #[doc = "Checks if the value of the field is `_202_5`"]
    #[inline(always)]
    pub fn is_202_5(&self) -> bool {
        *self == PHASE_A::_202_5
    }
    #[doc = "Checks if the value of the field is `_225`"]
    #[inline(always)]
    pub fn is_225(&self) -> bool {
        *self == PHASE_A::_225
    }
    #[doc = "Checks if the value of the field is `_247_5`"]
    #[inline(always)]
    pub fn is_247_5(&self) -> bool {
        *self == PHASE_A::_247_5
    }
    #[doc = "Checks if the value of the field is `_270`"]
    #[inline(always)]
    pub fn is_270(&self) -> bool {
        *self == PHASE_A::_270
    }
    #[doc = "Checks if the value of the field is `_292_5`"]
    #[inline(always)]
    pub fn is_292_5(&self) -> bool {
        *self == PHASE_A::_292_5
    }
    #[doc = "Checks if the value of the field is `_315`"]
    #[inline(always)]
    pub fn is_315(&self) -> bool {
        *self == PHASE_A::_315
    }
    #[doc = "Checks if the value of the field is `_337_5`"]
    #[inline(always)]
    pub fn is_337_5(&self) -> bool {
        *self == PHASE_A::_337_5
    }
}
#[doc = "Write proxy for field `PHASE`"]
pub struct PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC sample lags by 0.0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHASE_A::_0)
    }
    #[doc = "ADC sample lags by 22.5"]
    #[inline(always)]
    pub fn _22_5(self) -> &'a mut W {
        self.variant(PHASE_A::_22_5)
    }
    #[doc = "ADC sample lags by 45.0"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(PHASE_A::_45)
    }
    #[doc = "ADC sample lags by 67.5"]
    #[inline(always)]
    pub fn _67_5(self) -> &'a mut W {
        self.variant(PHASE_A::_67_5)
    }
    #[doc = "ADC sample lags by 90.0"]
    #[inline(always)]
    pub fn _90(self) -> &'a mut W {
        self.variant(PHASE_A::_90)
    }
    #[doc = "ADC sample lags by 112.5"]
    #[inline(always)]
    pub fn _112_5(self) -> &'a mut W {
        self.variant(PHASE_A::_112_5)
    }
    #[doc = "ADC sample lags by 135.0"]
    #[inline(always)]
    pub fn _135(self) -> &'a mut W {
        self.variant(PHASE_A::_135)
    }
    #[doc = "ADC sample lags by 157.5"]
    #[inline(always)]
    pub fn _157_5(self) -> &'a mut W {
        self.variant(PHASE_A::_157_5)
    }
    #[doc = "ADC sample lags by 180.0"]
    #[inline(always)]
    pub fn _180(self) -> &'a mut W {
        self.variant(PHASE_A::_180)
    }
    #[doc = "ADC sample lags by 202.5"]
    #[inline(always)]
    pub fn _202_5(self) -> &'a mut W {
        self.variant(PHASE_A::_202_5)
    }
    #[doc = "ADC sample lags by 225.0"]
    #[inline(always)]
    pub fn _225(self) -> &'a mut W {
        self.variant(PHASE_A::_225)
    }
    #[doc = "ADC sample lags by 247.5"]
    #[inline(always)]
    pub fn _247_5(self) -> &'a mut W {
        self.variant(PHASE_A::_247_5)
    }
    #[doc = "ADC sample lags by 270.0"]
    #[inline(always)]
    pub fn _270(self) -> &'a mut W {
        self.variant(PHASE_A::_270)
    }
    #[doc = "ADC sample lags by 292.5"]
    #[inline(always)]
    pub fn _292_5(self) -> &'a mut W {
        self.variant(PHASE_A::_292_5)
    }
    #[doc = "ADC sample lags by 315.0"]
    #[inline(always)]
    pub fn _315(self) -> &'a mut W {
        self.variant(PHASE_A::_315)
    }
    #[doc = "ADC sample lags by 337.5"]
    #[inline(always)]
    pub fn _337_5(self) -> &'a mut W {
        self.variant(PHASE_A::_337_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W {
        PHASE_W { w: self }
    }
}
