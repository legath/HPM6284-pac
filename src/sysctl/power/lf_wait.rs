#[doc = "Register `lf_wait` reader"]
pub type R = crate::R<LF_WAIT_SPEC>;
#[doc = "Register `lf_wait` writer"]
pub type W = crate::W<LF_WAIT_SPEC>;
#[doc = "Field `WAIT` reader - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
pub type WAIT_R = crate::FieldReader<u32>;
#[doc = "Field `WAIT` writer - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
pub type WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - wait time for low fan out power switch turn on, default value is 255 0: 0 clock cycle 1: 1 clock cycles . . . clock cycles count on 24MHz"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<LF_WAIT_SPEC> {
        WAIT_W::new(self, 0)
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
#[doc = "Power Setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lf_wait::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lf_wait::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LF_WAIT_SPEC;
impl crate::RegisterSpec for LF_WAIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lf_wait::R`](R) reader structure"]
impl crate::Readable for LF_WAIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lf_wait::W`](W) writer structure"]
impl crate::Writable for LF_WAIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lf_wait to value 0x0255"]
impl crate::Resettable for LF_WAIT_SPEC {
    const RESET_VALUE: u32 = 0x0255;
}
