#[doc = "Register `ENDPTCTRL[%s]` reader"]
pub type R = crate::R<ENDPTCTRL_SPEC>;
#[doc = "Register `ENDPTCTRL[%s]` writer"]
pub type W = crate::W<ENDPTCTRL_SPEC>;
#[doc = "Field `RXS` reader - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type RXS_R = crate::BitReader;
#[doc = "Field `RXS` writer - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type RXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXT` reader - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_R = crate::FieldReader;
#[doc = "Field `RXT` writer - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type RXT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXR` writer - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
pub type RXR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type RXE_R = crate::BitReader;
#[doc = "Field `RXE` writer - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type RXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXS` reader - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type TXS_R = crate::BitReader;
#[doc = "Field `TXS` writer - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
pub type TXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXT` reader - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_R = crate::FieldReader;
#[doc = "Field `TXT` writer - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
pub type TXT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXR` writer - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
pub type TXR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXE` reader - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    pub fn rxs(&self) -> RXS_R {
        RXS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 7 - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    pub fn txs(&self) -> TXS_R {
        TXS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 23 - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXS RX Endpoint Stall - Read/Write 0 End Point OK. \\[Default\\]
1 End Point Stalled This bit is set automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpointand this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    #[must_use]
    pub fn rxs(&mut self) -> RXS_W<ENDPTCTRL_SPEC> {
        RXS_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - RXT RX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RXT_W<ENDPTCTRL_SPEC> {
        RXT_W::new(self, 2)
    }
    #[doc = "Bit 6 - RXR RX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the host and device."]
    #[inline(always)]
    #[must_use]
    pub fn rxr(&mut self) -> RXR_W<ENDPTCTRL_SPEC> {
        RXR_W::new(self, 6)
    }
    #[doc = "Bit 7 - RXE RX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<ENDPTCTRL_SPEC> {
        RXE_W::new(self, 7)
    }
    #[doc = "Bit 16 - TXS TX Endpoint Stall - Read/Write 0 End Point OK 1 End Point Stalled This bit will be cleared automatically upon receipt of a SETUP request if this Endpoint is configured as a Control Endpoint and this bit will continue to be cleared by hardware until the associated ENDPTSETUPSTAT bit is cleared. Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host. This control will continue to STALL until this bit is either cleared by software or automatically cleared as above for control endpoints. NOTE: \\[CONTROL ENDPOINT TYPES ONLY\\]: there is a slight delay (50 clocks max) between the ENDPTSETUPSTAT begin cleared and hardware continuing to clear this bit. In most systems, it is unlikely the DCD software will observe this delay. However, should the DCD observe that the stall bit is not set after writing a one to it then follow this procedure: continually write this stall bit until it is set or until a new setup has been received by checking the associated endptsetupstat Bit."]
    #[inline(always)]
    #[must_use]
    pub fn txs(&mut self) -> TXS_W<ENDPTCTRL_SPEC> {
        TXS_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - TXT TX Endpoint Type - Read/Write 00 Control 01 Isochronous 10 Bulk 11 Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TXT_W<ENDPTCTRL_SPEC> {
        TXT_W::new(self, 18)
    }
    #[doc = "Bit 22 - TXR TX Data Toggle Reset (WS) Write 1 - Reset PID Sequence Whenever a configuration event is received for this Endpoint, software must write a one to this bit in order to synchronize the data PID's between the Host and device."]
    #[inline(always)]
    #[must_use]
    pub fn txr(&mut self) -> TXR_W<ENDPTCTRL_SPEC> {
        TXR_W::new(self, 22)
    }
    #[doc = "Bit 23 - TXE TX Endpoint Enable 0 Disabled \\[Default\\]
1 Enabled An Endpoint should be enabled only after it has been configured."]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<ENDPTCTRL_SPEC> {
        TXE_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTCTRL_SPEC;
impl crate::RegisterSpec for ENDPTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptctrl::R`](R) reader structure"]
impl crate::Readable for ENDPTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptctrl::W`](W) writer structure"]
impl crate::Writable for ENDPTCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTCTRL[%s]
to value 0"]
impl crate::Resettable for ENDPTCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
