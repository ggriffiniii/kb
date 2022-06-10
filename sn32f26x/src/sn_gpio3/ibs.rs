#[doc = "Register `IBS` reader"]
pub struct R(crate::R<IBS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBS` writer"]
pub struct W(crate::W<IBS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBS_SPEC>;
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
impl From<crate::W<IBS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt on Pn.0 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS0_A {
    #[doc = "0: Interrupt on Pn.0 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.0 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS0_A> for bool {
    #[inline(always)]
    fn from(variant: IBS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS0` reader - Interrupt on Pn.0 is triggered ob both edges"]
pub type IBS0_R = crate::BitReader<IBS0_A>;
impl IBS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS0_A {
        match self.bits {
            false => IBS0_A::IEV,
            true => IBS0_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS0_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS0_A::BOTHEDGE
    }
}
#[doc = "Field `IBS0` writer - Interrupt on Pn.0 is triggered ob both edges"]
pub type IBS0_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS0_A, 0>;
impl<'a> IBS0_W<'a> {
    #[doc = "Interrupt on Pn.0 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS0_A::IEV)
    }
    #[doc = "Both edges on Pn.0 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS0_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.1 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS1_A {
    #[doc = "0: Interrupt on Pn.1 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.1 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS1_A> for bool {
    #[inline(always)]
    fn from(variant: IBS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS1` reader - Interrupt on Pn.1 is triggered ob both edges"]
pub type IBS1_R = crate::BitReader<IBS1_A>;
impl IBS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS1_A {
        match self.bits {
            false => IBS1_A::IEV,
            true => IBS1_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS1_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS1_A::BOTHEDGE
    }
}
#[doc = "Field `IBS1` writer - Interrupt on Pn.1 is triggered ob both edges"]
pub type IBS1_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS1_A, 1>;
impl<'a> IBS1_W<'a> {
    #[doc = "Interrupt on Pn.1 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS1_A::IEV)
    }
    #[doc = "Both edges on Pn.1 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS1_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.2 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS2_A {
    #[doc = "0: Interrupt on Pn.2 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.2 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS2_A> for bool {
    #[inline(always)]
    fn from(variant: IBS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS2` reader - Interrupt on Pn.2 is triggered ob both edges"]
pub type IBS2_R = crate::BitReader<IBS2_A>;
impl IBS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS2_A {
        match self.bits {
            false => IBS2_A::IEV,
            true => IBS2_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS2_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS2_A::BOTHEDGE
    }
}
#[doc = "Field `IBS2` writer - Interrupt on Pn.2 is triggered ob both edges"]
pub type IBS2_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS2_A, 2>;
impl<'a> IBS2_W<'a> {
    #[doc = "Interrupt on Pn.2 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS2_A::IEV)
    }
    #[doc = "Both edges on Pn.2 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS2_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.3 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS3_A {
    #[doc = "0: Interrupt on Pn.3 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.3 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS3_A> for bool {
    #[inline(always)]
    fn from(variant: IBS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS3` reader - Interrupt on Pn.3 is triggered ob both edges"]
pub type IBS3_R = crate::BitReader<IBS3_A>;
impl IBS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS3_A {
        match self.bits {
            false => IBS3_A::IEV,
            true => IBS3_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS3_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS3_A::BOTHEDGE
    }
}
#[doc = "Field `IBS3` writer - Interrupt on Pn.3 is triggered ob both edges"]
pub type IBS3_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS3_A, 3>;
impl<'a> IBS3_W<'a> {
    #[doc = "Interrupt on Pn.3 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS3_A::IEV)
    }
    #[doc = "Both edges on Pn.3 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS3_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.4 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS4_A {
    #[doc = "0: Interrupt on Pn.4 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.4 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS4_A> for bool {
    #[inline(always)]
    fn from(variant: IBS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS4` reader - Interrupt on Pn.4 is triggered ob both edges"]
pub type IBS4_R = crate::BitReader<IBS4_A>;
impl IBS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS4_A {
        match self.bits {
            false => IBS4_A::IEV,
            true => IBS4_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS4_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS4_A::BOTHEDGE
    }
}
#[doc = "Field `IBS4` writer - Interrupt on Pn.4 is triggered ob both edges"]
pub type IBS4_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS4_A, 4>;
impl<'a> IBS4_W<'a> {
    #[doc = "Interrupt on Pn.4 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS4_A::IEV)
    }
    #[doc = "Both edges on Pn.4 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS4_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.5 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS5_A {
    #[doc = "0: Interrupt on Pn.5 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.5 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS5_A> for bool {
    #[inline(always)]
    fn from(variant: IBS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS5` reader - Interrupt on Pn.5 is triggered ob both edges"]
pub type IBS5_R = crate::BitReader<IBS5_A>;
impl IBS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS5_A {
        match self.bits {
            false => IBS5_A::IEV,
            true => IBS5_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS5_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS5_A::BOTHEDGE
    }
}
#[doc = "Field `IBS5` writer - Interrupt on Pn.5 is triggered ob both edges"]
pub type IBS5_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS5_A, 5>;
impl<'a> IBS5_W<'a> {
    #[doc = "Interrupt on Pn.5 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS5_A::IEV)
    }
    #[doc = "Both edges on Pn.5 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS5_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.6 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS6_A {
    #[doc = "0: Interrupt on Pn.6 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.6 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS6_A> for bool {
    #[inline(always)]
    fn from(variant: IBS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS6` reader - Interrupt on Pn.6 is triggered ob both edges"]
pub type IBS6_R = crate::BitReader<IBS6_A>;
impl IBS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS6_A {
        match self.bits {
            false => IBS6_A::IEV,
            true => IBS6_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS6_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS6_A::BOTHEDGE
    }
}
#[doc = "Field `IBS6` writer - Interrupt on Pn.6 is triggered ob both edges"]
pub type IBS6_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS6_A, 6>;
impl<'a> IBS6_W<'a> {
    #[doc = "Interrupt on Pn.6 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS6_A::IEV)
    }
    #[doc = "Both edges on Pn.6 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS6_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.7 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS7_A {
    #[doc = "0: Interrupt on Pn.7 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.7 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS7_A> for bool {
    #[inline(always)]
    fn from(variant: IBS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS7` reader - Interrupt on Pn.7 is triggered ob both edges"]
pub type IBS7_R = crate::BitReader<IBS7_A>;
impl IBS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS7_A {
        match self.bits {
            false => IBS7_A::IEV,
            true => IBS7_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS7_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS7_A::BOTHEDGE
    }
}
#[doc = "Field `IBS7` writer - Interrupt on Pn.7 is triggered ob both edges"]
pub type IBS7_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS7_A, 7>;
impl<'a> IBS7_W<'a> {
    #[doc = "Interrupt on Pn.7 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS7_A::IEV)
    }
    #[doc = "Both edges on Pn.7 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS7_A::BOTHEDGE)
    }
}
#[doc = "Interrupt on Pn.8 is triggered ob both edges\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IBS8_A {
    #[doc = "0: Interrupt on Pn.8 is controlled by GPIOn_IEV register"]
    IEV = 0,
    #[doc = "1: Both edges on Pn.8 trigger an interrupt"]
    BOTHEDGE = 1,
}
impl From<IBS8_A> for bool {
    #[inline(always)]
    fn from(variant: IBS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBS8` reader - Interrupt on Pn.8 is triggered ob both edges"]
pub type IBS8_R = crate::BitReader<IBS8_A>;
impl IBS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IBS8_A {
        match self.bits {
            false => IBS8_A::IEV,
            true => IBS8_A::BOTHEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `IEV`"]
    #[inline(always)]
    pub fn is_iev(&self) -> bool {
        *self == IBS8_A::IEV
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == IBS8_A::BOTHEDGE
    }
}
#[doc = "Field `IBS8` writer - Interrupt on Pn.8 is triggered ob both edges"]
pub type IBS8_W<'a> = crate::BitWriter<'a, u32, IBS_SPEC, IBS8_A, 8>;
impl<'a> IBS8_W<'a> {
    #[doc = "Interrupt on Pn.8 is controlled by GPIOn_IEV register"]
    #[inline(always)]
    pub fn iev(self) -> &'a mut W {
        self.variant(IBS8_A::IEV)
    }
    #[doc = "Both edges on Pn.8 trigger an interrupt"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(IBS8_A::BOTHEDGE)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on Pn.0 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs0(&self) -> IBS0_R {
        IBS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs1(&self) -> IBS1_R {
        IBS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs2(&self) -> IBS2_R {
        IBS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs3(&self) -> IBS3_R {
        IBS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs4(&self) -> IBS4_R {
        IBS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs5(&self) -> IBS5_R {
        IBS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Pn.6 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs6(&self) -> IBS6_R {
        IBS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt on Pn.7 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs7(&self) -> IBS7_R {
        IBS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt on Pn.8 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs8(&self) -> IBS8_R {
        IBS8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on Pn.0 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs0(&mut self) -> IBS0_W {
        IBS0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt on Pn.1 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs1(&mut self) -> IBS1_W {
        IBS1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt on Pn.2 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs2(&mut self) -> IBS2_W {
        IBS2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt on Pn.3 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs3(&mut self) -> IBS3_W {
        IBS3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt on Pn.4 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs4(&mut self) -> IBS4_W {
        IBS4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on Pn.5 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs5(&mut self) -> IBS5_W {
        IBS5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt on Pn.6 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs6(&mut self) -> IBS6_W {
        IBS6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt on Pn.7 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs7(&mut self) -> IBS7_W {
        IBS7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt on Pn.8 is triggered ob both edges"]
    #[inline(always)]
    pub fn ibs8(&mut self) -> IBS8_W {
        IBS8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 GPIO Port n Interrupt Both-edge Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibs](index.html) module"]
pub struct IBS_SPEC;
impl crate::RegisterSpec for IBS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibs::R](R) reader structure"]
impl crate::Readable for IBS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibs::W](W) writer structure"]
impl crate::Writable for IBS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBS to value 0"]
impl crate::Resettable for IBS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
