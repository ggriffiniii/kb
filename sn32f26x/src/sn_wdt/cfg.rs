#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTEN_A {
    #[doc = "0: Disable WDT"]
    DISABLE = 0,
    #[doc = "1: Enable WDT"]
    ENABLE = 1,
}
impl From<WDTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTEN` reader - WDT enable"]
pub type WDTEN_R = crate::BitReader<WDTEN_A>;
impl WDTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTEN_A {
        match self.bits {
            false => WDTEN_A::DISABLE,
            true => WDTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDTEN_A::ENABLE
    }
}
#[doc = "Field `WDTEN` writer - WDT enable"]
pub type WDTEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, WDTEN_A, 0>;
impl<'a> WDTEN_W<'a> {
    #[doc = "Disable WDT"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTEN_A::DISABLE)
    }
    #[doc = "Enable WDT"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTEN_A::ENABLE)
    }
}
#[doc = "WDT interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTIE_A {
    #[doc = "0: WDT reset when WDT time-out"]
    DISABLE = 0,
    #[doc = "1: Enable WDT interrupt"]
    ENABLE = 1,
}
impl From<WDTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WDTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTIE` reader - WDT interrupt enable"]
pub type WDTIE_R = crate::BitReader<WDTIE_A>;
impl WDTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIE_A {
        match self.bits {
            false => WDTIE_A::DISABLE,
            true => WDTIE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WDTIE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WDTIE_A::ENABLE
    }
}
#[doc = "Field `WDTIE` writer - WDT interrupt enable"]
pub type WDTIE_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, WDTIE_A, 1>;
impl<'a> WDTIE_W<'a> {
    #[doc = "WDT reset when WDT time-out"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WDTIE_A::DISABLE)
    }
    #[doc = "Enable WDT interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WDTIE_A::ENABLE)
    }
}
#[doc = "WDT interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTINT_A {
    #[doc = "0: No WDT time-out"]
    NO = 0,
    #[doc = "1: WDT interrupt is triggered if WDTIE=1"]
    WDTTIMEOUT = 1,
}
impl From<WDTINT_A> for bool {
    #[inline(always)]
    fn from(variant: WDTINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTINT` reader - WDT interrupt flag"]
pub type WDTINT_R = crate::BitReader<WDTINT_A>;
impl WDTINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTINT_A {
        match self.bits {
            false => WDTINT_A::NO,
            true => WDTINT_A::WDTTIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WDTINT_A::NO
    }
    #[doc = "Checks if the value of the field is `WDTTIMEOUT`"]
    #[inline(always)]
    pub fn is_wdttimeout(&self) -> bool {
        *self == WDTINT_A::WDTTIMEOUT
    }
}
#[doc = "Field `WDTINT` writer - WDT interrupt flag"]
pub type WDTINT_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, WDTINT_A, 2>;
impl<'a> WDTINT_W<'a> {
    #[doc = "No WDT time-out"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(WDTINT_A::NO)
    }
    #[doc = "WDT interrupt is triggered if WDTIE=1"]
    #[inline(always)]
    pub fn wdttimeout(self) -> &'a mut W {
        self.variant(WDTINT_A::WDTTIMEOUT)
    }
}
#[doc = "Field `WDKEY` writer - WDT register key"]
pub type WDKEY_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bit 0 - WDT enable"]
    #[inline(always)]
    pub fn wdten(&self) -> WDTEN_R {
        WDTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDT interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&self) -> WDTIE_R {
        WDTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDT interrupt flag"]
    #[inline(always)]
    pub fn wdtint(&self) -> WDTINT_R {
        WDTINT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT enable"]
    #[inline(always)]
    pub fn wdten(&mut self) -> WDTEN_W {
        WDTEN_W::new(self)
    }
    #[doc = "Bit 1 - WDT interrupt enable"]
    #[inline(always)]
    pub fn wdtie(&mut self) -> WDTIE_W {
        WDTIE_W::new(self)
    }
    #[doc = "Bit 2 - WDT interrupt flag"]
    #[inline(always)]
    pub fn wdtint(&mut self) -> WDTINT_W {
        WDTINT_W::new(self)
    }
    #[doc = "Bits 16:31 - WDT register key"]
    #[inline(always)]
    pub fn wdkey(&mut self) -> WDKEY_W {
        WDKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 WDT Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
