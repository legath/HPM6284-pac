#[doc = "Register `v` reader"]
pub type R = crate::R<V_SPEC>;
#[doc = "Register `v` writer"]
pub type W = crate::W<V_SPEC>;
#[doc = "Field `VCNT` reader - vcnt counter"]
pub type VCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - vcnt counter"]
    #[inline(always)]
    pub fn vcnt(&self) -> VCNT_R {
        VCNT_R::new(self.bits & 0x0fff_ffff)
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
#[doc = "V counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`v::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`v::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct V_SPEC;
impl crate::RegisterSpec for V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`v::R`](R) reader structure"]
impl crate::Readable for V_SPEC {}
#[doc = "`write(|w| ..)` method takes [`v::W`](W) writer structure"]
impl crate::Writable for V_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets v to value 0"]
impl crate::Resettable for V_SPEC {
    const RESET_VALUE: u32 = 0;
}
