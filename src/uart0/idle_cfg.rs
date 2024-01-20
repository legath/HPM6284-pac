#[doc = "Register `IDLE_CFG` reader"]
pub type R = crate::R<IDLE_CFG_SPEC>;
#[doc = "Register `IDLE_CFG` writer"]
pub type W = crate::W<IDLE_CFG_SPEC>;
#[doc = "Field `RX_IDLE_THR` reader - Threshold for UART Receive Idle detection (in terms of bits)"]
pub type RX_IDLE_THR_R = crate::FieldReader;
#[doc = "Field `RX_IDLE_THR` writer - Threshold for UART Receive Idle detection (in terms of bits)"]
pub type RX_IDLE_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_IDLE_EN` reader - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature"]
pub type RX_IDLE_EN_R = crate::BitReader;
#[doc = "Field `RX_IDLE_EN` writer - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature"]
pub type RX_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IDLE_COND` reader - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle"]
pub type RX_IDLE_COND_R = crate::BitReader;
#[doc = "Field `RX_IDLE_COND` writer - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle"]
pub type RX_IDLE_COND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Threshold for UART Receive Idle detection (in terms of bits)"]
    #[inline(always)]
    pub fn rx_idle_thr(&self) -> RX_IDLE_THR_R {
        RX_IDLE_THR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature"]
    #[inline(always)]
    pub fn rx_idle_en(&self) -> RX_IDLE_EN_R {
        RX_IDLE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle"]
    #[inline(always)]
    pub fn rx_idle_cond(&self) -> RX_IDLE_COND_R {
        RX_IDLE_COND_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Threshold for UART Receive Idle detection (in terms of bits)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle_thr(&mut self) -> RX_IDLE_THR_W<IDLE_CFG_SPEC> {
        RX_IDLE_THR_W::new(self, 0)
    }
    #[doc = "Bit 8 - UART Idle Detect Enable 0 - Disable 1 - Enable it should be enabled if enable address match feature"]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle_en(&mut self) -> RX_IDLE_EN_W<IDLE_CFG_SPEC> {
        RX_IDLE_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - IDLE Detection Condition 0 - Treat as idle if RX pin is logic one 1 - Treat as idle if UART state machine state is idle"]
    #[inline(always)]
    #[must_use]
    pub fn rx_idle_cond(&mut self) -> RX_IDLE_COND_W<IDLE_CFG_SPEC> {
        RX_IDLE_COND_W::new(self, 9)
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
#[doc = "Idle Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idle_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idle_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDLE_CFG_SPEC;
impl crate::RegisterSpec for IDLE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle_cfg::R`](R) reader structure"]
impl crate::Readable for IDLE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idle_cfg::W`](W) writer structure"]
impl crate::Writable for IDLE_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLE_CFG to value 0"]
impl crate::Resettable for IDLE_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
