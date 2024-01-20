#[doc = "Register `PWMCFG[%s]` reader"]
pub type R = crate::R<PWMCFG_SPEC>;
#[doc = "Register `PWMCFG[%s]` writer"]
pub type W = crate::W<PWMCFG_SPEC>;
#[doc = "Field `DEADAREA` reader - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
pub type DEADAREA_R = crate::FieldReader<u32>;
#[doc = "Field `DEADAREA` writer - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
pub type DEADAREA_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PAIR` reader - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
pub type PAIR_R = crate::BitReader;
#[doc = "Field `PAIR` writer - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
pub type PAIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCSRCSEL` reader - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
pub type FRCSRCSEL_R = crate::BitReader;
#[doc = "Field `FRCSRCSEL` writer - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
pub type FRCSRCSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULTRECTIME` reader - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
pub type FAULTRECTIME_R = crate::FieldReader;
#[doc = "Field `FAULTRECTIME` writer - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
pub type FAULTRECTIME_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FAULTMODE` reader - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
pub type FAULTMODE_R = crate::FieldReader;
#[doc = "Field `FAULTMODE` writer - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
pub type FAULTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FRCSHDWUPT` reader - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type FRCSHDWUPT_R = crate::FieldReader;
#[doc = "Field `FRCSHDWUPT` writer - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type FRCSHDWUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OEN` reader - PWM output enable 1- output is enabled 0- output is disabled"]
pub type OEN_R = crate::BitReader;
#[doc = "Field `OEN` writer - PWM output enable 1- output is enabled 0- output is disabled"]
pub type OEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HR_UPDATE_MODE` reader - 0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used"]
pub type HR_UPDATE_MODE_R = crate::BitReader;
#[doc = "Field `HR_UPDATE_MODE` writer - 0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used"]
pub type HR_UPDATE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    #[inline(always)]
    pub fn deadarea(&self) -> DEADAREA_R {
        DEADAREA_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    #[inline(always)]
    pub fn pair(&self) -> PAIR_R {
        PAIR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    #[inline(always)]
    pub fn frcsrcsel(&self) -> FRCSRCSEL_R {
        FRCSRCSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    #[inline(always)]
    pub fn faultrectime(&self) -> FAULTRECTIME_R {
        FAULTRECTIME_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    #[inline(always)]
    pub fn faultmode(&self) -> FAULTMODE_R {
        FAULTMODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn frcshdwupt(&self) -> FRCSHDWUPT_R {
        FRCSHDWUPT_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - PWM output enable 1- output is enabled 0- output is disabled"]
    #[inline(always)]
    pub fn oen(&self) -> OEN_R {
        OEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used"]
    #[inline(always)]
    pub fn hr_update_mode(&self) -> HR_UPDATE_MODE_R {
        HR_UPDATE_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - This bitfield define the PWM pair deadarea length. The unit is 0.5 cycle. The minimum length of deadarea is 1 cycle. Note: user should configure pair bit and this bitfield before PWM output is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn deadarea(&mut self) -> DEADAREA_W<PWMCFG_SPEC> {
        DEADAREA_W::new(self, 0)
    }
    #[doc = "Bit 20 - 1- PWM output is in pair mode. Note the two PWM outputs need to be both set to pair mode. 0- PWM output is in indepandent mode."]
    #[inline(always)]
    #[must_use]
    pub fn pair(&mut self) -> PAIR_W<PWMCFG_SPEC> {
        PAIR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Select sources for force output 0- force output is enabled when FRCI assert 1- force output is enabled by software write swfrc to 1"]
    #[inline(always)]
    #[must_use]
    pub fn frcsrcsel(&mut self) -> FRCSRCSEL_W<PWMCFG_SPEC> {
        FRCSRCSEL_W::new(self, 21)
    }
    #[doc = "Bits 22:23 - This bitfield select when to recover PWM output after fault condition removed. 00: immediately 01: after pwm timer counter reload time 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after software write faultclr bit in GCR register"]
    #[inline(always)]
    #[must_use]
    pub fn faultrectime(&mut self) -> FAULTRECTIME_W<PWMCFG_SPEC> {
        FAULTRECTIME_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - This bitfield defines the PWM output status when fault condition happen 00: force output 0 01: force output 1 1x: output highz"]
    #[inline(always)]
    #[must_use]
    pub fn faultmode(&mut self) -> FAULTMODE_W<PWMCFG_SPEC> {
        FAULTMODE_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - This bitfield select when the FRCMD shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn frcshdwupt(&mut self) -> FRCSHDWUPT_W<PWMCFG_SPEC> {
        FRCSHDWUPT_W::new(self, 26)
    }
    #[doc = "Bit 28 - PWM output enable 1- output is enabled 0- output is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn oen(&mut self) -> OEN_W<PWMCFG_SPEC> {
        OEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - 0: update the hr value for the first edge at reload point; 1: update the hr value for the first edge at the last edge; all others will be updated at previous edge for pair mode, only pwm_cfg 0/2/4/6 are used"]
    #[inline(always)]
    #[must_use]
    pub fn hr_update_mode(&mut self) -> HR_UPDATE_MODE_W<PWMCFG_SPEC> {
        HR_UPDATE_MODE_W::new(self, 29)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWMCFG_SPEC;
impl crate::RegisterSpec for PWMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcfg::R`](R) reader structure"]
impl crate::Readable for PWMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwmcfg::W`](W) writer structure"]
impl crate::Writable for PWMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWMCFG[%s]
to value 0"]
impl crate::Resettable for PWMCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
