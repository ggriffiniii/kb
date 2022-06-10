#[doc = "Register `PWMCTRL` reader"]
pub struct R(crate::R<PWMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCTRL` writer"]
pub struct W(crate::W<PWMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCTRL_SPEC>;
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
impl From<crate::W<PWMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM0 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM0MODE_A {
    #[doc = "0: During up-counting, PWM0 is 0 when TC is less than MR0. During down-counting, PWM0 is 1 when TC is larger/equal than MR0"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM0 is 1 when TC is less than MR0. During down-counting, PWM0 is 0 when TC is larger/equal than MR0"]
    PWMMODE2 = 1,
    #[doc = "2: PWM0 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM0 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM0MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM0MODE` reader - PWM0 output mode"]
pub type PWM0MODE_R = crate::FieldReader<u8, PWM0MODE_A>;
impl PWM0MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM0MODE_A {
        match self.bits {
            0 => PWM0MODE_A::PWMMODE1,
            1 => PWM0MODE_A::PWMMODE2,
            2 => PWM0MODE_A::FORCE0,
            3 => PWM0MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM0MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM0MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM0MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM0MODE_A::FORCE1
    }
}
#[doc = "Field `PWM0MODE` writer - PWM0 output mode"]
pub type PWM0MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM0MODE_A, 2, 0>;
impl<'a> PWM0MODE_W<'a> {
    #[doc = "During up-counting, PWM0 is 0 when TC is less than MR0. During down-counting, PWM0 is 1 when TC is larger/equal than MR0"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM0MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM0 is 1 when TC is less than MR0. During down-counting, PWM0 is 0 when TC is larger/equal than MR0"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM0MODE_A::PWMMODE2)
    }
    #[doc = "PWM0 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM0MODE_A::FORCE0)
    }
    #[doc = "PWM0 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM0MODE_A::FORCE1)
    }
}
#[doc = "PWM1 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM1MODE_A {
    #[doc = "0: During up-counting, PWM1 is 0 when TC is less than MR1. During down-counting, PWM1 is 1 when TC is larger/equal than MR1"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM1 is 1 when TC is less than MR1. During down-counting, PWM1 is 0 when TC is larger/equal than MR1"]
    PWMMODE2 = 1,
    #[doc = "2: PWM1 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM1 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM1MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM1MODE` reader - PWM1 output mode"]
pub type PWM1MODE_R = crate::FieldReader<u8, PWM1MODE_A>;
impl PWM1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM1MODE_A {
        match self.bits {
            0 => PWM1MODE_A::PWMMODE1,
            1 => PWM1MODE_A::PWMMODE2,
            2 => PWM1MODE_A::FORCE0,
            3 => PWM1MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM1MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM1MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM1MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM1MODE_A::FORCE1
    }
}
#[doc = "Field `PWM1MODE` writer - PWM1 output mode"]
pub type PWM1MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM1MODE_A, 2, 2>;
impl<'a> PWM1MODE_W<'a> {
    #[doc = "During up-counting, PWM1 is 0 when TC is less than MR1. During down-counting, PWM1 is 1 when TC is larger/equal than MR1"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM1MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM1 is 1 when TC is less than MR1. During down-counting, PWM1 is 0 when TC is larger/equal than MR1"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM1MODE_A::PWMMODE2)
    }
    #[doc = "PWM1 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM1MODE_A::FORCE0)
    }
    #[doc = "PWM1 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM1MODE_A::FORCE1)
    }
}
#[doc = "PWM2 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM2MODE_A {
    #[doc = "0: During up-counting, PWM2 is 0 when TC is less than MR2. During down-counting, PWM2 is 1 when TC is larger/equal than MR2"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM2 is 1 when TC is less than MR2. During down-counting, PWM2 is 0 when TC is larger/equal than MR2"]
    PWMMODE2 = 1,
    #[doc = "2: PWM2 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM2 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM2MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM2MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM2MODE` reader - PWM2 output mode"]
