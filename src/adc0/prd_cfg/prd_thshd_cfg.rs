#[doc = "Register `prd_thshd_cfg` reader"]
pub type R = crate::R<PRD_THSHD_CFG_SPEC>;
#[doc = "Register `prd_thshd_cfg` writer"]
pub type W = crate::W<PRD_THSHD_CFG_SPEC>;
#[doc = "Field `THSHDL` reader - threshold low"]
pub type THSHDL_R = crate::FieldReader<u16>;
#[doc = "Field `THSHDL` writer - threshold low"]
pub type THSHDL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `THSHDH` reader - threshold high, assert interrupt(if enabled) if result exceed high or low."]
pub type THSHDH_R = crate::FieldReader<u16>;
#[doc = "Field `THSHDH` writer - threshold high, assert interrupt(if enabled) if result exceed high or low."]
pub type THSHDH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - threshold low"]
    #[inline(always)]
    pub fn thshdl(&self) -> THSHDL_R {
        THSHDL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low."]
    #[inline(always)]
    pub fn thshdh(&self) -> THSHDH_R {
        THSHDH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - threshold low"]
    #[inline(always)]
    #[must_use]
    pub fn thshdl(&mut self) -> THSHDL_W<PRD_THSHD_CFG_SPEC> {
        THSHDL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - threshold high, assert interrupt(if enabled) if result exceed high or low."]
    #[inline(always)]
    #[must_use]
    pub fn thshdh(&mut self) -> THSHDH_W<PRD_THSHD_CFG_SPEC> {
        THSHDH_W::new(self, 16)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prd_thshd_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prd_thshd_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRD_THSHD_CFG_SPEC;
impl crate::RegisterSpec for PRD_THSHD_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prd_thshd_cfg::R`](R) reader structure"]
impl crate::Readable for PRD_THSHD_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prd_thshd_cfg::W`](W) writer structure"]
impl crate::Writable for PRD_THSHD_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets prd_thshd_cfg to value 0"]
impl crate::Resettable for PRD_THSHD_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
