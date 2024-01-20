#[doc = "Register `IIR` reader"]
pub type R = crate::R<IIR_SPEC>;
#[doc = "Register `IIR` writer"]
pub type W = crate::W<IIR_SPEC>;
#[doc = "Field `INTRID` reader - Interrupt ID, see IIR2 for detail decoding"]
pub type INTRID_R = crate::FieldReader;
#[doc = "Field `FIFOED` reader - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
pub type FIFOED_R = crate::FieldReader;
#[doc = "Field `RXIDLE_FLAG` writer - UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR"]
pub type RXIDLE_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Interrupt ID, see IIR2 for detail decoding"]
    #[inline(always)]
    pub fn intrid(&self) -> INTRID_R {
        INTRID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - FIFOs enabled These two bits are 1 when bit 0 of the FIFO Control Register (FIFOE) is set to 1."]
    #[inline(always)]
    pub fn fifoed(&self) -> FIFOED_R {
        FIFOED_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - UART IDLE Flag 0 - UART is busy 1 - UART is idle NOTE: when write one to clear this bit, avoid changging FCR register since it's same address as IIR"]
    #[inline(always)]
    #[must_use]
    pub fn rxidle_flag(&mut self) -> RXIDLE_FLAG_W<IIR_SPEC> {
        RXIDLE_FLAG_W::new(self, 31)
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
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IIR_SPEC;
impl crate::RegisterSpec for IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iir::W`](W) writer structure"]
impl crate::Writable for IIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IIR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
