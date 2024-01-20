#[doc = "Register `trg_sw_sta` reader"]
pub type R = crate::R<TRG_SW_STA_SPEC>;
#[doc = "Register `trg_sw_sta` writer"]
pub type W = crate::W<TRG_SW_STA_SPEC>;
#[doc = "Field `TRIG_SW_INDEX` reader - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c"]
pub type TRIG_SW_INDEX_R = crate::FieldReader;
#[doc = "Field `TRIG_SW_INDEX` writer - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c"]
pub type TRIG_SW_INDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRG_SW_STA` reader - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
pub type TRG_SW_STA_R = crate::BitReader;
#[doc = "Field `TRG_SW_STA` writer - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
pub type TRG_SW_STA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c"]
    #[inline(always)]
    pub fn trig_sw_index(&self) -> TRIG_SW_INDEX_R {
        TRIG_SW_INDEX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
    #[inline(always)]
    pub fn trg_sw_sta(&self) -> TRG_SW_STA_R {
        TRG_SW_STA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - which trigger for the SW trigger 0 for trig0a, 1 for trig0b… 3 for trig1a, …11 for trig3c"]
    #[inline(always)]
    #[must_use]
    pub fn trig_sw_index(&mut self) -> TRIG_SW_INDEX_W<TRG_SW_STA_SPEC> {
        TRIG_SW_INDEX_W::new(self, 0)
    }
    #[doc = "Bit 4 - SW trigger start bit, HW will clear it after all conversions(up to 4) finished. SW should make sure it's 0 before set it."]
    #[inline(always)]
    #[must_use]
    pub fn trg_sw_sta(&mut self) -> TRG_SW_STA_W<TRG_SW_STA_SPEC> {
        TRG_SW_STA_W::new(self, 4)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg_sw_sta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg_sw_sta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRG_SW_STA_SPEC;
impl crate::RegisterSpec for TRG_SW_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trg_sw_sta::R`](R) reader structure"]
impl crate::Readable for TRG_SW_STA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trg_sw_sta::W`](W) writer structure"]
impl crate::Writable for TRG_SW_STA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets trg_sw_sta to value 0"]
impl crate::Resettable for TRG_SW_STA_SPEC {
    const RESET_VALUE: u32 = 0;
}
