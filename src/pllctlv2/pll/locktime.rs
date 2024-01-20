#[doc = "Register `LOCKTIME` reader"]
pub type R = crate::R<LOCKTIME_SPEC>;
#[doc = "Register `LOCKTIME` writer"]
pub type W = crate::W<LOCKTIME_SPEC>;
#[doc = "Field `LOCKTIME` reader - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
pub type LOCKTIME_R = crate::FieldReader<u16>;
#[doc = "Field `LOCKTIME` writer - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
pub type LOCKTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
    #[inline(always)]
    pub fn locktime(&self) -> LOCKTIME_R {
        LOCKTIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lock time of PLL in 24M clock cycles, typical value is 2500. If MFI changed during PLL startup, PLL lock time may be longer than this setting."]
    #[inline(always)]
    #[must_use]
    pub fn locktime(&mut self) -> LOCKTIME_W<LOCKTIME_SPEC> {
        LOCKTIME_W::new(self, 0)
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
#[doc = "PLL0 lock time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`locktime::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`locktime::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCKTIME_SPEC;
impl crate::RegisterSpec for LOCKTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`locktime::R`](R) reader structure"]
impl crate::Readable for LOCKTIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`locktime::W`](W) writer structure"]
impl crate::Writable for LOCKTIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCKTIME to value 0x09c4"]
impl crate::Resettable for LOCKTIME_SPEC {
    const RESET_VALUE: u32 = 0x09c4;
}
