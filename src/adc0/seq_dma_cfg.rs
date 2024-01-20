#[doc = "Register `seq_dma_cfg` reader"]
pub type R = crate::R<SEQ_DMA_CFG_SPEC>;
#[doc = "Register `seq_dma_cfg` writer"]
pub type W = crate::W<SEQ_DMA_CFG_SPEC>;
#[doc = "Field `BUF_LEN` reader - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
pub type BUF_LEN_R = crate::FieldReader<u16>;
#[doc = "Field `BUF_LEN` writer - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
pub type BUF_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STOP_EN` reader - set to stop dma if reach the stop_pos"]
pub type STOP_EN_R = crate::BitReader;
#[doc = "Field `STOP_EN` writer - set to stop dma if reach the stop_pos"]
pub type STOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST` reader - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
pub type DMA_RST_R = crate::BitReader;
#[doc = "Field `DMA_RST` writer - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
pub type DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_POS` reader - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
pub type STOP_POS_R = crate::FieldReader<u16>;
#[doc = "Field `STOP_POS` writer - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
pub type STOP_POS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
    #[inline(always)]
    pub fn buf_len(&self) -> BUF_LEN_R {
        BUF_LEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - set to stop dma if reach the stop_pos"]
    #[inline(always)]
    pub fn stop_en(&self) -> STOP_EN_R {
        STOP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:27 - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
    #[inline(always)]
    pub fn stop_pos(&self) -> STOP_POS_R {
        STOP_POS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - dma buffer length, after write to (tar_addr\\[31:2\\]+buf_len)*4, the next dma address will be tar_addr\\[31:2\\]*4 0 for 4byte; 0xFFF for 16kbyte."]
    #[inline(always)]
    #[must_use]
    pub fn buf_len(&mut self) -> BUF_LEN_W<SEQ_DMA_CFG_SPEC> {
        BUF_LEN_W::new(self, 0)
    }
    #[doc = "Bit 12 - set to stop dma if reach the stop_pos"]
    #[inline(always)]
    #[must_use]
    pub fn stop_en(&mut self) -> STOP_EN_W<SEQ_DMA_CFG_SPEC> {
        STOP_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - set this bit will reset HW dma write pointer to seq_dma_addr, and set HW cycle bit to 1. dma is halted if this bit is set. SW should clear all cycle bit in buffer to 0 before clear dma_rst"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst(&mut self) -> DMA_RST_W<SEQ_DMA_CFG_SPEC> {
        DMA_RST_W::new(self, 13)
    }
    #[doc = "Bits 16:27 - if stop_en is set, SW is responsible to udpate this field to the next read point, HW should not write data to this point since it's not read out by SW yet"]
    #[inline(always)]
    #[must_use]
    pub fn stop_pos(&mut self) -> STOP_POS_W<SEQ_DMA_CFG_SPEC> {
        STOP_POS_W::new(self, 16)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_dma_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_dma_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_DMA_CFG_SPEC;
impl crate::RegisterSpec for SEQ_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_dma_cfg::R`](R) reader structure"]
impl crate::Readable for SEQ_DMA_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_dma_cfg::W`](W) writer structure"]
impl crate::Writable for SEQ_DMA_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets seq_dma_cfg to value 0"]
impl crate::Resettable for SEQ_DMA_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
