#[doc = "Register `NDTCTRL` reader"]
pub struct R(crate::R<NDTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDTCTRL` writer"]
pub struct W(crate::W<NDTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDTCTRL_SPEC>;
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
impl From<crate::W<NDTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "NDT for VDD interrupt enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDT5V_IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<NDT5V_IE_A> for bool {
    #[inline(always)]
    fn from(variant: NDT5V_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDT5V_IE` reader - NDT for VDD interrupt enable bit"]
pub type NDT5V_IE_R = crate::BitReader<NDT5V_IE_A>;
impl NDT5V_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDT5V_IE_A {
        match self.bits {
            false => NDT5V_IE_A::DISABLE,
            true => NDT5V_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NDT5V_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NDT5V_IE_A::ENABLE
    }
}
#[doc = "Field `NDT5V_IE` writer - NDT for VDD interrupt enable bit"]
pub type NDT5V_IE_W<'a> = crate::BitWriter<'a, u32, NDTCTRL_SPEC, NDT5V_IE_A, 1>;
impl<'a> NDT5V_IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NDT5V_IE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NDT5V_IE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 1 - NDT for VDD interrupt enable bit"]
    #[inline(always)]
    pub fn ndt5v_ie(&self) -> NDT5V_IE_R {
        NDT5V_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - NDT for VDD interrupt enable bit"]
    #[inline(always)]
    pub fn ndt5v_ie(&mut self) -> NDT5V_IE_W {
        NDT5V_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x28 Noise Detect Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndtctrl](index.html) module"]
pub struct NDTCTRL_SPEC;
impl crate::RegisterSpec for NDTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndtctrl::R](R) reader structure"]
impl crate::Readable for NDTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndtctrl::W](W) writer structure"]
impl crate::Writable for NDTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDTCTRL to value 0"]
impl crate::Resettable for NDTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
