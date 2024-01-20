#[doc = "Register `FILTER_1ST_PLA_IN[%s]` reader"]
pub type R = crate::R<FILTER_1ST_PLA_IN_SPEC>;
#[doc = "Register `FILTER_1ST_PLA_IN[%s]` writer"]
pub type W = crate::W<FILTER_1ST_PLA_IN_SPEC>;
#[doc = "Field `SYNC_EDGE_FILTER_ENABLE` reader - sync and edge detector filter. 0: disable. 1: enable."]
pub type SYNC_EDGE_FILTER_ENABLE_R = crate::BitReader;
#[doc = "Field `SYNC_EDGE_FILTER_ENABLE` writer - sync and edge detector filter. 0: disable. 1: enable."]
pub type SYNC_EDGE_FILTER_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTWARE_INJECT` reader - software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
pub type SOFTWARE_INJECT_R = crate::FieldReader;
#[doc = "Field `SOFTWARE_INJECT` writer - software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
pub type SOFTWARE_INJECT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FILTER_REVERSE` reader - reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
pub type FILTER_REVERSE_R = crate::BitReader;
#[doc = "Field `FILTER_REVERSE` writer - reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
pub type FILTER_REVERSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE_DECT_ENABLE` reader - edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
pub type EDGE_DECT_ENABLE_R = crate::BitReader;
#[doc = "Field `EDGE_DECT_ENABLE` writer - edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
pub type EDGE_DECT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGE_EDGE_DECT_ENABLE` reader - nege edge detector enable. 0: disable. 1: enable."]
pub type NEGE_EDGE_DECT_ENABLE_R = crate::BitReader;
#[doc = "Field `NEGE_EDGE_DECT_ENABLE` writer - nege edge detector enable. 0: disable. 1: enable."]
pub type NEGE_EDGE_DECT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSE_EDGE_DECT_ENABLE` reader - pose edge detector enable. 0: disable. 1: enable."]
pub type POSE_EDGE_DECT_ENABLE_R = crate::BitReader;
#[doc = "Field `POSE_EDGE_DECT_ENABLE` writer - pose edge detector enable. 0: disable. 1: enable."]
pub type POSE_EDGE_DECT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER_SYNC_LEVEL` reader - synchroniser level. 0: 2 level sync. 1: 3 level sync"]
pub type FILTER_SYNC_LEVEL_R = crate::BitReader;
#[doc = "Field `FILTER_SYNC_LEVEL` writer - synchroniser level. 0: 2 level sync. 1: 3 level sync"]
pub type FILTER_SYNC_LEVEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER_EXT_ENABLE` reader - filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
pub type FILTER_EXT_ENABLE_R = crate::BitReader;
#[doc = "Field `FILTER_EXT_ENABLE` writer - filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
pub type FILTER_EXT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FILTER_EXT_TYPE` reader - filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
pub type FILTER_EXT_TYPE_R = crate::FieldReader;
#[doc = "Field `FILTER_EXT_TYPE` writer - filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
pub type FILTER_EXT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER_EXT_COUNTER` reader - filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period"]
pub type FILTER_EXT_COUNTER_R = crate::FieldReader<u16>;
#[doc = "Field `FILTER_EXT_COUNTER` writer - filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period"]
pub type FILTER_EXT_COUNTER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - sync and edge detector filter. 0: disable. 1: enable."]
    #[inline(always)]
    pub fn sync_edge_filter_enable(&self) -> SYNC_EDGE_FILTER_ENABLE_R {
        SYNC_EDGE_FILTER_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
    #[inline(always)]
    pub fn software_inject(&self) -> SOFTWARE_INJECT_R {
        SOFTWARE_INJECT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
    #[inline(always)]
    pub fn filter_reverse(&self) -> FILTER_REVERSE_R {
        FILTER_REVERSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
    #[inline(always)]
    pub fn edge_dect_enable(&self) -> EDGE_DECT_ENABLE_R {
        EDGE_DECT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - nege edge detector enable. 0: disable. 1: enable."]
    #[inline(always)]
    pub fn nege_edge_dect_enable(&self) -> NEGE_EDGE_DECT_ENABLE_R {
        NEGE_EDGE_DECT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pose edge detector enable. 0: disable. 1: enable."]
    #[inline(always)]
    pub fn pose_edge_dect_enable(&self) -> POSE_EDGE_DECT_ENABLE_R {
        POSE_EDGE_DECT_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - synchroniser level. 0: 2 level sync. 1: 3 level sync"]
    #[inline(always)]
    pub fn filter_sync_level(&self) -> FILTER_SYNC_LEVEL_R {
        FILTER_SYNC_LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
    #[inline(always)]
    pub fn filter_ext_enable(&self) -> FILTER_EXT_ENABLE_R {
        FILTER_EXT_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:14 - filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
    #[inline(always)]
    pub fn filter_ext_type(&self) -> FILTER_EXT_TYPE_R {
        FILTER_EXT_TYPE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period"]
    #[inline(always)]
    pub fn filter_ext_counter(&self) -> FILTER_EXT_COUNTER_R {
        FILTER_EXT_COUNTER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - sync and edge detector filter. 0: disable. 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn sync_edge_filter_enable(&mut self) -> SYNC_EDGE_FILTER_ENABLE_W<FILTER_1ST_PLA_IN_SPEC> {
        SYNC_EDGE_FILTER_ENABLE_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - software inject value for sync and edge detector filter. 0: inject low level. 1: inject high level. 2: not inject. 3. inject high level."]
    #[inline(always)]
    #[must_use]
    pub fn software_inject(&mut self) -> SOFTWARE_INJECT_W<FILTER_1ST_PLA_IN_SPEC> {
        SOFTWARE_INJECT_W::new(self, 1)
    }
    #[doc = "Bit 3 - reverse sync and edge detector filter's output. 0: not reverse. 1: reverse."]
    #[inline(always)]
    #[must_use]
    pub fn filter_reverse(&mut self) -> FILTER_REVERSE_W<FILTER_1ST_PLA_IN_SPEC> {
        FILTER_REVERSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - edge detector enable. 0: disable. bit6/bit5 setting inactive. 1: enable. bit6/bit5 setting active."]
    #[inline(always)]
    #[must_use]
    pub fn edge_dect_enable(&mut self) -> EDGE_DECT_ENABLE_W<FILTER_1ST_PLA_IN_SPEC> {
        EDGE_DECT_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - nege edge detector enable. 0: disable. 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn nege_edge_dect_enable(&mut self) -> NEGE_EDGE_DECT_ENABLE_W<FILTER_1ST_PLA_IN_SPEC> {
        NEGE_EDGE_DECT_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - pose edge detector enable. 0: disable. 1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn pose_edge_dect_enable(&mut self) -> POSE_EDGE_DECT_ENABLE_W<FILTER_1ST_PLA_IN_SPEC> {
        POSE_EDGE_DECT_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - synchroniser level. 0: 2 level sync. 1: 3 level sync"]
    #[inline(always)]
    #[must_use]
    pub fn filter_sync_level(&mut self) -> FILTER_SYNC_LEVEL_W<FILTER_1ST_PLA_IN_SPEC> {
        FILTER_SYNC_LEVEL_W::new(self, 7)
    }
    #[doc = "Bit 8 - filter extend enable. 0. bypass filter extend. all setting in bit31:12 are inactive 1. enable filter extend, all setting in bit31:12 are active."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ext_enable(&mut self) -> FILTER_EXT_ENABLE_W<FILTER_1ST_PLA_IN_SPEC> {
        FILTER_EXT_ENABLE_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - filter extend type. 0-3：nothing to do. 4： input high level extend. 5： input low level extend. 6： output extend. 7： input pulse extend."]
    #[inline(always)]
    #[must_use]
    pub fn filter_ext_type(&mut self) -> FILTER_EXT_TYPE_W<FILTER_1ST_PLA_IN_SPEC> {
        FILTER_EXT_TYPE_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - filter_ext counter value, cycles for filter or extent by system clock。 0：0*apb_clk_period 1：1*apb_clk_period 2: 2*apb_clk_period … 65535: 65535*apb_clk_period"]
    #[inline(always)]
    #[must_use]
    pub fn filter_ext_counter(&mut self) -> FILTER_EXT_COUNTER_W<FILTER_1ST_PLA_IN_SPEC> {
        FILTER_EXT_COUNTER_W::new(self, 16)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_1st_pla_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_1st_pla_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_1ST_PLA_IN_SPEC;
impl crate::RegisterSpec for FILTER_1ST_PLA_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_1st_pla_in::R`](R) reader structure"]
impl crate::Readable for FILTER_1ST_PLA_IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_1st_pla_in::W`](W) writer structure"]
impl crate::Writable for FILTER_1ST_PLA_IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_1ST_PLA_IN[%s]
to value 0"]
impl crate::Resettable for FILTER_1ST_PLA_IN_SPEC {
    const RESET_VALUE: u32 = 0;
}
