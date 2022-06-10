#[doc = "Register `TMRCTRL` reader"]
pub struct R(crate::R<TMRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMRCTRL` writer"]
pub struct W(crate::W<TMRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMRCTRL_SPEC>;
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
impl From<crate::W<TMRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEN_A {
    #[doc = "0: Disable counter"]
    DISABLE = 0,
    #[doc = "1: Enable Timer Counter and Prescale Counter for counting"]
    ENABLE = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<CEN_A>;
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLE,
            true => CEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CEN_A::ENABLE
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a> = crate::BitWriter<'a, u32, TMRCTRL_SPEC, CEN_A, 0>;
impl<'a> CEN_W<'a> {
    #[doc = "Disable counter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CEN_A::DISABLE)
    }
    #[doc = "Enable Timer Counter and Prescale Counter for counting"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CEN_A::ENABLE)
    }
}
#[doc = "Counter Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRST_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK"]
    RESETCOUNTER = 1,
}
impl From<CRST_A> for bool {
    #[inline(always)]
    fn from(variant: CRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRST` reader - Counter Reset"]
pub type CRST_R = crate::BitReader<CRST_A>;
impl CRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRST_A {
        match self.bits {
            false => CRST_A::DISABLE,
            true => CRST_A::RESETCOUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RESETCOUNTER`"]
    #[inline(always)]
    pub fn is_reset_counter(&self) -> bool {
        *self == CRST_A::RESETCOUNTER
    }
}
#[doc = "Field `CRST` writer - Counter Reset"]
pub type CRST_W<'a> = crate::BitWriter<'a, u32, TMRCTRL_SPEC, CRST_A, 1>;
impl<'a> CRST_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRST_A::DISABLE)
    }
    #[doc = "Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK"]
    #[inline(always)]
    pub fn reset_counter(self) -> &'a mut W {
        self.variant(CRST_A::RESETCOUNTER)
    }
}
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn crst(&self) -> CRST_R {
        CRST_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn crst(&mut self) -> CRST_W {
        CRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 CT16Bn Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmrctrl](index.html) module"]
pub struct TMRCTRL_SPEC;
impl crate::RegisterSpec for TMRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmrctrl::R](R) reader structure"]
impl crate::Readable for TMRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmrctrl::W](W) writer structure"]
impl crate::Writable for TMRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMRCTRL to value 0"]
impl crate::Resettable for TMRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
