#[doc = "Register `PWMCTRL2` reader"]
pub struct R(crate::R<PWMCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMCTRL2` writer"]
pub struct W(crate::W<PWMCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMCTRL2_SPEC>;
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
impl From<crate::W<PWMCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM16 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM16MODE_A {
    #[doc = "0: During up-counting, PWM16 is 0 when TC is less than MR16. During down-counting, PWM16 is 1 when TC is larger/equal than MR16"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM16 is 1 when TC is less than MR16. During down-counting, PWM16 is 0 when TC is larger/equal than MR16"]
    PWMMODE2 = 1,
    #[doc = "2: PWM16 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM16 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM16MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM16MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM16MODE` reader - PWM16 output mode"]
pub type PWM16MODE_R = crate::FieldReader<u8, PWM16MODE_A>;
impl PWM16MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM16MODE_A {
        match self.bits {
            0 => PWM16MODE_A::PWMMODE1,
            1 => PWM16MODE_A::PWMMODE2,
            2 => PWM16MODE_A::FORCE0,
            3 => PWM16MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM16MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM16MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM16MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM16MODE_A::FORCE1
    }
}
#[doc = "Field `PWM16MODE` writer - PWM16 output mode"]
pub type PWM16MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM16MODE_A, 2, 0>;
impl<'a> PWM16MODE_W<'a> {
    #[doc = "During up-counting, PWM16 is 0 when TC is less than MR16. During down-counting, PWM16 is 1 when TC is larger/equal than MR16"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM16MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM16 is 1 when TC is less than MR16. During down-counting, PWM16 is 0 when TC is larger/equal than MR16"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM16MODE_A::PWMMODE2)
    }
    #[doc = "PWM16 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM16MODE_A::FORCE0)
    }
    #[doc = "PWM16 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM16MODE_A::FORCE1)
    }
}
#[doc = "PWM17 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM17MODE_A {
    #[doc = "0: During up-counting, PWM17 is 0 when TC is less than MR17. During down-counting, PWM17 is 1 when TC is larger/equal than MR17"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM17 is 1 when TC is less than MR17. During down-counting, PWM17 is 0 when TC is larger/equal than MR17"]
    PWMMODE2 = 1,
    #[doc = "2: PWM17 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM17 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM17MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM17MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM17MODE` reader - PWM17 output mode"]
pub type PWM17MODE_R = crate::FieldReader<u8, PWM17MODE_A>;
impl PWM17MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM17MODE_A {
        match self.bits {
            0 => PWM17MODE_A::PWMMODE1,
            1 => PWM17MODE_A::PWMMODE2,
            2 => PWM17MODE_A::FORCE0,
            3 => PWM17MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM17MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM17MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM17MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM17MODE_A::FORCE1
    }
}
#[doc = "Field `PWM17MODE` writer - PWM17 output mode"]
pub type PWM17MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM17MODE_A, 2, 2>;
impl<'a> PWM17MODE_W<'a> {
    #[doc = "During up-counting, PWM17 is 0 when TC is less than MR17. During down-counting, PWM17 is 1 when TC is larger/equal than MR17"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM17MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM17 is 1 when TC is less than MR17. During down-counting, PWM17 is 0 when TC is larger/equal than MR17"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM17MODE_A::PWMMODE2)
    }
    #[doc = "PWM17 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM17MODE_A::FORCE0)
    }
    #[doc = "PWM17 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM17MODE_A::FORCE1)
    }
}
#[doc = "PWM18 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM18MODE_A {
    #[doc = "0: During up-counting, PWM18 is 0 when TC is less than MR18. During down-counting, PWM18 is 1 when TC is larger/equal than MR18"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM18 is 1 when TC is less than MR18. During down-counting, PWM18 is 0 when TC is larger/equal than MR18"]
    PWMMODE2 = 1,
    #[doc = "2: PWM18 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM18 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM18MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM18MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM18MODE` reader - PWM18 output mode"]
