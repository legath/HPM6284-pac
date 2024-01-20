#[doc = "Register `AOI_8to7_06` reader"]
pub type R = crate::R<AOI_8TO7_06_SPEC>;
#[doc = "Register `AOI_8to7_06` writer"]
pub type W = crate::W<AOI_8TO7_06_SPEC>;
#[doc = "Field `AOI_8TO7_06_0` reader - select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1"]
pub type AOI_8TO7_06_0_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_0` writer - select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1"]
pub type AOI_8TO7_06_0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_1` reader - select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1"]
pub type AOI_8TO7_06_1_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_1` writer - select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1"]
pub type AOI_8TO7_06_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_2` reader - select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1"]
pub type AOI_8TO7_06_2_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_2` writer - select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1"]
pub type AOI_8TO7_06_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_3` reader - select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1"]
pub type AOI_8TO7_06_3_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_3` writer - select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1"]
pub type AOI_8TO7_06_3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_4` reader - select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1"]
pub type AOI_8TO7_06_4_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_4` writer - select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1"]
pub type AOI_8TO7_06_4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_5` reader - select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1"]
pub type AOI_8TO7_06_5_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_5` writer - select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1"]
pub type AOI_8TO7_06_5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_6` reader - select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1"]
pub type AOI_8TO7_06_6_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_6` writer - select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1"]
pub type AOI_8TO7_06_6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AOI_8TO7_06_7` reader - select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1"]
pub type AOI_8TO7_06_7_R = crate::FieldReader;
#[doc = "Field `AOI_8TO7_06_7` writer - select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1"]
pub type AOI_8TO7_06_7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_0(&self) -> AOI_8TO7_06_0_R {
        AOI_8TO7_06_0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_1(&self) -> AOI_8TO7_06_1_R {
        AOI_8TO7_06_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_2(&self) -> AOI_8TO7_06_2_R {
        AOI_8TO7_06_2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_3(&self) -> AOI_8TO7_06_3_R {
        AOI_8TO7_06_3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_4(&self) -> AOI_8TO7_06_4_R {
        AOI_8TO7_06_4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_5(&self) -> AOI_8TO7_06_5_R {
        AOI_8TO7_06_5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_6(&self) -> AOI_8TO7_06_6_R {
        AOI_8TO7_06_6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1"]
    #[inline(always)]
    pub fn aoi_8to7_06_7(&self) -> AOI_8TO7_06_7_R {
        AOI_8TO7_06_7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - select value for AOI_8to7_06_0. 0: 0. 1: 2nd_filter_out\\[0\\]. 2: ~2nd_filter_out\\[0\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_0(&mut self) -> AOI_8TO7_06_0_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - select value for AOI_8to7_06_1. 0: 0. 1: 2nd_filter_out\\[1\\]. 2: ~2nd_filter_out\\[1\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_1(&mut self) -> AOI_8TO7_06_1_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - select value for AOI_8to7_06_2. 0: 0. 1: 2nd_filter_out\\[2\\]. 2: ~2nd_filter_out\\[2\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_2(&mut self) -> AOI_8TO7_06_2_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - select value for AOI_8to7_06_3. 0: 0. 1: 2nd_filter_out\\[3\\]. 2: ~2nd_filter_out\\[3\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_3(&mut self) -> AOI_8TO7_06_3_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - select value for AOI_8to7_06_4. 0: 0. 1: 2nd_filter_out\\[4\\]. 2: ~2nd_filter_out\\[4\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_4(&mut self) -> AOI_8TO7_06_4_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - select value for AOI_8to7_06_5. 0: 0. 1: 2nd_filter_out\\[5\\]. 2: ~2nd_filter_out\\[5\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_5(&mut self) -> AOI_8TO7_06_5_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - select value for AOI_8to7_06_6. 0: 0. 1: 2nd_filter_out\\[6\\]. 2: ~2nd_filter_out\\[6\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_6(&mut self) -> AOI_8TO7_06_6_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - select value for AOI_8to7_06_7. 0: 0. 1: 2nd_filter_out\\[7\\]. 2: ~2nd_filter_out\\[7\\]. 3: 1"]
    #[inline(always)]
    #[must_use]
    pub fn aoi_8to7_06_7(&mut self) -> AOI_8TO7_06_7_W<AOI_8TO7_06_SPEC> {
        AOI_8TO7_06_7_W::new(self, 14)
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
#[doc = "CHN&amp;index0 AOI_16to8_06 OR logic cfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoi_8to7_06::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoi_8to7_06::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AOI_8TO7_06_SPEC;
impl crate::RegisterSpec for AOI_8TO7_06_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aoi_8to7_06::R`](R) reader structure"]
impl crate::Readable for AOI_8TO7_06_SPEC {}
#[doc = "`write(|w| ..)` method takes [`aoi_8to7_06::W`](W) writer structure"]
impl crate::Writable for AOI_8TO7_06_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AOI_8to7_06 to value 0"]
impl crate::Resettable for AOI_8TO7_06_SPEC {
    const RESET_VALUE: u32 = 0;
}
