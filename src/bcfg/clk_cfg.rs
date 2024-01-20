#[doc = "Register `CLK_CFG` reader"]
pub type R = crate::R<CLK_CFG_SPEC>;
#[doc = "Register `CLK_CFG` writer"]
pub type W = crate::W<CLK_CFG_SPEC>;
#[doc = "Field `FORCE_XTAL` reader - force switch to crystal"]
pub type FORCE_XTAL_R = crate::BitReader;
#[doc = "Field `FORCE_XTAL` writer - force switch to crystal"]
pub type FORCE_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEP_IRC` reader - force irc32k run"]
pub type KEEP_IRC_R = crate::BitReader;
#[doc = "Field `KEEP_IRC` writer - force irc32k run"]
pub type KEEP_IRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTAL_SEL` reader - crystal selected"]
pub type XTAL_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bit 4 - force switch to crystal"]
    #[inline(always)]
    pub fn force_xtal(&self) -> FORCE_XTAL_R {
        FORCE_XTAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - force irc32k run"]
    #[inline(always)]
    pub fn keep_irc(&self) -> KEEP_IRC_R {
        KEEP_IRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - crystal selected"]
    #[inline(always)]
    pub fn xtal_sel(&self) -> XTAL_SEL_R {
        XTAL_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - force switch to crystal"]
    #[inline(always)]
    #[must_use]
    pub fn force_xtal(&mut self) -> FORCE_XTAL_W<CLK_CFG_SPEC> {
        FORCE_XTAL_W::new(self, 4)
    }
    #[doc = "Bit 16 - force irc32k run"]
    #[inline(always)]
    #[must_use]
    pub fn keep_irc(&mut self) -> KEEP_IRC_W<CLK_CFG_SPEC> {
        KEEP_IRC_W::new(self, 16)
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
#[doc = "Clock config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg::R`](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg::W`](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
