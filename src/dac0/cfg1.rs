#[doc = "Register `cfg1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `cfg1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `DIV_CFG` reader - how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Used for step mode and buffer mode, if set to continual trigger mode"]
pub type DIV_CFG_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_CFG` writer - how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Used for step mode and buffer mode, if set to continual trigger mode"]
pub type DIV_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ANA_DIV_CFG` reader - clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8"]
pub type ANA_DIV_CFG_R = crate::FieldReader;
#[doc = "Field `ANA_DIV_CFG` writer - clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8"]
pub type ANA_DIV_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ANA_CLK_EN` reader - set to enable analog clock(divided by ana_div_cfg)"]
pub type ANA_CLK_EN_R = crate::BitReader;
#[doc = "Field `ANA_CLK_EN` writer - set to enable analog clock(divided by ana_div_cfg)"]
pub type ANA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Used for step mode and buffer mode, if set to continual trigger mode"]
    #[inline(always)]
    pub fn div_cfg(&self) -> DIV_CFG_R {
        DIV_CFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8"]
    #[inline(always)]
    pub fn ana_div_cfg(&self) -> ANA_DIV_CFG_R {
        ANA_DIV_CFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - set to enable analog clock(divided by ana_div_cfg)"]
    #[inline(always)]
    pub fn ana_clk_en(&self) -> ANA_CLK_EN_R {
        ANA_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - how many clk_dac cycles to change data to analog, should configured to less than 1MHz data rate. Used for step mode and buffer mode, if set to continual trigger mode"]
    #[inline(always)]
    #[must_use]
    pub fn div_cfg(&mut self) -> DIV_CFG_W<CFG1_SPEC> {
        DIV_CFG_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - clock divider config for ana_clk to dac analog; 00: div2 01: div4 10: div6 11: div8"]
    #[inline(always)]
    #[must_use]
    pub fn ana_div_cfg(&mut self) -> ANA_DIV_CFG_W<CFG1_SPEC> {
        ANA_DIV_CFG_W::new(self, 16)
    }
    #[doc = "Bit 18 - set to enable analog clock(divided by ana_div_cfg)"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_en(&mut self) -> ANA_CLK_EN_W<CFG1_SPEC> {
        ANA_CLK_EN_W::new(self, 18)
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
#[doc = "No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cfg1 to value 0x0001_0000"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
