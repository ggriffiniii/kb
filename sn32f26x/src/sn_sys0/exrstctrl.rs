#[doc = "Register `EXRSTCTRL` reader"]
pub struct R(crate::R<EXRSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXRSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXRSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXRSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXRSTCTRL` writer"]
pub struct W(crate::W<EXRSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXRSTCTRL_SPEC>;
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
impl From<crate::W<EXRSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXRSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External reset pin disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETDIS_A {
    #[doc = "0: P3.8 acts as RESET pin"]
    ENABLE = 0,
    #[doc = "1: P3.8 acts as GPIO pin"]
    DISABLE = 1,
}
impl From<RESETDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RESETDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETDIS` reader - External reset pin disable"]
pub type RESETDIS_R = crate::BitReader<RESETDIS_A>;
impl RESETDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETDIS_A {
        match self.bits {
            false => RESETDIS_A::ENABLE,
            true => RESETDIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RESETDIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RESETDIS_A::DISABLE
    }
}
#[doc = "Field `RESETDIS` writer - External reset pin disable"]
pub type RESETDIS_W<'a> = crate::BitWriter<'a, u32, EXRSTCTRL_SPEC, RESETDIS_A, 0>;
impl<'a> RESETDIS_W<'a> {
    #[doc = "P3.8 acts as RESET pin"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RESETDIS_A::ENABLE)
    }
    #[doc = "P3.8 acts as GPIO pin"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESETDIS_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - External reset pin disable"]
    #[inline(always)]
    pub fn resetdis(&self) -> RESETDIS_R {
        RESETDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External reset pin disable"]
    #[inline(always)]
    pub fn resetdis(&mut self) -> RESETDIS_W {
        RESETDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x1C External Reset Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exrstctrl](index.html) module"]
pub struct EXRSTCTRL_SPEC;
impl crate::RegisterSpec for EXRSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exrstctrl::R](R) reader structure"]
impl crate::Readable for EXRSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exrstctrl::W](W) writer structure"]
impl crate::Writable for EXRSTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXRSTCTRL to value 0x01"]
impl crate::Resettable for EXRSTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
