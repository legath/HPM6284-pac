#[doc = "Register `RESOURCE[%s]` reader"]
pub type R = crate::R<RESOURCE_SPEC>;
#[doc = "Register `RESOURCE[%s]` writer"]
pub type W = crate::W<RESOURCE_SPEC>;
#[doc = "Field `MODE` reader - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOC_BUSY` reader - local busy 0: no change is pending for current node 1: current node is changing status"]
pub type LOC_BUSY_R = crate::BitReader;
#[doc = "Field `GLB_BUSY` reader - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
pub type GLB_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 30 - local busy 0: no change is pending for current node 1: current node is changing status"]
    #[inline(always)]
    pub fn loc_busy(&self) -> LOC_BUSY_R {
        LOC_BUSY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - global busy 0: no changes pending to any nodes 1: any of nodes is changing status"]
    #[inline(always)]
    pub fn glb_busy(&self) -> GLB_BUSY_R {
        GLB_BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - resource work mode 0:auto turn on and off as system required(recommended) 1:always on 2:always off 3:reserved"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<RESOURCE_SPEC> {
        MODE_W::new(self, 0)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resource::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resource::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESOURCE_SPEC;
impl crate::RegisterSpec for RESOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resource::R`](R) reader structure"]
impl crate::Readable for RESOURCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`resource::W`](W) writer structure"]
impl crate::Writable for RESOURCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESOURCE[%s]
to value 0"]
impl crate::Resettable for RESOURCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
