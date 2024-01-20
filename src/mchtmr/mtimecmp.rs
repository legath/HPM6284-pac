#[doc = "Register `MTIMECMP` reader"]
pub type R = crate::R<MTIMECMP_SPEC>;
#[doc = "Register `MTIMECMP` writer"]
pub type W = crate::W<MTIMECMP_SPEC>;
#[doc = "Field `MTIMECMP` reader - Machine time compare"]
pub type MTIMECMP_R = crate::FieldReader<u64>;
#[doc = "Field `MTIMECMP` writer - Machine time compare"]
pub type MTIMECMP_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - Machine time compare"]
    #[inline(always)]
    pub fn mtimecmp(&self) -> MTIMECMP_R {
        MTIMECMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - Machine time compare"]
    #[inline(always)]
    #[must_use]
    pub fn mtimecmp(&mut self) -> MTIMECMP_W<MTIMECMP_SPEC> {
        MTIMECMP_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Machine Time Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIMECMP_SPEC;
impl crate::RegisterSpec for MTIMECMP_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtimecmp::R`](R) reader structure"]
impl crate::Readable for MTIMECMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtimecmp::W`](W) writer structure"]
impl crate::Writable for MTIMECMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets MTIMECMP to value 0x0002_0210"]
impl crate::Resettable for MTIMECMP_SPEC {
    const RESET_VALUE: u64 = 0x0002_0210;
}
