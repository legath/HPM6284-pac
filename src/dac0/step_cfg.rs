#[doc = "Register `STEP_CFG[%s]` reader"]
pub type R = crate::R<STEP_CFG_SPEC>;
#[doc = "Register `STEP_CFG[%s]` writer"]
pub type W = crate::W<STEP_CFG_SPEC>;
#[doc = "Field `START_POINT` reader - No description avaiable"]
pub type START_POINT_R = crate::FieldReader<u16>;
#[doc = "Field `START_POINT` writer - No description avaiable"]
pub type START_POINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `STEP_NUM` reader - output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point"]
pub type STEP_NUM_R = crate::FieldReader;
#[doc = "Field `STEP_NUM` writer - output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point"]
pub type STEP_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `END_POINT` reader - No description avaiable"]
pub type END_POINT_R = crate::FieldReader<u16>;
#[doc = "Field `END_POINT` writer - No description avaiable"]
pub type END_POINT_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UP_DOWN` reader - 0 for up, 1 for down"]
pub type UP_DOWN_R = crate::BitReader;
#[doc = "Field `UP_DOWN` writer - 0 for up, 1 for down"]
pub type UP_DOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROUND_MODE` reader - 0: stop at end point; 1: reload start point, step again"]
pub type ROUND_MODE_R = crate::BitReader;
#[doc = "Field `ROUND_MODE` writer - 0: stop at end point; 1: reload start point, step again"]
pub type ROUND_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - No description avaiable"]
    #[inline(always)]
    pub fn start_point(&self) -> START_POINT_R {
        START_POINT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point"]
    #[inline(always)]
    pub fn step_num(&self) -> STEP_NUM_R {
        STEP_NUM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - No description avaiable"]
    #[inline(always)]
    pub fn end_point(&self) -> END_POINT_R {
        END_POINT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - 0 for up, 1 for down"]
    #[inline(always)]
    pub fn up_down(&self) -> UP_DOWN_R {
        UP_DOWN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: stop at end point; 1: reload start point, step again"]
    #[inline(always)]
    pub fn round_mode(&self) -> ROUND_MODE_R {
        ROUND_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn start_point(&mut self) -> START_POINT_W<STEP_CFG_SPEC> {
        START_POINT_W::new(self, 0)
    }
    #[doc = "Bits 12:15 - output data change step_num each DAC clock cycle. Ex: if step_num=3, output data sequence is 0,3,6,9 NOTE: user should make sure end_point can be reached if step_num is not 1 if step_num is 0, output data will always at start point"]
    #[inline(always)]
    #[must_use]
    pub fn step_num(&mut self) -> STEP_NUM_W<STEP_CFG_SPEC> {
        STEP_NUM_W::new(self, 12)
    }
    #[doc = "Bits 16:27 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn end_point(&mut self) -> END_POINT_W<STEP_CFG_SPEC> {
        END_POINT_W::new(self, 16)
    }
    #[doc = "Bit 28 - 0 for up, 1 for down"]
    #[inline(always)]
    #[must_use]
    pub fn up_down(&mut self) -> UP_DOWN_W<STEP_CFG_SPEC> {
        UP_DOWN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 0: stop at end point; 1: reload start point, step again"]
    #[inline(always)]
    #[must_use]
    pub fn round_mode(&mut self) -> ROUND_MODE_W<STEP_CFG_SPEC> {
        ROUND_MODE_W::new(self, 29)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`step_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`step_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STEP_CFG_SPEC;
impl crate::RegisterSpec for STEP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`step_cfg::R`](R) reader structure"]
impl crate::Readable for STEP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`step_cfg::W`](W) writer structure"]
impl crate::Writable for STEP_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STEP_CFG[%s]
to value 0"]
impl crate::Resettable for STEP_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
