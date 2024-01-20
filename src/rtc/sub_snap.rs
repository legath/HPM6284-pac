#[doc = "Register `SUB_SNAP` reader"]
pub type R = crate::R<SUB_SNAP_SPEC>;
#[doc = "Register `SUB_SNAP` writer"]
pub type W = crate::W<SUB_SNAP_SPEC>;
#[doc = "Field `SUB_SNAP` reader - sub second snap shot, write to take snap shot"]
pub type SUB_SNAP_R = crate::FieldReader<u32>;
#[doc = "Field `SUB_SNAP` writer - sub second snap shot, write to take snap shot"]
pub type SUB_SNAP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sub second snap shot, write to take snap shot"]
    #[inline(always)]
    pub fn sub_snap(&self) -> SUB_SNAP_R {
        SUB_SNAP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - sub second snap shot, write to take snap shot"]
    #[inline(always)]
    #[must_use]
    pub fn sub_snap(&mut self) -> SUB_SNAP_W<SUB_SNAP_SPEC> {
        SUB_SNAP_W::new(self, 0)
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
#[doc = "Sub-second counter snap shot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_snap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_snap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUB_SNAP_SPEC;
impl crate::RegisterSpec for SUB_SNAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub_snap::R`](R) reader structure"]
impl crate::Readable for SUB_SNAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sub_snap::W`](W) writer structure"]
impl crate::Writable for SUB_SNAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUB_SNAP to value 0"]
impl crate::Resettable for SUB_SNAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
