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
