#[doc = "Register `int_sts` reader"]
pub type R = crate::R<INT_STS_SPEC>;
#[doc = "Register `int_sts` writer"]
pub type W = crate::W<INT_STS_SPEC>;
#[doc = "Field `WDOG` writer - set if one chanel watch dog event triggered"]
pub type WDOG_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `AHB_ERR` reader - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr"]
pub type AHB_ERR_R = crate::BitReader;
#[doc = "Field `AHB_ERR` writer - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr"]
pub type AHB_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_FIFO_FULL` reader - DMA fifo full interrupt, user need to check clock frequency if it's set."]
pub type DMA_FIFO_FULL_R = crate::BitReader;
#[doc = "Field `DMA_FIFO_FULL` writer - DMA fifo full interrupt, user need to check clock frequency if it's set."]
pub type DMA_FIFO_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_CVC` writer - one conversion complete in seq_queue if related seq_int_en is set"]
pub type SEQ_CVC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_CMPT` writer - the whole sequence complete interrupt"]
pub type SEQ_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_DMAABT` writer - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
pub type SEQ_DMAABT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_HW_CFLCT` reader - No description avaiable"]
pub type SEQ_HW_CFLCT_R = crate::BitReader;
#[doc = "Field `SEQ_HW_CFLCT` writer - No description avaiable"]
pub type SEQ_HW_CFLCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_SW_CFLCT` writer - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
pub type SEQ_SW_CFLCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_CFLCT` writer - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
pub type READ_CFLCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_HW_CFLCT` reader - No description avaiable"]
pub type TRIG_HW_CFLCT_R = crate::BitReader;
#[doc = "Field `TRIG_HW_CFLCT` writer - No description avaiable"]
pub type TRIG_HW_CFLCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_SW_CFLCT` writer - No description avaiable"]
pub type TRIG_SW_CFLCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIG_CMPT` writer - interrupt for one trigger conversion complete if enabled"]
pub type TRIG_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AHB_ERR_R {
        AHB_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA fifo full interrupt, user need to check clock frequency if it's set."]
    #[inline(always)]
    pub fn dma_fifo_full(&self) -> DMA_FIFO_FULL_R {
        DMA_FIFO_FULL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - No description avaiable"]
    #[inline(always)]
    pub fn seq_hw_cflct(&self) -> SEQ_HW_CFLCT_R {
        SEQ_HW_CFLCT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - No description avaiable"]
    #[inline(always)]
    pub fn trig_hw_cflct(&self) -> TRIG_HW_CFLCT_R {
        TRIG_HW_CFLCT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - set if one chanel watch dog event triggered"]
    #[inline(always)]
    #[must_use]
    pub fn wdog(&mut self) -> WDOG_W<INT_STS_SPEC> {
        WDOG_W::new(self, 0)
    }
    #[doc = "Bit 21 - set if got hresp=1, generally caused by wrong trg_dma_addr or seq_dma_addr"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_err(&mut self) -> AHB_ERR_W<INT_STS_SPEC> {
        AHB_ERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA fifo full interrupt, user need to check clock frequency if it's set."]
    #[inline(always)]
    #[must_use]
    pub fn dma_fifo_full(&mut self) -> DMA_FIFO_FULL_W<INT_STS_SPEC> {
        DMA_FIFO_FULL_W::new(self, 22)
    }
    #[doc = "Bit 23 - one conversion complete in seq_queue if related seq_int_en is set"]
    #[inline(always)]
    #[must_use]
    pub fn seq_cvc(&mut self) -> SEQ_CVC_W<INT_STS_SPEC> {
        SEQ_CVC_W::new(self, 23)
    }
    #[doc = "Bit 24 - the whole sequence complete interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn seq_cmpt(&mut self) -> SEQ_CMPT_W<INT_STS_SPEC> {
        SEQ_CMPT_W::new(self, 24)
    }
    #[doc = "Bit 25 - dma abort interrupt, set if seqence dma write pointer reachs sw read pointer if stop_en is set"]
    #[inline(always)]
    #[must_use]
    pub fn seq_dmaabt(&mut self) -> SEQ_DMAABT_W<INT_STS_SPEC> {
        SEQ_DMAABT_W::new(self, 25)
    }
    #[doc = "Bit 26 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn seq_hw_cflct(&mut self) -> SEQ_HW_CFLCT_W<INT_STS_SPEC> {
        SEQ_HW_CFLCT_W::new(self, 26)
    }
    #[doc = "Bit 27 - sequence queue conflict interrup, set if HW or SW trigger received during conversion"]
    #[inline(always)]
    #[must_use]
    pub fn seq_sw_cflct(&mut self) -> SEQ_SW_CFLCT_W<INT_STS_SPEC> {
        SEQ_SW_CFLCT_W::new(self, 27)
    }
    #[doc = "Bit 28 - read conflict interrup, set if wait_dis is set, one conversion is in progress, SW read another channel"]
    #[inline(always)]
    #[must_use]
    pub fn read_cflct(&mut self) -> READ_CFLCT_W<INT_STS_SPEC> {
        READ_CFLCT_W::new(self, 28)
    }
    #[doc = "Bit 29 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn trig_hw_cflct(&mut self) -> TRIG_HW_CFLCT_W<INT_STS_SPEC> {
        TRIG_HW_CFLCT_W::new(self, 29)
    }
    #[doc = "Bit 30 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn trig_sw_cflct(&mut self) -> TRIG_SW_CFLCT_W<INT_STS_SPEC> {
        TRIG_SW_CFLCT_W::new(self, 30)
    }
    #[doc = "Bit 31 - interrupt for one trigger conversion complete if enabled"]
    #[inline(always)]
    #[must_use]
    pub fn trig_cmpt(&mut self) -> TRIG_CMPT_W<INT_STS_SPEC> {
        TRIG_CMPT_W::new(self, 31)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_STS_SPEC;
impl crate::RegisterSpec for INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_sts::R`](R) reader structure"]
impl crate::Readable for INT_STS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_sts::W`](W) writer structure"]
impl crate::Writable for INT_STS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_sts to value 0"]
impl crate::Resettable for INT_STS_SPEC {
    const RESET_VALUE: u32 = 0;
}
