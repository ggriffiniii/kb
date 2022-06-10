#[doc = "Register `CAPCTRL` reader"]
pub struct R(crate::R<CAPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPCTRL` writer"]
pub struct W(crate::W<CAPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_SPEC>;
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
impl From<crate::W<CAPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Capture on CT16Bn_CAP0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: A sequence of 0 then 1 on CT16Bn_CAP0 will cause CAP0 to be loaded with the contents of TC."]
    ENABLE = 1,
}
impl From<CAP0RE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0RE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0RE` reader - Capture on CT16Bn_CAP0 rising edge"]
pub type CAP0RE_R = crate::BitReader<CAP0RE_A>;
impl CAP0RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0RE_A {
        match self.bits {
            false => CAP0RE_A::DISABLE,
            true => CAP0RE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0RE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0RE_A::ENABLE
    }
}
#[doc = "Field `CAP0RE` writer - Capture on CT16Bn_CAP0 rising edge"]
pub type CAP0RE_W<'a> = crate::BitWriter<'a, u32, CAPCTRL_SPEC, CAP0RE_A, 0>;
impl<'a> CAP0RE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0RE_A::DISABLE)
    }
    #[doc = "A sequence of 0 then 1 on CT16Bn_CAP0 will cause CAP0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0RE_A::ENABLE)
    }
}
#[doc = "Capture on CT16Bn_CAP0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: A sequence of 1 then 0 on CT16Bn_CAP0 will cause CAP0 to be loaded with the contents of TC."]
    ENABLE = 1,
}
impl From<CAP0FE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0FE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0FE` reader - Capture on CT16Bn_CAP0 falling edge"]
pub type CAP0FE_R = crate::BitReader<CAP0FE_A>;
impl CAP0FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0FE_A {
        match self.bits {
            false => CAP0FE_A::DISABLE,
            true => CAP0FE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0FE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0FE_A::ENABLE
    }
}
#[doc = "Field `CAP0FE` writer - Capture on CT16Bn_CAP0 falling edge"]
pub type CAP0FE_W<'a> = crate::BitWriter<'a, u32, CAPCTRL_SPEC, CAP0FE_A, 1>;
impl<'a> CAP0FE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0FE_A::DISABLE)
    }
    #[doc = "A sequence of 1 then 0 on CT16Bn_CAP0 will cause CAP0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0FE_A::ENABLE)
    }
}
#[doc = "Interrupt on CT16Bn_CAP0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: A CAP0 load due to a CT16Bn_CAP0 event will generate an interrupt."]
    ENABLE = 1,
}
impl From<CAP0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0IE` reader - Interrupt on CT16Bn_CAP0 event"]
pub type CAP0IE_R = crate::BitReader<CAP0IE_A>;
impl CAP0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0IE_A {
        match self.bits {
            false => CAP0IE_A::DISABLE,
            true => CAP0IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0IE_A::ENABLE
    }
}
#[doc = "Field `CAP0IE` writer - Interrupt on CT16Bn_CAP0 event"]
pub type CAP0IE_W<'a> = crate::BitWriter<'a, u32, CAPCTRL_SPEC, CAP0IE_A, 2>;
impl<'a> CAP0IE_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0IE_A::DISABLE)
    }
    #[doc = "A CAP0 load due to a CT16Bn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0IE_A::ENABLE)
    }
}
#[doc = "CAP0 function enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable CAP0 function"]
    ENABLE = 1,
}
impl From<CAP0EN_A> for bool {
    #[inline(always)]
    fn from(variant: CAP0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0EN` reader - CAP0 function enable"]
pub type CAP0EN_R = crate::BitReader<CAP0EN_A>;
impl CAP0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAP0EN_A {
        match self.bits {
            false => CAP0EN_A::DISABLE,
            true => CAP0EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CAP0EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CAP0EN_A::ENABLE
    }
}
#[doc = "Field `CAP0EN` writer - CAP0 function enable"]
pub type CAP0EN_W<'a> = crate::BitWriter<'a, u32, CAPCTRL_SPEC, CAP0EN_A, 3>;
impl<'a> CAP0EN_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0EN_A::DISABLE)
    }
    #[doc = "Enable CAP0 function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on CT16Bn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&self) -> CAP0RE_R {
        CAP0RE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on CT16Bn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&self) -> CAP0FE_R {
        CAP0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on CT16Bn_CAP0 event"]
    #[inline(always)]
    pub fn cap0ie(&self) -> CAP0IE_R {
        CAP0IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CAP0 function enable"]
    #[inline(always)]
    pub fn cap0en(&self) -> CAP0EN_R {
        CAP0EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on CT16Bn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0re(&mut self) -> CAP0RE_W {
        CAP0RE_W::new(self)
    }
    #[doc = "Bit 1 - Capture on CT16Bn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0fe(&mut self) -> CAP0FE_W {
        CAP0FE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on CT16Bn_CAP0 event"]
    #[inline(always)]
    pub fn cap0ie(&mut self) -> CAP0IE_W {
        CAP0IE_W::new(self)
    }
    #[doc = "Bit 3 - CAP0 function enable"]
    #[inline(always)]
    pub fn cap0en(&mut self) -> CAP0EN_W {
        CAP0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x80 CT16Bn Capture Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl](index.html) module"]
pub struct CAPCTRL_SPEC;
impl crate::RegisterSpec for CAPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capctrl::R](R) reader structure"]
impl crate::Readable for CAPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capctrl::W](W) writer structure"]
impl crate::Writable for CAPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPCTRL to value 0"]
impl crate::Resettable for CAPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
