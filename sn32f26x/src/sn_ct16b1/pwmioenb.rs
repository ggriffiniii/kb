#[doc = "Register `PWMIOENB` reader"]
pub struct R(crate::R<PWMIOENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMIOENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMIOENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMIOENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMIOENB` writer"]
pub struct W(crate::W<PWMIOENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMIOENB_SPEC>;
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
impl From<crate::W<PWMIOENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMIOENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CT16Bn_PWM0/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0IOEN_A {
    #[doc = "0: CT16Bn_PWM0 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM0 pin act as match output, and output depends on PWM0EN bit"]
    ENABLE = 1,
}
impl From<PWM0IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM0IOEN` reader - CT16Bn_PWM0/GPIO selection"]
pub type PWM0IOEN_R = crate::BitReader<PWM0IOEN_A>;
impl PWM0IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0IOEN_A {
        match self.bits {
            false => PWM0IOEN_A::DISABLE,
            true => PWM0IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM0IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM0IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM0IOEN` writer - CT16Bn_PWM0/GPIO selection"]
pub type PWM0IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM0IOEN_A, 0>;
impl<'a> PWM0IOEN_W<'a> {
    #[doc = "CT16Bn_PWM0 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM0IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM0 pin act as match output, and output depends on PWM0EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM0IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM1/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1IOEN_A {
    #[doc = "0: CT16Bn_PWM1 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM1 pin act as match output, and output depends on PWM1EN bit"]
    ENABLE = 1,
}
impl From<PWM1IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM1IOEN` reader - CT16Bn_PWM1/GPIO selection"]
pub type PWM1IOEN_R = crate::BitReader<PWM1IOEN_A>;
impl PWM1IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1IOEN_A {
        match self.bits {
            false => PWM1IOEN_A::DISABLE,
            true => PWM1IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM1IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM1IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM1IOEN` writer - CT16Bn_PWM1/GPIO selection"]
pub type PWM1IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM1IOEN_A, 1>;
impl<'a> PWM1IOEN_W<'a> {
    #[doc = "CT16Bn_PWM1 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM1IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM1 pin act as match output, and output depends on PWM1EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM1IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM2/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM2IOEN_A {
    #[doc = "0: CT16Bn_PWM2 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM2 pin act as match output, and output depends on PWM2EN bit"]
    ENABLE = 1,
}
impl From<PWM2IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM2IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM2IOEN` reader - CT16Bn_PWM2/GPIO selection"]
pub type PWM2IOEN_R = crate::BitReader<PWM2IOEN_A>;
impl PWM2IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM2IOEN_A {
        match self.bits {
            false => PWM2IOEN_A::DISABLE,
            true => PWM2IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM2IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM2IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM2IOEN` writer - CT16Bn_PWM2/GPIO selection"]
pub type PWM2IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM2IOEN_A, 2>;
impl<'a> PWM2IOEN_W<'a> {
    #[doc = "CT16Bn_PWM2 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM2IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM2 pin act as match output, and output depends on PWM2EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM2IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM3/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM3IOEN_A {
    #[doc = "0: CT16Bn_PWM3 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM3 pin act as match output, and output depends on PWM3EN bit"]
    ENABLE = 1,
}
impl From<PWM3IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM3IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM3IOEN` reader - CT16Bn_PWM3/GPIO selection"]
pub type PWM3IOEN_R = crate::BitReader<PWM3IOEN_A>;
impl PWM3IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM3IOEN_A {
        match self.bits {
            false => PWM3IOEN_A::DISABLE,
            true => PWM3IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM3IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM3IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM3IOEN` writer - CT16Bn_PWM3/GPIO selection"]
pub type PWM3IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM3IOEN_A, 3>;
impl<'a> PWM3IOEN_W<'a> {
    #[doc = "CT16Bn_PWM3 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM3IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM3 pin act as match output, and output depends on PWM3EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM3IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM4/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM4IOEN_A {
    #[doc = "0: CT16Bn_PWM4 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM4 pin act as match output, and output depends on PWM4EN bit"]
    ENABLE = 1,
}
impl From<PWM4IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM4IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM4IOEN` reader - CT16Bn_PWM4/GPIO selection"]
pub type PWM4IOEN_R = crate::BitReader<PWM4IOEN_A>;
impl PWM4IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM4IOEN_A {
        match self.bits {
            false => PWM4IOEN_A::DISABLE,
            true => PWM4IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM4IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM4IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM4IOEN` writer - CT16Bn_PWM4/GPIO selection"]
pub type PWM4IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM4IOEN_A, 4>;
impl<'a> PWM4IOEN_W<'a> {
    #[doc = "CT16Bn_PWM4 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM4IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM4 pin act as match output, and output depends on PWM4EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM4IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM5/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM5IOEN_A {
    #[doc = "0: CT16Bn_PWM5 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM5 pin act as match output, and output depends on PWM5EN bit"]
    ENABLE = 1,
}
impl From<PWM5IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM5IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM5IOEN` reader - CT16Bn_PWM5/GPIO selection"]
pub type PWM5IOEN_R = crate::BitReader<PWM5IOEN_A>;
impl PWM5IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM5IOEN_A {
        match self.bits {
            false => PWM5IOEN_A::DISABLE,
            true => PWM5IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM5IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM5IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM5IOEN` writer - CT16Bn_PWM5/GPIO selection"]
pub type PWM5IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM5IOEN_A, 5>;
impl<'a> PWM5IOEN_W<'a> {
    #[doc = "CT16Bn_PWM5 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM5IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM5 pin act as match output, and output depends on PWM5EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM5IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM6/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM6IOEN_A {
    #[doc = "0: CT16Bn_PWM6 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM6 pin act as match output, and output depends on PWM6EN bit"]
    ENABLE = 1,
}
impl From<PWM6IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM6IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM6IOEN` reader - CT16Bn_PWM6/GPIO selection"]
pub type PWM6IOEN_R = crate::BitReader<PWM6IOEN_A>;
impl PWM6IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM6IOEN_A {
        match self.bits {
            false => PWM6IOEN_A::DISABLE,
            true => PWM6IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM6IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM6IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM6IOEN` writer - CT16Bn_PWM6/GPIO selection"]
pub type PWM6IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM6IOEN_A, 6>;
impl<'a> PWM6IOEN_W<'a> {
    #[doc = "CT16Bn_PWM6 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM6IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM6 pin act as match output, and output depends on PWM6EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM6IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM7/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM7IOEN_A {
    #[doc = "0: CT16Bn_PWM7 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM7 pin act as match output, and output depends on PWM7EN bit"]
    ENABLE = 1,
}
impl From<PWM7IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM7IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM7IOEN` reader - CT16Bn_PWM7/GPIO selection"]
pub type PWM7IOEN_R = crate::BitReader<PWM7IOEN_A>;
impl PWM7IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM7IOEN_A {
        match self.bits {
            false => PWM7IOEN_A::DISABLE,
            true => PWM7IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM7IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM7IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM7IOEN` writer - CT16Bn_PWM7/GPIO selection"]
pub type PWM7IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM7IOEN_A, 7>;
impl<'a> PWM7IOEN_W<'a> {
    #[doc = "CT16Bn_PWM7 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM7IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM7 pin act as match output, and output depends on PWM7EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM7IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM8/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM8IOEN_A {
    #[doc = "0: CT16Bn_PWM8 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM8 pin act as match output, and output depends on PWM8EN bit"]
    ENABLE = 1,
}
impl From<PWM8IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM8IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM8IOEN` reader - CT16Bn_PWM8/GPIO selection"]
pub type PWM8IOEN_R = crate::BitReader<PWM8IOEN_A>;
impl PWM8IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM8IOEN_A {
        match self.bits {
            false => PWM8IOEN_A::DISABLE,
            true => PWM8IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM8IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM8IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM8IOEN` writer - CT16Bn_PWM8/GPIO selection"]
pub type PWM8IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM8IOEN_A, 8>;
impl<'a> PWM8IOEN_W<'a> {
    #[doc = "CT16Bn_PWM8 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM8IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM8 pin act as match output, and output depends on PWM8EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM8IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM9/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM9IOEN_A {
    #[doc = "0: CT16Bn_PWM9 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM9 pin act as match output, and output depends on PWM9EN bit"]
    ENABLE = 1,
}
impl From<PWM9IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM9IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM9IOEN` reader - CT16Bn_PWM9/GPIO selection"]
pub type PWM9IOEN_R = crate::BitReader<PWM9IOEN_A>;
impl PWM9IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM9IOEN_A {
        match self.bits {
            false => PWM9IOEN_A::DISABLE,
            true => PWM9IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM9IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM9IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM9IOEN` writer - CT16Bn_PWM9/GPIO selection"]
pub type PWM9IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM9IOEN_A, 9>;
impl<'a> PWM9IOEN_W<'a> {
    #[doc = "CT16Bn_PWM9 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM9IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM9 pin act as match output, and output depends on PWM9EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM9IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM10/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM10IOEN_A {
    #[doc = "0: CT16Bn_PWM10 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM10 pin act as match output, and output depends on PWM10EN bit"]
    ENABLE = 1,
}
impl From<PWM10IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM10IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM10IOEN` reader - CT16Bn_PWM10/GPIO selection"]
pub type PWM10IOEN_R = crate::BitReader<PWM10IOEN_A>;
impl PWM10IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM10IOEN_A {
        match self.bits {
            false => PWM10IOEN_A::DISABLE,
            true => PWM10IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM10IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM10IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM10IOEN` writer - CT16Bn_PWM10/GPIO selection"]
pub type PWM10IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM10IOEN_A, 10>;
impl<'a> PWM10IOEN_W<'a> {
    #[doc = "CT16Bn_PWM10 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM10IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM10 pin act as match output, and output depends on PWM10EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM10IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM11/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM11IOEN_A {
    #[doc = "0: CT16Bn_PWM11 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM11 pin act as match output, and output depends on PWM11EN bit"]
    ENABLE = 1,
}
impl From<PWM11IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM11IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM11IOEN` reader - CT16Bn_PWM11/GPIO selection"]
pub type PWM11IOEN_R = crate::BitReader<PWM11IOEN_A>;
impl PWM11IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM11IOEN_A {
        match self.bits {
            false => PWM11IOEN_A::DISABLE,
            true => PWM11IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM11IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM11IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM11IOEN` writer - CT16Bn_PWM11/GPIO selection"]
pub type PWM11IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM11IOEN_A, 11>;
impl<'a> PWM11IOEN_W<'a> {
    #[doc = "CT16Bn_PWM11 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM11IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM11 pin act as match output, and output depends on PWM11EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM11IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM12/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM12IOEN_A {
    #[doc = "0: CT16Bn_PWM12 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM12 pin act as match output, and output depends on PWM12EN bit"]
    ENABLE = 1,
}
impl From<PWM12IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM12IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM12IOEN` reader - CT16Bn_PWM12/GPIO selection"]
pub type PWM12IOEN_R = crate::BitReader<PWM12IOEN_A>;
impl PWM12IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM12IOEN_A {
        match self.bits {
            false => PWM12IOEN_A::DISABLE,
            true => PWM12IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM12IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM12IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM12IOEN` writer - CT16Bn_PWM12/GPIO selection"]
pub type PWM12IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM12IOEN_A, 12>;
impl<'a> PWM12IOEN_W<'a> {
    #[doc = "CT16Bn_PWM12 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM12IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM12 pin act as match output, and output depends on PWM12EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM12IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM13/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM13IOEN_A {
    #[doc = "0: CT16Bn_PWM13 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM13 pin act as match output, and output depends on PWM13EN bit"]
    ENABLE = 1,
}
impl From<PWM13IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM13IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM13IOEN` reader - CT16Bn_PWM13/GPIO selection"]
pub type PWM13IOEN_R = crate::BitReader<PWM13IOEN_A>;
impl PWM13IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM13IOEN_A {
        match self.bits {
            false => PWM13IOEN_A::DISABLE,
            true => PWM13IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM13IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM13IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM13IOEN` writer - CT16Bn_PWM13/GPIO selection"]
pub type PWM13IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM13IOEN_A, 13>;
impl<'a> PWM13IOEN_W<'a> {
    #[doc = "CT16Bn_PWM13 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM13IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM13 pin act as match output, and output depends on PWM13EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM13IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM14/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM14IOEN_A {
    #[doc = "0: CT16Bn_PWM14 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM14 pin act as match output, and output depends on PWM14EN bit"]
    ENABLE = 1,
}
impl From<PWM14IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM14IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM14IOEN` reader - CT16Bn_PWM14/GPIO selection"]
pub type PWM14IOEN_R = crate::BitReader<PWM14IOEN_A>;
impl PWM14IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM14IOEN_A {
        match self.bits {
            false => PWM14IOEN_A::DISABLE,
            true => PWM14IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM14IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM14IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM14IOEN` writer - CT16Bn_PWM14/GPIO selection"]
pub type PWM14IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM14IOEN_A, 14>;
impl<'a> PWM14IOEN_W<'a> {
    #[doc = "CT16Bn_PWM14 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM14IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM14 pin act as match output, and output depends on PWM14EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM14IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM15/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM15IOEN_A {
    #[doc = "0: CT16Bn_PWM15 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM15 pin act as match output, and output depends on PWM15EN bit"]
    ENABLE = 1,
}
impl From<PWM15IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM15IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM15IOEN` reader - CT16Bn_PWM15/GPIO selection"]
pub type PWM15IOEN_R = crate::BitReader<PWM15IOEN_A>;
impl PWM15IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM15IOEN_A {
        match self.bits {
            false => PWM15IOEN_A::DISABLE,
            true => PWM15IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM15IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM15IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM15IOEN` writer - CT16Bn_PWM15/GPIO selection"]
pub type PWM15IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM15IOEN_A, 15>;
impl<'a> PWM15IOEN_W<'a> {
    #[doc = "CT16Bn_PWM15 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM15IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM15 pin act as match output, and output depends on PWM15EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM15IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM16/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM16IOEN_A {
    #[doc = "0: CT16Bn_PWM16 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM16 pin act as match output, and output depends on PWM16EN bit"]
    ENABLE = 1,
}
impl From<PWM16IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM16IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM16IOEN` reader - CT16Bn_PWM16/GPIO selection"]
pub type PWM16IOEN_R = crate::BitReader<PWM16IOEN_A>;
impl PWM16IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM16IOEN_A {
        match self.bits {
            false => PWM16IOEN_A::DISABLE,
            true => PWM16IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM16IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM16IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM16IOEN` writer - CT16Bn_PWM16/GPIO selection"]
pub type PWM16IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM16IOEN_A, 16>;
impl<'a> PWM16IOEN_W<'a> {
    #[doc = "CT16Bn_PWM16 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM16IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM16 pin act as match output, and output depends on PWM16EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM16IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM17/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM17IOEN_A {
    #[doc = "0: CT16Bn_PWM17 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM17 pin act as match output, and output depends on PWM17EN bit"]
    ENABLE = 1,
}
impl From<PWM17IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM17IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM17IOEN` reader - CT16Bn_PWM17/GPIO selection"]
pub type PWM17IOEN_R = crate::BitReader<PWM17IOEN_A>;
impl PWM17IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM17IOEN_A {
        match self.bits {
            false => PWM17IOEN_A::DISABLE,
            true => PWM17IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM17IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM17IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM17IOEN` writer - CT16Bn_PWM17/GPIO selection"]
pub type PWM17IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM17IOEN_A, 17>;
impl<'a> PWM17IOEN_W<'a> {
    #[doc = "CT16Bn_PWM17 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM17IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM17 pin act as match output, and output depends on PWM17EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM17IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM18/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM18IOEN_A {
    #[doc = "0: CT16Bn_PWM18 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM18 pin act as match output, and output depends on PWM18EN bit"]
    ENABLE = 1,
}
impl From<PWM18IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM18IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM18IOEN` reader - CT16Bn_PWM18/GPIO selection"]
pub type PWM18IOEN_R = crate::BitReader<PWM18IOEN_A>;
impl PWM18IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM18IOEN_A {
        match self.bits {
            false => PWM18IOEN_A::DISABLE,
            true => PWM18IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM18IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM18IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM18IOEN` writer - CT16Bn_PWM18/GPIO selection"]
pub type PWM18IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM18IOEN_A, 18>;
impl<'a> PWM18IOEN_W<'a> {
    #[doc = "CT16Bn_PWM18 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM18IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM18 pin act as match output, and output depends on PWM18EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM18IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM19/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM19IOEN_A {
    #[doc = "0: CT16Bn_PWM19 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM19 pin act as match output, and output depends on PWM19EN bit"]
    ENABLE = 1,
}
impl From<PWM19IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM19IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM19IOEN` reader - CT16Bn_PWM19/GPIO selection"]
pub type PWM19IOEN_R = crate::BitReader<PWM19IOEN_A>;
impl PWM19IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM19IOEN_A {
        match self.bits {
            false => PWM19IOEN_A::DISABLE,
            true => PWM19IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM19IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM19IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM19IOEN` writer - CT16Bn_PWM19/GPIO selection"]
pub type PWM19IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM19IOEN_A, 19>;
impl<'a> PWM19IOEN_W<'a> {
    #[doc = "CT16Bn_PWM19 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM19IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM19 pin act as match output, and output depends on PWM19EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM19IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM20/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM20IOEN_A {
    #[doc = "0: CT16Bn_PWM20 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM20 pin act as match output, and output depends on PWM20EN bit"]
    ENABLE = 1,
}
impl From<PWM20IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM20IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM20IOEN` reader - CT16Bn_PWM20/GPIO selection"]
pub type PWM20IOEN_R = crate::BitReader<PWM20IOEN_A>;
impl PWM20IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM20IOEN_A {
        match self.bits {
            false => PWM20IOEN_A::DISABLE,
            true => PWM20IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM20IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM20IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM20IOEN` writer - CT16Bn_PWM20/GPIO selection"]
pub type PWM20IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM20IOEN_A, 20>;
impl<'a> PWM20IOEN_W<'a> {
    #[doc = "CT16Bn_PWM20 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM20IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM20 pin act as match output, and output depends on PWM20EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM20IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM21/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM21IOEN_A {
    #[doc = "0: CT16Bn_PWM21 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM21 pin act as match output, and output depends on PWM21EN bit"]
    ENABLE = 1,
}
impl From<PWM21IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM21IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM21IOEN` reader - CT16Bn_PWM21/GPIO selection"]
pub type PWM21IOEN_R = crate::BitReader<PWM21IOEN_A>;
impl PWM21IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM21IOEN_A {
        match self.bits {
            false => PWM21IOEN_A::DISABLE,
            true => PWM21IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM21IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM21IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM21IOEN` writer - CT16Bn_PWM21/GPIO selection"]
pub type PWM21IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM21IOEN_A, 21>;
impl<'a> PWM21IOEN_W<'a> {
    #[doc = "CT16Bn_PWM21 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM21IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM21 pin act as match output, and output depends on PWM21EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM21IOEN_A::ENABLE)
    }
}
#[doc = "CT16Bn_PWM22/GPIO selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM22IOEN_A {
    #[doc = "0: CT16Bn_PWM22 pin is act as GPIO"]
    DISABLE = 0,
    #[doc = "1: CT16Bn_PWM22 pin act as match output, and output depends on PWM22EN bit"]
    ENABLE = 1,
}
impl From<PWM22IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM22IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM22IOEN` reader - CT16Bn_PWM22/GPIO selection"]
pub type PWM22IOEN_R = crate::BitReader<PWM22IOEN_A>;
impl PWM22IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM22IOEN_A {
        match self.bits {
            false => PWM22IOEN_A::DISABLE,
            true => PWM22IOEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM22IOEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM22IOEN_A::ENABLE
    }
}
#[doc = "Field `PWM22IOEN` writer - CT16Bn_PWM22/GPIO selection"]
pub type PWM22IOEN_W<'a> = crate::BitWriter<'a, u32, PWMIOENB_SPEC, PWM22IOEN_A, 22>;
impl<'a> PWM22IOEN_W<'a> {
    #[doc = "CT16Bn_PWM22 pin is act as GPIO"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM22IOEN_A::DISABLE)
    }
    #[doc = "CT16Bn_PWM22 pin act as match output, and output depends on PWM22EN bit"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM22IOEN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - CT16Bn_PWM0/GPIO selection"]
    #[inline(always)]
    pub fn pwm0ioen(&self) -> PWM0IOEN_R {
        PWM0IOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CT16Bn_PWM1/GPIO selection"]
    #[inline(always)]
    pub fn pwm1ioen(&self) -> PWM1IOEN_R {
        PWM1IOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CT16Bn_PWM2/GPIO selection"]
    #[inline(always)]
    pub fn pwm2ioen(&self) -> PWM2IOEN_R {
        PWM2IOEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CT16Bn_PWM3/GPIO selection"]
    #[inline(always)]
    pub fn pwm3ioen(&self) -> PWM3IOEN_R {
        PWM3IOEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CT16Bn_PWM4/GPIO selection"]
    #[inline(always)]
    pub fn pwm4ioen(&self) -> PWM4IOEN_R {
        PWM4IOEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CT16Bn_PWM5/GPIO selection"]
    #[inline(always)]
    pub fn pwm5ioen(&self) -> PWM5IOEN_R {
        PWM5IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CT16Bn_PWM6/GPIO selection"]
    #[inline(always)]
    pub fn pwm6ioen(&self) -> PWM6IOEN_R {
        PWM6IOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CT16Bn_PWM7/GPIO selection"]
    #[inline(always)]
    pub fn pwm7ioen(&self) -> PWM7IOEN_R {
        PWM7IOEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CT16Bn_PWM8/GPIO selection"]
    #[inline(always)]
    pub fn pwm8ioen(&self) -> PWM8IOEN_R {
        PWM8IOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CT16Bn_PWM9/GPIO selection"]
    #[inline(always)]
    pub fn pwm9ioen(&self) -> PWM9IOEN_R {
        PWM9IOEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CT16Bn_PWM10/GPIO selection"]
    #[inline(always)]
    pub fn pwm10ioen(&self) -> PWM10IOEN_R {
        PWM10IOEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CT16Bn_PWM11/GPIO selection"]
    #[inline(always)]
    pub fn pwm11ioen(&self) -> PWM11IOEN_R {
        PWM11IOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CT16Bn_PWM12/GPIO selection"]
    #[inline(always)]
    pub fn pwm12ioen(&self) -> PWM12IOEN_R {
        PWM12IOEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CT16Bn_PWM13/GPIO selection"]
    #[inline(always)]
    pub fn pwm13ioen(&self) -> PWM13IOEN_R {
        PWM13IOEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CT16Bn_PWM14/GPIO selection"]
    #[inline(always)]
    pub fn pwm14ioen(&self) -> PWM14IOEN_R {
        PWM14IOEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CT16Bn_PWM15/GPIO selection"]
    #[inline(always)]
    pub fn pwm15ioen(&self) -> PWM15IOEN_R {
        PWM15IOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CT16Bn_PWM16/GPIO selection"]
    #[inline(always)]
    pub fn pwm16ioen(&self) -> PWM16IOEN_R {
        PWM16IOEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CT16Bn_PWM17/GPIO selection"]
    #[inline(always)]
    pub fn pwm17ioen(&self) -> PWM17IOEN_R {
        PWM17IOEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CT16Bn_PWM18/GPIO selection"]
    #[inline(always)]
    pub fn pwm18ioen(&self) -> PWM18IOEN_R {
        PWM18IOEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CT16Bn_PWM19/GPIO selection"]
    #[inline(always)]
    pub fn pwm19ioen(&self) -> PWM19IOEN_R {
        PWM19IOEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CT16Bn_PWM20/GPIO selection"]
    #[inline(always)]
    pub fn pwm20ioen(&self) -> PWM20IOEN_R {
        PWM20IOEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CT16Bn_PWM21/GPIO selection"]
    #[inline(always)]
    pub fn pwm21ioen(&self) -> PWM21IOEN_R {
        PWM21IOEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CT16Bn_PWM22/GPIO selection"]
    #[inline(always)]
    pub fn pwm22ioen(&self) -> PWM22IOEN_R {
        PWM22IOEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CT16Bn_PWM0/GPIO selection"]
    #[inline(always)]
    pub fn pwm0ioen(&mut self) -> PWM0IOEN_W {
        PWM0IOEN_W::new(self)
    }
    #[doc = "Bit 1 - CT16Bn_PWM1/GPIO selection"]
    #[inline(always)]
    pub fn pwm1ioen(&mut self) -> PWM1IOEN_W {
        PWM1IOEN_W::new(self)
    }
    #[doc = "Bit 2 - CT16Bn_PWM2/GPIO selection"]
    #[inline(always)]
    pub fn pwm2ioen(&mut self) -> PWM2IOEN_W {
        PWM2IOEN_W::new(self)
    }
    #[doc = "Bit 3 - CT16Bn_PWM3/GPIO selection"]
    #[inline(always)]
    pub fn pwm3ioen(&mut self) -> PWM3IOEN_W {
        PWM3IOEN_W::new(self)
    }
    #[doc = "Bit 4 - CT16Bn_PWM4/GPIO selection"]
    #[inline(always)]
    pub fn pwm4ioen(&mut self) -> PWM4IOEN_W {
        PWM4IOEN_W::new(self)
    }
    #[doc = "Bit 5 - CT16Bn_PWM5/GPIO selection"]
    #[inline(always)]
    pub fn pwm5ioen(&mut self) -> PWM5IOEN_W {
        PWM5IOEN_W::new(self)
    }
    #[doc = "Bit 6 - CT16Bn_PWM6/GPIO selection"]
    #[inline(always)]
    pub fn pwm6ioen(&mut self) -> PWM6IOEN_W {
        PWM6IOEN_W::new(self)
    }
    #[doc = "Bit 7 - CT16Bn_PWM7/GPIO selection"]
    #[inline(always)]
    pub fn pwm7ioen(&mut self) -> PWM7IOEN_W {
        PWM7IOEN_W::new(self)
    }
    #[doc = "Bit 8 - CT16Bn_PWM8/GPIO selection"]
    #[inline(always)]
    pub fn pwm8ioen(&mut self) -> PWM8IOEN_W {
        PWM8IOEN_W::new(self)
    }
    #[doc = "Bit 9 - CT16Bn_PWM9/GPIO selection"]
    #[inline(always)]
    pub fn pwm9ioen(&mut self) -> PWM9IOEN_W {
        PWM9IOEN_W::new(self)
    }
    #[doc = "Bit 10 - CT16Bn_PWM10/GPIO selection"]
    #[inline(always)]
    pub fn pwm10ioen(&mut self) -> PWM10IOEN_W {
        PWM10IOEN_W::new(self)
    }
    #[doc = "Bit 11 - CT16Bn_PWM11/GPIO selection"]
    #[inline(always)]
    pub fn pwm11ioen(&mut self) -> PWM11IOEN_W {
        PWM11IOEN_W::new(self)
    }
    #[doc = "Bit 12 - CT16Bn_PWM12/GPIO selection"]
    #[inline(always)]
    pub fn pwm12ioen(&mut self) -> PWM12IOEN_W {
        PWM12IOEN_W::new(self)
    }
    #[doc = "Bit 13 - CT16Bn_PWM13/GPIO selection"]
    #[inline(always)]
    pub fn pwm13ioen(&mut self) -> PWM13IOEN_W {
        PWM13IOEN_W::new(self)
    }
    #[doc = "Bit 14 - CT16Bn_PWM14/GPIO selection"]
    #[inline(always)]
    pub fn pwm14ioen(&mut self) -> PWM14IOEN_W {
        PWM14IOEN_W::new(self)
    }
    #[doc = "Bit 15 - CT16Bn_PWM15/GPIO selection"]
    #[inline(always)]
    pub fn pwm15ioen(&mut self) -> PWM15IOEN_W {
        PWM15IOEN_W::new(self)
    }
    #[doc = "Bit 16 - CT16Bn_PWM16/GPIO selection"]
    #[inline(always)]
    pub fn pwm16ioen(&mut self) -> PWM16IOEN_W {
        PWM16IOEN_W::new(self)
    }
    #[doc = "Bit 17 - CT16Bn_PWM17/GPIO selection"]
    #[inline(always)]
    pub fn pwm17ioen(&mut self) -> PWM17IOEN_W {
        PWM17IOEN_W::new(self)
    }
    #[doc = "Bit 18 - CT16Bn_PWM18/GPIO selection"]
    #[inline(always)]
    pub fn pwm18ioen(&mut self) -> PWM18IOEN_W {
        PWM18IOEN_W::new(self)
    }
    #[doc = "Bit 19 - CT16Bn_PWM19/GPIO selection"]
    #[inline(always)]
    pub fn pwm19ioen(&mut self) -> PWM19IOEN_W {
        PWM19IOEN_W::new(self)
    }
    #[doc = "Bit 20 - CT16Bn_PWM20/GPIO selection"]
    #[inline(always)]
    pub fn pwm20ioen(&mut self) -> PWM20IOEN_W {
        PWM20IOEN_W::new(self)
    }
    #[doc = "Bit 21 - CT16Bn_PWM21/GPIO selection"]
    #[inline(always)]
    pub fn pwm21ioen(&mut self) -> PWM21IOEN_W {
        PWM21IOEN_W::new(self)
    }
    #[doc = "Bit 22 - CT16Bn_PWM22/GPIO selection"]
    #[inline(always)]
    pub fn pwm22ioen(&mut self) -> PWM22IOEN_W {
        PWM22IOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0xA0 CT16Bn PWM IO Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmioenb](index.html) module"]
pub struct PWMIOENB_SPEC;
impl crate::RegisterSpec for PWMIOENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmioenb::R](R) reader structure"]
impl crate::Readable for PWMIOENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmioenb::W](W) writer structure"]
impl crate::Writable for PWMIOENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMIOENB to value 0"]
impl crate::Resettable for PWMIOENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
