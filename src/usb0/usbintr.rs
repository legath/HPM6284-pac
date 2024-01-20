#[doc = "Register `USBINTR` reader"]
pub type R = crate::R<USBINTR_SPEC>;
#[doc = "Register `USBINTR` writer"]
pub type W = crate::W<USBINTR_SPEC>;
#[doc = "Field `UE` reader - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type UE_R = crate::BitReader;
#[doc = "Field `UE` writer - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEE` reader - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type UEE_R = crate::BitReader;
#[doc = "Field `UEE` writer - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type UEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type PCE_R = crate::BitReader;
#[doc = "Field `PCE` writer - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRE` reader - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type FRE_R = crate::BitReader;
#[doc = "Field `FRE` writer - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type FRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEE` reader - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type SEE_R = crate::BitReader;
#[doc = "Field `SEE` writer - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type SEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAE` reader - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type AAE_R = crate::BitReader;
#[doc = "Field `AAE` writer - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
pub type AAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URE` reader - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
pub type URE_R = crate::BitReader;
#[doc = "Field `URE` writer - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
pub type URE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRE` reader - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type SRE_R = crate::BitReader;
#[doc = "Field `SRE` writer - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type SRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLE` reader - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
pub type SLE_R = crate::BitReader;
#[doc = "Field `SLE` writer - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
pub type SLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKE` reader - NAKE NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type NAKE_R = crate::BitReader;
#[doc = "Field `UAIE` reader - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
pub type UAIE_R = crate::BitReader;
#[doc = "Field `UAIE` writer - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
pub type UAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPIE` reader - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
pub type UPIE_R = crate::BitReader;
#[doc = "Field `UPIE` writer - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
pub type UPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE0` reader - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type TIE0_R = crate::BitReader;
#[doc = "Field `TIE0` writer - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type TIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE1` reader - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type TIE1_R = crate::BitReader;
#[doc = "Field `TIE1` writer - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt."]
pub type TIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn uee(&self) -> UEE_R {
        UEE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    pub fn see(&self) -> SEE_R {
        SEE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    pub fn aae(&self) -> AAE_R {
        AAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
    #[inline(always)]
    pub fn ure(&self) -> URE_R {
        URE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
    #[inline(always)]
    pub fn sle(&self) -> SLE_R {
        SLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - NAKE NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn nake(&self) -> NAKE_R {
        NAKE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
    #[inline(always)]
    pub fn uaie(&self) -> UAIE_R {
        UAIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn tie0(&self) -> TIE0_R {
        TIE0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UE USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<USBINTR_SPEC> {
        UE_W::new(self, 0)
    }
    #[doc = "Bit 1 - UEE USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn uee(&mut self) -> UEE_W<USBINTR_SPEC> {
        UEE_W::new(self, 1)
    }
    #[doc = "Bit 2 - PCE Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<USBINTR_SPEC> {
        PCE_W::new(self, 2)
    }
    #[doc = "Bit 3 - FRE Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn fre(&mut self) -> FRE_W<USBINTR_SPEC> {
        FRE_W::new(self, 3)
    }
    #[doc = "Bit 4 - SEE System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn see(&mut self) -> SEE_W<USBINTR_SPEC> {
        SEE_W::new(self, 4)
    }
    #[doc = "Bit 5 - AAE Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in host operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn aae(&mut self) -> AAE_W<USBINTR_SPEC> {
        AAE_W::new(self, 5)
    }
    #[doc = "Bit 6 - URE USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn ure(&mut self) -> URE_W<USBINTR_SPEC> {
        URE_W::new(self, 6)
    }
    #[doc = "Bit 7 - SRE SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<USBINTR_SPEC> {
        SRE_W::new(self, 7)
    }
    #[doc = "Bit 8 - SLE Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt. Only used in device operation mode."]
    #[inline(always)]
    #[must_use]
    pub fn sle(&mut self) -> SLE_W<USBINTR_SPEC> {
        SLE_W::new(self, 8)
    }
    #[doc = "Bit 18 - UAIE USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
    #[inline(always)]
    #[must_use]
    pub fn uaie(&mut self) -> UAIE_W<USBINTR_SPEC> {
        UAIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - UPIE USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold."]
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<USBINTR_SPEC> {
        UPIE_W::new(self, 19)
    }
    #[doc = "Bit 24 - TIE0 General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tie0(&mut self) -> TIE0_W<USBINTR_SPEC> {
        TIE0_W::new(self, 24)
    }
    #[doc = "Bit 25 - TIE1 General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tie1(&mut self) -> TIE1_W<USBINTR_SPEC> {
        TIE1_W::new(self, 25)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBINTR_SPEC;
impl crate::RegisterSpec for USBINTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintr::R`](R) reader structure"]
impl crate::Readable for USBINTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbintr::W`](W) writer structure"]
impl crate::Writable for USBINTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBINTR to value 0"]
impl crate::Resettable for USBINTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
