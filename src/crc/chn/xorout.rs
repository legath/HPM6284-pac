#[doc = "Register `xorout` reader"]
pub type R = crate::R<XOROUT_SPEC>;
#[doc = "Register `xorout` writer"]
pub type W = crate::W<XOROUT_SPEC>;
#[doc = "Field `XOROUT` reader - XOR for CRC result"]
pub type XOROUT_R = crate::FieldReader<u32>;
#[doc = "Field `XOROUT` writer - XOR for CRC result"]
pub type XOROUT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XOR for CRC result"]
    #[inline(always)]
    pub fn xorout(&self) -> XOROUT_R {
        XOROUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR for CRC result"]
    #[inline(always)]
    #[must_use]
    pub fn xorout(&mut self) -> XOROUT_W<XOROUT_SPEC> {
        XOROUT_W::new(self, 0)
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
#[doc = "chn&amp;index0 xorout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xorout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xorout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XOROUT_SPEC;
impl crate::RegisterSpec for XOROUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xorout::R`](R) reader structure"]
impl crate::Readable for XOROUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xorout::W`](W) writer structure"]
impl crate::Writable for XOROUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets xorout to value 0"]
impl crate::Resettable for XOROUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
