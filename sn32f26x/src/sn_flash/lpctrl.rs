#[doc = "Register `LPCTRL` reader"]
pub struct R(crate::R<LPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCTRL` writer"]
pub struct W(crate::W<LPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCTRL_SPEC>;
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
impl From<crate::W<LPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash Low Power mode selection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMODE_A {
    #[doc = "0: HCLK is less than 24MHz"]
    _0 = 0,
    #[doc = "5: HCLK is more than or equal to 24MHz"]
    _5 = 5,
}
impl From<LPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMODE` reader - Flash Low Power mode selection bit"]
pub type LPMODE_R = crate::FieldReader<u8, LPMODE_A>;
impl LPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMODE_A> {
        match self.bits {
            0 => Some(LPMODE_A::_0),
            5 => Some(LPMODE_A::_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == LPMODE_A::_5
    }
}
#[doc = "Field `LPMODE` writer - Flash Low Power mode selection bit"]
pub type LPMODE_W<'a> = crate::FieldWriter<'a, u32, LPCTRL_SPEC, u8, LPMODE_A, 4, 0>;
impl<'a> LPMODE_W<'a> {
    #[doc = "HCLK is less than 24MHz"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPMODE_A::_0)
    }
    #[doc = "HCLK is more than or equal to 24MHz"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(LPMODE_A::_5)
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash Low Power mode selection bit"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash Low Power mode selection bit"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W {
        LPMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 Flash Low Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpctrl](index.html) module"]
pub struct LPCTRL_SPEC;
impl crate::RegisterSpec for LPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpctrl::R](R) reader structure"]
impl crate::Readable for LPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpctrl::W](W) writer structure"]
impl crate::Writable for LPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPCTRL to value 0"]
impl crate::Resettable for LPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