pub type PWM2MODE_R = crate::FieldReader<u8, PWM2MODE_A>;
impl PWM2MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM2MODE_A {
        match self.bits {
            0 => PWM2MODE_A::PWMMODE1,
            1 => PWM2MODE_A::PWMMODE2,
            2 => PWM2MODE_A::FORCE0,
            3 => PWM2MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM2MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM2MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM2MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM2MODE_A::FORCE1
    }
}
#[doc = "Field `PWM2MODE` writer - PWM2 output mode"]
pub type PWM2MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM2MODE_A, 2, 4>;
impl<'a> PWM2MODE_W<'a> {
    #[doc = "During up-counting, PWM2 is 0 when TC is less than MR2. During down-counting, PWM2 is 1 when TC is larger/equal than MR2"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM2MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM2 is 1 when TC is less than MR2. During down-counting, PWM2 is 0 when TC is larger/equal than MR2"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM2MODE_A::PWMMODE2)
    }
    #[doc = "PWM2 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM2MODE_A::FORCE0)
    }
    #[doc = "PWM2 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM2MODE_A::FORCE1)
    }
}
#[doc = "PWM3 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM3MODE_A {
    #[doc = "0: During up-counting, PWM3 is 0 when TC is less than MR3. During down-counting, PWM3 is 1 when TC is larger/equal than MR3"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM3 is 1 when TC is less than MR3. During down-counting, PWM3 is 0 when TC is larger/equal than MR3"]
    PWMMODE2 = 1,
    #[doc = "2: PWM3 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM3 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM3MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM3MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM3MODE` reader - PWM3 output mode"]
pub type PWM3MODE_R = crate::FieldReader<u8, PWM3MODE_A>;
impl PWM3MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM3MODE_A {
        match self.bits {
            0 => PWM3MODE_A::PWMMODE1,
            1 => PWM3MODE_A::PWMMODE2,
            2 => PWM3MODE_A::FORCE0,
            3 => PWM3MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM3MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM3MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM3MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM3MODE_A::FORCE1
    }
}
#[doc = "Field `PWM3MODE` writer - PWM3 output mode"]
pub type PWM3MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM3MODE_A, 2, 6>;
impl<'a> PWM3MODE_W<'a> {
    #[doc = "During up-counting, PWM3 is 0 when TC is less than MR3. During down-counting, PWM3 is 1 when TC is larger/equal than MR3"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM3MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM3 is 1 when TC is less than MR3. During down-counting, PWM3 is 0 when TC is larger/equal than MR3"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM3MODE_A::PWMMODE2)
    }
    #[doc = "PWM3 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM3MODE_A::FORCE0)
    }
    #[doc = "PWM3 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM3MODE_A::FORCE1)
    }
}
#[doc = "PWM4 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM4MODE_A {
    #[doc = "0: During up-counting, PWM4 is 0 when TC is less than MR4. During down-counting, PWM4 is 1 when TC is larger/equal than MR4"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM4 is 1 when TC is less than MR4. During down-counting, PWM4 is 0 when TC is larger/equal than MR4"]
    PWMMODE2 = 1,
    #[doc = "2: PWM4 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM4 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM4MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM4MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM4MODE` reader - PWM4 output mode"]
