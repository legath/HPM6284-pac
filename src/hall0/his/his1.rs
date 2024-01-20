#[doc = "Register `his1` reader"]
pub type R = crate::R<HIS1_SPEC>;
#[doc = "Register `his1` writer"]
pub type W = crate::W<HIS1_SPEC>;
#[doc = "Field `UHIS1` reader - copy of ucnt when u signal transition from 1 to 0"]
pub type UHIS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - copy of ucnt when u signal transition from 1 to 0"]
    #[inline(always)]
    pub fn uhis1(&self) -> UHIS1_R {
        UHIS1_R::new(self.bits)
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
#[doc = "history register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`his1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`his1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIS1_SPEC;
impl crate::RegisterSpec for HIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`his1::R`](R) reader structure"]
impl crate::Readable for HIS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`his1::W`](W) writer structure"]
impl crate::Writable for HIS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets his1 to value 0"]
impl crate::Resettable for HIS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
