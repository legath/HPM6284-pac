#[doc = "Register `bardrate_ctl_high` reader"]
pub type R = crate::R<BARDRATE_CTL_HIGH_SPEC>;
#[doc = "Register `bardrate_ctl_high` writer"]
pub type W = crate::W<BARDRATE_CTL_HIGH_SPEC>;
#[doc = "Field `BT_DIV_HIGH` reader - bit div register 8"]
pub type BT_DIV_HIGH_R = crate::BitReader;
#[doc = "Field `BT_DIV_HIGH` writer - bit div register 8"]
pub type BT_DIV_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BT_MUL` reader - bt_mul register"]
pub type BT_MUL_R = crate::FieldReader;
#[doc = "Field `BT_MUL` writer - bt_mul register"]
pub type BT_MUL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRESCL` reader - prescl register"]
pub type PRESCL_R = crate::FieldReader;
#[doc = "Field `PRESCL` writer - prescl register"]
pub type PRESCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - bit div register 8"]
    #[inline(always)]
    pub fn bt_div_high(&self) -> BT_DIV_HIGH_R {
        BT_DIV_HIGH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - bt_mul register"]
    #[inline(always)]
    pub fn bt_mul(&self) -> BT_MUL_R {
        BT_MUL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - prescl register"]
    #[inline(always)]
    pub fn prescl(&self) -> PRESCL_R {
        PRESCL_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - bit div register 8"]
    #[inline(always)]
    #[must_use]
    pub fn bt_div_high(&mut self) -> BT_DIV_HIGH_W<BARDRATE_CTL_HIGH_SPEC> {
        BT_DIV_HIGH_W::new(self, 0)
    }
    #[doc = "Bits 1:5 - bt_mul register"]
    #[inline(always)]
    #[must_use]
    pub fn bt_mul(&mut self) -> BT_MUL_W<BARDRATE_CTL_HIGH_SPEC> {
        BT_MUL_W::new(self, 1)
    }
    #[doc = "Bits 6:7 - prescl register"]
    #[inline(always)]
    #[must_use]
    pub fn prescl(&mut self) -> PRESCL_W<BARDRATE_CTL_HIGH_SPEC> {
        PRESCL_W::new(self, 6)
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
#[doc = "baudrate control high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bardrate_ctl_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bardrate_ctl_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BARDRATE_CTL_HIGH_SPEC;
impl crate::RegisterSpec for BARDRATE_CTL_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bardrate_ctl_high::R`](R) reader structure"]
impl crate::Readable for BARDRATE_CTL_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bardrate_ctl_high::W`](W) writer structure"]
impl crate::Writable for BARDRATE_CTL_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bardrate_ctl_high to value 0"]
impl crate::Resettable for BARDRATE_CTL_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
