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
#[doc = "Configuration of Pn.0\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG0_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG0_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG0` reader - Configuration of Pn.0"]
pub type CFG0_R = crate::FieldReader<u8, CFG0_A>;
impl CFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG0_A> {
        match self.bits {
            0 => Some(CFG0_A::_00),
            2 => Some(CFG0_A::_10),
            3 => Some(CFG0_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG0_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG0_A::_11
    }
}
#[doc = "Field `CFG0` writer - Configuration of Pn.0"]
pub type CFG0_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG0_A, 2, 0>;
impl<'a> CFG0_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG0_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG0_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG0_A::_11)
    }
}
#[doc = "Configuration of Pn.1\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG1_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG1_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG1` reader - Configuration of Pn.1"]
pub type CFG1_R = crate::FieldReader<u8, CFG1_A>;
impl CFG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG1_A> {
        match self.bits {
            0 => Some(CFG1_A::_00),
            2 => Some(CFG1_A::_10),
            3 => Some(CFG1_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG1_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG1_A::_11
    }
}
#[doc = "Field `CFG1` writer - Configuration of Pn.1"]
pub type CFG1_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG1_A, 2, 2>;
impl<'a> CFG1_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG1_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG1_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG1_A::_11)
    }
}
#[doc = "Configuration of Pn.2\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG2_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG2_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG2` reader - Configuration of Pn.2"]
pub type CFG2_R = crate::FieldReader<u8, CFG2_A>;
impl CFG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG2_A> {
        match self.bits {
            0 => Some(CFG2_A::_00),
            2 => Some(CFG2_A::_10),
            3 => Some(CFG2_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG2_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG2_A::_11
    }
}
#[doc = "Field `CFG2` writer - Configuration of Pn.2"]
pub type CFG2_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG2_A, 2, 4>;
impl<'a> CFG2_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG2_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG2_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG2_A::_11)
    }
}
#[doc = "Configuration of Pn.3\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG3_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG3_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG3` reader - Configuration of Pn.3"]
pub type CFG3_R = crate::FieldReader<u8, CFG3_A>;
impl CFG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG3_A> {
        match self.bits {
            0 => Some(CFG3_A::_00),
            2 => Some(CFG3_A::_10),
            3 => Some(CFG3_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG3_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG3_A::_11
    }
}
#[doc = "Field `CFG3` writer - Configuration of Pn.3"]
pub type CFG3_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG3_A, 2, 6>;
impl<'a> CFG3_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG3_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG3_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG3_A::_11)
    }
}
#[doc = "Configuration of Pn.4\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG4_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG4_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG4` reader - Configuration of Pn.4"]
pub type CFG4_R = crate::FieldReader<u8, CFG4_A>;
impl CFG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG4_A> {
        match self.bits {
            0 => Some(CFG4_A::_00),
            2 => Some(CFG4_A::_10),
            3 => Some(CFG4_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG4_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG4_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG4_A::_11
    }
}
#[doc = "Field `CFG4` writer - Configuration of Pn.4"]
pub type CFG4_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG4_A, 2, 8>;
impl<'a> CFG4_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG4_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG4_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG4_A::_11)
    }
}
#[doc = "Configuration of Pn.5\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG5_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG5_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG5` reader - Configuration of Pn.5"]
pub type CFG5_R = crate::FieldReader<u8, CFG5_A>;
impl CFG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG5_A> {
        match self.bits {
            0 => Some(CFG5_A::_00),
            2 => Some(CFG5_A::_10),
            3 => Some(CFG5_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG5_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG5_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG5_A::_11
    }
}
#[doc = "Field `CFG5` writer - Configuration of Pn.5"]
pub type CFG5_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG5_A, 2, 10>;
impl<'a> CFG5_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG5_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG5_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG5_A::_11)
    }
}
#[doc = "Configuration of Pn.6\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG6_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG6_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG6` reader - Configuration of Pn.6"]
pub type CFG6_R = crate::FieldReader<u8, CFG6_A>;
impl CFG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG6_A> {
        match self.bits {
            0 => Some(CFG6_A::_00),
            2 => Some(CFG6_A::_10),
            3 => Some(CFG6_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG6_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG6_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG6_A::_11
    }
}
#[doc = "Field `CFG6` writer - Configuration of Pn.6"]
pub type CFG6_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG6_A, 2, 12>;
impl<'a> CFG6_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG6_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG6_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG6_A::_11)
    }
}
#[doc = "Configuration of Pn.7\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG7_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG7_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG7` reader - Configuration of Pn.7"]
pub type CFG7_R = crate::FieldReader<u8, CFG7_A>;
impl CFG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG7_A> {
        match self.bits {
            0 => Some(CFG7_A::_00),
            2 => Some(CFG7_A::_10),
            3 => Some(CFG7_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG7_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG7_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG7_A::_11
    }
}
#[doc = "Field `CFG7` writer - Configuration of Pn.7"]
pub type CFG7_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG7_A, 2, 14>;
impl<'a> CFG7_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG7_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG7_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG7_A::_11)
    }
}
#[doc = "Configuration of Pn.8\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG8_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG8_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG8` reader - Configuration of Pn.8"]
pub type CFG8_R = crate::FieldReader<u8, CFG8_A>;
impl CFG8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG8_A> {
        match self.bits {
            0 => Some(CFG8_A::_00),
            2 => Some(CFG8_A::_10),
            3 => Some(CFG8_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG8_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG8_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG8_A::_11
    }
}
#[doc = "Field `CFG8` writer - Configuration of Pn.8"]
pub type CFG8_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG8_A, 2, 16>;
impl<'a> CFG8_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG8_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG8_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG8_A::_11)
    }
}
#[doc = "Configuration of Pn.9\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG9_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG9_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG9` reader - Configuration of Pn.9"]
pub type CFG9_R = crate::FieldReader<u8, CFG9_A>;
impl CFG9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG9_A> {
        match self.bits {
            0 => Some(CFG9_A::_00),
            2 => Some(CFG9_A::_10),
            3 => Some(CFG9_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG9_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG9_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG9_A::_11
    }
}
#[doc = "Field `CFG9` writer - Configuration of Pn.9"]
pub type CFG9_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG9_A, 2, 18>;
impl<'a> CFG9_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG9_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG9_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG9_A::_11)
    }
}
#[doc = "Configuration of Pn.10\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG10_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG10_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG10` reader - Configuration of Pn.10"]
pub type CFG10_R = crate::FieldReader<u8, CFG10_A>;
impl CFG10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG10_A> {
        match self.bits {
            0 => Some(CFG10_A::_00),
            2 => Some(CFG10_A::_10),
            3 => Some(CFG10_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG10_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG10_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG10_A::_11
    }
}
#[doc = "Field `CFG10` writer - Configuration of Pn.10"]
pub type CFG10_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG10_A, 2, 20>;
impl<'a> CFG10_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG10_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG10_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG10_A::_11)
    }
}
#[doc = "Configuration of Pn.11\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG11_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG11_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG11` reader - Configuration of Pn.11"]
pub type CFG11_R = crate::FieldReader<u8, CFG11_A>;
impl CFG11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG11_A> {
        match self.bits {
            0 => Some(CFG11_A::_00),
            2 => Some(CFG11_A::_10),
            3 => Some(CFG11_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG11_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG11_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG11_A::_11
    }
}
#[doc = "Field `CFG11` writer - Configuration of Pn.11"]
pub type CFG11_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG11_A, 2, 22>;
impl<'a> CFG11_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG11_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG11_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG11_A::_11)
    }
}
#[doc = "Configuration of Pn.12\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG12_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG12_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG12` reader - Configuration of Pn.12"]
pub type CFG12_R = crate::FieldReader<u8, CFG12_A>;
impl CFG12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG12_A> {
        match self.bits {
            0 => Some(CFG12_A::_00),
            2 => Some(CFG12_A::_10),
            3 => Some(CFG12_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG12_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG12_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG12_A::_11
    }
}
#[doc = "Field `CFG12` writer - Configuration of Pn.12"]
pub type CFG12_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG12_A, 2, 24>;
impl<'a> CFG12_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG12_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG12_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG12_A::_11)
    }
}
#[doc = "Configuration of Pn.13\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG13_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG13_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG13` reader - Configuration of Pn.13"]
pub type CFG13_R = crate::FieldReader<u8, CFG13_A>;
impl CFG13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG13_A> {
        match self.bits {
            0 => Some(CFG13_A::_00),
            2 => Some(CFG13_A::_10),
            3 => Some(CFG13_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG13_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG13_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG13_A::_11
    }
}
#[doc = "Field `CFG13` writer - Configuration of Pn.13"]
pub type CFG13_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG13_A, 2, 26>;
impl<'a> CFG13_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG13_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG13_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG13_A::_11)
    }
}
#[doc = "Configuration of Pn.14\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG14_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG14_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG14` reader - Configuration of Pn.14"]
pub type CFG14_R = crate::FieldReader<u8, CFG14_A>;
impl CFG14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG14_A> {
        match self.bits {
            0 => Some(CFG14_A::_00),
            2 => Some(CFG14_A::_10),
            3 => Some(CFG14_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG14_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG14_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG14_A::_11
    }
}
#[doc = "Field `CFG14` writer - Configuration of Pn.14"]
pub type CFG14_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG14_A, 2, 28>;
impl<'a> CFG14_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG14_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG14_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG14_A::_11)
    }
}
#[doc = "Configuration of Pn.15\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG15_A {
    #[doc = "0: Enable pull-up resistor"]
    _00 = 0,
    #[doc = "2: No pull-up/pull-down resistor enabled"]
    _10 = 2,
    #[doc = "3: no pull-up resistor enabled, schmit trigger disabled"]
    _11 = 3,
}
impl From<CFG15_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG15` reader - Configuration of Pn.15"]
pub type CFG15_R = crate::FieldReader<u8, CFG15_A>;
impl CFG15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG15_A> {
        match self.bits {
            0 => Some(CFG15_A::_00),
            2 => Some(CFG15_A::_10),
            3 => Some(CFG15_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CFG15_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CFG15_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CFG15_A::_11
    }
}
#[doc = "Field `CFG15` writer - Configuration of Pn.15"]
pub type CFG15_W<'a> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, CFG15_A, 2, 30>;
impl<'a> CFG15_W<'a> {
    #[doc = "Enable pull-up resistor"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CFG15_A::_00)
    }
    #[doc = "No pull-up/pull-down resistor enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CFG15_A::_10)
    }
    #[doc = "no pull-up resistor enabled, schmit trigger disabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CFG15_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration of Pn.0"]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configuration of Pn.1"]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configuration of Pn.2"]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configuration of Pn.3"]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configuration of Pn.4"]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configuration of Pn.5"]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configuration of Pn.6"]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configuration of Pn.7"]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Configuration of Pn.8"]
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Configuration of Pn.9"]
    #[inline(always)]
    pub fn cfg9(&self) -> CFG9_R {
        CFG9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Configuration of Pn.10"]
    #[inline(always)]
    pub fn cfg10(&self) -> CFG10_R {
        CFG10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Configuration of Pn.11"]
    #[inline(always)]
    pub fn cfg11(&self) -> CFG11_R {
        CFG11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Configuration of Pn.12"]
    #[inline(always)]
    pub fn cfg12(&self) -> CFG12_R {
        CFG12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Configuration of Pn.13"]
    #[inline(always)]
    pub fn cfg13(&self) -> CFG13_R {
        CFG13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Configuration of Pn.14"]
    #[inline(always)]
    pub fn cfg14(&self) -> CFG14_R {
        CFG14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Configuration of Pn.15"]
    #[inline(always)]
    pub fn cfg15(&self) -> CFG15_R {
        CFG15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration of Pn.0"]
    #[inline(always)]
    pub fn cfg0(&mut self) -> CFG0_W {
        CFG0_W::new(self)
    }
    #[doc = "Bits 2:3 - Configuration of Pn.1"]
    #[inline(always)]
    pub fn cfg1(&mut self) -> CFG1_W {
        CFG1_W::new(self)
    }
    #[doc = "Bits 4:5 - Configuration of Pn.2"]
    #[inline(always)]
    pub fn cfg2(&mut self) -> CFG2_W {
        CFG2_W::new(self)
    }
    #[doc = "Bits 6:7 - Configuration of Pn.3"]
    #[inline(always)]
    pub fn cfg3(&mut self) -> CFG3_W {
        CFG3_W::new(self)
    }
    #[doc = "Bits 8:9 - Configuration of Pn.4"]
    #[inline(always)]
    pub fn cfg4(&mut self) -> CFG4_W {
        CFG4_W::new(self)
    }
    #[doc = "Bits 10:11 - Configuration of Pn.5"]
    #[inline(always)]
    pub fn cfg5(&mut self) -> CFG5_W {
        CFG5_W::new(self)
    }
    #[doc = "Bits 12:13 - Configuration of Pn.6"]
    #[inline(always)]
    pub fn cfg6(&mut self) -> CFG6_W {
        CFG6_W::new(self)
    }
    #[doc = "Bits 14:15 - Configuration of Pn.7"]
    #[inline(always)]
    pub fn cfg7(&mut self) -> CFG7_W {
        CFG7_W::new(self)
    }
    #[doc = "Bits 16:17 - Configuration of Pn.8"]
    #[inline(always)]
    pub fn cfg8(&mut self) -> CFG8_W {
        CFG8_W::new(self)
    }
    #[doc = "Bits 18:19 - Configuration of Pn.9"]
    #[inline(always)]
    pub fn cfg9(&mut self) -> CFG9_W {
        CFG9_W::new(self)
    }
    #[doc = "Bits 20:21 - Configuration of Pn.10"]
    #[inline(always)]
    pub fn cfg10(&mut self) -> CFG10_W {
        CFG10_W::new(self)
    }
    #[doc = "Bits 22:23 - Configuration of Pn.11"]
    #[inline(always)]
    pub fn cfg11(&mut self) -> CFG11_W {
        CFG11_W::new(self)
    }
    #[doc = "Bits 24:25 - Configuration of Pn.12"]
    #[inline(always)]
    pub fn cfg12(&mut self) -> CFG12_W {
        CFG12_W::new(self)
    }
    #[doc = "Bits 26:27 - Configuration of Pn.13"]
    #[inline(always)]
    pub fn cfg13(&mut self) -> CFG13_W {
        CFG13_W::new(self)
    }
    #[doc = "Bits 28:29 - Configuration of Pn.14"]
    #[inline(always)]
    pub fn cfg14(&mut self) -> CFG14_W {
        CFG14_W::new(self)
    }
    #[doc = "Bits 30:31 - Configuration of Pn.15"]
    #[inline(always)]
    pub fn cfg15(&mut self) -> CFG15_W {
        CFG15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x08 GPIO Port n Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CFG to value 0xaaaa_aaaa"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xaaaa_aaaa
    }
}
