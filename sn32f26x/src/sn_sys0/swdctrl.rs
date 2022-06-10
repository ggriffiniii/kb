#[doc = "Register `SWDCTRL` reader"]
pub struct R(crate::R<SWDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWDCTRL` writer"]
pub struct W(crate::W<SWDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWDCTRL_SPEC>;
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
impl From<crate::W<SWDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SWD pin disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDDIS_A {
    #[doc = "0: Enable SWD pins"]
    ENABLE = 0,
    #[doc = "1: Disable SWD pins, P3.7 and P3.6 as GPIO pins"]
    DISABLE = 1,
}
impl From<SWDDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SWDDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDDIS` reader - SWD pin disable"]
pub type SWDDIS_R = crate::BitReader<SWDDIS_A>;
impl SWDDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDDIS_A {
        match self.bits {
            false => SWDDIS_A::ENABLE,
            true => SWDDIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWDDIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWDDIS_A::DISABLE
    }
}
#[doc = "Field `SWDDIS` writer - SWD pin disable"]
pub type SWDDIS_W<'a> = crate::BitWriter<'a, u32, SWDCTRL_SPEC, SWDDIS_A, 0>;
impl<'a> SWDDIS_W<'a> {
    #[doc = "Enable SWD pins"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWDDIS_A::ENABLE)
    }
    #[doc = "Disable SWD pins, P3.7 and P3.6 as GPIO pins"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWDDIS_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - SWD pin disable"]
    #[inline(always)]
    pub fn swddis(&self) -> SWDDIS_R {
        SWDDIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWD pin disable"]
    #[inline(always)]
    pub fn swddis(&mut self) -> SWDDIS_W {
        SWDDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x20 SWD Pin Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swdctrl](index.html) module"]
pub struct SWDCTRL_SPEC;
impl crate::RegisterSpec for SWDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swdctrl::R](R) reader structure"]
impl crate::Readable for SWDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swdctrl::W](W) writer structure"]
impl crate::Writable for SWDCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWDCTRL to value 0"]
impl crate::Resettable for SWDCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
