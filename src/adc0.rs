#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    config: [CONFIG; 12],
    trg_dma_addr: TRG_DMA_ADDR,
    trg_sw_sta: TRG_SW_STA,
    _reserved3: [u8; 0x03c8],
    bus_result: [BUS_RESULT; 16],
    _reserved4: [u8; 0xc0],
    buf_cfg0: BUF_CFG0,
    _reserved5: [u8; 0x02fc],
    seq_cfg0: SEQ_CFG0,
    seq_dma_addr: SEQ_DMA_ADDR,
    seq_wr_addr: SEQ_WR_ADDR,
    seq_dma_cfg: SEQ_DMA_CFG,
    seq_que: [SEQ_QUE; 16],
    _reserved10: [u8; 0x03b0],
    prd_cfg: (),
    _reserved11: [u8; 0x0400],
    sample_cfg: [SAMPLE_CFG; 16],
    _reserved12: [u8; 0xc4],
    conv_cfg1: CONV_CFG1,
    adc_cfg0: ADC_CFG0,
    _reserved14: [u8; 0x04],
    int_sts: INT_STS,
    int_en: INT_EN,
    _reserved16: [u8; 0xe8],
    ana_ctrl0: ANA_CTRL0,
    _reserved17: [u8; 0x0c],
    ana_status: ANA_STATUS,
    _reserved18: [u8; 0x01ec],
    adc16_params: [ADC16_PARAMS; 34],
    adc16_config0: ADC16_CONFIG0,
    _reserved20: [u8; 0x18],
    adc16_config1: ADC16_CONFIG1,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - no description available"]
    #[inline(always)]
    pub const fn config(&self, n: usize) -> &CONFIG {
        &self.config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - no description available"]
    #[inline(always)]
    pub fn config_iter(&self) -> impl Iterator<Item = &CONFIG> {
        self.config.iter()
    }
    #[doc = "0x00 - no description available"]
    #[inline(always)]
    pub const fn configtrg0a(&self) -> &CONFIG {
        self.config(0)
    }
    #[doc = "0x04 - no description available"]
    #[inline(always)]
    pub const fn configtrg0b(&self) -> &CONFIG {
        self.config(1)
    }
    #[doc = "0x08 - no description available"]
    #[inline(always)]
    pub const fn configtrg0c(&self) -> &CONFIG {
        self.config(2)
    }
    #[doc = "0x0c - no description available"]
    #[inline(always)]
    pub const fn configtrg1a(&self) -> &CONFIG {
        self.config(3)
    }
    #[doc = "0x10 - no description available"]
    #[inline(always)]
    pub const fn configtrg1b(&self) -> &CONFIG {
        self.config(4)
    }
    #[doc = "0x14 - no description available"]
    #[inline(always)]
    pub const fn configtrg1c(&self) -> &CONFIG {
        self.config(5)
    }
    #[doc = "0x18 - no description available"]
    #[inline(always)]
    pub const fn configtrg2a(&self) -> &CONFIG {
        self.config(6)
    }
    #[doc = "0x1c - no description available"]
    #[inline(always)]
    pub const fn configtrg2b(&self) -> &CONFIG {
        self.config(7)
    }
    #[doc = "0x20 - no description available"]
    #[inline(always)]
    pub const fn configtrg2c(&self) -> &CONFIG {
        self.config(8)
    }
    #[doc = "0x24 - no description available"]
    #[inline(always)]
    pub const fn configtrg3a(&self) -> &CONFIG {
        self.config(9)
    }
    #[doc = "0x28 - no description available"]
    #[inline(always)]
    pub const fn configtrg3b(&self) -> &CONFIG {
        self.config(10)
    }
    #[doc = "0x2c - no description available"]
    #[inline(always)]
    pub const fn configtrg3c(&self) -> &CONFIG {
        self.config(11)
    }
    #[doc = "0x30 - No description avaiable"]
    #[inline(always)]
    pub const fn trg_dma_addr(&self) -> &TRG_DMA_ADDR {
        &self.trg_dma_addr
    }
    #[doc = "0x34 - No description avaiable"]
    #[inline(always)]
    pub const fn trg_sw_sta(&self) -> &TRG_SW_STA {
        &self.trg_sw_sta
    }
    #[doc = "0x400..0x440 - no description available"]
    #[inline(always)]
    pub const fn bus_result(&self, n: usize) -> &BUS_RESULT {
        &self.bus_result[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x440 - no description available"]
    #[inline(always)]
    pub fn bus_result_iter(&self) -> impl Iterator<Item = &BUS_RESULT> {
        self.bus_result.iter()
    }
    #[doc = "0x400 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn0(&self) -> &BUS_RESULT {
        self.bus_result(0)
    }
    #[doc = "0x404 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn1(&self) -> &BUS_RESULT {
        self.bus_result(1)
    }
    #[doc = "0x408 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn2(&self) -> &BUS_RESULT {
        self.bus_result(2)
    }
    #[doc = "0x40c - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn3(&self) -> &BUS_RESULT {
        self.bus_result(3)
    }
    #[doc = "0x410 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn4(&self) -> &BUS_RESULT {
        self.bus_result(4)
    }
    #[doc = "0x414 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn5(&self) -> &BUS_RESULT {
        self.bus_result(5)
    }
    #[doc = "0x418 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn6(&self) -> &BUS_RESULT {
        self.bus_result(6)
    }
    #[doc = "0x41c - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn7(&self) -> &BUS_RESULT {
        self.bus_result(7)
    }
    #[doc = "0x420 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn8(&self) -> &BUS_RESULT {
        self.bus_result(8)
    }
    #[doc = "0x424 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn9(&self) -> &BUS_RESULT {
        self.bus_result(9)
    }
    #[doc = "0x428 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn10(&self) -> &BUS_RESULT {
        self.bus_result(10)
    }
    #[doc = "0x42c - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn11(&self) -> &BUS_RESULT {
        self.bus_result(11)
    }
    #[doc = "0x430 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn12(&self) -> &BUS_RESULT {
        self.bus_result(12)
    }
    #[doc = "0x434 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn13(&self) -> &BUS_RESULT {
        self.bus_result(13)
    }
    #[doc = "0x438 - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn14(&self) -> &BUS_RESULT {
        self.bus_result(14)
    }
    #[doc = "0x43c - no description available"]
    #[inline(always)]
    pub const fn bus_resultchn15(&self) -> &BUS_RESULT {
        self.bus_result(15)
    }
    #[doc = "0x500 - No description avaiable"]
    #[inline(always)]
    pub const fn buf_cfg0(&self) -> &BUF_CFG0 {
        &self.buf_cfg0
    }
    #[doc = "0x800 - No description avaiable"]
    #[inline(always)]
    pub const fn seq_cfg0(&self) -> &SEQ_CFG0 {
        &self.seq_cfg0
    }
    #[doc = "0x804 - No description avaiable"]
    #[inline(always)]
    pub const fn seq_dma_addr(&self) -> &SEQ_DMA_ADDR {
        &self.seq_dma_addr
    }
    #[doc = "0x808 - No description avaiable"]
    #[inline(always)]
    pub const fn seq_wr_addr(&self) -> &SEQ_WR_ADDR {
        &self.seq_wr_addr
    }
    #[doc = "0x80c - No description avaiable"]
    #[inline(always)]
    pub const fn seq_dma_cfg(&self) -> &SEQ_DMA_CFG {
        &self.seq_dma_cfg
    }
    #[doc = "0x810..0x850 - no description available"]
    #[inline(always)]
    pub const fn seq_que(&self, n: usize) -> &SEQ_QUE {
        &self.seq_que[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x810..0x850 - no description available"]
    #[inline(always)]
    pub fn seq_que_iter(&self) -> impl Iterator<Item = &SEQ_QUE> {
        self.seq_que.iter()
    }
    #[doc = "0x810 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg0(&self) -> &SEQ_QUE {
        self.seq_que(0)
    }
    #[doc = "0x814 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg1(&self) -> &SEQ_QUE {
        self.seq_que(1)
    }
    #[doc = "0x818 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg2(&self) -> &SEQ_QUE {
        self.seq_que(2)
    }
    #[doc = "0x81c - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg3(&self) -> &SEQ_QUE {
        self.seq_que(3)
    }
    #[doc = "0x820 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg4(&self) -> &SEQ_QUE {
        self.seq_que(4)
    }
    #[doc = "0x824 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg5(&self) -> &SEQ_QUE {
        self.seq_que(5)
    }
    #[doc = "0x828 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg6(&self) -> &SEQ_QUE {
        self.seq_que(6)
    }
    #[doc = "0x82c - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg7(&self) -> &SEQ_QUE {
        self.seq_que(7)
    }
    #[doc = "0x830 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg8(&self) -> &SEQ_QUE {
        self.seq_que(8)
    }
    #[doc = "0x834 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg9(&self) -> &SEQ_QUE {
        self.seq_que(9)
    }
    #[doc = "0x838 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg10(&self) -> &SEQ_QUE {
        self.seq_que(10)
    }
    #[doc = "0x83c - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg11(&self) -> &SEQ_QUE {
        self.seq_que(11)
    }
    #[doc = "0x840 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg12(&self) -> &SEQ_QUE {
        self.seq_que(12)
    }
    #[doc = "0x844 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg13(&self) -> &SEQ_QUE {
        self.seq_que(13)
    }
    #[doc = "0x848 - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg14(&self) -> &SEQ_QUE {
        self.seq_que(14)
    }
    #[doc = "0x84c - no description available"]
    #[inline(always)]
    pub const fn seq_quecfg15(&self) -> &SEQ_QUE {
        self.seq_que(15)
    }
    #[doc = "0xc00..0xcc0 - no description available"]
    #[inline(always)]
    pub const fn prd_cfg(&self, n: usize) -> &PRD_CFG {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(3072)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc00..0xcc0 - no description available"]
    #[inline(always)]
    pub fn prd_cfg_iter(&self) -> impl Iterator<Item = &PRD_CFG> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(3072)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0xc00..0xc0c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn0(&self) -> &PRD_CFG {
        self.prd_cfg(0)
    }
    #[doc = "0xc10..0xc1c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn1(&self) -> &PRD_CFG {
        self.prd_cfg(1)
    }
    #[doc = "0xc20..0xc2c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn2(&self) -> &PRD_CFG {
        self.prd_cfg(2)
    }
    #[doc = "0xc30..0xc3c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn3(&self) -> &PRD_CFG {
        self.prd_cfg(3)
    }
    #[doc = "0xc40..0xc4c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn4(&self) -> &PRD_CFG {
        self.prd_cfg(4)
    }
    #[doc = "0xc50..0xc5c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn5(&self) -> &PRD_CFG {
        self.prd_cfg(5)
    }
    #[doc = "0xc60..0xc6c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn6(&self) -> &PRD_CFG {
        self.prd_cfg(6)
    }
    #[doc = "0xc70..0xc7c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn7(&self) -> &PRD_CFG {
        self.prd_cfg(7)
    }
    #[doc = "0xc80..0xc8c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn8(&self) -> &PRD_CFG {
        self.prd_cfg(8)
    }
    #[doc = "0xc90..0xc9c - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn9(&self) -> &PRD_CFG {
        self.prd_cfg(9)
    }
    #[doc = "0xca0..0xcac - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn10(&self) -> &PRD_CFG {
        self.prd_cfg(10)
    }
    #[doc = "0xcb0..0xcbc - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn11(&self) -> &PRD_CFG {
        self.prd_cfg(11)
    }
    #[doc = "0xcc0..0xccc - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn12(&self) -> &PRD_CFG {
        self.prd_cfg(12)
    }
    #[doc = "0xcd0..0xcdc - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn13(&self) -> &PRD_CFG {
        self.prd_cfg(13)
    }
    #[doc = "0xce0..0xcec - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn14(&self) -> &PRD_CFG {
        self.prd_cfg(14)
    }
    #[doc = "0xcf0..0xcfc - no description available"]
    #[inline(always)]
    pub const fn prd_cfgchn15(&self) -> &PRD_CFG {
        self.prd_cfg(15)
    }
    #[doc = "0x1000..0x1040 - no description available"]
    #[inline(always)]
    pub const fn sample_cfg(&self, n: usize) -> &SAMPLE_CFG {
        &self.sample_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1040 - no description available"]
    #[inline(always)]
    pub fn sample_cfg_iter(&self) -> impl Iterator<Item = &SAMPLE_CFG> {
        self.sample_cfg.iter()
    }
    #[doc = "0x1000 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn0(&self) -> &SAMPLE_CFG {
        self.sample_cfg(0)
    }
    #[doc = "0x1004 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn1(&self) -> &SAMPLE_CFG {
        self.sample_cfg(1)
    }
    #[doc = "0x1008 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn2(&self) -> &SAMPLE_CFG {
        self.sample_cfg(2)
    }
    #[doc = "0x100c - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn3(&self) -> &SAMPLE_CFG {
        self.sample_cfg(3)
    }
    #[doc = "0x1010 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn4(&self) -> &SAMPLE_CFG {
        self.sample_cfg(4)
    }
    #[doc = "0x1014 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn5(&self) -> &SAMPLE_CFG {
        self.sample_cfg(5)
    }
    #[doc = "0x1018 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn6(&self) -> &SAMPLE_CFG {
        self.sample_cfg(6)
    }
    #[doc = "0x101c - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn7(&self) -> &SAMPLE_CFG {
        self.sample_cfg(7)
    }
    #[doc = "0x1020 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn8(&self) -> &SAMPLE_CFG {
        self.sample_cfg(8)
    }
    #[doc = "0x1024 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn9(&self) -> &SAMPLE_CFG {
        self.sample_cfg(9)
    }
    #[doc = "0x1028 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn10(&self) -> &SAMPLE_CFG {
        self.sample_cfg(10)
    }
    #[doc = "0x102c - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn11(&self) -> &SAMPLE_CFG {
        self.sample_cfg(11)
    }
    #[doc = "0x1030 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn12(&self) -> &SAMPLE_CFG {
        self.sample_cfg(12)
    }
    #[doc = "0x1034 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn13(&self) -> &SAMPLE_CFG {
        self.sample_cfg(13)
    }
    #[doc = "0x1038 - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn14(&self) -> &SAMPLE_CFG {
        self.sample_cfg(14)
    }
    #[doc = "0x103c - no description available"]
    #[inline(always)]
    pub const fn sample_cfgchn15(&self) -> &SAMPLE_CFG {
        self.sample_cfg(15)
    }
    #[doc = "0x1104 - No description avaiable"]
    #[inline(always)]
    pub const fn conv_cfg1(&self) -> &CONV_CFG1 {
        &self.conv_cfg1
    }
    #[doc = "0x1108 - No description avaiable"]
    #[inline(always)]
    pub const fn adc_cfg0(&self) -> &ADC_CFG0 {
        &self.adc_cfg0
    }
    #[doc = "0x1110 - No description avaiable"]
    #[inline(always)]
    pub const fn int_sts(&self) -> &INT_STS {
        &self.int_sts
    }
    #[doc = "0x1114 - No description avaiable"]
    #[inline(always)]
    pub const fn int_en(&self) -> &INT_EN {
        &self.int_en
    }
    #[doc = "0x1200 - No description avaiable"]
    #[inline(always)]
    pub const fn ana_ctrl0(&self) -> &ANA_CTRL0 {
        &self.ana_ctrl0
    }
    #[doc = "0x1210 - No description avaiable"]
    #[inline(always)]
    pub const fn ana_status(&self) -> &ANA_STATUS {
        &self.ana_status
    }
    #[doc = "0x1400..0x1444 - no description available"]
    #[inline(always)]
    pub const fn adc16_params(&self, n: usize) -> &ADC16_PARAMS {
        &self.adc16_params[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x1444 - no description available"]
    #[inline(always)]
    pub fn adc16_params_iter(&self) -> impl Iterator<Item = &ADC16_PARAMS> {
        self.adc16_params.iter()
    }
    #[doc = "0x1400 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para00(&self) -> &ADC16_PARAMS {
        self.adc16_params(0)
    }
    #[doc = "0x1402 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para01(&self) -> &ADC16_PARAMS {
        self.adc16_params(1)
    }
    #[doc = "0x1404 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para02(&self) -> &ADC16_PARAMS {
        self.adc16_params(2)
    }
    #[doc = "0x1406 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para03(&self) -> &ADC16_PARAMS {
        self.adc16_params(3)
    }
    #[doc = "0x1408 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para04(&self) -> &ADC16_PARAMS {
        self.adc16_params(4)
    }
    #[doc = "0x140a - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para05(&self) -> &ADC16_PARAMS {
        self.adc16_params(5)
    }
    #[doc = "0x140c - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para06(&self) -> &ADC16_PARAMS {
        self.adc16_params(6)
    }
    #[doc = "0x140e - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para07(&self) -> &ADC16_PARAMS {
        self.adc16_params(7)
    }
    #[doc = "0x1410 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para08(&self) -> &ADC16_PARAMS {
        self.adc16_params(8)
    }
    #[doc = "0x1412 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para09(&self) -> &ADC16_PARAMS {
        self.adc16_params(9)
    }
    #[doc = "0x1414 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para10(&self) -> &ADC16_PARAMS {
        self.adc16_params(10)
    }
    #[doc = "0x1416 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para11(&self) -> &ADC16_PARAMS {
        self.adc16_params(11)
    }
    #[doc = "0x1418 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para12(&self) -> &ADC16_PARAMS {
        self.adc16_params(12)
    }
    #[doc = "0x141a - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para13(&self) -> &ADC16_PARAMS {
        self.adc16_params(13)
    }
    #[doc = "0x141c - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para14(&self) -> &ADC16_PARAMS {
        self.adc16_params(14)
    }
    #[doc = "0x141e - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para15(&self) -> &ADC16_PARAMS {
        self.adc16_params(15)
    }
    #[doc = "0x1420 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para16(&self) -> &ADC16_PARAMS {
        self.adc16_params(16)
    }
    #[doc = "0x1422 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para17(&self) -> &ADC16_PARAMS {
        self.adc16_params(17)
    }
    #[doc = "0x1424 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para18(&self) -> &ADC16_PARAMS {
        self.adc16_params(18)
    }
    #[doc = "0x1426 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para19(&self) -> &ADC16_PARAMS {
        self.adc16_params(19)
    }
    #[doc = "0x1428 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para20(&self) -> &ADC16_PARAMS {
        self.adc16_params(20)
    }
    #[doc = "0x142a - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para21(&self) -> &ADC16_PARAMS {
        self.adc16_params(21)
    }
    #[doc = "0x142c - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para22(&self) -> &ADC16_PARAMS {
        self.adc16_params(22)
    }
    #[doc = "0x142e - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para23(&self) -> &ADC16_PARAMS {
        self.adc16_params(23)
    }
    #[doc = "0x1430 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para24(&self) -> &ADC16_PARAMS {
        self.adc16_params(24)
    }
    #[doc = "0x1432 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para25(&self) -> &ADC16_PARAMS {
        self.adc16_params(25)
    }
    #[doc = "0x1434 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para26(&self) -> &ADC16_PARAMS {
        self.adc16_params(26)
    }
    #[doc = "0x1436 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para27(&self) -> &ADC16_PARAMS {
        self.adc16_params(27)
    }
    #[doc = "0x1438 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para28(&self) -> &ADC16_PARAMS {
        self.adc16_params(28)
    }
    #[doc = "0x143a - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para29(&self) -> &ADC16_PARAMS {
        self.adc16_params(29)
    }
    #[doc = "0x143c - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para30(&self) -> &ADC16_PARAMS {
        self.adc16_params(30)
    }
    #[doc = "0x143e - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para31(&self) -> &ADC16_PARAMS {
        self.adc16_params(31)
    }
    #[doc = "0x1440 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para32(&self) -> &ADC16_PARAMS {
        self.adc16_params(32)
    }
    #[doc = "0x1442 - no description available"]
    #[inline(always)]
    pub const fn adc16_paramsadc16_para33(&self) -> &ADC16_PARAMS {
        self.adc16_params(33)
    }
    #[doc = "0x1444 - No description avaiable"]
    #[inline(always)]
    pub const fn adc16_config0(&self) -> &ADC16_CONFIG0 {
        &self.adc16_config0
    }
    #[doc = "0x1460 - No description avaiable"]
    #[inline(always)]
    pub const fn adc16_config1(&self) -> &ADC16_CONFIG1 {
        &self.adc16_config1
    }
}
#[doc = "CONFIG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "no description available"]
pub mod config;
#[doc = "trg_dma_addr (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg_dma_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg_dma_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg_dma_addr`]
module"]
pub type TRG_DMA_ADDR = crate::Reg<trg_dma_addr::TRG_DMA_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod trg_dma_addr;
#[doc = "trg_sw_sta (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg_sw_sta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg_sw_sta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg_sw_sta`]
module"]
pub type TRG_SW_STA = crate::Reg<trg_sw_sta::TRG_SW_STA_SPEC>;
#[doc = "No description avaiable"]
pub mod trg_sw_sta;
#[doc = "BUS_RESULT (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_result::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_result::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_result`]
module"]
pub type BUS_RESULT = crate::Reg<bus_result::BUS_RESULT_SPEC>;
#[doc = "no description available"]
pub mod bus_result;
#[doc = "buf_cfg0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_cfg0`]
module"]
pub type BUF_CFG0 = crate::Reg<buf_cfg0::BUF_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod buf_cfg0;
#[doc = "seq_cfg0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_cfg0`]
module"]
pub type SEQ_CFG0 = crate::Reg<seq_cfg0::SEQ_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_cfg0;
#[doc = "seq_dma_addr (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_dma_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_dma_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_dma_addr`]
module"]
pub type SEQ_DMA_ADDR = crate::Reg<seq_dma_addr::SEQ_DMA_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_dma_addr;
#[doc = "seq_wr_addr (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_wr_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_wr_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_wr_addr`]
module"]
pub type SEQ_WR_ADDR = crate::Reg<seq_wr_addr::SEQ_WR_ADDR_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_wr_addr;
#[doc = "seq_dma_cfg (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_dma_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_dma_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_dma_cfg`]
module"]
pub type SEQ_DMA_CFG = crate::Reg<seq_dma_cfg::SEQ_DMA_CFG_SPEC>;
#[doc = "No description avaiable"]
pub mod seq_dma_cfg;
#[doc = "SEQ_QUE (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seq_que::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seq_que::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_que`]
module"]
pub type SEQ_QUE = crate::Reg<seq_que::SEQ_QUE_SPEC>;
#[doc = "no description available"]
pub mod seq_que;
#[doc = "no description available"]
pub use self::prd_cfg::PRD_CFG;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod prd_cfg;
#[doc = "SAMPLE_CFG (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sample_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sample_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_cfg`]
module"]
pub type SAMPLE_CFG = crate::Reg<sample_cfg::SAMPLE_CFG_SPEC>;
#[doc = "no description available"]
pub mod sample_cfg;
#[doc = "conv_cfg1 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conv_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conv_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conv_cfg1`]
module"]
pub type CONV_CFG1 = crate::Reg<conv_cfg1::CONV_CFG1_SPEC>;
#[doc = "No description avaiable"]
pub mod conv_cfg1;
#[doc = "adc_cfg0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_cfg0`]
module"]
pub type ADC_CFG0 = crate::Reg<adc_cfg0::ADC_CFG0_SPEC>;
#[doc = "No description avaiable"]
pub mod adc_cfg0;
#[doc = "int_sts (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_sts`]
module"]
pub type INT_STS = crate::Reg<int_sts::INT_STS_SPEC>;
#[doc = "No description avaiable"]
pub mod int_sts;
#[doc = "int_en (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "No description avaiable"]
pub mod int_en;
#[doc = "ana_ctrl0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_ctrl0`]
module"]
pub type ANA_CTRL0 = crate::Reg<ana_ctrl0::ANA_CTRL0_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_ctrl0;
#[doc = "ana_status (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_status`]
module"]
pub type ANA_STATUS = crate::Reg<ana_status::ANA_STATUS_SPEC>;
#[doc = "No description avaiable"]
pub mod ana_status;
#[doc = "ADC16_PARAMS (rw) register accessor: no description available\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_params::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_params::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc16_params`]
module"]
pub type ADC16_PARAMS = crate::Reg<adc16_params::ADC16_PARAMS_SPEC>;
#[doc = "no description available"]
pub mod adc16_params;
#[doc = "adc16_config0 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_config0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_config0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc16_config0`]
module"]
pub type ADC16_CONFIG0 = crate::Reg<adc16_config0::ADC16_CONFIG0_SPEC>;
#[doc = "No description avaiable"]
pub mod adc16_config0;
#[doc = "adc16_config1 (rw) register accessor: No description avaiable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc16_config1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc16_config1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc16_config1`]
module"]
pub type ADC16_CONFIG1 = crate::Reg<adc16_config1::ADC16_CONFIG1_SPEC>;
#[doc = "No description avaiable"]
pub mod adc16_config1;
