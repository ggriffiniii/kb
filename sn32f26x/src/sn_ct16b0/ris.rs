#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Match channel 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0IF_A {
    #[doc = "0: No interrupt on match channel 0"]
    NOINTERRUPT = 0,
    #[doc = "1: Interrupt requirements met on match channel 0"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<MR0IF_A> for bool {
    #[inline(always)]
    fn from(variant: MR0IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0IF` reader - Match channel 0 interrupt flag"]
pub type MR0IF_R = crate::BitReader<MR0IF_A>;
impl MR0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0IF_A {
        match self.bits {
            false => MR0IF_A::NOINTERRUPT,
            true => MR0IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NOINTERRUPT`"]
    #[inline(always)]
    pub fn is_nointerrupt(&self) -> bool {
        *self == MR0IF_A::NOINTERRUPT
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == MR0IF_A::METINTERRUPTREQUIREMENTS
    }
}
#[doc = "Capture channel 0 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IF_A {
    #[doc = "0: No interrupt on CAP0"]
    NO = 0,
    #[doc = "1: Interrupt requirements met on CAP0"]
    METINTERRUPTREQUIREMENTS = 1,
}
impl From<CAP0IF_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0IF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0IF` reader - Capture channel 0 interrupt flag"]
pub type CAP0IF_R = crate::BitReader<CAP0IF_A>;
impl CAP0IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0IF_A {
        match self.bits {
            false => CAP0IF_A::NO,
            true => CAP0IF_A::METINTERRUPTREQUIREMENTS,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CAP0IF_A::NO
    }
    #[doc = "Checks if the value of the field is `METINTERRUPTREQUIREMENTS`"]
    #[inline(always)]
    pub fn is_metinterruptrequirements(&self) -> bool {
        *self == CAP0IF_A::METINTERRUPTREQUIREMENTS
    }
}
impl R {
    #[doc = "Bit 0 - Match channel 0 interrupt flag"]
    #[inline(always)]
    pub fn mr0if(&self) -> MR0IF_R {
        MR0IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 24 - Capture channel 0 interrupt flag"]
    #[inline(always)]
    pub fn cap0if(&self) -> CAP0IF_R {
        CAP0IF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Offset:0xA4 CT16Bn Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
