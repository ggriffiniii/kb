#[doc = "Register `CHKSUM` reader"]
pub struct R(crate::R<CHKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHKSUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UserROM` reader - Checksum of User ROM"]
pub type USERROM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BootROM` reader - Checksum of Boot ROM"]
pub type BOOTROM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Checksum of User ROM"]
    #[inline(always)]
    pub fn user_rom(&self) -> USERROM_R {
        USERROM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Checksum of Boot ROM"]
    #[inline(always)]
    pub fn boot_rom(&self) -> BOOTROM_R {
        BOOTROM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Offset:0x14 Flash Checksum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chksum](index.html) module"]
pub struct CHKSUM_SPEC;
impl crate::RegisterSpec for CHKSUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chksum::R](R) reader structure"]
impl crate::Readable for CHKSUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHKSUM to value 0"]
impl crate::Resettable for CHKSUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
