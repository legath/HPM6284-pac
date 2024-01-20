#[doc = "Register `spd` reader"]
pub type R = crate::R<SPD_SPEC>;
#[doc = "Register `spd` writer"]
pub type W = crate::W<SPD_SPEC>;
#[doc = "Field `SPDCNT` reader - spdcnt value"]
pub type SPDCNT_R = crate::FieldReader<u32>;
#[doc = "Field `BSTAT` reader - 1- b input is high 0- b input is low"]
pub type BSTAT_R = crate::BitReader;
#[doc = "Field `BSTAT` writer - 1- b input is high 0- b input is low"]
pub type BSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASTAT` reader - 1- a input is high 0- a input is low"]
pub type ASTAT_R = crate::BitReader;
#[doc = "Field `DIR` reader - 1- reverse rotation 0- forward rotation"]
pub type DIR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:27 - spdcnt value"]
    #[inline(always)]
    pub fn spdcnt(&self) -> SPDCNT_R {
        SPDCNT_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 29 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    pub fn bstat(&self) -> BSTAT_R {
        BSTAT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- a input is high 0- a input is low"]
    #[inline(always)]
    pub fn astat(&self) -> ASTAT_R {
        ASTAT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- reverse rotation 0- forward rotation"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - 1- b input is high 0- b input is low"]
    #[inline(always)]
    #[must_use]
    pub fn bstat(&mut self) -> BSTAT_W<SPD_SPEC> {
        BSTAT_W::new(self, 29)
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
#[doc = "Speed counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPD_SPEC;
impl crate::RegisterSpec for SPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spd::R`](R) reader structure"]
impl crate::Readable for SPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spd::W`](W) writer structure"]
impl crate::Writable for SPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets spd to value 0"]
impl crate::Resettable for SPD_SPEC {
    const RESET_VALUE: u32 = 0;
}
