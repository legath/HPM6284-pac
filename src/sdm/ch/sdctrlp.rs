#[doc = "Register `SDCTRLP` reader"]
pub type R = crate::R<SDCTRLP_SPEC>;
#[doc = "Register `SDCTRLP` writer"]
pub type W = crate::W<SDCTRLP_SPEC>;
#[doc = "Field `EN` reader - Data Path Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Data Path Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR_OPT` reader - 1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready"]
pub type DR_OPT_R = crate::BitReader;
#[doc = "Field `DR_OPT` writer - 1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready"]
pub type DR_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D32` reader - 1:32 bit data 0:16 bit data"]
pub type D32_R = crate::BitReader;
#[doc = "Field `D32` writer - 1:32 bit data 0:16 bit data"]
pub type D32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTSYNCEN` reader - 1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled"]
pub type WTSYNCEN_R = crate::BitReader;
#[doc = "Field `WTSYNCEN` writer - 1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled"]
pub type WTSYNCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTSYNMCLR` reader - 1: Manually clear WTSYNFLG. Auto-clear."]
pub type WTSYNMCLR_R = crate::BitReader;
#[doc = "Field `WTSYNMCLR` writer - 1: Manually clear WTSYNFLG. Auto-clear."]
pub type WTSYNMCLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTSYNACLR` reader - 1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR"]
pub type WTSYNACLR_R = crate::BitReader;
#[doc = "Field `WTSYNACLR` writer - 1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR"]
pub type WTSYNACLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFSYNCCLREN` reader - Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1"]
pub type FFSYNCCLREN_R = crate::BitReader;
#[doc = "Field `FFSYNCCLREN` writer - Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1"]
pub type FFSYNCCLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCSEL` reader - Select the PWM SYNC Source"]
pub type SYNCSEL_R = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Select the PWM SYNC Source"]
pub type SYNCSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DRIE` reader - Ch Data Ready Interrupt Enable"]
pub type DRIE_R = crate::BitReader;
#[doc = "Field `DRIE` writer - Ch Data Ready Interrupt Enable"]
pub type DRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSATIE` reader - Ch CIC Data Saturation Interrupt Enable"]
pub type DSATIE_R = crate::BitReader;
#[doc = "Field `DSATIE` writer - Ch CIC Data Saturation Interrupt Enable"]
pub type DSATIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFFOVIE` reader - Ch Data FIFO overflow interrupt enable"]
pub type DFFOVIE_R = crate::BitReader;
#[doc = "Field `DFFOVIE` writer - Ch Data FIFO overflow interrupt enable"]
pub type DFFOVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_IE` reader - Acknowledge feedback interrupt enable"]
pub type AF_IE_R = crate::BitReader;
#[doc = "Field `AF_IE` writer - Acknowledge feedback interrupt enable"]
pub type AF_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG_THR` reader - Watch dog threshold for channel failure of CLK halting"]
pub type WDOG_THR_R = crate::FieldReader;
#[doc = "Field `WDOG_THR` writer - Watch dog threshold for channel failure of CLK halting"]
pub type WDOG_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MANCH_THR` reader - Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]"]
pub type MANCH_THR_R = crate::FieldReader;
#[doc = "Field `MANCH_THR` writer - Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]"]
pub type MANCH_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Data Path Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready"]
    #[inline(always)]
    pub fn dr_opt(&self) -> DR_OPT_R {
        DR_OPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1:32 bit data 0:16 bit data"]
    #[inline(always)]
    pub fn d32(&self) -> D32_R {
        D32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled"]
    #[inline(always)]
    pub fn wtsyncen(&self) -> WTSYNCEN_R {
        WTSYNCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Manually clear WTSYNFLG. Auto-clear."]
    #[inline(always)]
    pub fn wtsynmclr(&self) -> WTSYNMCLR_R {
        WTSYNMCLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR"]
    #[inline(always)]
    pub fn wtsynaclr(&self) -> WTSYNACLR_R {
        WTSYNACLR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1"]
    #[inline(always)]
    pub fn ffsyncclren(&self) -> FFSYNCCLREN_R {
        FFSYNCCLREN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Select the PWM SYNC Source"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Ch Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn drie(&self) -> DRIE_R {
        DRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Ch CIC Data Saturation Interrupt Enable"]
    #[inline(always)]
    pub fn dsatie(&self) -> DSATIE_R {
        DSATIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Ch Data FIFO overflow interrupt enable"]
    #[inline(always)]
    pub fn dffovie(&self) -> DFFOVIE_R {
        DFFOVIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Acknowledge feedback interrupt enable"]
    #[inline(always)]
    pub fn af_ie(&self) -> AF_IE_R {
        AF_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - Watch dog threshold for channel failure of CLK halting"]
    #[inline(always)]
    pub fn wdog_thr(&self) -> WDOG_THR_R {
        WDOG_THR_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:31 - Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]"]
    #[inline(always)]
    pub fn manch_thr(&self) -> MANCH_THR_R {
        MANCH_THR_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data Path Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<SDCTRLP_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Use Data FIFO Ready as data ready when fifo fillings are greater than the threshold 0: Use Data Reg Ready as data ready"]
    #[inline(always)]
    #[must_use]
    pub fn dr_opt(&mut self) -> DR_OPT_W<SDCTRLP_SPEC> {
        DR_OPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1:32 bit data 0:16 bit data"]
    #[inline(always)]
    #[must_use]
    pub fn d32(&mut self) -> D32_W<SDCTRLP_SPEC> {
        D32_W::new(self, 2)
    }
    #[doc = "Bit 3 - 1: Start to store data only after PWM SYNC event 0: Start to store data whenever enabled"]
    #[inline(always)]
    #[must_use]
    pub fn wtsyncen(&mut self) -> WTSYNCEN_W<SDCTRLP_SPEC> {
        WTSYNCEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1: Manually clear WTSYNFLG. Auto-clear."]
    #[inline(always)]
    #[must_use]
    pub fn wtsynmclr(&mut self) -> WTSYNMCLR_W<SDCTRLP_SPEC> {
        WTSYNMCLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1: Asserted to Auto clear WTSYNFLG when the SDFFINT is gen 0: WTSYNFLG should be cleared manually by WTSYNMCLR"]
    #[inline(always)]
    #[must_use]
    pub fn wtsynaclr(&mut self) -> WTSYNACLR_W<SDCTRLP_SPEC> {
        WTSYNACLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Auto clear FIFO when a new SDSYNC event is found. Only valid when WTSYNCEN=1"]
    #[inline(always)]
    #[must_use]
    pub fn ffsyncclren(&mut self) -> FFSYNCCLREN_W<SDCTRLP_SPEC> {
        FFSYNCCLREN_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - Select the PWM SYNC Source"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SYNCSEL_W<SDCTRLP_SPEC> {
        SYNCSEL_W::new(self, 7)
    }
    #[doc = "Bit 13 - Ch Data Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn drie(&mut self) -> DRIE_W<SDCTRLP_SPEC> {
        DRIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Ch CIC Data Saturation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsatie(&mut self) -> DSATIE_W<SDCTRLP_SPEC> {
        DSATIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Ch Data FIFO overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dffovie(&mut self) -> DFFOVIE_W<SDCTRLP_SPEC> {
        DFFOVIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Acknowledge feedback interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn af_ie(&mut self) -> AF_IE_W<SDCTRLP_SPEC> {
        AF_IE_W::new(self, 16)
    }
    #[doc = "Bits 17:24 - Watch dog threshold for channel failure of CLK halting"]
    #[inline(always)]
    #[must_use]
    pub fn wdog_thr(&mut self) -> WDOG_THR_W<SDCTRLP_SPEC> {
        WDOG_THR_W::new(self, 17)
    }
    #[doc = "Bits 25:31 - Manchester Decoding threshold. 3/4 of PERIOD_MCLK\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn manch_thr(&mut self) -> MANCH_THR_W<SDCTRLP_SPEC> {
        MANCH_THR_W::new(self, 25)
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
#[doc = "Data Path Control Primary Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdctrlp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdctrlp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDCTRLP_SPEC;
impl crate::RegisterSpec for SDCTRLP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdctrlp::R`](R) reader structure"]
impl crate::Readable for SDCTRLP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdctrlp::W`](W) writer structure"]
impl crate::Writable for SDCTRLP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDCTRLP to value 0"]
impl crate::Resettable for SDCTRLP_SPEC {
    const RESET_VALUE: u32 = 0;
}
