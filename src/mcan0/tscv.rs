#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TSCV_SPEC>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TSCV_SPEC>;
#[doc = "Field `TSC` reader - Timestamp Counter The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx).When TSCC.TSS = “01”, the Timestamp Counter is incremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = “10”, TSC reflects the external Timestamp Counter value. A write access has no impact."]
pub type TSC_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter The internal/external Timestamp Counter value is captured on start of frame (both Rx and Tx).When TSCC.TSS = “01”, the Timestamp Counter is incremented in multiples of CAN bit times \\[1…16\\]
depending on the configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS = “10”, TSC reflects the external Timestamp Counter value. A write access has no impact."]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
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
#[doc = "timestamp counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TSCV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TSCV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    const RESET_VALUE: u32 = 0;
}
