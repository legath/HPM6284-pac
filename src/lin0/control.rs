#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `START_REQ` reader - master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred"]
pub type START_REQ_R = crate::BitReader;
#[doc = "Field `START_REQ` writer - master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred"]
pub type START_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_REQ` reader - wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core"]
pub type WAKEUP_REQ_R = crate::BitReader;
#[doc = "Field `WAKEUP_REQ` writer - wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core"]
pub type WAKEUP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ERROR` writer - assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0"]
pub type RESET_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_INT` writer - write 1 to reset the int bit in the status register and the interrupt request output of LIN"]
pub type RESET_INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ACK` reader - slave only. Write 1 after handling a data request interrupt"]
pub type DATA_ACK_R = crate::BitReader;
#[doc = "Field `DATA_ACK` writer - slave only. Write 1 after handling a data request interrupt"]
pub type DATA_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSMIT` reader - 1: transmit operation 0: receive operation"]
pub type TRANSMIT_R = crate::BitReader;
#[doc = "Field `TRANSMIT` writer - 1: transmit operation 0: receive operation"]
pub type TRANSMIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred"]
    #[inline(always)]
    pub fn start_req(&self) -> START_REQ_R {
        START_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core"]
    #[inline(always)]
    pub fn wakeup_req(&self) -> WAKEUP_REQ_R {
        WAKEUP_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - slave only. Write 1 after handling a data request interrupt"]
    #[inline(always)]
    pub fn data_ack(&self) -> DATA_ACK_R {
        DATA_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: transmit operation 0: receive operation"]
    #[inline(always)]
    pub fn transmit(&self) -> TRANSMIT_R {
        TRANSMIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - master only. Set by host controller of a LIN master to start the LIN transmission. The core will reset the bit after the transmission is finished or an error is occurred"]
    #[inline(always)]
    #[must_use]
    pub fn start_req(&mut self) -> START_REQ_W<CONTROL_SPEC> {
        START_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - wakeup request. Assert to terminate the Sleep mode of the LIN bus. The bit will be reset by core"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_req(&mut self) -> WAKEUP_REQ_W<CONTROL_SPEC> {
        WAKEUP_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - assert 1 to reset the error bits in status register and error register. A read access to this bit delivers always the value 0"]
    #[inline(always)]
    #[must_use]
    pub fn reset_error(&mut self) -> RESET_ERROR_W<CONTROL_SPEC> {
        RESET_ERROR_W::new(self, 2)
    }
    #[doc = "Bit 3 - write 1 to reset the int bit in the status register and the interrupt request output of LIN"]
    #[inline(always)]
    #[must_use]
    pub fn reset_int(&mut self) -> RESET_INT_W<CONTROL_SPEC> {
        RESET_INT_W::new(self, 3)
    }
    #[doc = "Bit 4 - slave only. Write 1 after handling a data request interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn data_ack(&mut self) -> DATA_ACK_W<CONTROL_SPEC> {
        DATA_ACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: transmit operation 0: receive operation"]
    #[inline(always)]
    #[must_use]
    pub fn transmit(&mut self) -> TRANSMIT_W<CONTROL_SPEC> {
        TRANSMIT_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used by the LIN core to determine whether the LIN bus is in sleep mode or not. Set this bit after sending or receiving a Sleep Mode frame or if a bus idle timeout interrupt is requested or if after a wakeup request there is no response from the master and a timeout is signaled. The bit will be automatically reset by the LIN core."]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<CONTROL_SPEC> {
        SLEEP_W::new(self, 6)
    }
    #[doc = "Bit 7 - slave only. Write 1 when the Host determin do not response to the data request according to a unkown ID"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CONTROL_SPEC> {
        STOP_W::new(self, 7)
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
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
