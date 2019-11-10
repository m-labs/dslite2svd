#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Error Detected"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction of Rotation"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
