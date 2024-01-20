#[doc = "Register `St` reader"]
pub type R = crate::R<ST_SPEC>;
#[doc = "Register `St` writer"]
pub type W = crate::W<ST_SPEC>;
#[doc = "Field `INTEXPIRED` writer - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
pub type INTEXPIRED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - The status of the watchdog interrupt timer 0: timer is not expired yet 1: timer is expired"]
    #[inline(always)]
    #[must_use]
    pub fn intexpired(&mut self) -> INTEXPIRED_W<ST_SPEC> {
        INTEXPIRED_W::new(self, 0)
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
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ST_SPEC;
impl crate::RegisterSpec for ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`st::R`](R) reader structure"]
impl crate::Readable for ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`st::W`](W) writer structure"]
impl crate::Writable for ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets St to value 0"]
impl crate::Resettable for ST_SPEC {
    const RESET_VALUE: u32 = 0;
}