#[doc = "Register `TSS2` reader"]
pub type R = crate::R<TSS2_SPEC>;
#[doc = "Register `TSS2` writer"]
pub type W = crate::W<TSS2_SPEC>;
#[doc = "Field `TSP` reader - Timestamp Pointer The Timestamp Pointer is incremented by one each time a timestamp is captured. From its maximum value (3, 7, or 15 depending on number_ts_g), it is incremented to 0. Value also signalled on output m_can_tsp\\[3:0\\]."]
pub type TSP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Timestamp Pointer The Timestamp Pointer is incremented by one each time a timestamp is captured. From its maximum value (3, 7, or 15 depending on number_ts_g), it is incremented to 0. Value also signalled on output m_can_tsp\\[3:0\\]."]
    #[inline(always)]
    pub fn tsp(&self) -> TSP_R {
        TSP_R::new((self.bits & 0x0f) as u8)
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
#[doc = "timestamp status2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tss2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tss2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSS2_SPEC;
impl crate::RegisterSpec for TSS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tss2::R`](R) reader structure"]
impl crate::Readable for TSS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tss2::W`](W) writer structure"]
impl crate::Writable for TSS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSS2 to value 0"]
impl crate::Resettable for TSS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
