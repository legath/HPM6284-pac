#[doc = "Register `seq_dma_addr` reader"]
pub type R = crate::R<SEQ_DMA_ADDR_SPEC>;
#[doc = "Register `seq_dma_addr` writer"]
pub type W = crate::W<SEQ_DMA_ADDR_SPEC>;
#[doc = "Field `TAR_ADDR` reader - dma target address, should be 4-byte aligned"]
pub type TAR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TAR_ADDR` writer - dma target address, should be 4-byte aligned"]
pub type TAR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - dma target address, should be 4-byte aligned"]
    #[inline(always)]
    pub fn tar_addr(&self) -> TAR_ADDR_R {
        TAR_ADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - dma target address, should be 4-byte aligned"]
    #[inline(always)]
    #[must_use]
    pub fn tar_addr(&mut self) -> TAR_ADDR_W<SEQ_DMA_ADDR_SPEC> {
        TAR_ADDR_W::new(self, 2)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_dma_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_dma_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_DMA_ADDR_SPEC;
impl crate::RegisterSpec for SEQ_DMA_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_dma_addr::R`](R) reader structure"]
impl crate::Readable for SEQ_DMA_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_dma_addr::W`](W) writer structure"]
impl crate::Writable for SEQ_DMA_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets seq_dma_addr to value 0"]
impl crate::Resettable for SEQ_DMA_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