pub type PWM4MODE_R = crate::FieldReader<u8, PWM4MODE_A>;
impl PWM4MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM4MODE_A {
        match self.bits {
            0 => PWM4MODE_A::PWMMODE1,
            1 => PWM4MODE_A::PWMMODE2,
            2 => PWM4MODE_A::FORCE0,
            3 => PWM4MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM4MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM4MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM4MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM4MODE_A::FORCE1
    }
}
#[doc = "Field `PWM4MODE` writer - PWM4 output mode"]
pub type PWM4MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM4MODE_A, 2, 8>;
impl<'a> PWM4MODE_W<'a> {
    #[doc = "During up-counting, PWM4 is 0 when TC is less than MR4. During down-counting, PWM4 is 1 when TC is larger/equal than MR4"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM4MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM4 is 1 when TC is less than MR4. During down-counting, PWM4 is 0 when TC is larger/equal than MR4"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM4MODE_A::PWMMODE2)
    }
    #[doc = "PWM4 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM4MODE_A::FORCE0)
    }
    #[doc = "PWM4 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM4MODE_A::FORCE1)
    }
}
#[doc = "PWM5 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM5MODE_A {
    #[doc = "0: During up-counting, PWM5 is 0 when TC is less than MR5. During down-counting, PWM5 is 1 when TC is larger/equal than MR5"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM5 is 1 when TC is less than MR5. During down-counting, PWM5 is 0 when TC is larger/equal than MR5"]
    PWMMODE2 = 1,
    #[doc = "2: PWM5 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM5 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM5MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM5MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM5MODE` reader - PWM5 output mode"]
pub type PWM5MODE_R = crate::FieldReader<u8, PWM5MODE_A>;
impl PWM5MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM5MODE_A {
        match self.bits {
            0 => PWM5MODE_A::PWMMODE1,
            1 => PWM5MODE_A::PWMMODE2,
            2 => PWM5MODE_A::FORCE0,
            3 => PWM5MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM5MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM5MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM5MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM5MODE_A::FORCE1
    }
}
#[doc = "Field `PWM5MODE` writer - PWM5 output mode"]
pub type PWM5MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM5MODE_A, 2, 10>;
impl<'a> PWM5MODE_W<'a> {
    #[doc = "During up-counting, PWM5 is 0 when TC is less than MR5. During down-counting, PWM5 is 1 when TC is larger/equal than MR5"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM5MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM5 is 1 when TC is less than MR5. During down-counting, PWM5 is 0 when TC is larger/equal than MR5"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM5MODE_A::PWMMODE2)
    }
    #[doc = "PWM5 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM5MODE_A::FORCE0)
    }
    #[doc = "PWM5 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM5MODE_A::FORCE1)
    }
}
#[doc = "PWM6 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM6MODE_A {
    #[doc = "0: During up-counting, PWM6 is 0 when TC is less than MR6. During down-counting, PWM6 is 1 when TC is larger/equal than MR6"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM6 is 1 when TC is less than MR6. During down-counting, PWM6 is 0 when TC is larger/equal than MR6"]
    PWMMODE2 = 1,
    #[doc = "2: PWM6 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM6 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM6MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM6MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM6MODE` reader - PWM6 output mode"]
pub type PWM6MODE_R = crate::FieldReader<u8, PWM6MODE_A>;
impl PWM6MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM6MODE_A {
        match self.bits {
            0 => PWM6MODE_A::PWMMODE1,
            1 => PWM6MODE_A::PWMMODE2,
            2 => PWM6MODE_A::FORCE0,
            3 => PWM6MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM6MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM6MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM6MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM6MODE_A::FORCE1
    }
}
#[doc = "Field `PWM6MODE` writer - PWM6 output mode"]
pub type PWM6MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM6MODE_A, 2, 12>;
impl<'a> PWM6MODE_W<'a> {
    #[doc = "During up-counting, PWM6 is 0 when TC is less than MR6. During down-counting, PWM6 is 1 when TC is larger/equal than MR6"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM6MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM6 is 1 when TC is less than MR6. During down-counting, PWM6 is 0 when TC is larger/equal than MR6"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM6MODE_A::PWMMODE2)
    }
    #[doc = "PWM6 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM6MODE_A::FORCE0)
    }
    #[doc = "PWM6 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM6MODE_A::FORCE1)
    }
}
#[doc = "PWM7 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM7MODE_A {
    #[doc = "0: During up-counting, PWM7 is 0 when TC is less than MR7. During down-counting, PWM7 is 1 when TC is larger/equal than MR7"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM7 is 1 when TC is less than MR7. During down-counting, PWM7 is 0 when TC is larger/equal than MR7"]
    PWMMODE2 = 1,
    #[doc = "2: PWM7 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM7 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM7MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM7MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM7MODE` reader - PWM7 output mode"]
