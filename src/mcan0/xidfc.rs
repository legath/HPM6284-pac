#[doc = "Register `XIDFC` reader"]
pub type R = crate::R<XIDFC_SPEC>;
#[doc = "Register `XIDFC` writer"]
pub type W = crate::W<XIDFC_SPEC>;
#[doc = "Field `FLESA` reader - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
pub type FLESA_R = crate::FieldReader<u16>;
#[doc = "Field `FLESA` writer - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
pub type FLESA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSE` reader - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64"]
pub type LSE_R = crate::FieldReader;
#[doc = "Field `LSE` writer - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64"]
pub type LSE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
    #[inline(always)]
    pub fn flesa(&self) -> FLESA_R {
        FLESA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    pub fn lse(&self) -> LSE_R {
        LSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Extended Start Address Start address of extended Message ID filter list (32-bit word address)."]
    #[inline(always)]
    #[must_use]
    pub fn flesa(&mut self) -> FLESA_W<XIDFC_SPEC> {
        FLESA_W::new(self, 2)
    }
    #[doc = "Bits 16:22 - List Size Extended 0= No extended Message ID filter 1-64= Number of extended Message ID filter elements >64= Values greater than 64 are interpreted as 64"]
    #[inline(always)]
    #[must_use]
    pub fn lse(&mut self) -> LSE_W<XIDFC_SPEC> {
        LSE_W::new(self, 16)
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
#[doc = "extended ID filter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xidfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xidfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XIDFC_SPEC;
impl crate::RegisterSpec for XIDFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xidfc::R`](R) reader structure"]
impl crate::Readable for XIDFC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xidfc::W`](W) writer structure"]
impl crate::Writable for XIDFC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets XIDFC to value 0"]
impl crate::Resettable for XIDFC_SPEC {
    const RESET_VALUE: u32 = 0;
}