pub type PWM18MODE_R = crate::FieldReader<u8, PWM18MODE_A>;
impl PWM18MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM18MODE_A {
        match self.bits {
            0 => PWM18MODE_A::PWMMODE1,
            1 => PWM18MODE_A::PWMMODE2,
            2 => PWM18MODE_A::FORCE0,
            3 => PWM18MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM18MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM18MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM18MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM18MODE_A::FORCE1
    }
}
#[doc = "Field `PWM18MODE` writer - PWM18 output mode"]
pub type PWM18MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM18MODE_A, 2, 4>;
impl<'a> PWM18MODE_W<'a> {
    #[doc = "During up-counting, PWM18 is 0 when TC is less than MR18. During down-counting, PWM18 is 1 when TC is larger/equal than MR18"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM18MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM18 is 1 when TC is less than MR18. During down-counting, PWM18 is 0 when TC is larger/equal than MR18"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM18MODE_A::PWMMODE2)
    }
    #[doc = "PWM18 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM18MODE_A::FORCE0)
    }
    #[doc = "PWM18 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM18MODE_A::FORCE1)
    }
}
#[doc = "PWM19 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM19MODE_A {
    #[doc = "0: During up-counting, PWM19 is 0 when TC is less than MR19. During down-counting, PWM19 is 1 when TC is larger/equal than MR19"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM19 is 1 when TC is less than MR19. During down-counting, PWM19 is 0 when TC is larger/equal than MR19"]
    PWMMODE2 = 1,
    #[doc = "2: PWM19 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM19 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM19MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM19MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM19MODE` reader - PWM19 output mode"]
pub type PWM19MODE_R = crate::FieldReader<u8, PWM19MODE_A>;
impl PWM19MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM19MODE_A {
        match self.bits {
            0 => PWM19MODE_A::PWMMODE1,
            1 => PWM19MODE_A::PWMMODE2,
            2 => PWM19MODE_A::FORCE0,
            3 => PWM19MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM19MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM19MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM19MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM19MODE_A::FORCE1
    }
}
#[doc = "Field `PWM19MODE` writer - PWM19 output mode"]
pub type PWM19MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM19MODE_A, 2, 6>;
impl<'a> PWM19MODE_W<'a> {
    #[doc = "During up-counting, PWM19 is 0 when TC is less than MR19. During down-counting, PWM19 is 1 when TC is larger/equal than MR19"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM19MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM19 is 1 when TC is less than MR19. During down-counting, PWM19 is 0 when TC is larger/equal than MR19"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM19MODE_A::PWMMODE2)
    }
    #[doc = "PWM19 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM19MODE_A::FORCE0)
    }
    #[doc = "PWM19 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM19MODE_A::FORCE1)
    }
}
#[doc = "PWM20 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM20MODE_A {
    #[doc = "0: During up-counting, PWM20 is 0 when TC is less than MR20. During down-counting, PWM20 is 1 when TC is larger/equal than MR20"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM20 is 1 when TC is less than MR20. During down-counting, PWM20 is 0 when TC is larger/equal than MR20"]
    PWMMODE2 = 1,
    #[doc = "2: PWM20 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM20 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM20MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM20MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM20MODE` reader - PWM20 output mode"]
