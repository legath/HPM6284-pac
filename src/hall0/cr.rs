#[doc = "Register `cr` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `cr` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RSTCNT` reader - set to reset all counter and related snapshots"]
pub type RSTCNT_R = crate::BitReader;
#[doc = "Field `RSTCNT` writer - set to reset all counter and related snapshots"]
pub type RSTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPEN` reader - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_R = crate::BitReader;
#[doc = "Field `SNAPEN` writer - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ` writer - 1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - set to reset all counter and related snapshots"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    pub fn snapen(&self) -> SNAPEN_R {
        SNAPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - set to reset all counter and related snapshots"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RSTCNT_W<CR_SPEC> {
        RSTCNT_W::new(self, 4)
    }
    #[doc = "Bit 11 - 1- load ucnt, vcnt, wcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    #[must_use]
    pub fn snapen(&mut self) -> SNAPEN_W<CR_SPEC> {
        SNAPEN_W::new(self, 11)
    }
    #[doc = "Bit 31 - 1- load ucnt, vcnt, wcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<CR_SPEC> {
        READ_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
