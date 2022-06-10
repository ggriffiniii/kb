#[doc = "Register `DATA` reader"]
pub struct R(crate::R<DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA` writer"]
pub struct W(crate::W<DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_SPEC>;
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
impl From<crate::W<DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Data of Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA0_A {
    #[doc = "0: Pn.0 is 0"]
    _0 = 0,
    #[doc = "1: Pn.0 is 1"]
    _1 = 1,
}
impl From<DATA0_A> for bool {
    #[inline(always)]
    fn from(variant: DATA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA0` reader - Data of Pn.0"]
pub type DATA0_R = crate::BitReader<DATA0_A>;
impl DATA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA0_A {
        match self.bits {
            false => DATA0_A::_0,
            true => DATA0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA0_A::_1
    }
}
#[doc = "Field `DATA0` writer - Data of Pn.0"]
pub type DATA0_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA0_A, 0>;
impl<'a> DATA0_W<'a> {
    #[doc = "Pn.0 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA0_A::_0)
    }
    #[doc = "Pn.0 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA0_A::_1)
    }
}
#[doc = "Data of Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA1_A {
    #[doc = "0: Pn.1 is 0"]
    _0 = 0,
    #[doc = "1: Pn.1 is 1"]
    _1 = 1,
}
impl From<DATA1_A> for bool {
    #[inline(always)]
    fn from(variant: DATA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA1` reader - Data of Pn.1"]
pub type DATA1_R = crate::BitReader<DATA1_A>;
impl DATA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA1_A {
        match self.bits {
            false => DATA1_A::_0,
            true => DATA1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA1_A::_1
    }
}
#[doc = "Field `DATA1` writer - Data of Pn.1"]
pub type DATA1_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA1_A, 1>;
impl<'a> DATA1_W<'a> {
    #[doc = "Pn.1 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA1_A::_0)
    }
    #[doc = "Pn.1 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA1_A::_1)
    }
}
#[doc = "Data of Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA2_A {
    #[doc = "0: Pn.2 is 0"]
    _0 = 0,
    #[doc = "1: Pn.2 is 1"]
    _1 = 1,
}
impl From<DATA2_A> for bool {
    #[inline(always)]
    fn from(variant: DATA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA2` reader - Data of Pn.2"]
pub type DATA2_R = crate::BitReader<DATA2_A>;
impl DATA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA2_A {
        match self.bits {
            false => DATA2_A::_0,
            true => DATA2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA2_A::_1
    }
}
#[doc = "Field `DATA2` writer - Data of Pn.2"]
pub type DATA2_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA2_A, 2>;
impl<'a> DATA2_W<'a> {
    #[doc = "Pn.2 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA2_A::_0)
    }
    #[doc = "Pn.2 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA2_A::_1)
    }
}
#[doc = "Data of Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA3_A {
    #[doc = "0: Pn.3 is 0"]
    _0 = 0,
    #[doc = "1: Pn.3 is 1"]
    _1 = 1,
}
impl From<DATA3_A> for bool {
    #[inline(always)]
    fn from(variant: DATA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA3` reader - Data of Pn.3"]
pub type DATA3_R = crate::BitReader<DATA3_A>;
impl DATA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA3_A {
        match self.bits {
            false => DATA3_A::_0,
            true => DATA3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA3_A::_1
    }
}
#[doc = "Field `DATA3` writer - Data of Pn.3"]
pub type DATA3_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA3_A, 3>;
impl<'a> DATA3_W<'a> {
    #[doc = "Pn.3 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA3_A::_0)
    }
    #[doc = "Pn.3 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA3_A::_1)
    }
}
#[doc = "Data of Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA4_A {
    #[doc = "0: Pn.4 is 0"]
    _0 = 0,
    #[doc = "1: Pn.4 is 1"]
    _1 = 1,
}
impl From<DATA4_A> for bool {
    #[inline(always)]
    fn from(variant: DATA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA4` reader - Data of Pn.4"]
pub type DATA4_R = crate::BitReader<DATA4_A>;
impl DATA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA4_A {
        match self.bits {
            false => DATA4_A::_0,
            true => DATA4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA4_A::_1
    }
}
#[doc = "Field `DATA4` writer - Data of Pn.4"]
pub type DATA4_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA4_A, 4>;
impl<'a> DATA4_W<'a> {
    #[doc = "Pn.4 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA4_A::_0)
    }
    #[doc = "Pn.4 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA4_A::_1)
    }
}
#[doc = "Data of Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA5_A {
    #[doc = "0: Pn.5 is 0"]
    _0 = 0,
    #[doc = "1: Pn.5 is 1"]
    _1 = 1,
}
impl From<DATA5_A> for bool {
    #[inline(always)]
    fn from(variant: DATA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA5` reader - Data of Pn.5"]
pub type DATA5_R = crate::BitReader<DATA5_A>;
impl DATA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA5_A {
        match self.bits {
            false => DATA5_A::_0,
            true => DATA5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA5_A::_1
    }
}
#[doc = "Field `DATA5` writer - Data of Pn.5"]
pub type DATA5_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA5_A, 5>;
impl<'a> DATA5_W<'a> {
    #[doc = "Pn.5 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA5_A::_0)
    }
    #[doc = "Pn.5 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA5_A::_1)
    }
}
#[doc = "Data of Pn.6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA6_A {
    #[doc = "0: Pn.6 is 0"]
    _0 = 0,
    #[doc = "1: Pn.6 is 1"]
    _1 = 1,
}
impl From<DATA6_A> for bool {
    #[inline(always)]
    fn from(variant: DATA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA6` reader - Data of Pn.6"]
pub type DATA6_R = crate::BitReader<DATA6_A>;
impl DATA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA6_A {
        match self.bits {
            false => DATA6_A::_0,
            true => DATA6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA6_A::_1
    }
}
#[doc = "Field `DATA6` writer - Data of Pn.6"]
pub type DATA6_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA6_A, 6>;
impl<'a> DATA6_W<'a> {
    #[doc = "Pn.6 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA6_A::_0)
    }
    #[doc = "Pn.6 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA6_A::_1)
    }
}
#[doc = "Data of Pn.7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA7_A {
    #[doc = "0: Pn.7 is 0"]
    _0 = 0,
    #[doc = "1: Pn.7 is 1"]
    _1 = 1,
}
impl From<DATA7_A> for bool {
    #[inline(always)]
    fn from(variant: DATA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA7` reader - Data of Pn.7"]
pub type DATA7_R = crate::BitReader<DATA7_A>;
impl DATA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA7_A {
        match self.bits {
            false => DATA7_A::_0,
            true => DATA7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA7_A::_1
    }
}
#[doc = "Field `DATA7` writer - Data of Pn.7"]
pub type DATA7_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA7_A, 7>;
impl<'a> DATA7_W<'a> {
    #[doc = "Pn.7 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA7_A::_0)
    }
    #[doc = "Pn.7 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA7_A::_1)
    }
}
#[doc = "Data of Pn.8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA8_A {
    #[doc = "0: Pn.8 is 0"]
    _0 = 0,
    #[doc = "1: Pn.8 is 1"]
    _1 = 1,
}
impl From<DATA8_A> for bool {
    #[inline(always)]
    fn from(variant: DATA8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA8` reader - Data of Pn.8"]
pub type DATA8_R = crate::BitReader<DATA8_A>;
impl DATA8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA8_A {
        match self.bits {
            false => DATA8_A::_0,
            true => DATA8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA8_A::_1
    }
}
#[doc = "Field `DATA8` writer - Data of Pn.8"]
pub type DATA8_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA8_A, 8>;
impl<'a> DATA8_W<'a> {
    #[doc = "Pn.8 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA8_A::_0)
    }
    #[doc = "Pn.8 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA8_A::_1)
    }
}
#[doc = "Data of Pn.9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA9_A {
    #[doc = "0: Pn.9 is 0"]
    _0 = 0,
    #[doc = "1: Pn.9 is 1"]
    _1 = 1,
}
impl From<DATA9_A> for bool {
    #[inline(always)]
    fn from(variant: DATA9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA9` reader - Data of Pn.9"]
pub type DATA9_R = crate::BitReader<DATA9_A>;
impl DATA9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA9_A {
        match self.bits {
            false => DATA9_A::_0,
            true => DATA9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA9_A::_1
    }
}
#[doc = "Field `DATA9` writer - Data of Pn.9"]
pub type DATA9_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA9_A, 9>;
impl<'a> DATA9_W<'a> {
    #[doc = "Pn.9 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA9_A::_0)
    }
    #[doc = "Pn.9 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA9_A::_1)
    }
}
#[doc = "Data of Pn.10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA10_A {
    #[doc = "0: Pn.10 is 0"]
    _0 = 0,
    #[doc = "1: Pn.10 is 1"]
    _1 = 1,
}
impl From<DATA10_A> for bool {
    #[inline(always)]
    fn from(variant: DATA10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA10` reader - Data of Pn.10"]
pub type DATA10_R = crate::BitReader<DATA10_A>;
impl DATA10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA10_A {
        match self.bits {
            false => DATA10_A::_0,
            true => DATA10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA10_A::_1
    }
}
#[doc = "Field `DATA10` writer - Data of Pn.10"]
pub type DATA10_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA10_A, 10>;
impl<'a> DATA10_W<'a> {
    #[doc = "Pn.10 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA10_A::_0)
    }
    #[doc = "Pn.10 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA10_A::_1)
    }
}
#[doc = "Data of Pn.11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA11_A {
    #[doc = "0: Pn.11 is 0"]
    _0 = 0,
    #[doc = "1: Pn.11 is 1"]
    _1 = 1,
}
impl From<DATA11_A> for bool {
    #[inline(always)]
    fn from(variant: DATA11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA11` reader - Data of Pn.11"]
pub type DATA11_R = crate::BitReader<DATA11_A>;
impl DATA11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA11_A {
        match self.bits {
            false => DATA11_A::_0,
            true => DATA11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA11_A::_1
    }
}
#[doc = "Field `DATA11` writer - Data of Pn.11"]
pub type DATA11_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA11_A, 11>;
impl<'a> DATA11_W<'a> {
    #[doc = "Pn.11 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA11_A::_0)
    }
    #[doc = "Pn.11 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA11_A::_1)
    }
}
#[doc = "Data of Pn.12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA12_A {
    #[doc = "0: Pn.12 is 0"]
    _0 = 0,
    #[doc = "1: Pn.12 is 1"]
    _1 = 1,
}
impl From<DATA12_A> for bool {
    #[inline(always)]
    fn from(variant: DATA12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA12` reader - Data of Pn.12"]
pub type DATA12_R = crate::BitReader<DATA12_A>;
impl DATA12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA12_A {
        match self.bits {
            false => DATA12_A::_0,
            true => DATA12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA12_A::_1
    }
}
#[doc = "Field `DATA12` writer - Data of Pn.12"]
pub type DATA12_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA12_A, 12>;
impl<'a> DATA12_W<'a> {
    #[doc = "Pn.12 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA12_A::_0)
    }
    #[doc = "Pn.12 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA12_A::_1)
    }
}
#[doc = "Data of Pn.13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA13_A {
    #[doc = "0: Pn.13 is 0"]
    _0 = 0,
    #[doc = "1: Pn.13 is 1"]
    _1 = 1,
}
impl From<DATA13_A> for bool {
    #[inline(always)]
    fn from(variant: DATA13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA13` reader - Data of Pn.13"]
pub type DATA13_R = crate::BitReader<DATA13_A>;
impl DATA13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA13_A {
        match self.bits {
            false => DATA13_A::_0,
            true => DATA13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA13_A::_1
    }
}
#[doc = "Field `DATA13` writer - Data of Pn.13"]
pub type DATA13_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA13_A, 13>;
impl<'a> DATA13_W<'a> {
    #[doc = "Pn.13 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA13_A::_0)
    }
    #[doc = "Pn.13 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA13_A::_1)
    }
}
#[doc = "Data of Pn.14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA14_A {
    #[doc = "0: Pn.14 is 0"]
    _0 = 0,
    #[doc = "1: Pn.14 is 1"]
    _1 = 1,
}
impl From<DATA14_A> for bool {
    #[inline(always)]
    fn from(variant: DATA14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA14` reader - Data of Pn.14"]
pub type DATA14_R = crate::BitReader<DATA14_A>;
impl DATA14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA14_A {
        match self.bits {
            false => DATA14_A::_0,
            true => DATA14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA14_A::_1
    }
}
#[doc = "Field `DATA14` writer - Data of Pn.14"]
pub type DATA14_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA14_A, 14>;
impl<'a> DATA14_W<'a> {
    #[doc = "Pn.14 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA14_A::_0)
    }
    #[doc = "Pn.14 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA14_A::_1)
    }
}
#[doc = "Data of Pn.15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA15_A {
    #[doc = "0: Pn.15 is 0"]
    _0 = 0,
    #[doc = "1: Pn.15 is 1"]
    _1 = 1,
}
impl From<DATA15_A> for bool {
    #[inline(always)]
    fn from(variant: DATA15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA15` reader - Data of Pn.15"]
pub type DATA15_R = crate::BitReader<DATA15_A>;
impl DATA15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA15_A {
        match self.bits {
            false => DATA15_A::_0,
            true => DATA15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DATA15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DATA15_A::_1
    }
}
#[doc = "Field `DATA15` writer - Data of Pn.15"]
pub type DATA15_W<'a> = crate::BitWriter<'a, u32, DATA_SPEC, DATA15_A, 15>;
impl<'a> DATA15_W<'a> {
    #[doc = "Pn.15 is 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATA15_A::_0)
    }
    #[doc = "Pn.15 is 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATA15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data of Pn.0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data of Pn.1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data of Pn.2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data of Pn.3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data of Pn.4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data of Pn.5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data of Pn.6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data of Pn.7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data of Pn.8"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data of Pn.9"]
    #[inline(always)]
    pub fn data9(&self) -> DATA9_R {
        DATA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data of Pn.10"]
    #[inline(always)]
    pub fn data10(&self) -> DATA10_R {
        DATA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data of Pn.11"]
    #[inline(always)]
    pub fn data11(&self) -> DATA11_R {
        DATA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data of Pn.12"]
    #[inline(always)]
    pub fn data12(&self) -> DATA12_R {
        DATA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data of Pn.13"]
    #[inline(always)]
    pub fn data13(&self) -> DATA13_R {
        DATA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data of Pn.14"]
    #[inline(always)]
    pub fn data14(&self) -> DATA14_R {
        DATA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data of Pn.15"]
    #[inline(always)]
    pub fn data15(&self) -> DATA15_R {
        DATA15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data of Pn.0"]
    #[inline(always)]
    pub fn data0(&mut self) -> DATA0_W {
        DATA0_W::new(self)
    }
    #[doc = "Bit 1 - Data of Pn.1"]
    #[inline(always)]
    pub fn data1(&mut self) -> DATA1_W {
        DATA1_W::new(self)
    }
    #[doc = "Bit 2 - Data of Pn.2"]
    #[inline(always)]
    pub fn data2(&mut self) -> DATA2_W {
        DATA2_W::new(self)
    }
    #[doc = "Bit 3 - Data of Pn.3"]
    #[inline(always)]
    pub fn data3(&mut self) -> DATA3_W {
        DATA3_W::new(self)
    }
    #[doc = "Bit 4 - Data of Pn.4"]
    #[inline(always)]
    pub fn data4(&mut self) -> DATA4_W {
        DATA4_W::new(self)
    }
    #[doc = "Bit 5 - Data of Pn.5"]
    #[inline(always)]
    pub fn data5(&mut self) -> DATA5_W {
        DATA5_W::new(self)
    }
    #[doc = "Bit 6 - Data of Pn.6"]
    #[inline(always)]
    pub fn data6(&mut self) -> DATA6_W {
        DATA6_W::new(self)
    }
    #[doc = "Bit 7 - Data of Pn.7"]
    #[inline(always)]
    pub fn data7(&mut self) -> DATA7_W {
        DATA7_W::new(self)
    }
    #[doc = "Bit 8 - Data of Pn.8"]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W {
        DATA8_W::new(self)
    }
    #[doc = "Bit 9 - Data of Pn.9"]
    #[inline(always)]
    pub fn data9(&mut self) -> DATA9_W {
        DATA9_W::new(self)
    }
    #[doc = "Bit 10 - Data of Pn.10"]
    #[inline(always)]
    pub fn data10(&mut self) -> DATA10_W {
        DATA10_W::new(self)
    }
    #[doc = "Bit 11 - Data of Pn.11"]
    #[inline(always)]
    pub fn data11(&mut self) -> DATA11_W {
        DATA11_W::new(self)
    }
    #[doc = "Bit 12 - Data of Pn.12"]
    #[inline(always)]
    pub fn data12(&mut self) -> DATA12_W {
        DATA12_W::new(self)
    }
    #[doc = "Bit 13 - Data of Pn.13"]
    #[inline(always)]
    pub fn data13(&mut self) -> DATA13_W {
        DATA13_W::new(self)
    }
    #[doc = "Bit 14 - Data of Pn.14"]
    #[inline(always)]
    pub fn data14(&mut self) -> DATA14_W {
        DATA14_W::new(self)
    }
    #[doc = "Bit 15 - Data of Pn.15"]
    #[inline(always)]
    pub fn data15(&mut self) -> DATA15_W {
        DATA15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 GPIO Port n Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](index.html) module"]
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data::R](R) reader structure"]
impl crate::Readable for DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data::W](W) writer structure"]
impl crate::Writable for DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
