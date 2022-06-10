#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Endpoint 1 IN/OUT direction setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_DIR_A {
    #[doc = "0: EP1 only handshakes to IN token packet"]
    IN = 0,
    #[doc = "1: EP1 only handshakes to OUT token packet"]
    OUT = 1,
}
impl From<EP1_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_DIR` reader - Endpoint 1 IN/OUT direction setting"]
pub type EP1_DIR_R = crate::BitReader<EP1_DIR_A>;
impl EP1_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_DIR_A {
        match self.bits {
            false => EP1_DIR_A::IN,
            true => EP1_DIR_A::OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EP1_DIR_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EP1_DIR_A::OUT
    }
}
#[doc = "Field `EP1_DIR` writer - Endpoint 1 IN/OUT direction setting"]
pub type EP1_DIR_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, EP1_DIR_A, 0>;
impl<'a> EP1_DIR_W<'a> {
    #[doc = "EP1 only handshakes to IN token packet"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EP1_DIR_A::IN)
    }
    #[doc = "EP1 only handshakes to OUT token packet"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EP1_DIR_A::OUT)
    }
}
#[doc = "Endpoint 2 IN/OUT direction setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_DIR_A {
    #[doc = "0: EP2 only handshakes to IN token packet"]
    IN = 0,
    #[doc = "1: EP2 only handshakes to OUT token packet"]
    OUT = 1,
}
impl From<EP2_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_DIR` reader - Endpoint 2 IN/OUT direction setting"]
pub type EP2_DIR_R = crate::BitReader<EP2_DIR_A>;
impl EP2_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP2_DIR_A {
        match self.bits {
            false => EP2_DIR_A::IN,
            true => EP2_DIR_A::OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EP2_DIR_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EP2_DIR_A::OUT
    }
}
#[doc = "Field `EP2_DIR` writer - Endpoint 2 IN/OUT direction setting"]
pub type EP2_DIR_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, EP2_DIR_A, 1>;
impl<'a> EP2_DIR_W<'a> {
    #[doc = "EP2 only handshakes to IN token packet"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EP2_DIR_A::IN)
    }
    #[doc = "EP2 only handshakes to OUT token packet"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EP2_DIR_A::OUT)
    }
}
#[doc = "Endpoint 3 IN/OUT direction setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_DIR_A {
    #[doc = "0: EP3 only handshakes to IN token packet"]
    IN = 0,
    #[doc = "1: EP3 only handshakes to OUT token packet"]
    OUT = 1,
}
impl From<EP3_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_DIR` reader - Endpoint 3 IN/OUT direction setting"]
pub type EP3_DIR_R = crate::BitReader<EP3_DIR_A>;
impl EP3_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP3_DIR_A {
        match self.bits {
            false => EP3_DIR_A::IN,
            true => EP3_DIR_A::OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EP3_DIR_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EP3_DIR_A::OUT
    }
}
#[doc = "Field `EP3_DIR` writer - Endpoint 3 IN/OUT direction setting"]
pub type EP3_DIR_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, EP3_DIR_A, 2>;
impl<'a> EP3_DIR_W<'a> {
    #[doc = "EP3 only handshakes to IN token packet"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EP3_DIR_A::IN)
    }
    #[doc = "EP3 only handshakes to OUT token packet"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EP3_DIR_A::OUT)
    }
}
#[doc = "Endpoint 4 IN/OUT direction setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_DIR_A {
    #[doc = "0: EP4 only handshakes to IN token packet"]
    IN = 0,
    #[doc = "1: EP4 only handshakes to OUT token packet"]
    OUT = 1,
}
impl From<EP4_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_DIR` reader - Endpoint 4 IN/OUT direction setting"]
pub type EP4_DIR_R = crate::BitReader<EP4_DIR_A>;
impl EP4_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP4_DIR_A {
        match self.bits {
            false => EP4_DIR_A::IN,
            true => EP4_DIR_A::OUT,
        }
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == EP4_DIR_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == EP4_DIR_A::OUT
    }
}
#[doc = "Field `EP4_DIR` writer - Endpoint 4 IN/OUT direction setting"]
pub type EP4_DIR_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, EP4_DIR_A, 3>;
impl<'a> EP4_DIR_W<'a> {
    #[doc = "EP4 only handshakes to IN token packet"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(EP4_DIR_A::IN)
    }
    #[doc = "EP4 only handshakes to OUT token packet"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(EP4_DIR_A::OUT)
    }
}
#[doc = "Enable internal D+ and D- 175k pull-down resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIS_PDEN_A {
    #[doc = "1: Enable"]
    ENABLE = 1,
    #[doc = "0: Disable"]
    DISABLE = 0,
}
impl From<DIS_PDEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_PDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS_PDEN` reader - Enable internal D+ and D- 175k pull-down resistor"]
pub type DIS_PDEN_R = crate::BitReader<DIS_PDEN_A>;
impl DIS_PDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_PDEN_A {
        match self.bits {
            true => DIS_PDEN_A::ENABLE,
            false => DIS_PDEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DIS_PDEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DIS_PDEN_A::DISABLE
    }
}
#[doc = "Field `DIS_PDEN` writer - Enable internal D+ and D- 175k pull-down resistor"]
pub type DIS_PDEN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DIS_PDEN_A, 26>;
impl<'a> DIS_PDEN_W<'a> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DIS_PDEN_A::ENABLE)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DIS_PDEN_A::DISABLE)
    }
}
#[doc = "Enable USB anti-ESD protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESD_EN_A {
    #[doc = "0: Disable anti-ESD protection"]
    DISABLE = 0,
    #[doc = "1: Enable anti-ESD protection"]
    ENABLE = 1,
}
impl From<ESD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ESD_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESD_EN` reader - Enable USB anti-ESD protection"]
pub type ESD_EN_R = crate::BitReader<ESD_EN_A>;
impl ESD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESD_EN_A {
        match self.bits {
            false => ESD_EN_A::DISABLE,
            true => ESD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ESD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ESD_EN_A::ENABLE
    }
}
#[doc = "Field `ESD_EN` writer - Enable USB anti-ESD protection"]
pub type ESD_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, ESD_EN_A, 27>;
impl<'a> ESD_EN_W<'a> {
    #[doc = "Disable anti-ESD protection"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ESD_EN_A::DISABLE)
    }
    #[doc = "Enable anti-ESD protection"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ESD_EN_A::ENABLE)
    }
}
#[doc = "USB Serial Interface Engine Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIE_EN_A {
    #[doc = "0: Disable USB SIE function"]
    DISABLE = 0,
    #[doc = "1: Enable USB SIE function"]
    ENABLE = 1,
}
impl From<SIE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SIE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIE_EN` reader - USB Serial Interface Engine Enable"]
pub type SIE_EN_R = crate::BitReader<SIE_EN_A>;
impl SIE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIE_EN_A {
        match self.bits {
            false => SIE_EN_A::DISABLE,
            true => SIE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SIE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SIE_EN_A::ENABLE
    }
}
#[doc = "Field `SIE_EN` writer - USB Serial Interface Engine Enable"]
pub type SIE_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, SIE_EN_A, 28>;
impl<'a> SIE_EN_W<'a> {
    #[doc = "Disable USB SIE function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SIE_EN_A::DISABLE)
    }
    #[doc = "Enable USB SIE function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SIE_EN_A::ENABLE)
    }
}
#[doc = "Enable internal D+ 1.5k pull-up resistor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPPU_EN_A {
    #[doc = "0: Disable internal D+ pull-up resistor"]
    DISABLE = 0,
    #[doc = "1: Enable internal D+ pull-up resistor"]
    ENABLE = 1,
}
impl From<DPPU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DPPU_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPU_EN` reader - Enable internal D+ 1.5k pull-up resistor"]
pub type DPPU_EN_R = crate::BitReader<DPPU_EN_A>;
impl DPPU_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPPU_EN_A {
        match self.bits {
            false => DPPU_EN_A::DISABLE,
            true => DPPU_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DPPU_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DPPU_EN_A::ENABLE
    }
}
#[doc = "Field `DPPU_EN` writer - Enable internal D+ 1.5k pull-up resistor"]
pub type DPPU_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, DPPU_EN_A, 29>;
impl<'a> DPPU_EN_W<'a> {
    #[doc = "Disable internal D+ pull-up resistor"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPPU_EN_A::DISABLE)
    }
    #[doc = "Enable internal D+ pull-up resistor"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPPU_EN_A::ENABLE)
    }
}
#[doc = "PHY Transceiver Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHY_EN_A {
    #[doc = "0: Disable PHY transceiver function"]
    DISABLE = 0,
    #[doc = "1: Enable PHY transceiver function"]
    ENABLE = 1,
}
impl From<PHY_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PHY_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHY_EN` reader - PHY Transceiver Function Enable"]
pub type PHY_EN_R = crate::BitReader<PHY_EN_A>;
impl PHY_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHY_EN_A {
        match self.bits {
            false => PHY_EN_A::DISABLE,
            true => PHY_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PHY_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PHY_EN_A::ENABLE
    }
}
#[doc = "Field `PHY_EN` writer - PHY Transceiver Function Enable"]
pub type PHY_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, PHY_EN_A, 30>;
impl<'a> PHY_EN_W<'a> {
    #[doc = "Disable PHY transceiver function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PHY_EN_A::DISABLE)
    }
    #[doc = "Enable PHY transceiver function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PHY_EN_A::ENABLE)
    }
}
#[doc = "Enable the internal VREG33 ouput\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREG33_EN_A {
    #[doc = "0: Disable VREG33 ouput"]
    DISABLE = 0,
    #[doc = "1: Enable VREG33 ouput"]
    ENABLE = 1,
}
impl From<VREG33_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VREG33_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREG33_EN` reader - Enable the internal VREG33 ouput"]
pub type VREG33_EN_R = crate::BitReader<VREG33_EN_A>;
impl VREG33_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREG33_EN_A {
        match self.bits {
            false => VREG33_EN_A::DISABLE,
            true => VREG33_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == VREG33_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == VREG33_EN_A::ENABLE
    }
}
#[doc = "Field `VREG33_EN` writer - Enable the internal VREG33 ouput"]
pub type VREG33_EN_W<'a> = crate::BitWriter<'a, u32, CFG_SPEC, VREG33_EN_A, 31>;
impl<'a> VREG33_EN_W<'a> {
    #[doc = "Disable VREG33 ouput"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(VREG33_EN_A::DISABLE)
    }
    #[doc = "Enable VREG33 ouput"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(VREG33_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint 1 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep1_dir(&self) -> EP1_DIR_R {
        EP1_DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 2 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep2_dir(&self) -> EP2_DIR_R {
        EP2_DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 3 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep3_dir(&self) -> EP3_DIR_R {
        EP3_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 4 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep4_dir(&self) -> EP4_DIR_R {
        EP4_DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable internal D+ and D- 175k pull-down resistor"]
    #[inline(always)]
    pub fn dis_pden(&self) -> DIS_PDEN_R {
        DIS_PDEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable USB anti-ESD protection"]
    #[inline(always)]
    pub fn esd_en(&self) -> ESD_EN_R {
        ESD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB Serial Interface Engine Enable"]
    #[inline(always)]
    pub fn sie_en(&self) -> SIE_EN_R {
        SIE_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable internal D+ 1.5k pull-up resistor"]
    #[inline(always)]
    pub fn dppu_en(&self) -> DPPU_EN_R {
        DPPU_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PHY Transceiver Function Enable"]
    #[inline(always)]
    pub fn phy_en(&self) -> PHY_EN_R {
        PHY_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable the internal VREG33 ouput"]
    #[inline(always)]
    pub fn vreg33_en(&self) -> VREG33_EN_R {
        VREG33_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 1 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep1_dir(&mut self) -> EP1_DIR_W {
        EP1_DIR_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint 2 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep2_dir(&mut self) -> EP2_DIR_W {
        EP2_DIR_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint 3 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep3_dir(&mut self) -> EP3_DIR_W {
        EP3_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint 4 IN/OUT direction setting"]
    #[inline(always)]
    pub fn ep4_dir(&mut self) -> EP4_DIR_W {
        EP4_DIR_W::new(self)
    }
    #[doc = "Bit 26 - Enable internal D+ and D- 175k pull-down resistor"]
    #[inline(always)]
    pub fn dis_pden(&mut self) -> DIS_PDEN_W {
        DIS_PDEN_W::new(self)
    }
    #[doc = "Bit 27 - Enable USB anti-ESD protection"]
    #[inline(always)]
    pub fn esd_en(&mut self) -> ESD_EN_W {
        ESD_EN_W::new(self)
    }
    #[doc = "Bit 28 - USB Serial Interface Engine Enable"]
    #[inline(always)]
    pub fn sie_en(&mut self) -> SIE_EN_W {
        SIE_EN_W::new(self)
    }
    #[doc = "Bit 29 - Enable internal D+ 1.5k pull-up resistor"]
    #[inline(always)]
    pub fn dppu_en(&mut self) -> DPPU_EN_W {
        DPPU_EN_W::new(self)
    }
    #[doc = "Bit 30 - PHY Transceiver Function Enable"]
    #[inline(always)]
    pub fn phy_en(&mut self) -> PHY_EN_W {
        PHY_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable the internal VREG33 ouput"]
    #[inline(always)]
    pub fn vreg33_en(&mut self) -> VREG33_EN_W {
        VREG33_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x10 USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x8000_0000"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
