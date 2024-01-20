#[doc = "Register `Ctrl` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `Ctrl` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `DATACNT` reader - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
pub type DATACNT_R = crate::FieldReader;
#[doc = "Field `DATACNT` writer - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
pub type DATACNT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DIR` reader - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE_STOP` reader - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
pub type PHASE_STOP_R = crate::BitReader;
#[doc = "Field `PHASE_STOP` writer - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
pub type PHASE_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE_DATA` reader - Enable this bit to send the data after Address phase. Master mode only."]
pub type PHASE_DATA_R = crate::BitReader;
#[doc = "Field `PHASE_DATA` writer - Enable this bit to send the data after Address phase. Master mode only."]
pub type PHASE_DATA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE_ADDR` reader - Enable this bit to send the address after START condition. Master mode only."]
pub type PHASE_ADDR_R = crate::BitReader;
#[doc = "Field `PHASE_ADDR` writer - Enable this bit to send the address after START condition. Master mode only."]
pub type PHASE_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE_START` reader - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
pub type PHASE_START_R = crate::BitReader;
#[doc = "Field `PHASE_START` writer - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
pub type PHASE_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
    #[inline(always)]
    pub fn datacnt(&self) -> DATACNT_R {
        DATACNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_stop(&self) -> PHASE_STOP_R {
        PHASE_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable this bit to send the data after Address phase. Master mode only."]
    #[inline(always)]
    pub fn phase_data(&self) -> PHASE_DATA_R {
        PHASE_DATA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable this bit to send the address after START condition. Master mode only."]
    #[inline(always)]
    pub fn phase_addr(&self) -> PHASE_ADDR_R {
        PHASE_ADDR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
    #[inline(always)]
    pub fn phase_start(&self) -> PHASE_START_R {
        PHASE_START_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data counts in bytes. Master: The number of bytes to transmit/receive. 0 means max length. DataCnt will be decreased by one for each byte transmitted/received. Slave: the meaning of DataCnt depends on the DMA mode: If DMA is not enabled, DataCnt is the number of bytes transmitted/received from the bus master. It is reset to 0 when the controller is addressed and then increased by one for each byte of data transmitted/received. If DMA is enabled, DataCnt is the number of bytes to transmit/receive. It will not be reset to 0 when the slave is addressed and it will be decreased by one for each byte of data transmitted/received."]
    #[inline(always)]
    #[must_use]
    pub fn datacnt(&mut self) -> DATACNT_W<CTRL_SPEC> {
        DATACNT_W::new(self, 0)
    }
    #[doc = "Bit 8 - Transaction direction Master: Set this bit to determine the direction for the next transaction. 0: Transmitter 1: Receiver Slave: The direction of the last received transaction. 0: Receiver 1: Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CTRL_SPEC> {
        DIR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable this bit to send a STOP condition at the end of a transaction. Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn phase_stop(&mut self) -> PHASE_STOP_W<CTRL_SPEC> {
        PHASE_STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable this bit to send the data after Address phase. Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn phase_data(&mut self) -> PHASE_DATA_W<CTRL_SPEC> {
        PHASE_DATA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable this bit to send the address after START condition. Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn phase_addr(&mut self) -> PHASE_ADDR_W<CTRL_SPEC> {
        PHASE_ADDR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable this bit to send a START condition at the beginning of transaction. Master mode only."]
    #[inline(always)]
    #[must_use]
    pub fn phase_start(&mut self) -> PHASE_START_W<CTRL_SPEC> {
        PHASE_START_W::new(self, 12)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Ctrl to value 0x1e00"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x1e00;
}