pub type PWM20MODE_R = crate::FieldReader<u8, PWM20MODE_A>;
impl PWM20MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM20MODE_A {
        match self.bits {
            0 => PWM20MODE_A::PWMMODE1,
            1 => PWM20MODE_A::PWMMODE2,
            2 => PWM20MODE_A::FORCE0,
            3 => PWM20MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM20MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM20MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM20MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM20MODE_A::FORCE1
    }
}
#[doc = "Field `PWM20MODE` writer - PWM20 output mode"]
pub type PWM20MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM20MODE_A, 2, 8>;
impl<'a> PWM20MODE_W<'a> {
    #[doc = "During up-counting, PWM20 is 0 when TC is less than MR20. During down-counting, PWM20 is 1 when TC is larger/equal than MR20"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM20MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM20 is 1 when TC is less than MR20. During down-counting, PWM20 is 0 when TC is larger/equal than MR20"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM20MODE_A::PWMMODE2)
    }
    #[doc = "PWM20 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM20MODE_A::FORCE0)
    }
    #[doc = "PWM20 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM20MODE_A::FORCE1)
    }
}
#[doc = "PWM21 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM21MODE_A {
    #[doc = "0: During up-counting, PWM21 is 0 when TC is less than MR21. During down-counting, PWM21 is 1 when TC is larger/equal than MR21"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM21 is 1 when TC is less than MR21. During down-counting, PWM21 is 0 when TC is larger/equal than MR21"]
    PWMMODE2 = 1,
    #[doc = "2: PWM21 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM21 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM21MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM21MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM21MODE` reader - PWM21 output mode"]
