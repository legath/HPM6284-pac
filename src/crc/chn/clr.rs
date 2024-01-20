#[doc = "Register `clr` reader"]
pub type R = crate::R<CLR_SPEC>;
#[doc = "Register `clr` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `CLR` reader - write 1 to clr crc setting and result for its channel. always read 0."]
pub type CLR_R = crate::BitReader;
#[doc = "Field `CLR` writer - write 1 to clr crc setting and result for its channel. always read 0."]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to clr crc setting and result for its channel. always read 0."]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to clr crc setting and result for its channel. always read 0."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<CLR_SPEC> {
        CLR_W::new(self, 0)
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
#[doc = "chn&amp;index0 clear crc result and setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
