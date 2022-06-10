#[doc = "Register `PWMENB` reader"]
pub struct R(crate::R<PWMENB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMENB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMENB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMENB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMENB` writer"]
pub struct W(crate::W<PWMENB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMENB_SPEC>;
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
impl From<crate::W<PWMENB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMENB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM0 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM0EN_A {
    #[doc = "0: CT16Bn_PWM0 is controlled by EM0"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM0"]
    ENABLE = 1,
}
impl From<PWM0EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM0EN` reader - PWM0 enable"]
pub type PWM0EN_R = crate::BitReader<PWM0EN_A>;
impl PWM0EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0EN_A {
        match self.bits {
            false => PWM0EN_A::DISABLE,
            true => PWM0EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM0EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM0EN_A::ENABLE
    }
}
#[doc = "Field `PWM0EN` writer - PWM0 enable"]
pub type PWM0EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM0EN_A, 0>;
impl<'a> PWM0EN_W<'a> {
    #[doc = "CT16Bn_PWM0 is controlled by EM0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM0EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM0EN_A::ENABLE)
    }
}
#[doc = "PWM1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM1EN_A {
    #[doc = "0: CT16Bn_PWM1 is controlled by EM1"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM1"]
    ENABLE = 1,
}
impl From<PWM1EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM1EN` reader - PWM1 enable"]
pub type PWM1EN_R = crate::BitReader<PWM1EN_A>;
impl PWM1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1EN_A {
        match self.bits {
            false => PWM1EN_A::DISABLE,
            true => PWM1EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM1EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM1EN_A::ENABLE
    }
}
#[doc = "Field `PWM1EN` writer - PWM1 enable"]
pub type PWM1EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM1EN_A, 1>;
impl<'a> PWM1EN_W<'a> {
    #[doc = "CT16Bn_PWM1 is controlled by EM1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM1EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM1EN_A::ENABLE)
    }
}
#[doc = "PWM2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM2EN_A {
    #[doc = "0: CT16Bn_PWM2 is controlled by EM2"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM2"]
    ENABLE = 1,
}
impl From<PWM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM2EN` reader - PWM2 enable"]
pub type PWM2EN_R = crate::BitReader<PWM2EN_A>;
impl PWM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM2EN_A {
        match self.bits {
            false => PWM2EN_A::DISABLE,
            true => PWM2EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM2EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM2EN_A::ENABLE
    }
}
#[doc = "Field `PWM2EN` writer - PWM2 enable"]
pub type PWM2EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM2EN_A, 2>;
impl<'a> PWM2EN_W<'a> {
    #[doc = "CT16Bn_PWM2 is controlled by EM2"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM2EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM2"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM2EN_A::ENABLE)
    }
}
#[doc = "PWM3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM3EN_A {
    #[doc = "0: CT16Bn_PWM3 is controlled by EM3"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM3"]
    ENABLE = 1,
}
impl From<PWM3EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM3EN` reader - PWM3 enable"]
pub type PWM3EN_R = crate::BitReader<PWM3EN_A>;
impl PWM3EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM3EN_A {
        match self.bits {
            false => PWM3EN_A::DISABLE,
            true => PWM3EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM3EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM3EN_A::ENABLE
    }
}
#[doc = "Field `PWM3EN` writer - PWM3 enable"]
pub type PWM3EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM3EN_A, 3>;
impl<'a> PWM3EN_W<'a> {
    #[doc = "CT16Bn_PWM3 is controlled by EM3"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM3EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM3"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM3EN_A::ENABLE)
    }
}
#[doc = "PWM4 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM4EN_A {
    #[doc = "0: CT16Bn_PWM4 is controlled by EM4"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM4"]
    ENABLE = 1,
}
impl From<PWM4EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM4EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM4EN` reader - PWM4 enable"]
pub type PWM4EN_R = crate::BitReader<PWM4EN_A>;
impl PWM4EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM4EN_A {
        match self.bits {
            false => PWM4EN_A::DISABLE,
            true => PWM4EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM4EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM4EN_A::ENABLE
    }
}
#[doc = "Field `PWM4EN` writer - PWM4 enable"]
pub type PWM4EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM4EN_A, 4>;
impl<'a> PWM4EN_W<'a> {
    #[doc = "CT16Bn_PWM4 is controlled by EM4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM4EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM4"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM4EN_A::ENABLE)
    }
}
#[doc = "PWM5 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM5EN_A {
    #[doc = "0: CT16Bn_PWM5 is controlled by EM5"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM5"]
    ENABLE = 1,
}
impl From<PWM5EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM5EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM5EN` reader - PWM5 enable"]
pub type PWM5EN_R = crate::BitReader<PWM5EN_A>;
impl PWM5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM5EN_A {
        match self.bits {
            false => PWM5EN_A::DISABLE,
            true => PWM5EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM5EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM5EN_A::ENABLE
    }
}
#[doc = "Field `PWM5EN` writer - PWM5 enable"]
pub type PWM5EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM5EN_A, 5>;
impl<'a> PWM5EN_W<'a> {
    #[doc = "CT16Bn_PWM5 is controlled by EM5"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM5EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM5"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM5EN_A::ENABLE)
    }
}
#[doc = "PWM6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM6EN_A {
    #[doc = "0: CT16Bn_PWM6 is controlled by EM6"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM6"]
    ENABLE = 1,
}
impl From<PWM6EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM6EN` reader - PWM6 enable"]
pub type PWM6EN_R = crate::BitReader<PWM6EN_A>;
impl PWM6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM6EN_A {
        match self.bits {
            false => PWM6EN_A::DISABLE,
            true => PWM6EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM6EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM6EN_A::ENABLE
    }
}
#[doc = "Field `PWM6EN` writer - PWM6 enable"]
pub type PWM6EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM6EN_A, 6>;
impl<'a> PWM6EN_W<'a> {
    #[doc = "CT16Bn_PWM6 is controlled by EM6"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM6EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM6"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM6EN_A::ENABLE)
    }
}
#[doc = "PWM7 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM7EN_A {
    #[doc = "0: CT16Bn_PWM7 is controlled by EM7"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM7"]
    ENABLE = 1,
}
impl From<PWM7EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM7EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM7EN` reader - PWM7 enable"]
pub type PWM7EN_R = crate::BitReader<PWM7EN_A>;
impl PWM7EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM7EN_A {
        match self.bits {
            false => PWM7EN_A::DISABLE,
            true => PWM7EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM7EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM7EN_A::ENABLE
    }
}
#[doc = "Field `PWM7EN` writer - PWM7 enable"]
pub type PWM7EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM7EN_A, 7>;
impl<'a> PWM7EN_W<'a> {
    #[doc = "CT16Bn_PWM7 is controlled by EM7"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM7EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM7"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM7EN_A::ENABLE)
    }
}
#[doc = "PWM8 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM8EN_A {
    #[doc = "0: CT16Bn_PWM8 is controlled by EM8"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM8"]
    ENABLE = 1,
}
impl From<PWM8EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM8EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM8EN` reader - PWM8 enable"]
pub type PWM8EN_R = crate::BitReader<PWM8EN_A>;
impl PWM8EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM8EN_A {
        match self.bits {
            false => PWM8EN_A::DISABLE,
            true => PWM8EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM8EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM8EN_A::ENABLE
    }
}
#[doc = "Field `PWM8EN` writer - PWM8 enable"]
pub type PWM8EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM8EN_A, 8>;
impl<'a> PWM8EN_W<'a> {
    #[doc = "CT16Bn_PWM8 is controlled by EM8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM8EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM8EN_A::ENABLE)
    }
}
#[doc = "PWM9 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM9EN_A {
    #[doc = "0: CT16Bn_PWM9 is controlled by EM9"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM9"]
    ENABLE = 1,
}
impl From<PWM9EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM9EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM9EN` reader - PWM9 enable"]
pub type PWM9EN_R = crate::BitReader<PWM9EN_A>;
impl PWM9EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM9EN_A {
        match self.bits {
            false => PWM9EN_A::DISABLE,
            true => PWM9EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM9EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM9EN_A::ENABLE
    }
}
#[doc = "Field `PWM9EN` writer - PWM9 enable"]
pub type PWM9EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM9EN_A, 9>;
impl<'a> PWM9EN_W<'a> {
    #[doc = "CT16Bn_PWM9 is controlled by EM9"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM9EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM9"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM9EN_A::ENABLE)
    }
}
#[doc = "PWM10 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM10EN_A {
    #[doc = "0: CT16Bn_PWM10 is controlled by EM10"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM10"]
    ENABLE = 1,
}
impl From<PWM10EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM10EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM10EN` reader - PWM10 enable"]
pub type PWM10EN_R = crate::BitReader<PWM10EN_A>;
impl PWM10EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM10EN_A {
        match self.bits {
            false => PWM10EN_A::DISABLE,
            true => PWM10EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM10EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM10EN_A::ENABLE
    }
}
#[doc = "Field `PWM10EN` writer - PWM10 enable"]
pub type PWM10EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM10EN_A, 10>;
impl<'a> PWM10EN_W<'a> {
    #[doc = "CT16Bn_PWM10 is controlled by EM10"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM10EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM10"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM10EN_A::ENABLE)
    }
}
#[doc = "PWM11 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM11EN_A {
    #[doc = "0: CT16Bn_PWM11 is controlled by EM11"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM11"]
    ENABLE = 1,
}
impl From<PWM11EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM11EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM11EN` reader - PWM11 enable"]
pub type PWM11EN_R = crate::BitReader<PWM11EN_A>;
impl PWM11EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM11EN_A {
        match self.bits {
            false => PWM11EN_A::DISABLE,
            true => PWM11EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM11EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM11EN_A::ENABLE
    }
}
#[doc = "Field `PWM11EN` writer - PWM11 enable"]
pub type PWM11EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM11EN_A, 11>;
impl<'a> PWM11EN_W<'a> {
    #[doc = "CT16Bn_PWM11 is controlled by EM11"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM11EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM11"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM11EN_A::ENABLE)
    }
}
#[doc = "PWM12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM12EN_A {
    #[doc = "0: CT16Bn_PWM12 is controlled by EM12"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM12"]
    ENABLE = 1,
}
impl From<PWM12EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM12EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM12EN` reader - PWM12 enable"]
pub type PWM12EN_R = crate::BitReader<PWM12EN_A>;
impl PWM12EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM12EN_A {
        match self.bits {
            false => PWM12EN_A::DISABLE,
            true => PWM12EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM12EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM12EN_A::ENABLE
    }
}
#[doc = "Field `PWM12EN` writer - PWM12 enable"]
pub type PWM12EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM12EN_A, 12>;
impl<'a> PWM12EN_W<'a> {
    #[doc = "CT16Bn_PWM12 is controlled by EM12"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM12EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM12"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM12EN_A::ENABLE)
    }
}
#[doc = "PWM13 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM13EN_A {
    #[doc = "0: CT16Bn_PWM13 is controlled by EM13"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM13"]
    ENABLE = 1,
}
impl From<PWM13EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM13EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM13EN` reader - PWM13 enable"]
pub type PWM13EN_R = crate::BitReader<PWM13EN_A>;
impl PWM13EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM13EN_A {
        match self.bits {
            false => PWM13EN_A::DISABLE,
            true => PWM13EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM13EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM13EN_A::ENABLE
    }
}
#[doc = "Field `PWM13EN` writer - PWM13 enable"]
pub type PWM13EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM13EN_A, 13>;
impl<'a> PWM13EN_W<'a> {
    #[doc = "CT16Bn_PWM13 is controlled by EM13"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM13EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM13"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM13EN_A::ENABLE)
    }
}
#[doc = "PWM14 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM14EN_A {
    #[doc = "0: CT16Bn_PWM14 is controlled by EM14"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM14"]
    ENABLE = 1,
}
impl From<PWM14EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM14EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM14EN` reader - PWM14 enable"]
pub type PWM14EN_R = crate::BitReader<PWM14EN_A>;
impl PWM14EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM14EN_A {
        match self.bits {
            false => PWM14EN_A::DISABLE,
            true => PWM14EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM14EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM14EN_A::ENABLE
    }
}
#[doc = "Field `PWM14EN` writer - PWM14 enable"]
pub type PWM14EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM14EN_A, 14>;
impl<'a> PWM14EN_W<'a> {
    #[doc = "CT16Bn_PWM14 is controlled by EM14"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM14EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM14"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM14EN_A::ENABLE)
    }
}
#[doc = "PWM15 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM15EN_A {
    #[doc = "0: CT16Bn_PWM15 is controlled by EM15"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM15"]
    ENABLE = 1,
}
impl From<PWM15EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM15EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM15EN` reader - PWM15 enable"]
pub type PWM15EN_R = crate::BitReader<PWM15EN_A>;
impl PWM15EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM15EN_A {
        match self.bits {
            false => PWM15EN_A::DISABLE,
            true => PWM15EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM15EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM15EN_A::ENABLE
    }
}
#[doc = "Field `PWM15EN` writer - PWM15 enable"]
pub type PWM15EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM15EN_A, 15>;
impl<'a> PWM15EN_W<'a> {
    #[doc = "CT16Bn_PWM15 is controlled by EM15"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM15EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM15"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM15EN_A::ENABLE)
    }
}
#[doc = "PWM16 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM16EN_A {
    #[doc = "0: CT16Bn_PWM16 is controlled by EM16"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM16"]
    ENABLE = 1,
}
impl From<PWM16EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM16EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM16EN` reader - PWM16 enable"]
pub type PWM16EN_R = crate::BitReader<PWM16EN_A>;
impl PWM16EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM16EN_A {
        match self.bits {
            false => PWM16EN_A::DISABLE,
            true => PWM16EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM16EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM16EN_A::ENABLE
    }
}
#[doc = "Field `PWM16EN` writer - PWM16 enable"]
pub type PWM16EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM16EN_A, 16>;
impl<'a> PWM16EN_W<'a> {
    #[doc = "CT16Bn_PWM16 is controlled by EM16"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM16EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM16"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM16EN_A::ENABLE)
    }
}
#[doc = "PWM17 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM17EN_A {
    #[doc = "0: CT16Bn_PWM17 is controlled by EM17"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM17"]
    ENABLE = 1,
}
impl From<PWM17EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM17EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM17EN` reader - PWM17 enable"]
pub type PWM17EN_R = crate::BitReader<PWM17EN_A>;
impl PWM17EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM17EN_A {
        match self.bits {
            false => PWM17EN_A::DISABLE,
            true => PWM17EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM17EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM17EN_A::ENABLE
    }
}
#[doc = "Field `PWM17EN` writer - PWM17 enable"]
pub type PWM17EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM17EN_A, 17>;
impl<'a> PWM17EN_W<'a> {
    #[doc = "CT16Bn_PWM17 is controlled by EM17"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM17EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM17"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM17EN_A::ENABLE)
    }
}
#[doc = "PWM18 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM18EN_A {
    #[doc = "0: CT16Bn_PWM18 is controlled by EM18"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM18"]
    ENABLE = 1,
}
impl From<PWM18EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM18EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM18EN` reader - PWM18 enable"]
pub type PWM18EN_R = crate::BitReader<PWM18EN_A>;
impl PWM18EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM18EN_A {
        match self.bits {
            false => PWM18EN_A::DISABLE,
            true => PWM18EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM18EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM18EN_A::ENABLE
    }
}
#[doc = "Field `PWM18EN` writer - PWM18 enable"]
pub type PWM18EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM18EN_A, 18>;
impl<'a> PWM18EN_W<'a> {
    #[doc = "CT16Bn_PWM18 is controlled by EM18"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM18EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM18"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM18EN_A::ENABLE)
    }
}
#[doc = "PWM19 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM19EN_A {
    #[doc = "0: CT16Bn_PWM19 is controlled by EM19"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM19"]
    ENABLE = 1,
}
impl From<PWM19EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM19EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM19EN` reader - PWM19 enable"]
pub type PWM19EN_R = crate::BitReader<PWM19EN_A>;
impl PWM19EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM19EN_A {
        match self.bits {
            false => PWM19EN_A::DISABLE,
            true => PWM19EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM19EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM19EN_A::ENABLE
    }
}
#[doc = "Field `PWM19EN` writer - PWM19 enable"]
pub type PWM19EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM19EN_A, 19>;
impl<'a> PWM19EN_W<'a> {
    #[doc = "CT16Bn_PWM19 is controlled by EM19"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM19EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM19"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM19EN_A::ENABLE)
    }
}
#[doc = "PWM20 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM20EN_A {
    #[doc = "0: CT16Bn_PWM20 is controlled by EM20"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM20"]
    ENABLE = 1,
}
impl From<PWM20EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM20EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM20EN` reader - PWM20 enable"]
pub type PWM20EN_R = crate::BitReader<PWM20EN_A>;
impl PWM20EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM20EN_A {
        match self.bits {
            false => PWM20EN_A::DISABLE,
            true => PWM20EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM20EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM20EN_A::ENABLE
    }
}
#[doc = "Field `PWM20EN` writer - PWM20 enable"]
pub type PWM20EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM20EN_A, 20>;
impl<'a> PWM20EN_W<'a> {
    #[doc = "CT16Bn_PWM20 is controlled by EM20"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM20EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM20"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM20EN_A::ENABLE)
    }
}
#[doc = "PWM21 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM21EN_A {
    #[doc = "0: CT16Bn_PWM21 is controlled by EM21"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM21"]
    ENABLE = 1,
}
impl From<PWM21EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM21EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM21EN` reader - PWM21 enable"]
pub type PWM21EN_R = crate::BitReader<PWM21EN_A>;
impl PWM21EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM21EN_A {
        match self.bits {
            false => PWM21EN_A::DISABLE,
            true => PWM21EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM21EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM21EN_A::ENABLE
    }
}
#[doc = "Field `PWM21EN` writer - PWM21 enable"]
pub type PWM21EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM21EN_A, 21>;
impl<'a> PWM21EN_W<'a> {
    #[doc = "CT16Bn_PWM21 is controlled by EM21"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM21EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM21"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM21EN_A::ENABLE)
    }
}
#[doc = "PWM22 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM22EN_A {
    #[doc = "0: CT16Bn_PWM22 is controlled by EM22"]
    DISABLE = 0,
    #[doc = "1: Enable PWM mode for CT16Bn_PWM22"]
    ENABLE = 1,
}
impl From<PWM22EN_A> for bool {
    #[inline(always)]
    fn from(variant: PWM22EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWM22EN` reader - PWM22 enable"]
