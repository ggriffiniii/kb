#[doc = "Register `ANBCTRL` reader"]
pub struct R(crate::R<ANBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANBCTRL` writer"]
pub struct W(crate::W<ANBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANBCTRL_SPEC>;
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
impl From<crate::W<ANBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IHRC enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IHRCEN_A {
    #[doc = "0: Disable IHRC"]
    DISABLE = 0,
    #[doc = "1: Enable IHRC"]
    ENABLE = 1,
}
impl From<IHRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: IHRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IHRCEN` reader - IHRC enable"]
pub type IHRCEN_R = crate::BitReader<IHRCEN_A>;
impl IHRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IHRCEN_A {
        match self.bits {
            false => IHRCEN_A::DISABLE,
            true => IHRCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IHRCEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IHRCEN_A::ENABLE
    }
}
#[doc = "Field `IHRCEN` writer - IHRC enable"]
pub type IHRCEN_W<'a> = crate::BitWriter<'a, u32, ANBCTRL_SPEC, IHRCEN_A, 0>;
impl<'a> IHRCEN_W<'a> {
    #[doc = "Disable IHRC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IHRCEN_A::DISABLE)
    }
    #[doc = "Enable IHRC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IHRCEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - IHRC enable"]
    #[inline(always)]
    pub fn ihrcen(&self) -> IHRCEN_R {
        IHRCEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IHRC enable"]
    #[inline(always)]
    pub fn ihrcen(&mut self) -> IHRCEN_W {
        IHRCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 Analog Block Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anbctrl](index.html) module"]
pub struct ANBCTRL_SPEC;
impl crate::RegisterSpec for ANBCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anbctrl::R](R) reader structure"]
impl crate::Readable for ANBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anbctrl::W](W) writer structure"]
impl crate::Writable for ANBCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANBCTRL to value 0x01"]
impl crate::Resettable for ANBCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
