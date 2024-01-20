#[doc = "Register `SBUSCFG` reader"]
pub type R = crate::R<SBUSCFG_SPEC>;
#[doc = "Register `SBUSCFG` writer"]
pub type W = crate::W<SBUSCFG_SPEC>;
#[doc = "Field `AHBBRST` reader - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
pub type AHBBRST_R = crate::FieldReader;
#[doc = "Field `AHBBRST` writer - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
pub type AHBBRST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    pub fn ahbbrst(&self) -> AHBBRST_R {
        AHBBRST_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - AHBBRST AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority). NOTE: This register overrides n_BURSTSIZE register when its value is not zero. 000 - Incremental burst of unspecified length only 001 - INCR4 burst, then single transfer 010 - INCR8 burst, INCR4 burst, then single transfer 011 - INCR16 burst, INCR8 burst, INCR4 burst, then single transfer 100 - Reserved, don't use 101 - INCR4 burst, then incremental burst of unspecified length 110 - INCR8 burst, INCR4 burst, then incremental burst of unspecified length 111 - INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length"]
    #[inline(always)]
    #[must_use]
    pub fn ahbbrst(&mut self) -> AHBBRST_W<SBUSCFG_SPEC> {
        AHBBRST_W::new(self, 0)
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
#[doc = "System Bus Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbuscfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbuscfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBUSCFG_SPEC;
impl crate::RegisterSpec for SBUSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbuscfg::R`](R) reader structure"]
impl crate::Readable for SBUSCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sbuscfg::W`](W) writer structure"]
impl crate::Writable for SBUSCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SBUSCFG to value 0"]
impl crate::Resettable for SBUSCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
