#[doc = "Register `MTIME` reader"]
pub type R = crate::R<MTIME_SPEC>;
#[doc = "Register `MTIME` writer"]
pub type W = crate::W<MTIME_SPEC>;
#[doc = "Field `MTIME` reader - Machine time"]
pub type MTIME_R = crate::FieldReader<u64>;
#[doc = "Field `MTIME` writer - Machine time"]
pub type MTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - Machine time"]
    #[inline(always)]
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - Machine time"]
    #[inline(always)]
    #[must_use]
    pub fn mtime(&mut self) -> MTIME_W<MTIME_SPEC> {
        MTIME_W::new(self, 0)
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
#[doc = "Machine Time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtime::W`](W) writer structure"]
impl crate::Writable for MTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets MTIME to value 0x0002_0210"]
impl crate::Resettable for MTIME_SPEC {
    const RESET_VALUE: u64 = 0x0002_0210;
}
