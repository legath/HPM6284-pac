#[doc = "Register `MSR` reader"]
pub type R = crate::R<MSR_SPEC>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MSR_SPEC>;
#[doc = "Field `DCTS` reader - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
pub type DCTS_R = crate::BitReader;
#[doc = "Field `CTS` reader - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
pub type CTS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta clear to send This bit is set when the state of the modem_ctsn input signal has been changed since the last time this register is read."]
    #[inline(always)]
    pub fn dcts(&self) -> DCTS_R {
        DCTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Clear to send 0: The modem_ctsn input signal is HIGH. 1: The modem_ctsn input signal is LOW."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 4) & 1) != 0)
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
#[doc = "Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSR_SPEC;
impl crate::RegisterSpec for MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
