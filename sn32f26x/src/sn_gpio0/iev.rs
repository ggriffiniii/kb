#[doc = "Register `IEV` reader"]
pub struct R(crate::R<IEV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEV` writer"]
pub struct W(crate::W<IEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEV_SPEC>;
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
impl From<crate::W<IEV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt trigged evnet on Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV0_A {
    #[doc = "0: Rising edge or High level on Pn.0 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.0 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV0_A> for bool {
    #[inline(always)]
    fn from(variant: IEV0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV0` reader - Interrupt trigged evnet on Pn.0"]
pub type IEV0_R = crate::BitReader<IEV0_A>;
impl IEV0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV0_A {
        match self.bits {
            false => IEV0_A::_0,
            true => IEV0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV0_A::_1
    }
}
#[doc = "Field `IEV0` writer - Interrupt trigged evnet on Pn.0"]
pub type IEV0_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV0_A, 0>;
impl<'a> IEV0_W<'a> {
    #[doc = "Rising edge or High level on Pn.0 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV0_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.0 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV0_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV1_A {
    #[doc = "0: Rising edge or High level on Pn.1 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.1 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV1_A> for bool {
    #[inline(always)]
    fn from(variant: IEV1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV1` reader - Interrupt trigged evnet on Pn.1"]
pub type IEV1_R = crate::BitReader<IEV1_A>;
impl IEV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV1_A {
        match self.bits {
            false => IEV1_A::_0,
            true => IEV1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV1_A::_1
    }
}
#[doc = "Field `IEV1` writer - Interrupt trigged evnet on Pn.1"]
pub type IEV1_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV1_A, 1>;
impl<'a> IEV1_W<'a> {
    #[doc = "Rising edge or High level on Pn.1 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV1_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.1 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV1_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV2_A {
    #[doc = "0: Rising edge or High level on Pn.2 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.2 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV2_A> for bool {
    #[inline(always)]
    fn from(variant: IEV2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV2` reader - Interrupt trigged evnet on Pn.2"]
pub type IEV2_R = crate::BitReader<IEV2_A>;
impl IEV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV2_A {
        match self.bits {
            false => IEV2_A::_0,
            true => IEV2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV2_A::_1
    }
}
#[doc = "Field `IEV2` writer - Interrupt trigged evnet on Pn.2"]
pub type IEV2_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV2_A, 2>;
impl<'a> IEV2_W<'a> {
    #[doc = "Rising edge or High level on Pn.2 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV2_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.2 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV2_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV3_A {
    #[doc = "0: Rising edge or High level on Pn.3 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.3 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV3_A> for bool {
    #[inline(always)]
    fn from(variant: IEV3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV3` reader - Interrupt trigged evnet on Pn.3"]
pub type IEV3_R = crate::BitReader<IEV3_A>;
impl IEV3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV3_A {
        match self.bits {
            false => IEV3_A::_0,
            true => IEV3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV3_A::_1
    }
}
#[doc = "Field `IEV3` writer - Interrupt trigged evnet on Pn.3"]
pub type IEV3_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV3_A, 3>;
impl<'a> IEV3_W<'a> {
    #[doc = "Rising edge or High level on Pn.3 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV3_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.3 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV3_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV4_A {
    #[doc = "0: Rising edge or High level on Pn.4 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.4 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV4_A> for bool {
    #[inline(always)]
    fn from(variant: IEV4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV4` reader - Interrupt trigged evnet on Pn.4"]
pub type IEV4_R = crate::BitReader<IEV4_A>;
impl IEV4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV4_A {
        match self.bits {
            false => IEV4_A::_0,
            true => IEV4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV4_A::_1
    }
}
#[doc = "Field `IEV4` writer - Interrupt trigged evnet on Pn.4"]
pub type IEV4_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV4_A, 4>;
impl<'a> IEV4_W<'a> {
    #[doc = "Rising edge or High level on Pn.4 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV4_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.4 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV4_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV5_A {
    #[doc = "0: Rising edge or High level on Pn.5 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.5 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV5_A> for bool {
    #[inline(always)]
    fn from(variant: IEV5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV5` reader - Interrupt trigged evnet on Pn.5"]
pub type IEV5_R = crate::BitReader<IEV5_A>;
impl IEV5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV5_A {
        match self.bits {
            false => IEV5_A::_0,
            true => IEV5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV5_A::_1
    }
}
#[doc = "Field `IEV5` writer - Interrupt trigged evnet on Pn.5"]
pub type IEV5_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV5_A, 5>;
impl<'a> IEV5_W<'a> {
    #[doc = "Rising edge or High level on Pn.5 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV5_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.5 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV5_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV6_A {
    #[doc = "0: Rising edge or High level on Pn.6 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.6 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV6_A> for bool {
    #[inline(always)]
    fn from(variant: IEV6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV6` reader - Interrupt trigged evnet on Pn.6"]
pub type IEV6_R = crate::BitReader<IEV6_A>;
impl IEV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV6_A {
        match self.bits {
            false => IEV6_A::_0,
            true => IEV6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV6_A::_1
    }
}
#[doc = "Field `IEV6` writer - Interrupt trigged evnet on Pn.6"]
pub type IEV6_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV6_A, 6>;
impl<'a> IEV6_W<'a> {
    #[doc = "Rising edge or High level on Pn.6 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV6_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.6 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV6_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV7_A {
    #[doc = "0: Rising edge or High level on Pn.7 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.7 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV7_A> for bool {
    #[inline(always)]
    fn from(variant: IEV7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV7` reader - Interrupt trigged evnet on Pn.7"]
pub type IEV7_R = crate::BitReader<IEV7_A>;
impl IEV7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV7_A {
        match self.bits {
            false => IEV7_A::_0,
            true => IEV7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV7_A::_1
    }
}
#[doc = "Field `IEV7` writer - Interrupt trigged evnet on Pn.7"]
pub type IEV7_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV7_A, 7>;
impl<'a> IEV7_W<'a> {
    #[doc = "Rising edge or High level on Pn.7 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV7_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.7 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV7_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV8_A {
    #[doc = "0: Rising edge or High level on Pn.8 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.8 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV8_A> for bool {
    #[inline(always)]
    fn from(variant: IEV8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV8` reader - Interrupt trigged evnet on Pn.8"]
pub type IEV8_R = crate::BitReader<IEV8_A>;
impl IEV8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV8_A {
        match self.bits {
            false => IEV8_A::_0,
            true => IEV8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV8_A::_1
    }
}
#[doc = "Field `IEV8` writer - Interrupt trigged evnet on Pn.8"]
pub type IEV8_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV8_A, 8>;
impl<'a> IEV8_W<'a> {
    #[doc = "Rising edge or High level on Pn.8 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV8_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.8 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV8_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV9_A {
    #[doc = "0: Rising edge or High level on Pn.9 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.9 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV9_A> for bool {
    #[inline(always)]
    fn from(variant: IEV9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV9` reader - Interrupt trigged evnet on Pn.9"]
pub type IEV9_R = crate::BitReader<IEV9_A>;
impl IEV9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV9_A {
        match self.bits {
            false => IEV9_A::_0,
            true => IEV9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV9_A::_1
    }
}
#[doc = "Field `IEV9` writer - Interrupt trigged evnet on Pn.9"]
pub type IEV9_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV9_A, 9>;
impl<'a> IEV9_W<'a> {
    #[doc = "Rising edge or High level on Pn.9 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV9_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.9 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV9_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV10_A {
    #[doc = "0: Rising edge or High level on Pn.10 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.10 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV10_A> for bool {
    #[inline(always)]
    fn from(variant: IEV10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV10` reader - Interrupt trigged evnet on Pn.10"]
pub type IEV10_R = crate::BitReader<IEV10_A>;
impl IEV10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV10_A {
        match self.bits {
            false => IEV10_A::_0,
            true => IEV10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV10_A::_1
    }
}
#[doc = "Field `IEV10` writer - Interrupt trigged evnet on Pn.10"]
pub type IEV10_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV10_A, 10>;
impl<'a> IEV10_W<'a> {
    #[doc = "Rising edge or High level on Pn.10 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV10_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.10 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV10_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV11_A {
    #[doc = "0: Rising edge or High level on Pn.11 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.11 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV11_A> for bool {
    #[inline(always)]
    fn from(variant: IEV11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV11` reader - Interrupt trigged evnet on Pn.11"]
pub type IEV11_R = crate::BitReader<IEV11_A>;
impl IEV11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV11_A {
        match self.bits {
            false => IEV11_A::_0,
            true => IEV11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV11_A::_1
    }
}
#[doc = "Field `IEV11` writer - Interrupt trigged evnet on Pn.11"]
pub type IEV11_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV11_A, 11>;
impl<'a> IEV11_W<'a> {
    #[doc = "Rising edge or High level on Pn.11 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV11_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.11 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV11_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV12_A {
    #[doc = "0: Rising edge or High level on Pn.12 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.12 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV12_A> for bool {
    #[inline(always)]
    fn from(variant: IEV12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV12` reader - Interrupt trigged evnet on Pn.12"]
pub type IEV12_R = crate::BitReader<IEV12_A>;
impl IEV12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV12_A {
        match self.bits {
            false => IEV12_A::_0,
            true => IEV12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV12_A::_1
    }
}
#[doc = "Field `IEV12` writer - Interrupt trigged evnet on Pn.12"]
pub type IEV12_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV12_A, 12>;
impl<'a> IEV12_W<'a> {
    #[doc = "Rising edge or High level on Pn.12 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV12_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.12 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV12_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV13_A {
    #[doc = "0: Rising edge or High level on Pn.13 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.13 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV13_A> for bool {
    #[inline(always)]
    fn from(variant: IEV13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV13` reader - Interrupt trigged evnet on Pn.13"]
pub type IEV13_R = crate::BitReader<IEV13_A>;
impl IEV13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV13_A {
        match self.bits {
            false => IEV13_A::_0,
            true => IEV13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV13_A::_1
    }
}
#[doc = "Field `IEV13` writer - Interrupt trigged evnet on Pn.13"]
pub type IEV13_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV13_A, 13>;
impl<'a> IEV13_W<'a> {
    #[doc = "Rising edge or High level on Pn.13 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV13_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.13 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV13_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV14_A {
    #[doc = "0: Rising edge or High level on Pn.14 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.14 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV14_A> for bool {
    #[inline(always)]
    fn from(variant: IEV14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV14` reader - Interrupt trigged evnet on Pn.14"]
pub type IEV14_R = crate::BitReader<IEV14_A>;
impl IEV14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV14_A {
        match self.bits {
            false => IEV14_A::_0,
            true => IEV14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV14_A::_1
    }
}
#[doc = "Field `IEV14` writer - Interrupt trigged evnet on Pn.14"]
pub type IEV14_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV14_A, 14>;
impl<'a> IEV14_W<'a> {
    #[doc = "Rising edge or High level on Pn.14 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV14_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.14 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV14_A::_1)
    }
}
#[doc = "Interrupt trigged evnet on Pn.15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IEV15_A {
    #[doc = "0: Rising edge or High level on Pn.15 triggers an interrupt"]
    _0 = 0,
    #[doc = "1: Falling edge or Low level on Pn.15 triggers an interrupt"]
    _1 = 1,
}
impl From<IEV15_A> for bool {
    #[inline(always)]
    fn from(variant: IEV15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEV15` reader - Interrupt trigged evnet on Pn.15"]
pub type IEV15_R = crate::BitReader<IEV15_A>;
impl IEV15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEV15_A {
        match self.bits {
            false => IEV15_A::_0,
            true => IEV15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEV15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEV15_A::_1
    }
}
#[doc = "Field `IEV15` writer - Interrupt trigged evnet on Pn.15"]
pub type IEV15_W<'a> = crate::BitWriter<'a, u32, IEV_SPEC, IEV15_A, 15>;
impl<'a> IEV15_W<'a> {
    #[doc = "Rising edge or High level on Pn.15 triggers an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEV15_A::_0)
    }
    #[doc = "Falling edge or Low level on Pn.15 triggers an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEV15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt trigged evnet on Pn.0"]
    #[inline(always)]
    pub fn iev0(&self) -> IEV0_R {
        IEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt trigged evnet on Pn.1"]
    #[inline(always)]
    pub fn iev1(&self) -> IEV1_R {
        IEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt trigged evnet on Pn.2"]
    #[inline(always)]
    pub fn iev2(&self) -> IEV2_R {
        IEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt trigged evnet on Pn.3"]
    #[inline(always)]
    pub fn iev3(&self) -> IEV3_R {
        IEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt trigged evnet on Pn.4"]
    #[inline(always)]
    pub fn iev4(&self) -> IEV4_R {
        IEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt trigged evnet on Pn.5"]
    #[inline(always)]
    pub fn iev5(&self) -> IEV5_R {
        IEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt trigged evnet on Pn.6"]
    #[inline(always)]
    pub fn iev6(&self) -> IEV6_R {
        IEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt trigged evnet on Pn.7"]
    #[inline(always)]
    pub fn iev7(&self) -> IEV7_R {
        IEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt trigged evnet on Pn.8"]
    #[inline(always)]
    pub fn iev8(&self) -> IEV8_R {
        IEV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt trigged evnet on Pn.9"]
    #[inline(always)]
    pub fn iev9(&self) -> IEV9_R {
        IEV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt trigged evnet on Pn.10"]
    #[inline(always)]
    pub fn iev10(&self) -> IEV10_R {
        IEV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt trigged evnet on Pn.11"]
    #[inline(always)]
    pub fn iev11(&self) -> IEV11_R {
        IEV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt trigged evnet on Pn.12"]
    #[inline(always)]
    pub fn iev12(&self) -> IEV12_R {
        IEV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt trigged evnet on Pn.13"]
    #[inline(always)]
    pub fn iev13(&self) -> IEV13_R {
        IEV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt trigged evnet on Pn.14"]
    #[inline(always)]
    pub fn iev14(&self) -> IEV14_R {
        IEV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt trigged evnet on Pn.15"]
    #[inline(always)]
    pub fn iev15(&self) -> IEV15_R {
        IEV15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt trigged evnet on Pn.0"]
    #[inline(always)]
    pub fn iev0(&mut self) -> IEV0_W {
        IEV0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt trigged evnet on Pn.1"]
    #[inline(always)]
    pub fn iev1(&mut self) -> IEV1_W {
        IEV1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt trigged evnet on Pn.2"]
    #[inline(always)]
    pub fn iev2(&mut self) -> IEV2_W {
        IEV2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt trigged evnet on Pn.3"]
    #[inline(always)]
    pub fn iev3(&mut self) -> IEV3_W {
        IEV3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt trigged evnet on Pn.4"]
    #[inline(always)]
    pub fn iev4(&mut self) -> IEV4_W {
        IEV4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt trigged evnet on Pn.5"]
    #[inline(always)]
    pub fn iev5(&mut self) -> IEV5_W {
        IEV5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt trigged evnet on Pn.6"]
    #[inline(always)]
    pub fn iev6(&mut self) -> IEV6_W {
        IEV6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt trigged evnet on Pn.7"]
    #[inline(always)]
    pub fn iev7(&mut self) -> IEV7_W {
        IEV7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt trigged evnet on Pn.8"]
    #[inline(always)]
    pub fn iev8(&mut self) -> IEV8_W {
        IEV8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt trigged evnet on Pn.9"]
    #[inline(always)]
    pub fn iev9(&mut self) -> IEV9_W {
        IEV9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt trigged evnet on Pn.10"]
    #[inline(always)]
    pub fn iev10(&mut self) -> IEV10_W {
        IEV10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt trigged evnet on Pn.11"]
    #[inline(always)]
    pub fn iev11(&mut self) -> IEV11_W {
        IEV11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt trigged evnet on Pn.12"]
    #[inline(always)]
    pub fn iev12(&mut self) -> IEV12_W {
        IEV12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt trigged evnet on Pn.13"]
    #[inline(always)]
    pub fn iev13(&mut self) -> IEV13_W {
        IEV13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt trigged evnet on Pn.14"]
    #[inline(always)]
    pub fn iev14(&mut self) -> IEV14_W {
        IEV14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt trigged evnet on Pn.15"]
    #[inline(always)]
    pub fn iev15(&mut self) -> IEV15_W {
        IEV15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x14 GPIO Port n Interrupt Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iev](index.html) module"]
pub struct IEV_SPEC;
impl crate::RegisterSpec for IEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iev::R](R) reader structure"]
impl crate::Readable for IEV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iev::W](W) writer structure"]
impl crate::Writable for IEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
