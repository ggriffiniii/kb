#[doc = "Register `RIS` reader"]
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Pn.0 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF0_A {
    #[doc = "0: No interrupt on Pn.0"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.0"]
    _1 = 1,
}
impl From<IF0_A> for bool {
    #[inline(always)]
    fn from(variant: IF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF0` reader - Pn.0 raw interrupt flag"]
pub type IF0_R = crate::BitReader<IF0_A>;
impl IF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF0_A {
        match self.bits {
            false => IF0_A::_0,
            true => IF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF0_A::_1
    }
}
#[doc = "Pn.1 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF1_A {
    #[doc = "0: No interrupt on Pn.1"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.1"]
    _1 = 1,
}
impl From<IF1_A> for bool {
    #[inline(always)]
    fn from(variant: IF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF1` reader - Pn.1 raw interrupt flag"]
pub type IF1_R = crate::BitReader<IF1_A>;
impl IF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF1_A {
        match self.bits {
            false => IF1_A::_0,
            true => IF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF1_A::_1
    }
}
#[doc = "Pn.2 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF2_A {
    #[doc = "0: No interrupt on Pn.2"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.2"]
    _1 = 1,
}
impl From<IF2_A> for bool {
    #[inline(always)]
    fn from(variant: IF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF2` reader - Pn.2 raw interrupt flag"]
pub type IF2_R = crate::BitReader<IF2_A>;
impl IF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF2_A {
        match self.bits {
            false => IF2_A::_0,
            true => IF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF2_A::_1
    }
}
#[doc = "Pn.3 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF3_A {
    #[doc = "0: No interrupt on Pn.3"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.3"]
    _1 = 1,
}
impl From<IF3_A> for bool {
    #[inline(always)]
    fn from(variant: IF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF3` reader - Pn.3 raw interrupt flag"]
pub type IF3_R = crate::BitReader<IF3_A>;
impl IF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF3_A {
        match self.bits {
            false => IF3_A::_0,
            true => IF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF3_A::_1
    }
}
#[doc = "Pn.4 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF4_A {
    #[doc = "0: No interrupt on Pn.4"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.4"]
    _1 = 1,
}
impl From<IF4_A> for bool {
    #[inline(always)]
    fn from(variant: IF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF4` reader - Pn.4 raw interrupt flag"]
pub type IF4_R = crate::BitReader<IF4_A>;
impl IF4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF4_A {
        match self.bits {
            false => IF4_A::_0,
            true => IF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF4_A::_1
    }
}
#[doc = "Pn.5 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF5_A {
    #[doc = "0: No interrupt on Pn.5"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.5"]
    _1 = 1,
}
impl From<IF5_A> for bool {
    #[inline(always)]
    fn from(variant: IF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF5` reader - Pn.5 raw interrupt flag"]
pub type IF5_R = crate::BitReader<IF5_A>;
impl IF5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF5_A {
        match self.bits {
            false => IF5_A::_0,
            true => IF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF5_A::_1
    }
}
#[doc = "Pn.6 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF6_A {
    #[doc = "0: No interrupt on Pn.6"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.6"]
    _1 = 1,
}
impl From<IF6_A> for bool {
    #[inline(always)]
    fn from(variant: IF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF6` reader - Pn.6 raw interrupt flag"]
pub type IF6_R = crate::BitReader<IF6_A>;
impl IF6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF6_A {
        match self.bits {
            false => IF6_A::_0,
            true => IF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF6_A::_1
    }
}
#[doc = "Pn.7 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF7_A {
    #[doc = "0: No interrupt on Pn.7"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.7"]
    _1 = 1,
}
impl From<IF7_A> for bool {
    #[inline(always)]
    fn from(variant: IF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF7` reader - Pn.7 raw interrupt flag"]
pub type IF7_R = crate::BitReader<IF7_A>;
impl IF7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF7_A {
        match self.bits {
            false => IF7_A::_0,
            true => IF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF7_A::_1
    }
}
#[doc = "Pn.8 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF8_A {
    #[doc = "0: No interrupt on Pn.8"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.8"]
    _1 = 1,
}
impl From<IF8_A> for bool {
    #[inline(always)]
    fn from(variant: IF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF8` reader - Pn.8 raw interrupt flag"]
pub type IF8_R = crate::BitReader<IF8_A>;
impl IF8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF8_A {
        match self.bits {
            false => IF8_A::_0,
            true => IF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF8_A::_1
    }
}
#[doc = "Pn.9 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF9_A {
    #[doc = "0: No interrupt on Pn.9"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.9"]
    _1 = 1,
}
impl From<IF9_A> for bool {
    #[inline(always)]
    fn from(variant: IF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF9` reader - Pn.9 raw interrupt flag"]
pub type IF9_R = crate::BitReader<IF9_A>;
impl IF9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF9_A {
        match self.bits {
            false => IF9_A::_0,
            true => IF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF9_A::_1
    }
}
#[doc = "Pn.10 raw interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF10_A {
    #[doc = "0: No interrupt on Pn.10"]
    _0 = 0,
    #[doc = "1: Interrupt requirements met on Pn.10"]
    _1 = 1,
}
impl From<IF10_A> for bool {
    #[inline(always)]
    fn from(variant: IF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IF10` reader - Pn.10 raw interrupt flag"]
pub type IF10_R = crate::BitReader<IF10_A>;
impl IF10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF10_A {
        match self.bits {
            false => IF10_A::_0,
            true => IF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF10_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Pn.0 raw interrupt flag"]
    #[inline(always)]
    pub fn if0(&self) -> IF0_R {
        IF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pn.1 raw interrupt flag"]
    #[inline(always)]
    pub fn if1(&self) -> IF1_R {
        IF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pn.2 raw interrupt flag"]
    #[inline(always)]
    pub fn if2(&self) -> IF2_R {
        IF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pn.3 raw interrupt flag"]
    #[inline(always)]
    pub fn if3(&self) -> IF3_R {
        IF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pn.4 raw interrupt flag"]
    #[inline(always)]
    pub fn if4(&self) -> IF4_R {
        IF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pn.5 raw interrupt flag"]
    #[inline(always)]
    pub fn if5(&self) -> IF5_R {
        IF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pn.6 raw interrupt flag"]
    #[inline(always)]
    pub fn if6(&self) -> IF6_R {
        IF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pn.7 raw interrupt flag"]
    #[inline(always)]
    pub fn if7(&self) -> IF7_R {
        IF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pn.8 raw interrupt flag"]
    #[inline(always)]
    pub fn if8(&self) -> IF8_R {
        IF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pn.9 raw interrupt flag"]
    #[inline(always)]
    pub fn if9(&self) -> IF9_R {
        IF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pn.10 raw interrupt flag"]
    #[inline(always)]
    pub fn if10(&self) -> IF10_R {
        IF10_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Offset:0x1C GPIO Port n Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](index.html) module"]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ris::R](R) reader structure"]
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