pub type PWM7MODE_R = crate::FieldReader<u8, PWM7MODE_A>;
impl PWM7MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM7MODE_A {
        match self.bits {
            0 => PWM7MODE_A::PWMMODE1,
            1 => PWM7MODE_A::PWMMODE2,
            2 => PWM7MODE_A::FORCE0,
            3 => PWM7MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM7MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM7MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM7MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM7MODE_A::FORCE1
    }
}
#[doc = "Field `PWM7MODE` writer - PWM7 output mode"]
pub type PWM7MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM7MODE_A, 2, 14>;
impl<'a> PWM7MODE_W<'a> {
    #[doc = "During up-counting, PWM7 is 0 when TC is less than MR7. During down-counting, PWM7 is 1 when TC is larger/equal than MR7"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM7MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM7 is 1 when TC is less than MR7. During down-counting, PWM7 is 0 when TC is larger/equal than MR7"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM7MODE_A::PWMMODE2)
    }
    #[doc = "PWM7 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM7MODE_A::FORCE0)
    }
    #[doc = "PWM7 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM7MODE_A::FORCE1)
    }
}
#[doc = "PWM8 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM8MODE_A {
    #[doc = "0: During up-counting, PWM8 is 0 when TC is less than MR8. During down-counting, PWM8 is 1 when TC is larger/equal than MR8"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM8 is 1 when TC is less than MR8. During down-counting, PWM8 is 0 when TC is larger/equal than MR8"]
    PWMMODE2 = 1,
    #[doc = "2: PWM8 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM8 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM8MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM8MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM8MODE` reader - PWM8 output mode"]
pub type PWM8MODE_R = crate::FieldReader<u8, PWM8MODE_A>;
impl PWM8MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM8MODE_A {
        match self.bits {
            0 => PWM8MODE_A::PWMMODE1,
            1 => PWM8MODE_A::PWMMODE2,
            2 => PWM8MODE_A::FORCE0,
            3 => PWM8MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM8MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM8MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM8MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM8MODE_A::FORCE1
    }
}
#[doc = "Field `PWM8MODE` writer - PWM8 output mode"]
pub type PWM8MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM8MODE_A, 2, 16>;
impl<'a> PWM8MODE_W<'a> {
    #[doc = "During up-counting, PWM8 is 0 when TC is less than MR8. During down-counting, PWM8 is 1 when TC is larger/equal than MR8"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM8MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM8 is 1 when TC is less than MR8. During down-counting, PWM8 is 0 when TC is larger/equal than MR8"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM8MODE_A::PWMMODE2)
    }
    #[doc = "PWM8 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM8MODE_A::FORCE0)
    }
    #[doc = "PWM8 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM8MODE_A::FORCE1)
    }
}
#[doc = "PWM9 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM9MODE_A {
    #[doc = "0: During up-counting, PWM9 is 0 when TC is less than MR9. During down-counting, PWM9 is 1 when TC is larger/equal than MR9"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM9 is 1 when TC is less than MR9. During down-counting, PWM9 is 0 when TC is larger/equal than MR9"]
    PWMMODE2 = 1,
    #[doc = "2: PWM9 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM9 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM9MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM9MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM9MODE` reader - PWM9 output mode"]
