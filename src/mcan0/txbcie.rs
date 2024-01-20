#[doc = "Register `TXBCIE` reader"]
pub type R = crate::R<TXBCIE_SPEC>;
#[doc = "Register `TXBCIE` writer"]
pub type W = crate::W<TXBCIE_SPEC>;
#[doc = "Field `CFIE` reader - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
pub type CFIE_R = crate::FieldReader<u32>;
#[doc = "Field `CFIE` writer - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
pub type CFIE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    pub fn cfie(&self) -> CFIE_R {
        CFIE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cancellation Finished Interrupt Enable Each Tx Buffer has its own Cancellation Finished Interrupt Enable bit. 0= Cancellation finished interrupt disabled 1= Cancellation finished interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cfie(&mut self) -> CFIE_W<TXBCIE_SPEC> {
        CFIE_W::new(self, 0)
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
#[doc = "tx buffer cancellation finished interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txbcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbcie::R`](R) reader structure"]
impl crate::Readable for TXBCIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txbcie::W`](W) writer structure"]
impl crate::Writable for TXBCIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TXBCIE_SPEC {
    const RESET_VALUE: u32 = 0;
}
