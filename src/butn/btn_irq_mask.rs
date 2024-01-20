#[doc = "Register `BTN_IRQ_MASK` reader"]
pub type R = crate::R<BTN_IRQ_MASK_SPEC>;
#[doc = "Register `BTN_IRQ_MASK` writer"]
pub type W = crate::W<BTN_IRQ_MASK_SPEC>;
#[doc = "Field `PBTN` reader - Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type PBTN_R = crate::FieldReader;
#[doc = "Field `PBTN` writer - Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type PBTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WBTN` reader - Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type WBTN_R = crate::FieldReader;
#[doc = "Field `WBTN` writer - Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type WBTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DBTN` reader - Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type DBTN_R = crate::FieldReader;
#[doc = "Field `DBTN` writer - Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
pub type DBTN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCLICK` reader - power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type PCLICK_R = crate::FieldReader;
#[doc = "Field `PCLICK` writer - power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type PCLICK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XPCLICK` reader - power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XPCLICK_R = crate::FieldReader;
#[doc = "Field `XPCLICK` writer - power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XPCLICK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WCLICK` reader - wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type WCLICK_R = crate::FieldReader;
#[doc = "Field `WCLICK` writer - wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type WCLICK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XWCLICK` reader - wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XWCLICK_R = crate::FieldReader;
#[doc = "Field `XWCLICK` writer - wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
pub type XWCLICK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn pbtn(&self) -> PBTN_R {
        PBTN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn wbtn(&self) -> WBTN_R {
        WBTN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    pub fn dbtn(&self) -> DBTN_R {
        DBTN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn pclick(&self) -> PCLICK_R {
        PCLICK_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn xpclick(&self) -> XPCLICK_R {
        XPCLICK_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn wclick(&self) -> WCLICK_R {
        WCLICK_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    pub fn xwclick(&self) -> XWCLICK_R {
        XWCLICK_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn pbtn(&mut self) -> PBTN_W<BTN_IRQ_MASK_SPEC> {
        PBTN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Wake button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn wbtn(&mut self) -> WBTN_W<BTN_IRQ_MASK_SPEC> {
        WBTN_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Dual button press interrupt enable bit0: button pressed bit1: button confirmd bit2: button long pressed bit3: button long long pressed"]
    #[inline(always)]
    #[must_use]
    pub fn dbtn(&mut self) -> DBTN_W<BTN_IRQ_MASK_SPEC> {
        DBTN_W::new(self, 8)
    }
    #[doc = "Bits 16:18 - power button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn pclick(&mut self) -> PCLICK_W<BTN_IRQ_MASK_SPEC> {
        PCLICK_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - power button click status when wake button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn xpclick(&mut self) -> XPCLICK_W<BTN_IRQ_MASK_SPEC> {
        XPCLICK_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - wake button click interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn wclick(&mut self) -> WCLICK_W<BTN_IRQ_MASK_SPEC> {
        WCLICK_W::new(self, 24)
    }
    #[doc = "Bits 28:30 - wake button click status when power button held interrupt enable bit0: clicked bit1: double clicked bit2: tripple clicked"]
    #[inline(always)]
    #[must_use]
    pub fn xwclick(&mut self) -> XWCLICK_W<BTN_IRQ_MASK_SPEC> {
        XWCLICK_W::new(self, 28)
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
#[doc = "Button interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btn_irq_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btn_irq_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTN_IRQ_MASK_SPEC;
impl crate::RegisterSpec for BTN_IRQ_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btn_irq_mask::R`](R) reader structure"]
impl crate::Readable for BTN_IRQ_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btn_irq_mask::W`](W) writer structure"]
impl crate::Writable for BTN_IRQ_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTN_IRQ_MASK to value 0"]
impl crate::Resettable for BTN_IRQ_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
