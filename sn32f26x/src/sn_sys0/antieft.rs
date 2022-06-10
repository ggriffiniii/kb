#[doc = "Register `ANTIEFT` reader"]
pub struct R(crate::R<ANTIEFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANTIEFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANTIEFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANTIEFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANTIEFT` writer"]
pub struct W(crate::W<ANTIEFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANTIEFT_SPEC>;
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
impl From<crate::W<ANTIEFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANTIEFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Anti-EFT ability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AEFT_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "2: Low"]
    _2 = 2,
    #[doc = "3: Medium"]
    _3 = 3,
    #[doc = "4: Strong"]
    _4 = 4,
}
impl From<AEFT_A> for u8 {
    #[inline(always)]
    fn from(variant: AEFT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AEFT` reader - Anti-EFT ability"]
pub type AEFT_R = crate::FieldReader<u8, AEFT_A>;
impl AEFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AEFT_A> {
        match self.bits {
            0 => Some(AEFT_A::_0),
            2 => Some(AEFT_A::_2),
            3 => Some(AEFT_A::_3),
            4 => Some(AEFT_A::_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEFT_A::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == AEFT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == AEFT_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == AEFT_A::_4
    }
}
#[doc = "Field `AEFT` writer - Anti-EFT ability"]
pub type AEFT_W<'a> = crate::FieldWriter<'a, u32, ANTIEFT_SPEC, u8, AEFT_A, 3, 0>;
impl<'a> AEFT_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEFT_A::_0)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(AEFT_A::_2)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(AEFT_A::_3)
    }
    #[doc = "Strong"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(AEFT_A::_4)
    }
}
impl R {
    #[doc = "Bits 0:2 - Anti-EFT ability"]
    #[inline(always)]
    pub fn aeft(&self) -> AEFT_R {
        AEFT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Anti-EFT ability"]
    #[inline(always)]
    pub fn aeft(&mut self) -> AEFT_W {
        AEFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x30 Anti-EFT Ability Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [antieft](index.html) module"]
pub struct ANTIEFT_SPEC;
impl crate::RegisterSpec for ANTIEFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [antieft::R](R) reader structure"]
impl crate::Readable for ANTIEFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [antieft::W](W) writer structure"]
impl crate::Writable for ANTIEFT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANTIEFT to value 0"]
impl crate::Resettable for ANTIEFT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
