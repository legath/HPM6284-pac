#[doc = "Register `CMD` reader"]
pub type R = crate::R<CMD_SPEC>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CMD_SPEC>;
#[doc = "Field `SLFCHK` reader - Self Test, when both ST and GS triggered, ST first and GS next."]
pub type SLFCHK_R = crate::BitReader;
#[doc = "Field `SLFCHK` writer - Self Test, when both ST and GS triggered, ST first and GS next."]
pub type SLFCHK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSD` reader - Generate Seed, when both ST and GS triggered, ST first and GS next."]
pub type GENSD_R = crate::BitReader;
#[doc = "Field `GENSD` writer - Generate Seed, when both ST and GS triggered, ST first and GS next."]
pub type GENSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRINT` reader - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
pub type CLRINT_R = crate::BitReader;
#[doc = "Field `CLRINT` writer - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
pub type CLRINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRERR` reader - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
pub type CLRERR_R = crate::BitReader;
#[doc = "Field `CLRERR` writer - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
pub type CLRERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRST` reader - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
pub type SFTRST_R = crate::BitReader;
#[doc = "Field `SFTRST` writer - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
pub type SFTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Self Test, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    pub fn slfchk(&self) -> SLFCHK_R {
        SLFCHK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generate Seed, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    pub fn gensd(&self) -> GENSD_R {
        GENSD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
    #[inline(always)]
    pub fn clrint(&self) -> CLRINT_R {
        CLRINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
    #[inline(always)]
    pub fn clrerr(&self) -> CLRERR_R {
        CLRERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Self Test, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    #[must_use]
    pub fn slfchk(&mut self) -> SLFCHK_W<CMD_SPEC> {
        SLFCHK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Generate Seed, when both ST and GS triggered, ST first and GS next."]
    #[inline(always)]
    #[must_use]
    pub fn gensd(&mut self) -> GENSD_W<CMD_SPEC> {
        GENSD_W::new(self, 1)
    }
    #[doc = "Bit 4 - Clear the Interrupt, clear the RNG interrupt if an error is not present. This bit is self-clearing. 0 Do not clear the interrupt. 1 Clear the interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clrint(&mut self) -> CLRINT_W<CMD_SPEC> {
        CLRINT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear the Error, clear the errors in the ESR register and the RNG interrupt. This bit is self-clearing. 0 Do not clear the errors and the interrupt. 1 Clear the errors and the interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn clrerr(&mut self) -> CLRERR_W<CMD_SPEC> {
        CLRERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Soft Reset, Perform a software reset of the RNG This bit is self-clearing. 0 Do not perform a software reset. 1 Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<CMD_SPEC> {
        SFTRST_W::new(self, 6)
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
#[doc = "Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CMD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
