#[doc = "Register `CMPCFG[%s]` reader"]
pub type R = crate::R<CMPCFG_SPEC>;
#[doc = "Register `CMPCFG[%s]` writer"]
pub type W = crate::W<CMPCFG_SPEC>;
#[doc = "Field `CMPMODE` reader - comparator mode 0- output compare mode 1- input capture mode"]
pub type CMPMODE_R = crate::BitReader;
#[doc = "Field `CMPMODE` writer - comparator mode 0- output compare mode 1- input capture mode"]
pub type CMPMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSHDWUPT` reader - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CMPSHDWUPT_R = crate::FieldReader;
#[doc = "Field `CMPSHDWUPT` writer - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
pub type CMPSHDWUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XCNTCMPEN` reader - This bitfield enable the comparator to compare xcmp with xcnt."]
pub type XCNTCMPEN_R = crate::FieldReader;
#[doc = "Field `XCNTCMPEN` writer - This bitfield enable the comparator to compare xcmp with xcnt."]
pub type XCNTCMPEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - comparator mode 0- output compare mode 1- input capture mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    pub fn cmpshdwupt(&self) -> CMPSHDWUPT_R {
        CMPSHDWUPT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - This bitfield enable the comparator to compare xcmp with xcnt."]
    #[inline(always)]
    pub fn xcntcmpen(&self) -> XCNTCMPEN_R {
        XCNTCMPEN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - comparator mode 0- output compare mode 1- input capture mode"]
    #[inline(always)]
    #[must_use]
    pub fn cmpmode(&mut self) -> CMPMODE_W<CMPCFG_SPEC> {
        CMPMODE_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - This bitfield select when the comparator shadow register will be loaded to its work register 00: after software set shlk bit of shlk register 01: immediately after the register being modified 10: after hardware event assert, user can select one of the comparators to generate this hardware event. The comparator can be either output compare mode or input capture mode. 11: after SHSYNCI assert"]
    #[inline(always)]
    #[must_use]
    pub fn cmpshdwupt(&mut self) -> CMPSHDWUPT_W<CMPCFG_SPEC> {
        CMPSHDWUPT_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - This bitfield enable the comparator to compare xcmp with xcnt."]
    #[inline(always)]
    #[must_use]
    pub fn xcntcmpen(&mut self) -> XCNTCMPEN_W<CMPCFG_SPEC> {
        XCNTCMPEN_W::new(self, 4)
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
#[doc = "no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPCFG_SPEC;
impl crate::RegisterSpec for CMPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpcfg::R`](R) reader structure"]
impl crate::Readable for CMPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpcfg::W`](W) writer structure"]
impl crate::Writable for CMPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPCFG[%s]
to value 0"]
impl crate::Resettable for CMPCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
