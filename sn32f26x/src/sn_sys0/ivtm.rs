#[doc = "Register `IVTM` reader"]
pub struct R(crate::R<IVTM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVTM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVTM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVTM` writer"]
pub struct W(crate::W<IVTM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IVTM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVTM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt table mapping selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IVTM_A {
    #[doc = "1: Map to User ROM"]
    MAPTOUSERROM = 1,
    #[doc = "2: Map to SRAM"]
    MAPTOSRAM = 2,
}
impl From<IVTM_A> for u8 {
    #[inline(always)]
    fn from(variant: IVTM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IVTM` reader - Interrupt table mapping selection"]
pub type IVTM_R = crate::FieldReader<u8, IVTM_A>;
impl IVTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IVTM_A> {
        match self.bits {
            1 => Some(IVTM_A::MAPTOUSERROM),
            2 => Some(IVTM_A::MAPTOSRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAPTOUSERROM`"]
    #[inline(always)]
    pub fn is_mapto_user_rom(&self) -> bool {
        *self == IVTM_A::MAPTOUSERROM
    }
    #[doc = "Checks if the value of the field is `MAPTOSRAM`"]
    #[inline(always)]
    pub fn is_mapto_sram(&self) -> bool {
        *self == IVTM_A::MAPTOSRAM
    }
}
#[doc = "Field `IVTM` writer - Interrupt table mapping selection"]
pub type IVTM_W<'a> = crate::FieldWriter<'a, u32, IVTM_SPEC, u8, IVTM_A, 2, 0>;
impl<'a> IVTM_W<'a> {
    #[doc = "Map to User ROM"]
    #[inline(always)]
    pub fn mapto_user_rom(self) -> &'a mut W {
        self.variant(IVTM_A::MAPTOUSERROM)
    }
    #[doc = "Map to SRAM"]
    #[inline(always)]
    pub fn mapto_sram(self) -> &'a mut W {
        self.variant(IVTM_A::MAPTOSRAM)
    }
}
#[doc = "Field `IVTMKEY` writer - IVTMKEY register key"]
pub type IVTMKEY_W<'a> = crate::FieldWriter<'a, u32, IVTM_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:1 - Interrupt table mapping selection"]
    #[inline(always)]
    pub fn ivtm(&self) -> IVTM_R {
        IVTM_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt table mapping selection"]
    #[inline(always)]
    pub fn ivtm(&mut self) -> IVTM_W {
        IVTM_W::new(self)
    }
    #[doc = "Bits 16:31 - IVTMKEY register key"]
    #[inline(always)]
    pub fn ivtmkey(&mut self) -> IVTMKEY_W {
        IVTMKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x24 Interrupt Vector Table Mapping Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivtm](index.html) module"]
pub struct IVTM_SPEC;
impl crate::RegisterSpec for IVTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivtm::R](R) reader structure"]
impl crate::Readable for IVTM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivtm::W](W) writer structure"]
impl crate::Writable for IVTM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVTM to value 0x01"]
impl crate::Resettable for IVTM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
