#[doc = "Register `GPTIMER0LD` reader"]
pub type R = crate::R<GPTIMER0LD_SPEC>;
#[doc = "Register `GPTIMER0LD` writer"]
pub type W = crate::W<GPTIMER0LD_SPEC>;
#[doc = "Field `GPTLD` reader - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
pub type GPTLD_R = crate::FieldReader<u32>;
#[doc = "Field `GPTLD` writer - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
pub type GPTLD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
    #[inline(always)]
    pub fn gptld(&self) -> GPTLD_R {
        GPTLD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - GPTLD General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'. This value represents the time in microseconds minus 1 for the timer duration. Example: for a one millisecond timer, load 1000-1=999 or 0x0003E7. NOTE: Max value is 0xFFFFFF or 16.777215 seconds."]
    #[inline(always)]
    #[must_use]
    pub fn gptld(&mut self) -> GPTLD_W<GPTIMER0LD_SPEC> {
        GPTLD_W::new(self, 0)
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
#[doc = "General Purpose Timer #0 Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer0ld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer0ld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPTIMER0LD_SPEC;
impl crate::RegisterSpec for GPTIMER0LD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gptimer0ld::R`](R) reader structure"]
impl crate::Readable for GPTIMER0LD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gptimer0ld::W`](W) writer structure"]
impl crate::Writable for GPTIMER0LD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPTIMER0LD to value 0"]
impl crate::Resettable for GPTIMER0LD_SPEC {
    const RESET_VALUE: u32 = 0;
}
