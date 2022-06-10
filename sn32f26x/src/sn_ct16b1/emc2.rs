#[doc = "Register `EMC2` reader"]
pub struct R(crate::R<EMC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMC2` writer"]
pub struct W(crate::W<EMC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMC2_SPEC>;
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
impl From<crate::W<EMC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CT16Bn_PWM16 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC16_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM16 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM16 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM16 pin"]
    TOGGLE = 3,
}
impl From<EMC16_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC16_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC16` reader - CT16Bn_PWM16 functionality"]
pub type EMC16_R = crate::FieldReader<u8, EMC16_A>;
impl EMC16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC16_A {
        match self.bits {
            0 => EMC16_A::DONOTHING,
            1 => EMC16_A::LOW,
            2 => EMC16_A::HIGH,
            3 => EMC16_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC16_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC16_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC16_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC16_A::TOGGLE
    }
}
#[doc = "Field `EMC16` writer - CT16Bn_PWM16 functionality"]
pub type EMC16_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC16_A, 2, 0>;
impl<'a> EMC16_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC16_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM16 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC16_A::LOW)
    }
    #[doc = "CT16Bn_PWM16 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC16_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM16 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC16_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM17 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC17_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM17 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM17 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM17 pin"]
    TOGGLE = 3,
}
impl From<EMC17_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC17_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC17` reader - CT16Bn_PWM17 functionality"]
pub type EMC17_R = crate::FieldReader<u8, EMC17_A>;
impl EMC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC17_A {
        match self.bits {
            0 => EMC17_A::DONOTHING,
            1 => EMC17_A::LOW,
            2 => EMC17_A::HIGH,
            3 => EMC17_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC17_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC17_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC17_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC17_A::TOGGLE
    }
}
#[doc = "Field `EMC17` writer - CT16Bn_PWM17 functionality"]
pub type EMC17_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC17_A, 2, 2>;
impl<'a> EMC17_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC17_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM17 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC17_A::LOW)
    }
    #[doc = "CT16Bn_PWM17 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC17_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM17 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC17_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM18 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC18_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM18 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM18 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM18 pin"]
    TOGGLE = 3,
}
impl From<EMC18_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC18_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC18` reader - CT16Bn_PWM18 functionality"]
pub type EMC18_R = crate::FieldReader<u8, EMC18_A>;
impl EMC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC18_A {
        match self.bits {
            0 => EMC18_A::DONOTHING,
            1 => EMC18_A::LOW,
            2 => EMC18_A::HIGH,
            3 => EMC18_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC18_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC18_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC18_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC18_A::TOGGLE
    }
}
#[doc = "Field `EMC18` writer - CT16Bn_PWM18 functionality"]
pub type EMC18_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC18_A, 2, 4>;
impl<'a> EMC18_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC18_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM18 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC18_A::LOW)
    }
    #[doc = "CT16Bn_PWM18 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC18_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM18 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC18_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM19 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC19_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM19 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM19 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM19 pin"]
    TOGGLE = 3,
}
impl From<EMC19_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC19_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC19` reader - CT16Bn_PWM19 functionality"]
pub type EMC19_R = crate::FieldReader<u8, EMC19_A>;
impl EMC19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC19_A {
        match self.bits {
            0 => EMC19_A::DONOTHING,
            1 => EMC19_A::LOW,
            2 => EMC19_A::HIGH,
            3 => EMC19_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC19_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC19_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC19_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC19_A::TOGGLE
    }
}
#[doc = "Field `EMC19` writer - CT16Bn_PWM19 functionality"]
pub type EMC19_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC19_A, 2, 6>;
impl<'a> EMC19_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC19_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM19 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC19_A::LOW)
    }
    #[doc = "CT16Bn_PWM19 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC19_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM19 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC19_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM20 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC20_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM20 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM20 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM20 pin"]
    TOGGLE = 3,
}
impl From<EMC20_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC20_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC20` reader - CT16Bn_PWM20 functionality"]
pub type EMC20_R = crate::FieldReader<u8, EMC20_A>;
impl EMC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC20_A {
        match self.bits {
            0 => EMC20_A::DONOTHING,
            1 => EMC20_A::LOW,
            2 => EMC20_A::HIGH,
            3 => EMC20_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC20_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC20_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC20_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC20_A::TOGGLE
    }
}
#[doc = "Field `EMC20` writer - CT16Bn_PWM20 functionality"]
pub type EMC20_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC20_A, 2, 8>;
impl<'a> EMC20_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC20_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM20 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC20_A::LOW)
    }
    #[doc = "CT16Bn_PWM20 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC20_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM20 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC20_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM21 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC21_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM21 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM21 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM21 pin"]
    TOGGLE = 3,
}
impl From<EMC21_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC21_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC21` reader - CT16Bn_PWM21 functionality"]
pub type EMC21_R = crate::FieldReader<u8, EMC21_A>;
impl EMC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC21_A {
        match self.bits {
            0 => EMC21_A::DONOTHING,
            1 => EMC21_A::LOW,
            2 => EMC21_A::HIGH,
            3 => EMC21_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC21_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC21_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC21_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC21_A::TOGGLE
    }
}
#[doc = "Field `EMC21` writer - CT16Bn_PWM21 functionality"]
pub type EMC21_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC21_A, 2, 10>;
impl<'a> EMC21_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC21_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM21 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC21_A::LOW)
    }
    #[doc = "CT16Bn_PWM21 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC21_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM21 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC21_A::TOGGLE)
    }
}
#[doc = "CT16Bn_PWM22 functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC22_A {
    #[doc = "0: Do nothing"]
    DONOTHING = 0,
    #[doc = "1: CT16Bn_PWM22 pin is LOW"]
    LOW = 1,
    #[doc = "2: CT16Bn_PWM22 pin is HIGH"]
    HIGH = 2,
    #[doc = "3: Toggle CT16Bn_PWM22 pin"]
    TOGGLE = 3,
}
impl From<EMC22_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC22_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC22` reader - CT16Bn_PWM22 functionality"]
pub type EMC22_R = crate::FieldReader<u8, EMC22_A>;
impl EMC22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC22_A {
        match self.bits {
            0 => EMC22_A::DONOTHING,
            1 => EMC22_A::LOW,
            2 => EMC22_A::HIGH,
            3 => EMC22_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DONOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        *self == EMC22_A::DONOTHING
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EMC22_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EMC22_A::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == EMC22_A::TOGGLE
    }
}
#[doc = "Field `EMC22` writer - CT16Bn_PWM22 functionality"]
pub type EMC22_W<'a> = crate::FieldWriterSafe<'a, u32, EMC2_SPEC, u8, EMC22_A, 2, 12>;
impl<'a> EMC22_W<'a> {
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC22_A::DONOTHING)
    }
    #[doc = "CT16Bn_PWM22 pin is LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EMC22_A::LOW)
    }
    #[doc = "CT16Bn_PWM22 pin is HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EMC22_A::HIGH)
    }
    #[doc = "Toggle CT16Bn_PWM22 pin"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC22_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - CT16Bn_PWM16 functionality"]
    #[inline(always)]
    pub fn emc16(&self) -> EMC16_R {
        EMC16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CT16Bn_PWM17 functionality"]
    #[inline(always)]
    pub fn emc17(&self) -> EMC17_R {
        EMC17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CT16Bn_PWM18 functionality"]
    #[inline(always)]
    pub fn emc18(&self) -> EMC18_R {
        EMC18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CT16Bn_PWM19 functionality"]
    #[inline(always)]
    pub fn emc19(&self) -> EMC19_R {
        EMC19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - CT16Bn_PWM20 functionality"]
    #[inline(always)]
    pub fn emc20(&self) -> EMC20_R {
        EMC20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - CT16Bn_PWM21 functionality"]
    #[inline(always)]
    pub fn emc21(&self) -> EMC21_R {
        EMC21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CT16Bn_PWM22 functionality"]
    #[inline(always)]
    pub fn emc22(&self) -> EMC22_R {
        EMC22_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CT16Bn_PWM16 functionality"]
    #[inline(always)]
    pub fn emc16(&mut self) -> EMC16_W {
        EMC16_W::new(self)
    }
    #[doc = "Bits 2:3 - CT16Bn_PWM17 functionality"]
    #[inline(always)]
    pub fn emc17(&mut self) -> EMC17_W {
        EMC17_W::new(self)
    }
    #[doc = "Bits 4:5 - CT16Bn_PWM18 functionality"]
    #[inline(always)]
    pub fn emc18(&mut self) -> EMC18_W {
        EMC18_W::new(self)
    }
    #[doc = "Bits 6:7 - CT16Bn_PWM19 functionality"]
    #[inline(always)]
    pub fn emc19(&mut self) -> EMC19_W {
        EMC19_W::new(self)
    }
    #[doc = "Bits 8:9 - CT16Bn_PWM20 functionality"]
    #[inline(always)]
    pub fn emc20(&mut self) -> EMC20_W {
        EMC20_W::new(self)
    }
    #[doc = "Bits 10:11 - CT16Bn_PWM21 functionality"]
    #[inline(always)]
    pub fn emc21(&mut self) -> EMC21_W {
        EMC21_W::new(self)
    }
    #[doc = "Bits 12:13 - CT16Bn_PWM22 functionality"]
    #[inline(always)]
    pub fn emc22(&mut self) -> EMC22_W {
        EMC22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x90 CT16Bn External Match Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emc2](index.html) module"]
pub struct EMC2_SPEC;
impl crate::RegisterSpec for EMC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emc2::R](R) reader structure"]
impl crate::Readable for EMC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emc2::W](W) writer structure"]
impl crate::Writable for EMC2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMC2 to value 0"]
impl crate::Resettable for EMC2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
