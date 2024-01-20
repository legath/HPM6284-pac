#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x80],
    gptimer0ld: GPTIMER0LD,
    gptimer0ctrl: GPTIMER0CTRL,
    gptimer1ld: GPTIMER1LD,
    gptimer1ctrl: GPTIMER1CTRL,
    sbuscfg: SBUSCFG,
    _reserved5: [u8; 0xac],
    usbcmd: USBCMD,
    usbsts: USBSTS,
    usbintr: USBINTR,
    frindex: FRINDEX,
    _reserved9: [u8; 0x04],
    _reserved_9_deviceaddr: [u8; 0x04],
    _reserved_10_asynclistaddr: [u8; 0x04],
    _reserved11: [u8; 0x04],
    burstsize: BURSTSIZE,
    txfilltuning: TXFILLTUNING,
    _reserved13: [u8; 0x10],
    endptnak: ENDPTNAK,
    endptnaken: ENDPTNAKEN,
    _reserved15: [u8; 0x04],
    portsc1: PORTSC1,
    _reserved16: [u8; 0x1c],
    otgsc: OTGSC,
    usbmode: USBMODE,
    endptsetupstat: ENDPTSETUPSTAT,
    endptprime: ENDPTPRIME,
    endptflush: ENDPTFLUSH,
    endptstat: ENDPTSTAT,
    endptcomplete: ENDPTCOMPLETE,
    endptctrl: [ENDPTCTRL; 8],
    _reserved24: [u8; 0x20],
    otg_ctrl0: OTG_CTRL0,
    _reserved25: [u8; 0x0c],
    phy_ctrl0: PHY_CTRL0,
    phy_ctrl1: PHY_CTRL1,
    _reserved27: [u8; 0x08],
    top_status: TOP_STATUS,
    phy_status: PHY_STATUS,
}
impl RegisterBlock {
    #[doc = "0x80 - General Purpose Timer #0 Load Register"]
    #[inline(always)]
    pub const fn gptimer0ld(&self) -> &GPTIMER0LD {
        &self.gptimer0ld
    }
    #[doc = "0x84 - General Purpose Timer #0 Controller Register"]
    #[inline(always)]
    pub const fn gptimer0ctrl(&self) -> &GPTIMER0CTRL {
        &self.gptimer0ctrl
    }
    #[doc = "0x88 - General Purpose Timer #1 Load Register"]
    #[inline(always)]
    pub const fn gptimer1ld(&self) -> &GPTIMER1LD {
        &self.gptimer1ld
    }
    #[doc = "0x8c - General Purpose Timer #1 Controller Register"]
    #[inline(always)]
    pub const fn gptimer1ctrl(&self) -> &GPTIMER1CTRL {
        &self.gptimer1ctrl
    }
    #[doc = "0x90 - System Bus Config Register"]
    #[inline(always)]
    pub const fn sbuscfg(&self) -> &SBUSCFG {
        &self.sbuscfg
    }
    #[doc = "0x140 - USB Command Register"]
    #[inline(always)]
    pub const fn usbcmd(&self) -> &USBCMD {
        &self.usbcmd
    }
    #[doc = "0x144 - USB Status Register"]
    #[inline(always)]
    pub const fn usbsts(&self) -> &USBSTS {
        &self.usbsts
    }
    #[doc = "0x148 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn usbintr(&self) -> &USBINTR {
        &self.usbintr
    }
    #[doc = "0x14c - USB Frame Index Register"]
    #[inline(always)]
    pub const fn frindex(&self) -> &FRINDEX {
        &self.frindex
    }
    #[doc = "0x154 - Frame List Base Address Register"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &PERIODICLISTBASE {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x154 - Device Address Register"]
    #[inline(always)]
    pub const fn deviceaddr(&self) -> &DEVICEADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(340).cast() }
    }
    #[doc = "0x158 - Endpoint List Address Register"]
    #[inline(always)]
    pub const fn endptlistaddr(&self) -> &ENDPTLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x158 - Next Asynch. Address Register"]
    #[inline(always)]
    pub const fn asynclistaddr(&self) -> &ASYNCLISTADDR {
        unsafe { &*(self as *const Self).cast::<u8>().add(344).cast() }
    }
    #[doc = "0x160 - Programmable Burst Size Register"]
    #[inline(always)]
    pub const fn burstsize(&self) -> &BURSTSIZE {
        &self.burstsize
    }
    #[doc = "0x164 - TX FIFO Fill Tuning Register"]
    #[inline(always)]
    pub const fn txfilltuning(&self) -> &TXFILLTUNING {
        &self.txfilltuning
    }
    #[doc = "0x178 - Endpoint NAK Register"]
    #[inline(always)]
    pub const fn endptnak(&self) -> &ENDPTNAK {
        &self.endptnak
    }
    #[doc = "0x17c - Endpoint NAK Enable Register"]
    #[inline(always)]
    pub const fn endptnaken(&self) -> &ENDPTNAKEN {
        &self.endptnaken
    }
    #[doc = "0x184 - Port Status &amp; Control"]
    #[inline(always)]
    pub const fn portsc1(&self) -> &PORTSC1 {
        &self.portsc1
    }
    #[doc = "0x1a4 - On-The-Go Status &amp; control Register"]
    #[inline(always)]
    pub const fn otgsc(&self) -> &OTGSC {
        &self.otgsc
    }
    #[doc = "0x1a8 - USB Device Mode Register"]
    #[inline(always)]
    pub const fn usbmode(&self) -> &USBMODE {
        &self.usbmode
    }
    #[doc = "0x1ac - Endpoint Setup Status Register"]
    #[inline(always)]
    pub const fn endptsetupstat(&self) -> &ENDPTSETUPSTAT {
        &self.endptsetupstat
    }
    #[doc = "0x1b0 - Endpoint Prime Register"]
    #[inline(always)]
    pub const fn endptprime(&self) -> &ENDPTPRIME {
        &self.endptprime
    }
    #[doc = "0x1b4 - Endpoint Flush Register"]
    #[inline(always)]
    pub const fn endptflush(&self) -> &ENDPTFLUSH {
        &self.endptflush
    }
    #[doc = "0x1b8 - Endpoint Status Register"]
    #[inline(always)]
    pub const fn endptstat(&self) -> &ENDPTSTAT {
        &self.endptstat
    }
    #[doc = "0x1bc - Endpoint Complete Register"]
    #[inline(always)]
    pub const fn endptcomplete(&self) -> &ENDPTCOMPLETE {
        &self.endptcomplete
    }
    #[doc = "0x1c0..0x1e0 - no description available"]
    #[inline(always)]
    pub const fn endptctrl(&self, n: usize) -> &ENDPTCTRL {
        &self.endptctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1e0 - no description available"]
    #[inline(always)]
    pub fn endptctrl_iter(&self) -> impl Iterator<Item = &ENDPTCTRL> {
        self.endptctrl.iter()
    }
    #[doc = "0x1c0 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl0(&self) -> &ENDPTCTRL {
        self.endptctrl(0)
    }
    #[doc = "0x1c4 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl1(&self) -> &ENDPTCTRL {
        self.endptctrl(1)
    }
    #[doc = "0x1c8 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl2(&self) -> &ENDPTCTRL {
        self.endptctrl(2)
    }
    #[doc = "0x1cc - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl3(&self) -> &ENDPTCTRL {
        self.endptctrl(3)
    }
    #[doc = "0x1d0 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl4(&self) -> &ENDPTCTRL {
        self.endptctrl(4)
    }
    #[doc = "0x1d4 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl5(&self) -> &ENDPTCTRL {
        self.endptctrl(5)
    }
    #[doc = "0x1d8 - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl6(&self) -> &ENDPTCTRL {
        self.endptctrl(6)
    }
    #[doc = "0x1dc - no description available"]
    #[inline(always)]
    pub const fn endptctrlendptctrl7(&self) -> &ENDPTCTRL {
        self.endptctrl(7)
    }
    #[doc = "0x200 - No description avaiable"]
    #[inline(always)]
    pub const fn otg_ctrl0(&self) -> &OTG_CTRL0 {
        &self.otg_ctrl0
    }
    #[doc = "0x210 - No description avaiable"]
    #[inline(always)]
    pub const fn phy_ctrl0(&self) -> &PHY_CTRL0 {
        &self.phy_ctrl0
    }
    #[doc = "0x214 - No description avaiable"]
    #[inline(always)]
    pub const fn phy_ctrl1(&self) -> &PHY_CTRL1 {
        &self.phy_ctrl1
    }
    #[doc = "0x220 - No description avaiable"]
    #[inline(always)]
    pub const fn top_status(&self) -> &TOP_STATUS {
        &self.top_status
    }
    #[doc = "0x224 - No description avaiable"]
    #[inline(always)]
    pub const fn phy_status(&self) -> &PHY_STATUS {
        &self.phy_status
    }
}
#[doc = "GPTIMER0LD (rw) register accessor: General Purpose Timer #0 Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer0ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer0ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptimer0ld`]
module"]
pub type GPTIMER0LD = crate::Reg<gptimer0ld::GPTIMER0LD_SPEC>;
#[doc = "General Purpose Timer #0 Load Register"]
pub mod gptimer0ld;
#[doc = "GPTIMER0CTRL (rw) register accessor: General Purpose Timer #0 Controller Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer0ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer0ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptimer0ctrl`]
module"]
pub type GPTIMER0CTRL = crate::Reg<gptimer0ctrl::GPTIMER0CTRL_SPEC>;
#[doc = "General Purpose Timer #0 Controller Register"]
pub mod gptimer0ctrl;
#[doc = "GPTIMER1LD (rw) register accessor: General Purpose Timer #1 Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer1ld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer1ld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptimer1ld`]
module"]
pub type GPTIMER1LD = crate::Reg<gptimer1ld::GPTIMER1LD_SPEC>;
#[doc = "General Purpose Timer #1 Load Register"]
pub mod gptimer1ld;
#[doc = "GPTIMER1CTRL (rw) register accessor: General Purpose Timer #1 Controller Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptimer1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptimer1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptimer1ctrl`]
module"]
pub type GPTIMER1CTRL = crate::Reg<gptimer1ctrl::GPTIMER1CTRL_SPEC>;
#[doc = "General Purpose Timer #1 Controller Register"]
pub mod gptimer1ctrl;
#[doc = "SBUSCFG (rw) register accessor: System Bus Config Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbuscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbuscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbuscfg`]
module"]
pub type SBUSCFG = crate::Reg<sbuscfg::SBUSCFG_SPEC>;
#[doc = "System Bus Config Register"]
pub mod sbuscfg;
#[doc = "USBCMD (rw) register accessor: USB Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbcmd`]
module"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "USB Command Register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: USB Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbsts`]
module"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "USB Status Register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbintr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbintr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintr`]
module"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod usbintr;
#[doc = "FRINDEX (rw) register accessor: USB Frame Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frindex`]
module"]
pub type FRINDEX = crate::Reg<frindex::FRINDEX_SPEC>;
#[doc = "USB Frame Index Register"]
pub mod frindex;
#[doc = "DEVICEADDR (rw) register accessor: Device Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deviceaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceaddr`]
module"]
pub type DEVICEADDR = crate::Reg<deviceaddr::DEVICEADDR_SPEC>;
#[doc = "Device Address Register"]
pub mod deviceaddr;
#[doc = "PERIODICLISTBASE (rw) register accessor: Frame List Base Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periodiclistbase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`periodiclistbase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periodiclistbase`]
module"]
pub type PERIODICLISTBASE = crate::Reg<periodiclistbase::PERIODICLISTBASE_SPEC>;
#[doc = "Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "ASYNCLISTADDR (rw) register accessor: Next Asynch. Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`asynclistaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`asynclistaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynclistaddr`]
module"]
pub type ASYNCLISTADDR = crate::Reg<asynclistaddr::ASYNCLISTADDR_SPEC>;
#[doc = "Next Asynch. Address Register"]
pub mod asynclistaddr;
#[doc = "ENDPTLISTADDR (rw) register accessor: Endpoint List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptlistaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptlistaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptlistaddr`]
module"]
pub type ENDPTLISTADDR = crate::Reg<endptlistaddr::ENDPTLISTADDR_SPEC>;
#[doc = "Endpoint List Address Register"]
pub mod endptlistaddr;
#[doc = "BURSTSIZE (rw) register accessor: Programmable Burst Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`burstsize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`burstsize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@burstsize`]
module"]
pub type BURSTSIZE = crate::Reg<burstsize::BURSTSIZE_SPEC>;
#[doc = "Programmable Burst Size Register"]
pub mod burstsize;
#[doc = "TXFILLTUNING (rw) register accessor: TX FIFO Fill Tuning Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfilltuning::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfilltuning::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfilltuning`]
module"]
pub type TXFILLTUNING = crate::Reg<txfilltuning::TXFILLTUNING_SPEC>;
#[doc = "TX FIFO Fill Tuning Register"]
pub mod txfilltuning;
#[doc = "ENDPTNAK (rw) register accessor: Endpoint NAK Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptnak::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptnak::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptnak`]
module"]
pub type ENDPTNAK = crate::Reg<endptnak::ENDPTNAK_SPEC>;
#[doc = "Endpoint NAK Register"]
pub mod endptnak;
#[doc = "ENDPTNAKEN (rw) register accessor: Endpoint NAK Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptnaken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptnaken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptnaken`]
module"]
pub type ENDPTNAKEN = crate::Reg<endptnaken::ENDPTNAKEN_SPEC>;
#[doc = "Endpoint NAK Enable Register"]
pub mod endptnaken;
#[doc = "PORTSC1 (rw) register accessor: Port Status &amp; Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`portsc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`portsc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@portsc1`]
module"]
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
#[doc = "Port Status &amp; Control"]
pub mod portsc1;
#[doc = "OTGSC (rw) register accessor: On-The-Go Status &amp; control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otgsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otgsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otgsc`]
module"]
pub type OTGSC = crate::Reg<otgsc::OTGSC_SPEC>;
#[doc = "On-The-Go Status &amp; control Register"]
pub mod otgsc;
#[doc = "USBMODE (rw) register accessor: USB Device Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbmode`]
module"]
pub type USBMODE = crate::Reg<usbmode::USBMODE_SPEC>;
#[doc = "USB Device Mode Register"]
pub mod usbmode;
#[doc = "ENDPTSETUPSTAT (rw) register accessor: Endpoint Setup Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptsetupstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptsetupstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptsetupstat`]
module"]
pub type ENDPTSETUPSTAT = crate::Reg<endptsetupstat::ENDPTSETUPSTAT_SPEC>;
#[doc = "Endpoint Setup Status Register"]
pub mod endptsetupstat;
#[doc = "ENDPTPRIME (rw) register accessor: Endpoint Prime Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptprime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptprime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptprime`]
module"]
pub type ENDPTPRIME = crate::Reg<endptprime::ENDPTPRIME_SPEC>;
#[doc = "Endpoint Prime Register"]
pub mod endptprime;
#[doc = "ENDPTFLUSH (rw) register accessor: Endpoint Flush Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptflush::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptflush::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptflush`]
module"]
pub type ENDPTFLUSH = crate::Reg<endptflush::ENDPTFLUSH_SPEC>;
#[doc = "Endpoint Flush Register"]
pub mod endptflush;
#[doc = "ENDPTSTAT (rw) register accessor: Endpoint Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptstat`]
module"]
pub type ENDPTSTAT = crate::Reg<endptstat::ENDPTSTAT_SPEC>;
#[doc = "Endpoint Status Register"]
pub mod endptstat;
#[doc = "ENDPTCOMPLETE (rw) register accessor: Endpoint Complete Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptcomplete::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptcomplete::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptcomplete`]
module"]
pub type ENDPTCOMPLETE = crate::Reg<endptcomplete::ENDPTCOMPLETE_SPEC>;
#[doc = "Endpoint Complete Register"]
pub mod endptcomplete;
#[doc = "ENDPTCTRL (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endptctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endptctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endptctrl`]
module"]
pub type ENDPTCTRL = crate::Reg<endptctrl::ENDPTCTRL_SPEC>;
#[doc = "no description available"]
pub mod endptctrl;
#[doc = "OTG_CTRL0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_ctrl0`]
module"]
pub type OTG_CTRL0 = crate::Reg<otg_ctrl0::OTG_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod otg_ctrl0;
#[doc = "PHY_CTRL0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_ctrl0`]
module"]
pub type PHY_CTRL0 = crate::Reg<phy_ctrl0::PHY_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_ctrl0;
#[doc = "PHY_CTRL1 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_ctrl1`]
module"]
pub type PHY_CTRL1 = crate::Reg<phy_ctrl1::PHY_CTRL1_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_ctrl1;
#[doc = "TOP_STATUS (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`top_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top_status`]
module"]
pub type TOP_STATUS = crate::Reg<top_status::TOP_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod top_status;
#[doc = "PHY_STATUS (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phy_status`]
module"]
pub type PHY_STATUS = crate::Reg<phy_status::PHY_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod phy_status;