pub type PWM22EN_R = crate::BitReader<PWM22EN_A>;
impl PWM22EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM22EN_A {
        match self.bits {
            false => PWM22EN_A::DISABLE,
            true => PWM22EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PWM22EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PWM22EN_A::ENABLE
    }
}
#[doc = "Field `PWM22EN` writer - PWM22 enable"]
pub type PWM22EN_W<'a> = crate::BitWriter<'a, u32, PWMENB_SPEC, PWM22EN_A, 22>;
impl<'a> PWM22EN_W<'a> {
    #[doc = "CT16Bn_PWM22 is controlled by EM22"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PWM22EN_A::DISABLE)
    }
    #[doc = "Enable PWM mode for CT16Bn_PWM22"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PWM22EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - PWM0 enable"]
    #[inline(always)]
    pub fn pwm0en(&self) -> PWM0EN_R {
        PWM0EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 enable"]
    #[inline(always)]
    pub fn pwm1en(&self) -> PWM1EN_R {
        PWM1EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 enable"]
    #[inline(always)]
    pub fn pwm2en(&self) -> PWM2EN_R {
        PWM2EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 enable"]
    #[inline(always)]
    pub fn pwm3en(&self) -> PWM3EN_R {
        PWM3EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 enable"]
    #[inline(always)]
    pub fn pwm4en(&self) -> PWM4EN_R {
        PWM4EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 enable"]
    #[inline(always)]
    pub fn pwm5en(&self) -> PWM5EN_R {
        PWM5EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 enable"]
    #[inline(always)]
    pub fn pwm6en(&self) -> PWM6EN_R {
        PWM6EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 enable"]
    #[inline(always)]
    pub fn pwm7en(&self) -> PWM7EN_R {
        PWM7EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM8 enable"]
    #[inline(always)]
    pub fn pwm8en(&self) -> PWM8EN_R {
        PWM8EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM9 enable"]
    #[inline(always)]
    pub fn pwm9en(&self) -> PWM9EN_R {
        PWM9EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM10 enable"]
    #[inline(always)]
    pub fn pwm10en(&self) -> PWM10EN_R {
        PWM10EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM11 enable"]
    #[inline(always)]
    pub fn pwm11en(&self) -> PWM11EN_R {
        PWM11EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM12 enable"]
    #[inline(always)]
    pub fn pwm12en(&self) -> PWM12EN_R {
        PWM12EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM13 enable"]
    #[inline(always)]
    pub fn pwm13en(&self) -> PWM13EN_R {
        PWM13EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM14 enable"]
    #[inline(always)]
    pub fn pwm14en(&self) -> PWM14EN_R {
        PWM14EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PWM15 enable"]
    #[inline(always)]
    pub fn pwm15en(&self) -> PWM15EN_R {
        PWM15EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PWM16 enable"]
    #[inline(always)]
    pub fn pwm16en(&self) -> PWM16EN_R {
        PWM16EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWM17 enable"]
    #[inline(always)]
    pub fn pwm17en(&self) -> PWM17EN_R {
        PWM17EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWM18 enable"]
    #[inline(always)]
    pub fn pwm18en(&self) -> PWM18EN_R {
        PWM18EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWM19 enable"]
    #[inline(always)]
    pub fn pwm19en(&self) -> PWM19EN_R {
        PWM19EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM20 enable"]
    #[inline(always)]
    pub fn pwm20en(&self) -> PWM20EN_R {
        PWM20EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM21 enable"]
    #[inline(always)]
    pub fn pwm21en(&self) -> PWM21EN_R {
        PWM21EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PWM22 enable"]
    #[inline(always)]
    pub fn pwm22en(&self) -> PWM22EN_R {
        PWM22EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 enable"]
    #[inline(always)]
    pub fn pwm0en(&mut self) -> PWM0EN_W {
        PWM0EN_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 enable"]
    #[inline(always)]
    pub fn pwm1en(&mut self) -> PWM1EN_W {
        PWM1EN_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 enable"]
    #[inline(always)]
    pub fn pwm2en(&mut self) -> PWM2EN_W {
        PWM2EN_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 enable"]
    #[inline(always)]
    pub fn pwm3en(&mut self) -> PWM3EN_W {
        PWM3EN_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 enable"]
    #[inline(always)]
    pub fn pwm4en(&mut self) -> PWM4EN_W {
        PWM4EN_W::new(self)
    }
    #[doc = "Bit 5 - PWM5 enable"]
    #[inline(always)]
    pub fn pwm5en(&mut self) -> PWM5EN_W {
        PWM5EN_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 enable"]
    #[inline(always)]
    pub fn pwm6en(&mut self) -> PWM6EN_W {
        PWM6EN_W::new(self)
    }
    #[doc = "Bit 7 - PWM7 enable"]
    #[inline(always)]
    pub fn pwm7en(&mut self) -> PWM7EN_W {
        PWM7EN_W::new(self)
    }
    #[doc = "Bit 8 - PWM8 enable"]
    #[inline(always)]
    pub fn pwm8en(&mut self) -> PWM8EN_W {
        PWM8EN_W::new(self)
    }
    #[doc = "Bit 9 - PWM9 enable"]
    #[inline(always)]
    pub fn pwm9en(&mut self) -> PWM9EN_W {
        PWM9EN_W::new(self)
    }
    #[doc = "Bit 10 - PWM10 enable"]
    #[inline(always)]
    pub fn pwm10en(&mut self) -> PWM10EN_W {
        PWM10EN_W::new(self)
    }
    #[doc = "Bit 11 - PWM11 enable"]
    #[inline(always)]
    pub fn pwm11en(&mut self) -> PWM11EN_W {
        PWM11EN_W::new(self)
    }
    #[doc = "Bit 12 - PWM12 enable"]
    #[inline(always)]
    pub fn pwm12en(&mut self) -> PWM12EN_W {
        PWM12EN_W::new(self)
    }
    #[doc = "Bit 13 - PWM13 enable"]
    #[inline(always)]
    pub fn pwm13en(&mut self) -> PWM13EN_W {
        PWM13EN_W::new(self)
    }
    #[doc = "Bit 14 - PWM14 enable"]
    #[inline(always)]
    pub fn pwm14en(&mut self) -> PWM14EN_W {
        PWM14EN_W::new(self)
    }
    #[doc = "Bit 15 - PWM15 enable"]
    #[inline(always)]
    pub fn pwm15en(&mut self) -> PWM15EN_W {
        PWM15EN_W::new(self)
    }
    #[doc = "Bit 16 - PWM16 enable"]
    #[inline(always)]
    pub fn pwm16en(&mut self) -> PWM16EN_W {
        PWM16EN_W::new(self)
    }
    #[doc = "Bit 17 - PWM17 enable"]
    #[inline(always)]
    pub fn pwm17en(&mut self) -> PWM17EN_W {
        PWM17EN_W::new(self)
    }
    #[doc = "Bit 18 - PWM18 enable"]
    #[inline(always)]
    pub fn pwm18en(&mut self) -> PWM18EN_W {
        PWM18EN_W::new(self)
    }
    #[doc = "Bit 19 - PWM19 enable"]
    #[inline(always)]
    pub fn pwm19en(&mut self) -> PWM19EN_W {
        PWM19EN_W::new(self)
    }
    #[doc = "Bit 20 - PWM20 enable"]
    #[inline(always)]
    pub fn pwm20en(&mut self) -> PWM20EN_W {
        PWM20EN_W::new(self)
    }
    #[doc = "Bit 21 - PWM21 enable"]
    #[inline(always)]
    pub fn pwm21en(&mut self) -> PWM21EN_W {
        PWM21EN_W::new(self)
    }
    #[doc = "Bit 22 - PWM22 enable"]
    #[inline(always)]
    pub fn pwm22en(&mut self) -> PWM22EN_W {
        PWM22EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x9C CT16Bn PWM Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmenb](index.html) module"]
pub struct PWMENB_SPEC;
impl crate::RegisterSpec for PWMENB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmenb::R](R) reader structure"]
impl crate::Readable for PWMENB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmenb::W](W) writer structure"]
impl crate::Writable for PWMENB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMENB to value 0"]
impl crate::Resettable for PWMENB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
