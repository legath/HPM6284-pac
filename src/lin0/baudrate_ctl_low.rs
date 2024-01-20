#[doc = "Register `baudrate_ctl_low` reader"]
pub type R = crate::R<BAUDRATE_CTL_LOW_SPEC>;
#[doc = "Register `baudrate_ctl_low` writer"]
pub type W = crate::W<BAUDRATE_CTL_LOW_SPEC>;
#[doc = "Field `BT_DIV_LOW` reader - bit div register 7:0"]
pub type BT_DIV_LOW_R = crate::FieldReader;
#[doc = "Field `BT_DIV_LOW` writer - bit div register 7:0"]
pub type BT_DIV_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - bit div register 7:0"]
    #[inline(always)]
    pub fn bt_div_low(&self) -> BT_DIV_LOW_R {
        BT_DIV_LOW_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - bit div register 7:0"]
    #[inline(always)]
    #[must_use]
    pub fn bt_div_low(&mut self) -> BT_DIV_LOW_W<BAUDRATE_CTL_LOW_SPEC> {
        BT_DIV_LOW_W::new(self, 0)
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
#[doc = "baudrate control low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`baudrate_ctl_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudrate_ctl_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUDRATE_CTL_LOW_SPEC;
impl crate::RegisterSpec for BAUDRATE_CTL_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudrate_ctl_low::R`](R) reader structure"]
impl crate::Readable for BAUDRATE_CTL_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baudrate_ctl_low::W`](W) writer structure"]
impl crate::Writable for BAUDRATE_CTL_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets baudrate_ctl_low to value 0"]
impl crate::Resettable for BAUDRATE_CTL_LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
