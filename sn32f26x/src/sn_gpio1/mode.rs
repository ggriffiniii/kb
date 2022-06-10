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
