#[doc = "Register `irq_sts` reader"]
pub type R = crate::R<IRQ_STS_SPEC>;
#[doc = "Register `irq_sts` writer"]
pub type W = crate::W<IRQ_STS_SPEC>;
#[doc = "Field `BUF0_CMPT` writer - No description avaiable"]
pub type BUF0_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1_CMPT` writer - No description avaiable"]
pub type BUF1_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY` writer - No description avaiable"]
pub type FIFO_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_ERROR` writer - set if hresp==2'b01(ERROR)"]
pub type AHB_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_CMPT` writer - No description avaiable"]
pub type STEP_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn buf0_cmpt(&mut self) -> BUF0_CMPT_W<IRQ_STS_SPEC> {
        BUF0_CMPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn buf1_cmpt(&mut self) -> BUF1_CMPT_W<IRQ_STS_SPEC> {
        BUF1_CMPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty(&mut self) -> FIFO_EMPTY_W<IRQ_STS_SPEC> {
        FIFO_EMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3 - set if hresp==2'b01(ERROR)"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_error(&mut self) -> AHB_ERROR_W<IRQ_STS_SPEC> {
        AHB_ERROR_W::new(self, 3)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn step_cmpt(&mut self) -> STEP_CMPT_W<IRQ_STS_SPEC> {
        STEP_CMPT_W::new(self, 4)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_STS_SPEC;
impl crate::RegisterSpec for IRQ_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_sts::R`](R) reader structure"]
impl crate::Readable for IRQ_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_sts::W`](W) writer structure"]
impl crate::Writable for IRQ_STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq_sts to value 0"]
impl crate::Resettable for IRQ_STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
