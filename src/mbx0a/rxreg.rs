#[doc = "Register `RXREG` reader"]
pub type R = crate::R<RXREG_SPEC>;
#[doc = "Register `RXREG` writer"]
pub type W = crate::W<RXREG_SPEC>;
#[doc = "Field `RXREG` reader - Receive word message from other core."]
pub type RXREG_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive word message from other core."]
    #[inline(always)]
    pub fn rxreg(&self) -> RXREG_R {
        RXREG_R::new(self.bits)
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
#[doc = "Receive word message from other core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXREG_SPEC;
impl crate::RegisterSpec for RXREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxreg::R`](R) reader structure"]
impl crate::Readable for RXREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxreg::W`](W) writer structure"]
impl crate::Writable for RXREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXREG to value 0"]
impl crate::Resettable for RXREG_SPEC {
    const RESET_VALUE: u32 = 0;
}
