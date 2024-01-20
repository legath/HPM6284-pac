#[doc = "Register `TMAX` reader"]
pub type R = crate::R<TMAX_SPEC>;
#[doc = "Register `TMAX` writer"]
pub type W = crate::W<TMAX_SPEC>;
#[doc = "Field `T` reader - maximum temperature ever found"]
pub type T_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - maximum temperature ever found"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(self.bits)
    }
}
impl W {
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
#[doc = "Maximum Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmax::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmax::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMAX_SPEC;
impl crate::RegisterSpec for TMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmax::R`](R) reader structure"]
impl crate::Readable for TMAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmax::W`](W) writer structure"]
impl crate::Writable for TMAX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMAX to value 0xff80_0000"]
impl crate::Resettable for TMAX_SPEC {
    const RESET_VALUE: u32 = 0xff80_0000;
}
