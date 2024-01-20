#[doc = "Register `CREL` reader"]
pub type R = crate::R<CREL_SPEC>;
#[doc = "Register `CREL` writer"]
pub type W = crate::W<CREL_SPEC>;
#[doc = "Field `DAY` reader - Timestamp Day Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
pub type DAY_R = crate::FieldReader;
#[doc = "Field `MON` reader - Timestamp Month Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
pub type MON_R = crate::FieldReader;
#[doc = "Field `YEAR` reader - Timestamp Year One digit, BCD-coded. This field is set by generic parameter on synthesis."]
pub type YEAR_R = crate::FieldReader;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release One digit, BCD-coded"]
pub type SUBSTEP_R = crate::FieldReader;
#[doc = "Field `STEP` reader - Step of Core Release One digit, BCD-coded."]
pub type STEP_R = crate::FieldReader;
#[doc = "Field `REL` reader - Core Release One digit, BCD-coded"]
pub type REL_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Timestamp Day Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timestamp Month Two digits, BCD-coded. This field is set by generic parameter on synthesis."]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Year One digit, BCD-coded. This field is set by generic parameter on synthesis."]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release One digit, BCD-coded"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release One digit, BCD-coded."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release One digit, BCD-coded"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
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
#[doc = "core release register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CREL_SPEC;
impl crate::RegisterSpec for CREL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CREL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crel::W`](W) writer structure"]
impl crate::Writable for CREL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CREL_SPEC {
    const RESET_VALUE: u32 = 0;
}
