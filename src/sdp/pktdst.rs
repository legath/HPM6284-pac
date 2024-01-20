#[doc = "Register `PKTDST` reader"]
pub type R = crate::R<PKTDST_SPEC>;
#[doc = "Register `PKTDST` writer"]
pub type W = crate::W<PKTDST_SPEC>;
#[doc = "Field `PKTDST` reader - Packet Memory Destination Address"]
pub type PKTDST_R = crate::FieldReader<u32>;
#[doc = "Field `PKTDST` writer - Packet Memory Destination Address"]
pub type PKTDST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Packet Memory Destination Address"]
    #[inline(always)]
    pub fn pktdst(&self) -> PKTDST_R {
        PKTDST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Packet Memory Destination Address"]
    #[inline(always)]
    #[must_use]
    pub fn pktdst(&mut self) -> PKTDST_W<PKTDST_SPEC> {
        PKTDST_W::new(self, 0)
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
#[doc = "Packet Memory Destination Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PKTDST_SPEC;
impl crate::RegisterSpec for PKTDST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdst::R`](R) reader structure"]
impl crate::Readable for PKTDST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pktdst::W`](W) writer structure"]
impl crate::Writable for PKTDST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDST to value 0"]
impl crate::Resettable for PKTDST_SPEC {
    const RESET_VALUE: u32 = 0;
}
