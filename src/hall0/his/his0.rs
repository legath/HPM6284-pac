#[doc = "Register `his0` reader"]
pub type R = crate::R<HIS0_SPEC>;
#[doc = "Register `his0` writer"]
pub type W = crate::W<HIS0_SPEC>;
#[doc = "Field `UHIS0` reader - copy of ucnt when u signal transition from 0 to 1"]
pub type UHIS0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - copy of ucnt when u signal transition from 0 to 1"]
    #[inline(always)]
    pub fn uhis0(&self) -> UHIS0_R {
        UHIS0_R::new(self.bits)
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
#[doc = "history register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`his0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`his0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIS0_SPEC;
impl crate::RegisterSpec for HIS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`his0::R`](R) reader structure"]
impl crate::Readable for HIS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`his0::W`](W) writer structure"]
impl crate::Writable for HIS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets his0 to value 0"]
impl crate::Resettable for HIS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
