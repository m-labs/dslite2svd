#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: Unlocked"]
    UNLOCKED,
    #[doc = "1: Locked"]
    LOCKED,
    #[doc = "449635665: Unlocks the watchdog timer"]
    UNLOCK,
}
impl From<LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        match variant {
            LOCK_A::UNLOCKED => 0,
            LOCK_A::LOCKED => 1,
            LOCK_A::UNLOCK => 449635665,
        }
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<u32, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LOCK_A::UNLOCKED),
            1 => Val(LOCK_A::LOCKED),
            449635665 => Val(LOCK_A::UNLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == LOCK_A::UNLOCK
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
    }
    #[doc = "Unlocks the watchdog timer"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
