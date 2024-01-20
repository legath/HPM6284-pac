#[doc = "Register `TMIN` reader"]
pub type R = crate::R<TMIN_SPEC>;
#[doc = "Register `TMIN` writer"]
pub type W = crate::W<TMIN_SPEC>;
#[doc = "Field `T` reader - minimum temperature ever found"]
pub type T_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - minimum temperature ever found"]
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
#[doc = "Minimum Temperature\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMIN_SPEC;
impl crate::RegisterSpec for TMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmin::R`](R) reader structure"]
impl crate::Readable for TMIN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmin::W`](W) writer structure"]
impl crate::Writable for TMIN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMIN to value 0x007f_ffff"]
impl crate::Resettable for TMIN_SPEC {
    const RESET_VALUE: u32 = 0x007f_ffff;
}
