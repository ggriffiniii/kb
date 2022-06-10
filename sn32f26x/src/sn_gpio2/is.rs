#[doc = "Register `IS` reader"]
pub struct R(crate::R<IS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS` writer"]
pub struct W(crate::W<IS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IS_SPEC>;
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
impl From<crate::W<IS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt on Pn.0 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS0_A {
    #[doc = "0: Interrupt on Pn.0 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.0 is event sensitive"]
    EVENT = 1,
}
impl From<IS0_A> for bool {
    #[inline(always)]
    fn from(variant: IS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS0` reader - Interrupt on Pn.0 is event or edge sensitive"]
pub type IS0_R = crate::BitReader<IS0_A>;
impl IS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS0_A {
        match self.bits {
            false => IS0_A::EDGE,
            true => IS0_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS0_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS0_A::EVENT
    }
}
#[doc = "Field `IS0` writer - Interrupt on Pn.0 is event or edge sensitive"]
pub type IS0_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS0_A, 0>;
impl<'a> IS0_W<'a> {
    #[doc = "Interrupt on Pn.0 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS0_A::EDGE)
    }
    #[doc = "Interrupt on Pn.0 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS0_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.1 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS1_A {
    #[doc = "0: Interrupt on Pn.1 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.1 is event sensitive"]
    EVENT = 1,
}
impl From<IS1_A> for bool {
    #[inline(always)]
    fn from(variant: IS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS1` reader - Interrupt on Pn.1 is event or edge sensitive"]
pub type IS1_R = crate::BitReader<IS1_A>;
impl IS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS1_A {
        match self.bits {
            false => IS1_A::EDGE,
            true => IS1_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS1_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS1_A::EVENT
    }
}
#[doc = "Field `IS1` writer - Interrupt on Pn.1 is event or edge sensitive"]
pub type IS1_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS1_A, 1>;
impl<'a> IS1_W<'a> {
    #[doc = "Interrupt on Pn.1 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS1_A::EDGE)
    }
    #[doc = "Interrupt on Pn.1 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS1_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.2 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS2_A {
    #[doc = "0: Interrupt on Pn.2 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.2 is event sensitive"]
    EVENT = 1,
}
impl From<IS2_A> for bool {
    #[inline(always)]
    fn from(variant: IS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS2` reader - Interrupt on Pn.2 is event or edge sensitive"]
pub type IS2_R = crate::BitReader<IS2_A>;
impl IS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS2_A {
        match self.bits {
            false => IS2_A::EDGE,
            true => IS2_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS2_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS2_A::EVENT
    }
}
#[doc = "Field `IS2` writer - Interrupt on Pn.2 is event or edge sensitive"]
pub type IS2_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS2_A, 2>;
impl<'a> IS2_W<'a> {
    #[doc = "Interrupt on Pn.2 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS2_A::EDGE)
    }
    #[doc = "Interrupt on Pn.2 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS2_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.3 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS3_A {
    #[doc = "0: Interrupt on Pn.3 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.3 is event sensitive"]
    EVENT = 1,
}
impl From<IS3_A> for bool {
    #[inline(always)]
    fn from(variant: IS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS3` reader - Interrupt on Pn.3 is event or edge sensitive"]
pub type IS3_R = crate::BitReader<IS3_A>;
impl IS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS3_A {
        match self.bits {
            false => IS3_A::EDGE,
            true => IS3_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS3_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS3_A::EVENT
    }
}
#[doc = "Field `IS3` writer - Interrupt on Pn.3 is event or edge sensitive"]
pub type IS3_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS3_A, 3>;
impl<'a> IS3_W<'a> {
    #[doc = "Interrupt on Pn.3 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS3_A::EDGE)
    }
    #[doc = "Interrupt on Pn.3 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS3_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.4 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS4_A {
    #[doc = "0: Interrupt on Pn.4 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.4 is event sensitive"]
    EVENT = 1,
}
impl From<IS4_A> for bool {
    #[inline(always)]
    fn from(variant: IS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS4` reader - Interrupt on Pn.4 is event or edge sensitive"]
pub type IS4_R = crate::BitReader<IS4_A>;
impl IS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS4_A {
        match self.bits {
            false => IS4_A::EDGE,
            true => IS4_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS4_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS4_A::EVENT
    }
}
#[doc = "Field `IS4` writer - Interrupt on Pn.4 is event or edge sensitive"]
pub type IS4_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS4_A, 4>;
impl<'a> IS4_W<'a> {
    #[doc = "Interrupt on Pn.4 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS4_A::EDGE)
    }
    #[doc = "Interrupt on Pn.4 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS4_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.5 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS5_A {
    #[doc = "0: Interrupt on Pn.5 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.5 is event sensitive"]
    EVENT = 1,
}
impl From<IS5_A> for bool {
    #[inline(always)]
    fn from(variant: IS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS5` reader - Interrupt on Pn.5 is event or edge sensitive"]
pub type IS5_R = crate::BitReader<IS5_A>;
impl IS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS5_A {
        match self.bits {
            false => IS5_A::EDGE,
            true => IS5_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS5_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS5_A::EVENT
    }
}
#[doc = "Field `IS5` writer - Interrupt on Pn.5 is event or edge sensitive"]
pub type IS5_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS5_A, 5>;
impl<'a> IS5_W<'a> {
    #[doc = "Interrupt on Pn.5 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS5_A::EDGE)
    }
    #[doc = "Interrupt on Pn.5 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS5_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.6 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS6_A {
    #[doc = "0: Interrupt on Pn.6 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.6 is event sensitive"]
    EVENT = 1,
}
impl From<IS6_A> for bool {
    #[inline(always)]
    fn from(variant: IS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS6` reader - Interrupt on Pn.6 is event or edge sensitive"]
pub type IS6_R = crate::BitReader<IS6_A>;
impl IS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS6_A {
        match self.bits {
            false => IS6_A::EDGE,
            true => IS6_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS6_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS6_A::EVENT
    }
}
#[doc = "Field `IS6` writer - Interrupt on Pn.6 is event or edge sensitive"]
pub type IS6_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS6_A, 6>;
impl<'a> IS6_W<'a> {
    #[doc = "Interrupt on Pn.6 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS6_A::EDGE)
    }
    #[doc = "Interrupt on Pn.6 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS6_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.7 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS7_A {
    #[doc = "0: Interrupt on Pn.7 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.7 is event sensitive"]
    EVENT = 1,
}
impl From<IS7_A> for bool {
    #[inline(always)]
    fn from(variant: IS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS7` reader - Interrupt on Pn.7 is event or edge sensitive"]
pub type IS7_R = crate::BitReader<IS7_A>;
impl IS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS7_A {
        match self.bits {
            false => IS7_A::EDGE,
            true => IS7_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS7_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS7_A::EVENT
    }
}
#[doc = "Field `IS7` writer - Interrupt on Pn.7 is event or edge sensitive"]
pub type IS7_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS7_A, 7>;
impl<'a> IS7_W<'a> {
    #[doc = "Interrupt on Pn.7 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS7_A::EDGE)
    }
    #[doc = "Interrupt on Pn.7 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS7_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.8 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS8_A {
    #[doc = "0: Interrupt on Pn.8 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.8 is event sensitive"]
    EVENT = 1,
}
impl From<IS8_A> for bool {
    #[inline(always)]
    fn from(variant: IS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS8` reader - Interrupt on Pn.8 is event or edge sensitive"]
pub type IS8_R = crate::BitReader<IS8_A>;
impl IS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS8_A {
        match self.bits {
            false => IS8_A::EDGE,
            true => IS8_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS8_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS8_A::EVENT
    }
}
#[doc = "Field `IS8` writer - Interrupt on Pn.8 is event or edge sensitive"]
pub type IS8_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS8_A, 8>;
impl<'a> IS8_W<'a> {
    #[doc = "Interrupt on Pn.8 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS8_A::EDGE)
    }
    #[doc = "Interrupt on Pn.8 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS8_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.9 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS9_A {
    #[doc = "0: Interrupt on Pn.9 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.9 is event sensitive"]
    EVENT = 1,
}
impl From<IS9_A> for bool {
    #[inline(always)]
    fn from(variant: IS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS9` reader - Interrupt on Pn.9 is event or edge sensitive"]
pub type IS9_R = crate::BitReader<IS9_A>;
impl IS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS9_A {
        match self.bits {
            false => IS9_A::EDGE,
            true => IS9_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS9_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS9_A::EVENT
    }
}
#[doc = "Field `IS9` writer - Interrupt on Pn.9 is event or edge sensitive"]
pub type IS9_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS9_A, 9>;
impl<'a> IS9_W<'a> {
    #[doc = "Interrupt on Pn.9 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS9_A::EDGE)
    }
    #[doc = "Interrupt on Pn.9 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS9_A::EVENT)
    }
}
#[doc = "Interrupt on Pn.10 is event or edge sensitive\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IS10_A {
    #[doc = "0: Interrupt on Pn.10 is edge sensitive"]
    EDGE = 0,
    #[doc = "1: Interrupt on Pn.10 is event sensitive"]
    EVENT = 1,
}
impl From<IS10_A> for bool {
    #[inline(always)]
    fn from(variant: IS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IS10` reader - Interrupt on Pn.10 is event or edge sensitive"]
pub type IS10_R = crate::BitReader<IS10_A>;
impl IS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IS10_A {
        match self.bits {
            false => IS10_A::EDGE,
            true => IS10_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == IS10_A::EDGE
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == IS10_A::EVENT
    }
}
#[doc = "Field `IS10` writer - Interrupt on Pn.10 is event or edge sensitive"]
pub type IS10_W<'a> = crate::BitWriter<'a, u32, IS_SPEC, IS10_A, 10>;
impl<'a> IS10_W<'a> {
    #[doc = "Interrupt on Pn.10 is edge sensitive"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(IS10_A::EDGE)
    }
    #[doc = "Interrupt on Pn.10 is event sensitive"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(IS10_A::EVENT)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on Pn.0 is event or edge sensitive"]
    #[inline(always)]
    pub fn is0(&self) -> IS0_R {
        IS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 is event or edge sensitive"]
    #[inline(always)]
    pub fn is1(&self) -> IS1_R {
        IS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 is event or edge sensitive"]
    #[inline(always)]
    pub fn is2(&self) -> IS2_R {
        IS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 is event or edge sensitive"]
    #[inline(always)]
    pub fn is3(&self) -> IS3_R {
        IS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 is event or edge sensitive"]
    #[inline(always)]
    pub fn is4(&self) -> IS4_R {
        IS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 is event or edge sensitive"]
    #[inline(always)]
    pub fn is5(&self) -> IS5_R {
        IS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Pn.6 is event or edge sensitive"]
    #[inline(always)]
    pub fn is6(&self) -> IS6_R {
        IS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt on Pn.7 is event or edge sensitive"]
    #[inline(always)]
    pub fn is7(&self) -> IS7_R {
        IS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt on Pn.8 is event or edge sensitive"]
    #[inline(always)]
    pub fn is8(&self) -> IS8_R {
        IS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on Pn.9 is event or edge sensitive"]
    #[inline(always)]
    pub fn is9(&self) -> IS9_R {
        IS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt on Pn.10 is event or edge sensitive"]
    #[inline(always)]
    pub fn is10(&self) -> IS10_R {
        IS10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on Pn.0 is event or edge sensitive"]
    #[inline(always)]
    pub fn is0(&mut self) -> IS0_W {
        IS0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 is event or edge sensitive"]
    #[inline(always)]
    pub fn is1(&mut self) -> IS1_W {
        IS1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 is event or edge sensitive"]
    #[inline(always)]
    pub fn is2(&mut self) -> IS2_W {
        IS2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 is event or edge sensitive"]
    #[inline(always)]
    pub fn is3(&mut self) -> IS3_W {
        IS3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 is event or edge sensitive"]
    #[inline(always)]
    pub fn is4(&mut self) -> IS4_W {
        IS4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 is event or edge sensitive"]
    #[inline(always)]
    pub fn is5(&mut self) -> IS5_W {
        IS5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on Pn.6 is event or edge sensitive"]
    #[inline(always)]
    pub fn is6(&mut self) -> IS6_W {
        IS6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt on Pn.7 is event or edge sensitive"]
    #[inline(always)]
    pub fn is7(&mut self) -> IS7_W {
        IS7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt on Pn.8 is event or edge sensitive"]
    #[inline(always)]
    pub fn is8(&mut self) -> IS8_W {
        IS8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt on Pn.9 is event or edge sensitive"]
    #[inline(always)]
    pub fn is9(&mut self) -> IS9_W {
        IS9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt on Pn.10 is event or edge sensitive"]
    #[inline(always)]
    pub fn is10(&mut self) -> IS10_W {
        IS10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x0C GPIO Port n Interrupt Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](index.html) module"]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [is::R](R) reader structure"]
impl crate::Readable for IS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [is::W](W) writer structure"]
impl crate::Writable for IS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
