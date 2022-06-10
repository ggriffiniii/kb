#[doc = "Register `INSTSC` writer"]
pub struct W(crate::W<INSTSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTSC_SPEC>;
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
impl From<crate::W<INSTSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "EP1 NAK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_NAKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP1_NAK bit"]
    CLEAR = 1,
}
impl From<EP1_NAKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP1_NAKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_NAKC` writer - EP1 NAK clear bit"]
pub type EP1_NAKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP1_NAKC_AW, 0>;
impl<'a> EP1_NAKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP1_NAKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP1_NAK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP1_NAKC_AW::CLEAR)
    }
}
#[doc = "EP2 NAK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_NAKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP2_NAK bit"]
    CLEAR = 1,
}
impl From<EP2_NAKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP2_NAKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_NAKC` writer - EP2 NAK clear bit"]
pub type EP2_NAKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP2_NAKC_AW, 1>;
impl<'a> EP2_NAKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP2_NAKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP2_NAK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP2_NAKC_AW::CLEAR)
    }
}
#[doc = "EP3 NAK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_NAKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP3_NAK bit"]
    CLEAR = 1,
}
impl From<EP3_NAKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP3_NAKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_NAKC` writer - EP3 NAK clear bit"]
pub type EP3_NAKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP3_NAKC_AW, 2>;
impl<'a> EP3_NAKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP3_NAKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP3_NAK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP3_NAKC_AW::CLEAR)
    }
}
#[doc = "EP4 NAK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_NAKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP4_NAK bit"]
    CLEAR = 1,
}
impl From<EP4_NAKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP4_NAKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_NAKC` writer - EP4 NAK clear bit"]
pub type EP4_NAKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP4_NAKC_AW, 3>;
impl<'a> EP4_NAKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP4_NAKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP4_NAK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP4_NAKC_AW::CLEAR)
    }
}
#[doc = "EP1 ACK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_ACKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP1_ACK bit"]
    CLEAR = 1,
}
impl From<EP1_ACKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP1_ACKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_ACKC` writer - EP1 ACK clear bit"]
pub type EP1_ACKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP1_ACKC_AW, 8>;
impl<'a> EP1_ACKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP1_ACKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP1_ACK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP1_ACKC_AW::CLEAR)
    }
}
#[doc = "EP2 ACK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_ACKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP2_ACK bit"]
    CLEAR = 1,
}
impl From<EP2_ACKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP2_ACKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_ACKC` writer - EP2 ACK clear bit"]
pub type EP2_ACKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP2_ACKC_AW, 9>;
impl<'a> EP2_ACKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP2_ACKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP2_ACK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP2_ACKC_AW::CLEAR)
    }
}
#[doc = "EP3 ACK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_ACKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP3_ACK bit"]
    CLEAR = 1,
}
impl From<EP3_ACKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP3_ACKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_ACKC` writer - EP3 ACK clear bit"]
pub type EP3_ACKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP3_ACKC_AW, 10>;
impl<'a> EP3_ACKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP3_ACKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP3_ACK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP3_ACKC_AW::CLEAR)
    }
}
#[doc = "EP4 ACK clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_ACKC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP4_ACK bit"]
    CLEAR = 1,
}
impl From<EP4_ACKC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP4_ACKC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_ACKC` writer - EP4 ACK clear bit"]
pub type EP4_ACKC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP4_ACKC_AW, 11>;
impl<'a> EP4_ACKC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP4_ACKC_AW::NOEFFECT)
    }
    #[doc = "Clear EP4_ACK bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP4_ACKC_AW::CLEAR)
    }
}
#[doc = "Timeout Error clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_TIMEOUTC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear ERR_TIMEOUT bit"]
    CLEAR = 1,
}
impl From<ERR_TIMEOUTC_AW> for bool {
    #[inline(always)]
    fn from(variant: ERR_TIMEOUTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_TIMEOUTC` writer - Timeout Error clear bit"]
pub type ERR_TIMEOUTC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, ERR_TIMEOUTC_AW, 17>;
impl<'a> ERR_TIMEOUTC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(ERR_TIMEOUTC_AW::NOEFFECT)
    }
    #[doc = "Clear ERR_TIMEOUT bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERR_TIMEOUTC_AW::CLEAR)
    }
}
#[doc = "Error Setup clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_SETUPC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear ERR_SETUP bit"]
    CLEAR = 1,
}
impl From<ERR_SETUPC_AW> for bool {
    #[inline(always)]
    fn from(variant: ERR_SETUPC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR_SETUPC` writer - Error Setup clear bit"]
pub type ERR_SETUPC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, ERR_SETUPC_AW, 18>;
impl<'a> ERR_SETUPC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(ERR_SETUPC_AW::NOEFFECT)
    }
    #[doc = "Clear ERR_SETUP bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERR_SETUPC_AW::CLEAR)
    }
}
#[doc = "EP0 OUT STALL clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_OUT_STALLC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_OUT_STALL bit"]
    CLEAR = 1,
}
impl From<EP0_OUT_STALLC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_OUT_STALLC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_OUT_STALLC` writer - EP0 OUT STALL clear bit"]
pub type EP0_OUT_STALLC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_OUT_STALLC_AW, 19>;
impl<'a> EP0_OUT_STALLC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_OUT_STALLC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_OUT_STALL bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_OUT_STALLC_AW::CLEAR)
    }
}
#[doc = "EP0 IN STALL clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_IN_STALLC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_IN_STALL bit"]
    CLEAR = 1,
}
impl From<EP0_IN_STALLC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_IN_STALLC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_IN_STALLC` writer - EP0 IN STALL clear bit"]
pub type EP0_IN_STALLC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_IN_STALLC_AW, 20>;
impl<'a> EP0_IN_STALLC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_IN_STALLC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_IN_STALL bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_IN_STALLC_AW::CLEAR)
    }
}
#[doc = "EP0 OUT clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_OUTC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_OUT bit"]
    CLEAR = 1,
}
impl From<EP0_OUTC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_OUTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_OUTC` writer - EP0 OUT clear bit"]
pub type EP0_OUTC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_OUTC_AW, 21>;
impl<'a> EP0_OUTC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_OUTC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_OUT bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_OUTC_AW::CLEAR)
    }
}
#[doc = "EP0 IN clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_INC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_IN bit"]
    CLEAR = 1,
}
impl From<EP0_INC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_INC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_INC` writer - EP0 IN clear bit"]
pub type EP0_INC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_INC_AW, 22>;
impl<'a> EP0_INC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_INC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_IN bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_INC_AW::CLEAR)
    }
}
#[doc = "EP0 SETUP clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_SETUPC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_SETUP bit"]
    CLEAR = 1,
}
impl From<EP0_SETUPC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_SETUPC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_SETUPC` writer - EP0 SETUP clear bit"]
pub type EP0_SETUPC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_SETUPC_AW, 23>;
impl<'a> EP0_SETUPC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_SETUPC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_SETUP bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_SETUPC_AW::CLEAR)
    }
}
#[doc = "EP0 PRESETUP clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0_PRESETUPC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear EP0_PRESETUP bit"]
    CLEAR = 1,
}
impl From<EP0_PRESETUPC_AW> for bool {
    #[inline(always)]
    fn from(variant: EP0_PRESETUPC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP0_PRESETUPC` writer - EP0 PRESETUP clear bit"]
