#[doc = "Register `HASWRD[%s]` reader"]
pub type R = crate::R<HASWRD_SPEC>;
#[doc = "Register `HASWRD[%s]` writer"]
pub type W = crate::W<HASWRD_SPEC>;
#[doc = "Field `HASWRD` reader - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
pub type HASWRD_R = crate::FieldReader<u32>;
#[doc = "Field `HASWRD` writer - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
pub type HASWRD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    #[inline(always)]
    pub fn haswrd(&self) -> HASWRD_R {
        HASWRD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Data Word x - HASH result bit; will store the expected hash result bit if hash check enabled; when hash check is not enabled, the hash engine will store the final hash result\\[31:0\\]
here. If CRC mode enabled, this work store the CRC expected result if the check enabled, or store the final calcuated CRC result."]
    #[inline(always)]
    #[must_use]
    pub fn haswrd(&mut self) -> HASWRD_W<HASWRD_SPEC> {
        HASWRD_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haswrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haswrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASWRD_SPEC;
impl crate::RegisterSpec for HASWRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haswrd::R`](R) reader structure"]
impl crate::Readable for HASWRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`haswrd::W`](W) writer structure"]
impl crate::Writable for HASWRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASWRD[%s]
to value 0x30"]
impl crate::Resettable for HASWRD_SPEC {
    const RESET_VALUE: u32 = 0x30;
}
