#[doc = "Register `ERR` reader"]
pub type R = crate::R<ERR_SPEC>;
#[doc = "Register `ERR` writer"]
pub type W = crate::W<ERR_SPEC>;
#[doc = "Field `SCKERR` reader - Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]"]
pub type SCKERR_R = crate::BitReader;
#[doc = "Field `FUFE` reader - FIFO access error(underflow)"]
pub type FUFE_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Self-test error Indicates that the RNG failed the most recent self test. This bit is sticky and can only be reset by a hardware reset or by writing 1 to the CMD\\[CE\\]"]
    #[inline(always)]
    pub fn sckerr(&self) -> SCKERR_R {
        SCKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO access error(underflow)"]
    #[inline(always)]
    pub fn fufe(&self) -> FUFE_R {
        FUFE_R::new(((self.bits >> 5) & 1) != 0)
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
#[doc = "Error Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ERR_SPEC;
impl crate::RegisterSpec for ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err::R`](R) reader structure"]
impl crate::Readable for ERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`err::W`](W) writer structure"]
impl crate::Writable for ERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ERR to value 0"]
impl crate::Resettable for ERR_SPEC {
    const RESET_VALUE: u32 = 0;
}
