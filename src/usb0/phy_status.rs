#[doc = "Register `PHY_STATUS` reader"]
pub type R = crate::R<PHY_STATUS_SPEC>;
#[doc = "Register `PHY_STATUS` writer"]
pub type W = crate::W<PHY_STATUS_SPEC>;
#[doc = "Field `VBUS_VALID` reader - No description avaiable"]
pub type VBUS_VALID_R = crate::BitReader;
#[doc = "Field `VBUS_VALID` writer - No description avaiable"]
pub type VBUS_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UTMI_SESS_VALID` reader - No description avaiable"]
pub type UTMI_SESS_VALID_R = crate::BitReader;
#[doc = "Field `UTMI_SESS_VALID` writer - No description avaiable"]
pub type UTMI_SESS_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID_DIG` reader - No description avaiable"]
pub type ID_DIG_R = crate::BitReader;
#[doc = "Field `ID_DIG` writer - No description avaiable"]
pub type ID_DIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_DISCONNECT` reader - No description avaiable"]
pub type HOST_DISCONNECT_R = crate::BitReader;
#[doc = "Field `HOST_DISCONNECT` writer - No description avaiable"]
pub type HOST_DISCONNECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_STATE` reader - No description avaiable"]
pub type LINE_STATE_R = crate::FieldReader;
#[doc = "Field `LINE_STATE` writer - No description avaiable"]
pub type LINE_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTMI_CLK_VALID` reader - No description avaiable"]
pub type UTMI_CLK_VALID_R = crate::BitReader;
#[doc = "Field `UTMI_CLK_VALID` writer - No description avaiable"]
pub type UTMI_CLK_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VBUS_VALID_R {
        VBUS_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_sess_valid(&self) -> UTMI_SESS_VALID_R {
        UTMI_SESS_VALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    pub fn id_dig(&self) -> ID_DIG_R {
        ID_DIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    pub fn host_disconnect(&self) -> HOST_DISCONNECT_R {
        HOST_DISCONNECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    pub fn line_state(&self) -> LINE_STATE_R {
        LINE_STATE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    pub fn utmi_clk_valid(&self) -> UTMI_CLK_VALID_R {
        UTMI_CLK_VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_valid(&mut self) -> VBUS_VALID_W<PHY_STATUS_SPEC> {
        VBUS_VALID_W::new(self, 0)
    }
    #[doc = "Bit 2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_sess_valid(&mut self) -> UTMI_SESS_VALID_W<PHY_STATUS_SPEC> {
        UTMI_SESS_VALID_W::new(self, 2)
    }
    #[doc = "Bit 4 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn id_dig(&mut self) -> ID_DIG_W<PHY_STATUS_SPEC> {
        ID_DIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn host_disconnect(&mut self) -> HOST_DISCONNECT_W<PHY_STATUS_SPEC> {
        HOST_DISCONNECT_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn line_state(&mut self) -> LINE_STATE_W<PHY_STATUS_SPEC> {
        LINE_STATE_W::new(self, 6)
    }
    #[doc = "Bit 31 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn utmi_clk_valid(&mut self) -> UTMI_CLK_VALID_W<PHY_STATUS_SPEC> {
        UTMI_CLK_VALID_W::new(self, 31)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHY_STATUS_SPEC;
impl crate::RegisterSpec for PHY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_status::R`](R) reader structure"]
impl crate::Readable for PHY_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phy_status::W`](W) writer structure"]
impl crate::Writable for PHY_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_STATUS to value 0"]
impl crate::Resettable for PHY_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
