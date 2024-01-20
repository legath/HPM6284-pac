#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `RWMVIE` reader - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
pub type RWMVIE_R = crate::BitReader;
#[doc = "Field `RWMVIE` writer - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
pub type RWMVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWMEIE` reader - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
pub type TWMEIE_R = crate::BitReader;
#[doc = "Field `TWMEIE` writer - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
pub type TWMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFMFIE` reader - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
pub type RFMFIE_R = crate::BitReader;
#[doc = "Field `RFMFIE` writer - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
pub type RFMFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFMAIE` reader - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
pub type RFMAIE_R = crate::BitReader;
#[doc = "Field `RFMAIE` writer - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
pub type RFMAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFMEIE` reader - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
pub type TFMEIE_R = crate::BitReader;
#[doc = "Field `TFMEIE` writer - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
pub type TFMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFMAIE` reader - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
pub type TFMAIE_R = crate::BitReader;
#[doc = "Field `TFMAIE` writer - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
pub type TFMAIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
pub type BEIE_R = crate::BitReader;
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
pub type BEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BARCTL` reader - Bus Acccess Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
pub type BARCTL_R = crate::FieldReader;
#[doc = "Field `BARCTL` writer - Bus Acccess Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
pub type BARCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXRESET` reader - Reset TX Fifo and word."]
pub type TXRESET_R = crate::BitReader;
#[doc = "Field `TXRESET` writer - Reset TX Fifo and word."]
pub type TXRESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
    #[inline(always)]
    pub fn rwmvie(&self) -> RWMVIE_R {
        RWMVIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
    #[inline(always)]
    pub fn twmeie(&self) -> TWMEIE_R {
        TWMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
    #[inline(always)]
    pub fn rfmfie(&self) -> RFMFIE_R {
        RFMFIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
    #[inline(always)]
    pub fn rfmaie(&self) -> RFMAIE_R {
        RFMAIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
    #[inline(always)]
    pub fn tfmeie(&self) -> TFMEIE_R {
        TFMEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
    #[inline(always)]
    pub fn tfmaie(&self) -> TFMAIE_R {
        TFMAIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Bus Acccess Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
    #[inline(always)]
    pub fn barctl(&self) -> BARCTL_R {
        BARCTL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 31 - Reset TX Fifo and word."]
    #[inline(always)]
    pub fn txreset(&self) -> TXRESET_R {
        TXRESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX word message valid interrupt enable. 1, enable the RX word massage valid interrupt. 0, disable the RX word message valid interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rwmvie(&mut self) -> RWMVIE_W<CR_SPEC> {
        RWMVIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TX word message empty interrupt enable. 1, enable the TX word massage empty interrupt. 0, disable the TX word message empty interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn twmeie(&mut self) -> TWMEIE_W<CR_SPEC> {
        TWMEIE_W::new(self, 1)
    }
    #[doc = "Bit 4 - RX fifo message full interrupt enable. 1, enable the RX fifo message full interrupt. 0, disable the RX fifo message full interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rfmfie(&mut self) -> RFMFIE_W<CR_SPEC> {
        RFMFIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RX FIFO message available interrupt enable. 1, enable the RX FIFO massage available interrupt. 0, disable the RX FIFO message available interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rfmaie(&mut self) -> RFMAIE_W<CR_SPEC> {
        RFMAIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO message empty interrupt enable. 1, enable the TX FIFO massage empty interrupt. 0, disable the TX FIFO message empty interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tfmeie(&mut self) -> TFMEIE_W<CR_SPEC> {
        TFMEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO message available interrupt enable. 1, enable the TX FIFO massage available interrupt. 0, disable the TX FIFO message available interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tfmaie(&mut self) -> TFMAIE_W<CR_SPEC> {
        TFMAIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bus Error Interrupt Enable, will enable the interrupt for any bus error as described in the SR bit 13 to bit 8. 1, enable the bus access error interrupt. 0, disable the bus access error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn beie(&mut self) -> BEIE_W<CR_SPEC> {
        BEIE_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Bus Acccess Response Control, when bit 15:14= 00: no bus error will be generated, no wait for fifo write when fifo full and no wait for word/fifo read when word message invalid or fifo empty; or when write to word/fifo message will be ignored. 01: bus error will be generated when: 1, access invalid address; 2, write to ready only addr; 3, write to fulled fifo or valid message; 4, read from a emptied fifo/word message. 10: no error will be generated, but bus will wait when 1, write to fulled fifo/reg message; 2, read from a emptied fifo/reg message; write to word message will overwrite the existing reg value enven word message are still valid; read from invalid word message will read out last read out message data.happen. 11: reserved."]
    #[inline(always)]
    #[must_use]
    pub fn barctl(&mut self) -> BARCTL_W<CR_SPEC> {
        BARCTL_W::new(self, 14)
    }
    #[doc = "Bit 31 - Reset TX Fifo and word."]
    #[inline(always)]
    #[must_use]
    pub fn txreset(&mut self) -> TXRESET_W<CR_SPEC> {
        TXRESET_W::new(self, 31)
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
#[doc = "Command Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
