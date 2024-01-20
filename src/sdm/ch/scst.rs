#[doc = "Register `SCST` reader"]
pub type R = crate::R<SCST_SPEC>;
#[doc = "Register `SCST` writer"]
pub type W = crate::W<SCST_SPEC>;
#[doc = "Field `CMPL` writer - LLT out of range. Error flag."]
pub type CMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPH` writer - HLT out of range. Error flag."]
pub type CMPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MF` writer - power modulator Failure found. MCLK not found. Error flag."]
pub type MF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HZ` writer - Amplitude rising above HZ event found."]
pub type HZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LLT out of range. Error flag."]
    #[inline(always)]
    #[must_use]
    pub fn cmpl(&mut self) -> CMPL_W<SCST_SPEC> {
        CMPL_W::new(self, 0)
    }
    #[doc = "Bit 1 - HLT out of range. Error flag."]
    #[inline(always)]
    #[must_use]
    pub fn cmph(&mut self) -> CMPH_W<SCST_SPEC> {
        CMPH_W::new(self, 1)
    }
    #[doc = "Bit 2 - power modulator Failure found. MCLK not found. Error flag."]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MF_W<SCST_SPEC> {
        MF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Amplitude rising above HZ event found."]
    #[inline(always)]
    #[must_use]
    pub fn hz(&mut self) -> HZ_W<SCST_SPEC> {
        HZ_W::new(self, 3)
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
#[doc = "Amplitude Path Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCST_SPEC;
impl crate::RegisterSpec for SCST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scst::R`](R) reader structure"]
impl crate::Readable for SCST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scst::W`](W) writer structure"]
impl crate::Writable for SCST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCST to value 0"]
impl crate::Resettable for SCST_SPEC {
    const RESET_VALUE: u32 = 0;
}
