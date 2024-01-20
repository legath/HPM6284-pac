#[doc = "Register `gcr` reader"]
pub type R = crate::R<GCR_SPEC>;
#[doc = "Register `gcr` writer"]
pub type W = crate::W<GCR_SPEC>;
#[doc = "Field `SWFRC` reader - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
pub type SWFRC_R = crate::BitReader;
#[doc = "Field `SWFRC` writer - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
pub type SWFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCTIME` writer - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
pub type FRCTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMERRESET` reader - set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear"]
pub type TIMERRESET_R = crate::BitReader;
#[doc = "Field `TIMERRESET` writer - set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear"]
pub type TIMERRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HR_PWM_EN` reader - set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
pub type HR_PWM_EN_R = crate::BitReader;
#[doc = "Field `HR_PWM_EN` writer - set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
pub type HR_PWM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XRLDSYNCEN` reader - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
pub type XRLDSYNCEN_R = crate::BitReader;
#[doc = "Field `XRLDSYNCEN` writer - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
pub type XRLDSYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTCLR` reader - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
pub type FAULTCLR_R = crate::BitReader;
#[doc = "Field `FAULTCLR` writer - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
pub type FAULTCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLDSYNCEN` reader - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
pub type RLDSYNCEN_R = crate::BitReader;
#[doc = "Field `RLDSYNCEN` writer - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
pub type RLDSYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTEXPOL` reader - external fault polarity 1-active low 0-active high"]
pub type FAULTEXPOL_R = crate::FieldReader;
#[doc = "Field `FAULTEXPOL` writer - external fault polarity 1-active low 0-active high"]
pub type FAULTEXPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAULTE0EN` reader - 1- enable the external fault input 0"]
pub type FAULTE0EN_R = crate::BitReader;
#[doc = "Field `FAULTE0EN` writer - 1- enable the external fault input 0"]
pub type FAULTE0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTE1EN` reader - 1- enable the external fault input 1"]
pub type FAULTE1EN_R = crate::BitReader;
#[doc = "Field `FAULTE1EN` writer - 1- enable the external fault input 1"]
pub type FAULTE1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTRECHWSEL` reader - Selec one of the 24 comparators as fault output recover trigger."]
pub type FAULTRECHWSEL_R = crate::FieldReader;
#[doc = "Field `FAULTRECHWSEL` writer - Selec one of the 24 comparators as fault output recover trigger."]
pub type FAULTRECHWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FAULTRECEDG` reader - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
pub type FAULTRECEDG_R = crate::BitReader;
#[doc = "Field `FAULTRECEDG` writer - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
pub type FAULTRECEDG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
pub type CMPSHDWSEL_R = crate::FieldReader;
#[doc = "Field `CMPSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
pub type CMPSHDWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HWSHDWEDG` reader - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge"]
pub type HWSHDWEDG_R = crate::BitReader;
#[doc = "Field `HWSHDWEDG` writer - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge"]
pub type HWSHDWEDG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCPOL` reader - polarity of input pwm_force, 1- active low 0- active high"]
pub type FRCPOL_R = crate::BitReader;
#[doc = "Field `FRCPOL` writer - polarity of input pwm_force, 1- active low 0- active high"]
pub type FRCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUGFAULT` reader - 1- enable debug mode output protection"]
pub type DEBUGFAULT_R = crate::BitReader;
#[doc = "Field `DEBUGFAULT` writer - 1- enable debug mode output protection"]
pub type DEBUGFAULT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTI0EN` reader - 1- enable the internal fault input 0"]
pub type FAULTI0EN_R = crate::BitReader;
#[doc = "Field `FAULTI0EN` writer - 1- enable the internal fault input 0"]
pub type FAULTI0EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTI1EN` reader - 1- enable the internal fault input 1"]
pub type FAULTI1EN_R = crate::BitReader;
#[doc = "Field `FAULTI1EN` writer - 1- enable the internal fault input 1"]
pub type FAULTI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTI2EN` reader - 1- enable the internal fault input 2"]
pub type FAULTI2EN_R = crate::BitReader;
#[doc = "Field `FAULTI2EN` writer - 1- enable the internal fault input 2"]
pub type FAULTI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTI3EN` reader - 1- enable the internal fault input 3"]
pub type FAULTI3EN_R = crate::BitReader;
#[doc = "Field `FAULTI3EN` writer - 1- enable the internal fault input 3"]
pub type FAULTI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
    #[inline(always)]
    pub fn swfrc(&self) -> SWFRC_R {
        SWFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear"]
    #[inline(always)]
    pub fn timerreset(&self) -> TIMERRESET_R {
        TIMERRESET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
    #[inline(always)]
    pub fn hr_pwm_en(&self) -> HR_PWM_EN_R {
        HR_PWM_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
    #[inline(always)]
    pub fn xrldsyncen(&self) -> XRLDSYNCEN_R {
        XRLDSYNCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
    #[inline(always)]
    pub fn faultclr(&self) -> FAULTCLR_R {
        FAULTCLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
    #[inline(always)]
    pub fn rldsyncen(&self) -> RLDSYNCEN_R {
        RLDSYNCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - external fault polarity 1-active low 0-active high"]
    #[inline(always)]
    pub fn faultexpol(&self) -> FAULTEXPOL_R {
        FAULTEXPOL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - 1- enable the external fault input 0"]
    #[inline(always)]
    pub fn faulte0en(&self) -> FAULTE0EN_R {
        FAULTE0EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- enable the external fault input 1"]
    #[inline(always)]
    pub fn faulte1en(&self) -> FAULTE1EN_R {
        FAULTE1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:17 - Selec one of the 24 comparators as fault output recover trigger."]
    #[inline(always)]
    pub fn faultrechwsel(&self) -> FAULTRECHWSEL_R {
        FAULTRECHWSEL_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn faultrecedg(&self) -> FAULTRECEDG_R {
        FAULTRECEDG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
    #[inline(always)]
    pub fn cmpshdwsel(&self) -> CMPSHDWSEL_R {
        CMPSHDWSEL_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    pub fn hwshdwedg(&self) -> HWSHDWEDG_R {
        HWSHDWEDG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - polarity of input pwm_force, 1- active low 0- active high"]
    #[inline(always)]
    pub fn frcpol(&self) -> FRCPOL_R {
        FRCPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1- enable debug mode output protection"]
    #[inline(always)]
    pub fn debugfault(&self) -> DEBUGFAULT_R {
        DEBUGFAULT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1- enable the internal fault input 0"]
    #[inline(always)]
    pub fn faulti0en(&self) -> FAULTI0EN_R {
        FAULTI0EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1- enable the internal fault input 1"]
    #[inline(always)]
    pub fn faulti1en(&self) -> FAULTI1EN_R {
        FAULTI1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1- enable the internal fault input 2"]
    #[inline(always)]
    pub fn faulti2en(&self) -> FAULTI2EN_R {
        FAULTI2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1- enable the internal fault input 3"]
    #[inline(always)]
    pub fn faulti3en(&self) -> FAULTI3EN_R {
        FAULTI3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1- write 1 to enable software force, if the frcsrcsel is set to 0, force will take effect"]
    #[inline(always)]
    #[must_use]
    pub fn swfrc(&mut self) -> SWFRC_W<GCR_SPEC> {
        SWFRC_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - This bit field select the force effective time 00: force immediately 01: force at main counter reload time 10: force at FRCSYNCI 11: no force"]
    #[inline(always)]
    #[must_use]
    pub fn frctime(&mut self) -> FRCTIME_W<GCR_SPEC> {
        FRCTIME_W::new(self, 1)
    }
    #[doc = "Bit 3 - set to clear current timer(total 28bit, main counter and tmout_count ). Auto clear"]
    #[inline(always)]
    #[must_use]
    pub fn timerreset(&mut self) -> TIMERRESET_W<GCR_SPEC> {
        TIMERRESET_W::new(self, 3)
    }
    #[doc = "Bit 4 - set to enable high resolution pwm, trig_cmp, start/reload register will have different definition."]
    #[inline(always)]
    #[must_use]
    pub fn hr_pwm_en(&mut self) -> HR_PWM_EN_W<GCR_SPEC> {
        HR_PWM_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1- pwm timer extended counter (xcnt) reset to extended reload value (xrld) by synci is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xrldsyncen(&mut self) -> XRLDSYNCEN_W<GCR_SPEC> {
        XRLDSYNCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1- Write 1 to clear the fault condition. The output will recover if FAULTRECTIME is set to 2b'11. User should write 1 to this bit after the active FAULT signal de-assert and before it re-assert again."]
    #[inline(always)]
    #[must_use]
    pub fn faultclr(&mut self) -> FAULTCLR_W<GCR_SPEC> {
        FAULTCLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - 1- enable the pwm timer counter 0- stop the pwm timer counter"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<GCR_SPEC> {
        CEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - 1- pwm timer counter reset to reload value (rld) by synci is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rldsyncen(&mut self) -> RLDSYNCEN_W<GCR_SPEC> {
        RLDSYNCEN_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - external fault polarity 1-active low 0-active high"]
    #[inline(always)]
    #[must_use]
    pub fn faultexpol(&mut self) -> FAULTEXPOL_W<GCR_SPEC> {
        FAULTEXPOL_W::new(self, 9)
    }
    #[doc = "Bit 11 - 1- enable the external fault input 0"]
    #[inline(always)]
    #[must_use]
    pub fn faulte0en(&mut self) -> FAULTE0EN_W<GCR_SPEC> {
        FAULTE0EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1- enable the external fault input 1"]
    #[inline(always)]
    #[must_use]
    pub fn faulte1en(&mut self) -> FAULTE1EN_W<GCR_SPEC> {
        FAULTE1EN_W::new(self, 12)
    }
    #[doc = "Bits 13:17 - Selec one of the 24 comparators as fault output recover trigger."]
    #[inline(always)]
    #[must_use]
    pub fn faultrechwsel(&mut self) -> FAULTRECHWSEL_W<GCR_SPEC> {
        FAULTRECHWSEL_W::new(self, 13)
    }
    #[doc = "Bit 18 - When hardware load is selected as output fault recover trigger and the selected channel is capture mode. This bit assign its effective edge of fault recover trigger. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn faultrecedg(&mut self) -> FAULTRECEDG_W<GCR_SPEC> {
        FAULTRECEDG_W::new(self, 18)
    }
    #[doc = "Bits 19:23 - This bitfield select one of the comparators as hardware event time to load comparator shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpshdwsel(&mut self) -> CMPSHDWSEL_W<GCR_SPEC> {
        CMPSHDWSEL_W::new(self, 19)
    }
    #[doc = "Bit 24 - When hardware event is selected as shawdow register effective time and the select comparator is configured as input capture mode. This bit assign its which edge is used as compare shadow register hardware load event. 1- Falling edge 0- Rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn hwshdwedg(&mut self) -> HWSHDWEDG_W<GCR_SPEC> {
        HWSHDWEDG_W::new(self, 24)
    }
    #[doc = "Bit 26 - polarity of input pwm_force, 1- active low 0- active high"]
    #[inline(always)]
    #[must_use]
    pub fn frcpol(&mut self) -> FRCPOL_W<GCR_SPEC> {
        FRCPOL_W::new(self, 26)
    }
    #[doc = "Bit 27 - 1- enable debug mode output protection"]
    #[inline(always)]
    #[must_use]
    pub fn debugfault(&mut self) -> DEBUGFAULT_W<GCR_SPEC> {
        DEBUGFAULT_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1- enable the internal fault input 0"]
    #[inline(always)]
    #[must_use]
    pub fn faulti0en(&mut self) -> FAULTI0EN_W<GCR_SPEC> {
        FAULTI0EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1- enable the internal fault input 1"]
    #[inline(always)]
    #[must_use]
    pub fn faulti1en(&mut self) -> FAULTI1EN_W<GCR_SPEC> {
        FAULTI1EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1- enable the internal fault input 2"]
    #[inline(always)]
    #[must_use]
    pub fn faulti2en(&mut self) -> FAULTI2EN_W<GCR_SPEC> {
        FAULTI2EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1- enable the internal fault input 3"]
    #[inline(always)]
    #[must_use]
    pub fn faulti3en(&mut self) -> FAULTI3EN_W<GCR_SPEC> {
        FAULTI3EN_W::new(self, 31)
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
#[doc = "Global control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gcr to value 0"]
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
