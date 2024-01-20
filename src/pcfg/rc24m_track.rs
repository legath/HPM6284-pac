#[doc = "Register `RC24M_TRACK` reader"]
pub type R = crate::R<RC24M_TRACK_SPEC>;
#[doc = "Register `RC24M_TRACK` writer"]
pub type W = crate::W<RC24M_TRACK_SPEC>;
#[doc = "Field `TRACK` reader - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
pub type TRACK_R = crate::BitReader;
#[doc = "Field `TRACK` writer - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
pub type TRACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETURN` reader - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
pub type RETURN_R = crate::BitReader;
#[doc = "Field `RETURN` writer - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
pub type RETURN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL24M` reader - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
pub type SEL24M_R = crate::BitReader;
#[doc = "Field `SEL24M` writer - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
pub type SEL24M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
    #[inline(always)]
    pub fn track(&self) -> TRACK_R {
        TRACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
    #[inline(always)]
    pub fn return_(&self) -> RETURN_R {
        RETURN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
    #[inline(always)]
    pub fn sel24m(&self) -> SEL24M_R {
        SEL24M_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - track mode 0: RC24M free running 1: track RC24M to external XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn track(&mut self) -> TRACK_W<RC24M_TRACK_SPEC> {
        TRACK_W::new(self, 0)
    }
    #[doc = "Bit 4 - Retrun default value when XTAL loss 0: remain last tracking value 1: switch to default value"]
    #[inline(always)]
    #[must_use]
    pub fn return_(&mut self) -> RETURN_W<RC24M_TRACK_SPEC> {
        RETURN_W::new(self, 4)
    }
    #[doc = "Bit 16 - Select track reference 0: select 32K as reference 1: select 24M XTAL as reference"]
    #[inline(always)]
    #[must_use]
    pub fn sel24m(&mut self) -> SEL24M_W<RC24M_TRACK_SPEC> {
        SEL24M_W::new(self, 16)
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
#[doc = "RC 24M track mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rc24m_track::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rc24m_track::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RC24M_TRACK_SPEC;
impl crate::RegisterSpec for RC24M_TRACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rc24m_track::R`](R) reader structure"]
impl crate::Readable for RC24M_TRACK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rc24m_track::W`](W) writer structure"]
impl crate::Writable for RC24M_TRACK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RC24M_TRACK to value 0"]
impl crate::Resettable for RC24M_TRACK_SPEC {
    const RESET_VALUE: u32 = 0;
}
