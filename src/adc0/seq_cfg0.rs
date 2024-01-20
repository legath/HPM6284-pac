#[doc = "Register `seq_cfg0` reader"]
pub type R = crate::R<SEQ_CFG0_SPEC>;
#[doc = "Register `seq_cfg0` writer"]
pub type W = crate::W<SEQ_CFG0_SPEC>;
#[doc = "Field `HW_TRIG_EN` reader - set to enable external HW trigger, only trigger on posedge"]
pub type HW_TRIG_EN_R = crate::BitReader;
#[doc = "Field `HW_TRIG_EN` writer - set to enable external HW trigger, only trigger on posedge"]
pub type HW_TRIG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_TRIG_EN` reader - set to enable SW trigger"]
pub type SW_TRIG_EN_R = crate::BitReader;
#[doc = "Field `SW_TRIG_EN` writer - set to enable SW trigger"]
pub type SW_TRIG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_TRIG` writer - SW trigger, pulse signal, cleared by HW one cycle later"]
pub type SW_TRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT_EN` reader - if set, HW will continue process the queue till end(seq_len) after trigger once"]
pub type CONT_EN_R = crate::BitReader;
#[doc = "Field `CONT_EN` writer - if set, HW will continue process the queue till end(seq_len) after trigger once"]
pub type CONT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTART_EN` reader - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
pub type RESTART_EN_R = crate::BitReader;
#[doc = "Field `RESTART_EN` writer - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
pub type RESTART_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_LEN` reader - sequence queue length, 0 for one, 0xF for 16"]
pub type SEQ_LEN_R = crate::FieldReader;
#[doc = "Field `SEQ_LEN` writer - sequence queue length, 0 for one, 0xF for 16"]
pub type SEQ_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CYCLE` reader - current dma write cycle bit"]
pub type CYCLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - set to enable external HW trigger, only trigger on posedge"]
    #[inline(always)]
    pub fn hw_trig_en(&self) -> HW_TRIG_EN_R {
        HW_TRIG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - set to enable SW trigger"]
    #[inline(always)]
    pub fn sw_trig_en(&self) -> SW_TRIG_EN_R {
        SW_TRIG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once"]
    #[inline(always)]
    pub fn cont_en(&self) -> CONT_EN_R {
        CONT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
    #[inline(always)]
    pub fn restart_en(&self) -> RESTART_EN_R {
        RESTART_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - sequence queue length, 0 for one, 0xF for 16"]
    #[inline(always)]
    pub fn seq_len(&self) -> SEQ_LEN_R {
        SEQ_LEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - current dma write cycle bit"]
    #[inline(always)]
    pub fn cycle(&self) -> CYCLE_R {
        CYCLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - set to enable external HW trigger, only trigger on posedge"]
    #[inline(always)]
    #[must_use]
    pub fn hw_trig_en(&mut self) -> HW_TRIG_EN_W<SEQ_CFG0_SPEC> {
        HW_TRIG_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - set to enable SW trigger"]
    #[inline(always)]
    #[must_use]
    pub fn sw_trig_en(&mut self) -> SW_TRIG_EN_W<SEQ_CFG0_SPEC> {
        SW_TRIG_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SW trigger, pulse signal, cleared by HW one cycle later"]
    #[inline(always)]
    #[must_use]
    pub fn sw_trig(&mut self) -> SW_TRIG_W<SEQ_CFG0_SPEC> {
        SW_TRIG_W::new(self, 2)
    }
    #[doc = "Bit 3 - if set, HW will continue process the queue till end(seq_len) after trigger once"]
    #[inline(always)]
    #[must_use]
    pub fn cont_en(&mut self) -> CONT_EN_W<SEQ_CFG0_SPEC> {
        CONT_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - if set together with cont_en, HW will continue process the whole queue after trigger once. If cont_en is 0, this bit is not used"]
    #[inline(always)]
    #[must_use]
    pub fn restart_en(&mut self) -> RESTART_EN_W<SEQ_CFG0_SPEC> {
        RESTART_EN_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - sequence queue length, 0 for one, 0xF for 16"]
    #[inline(always)]
    #[must_use]
    pub fn seq_len(&mut self) -> SEQ_LEN_W<SEQ_CFG0_SPEC> {
        SEQ_LEN_W::new(self, 8)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_cfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_cfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEQ_CFG0_SPEC;
impl crate::RegisterSpec for SEQ_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_cfg0::R`](R) reader structure"]
impl crate::Readable for SEQ_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`seq_cfg0::W`](W) writer structure"]
impl crate::Writable for SEQ_CFG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets seq_cfg0 to value 0"]
impl crate::Resettable for SEQ_CFG0_SPEC {
    const RESET_VALUE: u32 = 0;
}
