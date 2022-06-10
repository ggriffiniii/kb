#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash program mode chosen bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PG_A {
    #[doc = "0: Disable Flash program mode"]
    _0 = 0,
    #[doc = "1: Enable Flash program mode"]
    _1 = 1,
}
impl From<PG_A> for bool {
    #[inline(always)]
    fn from(variant: PG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PG` reader - Flash program mode chosen bit"]
pub type PG_R = crate::BitReader<PG_A>;
impl PG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PG_A {
        match self.bits {
            false => PG_A::_0,
            true => PG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PG_A::_1
    }
}
#[doc = "Field `PG` writer - Flash program mode chosen bit"]
pub type PG_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, PG_A, 0>;
impl<'a> PG_W<'a> {
    #[doc = "Disable Flash program mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PG_A::_0)
    }
    #[doc = "Enable Flash program mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PG_A::_1)
    }
}
#[doc = "Page erase mode chosen bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER_A {
    #[doc = "0: Disable page erase mode"]
    _0 = 0,
    #[doc = "1: Enable page erase mode"]
    _1 = 1,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER` reader - Page erase mode chosen bit"]
pub type PER_R = crate::BitReader<PER_A>;
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            false => PER_A::_0,
            true => PER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER_A::_1
    }
}
#[doc = "Field `PER` writer - Page erase mode chosen bit"]
pub type PER_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, PER_A, 1>;
impl<'a> PER_W<'a> {
    #[doc = "Disable page erase mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER_A::_0)
    }
    #[doc = "Enable page erase mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER_A::_1)
    }
}
#[doc = "Mass erase mode chosen bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MER_A {
    #[doc = "0: Disable masse erase mode"]
    _0 = 0,
    #[doc = "1: Enable mass erase mode"]
    _1 = 1,
}
impl From<MER_A> for bool {
    #[inline(always)]
    fn from(variant: MER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MER` reader - Mass erase mode chosen bit"]
pub type MER_R = crate::BitReader<MER_A>;
impl MER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MER_A {
        match self.bits {
            false => MER_A::_0,
            true => MER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MER_A::_1
    }
}
#[doc = "Field `MER` writer - Mass erase mode chosen bit"]
pub type MER_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, MER_A, 2>;
impl<'a> MER_W<'a> {
    #[doc = "Disable masse erase mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MER_A::_0)
    }
    #[doc = "Enable mass erase mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MER_A::_1)
    }
}
#[doc = "Start erase/program operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: Stop/finish operation"]
    _0 = 0,
    #[doc = "1: Start erase/program operation"]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start erase/program operation"]
pub type START_R = crate::BitReader<START_A>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Field `START` writer - Start erase/program operation"]
pub type START_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, START_A, 6>;
impl<'a> START_W<'a> {
    #[doc = "Stop/finish operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_A::_0)
    }
    #[doc = "Start erase/program operation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_A::_1)
    }
}
#[doc = "Checksum calculation chosen\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHK_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Trigger checksum calculation"]
    ENABLE = 1,
}
impl From<CHK_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHK` reader - Checksum calculation chosen"]
pub type CHK_R = crate::BitReader<CHK_A>;
impl CHK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHK_A {
        match self.bits {
            false => CHK_A::DISABLE,
            true => CHK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHK_A::ENABLE
    }
}
#[doc = "Field `CHK` writer - Checksum calculation chosen"]
pub type CHK_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, CHK_A, 7>;
impl<'a> CHK_W<'a> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHK_A::DISABLE)
    }
    #[doc = "Trigger checksum calculation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHK_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Flash program mode chosen bit"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page erase mode chosen bit"]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mass erase mode chosen bit"]
    #[inline(always)]
    pub fn mer(&self) -> MER_R {
        MER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Start erase/program operation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Checksum calculation chosen"]
    #[inline(always)]
    pub fn chk(&self) -> CHK_R {
        CHK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash program mode chosen bit"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W::new(self)
    }
    #[doc = "Bit 1 - Page erase mode chosen bit"]
    #[inline(always)]
    pub fn per(&mut self) -> PER_W {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - Mass erase mode chosen bit"]
    #[inline(always)]
    pub fn mer(&mut self) -> MER_W {
        MER_W::new(self)
    }
    #[doc = "Bit 6 - Start erase/program operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W::new(self)
    }
    #[doc = "Bit 7 - Checksum calculation chosen"]
    #[inline(always)]
    pub fn chk(&mut self) -> CHK_W {
        CHK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x08 Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
