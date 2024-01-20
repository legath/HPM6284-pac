#[doc = "Register `IRC32K_CFG` reader"]
pub type R = crate::R<IRC32K_CFG_SPEC>;
#[doc = "Register `IRC32K_CFG` writer"]
pub type W = crate::W<IRC32K_CFG_SPEC>;
#[doc = "Field `CAP_TRIM` reader - capacitor trim bits"]
pub type CAP_TRIM_R = crate::FieldReader<u16>;
#[doc = "Field `CAP_TRIM` writer - capacitor trim bits"]
pub type CAP_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CAPEX6_TRIM` reader - IRC32K bit 6"]
pub type CAPEX6_TRIM_R = crate::BitReader;
#[doc = "Field `CAPEX6_TRIM` writer - IRC32K bit 6"]
pub type CAPEX6_TRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAPEX7_TRIM` reader - IRC32K bit 7"]
pub type CAPEX7_TRIM_R = crate::BitReader;
#[doc = "Field `CAPEX7_TRIM` writer - IRC32K bit 7"]
pub type CAPEX7_TRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRC_TRIMMED` reader - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
pub type IRC_TRIMMED_R = crate::BitReader;
#[doc = "Field `IRC_TRIMMED` writer - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
pub type IRC_TRIMMED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - capacitor trim bits"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CAP_TRIM_R {
        CAP_TRIM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - IRC32K bit 6"]
    #[inline(always)]
    pub fn capex6_trim(&self) -> CAPEX6_TRIM_R {
        CAPEX6_TRIM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IRC32K bit 7"]
    #[inline(always)]
    pub fn capex7_trim(&self) -> CAPEX7_TRIM_R {
        CAPEX7_TRIM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
    #[inline(always)]
    pub fn irc_trimmed(&self) -> IRC_TRIMMED_R {
        IRC_TRIMMED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - capacitor trim bits"]
    #[inline(always)]
    #[must_use]
    pub fn cap_trim(&mut self) -> CAP_TRIM_W<IRC32K_CFG_SPEC> {
        CAP_TRIM_W::new(self, 0)
    }
    #[doc = "Bit 22 - IRC32K bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn capex6_trim(&mut self) -> CAPEX6_TRIM_W<IRC32K_CFG_SPEC> {
        CAPEX6_TRIM_W::new(self, 22)
    }
    #[doc = "Bit 23 - IRC32K bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn capex7_trim(&mut self) -> CAPEX7_TRIM_W<IRC32K_CFG_SPEC> {
        CAPEX7_TRIM_W::new(self, 23)
    }
    #[doc = "Bit 31 - IRC32K trim happened, this bit set by hardware after trim value loaded, and stop load, write 0 will clear this bit and reload trim value 0: irc is not trimmed 1: irc is trimmed"]
    #[inline(always)]
    #[must_use]
    pub fn irc_trimmed(&mut self) -> IRC_TRIMMED_W<IRC32K_CFG_SPEC> {
        IRC_TRIMMED_W::new(self, 31)
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
#[doc = "On-chip 32k oscillator config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irc32k_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irc32k_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRC32K_CFG_SPEC;
impl crate::RegisterSpec for IRC32K_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irc32k_cfg::R`](R) reader structure"]
impl crate::Readable for IRC32K_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`irc32k_cfg::W`](W) writer structure"]
impl crate::Writable for IRC32K_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRC32K_CFG to value 0"]
impl crate::Resettable for IRC32K_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
