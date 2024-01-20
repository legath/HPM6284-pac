#[doc = "Register `T` reader"]
pub type R = crate::R<T_SPEC>;
#[doc = "Register `T` writer"]
pub type W = crate::W<T_SPEC>;
#[doc = "Field `T` reader - Signed number of temperature in 256 x celsius degree"]
pub type T_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Signed number of temperature in 256 x celsius degree"]
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
#[doc = "Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T_SPEC;
impl crate::RegisterSpec for T_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t::R`](R) reader structure"]
impl crate::Readable for T_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t::W`](W) writer structure"]
impl crate::Writable for T_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T to value 0"]
impl crate::Resettable for T_SPEC {
    const RESET_VALUE: u32 = 0;
}
