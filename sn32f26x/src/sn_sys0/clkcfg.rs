#[doc = "Register `CLKCFG` reader"]
pub struct R(crate::R<CLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCFG` writer"]
pub struct W(crate::W<CLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCFG_SPEC>;
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
impl From<crate::W<CLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "System clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLKSEL_A {
    #[doc = "0: HCLK=IHRC"]
    IHRC = 0,
    #[doc = "1: HCLK=ILRC"]
    ILRC = 1,
}
impl From<SYSCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCLKSEL` reader - System clock source selection"]
pub type SYSCLKSEL_R = crate::FieldReader<u8, SYSCLKSEL_A>;
impl SYSCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCLKSEL_A> {
        match self.bits {
            0 => Some(SYSCLKSEL_A::IHRC),
            1 => Some(SYSCLKSEL_A::ILRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IHRC`"]
    #[inline(always)]
    pub fn is_ihrc(&self) -> bool {
        *self == SYSCLKSEL_A::IHRC
    }
    #[doc = "Checks if the value of the field is `ILRC`"]
    #[inline(always)]
    pub fn is_ilrc(&self) -> bool {
        *self == SYSCLKSEL_A::ILRC
    }
}
#[doc = "Field `SYSCLKSEL` writer - System clock source selection"]
pub type SYSCLKSEL_W<'a> = crate::FieldWriter<'a, u32, CLKCFG_SPEC, u8, SYSCLKSEL_A, 3, 0>;
impl<'a> SYSCLKSEL_W<'a> {
    #[doc = "HCLK=IHRC"]
    #[inline(always)]
    pub fn ihrc(self) -> &'a mut W {
        self.variant(SYSCLKSEL_A::IHRC)
    }
    #[doc = "HCLK=ILRC"]
    #[inline(always)]
    pub fn ilrc(self) -> &'a mut W {
        self.variant(SYSCLKSEL_A::ILRC)
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYSCLKST_A {
    #[doc = "0: IHRC is used as system clock"]
    IHRC = 0,
    #[doc = "1: ILRC is used as system clock"]
    ILRC = 1,
}
impl From<SYSCLKST_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCLKST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYSCLKST` reader - System clock switch status"]
pub type SYSCLKST_R = crate::FieldReader<u8, SYSCLKST_A>;
impl SYSCLKST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCLKST_A> {
        match self.bits {
            0 => Some(SYSCLKST_A::IHRC),
            1 => Some(SYSCLKST_A::ILRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IHRC`"]
    #[inline(always)]
    pub fn is_ihrc(&self) -> bool {
        *self == SYSCLKST_A::IHRC
    }
    #[doc = "Checks if the value of the field is `ILRC`"]
    #[inline(always)]
    pub fn is_ilrc(&self) -> bool {
        *self == SYSCLKST_A::ILRC
    }
}
impl R {
    #[doc = "Bits 0:2 - System clock source selection"]
    #[inline(always)]
    pub fn sysclksel(&self) -> SYSCLKSEL_R {
        SYSCLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - System clock switch status"]
    #[inline(always)]
    pub fn sysclkst(&self) -> SYSCLKST_R {
        SYSCLKST_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock source selection"]
    #[inline(always)]
    pub fn sysclksel(&mut self) -> SYSCLKSEL_W {
        SYSCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x0C System Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcfg](index.html) module"]
pub struct CLKCFG_SPEC;
impl crate::RegisterSpec for CLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcfg::R](R) reader structure"]
impl crate::Readable for CLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](W) writer structure"]
impl crate::Writable for CLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for CLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