pub type PWM9MODE_R = crate::FieldReader<u8, PWM9MODE_A>;
impl PWM9MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM9MODE_A {
        match self.bits {
            0 => PWM9MODE_A::PWMMODE1,
            1 => PWM9MODE_A::PWMMODE2,
            2 => PWM9MODE_A::FORCE0,
            3 => PWM9MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM9MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM9MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM9MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM9MODE_A::FORCE1
    }
}
#[doc = "Field `PWM9MODE` writer - PWM9 output mode"]
pub type PWM9MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM9MODE_A, 2, 18>;
impl<'a> PWM9MODE_W<'a> {
    #[doc = "During up-counting, PWM9 is 0 when TC is less than MR9. During down-counting, PWM9 is 1 when TC is larger/equal than MR9"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM9MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM9 is 1 when TC is less than MR9. During down-counting, PWM9 is 0 when TC is larger/equal than MR9"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM9MODE_A::PWMMODE2)
    }
    #[doc = "PWM9 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM9MODE_A::FORCE0)
    }
    #[doc = "PWM9 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM9MODE_A::FORCE1)
    }
}
#[doc = "PWM10 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM10MODE_A {
    #[doc = "0: During up-counting, PWM10 is 0 when TC is less than MR10. During down-counting, PWM10 is 1 when TC is larger/equal than MR10"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM10 is 1 when TC is less than MR10. During down-counting, PWM10 is 0 when TC is larger/equal than MR10"]
    PWMMODE2 = 1,
    #[doc = "2: PWM10 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM10 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM10MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM10MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM10MODE` reader - PWM10 output mode"]
pub type PWM10MODE_R = crate::FieldReader<u8, PWM10MODE_A>;
impl PWM10MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM10MODE_A {
        match self.bits {
            0 => PWM10MODE_A::PWMMODE1,
            1 => PWM10MODE_A::PWMMODE2,
            2 => PWM10MODE_A::FORCE0,
            3 => PWM10MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM10MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM10MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM10MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM10MODE_A::FORCE1
    }
}
#[doc = "Field `PWM10MODE` writer - PWM10 output mode"]
pub type PWM10MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM10MODE_A, 2, 20>;
impl<'a> PWM10MODE_W<'a> {
    #[doc = "During up-counting, PWM10 is 0 when TC is less than MR10. During down-counting, PWM10 is 1 when TC is larger/equal than MR10"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM10MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM10 is 1 when TC is less than MR10. During down-counting, PWM10 is 0 when TC is larger/equal than MR10"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM10MODE_A::PWMMODE2)
    }
    #[doc = "PWM10 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM10MODE_A::FORCE0)
    }
    #[doc = "PWM10 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM10MODE_A::FORCE1)
    }
}
#[doc = "PWM11 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM11MODE_A {
    #[doc = "0: During up-counting, PWM11 is 0 when TC is less than MR11. During down-counting, PWM11 is 1 when TC is larger/equal than MR11"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM11 is 1 when TC is less than MR11. During down-counting, PWM11 is 0 when TC is larger/equal than MR11"]
    PWMMODE2 = 1,
    #[doc = "2: PWM11 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM11 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM11MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM11MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM11MODE` reader - PWM11 output mode"]
pub type PWM11MODE_R = crate::FieldReader<u8, PWM11MODE_A>;
impl PWM11MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM11MODE_A {
        match self.bits {
            0 => PWM11MODE_A::PWMMODE1,
            1 => PWM11MODE_A::PWMMODE2,
            2 => PWM11MODE_A::FORCE0,
            3 => PWM11MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM11MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM11MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM11MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM11MODE_A::FORCE1
    }
}
#[doc = "Field `PWM11MODE` writer - PWM11 output mode"]
pub type PWM11MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM11MODE_A, 2, 22>;
impl<'a> PWM11MODE_W<'a> {
    #[doc = "During up-counting, PWM11 is 0 when TC is less than MR11. During down-counting, PWM11 is 1 when TC is larger/equal than MR11"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM11MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM11 is 1 when TC is less than MR11. During down-counting, PWM11 is 0 when TC is larger/equal than MR11"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM11MODE_A::PWMMODE2)
    }
    #[doc = "PWM11 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM11MODE_A::FORCE0)
    }
    #[doc = "PWM11 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM11MODE_A::FORCE1)
    }
}
#[doc = "PWM12 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM12MODE_A {
    #[doc = "0: During up-counting, PWM12 is 0 when TC is less than MR12. During down-counting, PWM12 is 1 when TC is larger/equal than MR12"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM12 is 1 when TC is less than MR12. During down-counting, PWM12 is 0 when TC is larger/equal than MR12"]
    PWMMODE2 = 1,
    #[doc = "2: PWM12 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM12 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM12MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM12MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM12MODE` reader - PWM12 output mode"]
