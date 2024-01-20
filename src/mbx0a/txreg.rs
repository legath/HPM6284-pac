#[doc = "Register `TXREG` reader"]
pub type R = crate::R<TXREG_SPEC>;
#[doc = "Register `TXREG` writer"]
pub type W = crate::W<TXREG_SPEC>;
#[doc = "Field `TXREG` writer - Transmit word message to other core."]
pub type TXREG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Transmit word message to other core."]
    #[inline(always)]
    #[must_use]
    pub fn txreg(&mut self) -> TXREG_W<TXREG_SPEC> {
        TXREG_W::new(self, 0)
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
#[doc = "Transmit word message to other core.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXREG_SPEC;
impl crate::RegisterSpec for TXREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txreg::R`](R) reader structure"]
impl crate::Readable for TXREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txreg::W`](W) writer structure"]
impl crate::Writable for TXREG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXREG to value 0"]
impl crate::Resettable for TXREG_SPEC {
    const RESET_VALUE: u32 = 0;
}
