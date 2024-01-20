#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    cmd: CMD,
    ctrl: CTRL,
    sta: STA,
    err: ERR,
    fo2b: FO2B,
    _reserved5: [u8; 0x0c],
    r2sk: [R2SK; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn sta(&self) -> &STA {
        &self.sta
    }
    #[doc = "0x0c - Error Registers"]
    #[inline(always)]
    pub const fn err(&self) -> &ERR {
        &self.err
    }
    #[doc = "0x10 - FIFO out to bus/cpu"]
    #[inline(always)]
    pub const fn fo2b(&self) -> &FO2B {
        &self.fo2b
    }
    #[doc = "0x20..0x40 - no description available"]
    #[inline(always)]
    pub const fn r2sk(&self, n: usize) -> &R2SK {
        &self.r2sk[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - no description available"]
    #[inline(always)]
    pub fn r2sk_iter(&self) -> impl Iterator<Item = &R2SK> {
        self.r2sk.iter()
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s0(&self) -> &R2SK {
        self.r2sk(0)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s1(&self) -> &R2SK {
        self.r2sk(1)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s2(&self) -> &R2SK {
        self.r2sk(2)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s3(&self) -> &R2SK {
        self.r2sk(3)
    }
    #[doc = "0x30 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s4(&self) -> &R2SK {
        self.r2sk(4)
    }
    #[doc = "0x34 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s5(&self) -> &R2SK {
        self.r2sk(5)
    }
    #[doc = "0x38 - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s6(&self) -> &R2SK {
        self.r2sk(6)
    }
    #[doc = "0x3c - no description available"]
    #[inline(always)]
    pub const fn r2skfo2s7(&self) -> &R2SK {
        self.r2sk(7)
    }
}
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STA (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "Status Register"]
pub mod sta;
#[doc = "ERR (rw) register accessor: Error Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err`]
module"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Registers"]
pub mod err;
#[doc = "FO2B (rw) register accessor: FIFO out to bus/cpu\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fo2b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fo2b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fo2b`]
module"]
pub type FO2B = crate::Reg<fo2b::FO2B_SPEC>;
#[doc = "FIFO out to bus/cpu"]
pub mod fo2b;
#[doc = "R2SK (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2sk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2sk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r2sk`]
module"]
pub type R2SK = crate::Reg<r2sk::R2SK_SPEC>;
#[doc = "no description available"]
pub mod r2sk;
