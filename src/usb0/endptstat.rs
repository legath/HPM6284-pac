#[doc = "Register `ENDPTSTAT` reader"]
pub type R = crate::R<ENDPTSTAT_SPEC>;
#[doc = "Register `ENDPTSTAT` writer"]
pub type W = crate::W<ENDPTSTAT_SPEC>;
#[doc = "Field `ERBR` reader - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ERBR_R = crate::FieldReader;
#[doc = "Field `ETBR` reader - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type ETBR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - ERBR Endpoint Receive Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to a one by the hardware as a response to receiving a command from a corresponding bit in the ENDPRIME register. There is always a delay between setting a bit in the ENDPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ERBR\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn erbr(&self) -> ERBR_R {
        ERBR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ETBR Endpoint Transmit Buffer Ready -- Read Only. One bit for each endpoint indicates status of the respective endpoint buffer. This bit is set to one by the hardware as a response to receiving a command from a corresponding bit in the ENDPTPRIME register. There is always a delay between setting a bit in the ENDPTPRIME register and endpoint indicating ready. This delay time varies based upon the current USB traffic and the number of bits set in the ENDPRIME register. Buffer ready is cleared by USB reset, by the USB DMA system, or through the ENDPTFLUSH register. NOTE: These bits are momentarily cleared by hardware during hardware endpoint re-priming operations when a dTD is retired, and the dQH is updated. ETBR\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn etbr(&self) -> ETBR_R {
        ETBR_R::new(((self.bits >> 16) & 0xff) as u8)
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
#[doc = "Endpoint Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTSTAT_SPEC;
impl crate::RegisterSpec for ENDPTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptstat::R`](R) reader structure"]
impl crate::Readable for ENDPTSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptstat::W`](W) writer structure"]
impl crate::Writable for ENDPTSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTSTAT to value 0"]
impl crate::Resettable for ENDPTSTAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
