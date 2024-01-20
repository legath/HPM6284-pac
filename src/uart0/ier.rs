#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Field `ERBI` reader - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
pub type ERBI_R = crate::BitReader;
#[doc = "Field `ERBI` writer - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
pub type ERBI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETHEI` reader - Enable transmitter holding register interrupt"]
pub type ETHEI_R = crate::BitReader;
#[doc = "Field `ETHEI` writer - Enable transmitter holding register interrupt"]
pub type ETHEI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSI` reader - Enable receiver line status interrupt"]
pub type ELSI_R = crate::BitReader;
#[doc = "Field `ELSI` writer - Enable receiver line status interrupt"]
pub type ELSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMSI` reader - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
pub type EMSI_R = crate::BitReader;
#[doc = "Field `EMSI` writer - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
pub type EMSI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERXIDLE` reader - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt"]
pub type ERXIDLE_R = crate::BitReader;
#[doc = "Field `ERXIDLE` writer - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt"]
pub type ERXIDLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn erbi(&self) -> ERBI_R {
        ERBI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmitter holding register interrupt"]
    #[inline(always)]
    pub fn ethei(&self) -> ETHEI_R {
        ETHEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable receiver line status interrupt"]
    #[inline(always)]
    pub fn elsi(&self) -> ELSI_R {
        ELSI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
    #[inline(always)]
    pub fn emsi(&self) -> EMSI_R {
        EMSI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt"]
    #[inline(always)]
    pub fn erxidle(&self) -> ERXIDLE_R {
        ERXIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable received data available interrupt and the character timeout interrupt 0: Disable 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erbi(&mut self) -> ERBI_W<IER_SPEC> {
        ERBI_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmitter holding register interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ethei(&mut self) -> ETHEI_W<IER_SPEC> {
        ETHEI_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable receiver line status interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn elsi(&mut self) -> ELSI_W<IER_SPEC> {
        ELSI_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable modem status interrupt The interrupt asserts when the status of one of the following occurs: The status of modem_rin, modem_dcdn, modem_dsrn or modem_ctsn (If the auto-cts mode is disabled) has been changed. If the auto-cts mode is enabled (MCR bit4 (AFE) = 1), modem_ctsn would be used to control the transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn emsi(&mut self) -> EMSI_W<IER_SPEC> {
        EMSI_W::new(self, 3)
    }
    #[doc = "Bit 31 - Enable Receive Idle interrupt 0 - Disable Idle interrupt 1 - Enable Idle interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn erxidle(&mut self) -> ERXIDLE_W<IER_SPEC> {
        ERXIDLE_W::new(self, 31)
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
#[doc = "Interrupt Enable Register (when DLAB = 0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: u32 = 0;
}
