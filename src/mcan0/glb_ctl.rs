#[doc = "Register `GLB_CTL` reader"]
pub type R = crate::R<GLB_CTL_SPEC>;
#[doc = "Register `GLB_CTL` writer"]
pub type W = crate::W<GLB_CTL_SPEC>;
#[doc = "Field `TSU_TBIN_SEL` reader - No description avaiable"]
pub type TSU_TBIN_SEL_R = crate::FieldReader;
#[doc = "Field `TSU_TBIN_SEL` writer - No description avaiable"]
pub type TSU_TBIN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STBY_POL` reader - standby polarity selection"]
pub type STBY_POL_R = crate::BitReader;
#[doc = "Field `STBY_POL` writer - standby polarity selection"]
pub type STBY_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBY_CLR_EN` reader - m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0"]
pub type STBY_CLR_EN_R = crate::BitReader;
#[doc = "Field `STBY_CLR_EN` writer - m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0"]
pub type STBY_CLR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_CAN_STBY` reader - m_can standby control"]
pub type M_CAN_STBY_R = crate::BitReader;
#[doc = "Field `M_CAN_STBY` writer - m_can standby control"]
pub type M_CAN_STBY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - No description avaiable"]
    #[inline(always)]
    pub fn tsu_tbin_sel(&self) -> TSU_TBIN_SEL_R {
        TSU_TBIN_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 29 - standby polarity selection"]
    #[inline(always)]
    pub fn stby_pol(&self) -> STBY_POL_R {
        STBY_POL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0"]
    #[inline(always)]
    pub fn stby_clr_en(&self) -> STBY_CLR_EN_R {
        STBY_CLR_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - m_can standby control"]
    #[inline(always)]
    pub fn m_can_stby(&self) -> M_CAN_STBY_R {
        M_CAN_STBY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn tsu_tbin_sel(&mut self) -> TSU_TBIN_SEL_W<GLB_CTL_SPEC> {
        TSU_TBIN_SEL_W::new(self, 0)
    }
    #[doc = "Bit 29 - standby polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn stby_pol(&mut self) -> STBY_POL_W<GLB_CTL_SPEC> {
        STBY_POL_W::new(self, 29)
    }
    #[doc = "Bit 30 - m_can standby clear control 0:controlled by software by standby bit\\[bit31\\]
1:auto clear standby by hardware when rx data is 0"]
    #[inline(always)]
    #[must_use]
    pub fn stby_clr_en(&mut self) -> STBY_CLR_EN_W<GLB_CTL_SPEC> {
        STBY_CLR_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - m_can standby control"]
    #[inline(always)]
    #[must_use]
    pub fn m_can_stby(&mut self) -> M_CAN_STBY_W<GLB_CTL_SPEC> {
        M_CAN_STBY_W::new(self, 31)
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
#[doc = "global control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLB_CTL_SPEC;
impl crate::RegisterSpec for GLB_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_ctl::R`](R) reader structure"]
impl crate::Readable for GLB_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glb_ctl::W`](W) writer structure"]
impl crate::Writable for GLB_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_CTL to value 0"]
impl crate::Resettable for GLB_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
