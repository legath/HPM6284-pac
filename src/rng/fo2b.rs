#[doc = "Register `FO2B` reader"]
pub type R = crate::R<FO2B_SPEC>;
#[doc = "Register `FO2B` writer"]
pub type W = crate::W<FO2B_SPEC>;
#[doc = "Field `FO2B` reader - SW read the FIFO output."]
pub type FO2B_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SW read the FIFO output."]
    #[inline(always)]
    pub fn fo2b(&self) -> FO2B_R {
        FO2B_R::new(self.bits)
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
#[doc = "FIFO out to bus/cpu\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fo2b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fo2b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FO2B_SPEC;
impl crate::RegisterSpec for FO2B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fo2b::R`](R) reader structure"]
impl crate::Readable for FO2B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fo2b::W`](W) writer structure"]
impl crate::Writable for FO2B_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FO2B to value 0"]
impl crate::Resettable for FO2B_SPEC {
    const RESET_VALUE: u32 = 0;
}
