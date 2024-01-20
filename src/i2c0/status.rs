#[doc = "Register `Status` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `Status` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `FIFOEMPTY` reader - Indicates that the FIFO is empty."]
pub type FIFOEMPTY_R = crate::BitReader;
#[doc = "Field `FIFOFULL` reader - Indicates that the FIFO is full."]
pub type FIFOFULL_R = crate::BitReader;
#[doc = "Field `FIFOHALF` reader - Transmitter: Indicates that the FIFO is half-empty."]
pub type FIFOHALF_R = crate::BitReader;
#[doc = "Field `ADDRHIT` writer - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
pub type ADDRHIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLOSE` writer - Indicates that the controller has lost the bus arbitration."]
pub type ARBLOSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Indicates that a STOP Condition has been transmitted/received."]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` writer - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTETRANS` writer - Indicates that a byte of data has been transmitted."]
pub type BYTETRANS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTERECV` writer - Indicates that a byte of data has been received."]
pub type BYTERECV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPL` writer - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
pub type CMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `BUSBUSY` reader - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy"]
pub type BUSBUSY_R = crate::BitReader;
#[doc = "Field `GENCALL` reader - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call"]
pub type GENCALL_R = crate::BitReader;
#[doc = "Field `LINESCL` reader - Indicates the current status of the SCL line on the bus 1: high 0: low"]
pub type LINESCL_R = crate::BitReader;
#[doc = "Field `LINESDA` reader - Indicates the current status of the SDA line on the bus 1: high 0: low"]
pub type LINESDA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that the FIFO is empty."]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the FIFO is full."]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter: Indicates that the FIFO is half-empty."]
    #[inline(always)]
    pub fn fifohalf(&self) -> FIFOHALF_R {
        FIFOHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates the type of the last received/transmitted acknowledgement bit: 1: ACK 0: NACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates that the bus is busy The bus is busy when a START condition is on bus and it ends when a STOP condition is seen on bus 1: Busy 0: Not busy"]
    #[inline(always)]
    pub fn busbusy(&self) -> BUSBUSY_R {
        BUSBUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the address of the current transaction is a general call address: 1: General call 0: Not general call"]
    #[inline(always)]
    pub fn gencall(&self) -> GENCALL_R {
        GENCALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates the current status of the SCL line on the bus 1: high 0: low"]
    #[inline(always)]
    pub fn linescl(&self) -> LINESCL_R {
        LINESCL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates the current status of the SDA line on the bus 1: high 0: low"]
    #[inline(always)]
    pub fn linesda(&self) -> LINESDA_R {
        LINESDA_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Master: indicates that a slave has responded to the transaction. Slave: indicates that a transaction is targeting the controller (including the General Call)."]
    #[inline(always)]
    #[must_use]
    pub fn addrhit(&mut self) -> ADDRHIT_W<STATUS_SPEC> {
        ADDRHIT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that the controller has lost the bus arbitration."]
    #[inline(always)]
    #[must_use]
    pub fn arblose(&mut self) -> ARBLOSE_W<STATUS_SPEC> {
        ARBLOSE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that a STOP Condition has been transmitted/received."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<STATUS_SPEC> {
        STOP_W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that a START Condition or a repeated START condition has been transmitted/received."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<STATUS_SPEC> {
        START_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that a byte of data has been transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn bytetrans(&mut self) -> BYTETRANS_W<STATUS_SPEC> {
        BYTETRANS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates that a byte of data has been received."]
    #[inline(always)]
    #[must_use]
    pub fn byterecv(&mut self) -> BYTERECV_W<STATUS_SPEC> {
        BYTERECV_W::new(self, 8)
    }
    #[doc = "Bit 9 - Transaction Completion Master: Indicates that a transaction has been issued from this master and completed without losing the bus arbitration Slave: Indicates that a transaction addressing the controller has been completed. This status bit must be cleared to receive the next transaction; otherwise, the next incoming transaction will be blocked."]
    #[inline(always)]
    #[must_use]
    pub fn cmpl(&mut self) -> CMPL_W<STATUS_SPEC> {
        CMPL_W::new(self, 9)
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
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Status to value 0x01"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
