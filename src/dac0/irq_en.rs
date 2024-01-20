#[doc = "Register `irq_en` reader"]
pub type R = crate::R<IRQ_EN_SPEC>;
#[doc = "Register `irq_en` writer"]
pub type W = crate::W<IRQ_EN_SPEC>;
#[doc = "Field `BUF0_CMPT` reader - No description avaiable"]
pub type BUF0_CMPT_R = crate::BitReader;
#[doc = "Field `BUF0_CMPT` writer - No description avaiable"]
pub type BUF0_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF1_CMPT` reader - No description avaiable"]
pub type BUF1_CMPT_R = crate::BitReader;
#[doc = "Field `BUF1_CMPT` writer - No description avaiable"]
pub type BUF1_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY` reader - No description avaiable"]
pub type FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` writer - No description avaiable"]
pub type FIFO_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_ERROR` reader - No description avaiable"]
pub type AHB_ERROR_R = crate::BitReader;
#[doc = "Field `AHB_ERROR` writer - No description avaiable"]
pub type AHB_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_CMPT` reader - No description avaiable"]
pub type STEP_CMPT_R = crate::BitReader;
#[doc = "Field `STEP_CMPT` writer - No description avaiable"]
pub type STEP_CMPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn buf0_cmpt(&self) -> BUF0_CMPT_R {
        BUF0_CMPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn buf1_cmpt(&self) -> BUF1_CMPT_R {
        BUF1_CMPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    pub fn ahb_error(&self) -> AHB_ERROR_R {
        AHB_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    pub fn step_cmpt(&self) -> STEP_CMPT_R {
        STEP_CMPT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn buf0_cmpt(&mut self) -> BUF0_CMPT_W<IRQ_EN_SPEC> {
        BUF0_CMPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn buf1_cmpt(&mut self) -> BUF1_CMPT_W<IRQ_EN_SPEC> {
        BUF1_CMPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_empty(&mut self) -> FIFO_EMPTY_W<IRQ_EN_SPEC> {
        FIFO_EMPTY_W::new(self, 2)
    }
    #[doc = "Bit 3 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_error(&mut self) -> AHB_ERROR_W<IRQ_EN_SPEC> {
        AHB_ERROR_W::new(self, 3)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn step_cmpt(&mut self) -> STEP_CMPT_W<IRQ_EN_SPEC> {
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ_EN_SPEC;
impl crate::RegisterSpec for IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_en::R`](R) reader structure"]
impl crate::Readable for IRQ_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irq_en::W`](W) writer structure"]
impl crate::Writable for IRQ_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets irq_en to value 0"]
impl crate::Resettable for IRQ_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
