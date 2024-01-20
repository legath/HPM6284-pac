#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `ENABLE` reader - enable tamper 0: tamper disableed 1: tamper enabled"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - enable tamper 0: tamper disableed 1: tamper enabled"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE` reader - select active or passive tamper 0: passive tamper 1: active tamper"]
pub type ACTIVE_R = crate::BitReader;
#[doc = "Field `ACTIVE` writer - select active or passive tamper 0: passive tamper 1: active tamper"]
pub type ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECOVER` reader - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
pub type RECOVER_R = crate::BitReader;
#[doc = "Field `RECOVER` writer - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
pub type RECOVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPEED` reader - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
pub type SPEED_R = crate::FieldReader;
#[doc = "Field `SPEED` writer - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
pub type SPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VALUE` reader - pin value for passive tamper"]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - pin value for passive tamper"]
pub type VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FILTER` reader - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
pub type FILTER_R = crate::FieldReader;
#[doc = "Field `FILTER` writer - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
pub type FILTER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYPASS` reader - bypass tamper violation filter 0: filter applied 1: filter not used"]
pub type BYPASS_R = crate::BitReader;
#[doc = "Field `BYPASS` writer - bypass tamper violation filter 0: filter applied 1: filter not used"]
pub type BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable tamper 0: tamper disableed 1: tamper enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - select active or passive tamper 0: passive tamper 1: active tamper"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    #[inline(always)]
    pub fn recover(&self) -> RECOVER_R {
        RECOVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - pin value for passive tamper"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - bypass tamper violation filter 0: filter applied 1: filter not used"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable tamper 0: tamper disableed 1: tamper enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CONTROL_SPEC> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - select active or passive tamper 0: passive tamper 1: active tamper"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<CONTROL_SPEC> {
        ACTIVE_W::new(self, 1)
    }
    #[doc = "Bit 2 - tamper will recover itself if tamper LFSR goes wrong 0: tamper will not recover 1: tamper will recover"]
    #[inline(always)]
    #[must_use]
    pub fn recover(&mut self) -> RECOVER_W<CONTROL_SPEC> {
        RECOVER_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - tamper speed selection, (2^SPEED) changes per second 0: 1 shift per second 1: 2 shifts per second . . . 15: 32768 shifts per second"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<CONTROL_SPEC> {
        SPEED_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - pin value for passive tamper"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<CONTROL_SPEC> {
        VALUE_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - filter length 0: 1 cycle 1: 2 cycle 15: 65526 cycle"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<CONTROL_SPEC> {
        FILTER_W::new(self, 16)
    }
    #[doc = "Bit 20 - bypass tamper violation filter 0: filter applied 1: filter not used"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<CONTROL_SPEC> {
        BYPASS_W::new(self, 20)
    }
    #[doc = "Bit 31 - lock tamper setting 0: tamper setting can be changed 1: tamper setting will last to next battery domain power cycle"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<CONTROL_SPEC> {
        LOCK_W::new(self, 31)
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
#[doc = "Tamper n control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
