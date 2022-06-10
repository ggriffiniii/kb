#[doc = "Register `EMC` reader"]
pub struct R(crate::R<EMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMC` writer"]
pub struct W(crate::W<EMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMC_SPEC>;
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
impl From<crate::W<EMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CT16Bn_PWM0 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC0_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM0 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM0 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM0 pin"]
    TOGGLE = 3,
}
impl From<EMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC0` reader - CT16Bn_PWM0 functionality"]
pub type EMC0_R = crate::FieldReader<u8, EMC0_A>;
impl EMC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC0_A {
        match self.bits {
            0 => EMC0_A::DONOTHING,
            1 => EMC0_A::LOW,
            2 => EMC0_A::HIGH,
            3 => EMC0_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC0_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC0_A::TOGGLE
    }
}
#[doc = "Field `EMC0` writer - CT16Bn_PWM0 functionality"]
pub type EMC0_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC0_A, 2, 0>;
impl<'a> EMC0_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC0_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM0 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC0_A::LOW)
    }
    #[doc = "CT16Bn_PWM0 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC0_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM0 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC0_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM1 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC1_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM1 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM1 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM1 pin"]
    TOGGLE = 3,
}
impl From<EMC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC1` reader - CT16Bn_PWM1 functionality"]
pub type EMC1_R = crate::FieldReader<u8, EMC1_A>;
impl EMC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC1_A {
        match self.bits {
            0 => EMC1_A::DONOTHING,
            1 => EMC1_A::LOW,
            2 => EMC1_A::HIGH,
            3 => EMC1_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC1_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC1_A::TOGGLE
    }
}
#[doc = "Field `EMC1` writer - CT16Bn_PWM1 functionality"]
pub type EMC1_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC1_A, 2, 2>;
impl<'a> EMC1_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC1_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM1 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC1_A::LOW)
    }
    #[doc = "CT16Bn_PWM1 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC1_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM1 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC1_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM2 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC2_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM2 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM2 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM2 pin"]
    TOGGLE = 3,
}
impl From<EMC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC2` reader - CT16Bn_PWM2 functionality"]
pub type EMC2_R = crate::FieldReader<u8, EMC2_A>;
impl EMC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC2_A {
        match self.bits {
            0 => EMC2_A::DONOTHING,
            1 => EMC2_A::LOW,
            2 => EMC2_A::HIGH,
            3 => EMC2_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC2_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC2_A::TOGGLE
    }
}
#[doc = "Field `EMC2` writer - CT16Bn_PWM2 functionality"]
pub type EMC2_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC2_A, 2, 4>;
impl<'a> EMC2_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC2_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM2 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC2_A::LOW)
    }
    #[doc = "CT16Bn_PWM2 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC2_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM2 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC2_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM3 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC3_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM3 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM3 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM3 pin"]
    TOGGLE = 3,
}
impl From<EMC3_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC3` reader - CT16Bn_PWM3 functionality"]
pub type EMC3_R = crate::FieldReader<u8, EMC3_A>;
impl EMC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC3_A {
        match self.bits {
            0 => EMC3_A::DONOTHING,
            1 => EMC3_A::LOW,
            2 => EMC3_A::HIGH,
            3 => EMC3_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC3_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC3_A::TOGGLE
    }
}
#[doc = "Field `EMC3` writer - CT16Bn_PWM3 functionality"]
pub type EMC3_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC3_A, 2, 6>;
impl<'a> EMC3_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC3_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM3 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC3_A::LOW)
    }
    #[doc = "CT16Bn_PWM3 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC3_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM3 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC3_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM4 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC4_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM4 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM4 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM4 pin"]
    TOGGLE = 3,
}
impl From<EMC4_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC4` reader - CT16Bn_PWM4 functionality"]
pub type EMC4_R = crate::FieldReader<u8, EMC4_A>;
impl EMC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC4_A {
        match self.bits {
            0 => EMC4_A::DONOTHING,
            1 => EMC4_A::LOW,
            2 => EMC4_A::HIGH,
            3 => EMC4_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC4_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC4_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC4_A::TOGGLE
    }
}
#[doc = "Field `EMC4` writer - CT16Bn_PWM4 functionality"]
pub type EMC4_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC4_A, 2, 8>;
impl<'a> EMC4_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC4_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM4 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC4_A::LOW)
    }
    #[doc = "CT16Bn_PWM4 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC4_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM4 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC4_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM5 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC5_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM5 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM5 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM5 pin"]
    TOGGLE = 3,
}
impl From<EMC5_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC5` reader - CT16Bn_PWM5 functionality"]
pub type EMC5_R = crate::FieldReader<u8, EMC5_A>;
impl EMC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC5_A {
        match self.bits {
            0 => EMC5_A::DONOTHING,
            1 => EMC5_A::LOW,
            2 => EMC5_A::HIGH,
            3 => EMC5_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC5_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC5_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC5_A::TOGGLE
    }
}
#[doc = "Field `EMC5` writer - CT16Bn_PWM5 functionality"]
pub type EMC5_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC5_A, 2, 10>;
impl<'a> EMC5_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC5_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM5 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC5_A::LOW)
    }
    #[doc = "CT16Bn_PWM5 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC5_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM5 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC5_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM6 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC6_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM6 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM6 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM6 pin"]
    TOGGLE = 3,
}
impl From<EMC6_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC6` reader - CT16Bn_PWM6 functionality"]
pub type EMC6_R = crate::FieldReader<u8, EMC6_A>;
impl EMC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC6_A {
        match self.bits {
            0 => EMC6_A::DONOTHING,
            1 => EMC6_A::LOW,
            2 => EMC6_A::HIGH,
            3 => EMC6_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC6_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC6_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC6_A::TOGGLE
    }
}
#[doc = "Field `EMC6` writer - CT16Bn_PWM6 functionality"]
pub type EMC6_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC6_A, 2, 12>;
impl<'a> EMC6_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC6_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM6 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC6_A::LOW)
    }
    #[doc = "CT16Bn_PWM6 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC6_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM6 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC6_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM7 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC7_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM7 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM7 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM7 pin"]
    TOGGLE = 3,
}
impl From<EMC7_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC7` reader - CT16Bn_PWM7 functionality"]
pub type EMC7_R = crate::FieldReader<u8, EMC7_A>;
impl EMC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC7_A {
        match self.bits {
            0 => EMC7_A::DONOTHING,
            1 => EMC7_A::LOW,
            2 => EMC7_A::HIGH,
            3 => EMC7_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC7_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC7_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC7_A::TOGGLE
    }
}
#[doc = "Field `EMC7` writer - CT16Bn_PWM7 functionality"]
pub type EMC7_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC7_A, 2, 14>;
impl<'a> EMC7_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC7_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM7 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC7_A::LOW)
    }
    #[doc = "CT16Bn_PWM7 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC7_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM7 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC7_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM8 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC8_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM8 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM8 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM8 pin"]
    TOGGLE = 3,
}
impl From<EMC8_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC8_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC8` reader - CT16Bn_PWM8 functionality"]
pub type EMC8_R = crate::FieldReader<u8, EMC8_A>;
impl EMC8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC8_A {
        match self.bits {
            0 => EMC8_A::DONOTHING,
            1 => EMC8_A::LOW,
            2 => EMC8_A::HIGH,
            3 => EMC8_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC8_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC8_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC8_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC8_A::TOGGLE
    }
}
#[doc = "Field `EMC8` writer - CT16Bn_PWM8 functionality"]
pub type EMC8_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC8_A, 2, 16>;
impl<'a> EMC8_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC8_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM8 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC8_A::LOW)
    }
    #[doc = "CT16Bn_PWM8 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC8_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM8 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC8_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM9 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC9_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM9 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM9 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM9 pin"]
    TOGGLE = 3,
}
impl From<EMC9_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC9_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC9` reader - CT16Bn_PWM9 functionality"]
pub type EMC9_R = crate::FieldReader<u8, EMC9_A>;
impl EMC9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC9_A {
        match self.bits {
            0 => EMC9_A::DONOTHING,
            1 => EMC9_A::LOW,
            2 => EMC9_A::HIGH,
            3 => EMC9_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC9_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC9_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC9_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC9_A::TOGGLE
    }
}
#[doc = "Field `EMC9` writer - CT16Bn_PWM9 functionality"]
pub type EMC9_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC9_A, 2, 18>;
impl<'a> EMC9_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC9_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM9 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC9_A::LOW)
    }
    #[doc = "CT16Bn_PWM9 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC9_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM9 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC9_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM10 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC10_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM10 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM10 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM10 pin"]
    TOGGLE = 3,
}
impl From<EMC10_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC10_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC10` reader - CT16Bn_PWM10 functionality"]
pub type EMC10_R = crate::FieldReader<u8, EMC10_A>;
impl EMC10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC10_A {
        match self.bits {
            0 => EMC10_A::DONOTHING,
            1 => EMC10_A::LOW,
            2 => EMC10_A::HIGH,
            3 => EMC10_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC10_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC10_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC10_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC10_A::TOGGLE
    }
}
#[doc = "Field `EMC10` writer - CT16Bn_PWM10 functionality"]
pub type EMC10_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC10_A, 2, 20>;
impl<'a> EMC10_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC10_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM10 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC10_A::LOW)
    }
    #[doc = "CT16Bn_PWM10 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC10_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM10 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC10_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM11 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC11_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM11 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM11 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM11 pin"]
    TOGGLE = 3,
}
impl From<EMC11_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC11_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC11` reader - CT16Bn_PWM11 functionality"]
pub type EMC11_R = crate::FieldReader<u8, EMC11_A>;
impl EMC11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC11_A {
        match self.bits {
            0 => EMC11_A::DONOTHING,
            1 => EMC11_A::LOW,
            2 => EMC11_A::HIGH,
            3 => EMC11_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC11_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC11_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC11_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC11_A::TOGGLE
    }
}
#[doc = "Field `EMC11` writer - CT16Bn_PWM11 functionality"]
pub type EMC11_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC11_A, 2, 22>;
impl<'a> EMC11_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC11_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM11 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC11_A::LOW)
    }
    #[doc = "CT16Bn_PWM11 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC11_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM11 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC11_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM12 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC12_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM12 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM12 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM12 pin"]
    TOGGLE = 3,
}
impl From<EMC12_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC12_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC12` reader - CT16Bn_PWM12 functionality"]
pub type EMC12_R = crate::FieldReader<u8, EMC12_A>;
impl EMC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC12_A {
        match self.bits {
            0 => EMC12_A::DONOTHING,
            1 => EMC12_A::LOW,
            2 => EMC12_A::HIGH,
            3 => EMC12_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC12_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC12_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC12_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC12_A::TOGGLE
    }
}
#[doc = "Field `EMC12` writer - CT16Bn_PWM12 functionality"]
pub type EMC12_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC12_A, 2, 24>;
impl<'a> EMC12_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC12_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM12 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC12_A::LOW)
    }
    #[doc = "CT16Bn_PWM12 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC12_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM12 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC12_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM13 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC13_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM13 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM13 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM13 pin"]
    TOGGLE = 3,
}
impl From<EMC13_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC13_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC13` reader - CT16Bn_PWM13 functionality"]
pub type EMC13_R = crate::FieldReader<u8, EMC13_A>;
impl EMC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC13_A {
        match self.bits {
            0 => EMC13_A::DONOTHING,
            1 => EMC13_A::LOW,
            2 => EMC13_A::HIGH,
            3 => EMC13_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC13_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC13_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC13_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC13_A::TOGGLE
    }
}
#[doc = "Field `EMC13` writer - CT16Bn_PWM13 functionality"]
pub type EMC13_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC13_A, 2, 26>;
impl<'a> EMC13_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC13_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM13 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC13_A::LOW)
    }
    #[doc = "CT16Bn_PWM13 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC13_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM13 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC13_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM14 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC14_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM14 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM14 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM14 pin"]
    TOGGLE = 3,
}
impl From<EMC14_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC14_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC14` reader - CT16Bn_PWM14 functionality"]
pub type EMC14_R = crate::FieldReader<u8, EMC14_A>;
impl EMC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC14_A {
        match self.bits {
            0 => EMC14_A::DONOTHING,
            1 => EMC14_A::LOW,
            2 => EMC14_A::HIGH,
            3 => EMC14_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC14_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC14_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC14_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC14_A::TOGGLE
    }
}
#[doc = "Field `EMC14` writer - CT16Bn_PWM14 functionality"]
pub type EMC14_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC14_A, 2, 28>;
impl<'a> EMC14_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC14_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM14 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC14_A::LOW)
    }
    #[doc = "CT16Bn_PWM14 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC14_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM14 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC14_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM15 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC15_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM15 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM15 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM15 pin"]
    TOGGLE = 3,
}
impl From<EMC15_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC15_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC15` reader - CT16Bn_PWM15 functionality"]
pub type EMC15_R = crate::FieldReader<u8, EMC15_A>;
impl EMC15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC15_A {
        match self.bits {
            0 => EMC15_A::DONOTHING,
            1 => EMC15_A::LOW,
            2 => EMC15_A::HIGH,
            3 => EMC15_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC15_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC15_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC15_A::TOGGLE
    }
}
#[doc = "Field `EMC15` writer - CT16Bn_PWM15 functionality"]
pub type EMC15_W<'a> = crate::FieldWriterSafe<'a, u32, EMC_SPEC, u8, EMC15_A, 2, 30>;
impl<'a> EMC15_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC15_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM15 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC15_A::LOW)
    }
    #[doc = "CT16Bn_PWM15 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC15_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM15 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC15_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - CT16Bn_PWM0 functionality"]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CT16Bn_PWM1 functionality"]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CT16Bn_PWM2 functionality"]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CT16Bn_PWM3 functionality"]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CT16Bn_PWM4 functionality"]
    #[inline(always)]
    pub fn emc4(&self) -> EMC4_R {
        EMC4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CT16Bn_PWM5 functionality"]
    #[inline(always)]
    pub fn emc5(&self) -> EMC5_R {
        EMC5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CT16Bn_PWM6 functionality"]
    #[inline(always)]
    pub fn emc6(&self) -> EMC6_R {
        EMC6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - CT16Bn_PWM7 functionality"]
    #[inline(always)]
    pub fn emc7(&self) -> EMC7_R {
        EMC7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - CT16Bn_PWM8 functionality"]
    #[inline(always)]
    pub fn emc8(&self) -> EMC8_R {
        EMC8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CT16Bn_PWM9 functionality"]
    #[inline(always)]
    pub fn emc9(&self) -> EMC9_R {
        EMC9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - CT16Bn_PWM10 functionality"]
    #[inline(always)]
    pub fn emc10(&self) -> EMC10_R {
        EMC10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - CT16Bn_PWM11 functionality"]
    #[inline(always)]
    pub fn emc11(&self) -> EMC11_R {
        EMC11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - CT16Bn_PWM12 functionality"]
    #[inline(always)]
    pub fn emc12(&self) -> EMC12_R {
        EMC12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - CT16Bn_PWM13 functionality"]
    #[inline(always)]
    pub fn emc13(&self) -> EMC13_R {
        EMC13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - CT16Bn_PWM14 functionality"]
    #[inline(always)]
    pub fn emc14(&self) -> EMC14_R {
        EMC14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CT16Bn_PWM15 functionality"]
    #[inline(always)]
    pub fn emc15(&self) -> EMC15_R {
        EMC15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CT16Bn_PWM0 functionality"]
    #[inline(always)]
    pub fn emc0(&mut self) -> EMC0_W {
        EMC0_W::new(self)
    }
    #[doc = "Bits 2:3 - CT16Bn_PWM1 functionality"]
    #[inline(always)]
    pub fn emc1(&mut self) -> EMC1_W {
        EMC1_W::new(self)
    }
    #[doc = "Bits 4:5 - CT16Bn_PWM2 functionality"]
    #[inline(always)]
    pub fn emc2(&mut self) -> EMC2_W {
        EMC2_W::new(self)
    }
    #[doc = "Bits 6:7 - CT16Bn_PWM3 functionality"]
    #[inline(always)]
    pub fn emc3(&mut self) -> EMC3_W {
        EMC3_W::new(self)
    }
    #[doc = "Bits 8:9 - CT16Bn_PWM4 functionality"]
    #[inline(always)]
    pub fn emc4(&mut self) -> EMC4_W {
        EMC4_W::new(self)
    }
    #[doc = "Bits 10:11 - CT16Bn_PWM5 functionality"]
    #[inline(always)]
    pub fn emc5(&mut self) -> EMC5_W {
        EMC5_W::new(self)
    }
    #[doc = "Bits 12:13 - CT16Bn_PWM6 functionality"]
    #[inline(always)]
    pub fn emc6(&mut self) -> EMC6_W {
        EMC6_W::new(self)
    }
    #[doc = "Bits 14:15 - CT16Bn_PWM7 functionality"]
    #[inline(always)]
    pub fn emc7(&mut self) -> EMC7_W {
        EMC7_W::new(self)
    }
    #[doc = "Bits 16:17 - CT16Bn_PWM8 functionality"]
    #[inline(always)]
    pub fn emc8(&mut self) -> EMC8_W {
        EMC8_W::new(self)
    }
    #[doc = "Bits 18:19 - CT16Bn_PWM9 functionality"]
    #[inline(always)]
    pub fn emc9(&mut self) -> EMC9_W {
        EMC9_W::new(self)
    }
    #[doc = "Bits 20:21 - CT16Bn_PWM10 functionality"]
    #[inline(always)]
    pub fn emc10(&mut self) -> EMC10_W {
        EMC10_W::new(self)
    }
    #[doc = "Bits 22:23 - CT16Bn_PWM11 functionality"]
    #[inline(always)]
    pub fn emc11(&mut self) -> EMC11_W {
        EMC11_W::new(self)
    }
    #[doc = "Bits 24:25 - CT16Bn_PWM12 functionality"]
    #[inline(always)]
    pub fn emc12(&mut self) -> EMC12_W {
        EMC12_W::new(self)
    }
    #[doc = "Bits 26:27 - CT16Bn_PWM13 functionality"]
    #[inline(always)]
    pub fn emc13(&mut self) -> EMC13_W {
        EMC13_W::new(self)
    }
    #[doc = "Bits 28:29 - CT16Bn_PWM14 functionality"]
    #[inline(always)]
    pub fn emc14(&mut self) -> EMC14_W {
        EMC14_W::new(self)
    }
    #[doc = "Bits 30:31 - CT16Bn_PWM15 functionality"]
    #[inline(always)]
    pub fn emc15(&mut self) -> EMC15_W {
        EMC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x8C CT16Bn External Match Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emc](index.html) module"]
pub struct EMC_SPEC;
impl crate::RegisterSpec for EMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emc::R](R) reader structure"]
impl crate::Readable for EMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emc::W](W) writer structure"]
impl crate::Writable for EMC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMC to value 0"]
impl crate::Resettable for EMC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
