#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mode of Pn.0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE0_A {
    #[doc = "0: Pn.0 is Input pin"]
    I = 0,
    #[doc = "1: Pn.0 is Output pin"]
    O = 1,
}
impl From<MODE0_A> for bool {
    #[inline(always)]
    fn from(variant: MODE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE0` reader - Mode of Pn.0"]
pub type MODE0_R = crate::BitReader<MODE0_A>;
impl MODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE0_A {
        match self.bits {
            false => MODE0_A::I,
            true => MODE0_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE0_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE0_A::O
    }
}
#[doc = "Field `MODE0` writer - Mode of Pn.0"]
pub type MODE0_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE0_A, 0>;
impl<'a> MODE0_W<'a> {
    #[doc = "Pn.0 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE0_A::I)
    }
    #[doc = "Pn.0 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE0_A::O)
    }
}
#[doc = "Mode of Pn.1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE1_A {
    #[doc = "0: Pn.1 is Input pin"]
    I = 0,
    #[doc = "1: Pn.1 is Output pin"]
    O = 1,
}
impl From<MODE1_A> for bool {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE1` reader - Mode of Pn.1"]
pub type MODE1_R = crate::BitReader<MODE1_A>;
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            false => MODE1_A::I,
            true => MODE1_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE1_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE1_A::O
    }
}
#[doc = "Field `MODE1` writer - Mode of Pn.1"]
pub type MODE1_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE1_A, 1>;
impl<'a> MODE1_W<'a> {
    #[doc = "Pn.1 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE1_A::I)
    }
    #[doc = "Pn.1 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE1_A::O)
    }
}
#[doc = "Mode of Pn.2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE2_A {
    #[doc = "0: Pn.2 is Input pin"]
    I = 0,
    #[doc = "1: Pn.2 is Output pin"]
    O = 1,
}
impl From<MODE2_A> for bool {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE2` reader - Mode of Pn.2"]
pub type MODE2_R = crate::BitReader<MODE2_A>;
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            false => MODE2_A::I,
            true => MODE2_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE2_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE2_A::O
    }
}
#[doc = "Field `MODE2` writer - Mode of Pn.2"]
pub type MODE2_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE2_A, 2>;
impl<'a> MODE2_W<'a> {
    #[doc = "Pn.2 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE2_A::I)
    }
    #[doc = "Pn.2 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE2_A::O)
    }
}
#[doc = "Mode of Pn.3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE3_A {
    #[doc = "0: Pn.3 is Input pin"]
    I = 0,
    #[doc = "1: Pn.3 is Output pin"]
    O = 1,
}
impl From<MODE3_A> for bool {
    #[inline(always)]
    fn from(variant: MODE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE3` reader - Mode of Pn.3"]
pub type MODE3_R = crate::BitReader<MODE3_A>;
impl MODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE3_A {
        match self.bits {
            false => MODE3_A::I,
            true => MODE3_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE3_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE3_A::O
    }
}
#[doc = "Field `MODE3` writer - Mode of Pn.3"]
pub type MODE3_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE3_A, 3>;
impl<'a> MODE3_W<'a> {
    #[doc = "Pn.3 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE3_A::I)
    }
    #[doc = "Pn.3 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE3_A::O)
    }
}
#[doc = "Mode of Pn.4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE4_A {
    #[doc = "0: Pn.4 is Input pin"]
    I = 0,
    #[doc = "1: Pn.4 is Output pin"]
    O = 1,
}
impl From<MODE4_A> for bool {
    #[inline(always)]
    fn from(variant: MODE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE4` reader - Mode of Pn.4"]
pub type MODE4_R = crate::BitReader<MODE4_A>;
impl MODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE4_A {
        match self.bits {
            false => MODE4_A::I,
            true => MODE4_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE4_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE4_A::O
    }
}
#[doc = "Field `MODE4` writer - Mode of Pn.4"]
pub type MODE4_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE4_A, 4>;
impl<'a> MODE4_W<'a> {
    #[doc = "Pn.4 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE4_A::I)
    }
    #[doc = "Pn.4 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE4_A::O)
    }
}
#[doc = "Mode of Pn.5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE5_A {
    #[doc = "0: Pn.5 is Input pin"]
    I = 0,
    #[doc = "1: Pn.5 is Output pin"]
    O = 1,
}
impl From<MODE5_A> for bool {
    #[inline(always)]
    fn from(variant: MODE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE5` reader - Mode of Pn.5"]
pub type MODE5_R = crate::BitReader<MODE5_A>;
impl MODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE5_A {
        match self.bits {
            false => MODE5_A::I,
            true => MODE5_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE5_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE5_A::O
    }
}
#[doc = "Field `MODE5` writer - Mode of Pn.5"]
pub type MODE5_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE5_A, 5>;
impl<'a> MODE5_W<'a> {
    #[doc = "Pn.5 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE5_A::I)
    }
    #[doc = "Pn.5 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE5_A::O)
    }
}
#[doc = "Mode of Pn.6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE6_A {
    #[doc = "0: Pn.6 is Input pin"]
    I = 0,
    #[doc = "1: Pn.6 is Output pin"]
    O = 1,
}
impl From<MODE6_A> for bool {
    #[inline(always)]
    fn from(variant: MODE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE6` reader - Mode of Pn.6"]
pub type MODE6_R = crate::BitReader<MODE6_A>;
impl MODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE6_A {
        match self.bits {
            false => MODE6_A::I,
            true => MODE6_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE6_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE6_A::O
    }
}
#[doc = "Field `MODE6` writer - Mode of Pn.6"]
pub type MODE6_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE6_A, 6>;
impl<'a> MODE6_W<'a> {
    #[doc = "Pn.6 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE6_A::I)
    }
    #[doc = "Pn.6 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE6_A::O)
    }
}
#[doc = "Mode of Pn.7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE7_A {
    #[doc = "0: Pn.7 is Input pin"]
    I = 0,
    #[doc = "1: Pn.7 is Output pin"]
    O = 1,
}
impl From<MODE7_A> for bool {
    #[inline(always)]
    fn from(variant: MODE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE7` reader - Mode of Pn.7"]
pub type MODE7_R = crate::BitReader<MODE7_A>;
impl MODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE7_A {
        match self.bits {
            false => MODE7_A::I,
            true => MODE7_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE7_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE7_A::O
    }
}
#[doc = "Field `MODE7` writer - Mode of Pn.7"]
pub type MODE7_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE7_A, 7>;
impl<'a> MODE7_W<'a> {
    #[doc = "Pn.7 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE7_A::I)
    }
    #[doc = "Pn.7 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE7_A::O)
    }
}
#[doc = "Mode of Pn.8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE8_A {
    #[doc = "0: Pn.8 is Input pin"]
    I = 0,
    #[doc = "1: Pn.8 is Output pin"]
    O = 1,
}
impl From<MODE8_A> for bool {
    #[inline(always)]
    fn from(variant: MODE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE8` reader - Mode of Pn.8"]
pub type MODE8_R = crate::BitReader<MODE8_A>;
impl MODE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE8_A {
        match self.bits {
            false => MODE8_A::I,
            true => MODE8_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE8_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE8_A::O
    }
}
#[doc = "Field `MODE8` writer - Mode of Pn.8"]
pub type MODE8_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE8_A, 8>;
impl<'a> MODE8_W<'a> {
    #[doc = "Pn.8 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE8_A::I)
    }
    #[doc = "Pn.8 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE8_A::O)
    }
}
#[doc = "Mode of Pn.9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE9_A {
    #[doc = "0: Pn.9 is Input pin"]
    I = 0,
    #[doc = "1: Pn.9 is Output pin"]
    O = 1,
}
impl From<MODE9_A> for bool {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE9` reader - Mode of Pn.9"]
pub type MODE9_R = crate::BitReader<MODE9_A>;
impl MODE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9_A {
        match self.bits {
            false => MODE9_A::I,
            true => MODE9_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE9_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE9_A::O
    }
}
#[doc = "Field `MODE9` writer - Mode of Pn.9"]
pub type MODE9_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE9_A, 9>;
impl<'a> MODE9_W<'a> {
    #[doc = "Pn.9 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE9_A::I)
    }
    #[doc = "Pn.9 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE9_A::O)
    }
}
#[doc = "Mode of Pn.10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE10_A {
    #[doc = "0: Pn.10 is Input pin"]
    I = 0,
    #[doc = "1: Pn.10 is Output pin"]
    O = 1,
}
impl From<MODE10_A> for bool {
    #[inline(always)]
    fn from(variant: MODE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE10` reader - Mode of Pn.10"]
pub type MODE10_R = crate::BitReader<MODE10_A>;
impl MODE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE10_A {
        match self.bits {
            false => MODE10_A::I,
            true => MODE10_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE10_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE10_A::O
    }
}
#[doc = "Field `MODE10` writer - Mode of Pn.10"]
pub type MODE10_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE10_A, 10>;
impl<'a> MODE10_W<'a> {
    #[doc = "Pn.10 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE10_A::I)
    }
    #[doc = "Pn.10 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE10_A::O)
    }
}
#[doc = "Mode of Pn.11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE11_A {
    #[doc = "0: Pn.11 is Input pin"]
    I = 0,
    #[doc = "1: Pn.11 is Output pin"]
    O = 1,
}
impl From<MODE11_A> for bool {
    #[inline(always)]
    fn from(variant: MODE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE11` reader - Mode of Pn.11"]
pub type MODE11_R = crate::BitReader<MODE11_A>;
impl MODE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE11_A {
        match self.bits {
            false => MODE11_A::I,
            true => MODE11_A::O,
        }
    }
    #[doc = "Checks if the value of the field is `I`"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == MODE11_A::I
    }
    #[doc = "Checks if the value of the field is `O`"]
    #[inline(always)]
    pub fn is_o(&self) -> bool {
        *self == MODE11_A::O
    }
}
#[doc = "Field `MODE11` writer - Mode of Pn.11"]
pub type MODE11_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE11_A, 11>;
impl<'a> MODE11_W<'a> {
    #[doc = "Pn.11 is Input pin"]
    #[inline(always)]
    pub fn i(self) -> &'a mut W {
        self.variant(MODE11_A::I)
    }
    #[doc = "Pn.11 is Output pin"]
    #[inline(always)]
    pub fn o(self) -> &'a mut W {
        self.variant(MODE11_A::O)
    }
}
#[doc = "Mode of Pn.12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE12_A {
    #[doc = "0: Pn.12 is Input pin"]
    _0 = 0,
    #[doc = "1: Pn.12 is Output pin"]
    _1 = 1,
}
impl From<MODE12_A> for bool {
    #[inline(always)]
    fn from(variant: MODE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE12` reader - Mode of Pn.12"]
pub type MODE12_R = crate::BitReader<MODE12_A>;
impl MODE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE12_A {
        match self.bits {
            false => MODE12_A::_0,
            true => MODE12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE12_A::_1
    }
}
#[doc = "Field `MODE12` writer - Mode of Pn.12"]
pub type MODE12_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE12_A, 12>;
impl<'a> MODE12_W<'a> {
    #[doc = "Pn.12 is Input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE12_A::_0)
    }
    #[doc = "Pn.12 is Output pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE12_A::_1)
    }
}
#[doc = "Mode of Pn.13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE13_A {
    #[doc = "0: Pn.13 is Input pin"]
    _0 = 0,
    #[doc = "1: Pn.13 is Output pin"]
    _1 = 1,
}
impl From<MODE13_A> for bool {
    #[inline(always)]
    fn from(variant: MODE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE13` reader - Mode of Pn.13"]
pub type MODE13_R = crate::BitReader<MODE13_A>;
impl MODE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE13_A {
        match self.bits {
            false => MODE13_A::_0,
            true => MODE13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE13_A::_1
    }
}
#[doc = "Field `MODE13` writer - Mode of Pn.13"]
pub type MODE13_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE13_A, 13>;
impl<'a> MODE13_W<'a> {
    #[doc = "Pn.13 is Input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE13_A::_0)
    }
    #[doc = "Pn.13 is Output pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE13_A::_1)
    }
}
#[doc = "Mode of Pn.14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE14_A {
    #[doc = "0: Pn.14 is Input pin"]
    _0 = 0,
    #[doc = "1: Pn.14 is Output pin"]
    _1 = 1,
}
impl From<MODE14_A> for bool {
    #[inline(always)]
    fn from(variant: MODE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE14` reader - Mode of Pn.14"]
pub type MODE14_R = crate::BitReader<MODE14_A>;
impl MODE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE14_A {
        match self.bits {
            false => MODE14_A::_0,
            true => MODE14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE14_A::_1
    }
}
#[doc = "Field `MODE14` writer - Mode of Pn.14"]
pub type MODE14_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE14_A, 14>;
impl<'a> MODE14_W<'a> {
    #[doc = "Pn.14 is Input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE14_A::_0)
    }
    #[doc = "Pn.14 is Output pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE14_A::_1)
    }
}
#[doc = "Mode of Pn.15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE15_A {
    #[doc = "0: Pn.15 is Input pin"]
    _0 = 0,
    #[doc = "1: Pn.15 is Output pin"]
    _1 = 1,
}
impl From<MODE15_A> for bool {
    #[inline(always)]
    fn from(variant: MODE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE15` reader - Mode of Pn.15"]
pub type MODE15_R = crate::BitReader<MODE15_A>;
impl MODE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE15_A {
        match self.bits {
            false => MODE15_A::_0,
            true => MODE15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE15_A::_1
    }
}
#[doc = "Field `MODE15` writer - Mode of Pn.15"]
pub type MODE15_W<'a> = crate::BitWriter<'a, u32, MODE_SPEC, MODE15_A, 15>;
impl<'a> MODE15_W<'a> {
    #[doc = "Pn.15 is Input pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE15_A::_0)
    }
    #[doc = "Pn.15 is Output pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE15_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Mode of Pn.0"]
    #[inline(always)]
    pub fn mode0(&self) -> MODE0_R {
        MODE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode of Pn.1"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode of Pn.2"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mode of Pn.3"]
    #[inline(always)]
    pub fn mode3(&self) -> MODE3_R {
        MODE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode of Pn.4"]
    #[inline(always)]
    pub fn mode4(&self) -> MODE4_R {
        MODE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode of Pn.5"]
    #[inline(always)]
    pub fn mode5(&self) -> MODE5_R {
        MODE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mode of Pn.6"]
    #[inline(always)]
    pub fn mode6(&self) -> MODE6_R {
        MODE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mode of Pn.7"]
    #[inline(always)]
    pub fn mode7(&self) -> MODE7_R {
        MODE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mode of Pn.8"]
    #[inline(always)]
    pub fn mode8(&self) -> MODE8_R {
        MODE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mode of Pn.9"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mode of Pn.10"]
    #[inline(always)]
    pub fn mode10(&self) -> MODE10_R {
        MODE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mode of Pn.11"]
    #[inline(always)]
    pub fn mode11(&self) -> MODE11_R {
        MODE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mode of Pn.12"]
    #[inline(always)]
    pub fn mode12(&self) -> MODE12_R {
        MODE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mode of Pn.13"]
    #[inline(always)]
    pub fn mode13(&self) -> MODE13_R {
        MODE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Mode of Pn.14"]
    #[inline(always)]
    pub fn mode14(&self) -> MODE14_R {
        MODE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Mode of Pn.15"]
    #[inline(always)]
    pub fn mode15(&self) -> MODE15_R {
        MODE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of Pn.0"]
    #[inline(always)]
    pub fn mode0(&mut self) -> MODE0_W {
        MODE0_W::new(self)
    }
    #[doc = "Bit 1 - Mode of Pn.1"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W::new(self)
    }
    #[doc = "Bit 2 - Mode of Pn.2"]
    #[inline(always)]
    pub fn mode2(&mut self) -> MODE2_W {
        MODE2_W::new(self)
    }
    #[doc = "Bit 3 - Mode of Pn.3"]
    #[inline(always)]
    pub fn mode3(&mut self) -> MODE3_W {
        MODE3_W::new(self)
    }
    #[doc = "Bit 4 - Mode of Pn.4"]
    #[inline(always)]
    pub fn mode4(&mut self) -> MODE4_W {
        MODE4_W::new(self)
    }
    #[doc = "Bit 5 - Mode of Pn.5"]
    #[inline(always)]
    pub fn mode5(&mut self) -> MODE5_W {
        MODE5_W::new(self)
    }
    #[doc = "Bit 6 - Mode of Pn.6"]
    #[inline(always)]
    pub fn mode6(&mut self) -> MODE6_W {
        MODE6_W::new(self)
    }
    #[doc = "Bit 7 - Mode of Pn.7"]
    #[inline(always)]
    pub fn mode7(&mut self) -> MODE7_W {
        MODE7_W::new(self)
    }
    #[doc = "Bit 8 - Mode of Pn.8"]
    #[inline(always)]
    pub fn mode8(&mut self) -> MODE8_W {
        MODE8_W::new(self)
    }
    #[doc = "Bit 9 - Mode of Pn.9"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W::new(self)
    }
    #[doc = "Bit 10 - Mode of Pn.10"]
    #[inline(always)]
    pub fn mode10(&mut self) -> MODE10_W {
        MODE10_W::new(self)
    }
    #[doc = "Bit 11 - Mode of Pn.11"]
    #[inline(always)]
    pub fn mode11(&mut self) -> MODE11_W {
        MODE11_W::new(self)
    }
    #[doc = "Bit 12 - Mode of Pn.12"]
    #[inline(always)]
    pub fn mode12(&mut self) -> MODE12_W {
        MODE12_W::new(self)
    }
    #[doc = "Bit 13 - Mode of Pn.13"]
    #[inline(always)]
    pub fn mode13(&mut self) -> MODE13_W {
        MODE13_W::new(self)
    }
    #[doc = "Bit 14 - Mode of Pn.14"]
    #[inline(always)]
    pub fn mode14(&mut self) -> MODE14_W {
        MODE14_W::new(self)
    }
    #[doc = "Bit 15 - Mode of Pn.15"]
    #[inline(always)]
    pub fn mode15(&mut self) -> MODE15_W {
        MODE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x04 GPIO Port n Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
