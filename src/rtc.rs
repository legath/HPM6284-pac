#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    second: SECOND,
    subsec: SUBSEC,
    sec_snap: SEC_SNAP,
    sub_snap: SUB_SNAP,
    alarm0: ALARM0,
    alarm0_inc: ALARM0_INC,
    alarm1: ALARM1,
    alarm1_inc: ALARM1_INC,
    alarm_flag: ALARM_FLAG,
    alarm_en: ALARM_EN,
}
impl RegisterBlock {
    #[doc = "0x00 - Second counter"]
    #[inline(always)]
    pub const fn second(&self) -> &SECOND {
        &self.second
    }
    #[doc = "0x04 - Sub-second counter"]
    #[inline(always)]
    pub const fn subsec(&self) -> &SUBSEC {
        &self.subsec
    }
    #[doc = "0x08 - Second counter snap shot"]
    #[inline(always)]
    pub const fn sec_snap(&self) -> &SEC_SNAP {
        &self.sec_snap
    }
    #[doc = "0x0c - Sub-second counter snap shot"]
    #[inline(always)]
    pub const fn sub_snap(&self) -> &SUB_SNAP {
        &self.sub_snap
    }
    #[doc = "0x10 - RTC alarm0"]
    #[inline(always)]
    pub const fn alarm0(&self) -> &ALARM0 {
        &self.alarm0
    }
    #[doc = "0x14 - Alarm0 incremental"]
    #[inline(always)]
    pub const fn alarm0_inc(&self) -> &ALARM0_INC {
        &self.alarm0_inc
    }
    #[doc = "0x18 - RTC alarm1"]
    #[inline(always)]
    pub const fn alarm1(&self) -> &ALARM1 {
        &self.alarm1
    }
    #[doc = "0x1c - Alarm1 incremental"]
    #[inline(always)]
    pub const fn alarm1_inc(&self) -> &ALARM1_INC {
        &self.alarm1_inc
    }
    #[doc = "0x20 - RTC alarm flag"]
    #[inline(always)]
    pub const fn alarm_flag(&self) -> &ALARM_FLAG {
        &self.alarm_flag
    }
    #[doc = "0x24 - RTC alarm enable"]
    #[inline(always)]
    pub const fn alarm_en(&self) -> &ALARM_EN {
        &self.alarm_en
    }
}
#[doc = "SECOND (rw) register accessor: Second counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`second::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`second::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@second`]
module"]
pub type SECOND = crate::Reg<second::SECOND_SPEC>;
#[doc = "Second counter"]
pub mod second;
#[doc = "SUBSEC (rw) register accessor: Sub-second counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsec`]
module"]
pub type SUBSEC = crate::Reg<subsec::SUBSEC_SPEC>;
#[doc = "Sub-second counter"]
pub mod subsec;
#[doc = "SEC_SNAP (rw) register accessor: Second counter snap shot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec_snap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_snap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_snap`]
module"]
pub type SEC_SNAP = crate::Reg<sec_snap::SEC_SNAP_SPEC>;
#[doc = "Second counter snap shot"]
pub mod sec_snap;
#[doc = "SUB_SNAP (rw) register accessor: Sub-second counter snap shot\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sub_snap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sub_snap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sub_snap`]
module"]
pub type SUB_SNAP = crate::Reg<sub_snap::SUB_SNAP_SPEC>;
#[doc = "Sub-second counter snap shot"]
pub mod sub_snap;
#[doc = "ALARM0 (rw) register accessor: RTC alarm0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0`]
module"]
pub type ALARM0 = crate::Reg<alarm0::ALARM0_SPEC>;
#[doc = "RTC alarm0"]
pub mod alarm0;
#[doc = "ALARM0_INC (rw) register accessor: Alarm0 incremental\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm0_inc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm0_inc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm0_inc`]
module"]
pub type ALARM0_INC = crate::Reg<alarm0_inc::ALARM0_INC_SPEC>;
#[doc = "Alarm0 incremental"]
pub mod alarm0_inc;
#[doc = "ALARM1 (rw) register accessor: RTC alarm1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1`]
module"]
pub type ALARM1 = crate::Reg<alarm1::ALARM1_SPEC>;
#[doc = "RTC alarm1"]
pub mod alarm1;
#[doc = "ALARM1_INC (rw) register accessor: Alarm1 incremental\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm1_inc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm1_inc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1_inc`]
module"]
pub type ALARM1_INC = crate::Reg<alarm1_inc::ALARM1_INC_SPEC>;
#[doc = "Alarm1 incremental"]
pub mod alarm1_inc;
#[doc = "ALARM_FLAG (rw) register accessor: RTC alarm flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm_flag`]
module"]
pub type ALARM_FLAG = crate::Reg<alarm_flag::ALARM_FLAG_SPEC>;
#[doc = "RTC alarm flag"]
pub mod alarm_flag;
#[doc = "ALARM_EN (rw) register accessor: RTC alarm enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alarm_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alarm_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm_en`]
module"]
pub type ALARM_EN = crate::Reg<alarm_en::ALARM_EN_SPEC>;
#[doc = "RTC alarm enable"]
pub mod alarm_en;