pub type PWM21MODE_R = crate::FieldReader<u8, PWM21MODE_A>;
impl PWM21MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM21MODE_A {
        match self.bits {
            0 => PWM21MODE_A::PWMMODE1,
            1 => PWM21MODE_A::PWMMODE2,
            2 => PWM21MODE_A::FORCE0,
            3 => PWM21MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM21MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM21MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM21MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM21MODE_A::FORCE1
    }
}
#[doc = "Field `PWM21MODE` writer - PWM21 output mode"]
pub type PWM21MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM21MODE_A, 2, 10>;
impl<'a> PWM21MODE_W<'a> {
    #[doc = "During up-counting, PWM21 is 0 when TC is less than MR21. During down-counting, PWM21 is 1 when TC is larger/equal than MR21"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM21MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM21 is 1 when TC is less than MR21. During down-counting, PWM21 is 0 when TC is larger/equal than MR21"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM21MODE_A::PWMMODE2)
    }
    #[doc = "PWM21 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM21MODE_A::FORCE0)
    }
    #[doc = "PWM21 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM21MODE_A::FORCE1)
    }
}
#[doc = "PWM22 output mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWM22MODE_A {
    #[doc = "0: During up-counting, PWM22 is 0 when TC is less than MR22. During down-counting, PWM22 is 1 when TC is larger/equal than MR22"]
    PWMMODE1 = 0,
    #[doc = "1: During up-counting, PWM22 is 1 when TC is less than MR22. During down-counting, PWM22 is 0 when TC is larger/equal than MR22"]
    PWMMODE2 = 1,
    #[doc = "2: PWM22 is forced to 0"]
    FORCE0 = 2,
    #[doc = "3: PWM22 is forced to 1"]
    FORCE1 = 3,
}
impl From<PWM22MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWM22MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWM22MODE` reader - PWM22 output mode"]
pub type PWM22MODE_R = crate::FieldReader<u8, PWM22MODE_A>;
impl PWM22MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWM22MODE_A {
        match self.bits {
            0 => PWM22MODE_A::PWMMODE1,
            1 => PWM22MODE_A::PWMMODE2,
            2 => PWM22MODE_A::FORCE0,
            3 => PWM22MODE_A::FORCE1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWMMODE1`"]
    #[inline(always)]
    pub fn is_pwmmode1(&self) -> bool {
        *self == PWM22MODE_A::PWMMODE1
    }
    #[doc = "Checks if the value of the field is `PWMMODE2`"]
    #[inline(always)]
    pub fn is_pwmmode2(&self) -> bool {
        *self == PWM22MODE_A::PWMMODE2
    }
    #[doc = "Checks if the value of the field is `FORCE0`"]
    #[inline(always)]
    pub fn is_force0(&self) -> bool {
        *self == PWM22MODE_A::FORCE0
    }
    #[doc = "Checks if the value of the field is `FORCE1`"]
    #[inline(always)]
    pub fn is_force1(&self) -> bool {
        *self == PWM22MODE_A::FORCE1
    }
}
#[doc = "Field `PWM22MODE` writer - PWM22 output mode"]
pub type PWM22MODE_W<'a> = crate::FieldWriterSafe<'a, u32, PWMCTRL2_SPEC, u8, PWM22MODE_A, 2, 12>;
impl<'a> PWM22MODE_W<'a> {
    #[doc = "During up-counting, PWM22 is 0 when TC is less than MR22. During down-counting, PWM22 is 1 when TC is larger/equal than MR22"]
    #[inline(always)]
    pub fn pwmmode1(self) -> &'a mut W {
        self.variant(PWM22MODE_A::PWMMODE1)
    }
    #[doc = "During up-counting, PWM22 is 1 when TC is less than MR22. During down-counting, PWM22 is 0 when TC is larger/equal than MR22"]
    #[inline(always)]
    pub fn pwmmode2(self) -> &'a mut W {
        self.variant(PWM22MODE_A::PWMMODE2)
    }
    #[doc = "PWM22 is forced to 0"]
    #[inline(always)]
    pub fn force0(self) -> &'a mut W {
        self.variant(PWM22MODE_A::FORCE0)
    }
    #[doc = "PWM22 is forced to 1"]
    #[inline(always)]
    pub fn force1(self) -> &'a mut W {
        self.variant(PWM22MODE_A::FORCE1)
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM16 output mode"]
    #[inline(always)]
    pub fn pwm16mode(&self) -> PWM16MODE_R {
        PWM16MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PWM17 output mode"]
    #[inline(always)]
    pub fn pwm17mode(&self) -> PWM17MODE_R {
        PWM17MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PWM18 output mode"]
    #[inline(always)]
    pub fn pwm18mode(&self) -> PWM18MODE_R {
        PWM18MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PWM19 output mode"]
    #[inline(always)]
    pub fn pwm19mode(&self) -> PWM19MODE_R {
        PWM19MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWM20 output mode"]
    #[inline(always)]
    pub fn pwm20mode(&self) -> PWM20MODE_R {
        PWM20MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWM21 output mode"]
    #[inline(always)]
    pub fn pwm21mode(&self) -> PWM21MODE_R {
        PWM21MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWM22 output mode"]
    #[inline(always)]
    pub fn pwm22mode(&self) -> PWM22MODE_R {
        PWM22MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM16 output mode"]
    #[inline(always)]
    pub fn pwm16mode(&mut self) -> PWM16MODE_W {
        PWM16MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - PWM17 output mode"]
    #[inline(always)]
    pub fn pwm17mode(&mut self) -> PWM17MODE_W {
        PWM17MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - PWM18 output mode"]
    #[inline(always)]
    pub fn pwm18mode(&mut self) -> PWM18MODE_W {
        PWM18MODE_W::new(self)
    }
    #[doc = "Bits 6:7 - PWM19 output mode"]
    #[inline(always)]
    pub fn pwm19mode(&mut self) -> PWM19MODE_W {
        PWM19MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - PWM20 output mode"]
    #[inline(always)]
    pub fn pwm20mode(&mut self) -> PWM20MODE_W {
        PWM20MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - PWM21 output mode"]
    #[inline(always)]
    pub fn pwm21mode(&mut self) -> PWM21MODE_W {
        PWM21MODE_W::new(self)
    }
    #[doc = "Bits 12:13 - PWM22 output mode"]
    #[inline(always)]
    pub fn pwm22mode(&mut self) -> PWM22MODE_W {
        PWM22MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x98 CT16Bn PWM Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmctrl2](index.html) module"]
pub struct PWMCTRL2_SPEC;
impl crate::RegisterSpec for PWMCTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmctrl2::R](R) reader structure"]
impl crate::Readable for PWMCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmctrl2::W](W) writer structure"]
impl crate::Writable for PWMCTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMCTRL2 to value 0"]
impl crate::Resettable for PWMCTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
