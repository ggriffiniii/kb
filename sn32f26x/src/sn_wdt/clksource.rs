#[doc = "Register `CLKSOURCE` reader"]
pub struct R(crate::R<CLKSOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKSOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKSOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSOURCE` writer"]
pub struct W(crate::W<CLKSOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSOURCE_SPEC>;
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
impl From<crate::W<CLKSOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKSOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT clock source\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "2: WDT clock source=ILRC"]
    ILRC = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKSEL` reader - WDT clock source"]
pub type CLKSEL_R = crate::FieldReader<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSEL_A> {
        match self.bits {
            2 => Some(CLKSEL_A::ILRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ILRC`"]
    #[inline(always)]
    pub fn is_ilrc(&self) -> bool {
        *self == CLKSEL_A::ILRC
    }
}
#[doc = "Field `CLKSEL` writer - WDT clock source"]
pub type CLKSEL_W<'a> = crate::FieldWriter<'a, u32, CLKSOURCE_SPEC, u8, CLKSEL_A, 2, 0>;
impl<'a> CLKSEL_W<'a> {
    #[doc = "WDT clock source=ILRC"]
    #[inline(always)]
    pub fn ilrc(self) -> &'a mut W {
        self.variant(CLKSEL_A::ILRC)
    }
}
#[doc = "Field `WDKEY` writer - WDT register key"]
pub type WDKEY_W<'a> = crate::FieldWriter<'a, u32, CLKSOURCE_SPEC, u16, u16, 16, 16>;
impl R {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - WDT clock source"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W::new(self)
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
#[doc = "Offset:0x04 WDT Clock Source Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksource](index.html) module"]
pub struct CLKSOURCE_SPEC;
impl crate::RegisterSpec for CLKSOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clksource::R](R) reader structure"]
impl crate::Readable for CLKSOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clksource::W](W) writer structure"]
impl crate::Writable for CLKSOURCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSOURCE to value 0x02"]
impl crate::Resettable for CLKSOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
