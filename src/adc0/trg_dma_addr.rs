#[doc = "Register `trg_dma_addr` reader"]
pub type R = crate::R<TRG_DMA_ADDR_SPEC>;
#[doc = "Register `trg_dma_addr` writer"]
pub type W = crate::W<TRG_DMA_ADDR_SPEC>;
#[doc = "Field `TRG_DMA_ADDR` reader - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
pub type TRG_DMA_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TRG_DMA_ADDR` writer - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
pub type TRG_DMA_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
    #[inline(always)]
    pub fn trg_dma_addr(&self) -> TRG_DMA_ADDR_R {
        TRG_DMA_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - buffer start address for trigger queue, 192byte total, 16 bytes for each trigger (4 bytes for each conversion)"]
    #[inline(always)]
    #[must_use]
    pub fn trg_dma_addr(&mut self) -> TRG_DMA_ADDR_W<TRG_DMA_ADDR_SPEC> {
        TRG_DMA_ADDR_W::new(self, 2)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg_dma_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg_dma_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRG_DMA_ADDR_SPEC;
impl crate::RegisterSpec for TRG_DMA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trg_dma_addr::R`](R) reader structure"]
impl crate::Readable for TRG_DMA_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trg_dma_addr::W`](W) writer structure"]
impl crate::Writable for TRG_DMA_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets trg_dma_addr to value 0"]
impl crate::Resettable for TRG_DMA_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
