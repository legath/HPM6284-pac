#[doc = "Register `CAPPRD` reader"]
pub type R = crate::R<CAPPRD_SPEC>;
#[doc = "Register `CAPPRD` writer"]
pub type W = crate::W<CAPPRD_SPEC>;
#[doc = "Field `CAPPRD` reader - This register contains the input signal period when channel is configured to input capture measure mode."]
pub type CAPPRD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register contains the input signal period when channel is configured to input capture measure mode."]
    #[inline(always)]
    pub fn capprd(&self) -> CAPPRD_R {
        CAPPRD_R::new(self.bits)
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
#[doc = "PWM period measure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`capprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`capprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAPPRD_SPEC;
impl crate::RegisterSpec for CAPPRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capprd::R`](R) reader structure"]
impl crate::Readable for CAPPRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`capprd::W`](W) writer structure"]
impl crate::Writable for CAPPRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPPRD to value 0"]
impl crate::Resettable for CAPPRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
