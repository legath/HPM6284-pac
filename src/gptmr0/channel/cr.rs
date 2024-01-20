#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `CAPMODE` reader - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
pub type CAPMODE_R = crate::FieldReader;
#[doc = "Field `CAPMODE` writer - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
pub type CAPMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBGPAUSE` reader - 1- counter will pause if chip is in debug mode"]
pub type DBGPAUSE_R = crate::BitReader;
#[doc = "Field `DBGPAUSE` writer - 1- counter will pause if chip is in debug mode"]
pub type DBGPAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWSYNCIEN` reader - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
pub type SWSYNCIEN_R = crate::BitReader;
#[doc = "Field `SWSYNCIEN` writer - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
pub type SWSYNCIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - 1- enable dma"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - 1- enable dma"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL` reader - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;"]
pub type DMASEL_R = crate::FieldReader;
#[doc = "Field `DMASEL` writer - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;"]
pub type DMASEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPEN` reader - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPINIT` reader - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
pub type CMPINIT_R = crate::BitReader;
#[doc = "Field `CMPINIT` writer - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
pub type CMPINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - 1- counter enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - 1- counter enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCIREN` reader - 1- SYNCI is valid on its rising edge"]
pub type SYNCIREN_R = crate::BitReader;
#[doc = "Field `SYNCIREN` writer - 1- SYNCI is valid on its rising edge"]
pub type SYNCIREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCIFEN` reader - 1- SYNCI is valid on its falling edge"]
pub type SYNCIFEN_R = crate::BitReader;
#[doc = "Field `SYNCIFEN` writer - 1- SYNCI is valid on its falling edge"]
pub type SYNCIFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCFLW` reader - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
pub type SYNCFLW_R = crate::BitReader;
#[doc = "Field `SYNCFLW` writer - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
pub type SYNCFLW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRST` reader - 1- reset counter"]
pub type CNTRST_R = crate::BitReader;
#[doc = "Field `CNTRST` writer - 1- reset counter"]
pub type CNTRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTUPT` writer - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
pub type CNTUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    #[inline(always)]
    pub fn capmode(&self) -> CAPMODE_R {
        CAPMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 1- counter will pause if chip is in debug mode"]
    #[inline(always)]
    pub fn dbgpause(&self) -> DBGPAUSE_R {
        DBGPAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    #[inline(always)]
    pub fn swsyncien(&self) -> SWSYNCIEN_R {
        SWSYNCIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- enable dma"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;"]
    #[inline(always)]
    pub fn dmasel(&self) -> DMASEL_R {
        DMASEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    #[inline(always)]
    pub fn cmpinit(&self) -> CMPINIT_R {
        CMPINIT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1- counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- SYNCI is valid on its rising edge"]
    #[inline(always)]
    pub fn synciren(&self) -> SYNCIREN_R {
        SYNCIREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- SYNCI is valid on its falling edge"]
    #[inline(always)]
    pub fn syncifen(&self) -> SYNCIFEN_R {
        SYNCIFEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    #[inline(always)]
    pub fn syncflw(&self) -> SYNCFLW_R {
        SYNCFLW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1- reset counter"]
    #[inline(always)]
    pub fn cntrst(&self) -> CNTRST_R {
        CNTRST_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - This bitfield define the input capture mode 100: width measure mode, timer will calculate the input signal period and duty cycle 011: capture at both rising edge and falling edge 010: capture at falling edge 001: capture at rising edge 000: No capture"]
    #[inline(always)]
    #[must_use]
    pub fn capmode(&mut self) -> CAPMODE_W<CR_SPEC> {
        CAPMODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - 1- counter will pause if chip is in debug mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgpause(&mut self) -> DBGPAUSE_W<CR_SPEC> {
        DBGPAUSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - 1- enable software sync. When this bit is set, counter will reset to RLD when swsynct bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn swsyncien(&mut self) -> SWSYNCIEN_W<CR_SPEC> {
        SWSYNCIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1- enable dma"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<CR_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - select one of DMA request: 00- CMP0 flag 01- CMP1 flag 10- Input signal toggle captured 11- RLD flag, counter reload;"]
    #[inline(always)]
    #[must_use]
    pub fn dmasel(&mut self) -> DMASEL_W<CR_SPEC> {
        DMASEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - 1- Enable the channel output compare function. The output signal can be generated per comparator (CMPx) settings."]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<CR_SPEC> {
        CMPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Output compare initial poliarity 1- The channel output initial level is high 0- The channel output initial level is low User should set this bit before set CMPEN to 1."]
    #[inline(always)]
    #[must_use]
    pub fn cmpinit(&mut self) -> CMPINIT_W<CR_SPEC> {
        CMPINIT_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1- counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<CR_SPEC> {
        CEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1- SYNCI is valid on its rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn synciren(&mut self) -> SYNCIREN_W<CR_SPEC> {
        SYNCIREN_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1- SYNCI is valid on its falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn syncifen(&mut self) -> SYNCIFEN_W<CR_SPEC> {
        SYNCIFEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1- enable this channel to reset counter to reload(RLD) together with its previous channel. This bit is not valid for channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn syncflw(&mut self) -> SYNCFLW_W<CR_SPEC> {
        SYNCFLW_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1- reset counter"]
    #[inline(always)]
    #[must_use]
    pub fn cntrst(&mut self) -> CNTRST_W<CR_SPEC> {
        CNTRST_W::new(self, 14)
    }
    #[doc = "Bit 31 - 1- update counter to new value as CNTUPTVAL This bit will be auto cleared after 1 cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cntupt(&mut self) -> CNTUPT_W<CR_SPEC> {
        CNTUPT_W::new(self, 31)
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
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
