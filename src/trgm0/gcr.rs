#[doc = "Register `GCR` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `TRGOPEN` reader - The bitfield enable the TRGM outputs."]
pub type TRGOPEN_R = crate::FieldReader<u16>;
#[doc = "Field `TRGOPEN` writer - The bitfield enable the TRGM outputs."]
pub type TRGOPEN_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The bitfield enable the TRGM outputs."]
    #[inline(always)]
    pub fn trgopen(&self) -> TRGOPEN_R {
        TRGOPEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The bitfield enable the TRGM outputs."]
    #[inline(always)]
    #[must_use]
    pub fn trgopen(&mut self) -> TRGOPEN_W<GCR_SPEC> {
        TRGOPEN_W::new(self, 0)
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
#[doc = "General Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