pub type EP0_PRESETUPC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, EP0_PRESETUPC_AW, 24>;
impl<'a> EP0_PRESETUPC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(EP0_PRESETUPC_AW::NOEFFECT)
    }
    #[doc = "Clear EP0_PRESETUP bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EP0_PRESETUPC_AW::CLEAR)
    }
}
#[doc = "Bus Wakeup clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_WAKEUPC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear BUS_WAKEUP bit"]
    CLEAR = 1,
}
impl From<BUS_WAKEUPC_AW> for bool {
    #[inline(always)]
    fn from(variant: BUS_WAKEUPC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_WAKEUPC` writer - Bus Wakeup clear bit"]
pub type BUS_WAKEUPC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, BUS_WAKEUPC_AW, 25>;
impl<'a> BUS_WAKEUPC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BUS_WAKEUPC_AW::NOEFFECT)
    }
    #[doc = "Clear BUS_WAKEUP bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BUS_WAKEUPC_AW::CLEAR)
    }
}
#[doc = "USB SOF clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_SOFC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear USB_SOF bit"]
    CLEAR = 1,
}
impl From<USB_SOFC_AW> for bool {
    #[inline(always)]
    fn from(variant: USB_SOFC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB_SOFC` writer - USB SOF clear bit"]
pub type USB_SOFC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, USB_SOFC_AW, 26>;
impl<'a> USB_SOFC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(USB_SOFC_AW::NOEFFECT)
    }
    #[doc = "Clear USB_SOF bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(USB_SOFC_AW::CLEAR)
    }
}
#[doc = "USB Bus Resume clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_RESUMEC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear BUS_RESUME bit"]
    CLEAR = 1,
}
impl From<BUS_RESUMEC_AW> for bool {
    #[inline(always)]
    fn from(variant: BUS_RESUMEC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_RESUMEC` writer - USB Bus Resume clear bit"]
pub type BUS_RESUMEC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, BUS_RESUMEC_AW, 29>;
impl<'a> BUS_RESUMEC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BUS_RESUMEC_AW::NOEFFECT)
    }
    #[doc = "Clear BUS_RESUME bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BUS_RESUMEC_AW::CLEAR)
    }
}
#[doc = "USB Bus Reset clear bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_RESETC_AW {
    #[doc = "0: No effect"]
    NOEFFECT = 0,
    #[doc = "1: Clear BUS_RESET bit"]
    CLEAR = 1,
}
impl From<BUS_RESETC_AW> for bool {
    #[inline(always)]
    fn from(variant: BUS_RESETC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUS_RESETC` writer - USB Bus Reset clear bit"]
pub type BUS_RESETC_W<'a> = crate::BitWriter<'a, u32, INSTSC_SPEC, BUS_RESETC_AW, 31>;
impl<'a> BUS_RESETC_W<'a> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeffect(self) -> &'a mut W {
        self.variant(BUS_RESETC_AW::NOEFFECT)
    }
    #[doc = "Clear BUS_RESET bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BUS_RESETC_AW::CLEAR)
    }
}
impl W {
    #[doc = "Bit 0 - EP1 NAK clear bit"]
    #[inline(always)]
    pub fn ep1_nakc(&mut self) -> EP1_NAKC_W {
        EP1_NAKC_W::new(self)
    }
    #[doc = "Bit 1 - EP2 NAK clear bit"]
    #[inline(always)]
    pub fn ep2_nakc(&mut self) -> EP2_NAKC_W {
        EP2_NAKC_W::new(self)
    }
    #[doc = "Bit 2 - EP3 NAK clear bit"]
    #[inline(always)]
    pub fn ep3_nakc(&mut self) -> EP3_NAKC_W {
        EP3_NAKC_W::new(self)
    }
    #[doc = "Bit 3 - EP4 NAK clear bit"]
    #[inline(always)]
    pub fn ep4_nakc(&mut self) -> EP4_NAKC_W {
        EP4_NAKC_W::new(self)
    }
    #[doc = "Bit 8 - EP1 ACK clear bit"]
    #[inline(always)]
    pub fn ep1_ackc(&mut self) -> EP1_ACKC_W {
        EP1_ACKC_W::new(self)
    }
    #[doc = "Bit 9 - EP2 ACK clear bit"]
    #[inline(always)]
    pub fn ep2_ackc(&mut self) -> EP2_ACKC_W {
        EP2_ACKC_W::new(self)
    }
    #[doc = "Bit 10 - EP3 ACK clear bit"]
    #[inline(always)]
    pub fn ep3_ackc(&mut self) -> EP3_ACKC_W {
        EP3_ACKC_W::new(self)
    }
    #[doc = "Bit 11 - EP4 ACK clear bit"]
    #[inline(always)]
    pub fn ep4_ackc(&mut self) -> EP4_ACKC_W {
        EP4_ACKC_W::new(self)
    }
    #[doc = "Bit 17 - Timeout Error clear bit"]
    #[inline(always)]
    pub fn err_timeoutc(&mut self) -> ERR_TIMEOUTC_W {
        ERR_TIMEOUTC_W::new(self)
    }
    #[doc = "Bit 18 - Error Setup clear bit"]
    #[inline(always)]
    pub fn err_setupc(&mut self) -> ERR_SETUPC_W {
        ERR_SETUPC_W::new(self)
    }
    #[doc = "Bit 19 - EP0 OUT STALL clear bit"]
    #[inline(always)]
    pub fn ep0_out_stallc(&mut self) -> EP0_OUT_STALLC_W {
        EP0_OUT_STALLC_W::new(self)
    }
    #[doc = "Bit 20 - EP0 IN STALL clear bit"]
    #[inline(always)]
    pub fn ep0_in_stallc(&mut self) -> EP0_IN_STALLC_W {
        EP0_IN_STALLC_W::new(self)
    }
    #[doc = "Bit 21 - EP0 OUT clear bit"]
    #[inline(always)]
    pub fn ep0_outc(&mut self) -> EP0_OUTC_W {
        EP0_OUTC_W::new(self)
    }
    #[doc = "Bit 22 - EP0 IN clear bit"]
    #[inline(always)]
    pub fn ep0_inc(&mut self) -> EP0_INC_W {
        EP0_INC_W::new(self)
    }
    #[doc = "Bit 23 - EP0 SETUP clear bit"]
    #[inline(always)]
    pub fn ep0_setupc(&mut self) -> EP0_SETUPC_W {
        EP0_SETUPC_W::new(self)
    }
    #[doc = "Bit 24 - EP0 PRESETUP clear bit"]
    #[inline(always)]
    pub fn ep0_presetupc(&mut self) -> EP0_PRESETUPC_W {
        EP0_PRESETUPC_W::new(self)
    }
    #[doc = "Bit 25 - Bus Wakeup clear bit"]
    #[inline(always)]
    pub fn bus_wakeupc(&mut self) -> BUS_WAKEUPC_W {
        BUS_WAKEUPC_W::new(self)
    }
    #[doc = "Bit 26 - USB SOF clear bit"]
    #[inline(always)]
    pub fn usb_sofc(&mut self) -> USB_SOFC_W {
        USB_SOFC_W::new(self)
    }
    #[doc = "Bit 29 - USB Bus Resume clear bit"]
    #[inline(always)]
    pub fn bus_resumec(&mut self) -> BUS_RESUMEC_W {
        BUS_RESUMEC_W::new(self)
    }
    #[doc = "Bit 31 - USB Bus Reset clear bit"]
    #[inline(always)]
    pub fn bus_resetc(&mut self) -> BUS_RESETC_W {
        BUS_RESETC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset:0x08 USB Interrupt Event Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instsc](index.html) module"]
pub struct INSTSC_SPEC;
impl crate::RegisterSpec for INSTSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [instsc::W](W) writer structure"]
impl crate::Writable for INSTSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INSTSC to value 0"]
impl crate::Resettable for INSTSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
