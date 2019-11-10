#[doc = "Reader of register MBMON"]
pub type R = crate::R<u32, super::MBMON>;
#[doc = "Reader of field `SCL`"]
pub type SCL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDA`"]
pub type SDA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn scl(&self) -> SCL_R {
        SCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn sda(&self) -> SDA_R {
        SDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
