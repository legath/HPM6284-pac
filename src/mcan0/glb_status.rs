#[doc = "Register `GLB_STATUS` reader"]
pub type R = crate::R<GLB_STATUS_SPEC>;
#[doc = "Register `GLB_STATUS` writer"]
pub type W = crate::W<GLB_STATUS_SPEC>;
#[doc = "Field `M_CAN_INT0` reader - m_can interrupt status0"]
pub type M_CAN_INT0_R = crate::BitReader;
#[doc = "Field `M_CAN_INT1` reader - m_can interrupt status1"]
pub type M_CAN_INT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - m_can interrupt status0"]
    #[inline(always)]
    pub fn m_can_int0(&self) -> M_CAN_INT0_R {
        M_CAN_INT0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - m_can interrupt status1"]
    #[inline(always)]
    pub fn m_can_int1(&self) -> M_CAN_INT1_R {
        M_CAN_INT1_R::new(((self.bits >> 3) & 1) != 0)
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
#[doc = "global status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glb_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glb_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLB_STATUS_SPEC;
impl crate::RegisterSpec for GLB_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glb_status::R`](R) reader structure"]
impl crate::Readable for GLB_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`glb_status::W`](W) writer structure"]
impl crate::Writable for GLB_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLB_STATUS to value 0"]
impl crate::Resettable for GLB_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
