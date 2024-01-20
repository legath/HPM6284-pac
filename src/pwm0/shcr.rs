#[doc = "Register `shcr` reader"]
pub type R = crate::R<SHCR_SPEC>;
#[doc = "Register `shcr` writer"]
pub type W = crate::W<SHCR_SPEC>;
#[doc = "Field `SHLKEN` reader - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
pub type SHLKEN_R = crate::BitReader;
#[doc = "Field `SHLKEN` writer - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
pub type SHLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSHDWUPT` reader - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CNTSHDWUPT_R = crate::FieldReader;
#[doc = "Field `CNTSHDWUPT` writer - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CNTSHDWUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNTSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
pub type CNTSHDWSEL_R = crate::FieldReader;
#[doc = "Field `CNTSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
pub type CNTSHDWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FRCSHDWSEL` reader - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
pub type FRCSHDWSEL_R = crate::FieldReader;
#[doc = "Field `FRCSHDWSEL` writer - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
pub type FRCSHDWSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FORCE_UPDATE_EDGE` reader - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers"]
pub type FORCE_UPDATE_EDGE_R = crate::BitReader;
#[doc = "Field `FORCE_UPDATE_EDGE` writer - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers"]
pub type FORCE_UPDATE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_UPDATE_EDGE` reader - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers"]
pub type CNT_UPDATE_EDGE_R = crate::BitReader;
#[doc = "Field `CNT_UPDATE_EDGE` writer - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers"]
pub type CNT_UPDATE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_UPDATE_RELOAD` reader - set to update counter working register at reload point, clear to use cnt_update_time as old version."]
pub type CNT_UPDATE_RELOAD_R = crate::BitReader;
#[doc = "Field `CNT_UPDATE_RELOAD` writer - set to update counter working register at reload point, clear to use cnt_update_time as old version."]
pub type CNT_UPDATE_RELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
    #[inline(always)]
    pub fn shlken(&self) -> SHLKEN_R {
        SHLKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn cntshdwupt(&self) -> CNTSHDWUPT_R {
        CNTSHDWUPT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:7 - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
    #[inline(always)]
    pub fn cntshdwsel(&self) -> CNTSHDWSEL_R {
        CNTSHDWSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
    #[inline(always)]
    pub fn frcshdwsel(&self) -> FRCSHDWSEL_R {
        FRCSHDWSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers"]
    #[inline(always)]
    pub fn force_update_edge(&self) -> FORCE_UPDATE_EDGE_R {
        FORCE_UPDATE_EDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers"]
    #[inline(always)]
    pub fn cnt_update_edge(&self) -> CNT_UPDATE_EDGE_R {
        CNT_UPDATE_EDGE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - set to update counter working register at reload point, clear to use cnt_update_time as old version."]
    #[inline(always)]
    pub fn cnt_update_reload(&self) -> CNT_UPDATE_RELOAD_R {
        CNT_UPDATE_RELOAD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1- enable shadow registers lock feature, 0- disable shadow registers lock, shlk bit will always be 0"]
    #[inline(always)]
    #[must_use]
    pub fn shlken(&mut self) -> SHLKEN_W<SHCR_SPEC> {
        SHLKEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - This bitfield select when the counter related shadow registers (STA and RLD) will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn cntshdwupt(&mut self) -> CNTSHDWUPT_W<SHCR_SPEC> {
        CNTSHDWUPT_W::new(self, 1)
    }
    #[doc = "Bits 3:7 - This bitfield select one of the comparators as hardware event time to load the counter related shadow registers (STA and RLD)"]
    #[inline(always)]
    #[must_use]
    pub fn cntshdwsel(&mut self) -> CNTSHDWSEL_W<SHCR_SPEC> {
        CNTSHDWSEL_W::new(self, 3)
    }
    #[doc = "Bits 8:12 - This bitfield select one of the comparators as hardware event time to load FRCMD shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn frcshdwsel(&mut self) -> FRCSHDWSEL_W<SHCR_SPEC> {
        FRCSHDWSEL_W::new(self, 8)
    }
    #[doc = "Bit 13 - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for FRCMD shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn force_update_edge(&mut self) -> FORCE_UPDATE_EDGE_W<SHCR_SPEC> {
        FORCE_UPDATE_EDGE_W::new(self, 13)
    }
    #[doc = "Bit 14 - 0 for posedge; 1 for negedge if hardware trigger time is selected for update_time, and selected channel is capture mode, for counter shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_update_edge(&mut self) -> CNT_UPDATE_EDGE_W<SHCR_SPEC> {
        CNT_UPDATE_EDGE_W::new(self, 14)
    }
    #[doc = "Bit 15 - set to update counter working register at reload point, clear to use cnt_update_time as old version."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_update_reload(&mut self) -> CNT_UPDATE_RELOAD_W<SHCR_SPEC> {
        CNT_UPDATE_RELOAD_W::new(self, 15)
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
#[doc = "Shadow register control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHCR_SPEC;
impl crate::RegisterSpec for SHCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcr::R`](R) reader structure"]
impl crate::Readable for SHCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`shcr::W`](W) writer structure"]
impl crate::Writable for SHCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets shcr to value 0"]
impl crate::Resettable for SHCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
