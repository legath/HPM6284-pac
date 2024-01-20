#[doc = "Register `cr` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `cr` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `ENCTYP` reader - 00-abz; 01-pd; 10-ud; 11-reserved"]
pub type ENCTYP_R = crate::FieldReader;
#[doc = "Field `ENCTYP` writer - 00-abz; 01-pd; 10-ud; 11-reserved"]
pub type ENCTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSTCNT` reader - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
pub type RSTCNT_R = crate::BitReader;
#[doc = "Field `RSTCNT` writer - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
pub type RSTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPEN` reader - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_R = crate::BitReader;
#[doc = "Field `SNAPEN` writer - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
pub type SNAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFDIR0` reader - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
pub type HFDIR0_R = crate::BitReader;
#[doc = "Field `HFDIR0` writer - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
pub type HFDIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFDIR1` reader - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
pub type HFDIR1_R = crate::BitReader;
#[doc = "Field `HFDIR1` writer - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
pub type HFDIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRDIR0` reader - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
pub type HRDIR0_R = crate::BitReader;
#[doc = "Field `HRDIR0` writer - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
pub type HRDIR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRDIR1` reader - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
pub type HRDIR1_R = crate::BitReader;
#[doc = "Field `HRDIR1` writer - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
pub type HRDIR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSEZ` reader - 1- pause zcnt when PAUSE assert"]
pub type PAUSEZ_R = crate::BitReader;
#[doc = "Field `PAUSEZ` writer - 1- pause zcnt when PAUSE assert"]
pub type PAUSEZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSEPH` reader - 1- pause phcnt when PAUSE assert"]
pub type PAUSEPH_R = crate::BitReader;
#[doc = "Field `PAUSEPH` writer - 1- pause phcnt when PAUSE assert"]
pub type PAUSEPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAUSESPD` reader - 1- pause spdcnt when PAUSE assert"]
pub type PAUSESPD_R = crate::BitReader;
#[doc = "Field `PAUSESPD` writer - 1- pause spdcnt when PAUSE assert"]
pub type PAUSESPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTZ` reader - 1- reset zcnt when H assert"]
pub type HRSTZ_R = crate::BitReader;
#[doc = "Field `HRSTZ` writer - 1- reset zcnt when H assert"]
pub type HRSTZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTPH` reader - 1- reset phcnt when H assert"]
pub type HRSTPH_R = crate::BitReader;
#[doc = "Field `HRSTPH` writer - 1- reset phcnt when H assert"]
pub type HRSTPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRSTSPD` reader - 1- reset spdcnt when H assert"]
pub type HRSTSPD_R = crate::BitReader;
#[doc = "Field `HRSTSPD` writer - 1- reset spdcnt when H assert"]
pub type HRSTSPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ` writer - 1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 00-abz; 01-pd; 10-ud; 11-reserved"]
    #[inline(always)]
    pub fn enctyp(&self) -> ENCTYP_R {
        ENCTYP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    pub fn snapen(&self) -> SNAPEN_R {
        SNAPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
    #[inline(always)]
    pub fn hfdir0(&self) -> HFDIR0_R {
        HFDIR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    pub fn hfdir1(&self) -> HFDIR1_R {
        HFDIR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
    #[inline(always)]
    pub fn hrdir0(&self) -> HRDIR0_R {
        HRDIR0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    pub fn hrdir1(&self) -> HRDIR1_R {
        HRDIR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1- pause zcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pausez(&self) -> PAUSEZ_R {
        PAUSEZ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1- pause phcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pauseph(&self) -> PAUSEPH_R {
        PAUSEPH_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 1- pause spdcnt when PAUSE assert"]
    #[inline(always)]
    pub fn pausespd(&self) -> PAUSESPD_R {
        PAUSESPD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - 1- reset zcnt when H assert"]
    #[inline(always)]
    pub fn hrstz(&self) -> HRSTZ_R {
        HRSTZ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1- reset phcnt when H assert"]
    #[inline(always)]
    pub fn hrstph(&self) -> HRSTPH_R {
        HRSTPH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1- reset spdcnt when H assert"]
    #[inline(always)]
    pub fn hrstspd(&self) -> HRSTSPD_R {
        HRSTSPD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00-abz; 01-pd; 10-ud; 11-reserved"]
    #[inline(always)]
    #[must_use]
    pub fn enctyp(&mut self) -> ENCTYP_W<CR_SPEC> {
        ENCTYP_W::new(self, 0)
    }
    #[doc = "Bit 4 - 1- reset zcnt, spdcnt and tmrcnt to 0. reset phcnt to phidx"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RSTCNT_W<CR_SPEC> {
        RSTCNT_W::new(self, 4)
    }
    #[doc = "Bit 5 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their snap registers when snapi input assert"]
    #[inline(always)]
    #[must_use]
    pub fn snapen(&mut self) -> SNAPEN_W<CR_SPEC> {
        SNAPEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - 1- HOMEF will set at H falling edge when dir == 1 (positive rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hfdir0(&mut self) -> HFDIR0_W<CR_SPEC> {
        HFDIR0_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1- HOMEF will set at H falling edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hfdir1(&mut self) -> HFDIR1_W<CR_SPEC> {
        HFDIR1_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1- HOMEF will set at H rising edge when dir == 0 (positive rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hrdir0(&mut self) -> HRDIR0_W<CR_SPEC> {
        HRDIR0_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1- HOMEF will set at H rising edge when dir == 1 (negative rotation direction)"]
    #[inline(always)]
    #[must_use]
    pub fn hrdir1(&mut self) -> HRDIR1_W<CR_SPEC> {
        HRDIR1_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1- pause zcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pausez(&mut self) -> PAUSEZ_W<CR_SPEC> {
        PAUSEZ_W::new(self, 12)
    }
    #[doc = "Bit 13 - 1- pause phcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pauseph(&mut self) -> PAUSEPH_W<CR_SPEC> {
        PAUSEPH_W::new(self, 13)
    }
    #[doc = "Bit 14 - 1- pause spdcnt when PAUSE assert"]
    #[inline(always)]
    #[must_use]
    pub fn pausespd(&mut self) -> PAUSESPD_W<CR_SPEC> {
        PAUSESPD_W::new(self, 14)
    }
    #[doc = "Bit 16 - 1- reset zcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstz(&mut self) -> HRSTZ_W<CR_SPEC> {
        HRSTZ_W::new(self, 16)
    }
    #[doc = "Bit 17 - 1- reset phcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstph(&mut self) -> HRSTPH_W<CR_SPEC> {
        HRSTPH_W::new(self, 17)
    }
    #[doc = "Bit 18 - 1- reset spdcnt when H assert"]
    #[inline(always)]
    #[must_use]
    pub fn hrstspd(&mut self) -> HRSTSPD_W<CR_SPEC> {
        HRSTSPD_W::new(self, 18)
    }
    #[doc = "Bit 31 - 1- load phcnt, zcnt, spdcnt and tmrcnt into their read registers. Hardware auto-clear; read as 0"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<CR_SPEC> {
        READ_W::new(self, 31)
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
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0;
}
