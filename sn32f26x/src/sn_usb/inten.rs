#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EP1 NAK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_NAK_EN_A {
    #[doc = "0: Disable EP1 NAK interrupt function"]
    DISABLE = 0,
    #[doc = "1: Enable EP1 NAK interrupt function"]
    ENABLE = 1,
}
impl From<EP1_NAK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_NAK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_NAK_EN` reader - EP1 NAK Interrupt Enable"]
pub type EP1_NAK_EN_R = crate::BitReader<EP1_NAK_EN_A>;
impl EP1_NAK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_NAK_EN_A {
        match self.bits {
            false => EP1_NAK_EN_A::DISABLE,
            true => EP1_NAK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EP1_NAK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EP1_NAK_EN_A::ENABLE
    }
}
#[doc = "Field `EP1_NAK_EN` writer - EP1 NAK Interrupt Enable"]
pub type EP1_NAK_EN_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, EP1_NAK_EN_A, 0>;
impl<'a> EP1_NAK_EN_W<'a> {
    #[doc = "Disable EP1 NAK interrupt function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EP1_NAK_EN_A::DISABLE)
    }
    #[doc = "Enable EP1 NAK interrupt function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EP1_NAK_EN_A::ENABLE)
    }
}
#[doc = "EP2 NAK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_NAK_EN_A {
    #[doc = "0: Disable EP2 NAK interrupt function"]
    DISABLE = 0,
    #[doc = "1: Enable EP2 NAK interrupt function"]
    ENABLE = 1,
}
impl From<EP2_NAK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_NAK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_NAK_EN` reader - EP2 NAK Interrupt Enable"]
pub type EP2_NAK_EN_R = crate::BitReader<EP2_NAK_EN_A>;
impl EP2_NAK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP2_NAK_EN_A {
        match self.bits {
            false => EP2_NAK_EN_A::DISABLE,
            true => EP2_NAK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EP2_NAK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EP2_NAK_EN_A::ENABLE
    }
}
#[doc = "Field `EP2_NAK_EN` writer - EP2 NAK Interrupt Enable"]
pub type EP2_NAK_EN_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, EP2_NAK_EN_A, 1>;
impl<'a> EP2_NAK_EN_W<'a> {
    #[doc = "Disable EP2 NAK interrupt function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EP2_NAK_EN_A::DISABLE)
    }
    #[doc = "Enable EP2 NAK interrupt function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EP2_NAK_EN_A::ENABLE)
    }
}
#[doc = "EP3 NAK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_NAK_EN_A {
    #[doc = "0: Disable EP3 NAK interrupt function"]
    DISABLE = 0,
    #[doc = "1: Enable EP3 NAK interrupt function"]
    ENABLE = 1,
}
impl From<EP3_NAK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_NAK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_NAK_EN` reader - EP3 NAK Interrupt Enable"]
pub type EP3_NAK_EN_R = crate::BitReader<EP3_NAK_EN_A>;
impl EP3_NAK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP3_NAK_EN_A {
        match self.bits {
            false => EP3_NAK_EN_A::DISABLE,
            true => EP3_NAK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EP3_NAK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EP3_NAK_EN_A::ENABLE
    }
}
#[doc = "Field `EP3_NAK_EN` writer - EP3 NAK Interrupt Enable"]
pub type EP3_NAK_EN_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, EP3_NAK_EN_A, 2>;
impl<'a> EP3_NAK_EN_W<'a> {
    #[doc = "Disable EP3 NAK interrupt function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EP3_NAK_EN_A::DISABLE)
    }
    #[doc = "Enable EP3 NAK interrupt function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EP3_NAK_EN_A::ENABLE)
    }
}
#[doc = "EP4 NAK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_NAK_EN_A {
    #[doc = "0: Disable EP4 NAK interrupt function"]
    DISABLE = 0,
    #[doc = "1: Enable EP4 NAK interrupt function"]
    ENABLE = 1,
}
impl From<EP4_NAK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_NAK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_NAK_EN` reader - EP4 NAK Interrupt Enable"]
pub type EP4_NAK_EN_R = crate::BitReader<EP4_NAK_EN_A>;
impl EP4_NAK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP4_NAK_EN_A {
        match self.bits {
            false => EP4_NAK_EN_A::DISABLE,
            true => EP4_NAK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EP4_NAK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EP4_NAK_EN_A::ENABLE
    }
}
#[doc = "Field `EP4_NAK_EN` writer - EP4 NAK Interrupt Enable"]
pub type EP4_NAK_EN_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, EP4_NAK_EN_A, 3>;
impl<'a> EP4_NAK_EN_W<'a> {
    #[doc = "Disable EP4 NAK interrupt function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EP4_NAK_EN_A::DISABLE)
    }
    #[doc = "Enable EP4 NAK interrupt function"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EP4_NAK_EN_A::ENABLE)
    }
}
#[doc = "Enable all of EP(1~4) ACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPN_ACK_EN_A {
    #[doc = "0: Disable EP1 to 4 ACK interrupt function"]
    DISABLE = 0,
    #[doc = "1: Enable EP1 to 4 ACK interrupt function."]
    ENABLE = 1,
}
impl From<EPN_ACK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: EPN_ACK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPN_ACK_EN` reader - Enable all of EP(1~4) ACK Interrupt"]
pub type EPN_ACK_EN_R = crate::BitReader<EPN_ACK_EN_A>;
impl EPN_ACK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPN_ACK_EN_A {
        match self.bits {
            false => EPN_ACK_EN_A::DISABLE,
            true => EPN_ACK_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EPN_ACK_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EPN_ACK_EN_A::ENABLE
    }
}
#[doc = "Field `EPN_ACK_EN` writer - Enable all of EP(1~4) ACK Interrupt"]
pub type EPN_ACK_EN_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, EPN_ACK_EN_A, 4>;
impl<'a> EPN_ACK_EN_W<'a> {
    #[doc = "Disable EP1 to 4 ACK interrupt function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EPN_ACK_EN_A::DISABLE)
    }
    #[doc = "Enable EP1 to 4 ACK interrupt function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EPN_ACK_EN_A::ENABLE)
    }
}
#[doc = "Bus Wake Up Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSWK_IE_A {
    #[doc = "0: Disable Wake Up event interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable Wake Up event interrupt"]
    ENABLE = 1,
}
impl From<BUSWK_IE_A> for bool {
    #[inline(always)]
    fn from(variant: BUSWK_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSWK_IE` reader - Bus Wake Up Interrupt Enable"]
