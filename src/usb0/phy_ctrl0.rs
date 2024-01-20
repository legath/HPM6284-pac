#[doc = "Register `PHY_CTRL0` reader"]
pub type R = crate::R<PHY_CTRL0_SPEC>;
#[doc = "Register `PHY_CTRL0` writer"]
pub type W = crate::W<PHY_CTRL0_SPEC>;
#[doc = "Field `VBUS_VALID_OVERRIDE_EN` reader - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `VBUS_VALID_OVERRIDE_EN` writer - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESS_VALID_OVERRIDE_EN` reader - No description avaiable"]
pub type SESS_VALID_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `SESS_VALID_OVERRIDE_EN` writer - No description avaiable"]
pub type SESS_VALID_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_DIG_OVERRIDE_EN` reader - No description avaiable"]
pub type ID_DIG_OVERRIDE_EN_R = crate::BitReader;
#[doc = "Field `ID_DIG_OVERRIDE_EN` writer - No description avaiable"]
pub type ID_DIG_OVERRIDE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUS_VALID_OVERRIDE` reader - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_R = crate::BitReader;
#[doc = "Field `VBUS_VALID_OVERRIDE` writer - No description avaiable"]
pub type VBUS_VALID_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESS_VALID_OVERRIDE` reader - No description avaiable"]
pub type SESS_VALID_OVERRIDE_R = crate::BitReader;
#[doc = "Field `SESS_VALID_OVERRIDE` writer - No description avaiable"]
pub type SESS_VALID_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_DIG_OVERRIDE` reader - No description avaiable"]
pub type ID_DIG_OVERRIDE_R = crate::BitReader;
#[doc = "Field `ID_DIG_OVERRIDE` writer - No description avaiable"]
pub type ID_DIG_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_ID_SEL_N` reader - No description avaiable"]
pub type GPIO_ID_SEL_N_R = crate::BitReader;
#[doc = "Field `GPIO_ID_SEL_N` writer - No description avaiable"]
pub type GPIO_ID_SEL_N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override_en(&self) -> VBUS_VALID_OVERRIDE_EN_R {
        VBUS_VALID_OVERRIDE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override_en(&self) -> SESS_VALID_OVERRIDE_EN_R {
        SESS_VALID_OVERRIDE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override_en(&self) -> ID_DIG_OVERRIDE_EN_R {
        ID_DIG_OVERRIDE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid_override(&self) -> VBUS_VALID_OVERRIDE_R {
        VBUS_VALID_OVERRIDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    pub fn sess_valid_override(&self) -> SESS_VALID_OVERRIDE_R {
        SESS_VALID_OVERRIDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig_override(&self) -> ID_DIG_OVERRIDE_R {
        ID_DIG_OVERRIDE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    pub fn gpio_id_sel_n(&self) -> GPIO_ID_SEL_N_R {
        GPIO_ID_SEL_N_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_valid_override_en(&mut self) -> VBUS_VALID_OVERRIDE_EN_W<PHY_CTRL0_SPEC> {
        VBUS_VALID_OVERRIDE_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn sess_valid_override_en(&mut self) -> SESS_VALID_OVERRIDE_EN_W<PHY_CTRL0_SPEC> {
        SESS_VALID_OVERRIDE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn id_dig_override_en(&mut self) -> ID_DIG_OVERRIDE_EN_W<PHY_CTRL0_SPEC> {
        ID_DIG_OVERRIDE_EN_W::new(self, 2)
    }
    #[doc = "Bit 12 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_valid_override(&mut self) -> VBUS_VALID_OVERRIDE_W<PHY_CTRL0_SPEC> {
        VBUS_VALID_OVERRIDE_W::new(self, 12)
    }
    #[doc = "Bit 13 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn sess_valid_override(&mut self) -> SESS_VALID_OVERRIDE_W<PHY_CTRL0_SPEC> {
        SESS_VALID_OVERRIDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn id_dig_override(&mut self) -> ID_DIG_OVERRIDE_W<PHY_CTRL0_SPEC> {
        ID_DIG_OVERRIDE_W::new(self, 14)
    }
    #[doc = "Bit 25 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_id_sel_n(&mut self) -> GPIO_ID_SEL_N_W<PHY_CTRL0_SPEC> {
        GPIO_ID_SEL_N_W::new(self, 25)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_CTRL0_SPEC;
impl crate::RegisterSpec for PHY_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_ctrl0::R`](R) reader structure"]
impl crate::Readable for PHY_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_ctrl0::W`](W) writer structure"]
impl crate::Writable for PHY_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_CTRL0 to value 0"]
impl crate::Resettable for PHY_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
