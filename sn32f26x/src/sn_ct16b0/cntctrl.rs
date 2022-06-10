#[doc = "Register `CNTCTRL` reader"]
pub struct R(crate::R<CNTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTCTRL` writer"]
pub struct W(crate::W<CNTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTCTRL_SPEC>;
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
impl From<crate::W<CNTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CTM_A {
    #[doc = "0: Every rising PCLK edge"]
    TIMERMODE = 0,
    #[doc = "1: TC is incremented on rising edges on the CAP0 input selected by CIS bits."]
    COUNTERMODERISING = 1,
    #[doc = "2: TC is incremented on falling edges on the CAP0 input selected by CIS bits."]
    COUNTERMODEFALLING = 2,
    #[doc = "3: TC is incremented on both edges on the CAP0 input selected by CIS bits."]
    COUNTERMODE = 3,
}
impl From<CTM_A> for u8 {
    #[inline(always)]
    fn from(variant: CTM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CTM` reader - Counter/Timer Mode"]
pub type CTM_R = crate::FieldReader<u8, CTM_A>;
impl CTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTM_A {
        match self.bits {
            0 => CTM_A::TIMERMODE,
            1 => CTM_A::COUNTERMODERISING,
            2 => CTM_A::COUNTERMODEFALLING,
            3 => CTM_A::COUNTERMODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMERMODE`"]
    #[inline(always)]
    pub fn is_timer_mode(&self) -> bool {
        *self == CTM_A::TIMERMODE
    }
    #[doc = "Checks if the value of the field is `COUNTERMODERISING`"]
    #[inline(always)]
    pub fn is_counter_mode_rising(&self) -> bool {
        *self == CTM_A::COUNTERMODERISING
    }
    #[doc = "Checks if the value of the field is `COUNTERMODEFALLING`"]
    #[inline(always)]
    pub fn is_counter_mode_falling(&self) -> bool {
        *self == CTM_A::COUNTERMODEFALLING
    }
    #[doc = "Checks if the value of the field is `COUNTERMODE`"]
    #[inline(always)]
    pub fn is_counter_mode(&self) -> bool {
        *self == CTM_A::COUNTERMODE
    }
}
#[doc = "Field `CTM` writer - Counter/Timer Mode"]
pub type CTM_W<'a> = crate::FieldWriterSafe<'a, u32, CNTCTRL_SPEC, u8, CTM_A, 2, 0>;
impl<'a> CTM_W<'a> {
    #[doc = "Every rising PCLK edge"]
    #[inline(always)]
    pub fn timer_mode(self) -> &'a mut W {
        self.variant(CTM_A::TIMERMODE)
    }
    #[doc = "TC is incremented on rising edges on the CAP0 input selected by CIS bits."]
    #[inline(always)]
    pub fn counter_mode_rising(self) -> &'a mut W {
        self.variant(CTM_A::COUNTERMODERISING)
    }
    #[doc = "TC is incremented on falling edges on the CAP0 input selected by CIS bits."]
    #[inline(always)]
    pub fn counter_mode_falling(self) -> &'a mut W {
        self.variant(CTM_A::COUNTERMODEFALLING)
    }
    #[doc = "TC is incremented on both edges on the CAP0 input selected by CIS bits."]
    #[inline(always)]
    pub fn counter_mode(self) -> &'a mut W {
        self.variant(CTM_A::COUNTERMODE)
    }
}
#[doc = "Counter Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CIS_A {
    #[doc = "0: CT16Bn_CAP0"]
    CT16BN_CAP0 = 0,
}
impl From<CIS_A> for u8 {
    #[inline(always)]
    fn from(variant: CIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CIS` reader - Counter Input Select"]
pub type CIS_R = crate::FieldReader<u8, CIS_A>;
impl CIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CIS_A> {
        match self.bits {
            0 => Some(CIS_A::CT16BN_CAP0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CT16BN_CAP0`"]
    #[inline(always)]
    pub fn is_ct16bn_cap0(&self) -> bool {
        *self == CIS_A::CT16BN_CAP0
    }
}
#[doc = "Field `CIS` writer - Counter Input Select"]
pub type CIS_W<'a> = crate::FieldWriter<'a, u32, CNTCTRL_SPEC, u8, CIS_A, 2, 2>;
impl<'a> CIS_W<'a> {
    #[doc = "CT16Bn_CAP0"]
    #[inline(always)]
    pub fn ct16bn_cap0(self) -> &'a mut W {
        self.variant(CIS_A::CT16BN_CAP0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode"]
    #[inline(always)]
    pub fn ctm(&self) -> CTM_R {
        CTM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Counter Input Select"]
    #[inline(always)]
    pub fn cis(&self) -> CIS_R {
        CIS_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode"]
    #[inline(always)]
    pub fn ctm(&mut self) -> CTM_W {
        CTM_W::new(self)
    }
    #[doc = "Bits 2:3 - Counter Input Select"]
    #[inline(always)]
    pub fn cis(&mut self) -> CIS_W {
        CIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 CT16Bn Counter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntctrl](index.html) module"]
pub struct CNTCTRL_SPEC;
impl crate::RegisterSpec for CNTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntctrl::R](R) reader structure"]
impl crate::Readable for CNTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntctrl::W](W) writer structure"]
impl crate::Writable for CNTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTCTRL to value 0"]
impl crate::Resettable for CNTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