pub type PWM12MODE_R = crate::FieldReader<u8, PWM12MODE_A>;
impl PWM12MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM12MODE_A {
        match self.bits {
            0 => PWM12MODE_A::PWMMODE1,
            1 => PWM12MODE_A::PWMMODE2,
            2 => PWM12MODE_A::FORCE0,
            3 => PWM12MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM12MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM12MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM12MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM12MODE_A::FORCE1
    }
}
#[doc = "Field `PWM12MODE` writer - PWM12 output mode"]
pub type PWM12MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM12MODE_A, 2, 24>;
impl<'a> PWM12MODE_W<'a> {
    #[doc = "During up-counting, PWM12 is 0 when TC is less than MR12. During down-counting, PWM12 is 1 when TC is larger/equal than MR12"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM12MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM12 is 1 when TC is less than MR12. During down-counting, PWM12 is 0 when TC is larger/equal than MR12"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM12MODE_A::PWMMODE2)
    }
    #[doc = "PWM12 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM12MODE_A::FORCE0)
    }
    #[doc = "PWM12 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM12MODE_A::FORCE1)
    }
}
#[doc = "PWM13 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM13MODE_A {
    #[doc = "0: During up-counting, PWM13 is 0 when TC is less than MR13. During down-counting, PWM13 is 1 when TC is larger/equal than MR13"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM13 is 1 when TC is less than MR13. During down-counting, PWM13 is 0 when TC is larger/equal than MR13"]
    PWMMODE2 = 1,
    #[doc = "2: PWM13 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM13 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM13MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM13MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM13MODE` reader - PWM13 output mode"]
pub type PWM13MODE_R = crate::FieldReader<u8, PWM13MODE_A>;
impl PWM13MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM13MODE_A {
        match self.bits {
            0 => PWM13MODE_A::PWMMODE1,
            1 => PWM13MODE_A::PWMMODE2,
            2 => PWM13MODE_A::FORCE0,
            3 => PWM13MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM13MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM13MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM13MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM13MODE_A::FORCE1
    }
}
#[doc = "Field `PWM13MODE` writer - PWM13 output mode"]
pub type PWM13MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM13MODE_A, 2, 26>;
impl<'a> PWM13MODE_W<'a> {
    #[doc = "During up-counting, PWM13 is 0 when TC is less than MR13. During down-counting, PWM13 is 1 when TC is larger/equal than MR13"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM13MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM13 is 1 when TC is less than MR13. During down-counting, PWM13 is 0 when TC is larger/equal than MR13"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM13MODE_A::PWMMODE2)
    }
    #[doc = "PWM13 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM13MODE_A::FORCE0)
    }
    #[doc = "PWM13 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM13MODE_A::FORCE1)
    }
}
#[doc = "PWM14 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM14MODE_A {
    #[doc = "0: During up-counting, PWM14 is 0 when TC is less than MR14. During down-counting, PWM14 is 1 when TC is larger/equal than MR14"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM14 is 1 when TC is less than MR14. During down-counting, PWM14 is 0 when TC is larger/equal than MR14"]
    PWMMODE2 = 1,
    #[doc = "2: PWM14 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM14 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM14MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM14MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM14MODE` reader - PWM14 output mode"]
