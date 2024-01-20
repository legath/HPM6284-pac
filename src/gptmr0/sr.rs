#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SR_SPEC>;
#[doc = "Field `CH0RLDF` writer - channel 1 counter reload flag"]
pub type CH0RLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH0CAPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CMP0F` writer - channel 1 compare value 1 match flag"]
pub type CH0CMP0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0CMP1F` writer - channel 1 compare value 1 match flag"]
pub type CH0CMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1RLDF` writer - channel 1 counter reload flag"]
pub type CH1RLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CAPF` writer - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH1CAPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CMP0F` writer - channel 1 compare value 1 match flag"]
pub type CH1CMP0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CMP1F` writer - channel 1 compare value 1 match flag"]
pub type CH1CMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2RLDF` writer - channel 2 counter reload flag"]
pub type CH2RLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CAPF` writer - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH2CAPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CMP0F` writer - channel 2 compare value 1 match flag"]
pub type CH2CMP0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2CMP1F` writer - channel 2 compare value 1 match flag"]
pub type CH2CMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3RLDF` writer - channel 3 counter reload flag"]
pub type CH3RLDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CAPF` writer - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
pub type CH3CAPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CMP0F` writer - channel 3 compare value 1 match flag"]
pub type CH3CMP0F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3CMP1F` writer - channel 3 compare value 1 match flag"]
pub type CH3CMP1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - channel 1 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0rldf(&mut self) -> CH0RLDF_W<SR_SPEC> {
        CH0RLDF_W::new(self, 0)
    }
    #[doc = "Bit 1 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch0capf(&mut self) -> CH0CAPF_W<SR_SPEC> {
        CH0CAPF_W::new(self, 1)
    }
    #[doc = "Bit 2 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp0f(&mut self) -> CH0CMP0F_W<SR_SPEC> {
        CH0CMP0F_W::new(self, 2)
    }
    #[doc = "Bit 3 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cmp1f(&mut self) -> CH0CMP1F_W<SR_SPEC> {
        CH0CMP1F_W::new(self, 3)
    }
    #[doc = "Bit 4 - channel 1 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1rldf(&mut self) -> CH1RLDF_W<SR_SPEC> {
        CH1RLDF_W::new(self, 4)
    }
    #[doc = "Bit 5 - channel 1 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch1capf(&mut self) -> CH1CAPF_W<SR_SPEC> {
        CH1CAPF_W::new(self, 5)
    }
    #[doc = "Bit 6 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp0f(&mut self) -> CH1CMP0F_W<SR_SPEC> {
        CH1CMP0F_W::new(self, 6)
    }
    #[doc = "Bit 7 - channel 1 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cmp1f(&mut self) -> CH1CMP1F_W<SR_SPEC> {
        CH1CMP1F_W::new(self, 7)
    }
    #[doc = "Bit 8 - channel 2 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2rldf(&mut self) -> CH2RLDF_W<SR_SPEC> {
        CH2RLDF_W::new(self, 8)
    }
    #[doc = "Bit 9 - channel 2 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch2capf(&mut self) -> CH2CAPF_W<SR_SPEC> {
        CH2CAPF_W::new(self, 9)
    }
    #[doc = "Bit 10 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp0f(&mut self) -> CH2CMP0F_W<SR_SPEC> {
        CH2CMP0F_W::new(self, 10)
    }
    #[doc = "Bit 11 - channel 2 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2cmp1f(&mut self) -> CH2CMP1F_W<SR_SPEC> {
        CH2CMP1F_W::new(self, 11)
    }
    #[doc = "Bit 12 - channel 3 counter reload flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3rldf(&mut self) -> CH3RLDF_W<SR_SPEC> {
        CH3RLDF_W::new(self, 12)
    }
    #[doc = "Bit 13 - channel 3 capture flag, the flag will be set at the valid capture edge per CAPMODE setting. If the capture channel is set to measure mode, the flag will be set at rising edge."]
    #[inline(always)]
    #[must_use]
    pub fn ch3capf(&mut self) -> CH3CAPF_W<SR_SPEC> {
        CH3CAPF_W::new(self, 13)
    }
    #[doc = "Bit 14 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp0f(&mut self) -> CH3CMP0F_W<SR_SPEC> {
        CH3CMP0F_W::new(self, 14)
    }
    #[doc = "Bit 15 - channel 3 compare value 1 match flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3cmp1f(&mut self) -> CH3CMP1F_W<SR_SPEC> {
        CH3CMP1F_W::new(self, 15)
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
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
