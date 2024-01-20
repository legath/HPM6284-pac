#[doc = "Register `OTGSC` reader"]
pub type R = crate::R<OTGSC_SPEC>;
#[doc = "Register `OTGSC` writer"]
pub type W = crate::W<OTGSC_SPEC>;
#[doc = "Field `VD` reader - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_R = crate::BitReader;
#[doc = "Field `VD` writer - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VC` reader - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
pub type VC_R = crate::BitReader;
#[doc = "Field `VC` writer - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
pub type VC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDPU` reader - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
pub type IDPU_R = crate::BitReader;
#[doc = "Field `IDPU` writer - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
pub type IDPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ID` reader - ID USB ID - Read Only. 0 = A device, 1 = B device"]
pub type ID_R = crate::BitReader;
#[doc = "Field `AVV` reader - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
pub type AVV_R = crate::BitReader;
#[doc = "Field `ASV` reader - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
pub type ASV_R = crate::BitReader;
#[doc = "Field `IDIS` reader - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
pub type IDIS_R = crate::BitReader;
#[doc = "Field `IDIS` writer - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
pub type IDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVVIS` reader - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
pub type AVVIS_R = crate::BitReader;
#[doc = "Field `AVVIS` writer - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
pub type AVVIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASVIS` reader - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
pub type ASVIS_R = crate::BitReader;
#[doc = "Field `ASVIS` writer - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
pub type ASVIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIE` reader - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_R = crate::BitReader;
#[doc = "Field `IDIE` writer - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVVIE` reader - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_R = crate::BitReader;
#[doc = "Field `AVVIE` writer - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASVIE` reader - ASVIE A Session Valid Interrupt Enable - Read/Write."]
pub type ASVIE_R = crate::BitReader;
#[doc = "Field `ASVIE` writer - ASVIE A Session Valid Interrupt Enable - Read/Write."]
pub type ASVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
    #[inline(always)]
    pub fn idpu(&self) -> IDPU_R {
        IDPU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ID USB ID - Read Only. 0 = A device, 1 = B device"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AVV A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[inline(always)]
    pub fn avv(&self) -> AVV_R {
        AVV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ASV A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn idis(&self) -> IDIS_R {
        IDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn avvis(&self) -> AVVIS_R {
        AVVIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
    #[inline(always)]
    pub fn asvis(&self) -> ASVIS_R {
        ASVIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&self) -> AVVIE_R {
        AVVIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write."]
    #[inline(always)]
    pub fn asvie(&self) -> ASVIE_R {
        ASVIE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VD VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    #[must_use]
    pub fn vd(&mut self) -> VD_W<OTGSC_SPEC> {
        VD_W::new(self, 0)
    }
    #[doc = "Bit 1 - VC VBUS Charge - Read/Write. Setting this bit causes the VBus line to be charged. This is used for VBus pulsing during SRP."]
    #[inline(always)]
    #[must_use]
    pub fn vc(&mut self) -> VC_W<OTGSC_SPEC> {
        VC_W::new(self, 1)
    }
    #[doc = "Bit 5 - IDPU ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]. When this bit is 0, the ID input will not be sampled."]
    #[inline(always)]
    #[must_use]
    pub fn idpu(&mut self) -> IDPU_W<OTGSC_SPEC> {
        IDPU_W::new(self, 5)
    }
    #[doc = "Bit 16 - IDIS USB ID Interrupt Status - Read/Write. This bit is set when a change on the ID input has been detected. Software must write a one to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn idis(&mut self) -> IDIS_W<OTGSC_SPEC> {
        IDIS_W::new(self, 16)
    }
    #[doc = "Bit 17 - AVVIS A VBus Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the VBus valid threshold on an A device. Software must write a one to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn avvis(&mut self) -> AVVIS_W<OTGSC_SPEC> {
        AVVIS_W::new(self, 17)
    }
    #[doc = "Bit 18 - ASVIS A Session Valid Interrupt Status - Read/Write to Clear. This bit is set when VBus has either risen above or fallen below the A session valid threshold. Software must write a one to clear this bit."]
    #[inline(always)]
    #[must_use]
    pub fn asvis(&mut self) -> ASVIS_W<OTGSC_SPEC> {
        ASVIS_W::new(self, 18)
    }
    #[doc = "Bit 24 - IDIE USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn idie(&mut self) -> IDIE_W<OTGSC_SPEC> {
        IDIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - AVVIE A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn avvie(&mut self) -> AVVIE_W<OTGSC_SPEC> {
        AVVIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - ASVIE A Session Valid Interrupt Enable - Read/Write."]
    #[inline(always)]
    #[must_use]
    pub fn asvie(&mut self) -> ASVIE_W<OTGSC_SPEC> {
        ASVIE_W::new(self, 26)
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
#[doc = "On-The-Go Status &amp; control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgsc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otgsc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTGSC_SPEC;
impl crate::RegisterSpec for OTGSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgsc::R`](R) reader structure"]
impl crate::Readable for OTGSC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`otgsc::W`](W) writer structure"]
impl crate::Writable for OTGSC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTGSC to value 0"]
impl crate::Resettable for OTGSC_SPEC {
    const RESET_VALUE: u32 = 0;
}
