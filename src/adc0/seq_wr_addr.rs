#[doc = "Register `seq_wr_addr` reader"]
pub type R = crate::R<SEQ_WR_ADDR_SPEC>;
#[doc = "Register `seq_wr_addr` writer"]
pub type W = crate::W<SEQ_WR_ADDR_SPEC>;
#[doc = "Field `SEQ_WR_POINTER` reader - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4"]
pub type SEQ_WR_POINTER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - HW update this field after each dma write, it indicate the next dma write pointer. dma write address is (tar_addr+seq_wr_pointer)*4"]
    #[inline(always)]
    pub fn seq_wr_pointer(&self) -> SEQ_WR_POINTER_R {
        SEQ_WR_POINTER_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_wr_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_wr_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_WR_ADDR_SPEC;
impl crate::RegisterSpec for SEQ_WR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_wr_addr::R`](R) reader structure"]
impl crate::Readable for SEQ_WR_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_wr_addr::W`](W) writer structure"]
impl crate::Writable for SEQ_WR_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets seq_wr_addr to value 0"]
impl crate::Resettable for SEQ_WR_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
