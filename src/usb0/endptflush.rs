#[doc = "Register `ENDPTFLUSH` reader"]
pub type R = crate::R<ENDPTFLUSH_SPEC>;
#[doc = "Register `ENDPTFLUSH` writer"]
pub type W = crate::W<ENDPTFLUSH_SPEC>;
#[doc = "Field `FERB` reader - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FERB_R = crate::FieldReader;
#[doc = "Field `FERB` writer - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FERB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FETB` reader - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FETB_R = crate::FieldReader;
#[doc = "Field `FETB` writer - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
pub type FETB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn ferb(&self) -> FERB_R {
        FERB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    pub fn fetb(&self) -> FETB_R {
        FETB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - FERB Flush Endpoint Receive Buffer - R/WS. Writing one to a bit(s) causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FERB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn ferb(&mut self) -> FERB_W<ENDPTFLUSH_SPEC> {
        FERB_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - FETB Flush Endpoint Transmit Buffer - R/WS. Writing one to a bit(s) in this register causes the associated endpoint(s) to clear any primed buffers. If a packet is in progress for one of the associated endpoints, then that transfer continues until completion. Hardware clears this register after the endpoint flush operation is successful. FETB\\[N\\]
- Endpoint #N, N is in 0..7"]
    #[inline(always)]
    #[must_use]
    pub fn fetb(&mut self) -> FETB_W<ENDPTFLUSH_SPEC> {
        FETB_W::new(self, 16)
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
#[doc = "Endpoint Flush Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptflush::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptflush::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDPTFLUSH_SPEC;
impl crate::RegisterSpec for ENDPTFLUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endptflush::R`](R) reader structure"]
impl crate::Readable for ENDPTFLUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endptflush::W`](W) writer structure"]
impl crate::Writable for ENDPTFLUSH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDPTFLUSH to value 0"]
impl crate::Resettable for ENDPTFLUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
