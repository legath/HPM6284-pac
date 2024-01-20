#[doc = "Register `ALARM0_INC` reader"]
pub type R = crate::R<ALARM0_INC_SPEC>;
#[doc = "Register `ALARM0_INC` writer"]
pub type W = crate::W<ALARM0_INC_SPEC>;
#[doc = "Field `INCREASE` reader - adder when ARLAM0 happen, helps to create periodical alarm"]
pub type INCREASE_R = crate::FieldReader<u32>;
#[doc = "Field `INCREASE` writer - adder when ARLAM0 happen, helps to create periodical alarm"]
pub type INCREASE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - adder when ARLAM0 happen, helps to create periodical alarm"]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - adder when ARLAM0 happen, helps to create periodical alarm"]
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<ALARM0_INC_SPEC> {
        INCREASE_W::new(self, 0)
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
#[doc = "Alarm0 incremental\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_inc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_inc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALARM0_INC_SPEC;
impl crate::RegisterSpec for ALARM0_INC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alarm0_inc::R`](R) reader structure"]
impl crate::Readable for ALARM0_INC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alarm0_inc::W`](W) writer structure"]
impl crate::Writable for ALARM0_INC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALARM0_INC to value 0"]
impl crate::Resettable for ALARM0_INC_SPEC {
    const RESET_VALUE: u32 = 0;
}