pub type PWM14MODE_R = crate::FieldReader<u8, PWM14MODE_A>;
impl PWM14MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM14MODE_A {
        match self.bits {
            0 => PWM14MODE_A::PWMMODE1,
            1 => PWM14MODE_A::PWMMODE2,
            2 => PWM14MODE_A::FORCE0,
            3 => PWM14MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM14MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM14MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM14MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM14MODE_A::FORCE1
    }
}
#[doc = "Field `PWM14MODE` writer - PWM14 output mode"]
pub type PWM14MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM14MODE_A, 2, 28>;
impl<'a> PWM14MODE_W<'a> {
    #[doc = "During up-counting, PWM14 is 0 when TC is less than MR14. During down-counting, PWM14 is 1 when TC is larger/equal than MR14"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM14MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM14 is 1 when TC is less than MR14. During down-counting, PWM14 is 0 when TC is larger/equal than MR14"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM14MODE_A::PWMMODE2)
    }
    #[doc = "PWM14 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM14MODE_A::FORCE0)
    }
    #[doc = "PWM14 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM14MODE_A::FORCE1)
    }
}
#[doc = "PWM15 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM15MODE_A {
    #[doc = "0: During up-counting, PWM15 is 0 when TC is less than MR15. During down-counting, PWM15 is 1 when TC is larger/equal than MR15"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM15 is 1 when TC is less than MR15. During down-counting, PWM15 is 0 when TC is larger/equal than MR15"]
    PWMMODE2 = 1,
    #[doc = "2: PWM15 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM15 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM15MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM15MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM15MODE` reader - PWM15 output mode"]
