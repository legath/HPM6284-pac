#[doc = r"Register block"]
#[repr(C)]
pub struct CPU {
    lp: LP,
    lock: LOCK,
    gpr: [GPR; 14],
    wakeup_status: [WAKEUP_STATUS; 4],
    _reserved4: [u8; 0x30],
    wakeup_enable: [WAKEUP_ENABLE; 4],
}
impl CPU {
    #[doc = "0x00 - CPU0 LP control"]
    #[inline(always)]
    pub const fn lp(&self) -> &LP {
        &self.lp
    }
    #[doc = "0x04 - CPU0 Lock GPR"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
    #[doc = "0x08..0x40 - no description available"]
    #[inline(always)]
    pub const fn gpr(&self, n: usize) -> &GPR {
        &self.gpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x40 - no description available"]
    #[inline(always)]
    pub fn gpr_iter(&self) -> impl Iterator<Item = &GPR> {
        self.gpr.iter()
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn gprgpr0(&self) -> &GPR {
        self.gpr(0)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn gprgpr1(&self) -> &GPR {
        self.gpr(1)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn gprgpr2(&self) -> &GPR {
        self.gpr(2)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn gprgpr3(&self) -> &GPR {
        self.gpr(3)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn gprgpr4(&self) -> &GPR {
        self.gpr(4)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn gprgpr5(&self) -> &GPR {
        self.gpr(5)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn gprgpr6(&self) -> &GPR {
        self.gpr(6)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn gprgpr7(&self) -> &GPR {
        self.gpr(7)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn gprgpr8(&self) -> &GPR {
        self.gpr(8)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn gprgpr9(&self) -> &GPR {
        self.gpr(9)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn gprgpr10(&self) -> &GPR {
        self.gpr(10)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn gprgpr11(&self) -> &GPR {
        self.gpr(11)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn gprgpr12(&self) -> &GPR {
        self.gpr(12)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn gprgpr13(&self) -> &GPR {
        self.gpr(13)
    }
    #[doc = "0x40..0x50 - no description available"]
    #[inline(always)]
    pub const fn wakeup_status(&self, n: usize) -> &WAKEUP_STATUS {
        &self.wakeup_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - no description available"]
    #[inline(always)]
    pub fn wakeup_status_iter(&self) -> impl Iterator<Item = &WAKEUP_STATUS> {
        self.wakeup_status.iter()
    }
    #[doc = "0x40 - no description available"]
    #[inline(always)]
    pub const fn wakeup_statusstatus0(&self) -> &WAKEUP_STATUS {
        self.wakeup_status(0)
    }
    #[doc = "0x44 - no description available"]
    #[inline(always)]
    pub const fn wakeup_statusstatus1(&self) -> &WAKEUP_STATUS {
        self.wakeup_status(1)
    }
    #[doc = "0x48 - no description available"]
    #[inline(always)]
    pub const fn wakeup_statusstatus2(&self) -> &WAKEUP_STATUS {
        self.wakeup_status(2)
    }
    #[doc = "0x4c - no description available"]
    #[inline(always)]
    pub const fn wakeup_statusstatus3(&self) -> &WAKEUP_STATUS {
        self.wakeup_status(3)
    }
    #[doc = "0x80..0x90 - no description available"]
    #[inline(always)]
    pub const fn wakeup_enable(&self, n: usize) -> &WAKEUP_ENABLE {
        &self.wakeup_enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - no description available"]
    #[inline(always)]
    pub fn wakeup_enable_iter(&self) -> impl Iterator<Item = &WAKEUP_ENABLE> {
        self.wakeup_enable.iter()
    }
    #[doc = "0x80 - no description available"]
    #[inline(always)]
    pub const fn wakeup_enableenable0(&self) -> &WAKEUP_ENABLE {
        self.wakeup_enable(0)
    }
    #[doc = "0x84 - no description available"]
    #[inline(always)]
    pub const fn wakeup_enableenable1(&self) -> &WAKEUP_ENABLE {
        self.wakeup_enable(1)
    }
    #[doc = "0x88 - no description available"]
    #[inline(always)]
    pub const fn wakeup_enableenable2(&self) -> &WAKEUP_ENABLE {
        self.wakeup_enable(2)
    }
    #[doc = "0x8c - no description available"]
    #[inline(always)]
    pub const fn wakeup_enableenable3(&self) -> &WAKEUP_ENABLE {
        self.wakeup_enable(3)
    }
}
#[doc = "LP (rw) register accessor: CPU0 LP control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp`]
module"]
pub type LP = crate::Reg<lp::LP_SPEC>;
#[doc = "CPU0 LP control"]
pub mod lp;
#[doc = "LOCK (rw) register accessor: CPU0 Lock GPR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "CPU0 Lock GPR"]
pub mod lock;
#[doc = "GPR (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpr`]
module"]
pub type GPR = crate::Reg<gpr::GPR_SPEC>;
#[doc = "no description available"]
pub mod gpr;
#[doc = "WAKEUP_STATUS (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_status`]
module"]
pub type WAKEUP_STATUS = crate::Reg<wakeup_status::WAKEUP_STATUS_SPEC>;
#[doc = "no description available"]
pub mod wakeup_status;
#[doc = "WAKEUP_ENABLE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeup_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wakeup_enable`]
module"]
pub type WAKEUP_ENABLE = crate::Reg<wakeup_enable::WAKEUP_ENABLE_SPEC>;
#[doc = "no description available"]
pub mod wakeup_enable;
