#[doc = "Register `PKTBUF` reader"]
pub type R = crate::R<PKTBUF_SPEC>;
#[doc = "Register `PKTBUF` writer"]
pub type W = crate::W<PKTBUF_SPEC>;
#[doc = "Field `PKTBUF` reader - No description avaiable"]
pub type PKTBUF_R = crate::FieldReader<u32>;
#[doc = "Field `PKTBUF` writer - No description avaiable"]
pub type PKTBUF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    pub fn pktbuf(&self) -> PKTBUF_R {
        PKTBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn pktbuf(&mut self) -> PKTBUF_W<PKTBUF_SPEC> {
        PKTBUF_W::new(self, 0)
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
#[doc = "Packet buffer size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktbuf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktbuf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKTBUF_SPEC;
impl crate::RegisterSpec for PKTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktbuf::R`](R) reader structure"]
impl crate::Readable for PKTBUF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pktbuf::W`](W) writer structure"]
impl crate::Writable for PKTBUF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTBUF to value 0"]
impl crate::Resettable for PKTBUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