pub type PWM15MODE_R = crate::FieldReader<u8, PWM15MODE_A>;
impl PWM15MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM15MODE_A {
        match self.bits {
            0 => PWM15MODE_A::PWMMODE1,
            1 => PWM15MODE_A::PWMMODE2,
            2 => PWM15MODE_A::FORCE0,
            3 => PWM15MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM15MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM15MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM15MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM15MODE_A::FORCE1
    }
}
#[doc = "Field `PWM15MODE` writer - PWM15 output mode"]
pub type PWM15MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL_SPEC, u8, PWM15MODE_A, 2, 30>;
impl<'a> PWM15MODE_W<'a> {
    #[doc = "During up-counting, PWM15 is 0 when TC is less than MR15. During down-counting, PWM15 is 1 when TC is larger/equal than MR15"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM15MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM15 is 1 when TC is less than MR15. During down-counting, PWM15 is 0 when TC is larger/equal than MR15"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM15MODE_A::PWMMODE2)
    }
    #[doc = "PWM15 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM15MODE_A::FORCE0)
    }
    #[doc = "PWM15 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM15MODE_A::FORCE1)
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM0 output mode"]
    #[inline(always)]
    pub fn pwm0mode(&self) -> PWM0MODE_R {
        PWM0MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PWM1 output mode"]
    #[inline(always)]
    pub fn pwm1mode(&self) -> PWM1MODE_R {
        PWM1MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PWM2 output mode"]
    #[inline(always)]
    pub fn pwm2mode(&self) -> PWM2MODE_R {
        PWM2MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PWM3 output mode"]
    #[inline(always)]
    pub fn pwm3mode(&self) -> PWM3MODE_R {
        PWM3MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWM4 output mode"]
    #[inline(always)]
    pub fn pwm4mode(&self) -> PWM4MODE_R {
        PWM4MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWM5 output mode"]
    #[inline(always)]
    pub fn pwm5mode(&self) -> PWM5MODE_R {
        PWM5MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWM6 output mode"]
    #[inline(always)]
    pub fn pwm6mode(&self) -> PWM6MODE_R {
        PWM6MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PWM7 output mode"]
    #[inline(always)]
    pub fn pwm7mode(&self) -> PWM7MODE_R {
        PWM7MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PWM8 output mode"]
    #[inline(always)]
    pub fn pwm8mode(&self) -> PWM8MODE_R {
        PWM8MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PWM9 output mode"]
    #[inline(always)]
    pub fn pwm9mode(&self) -> PWM9MODE_R {
        PWM9MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PWM10 output mode"]
    #[inline(always)]
    pub fn pwm10mode(&self) -> PWM10MODE_R {
        PWM10MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PWM11 output mode"]
    #[inline(always)]
    pub fn pwm11mode(&self) -> PWM11MODE_R {
        PWM11MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PWM12 output mode"]
    #[inline(always)]
    pub fn pwm12mode(&self) -> PWM12MODE_R {
        PWM12MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PWM13 output mode"]
    #[inline(always)]
    pub fn pwm13mode(&self) -> PWM13MODE_R {
        PWM13MODE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PWM14 output mode"]
    #[inline(always)]
    pub fn pwm14mode(&self) -> PWM14MODE_R {
        PWM14MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PWM15 output mode"]
    #[inline(always)]
    pub fn pwm15mode(&self) -> PWM15MODE_R {
        PWM15MODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM0 output mode"]
    #[inline(always)]
    pub fn pwm0mode(&mut self) -> PWM0MODE_W {
        PWM0MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM1 output mode"]
    #[inline(always)]
    pub fn pwm1mode(&mut self) -> PWM1MODE_W {
        PWM1MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM2 output mode"]
    #[inline(always)]
    pub fn pwm2mode(&mut self) -> PWM2MODE_W {
        PWM2MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - PWM3 output mode"]
    #[inline(always)]
    pub fn pwm3mode(&mut self) -> PWM3MODE_W {
        PWM3MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - PWM4 output mode"]
    #[inline(always)]
    pub fn pwm4mode(&mut self) -> PWM4MODE_W {
        PWM4MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - PWM5 output mode"]
    #[inline(always)]
    pub fn pwm5mode(&mut self) -> PWM5MODE_W {
        PWM5MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - PWM6 output mode"]
    #[inline(always)]
    pub fn pwm6mode(&mut self) -> PWM6MODE_W {
        PWM6MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - PWM7 output mode"]
    #[inline(always)]
    pub fn pwm7mode(&mut self) -> PWM7MODE_W {
        PWM7MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - PWM8 output mode"]
    #[inline(always)]
    pub fn pwm8mode(&mut self) -> PWM8MODE_W {
        PWM8MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - PWM9 output mode"]
    #[inline(always)]
    pub fn pwm9mode(&mut self) -> PWM9MODE_W {
        PWM9MODE_W::new(self)
    }
    #[doc = "Bits 20:21 - PWM10 output mode"]
    #[inline(always)]
    pub fn pwm10mode(&mut self) -> PWM10MODE_W {
        PWM10MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - PWM11 output mode"]
    #[inline(always)]
    pub fn pwm11mode(&mut self) -> PWM11MODE_W {
        PWM11MODE_W::new(self)
    }
    #[doc = "Bits 24:25 - PWM12 output mode"]
    #[inline(always)]
    pub fn pwm12mode(&mut self) -> PWM12MODE_W {
        PWM12MODE_W::new(self)
    }
    #[doc = "Bits 26:27 - PWM13 output mode"]
    #[inline(always)]
    pub fn pwm13mode(&mut self) -> PWM13MODE_W {
        PWM13MODE_W::new(self)
    }
    #[doc = "Bits 28:29 - PWM14 output mode"]
    #[inline(always)]
    pub fn pwm14mode(&mut self) -> PWM14MODE_W {
        PWM14MODE_W::new(self)
    }
    #[doc = "Bits 30:31 - PWM15 output mode"]
    #[inline(always)]
    pub fn pwm15mode(&mut self) -> PWM15MODE_W {
        PWM15MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x94 CT16Bn PWM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmctrl](index.html) module"]
pub struct PWMCTRL_SPEC;
impl crate::RegisterSpec for PWMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmctrl::R](R) reader structure"]
impl crate::Readable for PWMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmctrl::W](W) writer structure"]
impl crate::Writable for PWMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMCTRL to value 0"]
impl crate::Resettable for PWMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
