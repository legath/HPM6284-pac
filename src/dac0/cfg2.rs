#[doc = "Register `cfg2` reader"]
pub type R = crate::R<CFG2_SPEC>;
#[doc = "Register `cfg2` writer"]
pub type W = crate::W<CFG2_SPEC>;
#[doc = "Field `STEP_SW_TRIG0` reader - software trigger0 for step mode, W1C in single mode. RW in continual mode"]
pub type STEP_SW_TRIG0_R = crate::BitReader;
#[doc = "Field `STEP_SW_TRIG0` writer - software trigger0 for step mode, W1C in single mode. RW in continual mode"]
pub type STEP_SW_TRIG0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_SW_TRIG1` reader - No description avaiable"]
pub type STEP_SW_TRIG1_R = crate::BitReader;
#[doc = "Field `STEP_SW_TRIG1` writer - No description avaiable"]
pub type STEP_SW_TRIG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_SW_TRIG2` reader - No description avaiable"]
pub type STEP_SW_TRIG2_R = crate::BitReader;
#[doc = "Field `STEP_SW_TRIG2` writer - No description avaiable"]
pub type STEP_SW_TRIG2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_SW_TRIG3` reader - No description avaiable"]
pub type STEP_SW_TRIG3_R = crate::BitReader;
#[doc = "Field `STEP_SW_TRIG3` writer - No description avaiable"]
pub type STEP_SW_TRIG3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_SW_TRIG` reader - software trigger for buffer mode, W1C in single mode. RW in continual mode"]
pub type BUF_SW_TRIG_R = crate::BitReader;
#[doc = "Field `BUF_SW_TRIG` writer - software trigger for buffer mode, W1C in single mode. RW in continual mode"]
pub type BUF_SW_TRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_CLR` writer - set to clear FIFO content(set both read/write pointer to 0)"]
pub type FIFO_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST0` writer - set to reset dma read pointer to buf0_start_addr"]
pub type DMA_RST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST1` writer - set to reset dma read pointer to buf1_start_addr; if set both dma_rst0&amp;dma_rst1, will set to buf0_start_addr user can set fifo_clr bit when use dma_rst*"]
pub type DMA_RST1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - software trigger0 for step mode, W1C in single mode. RW in continual mode"]
    #[inline(always)]
    pub fn step_sw_trig0(&self) -> STEP_SW_TRIG0_R {
        STEP_SW_TRIG0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn step_sw_trig1(&self) -> STEP_SW_TRIG1_R {
        STEP_SW_TRIG1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn step_sw_trig2(&self) -> STEP_SW_TRIG2_R {
        STEP_SW_TRIG2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn step_sw_trig3(&self) -> STEP_SW_TRIG3_R {
        STEP_SW_TRIG3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - software trigger for buffer mode, W1C in single mode. RW in continual mode"]
    #[inline(always)]
    pub fn buf_sw_trig(&self) -> BUF_SW_TRIG_R {
        BUF_SW_TRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - software trigger0 for step mode, W1C in single mode. RW in continual mode"]
    #[inline(always)]
    #[must_use]
    pub fn step_sw_trig0(&mut self) -> STEP_SW_TRIG0_W<CFG2_SPEC> {
        STEP_SW_TRIG0_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn step_sw_trig1(&mut self) -> STEP_SW_TRIG1_W<CFG2_SPEC> {
        STEP_SW_TRIG1_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn step_sw_trig2(&mut self) -> STEP_SW_TRIG2_W<CFG2_SPEC> {
        STEP_SW_TRIG2_W::new(self, 2)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn step_sw_trig3(&mut self) -> STEP_SW_TRIG3_W<CFG2_SPEC> {
        STEP_SW_TRIG3_W::new(self, 3)
    }
    #[doc = "Bit 4 - software trigger for buffer mode, W1C in single mode. RW in continual mode"]
    #[inline(always)]
    #[must_use]
    pub fn buf_sw_trig(&mut self) -> BUF_SW_TRIG_W<CFG2_SPEC> {
        BUF_SW_TRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - set to clear FIFO content(set both read/write pointer to 0)"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_clr(&mut self) -> FIFO_CLR_W<CFG2_SPEC> {
        FIFO_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - set to reset dma read pointer to buf0_start_addr"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst0(&mut self) -> DMA_RST0_W<CFG2_SPEC> {
        DMA_RST0_W::new(self, 6)
    }
    #[doc = "Bit 7 - set to reset dma read pointer to buf1_start_addr; if set both dma_rst0&amp;dma_rst1, will set to buf0_start_addr user can set fifo_clr bit when use dma_rst*"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst1(&mut self) -> DMA_RST1_W<CFG2_SPEC> {
        DMA_RST1_W::new(self, 7)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: u32 = 0;
}
