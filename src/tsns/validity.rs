#[doc = "Register `VALIDITY` reader"]
pub type R = crate::R<VALIDITY_SPEC>;
#[doc = "Register `VALIDITY` writer"]
pub type W = crate::W<VALIDITY_SPEC>;
#[doc = "Field `VALIDITY` reader - time for temperature values to expire in 24M clock cycles"]
pub type VALIDITY_R = crate::FieldReader<u32>;
#[doc = "Field `VALIDITY` writer - time for temperature values to expire in 24M clock cycles"]
pub type VALIDITY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - time for temperature values to expire in 24M clock cycles"]
    #[inline(always)]
    pub fn validity(&self) -> VALIDITY_R {
        VALIDITY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - time for temperature values to expire in 24M clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn validity(&mut self) -> VALIDITY_W<VALIDITY_SPEC> {
        VALIDITY_W::new(self, 0)
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
#[doc = "Sample validity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`validity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`validity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALIDITY_SPEC;
impl crate::RegisterSpec for VALIDITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`validity::R`](R) reader structure"]
impl crate::Readable for VALIDITY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`validity::W`](W) writer structure"]
impl crate::Writable for VALIDITY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VALIDITY to value 0x016e_3600"]
impl crate::Resettable for VALIDITY_SPEC {
    const RESET_VALUE: u32 = 0x016e_3600;
}