pub type BUSWK_IE_R = crate::BitReader<BUSWK_IE_A>;
impl BUSWK_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSWK_IE_A {
        match self.bits {
            false => BUSWK_IE_A::DISABLE,
            true => BUSWK_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUSWK_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUSWK_IE_A::ENABLE
    }
}
#[doc = "Field `BUSWK_IE` writer - Bus Wake Up Interrupt Enable"]
pub type BUSWK_IE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, BUSWK_IE_A, 28>;
impl<'a> BUSWK_IE_W<'a> {
    #[doc = "Disable Wake Up event interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUSWK_IE_A::DISABLE)
    }
    #[doc = "Enable Wake Up event interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUSWK_IE_A::ENABLE)
    }
}
#[doc = "USB Event Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_IE_A {
    #[doc = "0: Disable USB event interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable USB event interrupt"]
    ENABLE = 1,
}
impl From<USB_IE_A> for bool {
    #[inline(always)]
    fn from(variant: USB_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_IE` reader - USB Event Interrupt Enable"]
pub type USB_IE_R = crate::BitReader<USB_IE_A>;
impl USB_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_IE_A {
        match self.bits {
            false => USB_IE_A::DISABLE,
            true => USB_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB_IE_A::ENABLE
    }
}
#[doc = "Field `USB_IE` writer - USB Event Interrupt Enable"]
pub type USB_IE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, USB_IE_A, 29>;
impl<'a> USB_IE_W<'a> {
    #[doc = "Disable USB event interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB_IE_A::DISABLE)
    }
    #[doc = "Enable USB event interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB_IE_A::ENABLE)
    }
}
#[doc = "USB SOF Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_SOF_IE_A {
    #[doc = "0: Disable USB SOF interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable USB SOF interrupt"]
    ENABLE = 1,
}
impl From<USB_SOF_IE_A> for bool {
    #[inline(always)]
    fn from(variant: USB_SOF_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_SOF_IE` reader - USB SOF Interrupt Enable"]
pub type USB_SOF_IE_R = crate::BitReader<USB_SOF_IE_A>;
impl USB_SOF_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_SOF_IE_A {
        match self.bits {
            false => USB_SOF_IE_A::DISABLE,
            true => USB_SOF_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USB_SOF_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == USB_SOF_IE_A::ENABLE
    }
}
#[doc = "Field `USB_SOF_IE` writer - USB SOF Interrupt Enable"]
pub type USB_SOF_IE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, USB_SOF_IE_A, 30>;
impl<'a> USB_SOF_IE_W<'a> {
    #[doc = "Disable USB SOF interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USB_SOF_IE_A::DISABLE)
    }
    #[doc = "Enable USB SOF interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(USB_SOF_IE_A::ENABLE)
    }
}
#[doc = "Bus Event Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_IE_A {
    #[doc = "0: Disable BUS event interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable Bus event interrupt"]
    ENABLE = 1,
}
impl From<BUS_IE_A> for bool {
    #[inline(always)]
    fn from(variant: BUS_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_IE` reader - Bus Event Interrupt Enable"]
pub type BUS_IE_R = crate::BitReader<BUS_IE_A>;
impl BUS_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUS_IE_A {
        match self.bits {
            false => BUS_IE_A::DISABLE,
            true => BUS_IE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BUS_IE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BUS_IE_A::ENABLE
    }
}
#[doc = "Field `BUS_IE` writer - Bus Event Interrupt Enable"]
pub type BUS_IE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, BUS_IE_A, 31>;
impl<'a> BUS_IE_W<'a> {
    #[doc = "Disable BUS event interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BUS_IE_A::DISABLE)
    }
    #[doc = "Enable Bus event interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BUS_IE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - EP1 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep1_nak_en(&self) -> EP1_NAK_EN_R {
        EP1_NAK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EP2 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep2_nak_en(&self) -> EP2_NAK_EN_R {
        EP2_NAK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP3 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep3_nak_en(&self) -> EP3_NAK_EN_R {
        EP3_NAK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP4 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep4_nak_en(&self) -> EP4_NAK_EN_R {
        EP4_NAK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable all of EP(1~4) ACK Interrupt"]
    #[inline(always)]
    pub fn epn_ack_en(&self) -> EPN_ACK_EN_R {
        EPN_ACK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 28 - Bus Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn buswk_ie(&self) -> BUSWK_IE_R {
        BUSWK_IE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - USB Event Interrupt Enable"]
    #[inline(always)]
    pub fn usb_ie(&self) -> USB_IE_R {
        USB_IE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB SOF Interrupt Enable"]
    #[inline(always)]
    pub fn usb_sof_ie(&self) -> USB_SOF_IE_R {
        USB_SOF_IE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Event Interrupt Enable"]
    #[inline(always)]
    pub fn bus_ie(&self) -> BUS_IE_R {
        BUS_IE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EP1 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep1_nak_en(&mut self) -> EP1_NAK_EN_W {
        EP1_NAK_EN_W::new(self)
    }
    #[doc = "Bit 1 - EP2 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep2_nak_en(&mut self) -> EP2_NAK_EN_W {
        EP2_NAK_EN_W::new(self)
    }
    #[doc = "Bit 2 - EP3 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep3_nak_en(&mut self) -> EP3_NAK_EN_W {
        EP3_NAK_EN_W::new(self)
    }
    #[doc = "Bit 3 - EP4 NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep4_nak_en(&mut self) -> EP4_NAK_EN_W {
        EP4_NAK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Enable all of EP(1~4) ACK Interrupt"]
    #[inline(always)]
    pub fn epn_ack_en(&mut self) -> EPN_ACK_EN_W {
        EPN_ACK_EN_W::new(self)
    }
    #[doc = "Bit 28 - Bus Wake Up Interrupt Enable"]
    #[inline(always)]
    pub fn buswk_ie(&mut self) -> BUSWK_IE_W {
        BUSWK_IE_W::new(self)
    }
    #[doc = "Bit 29 - USB Event Interrupt Enable"]
    #[inline(always)]
    pub fn usb_ie(&mut self) -> USB_IE_W {
        USB_IE_W::new(self)
    }
    #[doc = "Bit 30 - USB SOF Interrupt Enable"]
    #[inline(always)]
    pub fn usb_sof_ie(&mut self) -> USB_SOF_IE_W {
        USB_SOF_IE_W::new(self)
    }
    #[doc = "Bit 31 - Bus Event Interrupt Enable"]
    #[inline(always)]
    pub fn bus_ie(&mut self) -> BUS_IE_W {
        BUS_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x00 USB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
