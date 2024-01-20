#[doc = "Register `ENDPTPRIME` reader"]
pub type R = crate::R<ENDPTPRIME_SPEC>;
#[doc = "Register `ENDPTPRIME` writer"]
pub type W = crate::W<ENDPTPRIME_SPEC>;
#[doc = "Field `PERB` reader - PERB Prime Endpoint Receive Buffer - R/WS. For each endpoint, a corresponding bit is used to request a buffer prepare for a receive operation for when a USB host initiates a USB OUT transaction. Software should write a one to the corresponding bit whenever posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a receive buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type PERB_R = crate::FieldReader;
#[doc = "Field `PERB` writer - PERB Prime Endpoint Receive Buffer - R/WS. For each endpoint, a corresponding bit is used to request a buffer prepare for a receive operation for when a USB host initiates a USB OUT transaction. Software should write a one to the corresponding bit whenever posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a receive buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type PERB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PETB` reader - PETB Prime Endpoint Transmit Buffer - R/WS. For each endpoint a corresponding bit is used to request that a buffer is prepared for a transmit operation in order to respond to a USB IN/INTERRUPT transaction. Software should write a one to the corresponding bit when posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a transmit buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type PETB_R = crate::FieldReader;
#[doc = "Field `PETB` writer - PETB Prime Endpoint Transmit Buffer - R/WS. For each endpoint a corresponding bit is used to request that a buffer is prepared for a transmit operation in order to respond to a USB IN/INTERRUPT transaction. Software should write a one to the corresponding bit when posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a transmit buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type PETB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PERB Prime Endpoint Receive Buffer - R/WS. For each endpoint, a corresponding bit is used to request a buffer prepare for a receive operation for when a USB host initiates a USB OUT transaction. Software should write a one to the corresponding bit whenever posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a receive buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn perb(&self) -> PERB_R {
        PERB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PETB Prime Endpoint Transmit Buffer - R/WS. For each endpoint a corresponding bit is used to request that a buffer is prepared for a transmit operation in order to respond to a USB IN/INTERRUPT transaction. Software should write a one to the corresponding bit when posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a transmit buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn petb(&self) -> PETB_R {
        PETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PERB Prime Endpoint Receive Buffer - R/WS. For each endpoint, a corresponding bit is used to request a buffer prepare for a receive operation for when a USB host initiates a USB OUT transaction. Software should write a one to the corresponding bit whenever posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a receive buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn perb(&mut self) -> PERB_W<ENDPTPRIME_SPEC> {
        PERB_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - PETB Prime Endpoint Transmit Buffer - R/WS. For each endpoint a corresponding bit is used to request that a buffer is prepared for a transmit operation in order to respond to a USB IN/INTERRUPT transaction. Software should write a one to the corresponding bit when posting a new transfer descriptor to an endpoint queue head. Hardware automatically uses this bit to begin parsing for a new transfer descriptor from the queue head and prepare a transmit buffer. Hardware clears this bit when the associated endpoint(s) is (are) successfully primed. NOTE: These bits are momentarily set by hardware during hardware re-priming operations when a dTD is retired, and the dQH is updated. PETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn petb(&mut self) -> PETB_W<ENDPTPRIME_SPEC> {
        PETB_W::new(self, 16)
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
#[doc = "Endpoint Prime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptprime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptprime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTPRIME_SPEC;
impl crate::RegisterSpec for ENDPTPRIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptprime::R`](R) reader structure"]
impl crate::Readable for ENDPTPRIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptprime::W`](W) writer structure"]
impl crate::Writable for ENDPTPRIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTPRIME to value 0"]
impl crate::Resettable for ENDPTPRIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
