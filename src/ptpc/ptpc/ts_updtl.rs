#[doc = "Register `ts_updtl` reader"]
pub type R = crate::R<TS_UPDTL_SPEC>;
#[doc = "Register `ts_updtl` writer"]
pub type W = crate::W<TS_UPDTL_SPEC>;
#[doc = "Field `NS_UPDATE` reader - No description avaiable"]
pub type NS_UPDATE_R = crate::FieldReader<u32>;
#[doc = "Field `NS_UPDATE` writer - No description avaiable"]
pub type NS_UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `ADD_SUB` reader - 1 for sub; 0 for add, used only at update"]
pub type ADD_SUB_R = crate::BitReader;
#[doc = "Field `ADD_SUB` writer - 1 for sub; 0 for add, used only at update"]
pub type ADD_SUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - No description avaiable"]
    #[inline(always)]
    pub fn ns_update(&self) -> NS_UPDATE_R {
        NS_UPDATE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - 1 for sub; 0 for add, used only at update"]
    #[inline(always)]
    pub fn add_sub(&self) -> ADD_SUB_R {
        ADD_SUB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - No description avaiable"]
    #[inline(always)]
    #[must_use]
    pub fn ns_update(&mut self) -> NS_UPDATE_W<TS_UPDTL_SPEC> {
        NS_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - 1 for sub; 0 for add, used only at update"]
    #[inline(always)]
    #[must_use]
    pub fn add_sub(&mut self) -> ADD_SUB_W<TS_UPDTL_SPEC> {
        ADD_SUB_W::new(self, 31)
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
#[doc = "timestamp update low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ts_updtl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ts_updtl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TS_UPDTL_SPEC;
impl crate::RegisterSpec for TS_UPDTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ts_updtl::R`](R) reader structure"]
impl crate::Readable for TS_UPDTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ts_updtl::W`](W) writer structure"]
impl crate::Writable for TS_UPDTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ts_updtl to value 0"]
impl crate::Resettable for TS_UPDTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
