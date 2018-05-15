#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Functional Address"]
    pub faddr: FADDR,
    #[doc = "0x01 - USB Power"]
    pub power: POWER,
    #[doc = "0x02 - USB Transmit Interrupt Status"]
    pub txis: TXIS,
    #[doc = "0x04 - USB Receive Interrupt Status"]
    pub rxis: RXIS,
    #[doc = "0x06 - USB Transmit Interrupt Enable"]
    pub txie: TXIE,
    #[doc = "0x08 - USB Receive Interrupt Enable"]
    pub rxie: RXIE,
    #[doc = "0x0a - USB General Interrupt Status"]
    pub is: IS,
    #[doc = "0x0b - USB Interrupt Enable"]
    pub ie: IE,
    #[doc = "0x0c - USB Frame Value"]
    pub frame: FRAME,
    #[doc = "0x0e - USB Endpoint Index"]
    pub epidx: EPIDX,
    #[doc = "0x0f - USB Test Mode"]
    pub test: TEST,
    _reserved0: [u8; 16usize],
    #[doc = "0x20 - USB FIFO Endpoint 0"]
    pub fifo0: FIFO0,
    #[doc = "0x24 - USB FIFO Endpoint 1"]
    pub fifo1: FIFO1,
    #[doc = "0x28 - USB FIFO Endpoint 2"]
    pub fifo2: FIFO2,
    #[doc = "0x2c - USB FIFO Endpoint 3"]
    pub fifo3: FIFO3,
    #[doc = "0x30 - USB FIFO Endpoint 4"]
    pub fifo4: FIFO4,
    #[doc = "0x34 - USB FIFO Endpoint 5"]
    pub fifo5: FIFO5,
    #[doc = "0x38 - USB FIFO Endpoint 6"]
    pub fifo6: FIFO6,
    #[doc = "0x3c - USB FIFO Endpoint 7"]
    pub fifo7: FIFO7,
    _reserved1: [u8; 32usize],
    #[doc = "0x60 - USB Device Control"]
    pub devctl: DEVCTL,
    #[doc = "0x61 - USB Common Configuration"]
    pub cconf: CCONF,
    #[doc = "0x62 - USB Transmit Dynamic FIFO Sizing"]
    pub txfifosz: TXFIFOSZ,
    #[doc = "0x63 - USB Receive Dynamic FIFO Sizing"]
    pub rxfifosz: RXFIFOSZ,
    #[doc = "0x64 - USB Transmit FIFO Start Address"]
    pub txfifoadd: TXFIFOADD,
    #[doc = "0x66 - USB Receive FIFO Start Address"]
    pub rxfifoadd: RXFIFOADD,
    _reserved2: [u8; 8usize],
    #[doc = "0x70 - USB ULPI VBUS Control"]
    pub ulpivbusctl: ULPIVBUSCTL,
    _reserved3: [u8; 3usize],
    #[doc = "0x74 - USB ULPI Register Data"]
    pub ulpiregdata: ULPIREGDATA,
    #[doc = "0x75 - USB ULPI Register Address"]
    pub ulpiregaddr: ULPIREGADDR,
    #[doc = "0x76 - USB ULPI Register Control"]
    pub ulpiregctl: ULPIREGCTL,
    _reserved4: [u8; 1usize],
    #[doc = "0x78 - USB Endpoint Information"]
    pub epinfo: EPINFO,
    #[doc = "0x79 - USB RAM Information"]
    pub raminfo: RAMINFO,
    #[doc = "0x7a - USB Connect Timing"]
    pub contim: CONTIM,
    #[doc = "0x7b - USB OTG VBUS Pulse Timing"]
    pub vplen: VPLEN,
    #[doc = "0x7c - USB High-Speed Last Transaction to End of Frame Timing"]
    pub hseof: HSEOF,
    #[doc = "0x7d - USB Full-Speed Last Transaction to End of Frame Timing"]
    pub fseof: FSEOF,
    #[doc = "0x7e - USB Low-Speed Last Transaction to End of Frame Timing"]
    pub lseof: LSEOF,
    _reserved5: [u8; 1usize],
    #[doc = "0x80 - USB Transmit Functional Address Endpoint 0"]
    pub txfuncaddr0: TXFUNCADDR0,
    _reserved6: [u8; 1usize],
    #[doc = "0x82 - USB Transmit Hub Address Endpoint 0"]
    pub txhubaddr0: TXHUBADDR0,
    #[doc = "0x83 - USB Transmit Hub Port Endpoint 0"]
    pub txhubport0: TXHUBPORT0,
    _reserved7: [u8; 4usize],
    #[doc = "0x88 - USB Transmit Functional Address Endpoint 1"]
    pub txfuncaddr1: TXFUNCADDR1,
    _reserved8: [u8; 1usize],
    #[doc = "0x8a - USB Transmit Hub Address Endpoint 1"]
    pub txhubaddr1: TXHUBADDR1,
    #[doc = "0x8b - USB Transmit Hub Port Endpoint 1"]
    pub txhubport1: TXHUBPORT1,
    #[doc = "0x8c - USB Receive Functional Address Endpoint 1"]
    pub rxfuncaddr1: RXFUNCADDR1,
    _reserved9: [u8; 1usize],
    #[doc = "0x8e - USB Receive Hub Address Endpoint 1"]
    pub rxhubaddr1: RXHUBADDR1,
    #[doc = "0x8f - USB Receive Hub Port Endpoint 1"]
    pub rxhubport1: RXHUBPORT1,
    #[doc = "0x90 - USB Transmit Functional Address Endpoint 2"]
    pub txfuncaddr2: TXFUNCADDR2,
    _reserved10: [u8; 1usize],
    #[doc = "0x92 - USB Transmit Hub Address Endpoint 2"]
    pub txhubaddr2: TXHUBADDR2,
    #[doc = "0x93 - USB Transmit Hub Port Endpoint 2"]
    pub txhubport2: TXHUBPORT2,
    #[doc = "0x94 - USB Receive Functional Address Endpoint 2"]
    pub rxfuncaddr2: RXFUNCADDR2,
    _reserved11: [u8; 1usize],
    #[doc = "0x96 - USB Receive Hub Address Endpoint 2"]
    pub rxhubaddr2: RXHUBADDR2,
    #[doc = "0x97 - USB Receive Hub Port Endpoint 2"]
    pub rxhubport2: RXHUBPORT2,
    #[doc = "0x98 - USB Transmit Functional Address Endpoint 3"]
    pub txfuncaddr3: TXFUNCADDR3,
    _reserved12: [u8; 1usize],
    #[doc = "0x9a - USB Transmit Hub Address Endpoint 3"]
    pub txhubaddr3: TXHUBADDR3,
    #[doc = "0x9b - USB Transmit Hub Port Endpoint 3"]
    pub txhubport3: TXHUBPORT3,
    #[doc = "0x9c - USB Receive Functional Address Endpoint 3"]
    pub rxfuncaddr3: RXFUNCADDR3,
    _reserved13: [u8; 1usize],
    #[doc = "0x9e - USB Receive Hub Address Endpoint 3"]
    pub rxhubaddr3: RXHUBADDR3,
    #[doc = "0x9f - USB Receive Hub Port Endpoint 3"]
    pub rxhubport3: RXHUBPORT3,
    #[doc = "0xa0 - USB Transmit Functional Address Endpoint 4"]
    pub txfuncaddr4: TXFUNCADDR4,
    _reserved14: [u8; 1usize],
    #[doc = "0xa2 - USB Transmit Hub Address Endpoint 4"]
    pub txhubaddr4: TXHUBADDR4,
    #[doc = "0xa3 - USB Transmit Hub Port Endpoint 4"]
    pub txhubport4: TXHUBPORT4,
    #[doc = "0xa4 - USB Receive Functional Address Endpoint 4"]
    pub rxfuncaddr4: RXFUNCADDR4,
    _reserved15: [u8; 1usize],
    #[doc = "0xa6 - USB Receive Hub Address Endpoint 4"]
    pub rxhubaddr4: RXHUBADDR4,
    #[doc = "0xa7 - USB Receive Hub Port Endpoint 4"]
    pub rxhubport4: RXHUBPORT4,
    #[doc = "0xa8 - USB Transmit Functional Address Endpoint 5"]
    pub txfuncaddr5: TXFUNCADDR5,
    _reserved16: [u8; 1usize],
    #[doc = "0xaa - USB Transmit Hub Address Endpoint 5"]
    pub txhubaddr5: TXHUBADDR5,
    #[doc = "0xab - USB Transmit Hub Port Endpoint 5"]
    pub txhubport5: TXHUBPORT5,
    #[doc = "0xac - USB Receive Functional Address Endpoint 5"]
    pub rxfuncaddr5: RXFUNCADDR5,
    _reserved17: [u8; 1usize],
    #[doc = "0xae - USB Receive Hub Address Endpoint 5"]
    pub rxhubaddr5: RXHUBADDR5,
    #[doc = "0xaf - USB Receive Hub Port Endpoint 5"]
    pub rxhubport5: RXHUBPORT5,
    #[doc = "0xb0 - USB Transmit Functional Address Endpoint 6"]
    pub txfuncaddr6: TXFUNCADDR6,
    _reserved18: [u8; 1usize],
    #[doc = "0xb2 - USB Transmit Hub Address Endpoint 6"]
    pub txhubaddr6: TXHUBADDR6,
    #[doc = "0xb3 - USB Transmit Hub Port Endpoint 6"]
    pub txhubport6: TXHUBPORT6,
    #[doc = "0xb4 - USB Receive Functional Address Endpoint 6"]
    pub rxfuncaddr6: RXFUNCADDR6,
    _reserved19: [u8; 1usize],
    #[doc = "0xb6 - USB Receive Hub Address Endpoint 6"]
    pub rxhubaddr6: RXHUBADDR6,
    #[doc = "0xb7 - USB Receive Hub Port Endpoint 6"]
    pub rxhubport6: RXHUBPORT6,
    #[doc = "0xb8 - USB Transmit Functional Address Endpoint 7"]
    pub txfuncaddr7: TXFUNCADDR7,
    _reserved20: [u8; 1usize],
    #[doc = "0xba - USB Transmit Hub Address Endpoint 7"]
    pub txhubaddr7: TXHUBADDR7,
    #[doc = "0xbb - USB Transmit Hub Port Endpoint 7"]
    pub txhubport7: TXHUBPORT7,
    #[doc = "0xbc - USB Receive Functional Address Endpoint 7"]
    pub rxfuncaddr7: RXFUNCADDR7,
    _reserved21: [u8; 1usize],
    #[doc = "0xbe - USB Receive Hub Address Endpoint 7"]
    pub rxhubaddr7: RXHUBADDR7,
    #[doc = "0xbf - USB Receive Hub Port Endpoint 7"]
    pub rxhubport7: RXHUBPORT7,
    _reserved22: [u8; 66usize],
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    pub csrl0: CSRL0,
    #[doc = "0x103 - USB Control and Status Endpoint 0 High"]
    pub csrh0: CSRH0,
    _reserved23: [u8; 4usize],
    #[doc = "0x108 - USB Receive Byte Count Endpoint 0"]
    pub count0: COUNT0,
    _reserved24: [u8; 1usize],
    #[doc = "0x10a - USB Type Endpoint 0"]
    pub type0: TYPE0,
    #[doc = "0x10b - USB NAK Limit"]
    pub naklmt: NAKLMT,
    _reserved25: [u8; 4usize],
    #[doc = "0x110 - USB Maximum Transmit Data Endpoint 1"]
    pub txmaxp1: TXMAXP1,
    #[doc = "0x112 - USB Transmit Control and Status Endpoint 1 Low"]
    pub txcsrl1: TXCSRL1,
    #[doc = "0x113 - USB Transmit Control and Status Endpoint 1 High"]
    pub txcsrh1: TXCSRH1,
    #[doc = "0x114 - USB Maximum Receive Data Endpoint 1"]
    pub rxmaxp1: RXMAXP1,
    #[doc = "0x116 - USB Receive Control and Status Endpoint 1 Low"]
    pub rxcsrl1: RXCSRL1,
    #[doc = "0x117 - USB Receive Control and Status Endpoint 1 High"]
    pub rxcsrh1: RXCSRH1,
    #[doc = "0x118 - USB Receive Byte Count Endpoint 1"]
    pub rxcount1: RXCOUNT1,
    #[doc = "0x11a - USB Host Transmit Configure Type Endpoint 1"]
    pub txtype1: TXTYPE1,
    #[doc = "0x11b - USB Host Transmit Interval Endpoint 1"]
    pub txinterval1: TXINTERVAL1,
    #[doc = "0x11c - USB Host Configure Receive Type Endpoint 1"]
    pub rxtype1: RXTYPE1,
    #[doc = "0x11d - USB Host Receive Polling Interval Endpoint 1"]
    pub rxinterval1: RXINTERVAL1,
    _reserved26: [u8; 2usize],
    #[doc = "0x120 - USB Maximum Transmit Data Endpoint 2"]
    pub txmaxp2: TXMAXP2,
    #[doc = "0x122 - USB Transmit Control and Status Endpoint 2 Low"]
    pub txcsrl2: TXCSRL2,
    #[doc = "0x123 - USB Transmit Control and Status Endpoint 2 High"]
    pub txcsrh2: TXCSRH2,
    #[doc = "0x124 - USB Maximum Receive Data Endpoint 2"]
    pub rxmaxp2: RXMAXP2,
    #[doc = "0x126 - USB Receive Control and Status Endpoint 2 Low"]
    pub rxcsrl2: RXCSRL2,
    #[doc = "0x127 - USB Receive Control and Status Endpoint 2 High"]
    pub rxcsrh2: RXCSRH2,
    #[doc = "0x128 - USB Receive Byte Count Endpoint 2"]
    pub rxcount2: RXCOUNT2,
    #[doc = "0x12a - USB Host Transmit Configure Type Endpoint 2"]
    pub txtype2: TXTYPE2,
    #[doc = "0x12b - USB Host Transmit Interval Endpoint 2"]
    pub txinterval2: TXINTERVAL2,
    #[doc = "0x12c - USB Host Configure Receive Type Endpoint 2"]
    pub rxtype2: RXTYPE2,
    #[doc = "0x12d - USB Host Receive Polling Interval Endpoint 2"]
    pub rxinterval2: RXINTERVAL2,
    _reserved27: [u8; 2usize],
    #[doc = "0x130 - USB Maximum Transmit Data Endpoint 3"]
    pub txmaxp3: TXMAXP3,
    #[doc = "0x132 - USB Transmit Control and Status Endpoint 3 Low"]
    pub txcsrl3: TXCSRL3,
    #[doc = "0x133 - USB Transmit Control and Status Endpoint 3 High"]
    pub txcsrh3: TXCSRH3,
    #[doc = "0x134 - USB Maximum Receive Data Endpoint 3"]
    pub rxmaxp3: RXMAXP3,
    #[doc = "0x136 - USB Receive Control and Status Endpoint 3 Low"]
    pub rxcsrl3: RXCSRL3,
    #[doc = "0x137 - USB Receive Control and Status Endpoint 3 High"]
    pub rxcsrh3: RXCSRH3,
    #[doc = "0x138 - USB Receive Byte Count Endpoint 3"]
    pub rxcount3: RXCOUNT3,
    #[doc = "0x13a - USB Host Transmit Configure Type Endpoint 3"]
    pub txtype3: TXTYPE3,
    #[doc = "0x13b - USB Host Transmit Interval Endpoint 3"]
    pub txinterval3: TXINTERVAL3,
    #[doc = "0x13c - USB Host Configure Receive Type Endpoint 3"]
    pub rxtype3: RXTYPE3,
    #[doc = "0x13d - USB Host Receive Polling Interval Endpoint 3"]
    pub rxinterval3: RXINTERVAL3,
    _reserved28: [u8; 2usize],
    #[doc = "0x140 - USB Maximum Transmit Data Endpoint 4"]
    pub txmaxp4: TXMAXP4,
    #[doc = "0x142 - USB Transmit Control and Status Endpoint 4 Low"]
    pub txcsrl4: TXCSRL4,
    #[doc = "0x143 - USB Transmit Control and Status Endpoint 4 High"]
    pub txcsrh4: TXCSRH4,
    #[doc = "0x144 - USB Maximum Receive Data Endpoint 4"]
    pub rxmaxp4: RXMAXP4,
    #[doc = "0x146 - USB Receive Control and Status Endpoint 4 Low"]
    pub rxcsrl4: RXCSRL4,
    #[doc = "0x147 - USB Receive Control and Status Endpoint 4 High"]
    pub rxcsrh4: RXCSRH4,
    #[doc = "0x148 - USB Receive Byte Count Endpoint 4"]
    pub rxcount4: RXCOUNT4,
    #[doc = "0x14a - USB Host Transmit Configure Type Endpoint 4"]
    pub txtype4: TXTYPE4,
    #[doc = "0x14b - USB Host Transmit Interval Endpoint 4"]
    pub txinterval4: TXINTERVAL4,
    #[doc = "0x14c - USB Host Configure Receive Type Endpoint 4"]
    pub rxtype4: RXTYPE4,
    #[doc = "0x14d - USB Host Receive Polling Interval Endpoint 4"]
    pub rxinterval4: RXINTERVAL4,
    _reserved29: [u8; 2usize],
    #[doc = "0x150 - USB Maximum Transmit Data Endpoint 5"]
    pub txmaxp5: TXMAXP5,
    #[doc = "0x152 - USB Transmit Control and Status Endpoint 5 Low"]
    pub txcsrl5: TXCSRL5,
    #[doc = "0x153 - USB Transmit Control and Status Endpoint 5 High"]
    pub txcsrh5: TXCSRH5,
    #[doc = "0x154 - USB Maximum Receive Data Endpoint 5"]
    pub rxmaxp5: RXMAXP5,
    #[doc = "0x156 - USB Receive Control and Status Endpoint 5 Low"]
    pub rxcsrl5: RXCSRL5,
    #[doc = "0x157 - USB Receive Control and Status Endpoint 5 High"]
    pub rxcsrh5: RXCSRH5,
    #[doc = "0x158 - USB Receive Byte Count Endpoint 5"]
    pub rxcount5: RXCOUNT5,
    #[doc = "0x15a - USB Host Transmit Configure Type Endpoint 5"]
    pub txtype5: TXTYPE5,
    #[doc = "0x15b - USB Host Transmit Interval Endpoint 5"]
    pub txinterval5: TXINTERVAL5,
    #[doc = "0x15c - USB Host Configure Receive Type Endpoint 5"]
    pub rxtype5: RXTYPE5,
    #[doc = "0x15d - USB Host Receive Polling Interval Endpoint 5"]
    pub rxinterval5: RXINTERVAL5,
    _reserved30: [u8; 2usize],
    #[doc = "0x160 - USB Maximum Transmit Data Endpoint 6"]
    pub txmaxp6: TXMAXP6,
    #[doc = "0x162 - USB Transmit Control and Status Endpoint 6 Low"]
    pub txcsrl6: TXCSRL6,
    #[doc = "0x163 - USB Transmit Control and Status Endpoint 6 High"]
    pub txcsrh6: TXCSRH6,
    #[doc = "0x164 - USB Maximum Receive Data Endpoint 6"]
    pub rxmaxp6: RXMAXP6,
    #[doc = "0x166 - USB Receive Control and Status Endpoint 6 Low"]
    pub rxcsrl6: RXCSRL6,
    #[doc = "0x167 - USB Receive Control and Status Endpoint 6 High"]
    pub rxcsrh6: RXCSRH6,
    #[doc = "0x168 - USB Receive Byte Count Endpoint 6"]
    pub rxcount6: RXCOUNT6,
    #[doc = "0x16a - USB Host Transmit Configure Type Endpoint 6"]
    pub txtype6: TXTYPE6,
    #[doc = "0x16b - USB Host Transmit Interval Endpoint 6"]
    pub txinterval6: TXINTERVAL6,
    #[doc = "0x16c - USB Host Configure Receive Type Endpoint 6"]
    pub rxtype6: RXTYPE6,
    #[doc = "0x16d - USB Host Receive Polling Interval Endpoint 6"]
    pub rxinterval6: RXINTERVAL6,
    _reserved31: [u8; 2usize],
    #[doc = "0x170 - USB Maximum Transmit Data Endpoint 7"]
    pub txmaxp7: TXMAXP7,
    #[doc = "0x172 - USB Transmit Control and Status Endpoint 7 Low"]
    pub txcsrl7: TXCSRL7,
    #[doc = "0x173 - USB Transmit Control and Status Endpoint 7 High"]
    pub txcsrh7: TXCSRH7,
    #[doc = "0x174 - USB Maximum Receive Data Endpoint 7"]
    pub rxmaxp7: RXMAXP7,
    #[doc = "0x176 - USB Receive Control and Status Endpoint 7 Low"]
    pub rxcsrl7: RXCSRL7,
    #[doc = "0x177 - USB Receive Control and Status Endpoint 7 High"]
    pub rxcsrh7: RXCSRH7,
    #[doc = "0x178 - USB Receive Byte Count Endpoint 7"]
    pub rxcount7: RXCOUNT7,
    #[doc = "0x17a - USB Host Transmit Configure Type Endpoint 7"]
    pub txtype7: TXTYPE7,
    #[doc = "0x17b - USB Host Transmit Interval Endpoint 7"]
    pub txinterval7: TXINTERVAL7,
    #[doc = "0x17c - USB Host Configure Receive Type Endpoint 7"]
    pub rxtype7: RXTYPE7,
    #[doc = "0x17d - USB Host Receive Polling Interval Endpoint 7"]
    pub rxinterval7: RXINTERVAL7,
    _reserved32: [u8; 130usize],
    #[doc = "0x200 - USB DMA Interrupt"]
    pub dmaintr: DMAINTR,
    _reserved33: [u8; 3usize],
    #[doc = "0x204 - USB DMA Control 0"]
    pub dmactl0: DMACTL0,
    _reserved34: [u8; 2usize],
    #[doc = "0x208 - USB DMA Address 0"]
    pub dmaaddr0: DMAADDR0,
    #[doc = "0x20c - USB DMA Count 0"]
    pub dmacount0: DMACOUNT0,
    _reserved35: [u8; 4usize],
    #[doc = "0x214 - USB DMA Control 1"]
    pub dmactl1: DMACTL1,
    _reserved36: [u8; 2usize],
    #[doc = "0x218 - USB DMA Address 1"]
    pub dmaaddr1: DMAADDR1,
    #[doc = "0x21c - USB DMA Count 1"]
    pub dmacount1: DMACOUNT1,
    _reserved37: [u8; 4usize],
    #[doc = "0x224 - USB DMA Control 2"]
    pub dmactl2: DMACTL2,
    _reserved38: [u8; 2usize],
    #[doc = "0x228 - USB DMA Address 2"]
    pub dmaaddr2: DMAADDR2,
    #[doc = "0x22c - USB DMA Count 2"]
    pub dmacount2: DMACOUNT2,
    _reserved39: [u8; 4usize],
    #[doc = "0x234 - USB DMA Control 3"]
    pub dmactl3: DMACTL3,
    _reserved40: [u8; 2usize],
    #[doc = "0x238 - USB DMA Address 3"]
    pub dmaaddr3: DMAADDR3,
    #[doc = "0x23c - USB DMA Count 3"]
    pub dmacount3: DMACOUNT3,
    _reserved41: [u8; 4usize],
    #[doc = "0x244 - USB DMA Control 4"]
    pub dmactl4: DMACTL4,
    _reserved42: [u8; 2usize],
    #[doc = "0x248 - USB DMA Address 4"]
    pub dmaaddr4: DMAADDR4,
    #[doc = "0x24c - USB DMA Count 4"]
    pub dmacount4: DMACOUNT4,
    _reserved43: [u8; 4usize],
    #[doc = "0x254 - USB DMA Control 5"]
    pub dmactl5: DMACTL5,
    _reserved44: [u8; 2usize],
    #[doc = "0x258 - USB DMA Address 5"]
    pub dmaaddr5: DMAADDR5,
    #[doc = "0x25c - USB DMA Count 5"]
    pub dmacount5: DMACOUNT5,
    _reserved45: [u8; 4usize],
    #[doc = "0x264 - USB DMA Control 6"]
    pub dmactl6: DMACTL6,
    _reserved46: [u8; 2usize],
    #[doc = "0x268 - USB DMA Address 6"]
    pub dmaaddr6: DMAADDR6,
    #[doc = "0x26c - USB DMA Count 6"]
    pub dmacount6: DMACOUNT6,
    _reserved47: [u8; 4usize],
    #[doc = "0x274 - USB DMA Control 7"]
    pub dmactl7: DMACTL7,
    _reserved48: [u8; 2usize],
    #[doc = "0x278 - USB DMA Address 7"]
    pub dmaaddr7: DMAADDR7,
    #[doc = "0x27c - USB DMA Count 7"]
    pub dmacount7: DMACOUNT7,
    _reserved49: [u8; 132usize],
    #[doc = "0x304 - USB Request Packet Count in Block Transfer Endpoint 1"]
    pub rqpktcount1: RQPKTCOUNT1,
    _reserved50: [u8; 2usize],
    #[doc = "0x308 - USB Request Packet Count in Block Transfer Endpoint 2"]
    pub rqpktcount2: RQPKTCOUNT2,
    _reserved51: [u8; 2usize],
    #[doc = "0x30c - USB Request Packet Count in Block Transfer Endpoint 3"]
    pub rqpktcount3: RQPKTCOUNT3,
    _reserved52: [u8; 2usize],
    #[doc = "0x310 - USB Request Packet Count in Block Transfer Endpoint 4"]
    pub rqpktcount4: RQPKTCOUNT4,
    _reserved53: [u8; 2usize],
    #[doc = "0x314 - USB Request Packet Count in Block Transfer Endpoint 5"]
    pub rqpktcount5: RQPKTCOUNT5,
    _reserved54: [u8; 2usize],
    #[doc = "0x318 - USB Request Packet Count in Block Transfer Endpoint 6"]
    pub rqpktcount6: RQPKTCOUNT6,
    _reserved55: [u8; 2usize],
    #[doc = "0x31c - USB Request Packet Count in Block Transfer Endpoint 7"]
    pub rqpktcount7: RQPKTCOUNT7,
    _reserved56: [u8; 34usize],
    #[doc = "0x340 - USB Receive Double Packet Buffer Disable"]
    pub rxdpktbufdis: RXDPKTBUFDIS,
    #[doc = "0x342 - USB Transmit Double Packet Buffer Disable"]
    pub txdpktbufdis: TXDPKTBUFDIS,
    #[doc = "0x344 - USB Chirp Timeout"]
    pub cto: CTO,
    #[doc = "0x346 - USB High Speed to UTM Operating Delay"]
    pub hhsrtn: HHSRTN,
    #[doc = "0x348 - USB High Speed Time-out Adder"]
    pub hsbt: HSBT,
    _reserved57: [u8; 22usize],
    #[doc = "0x360 - USB LPM Attributes"]
    pub lpmattr: LPMATTR,
    #[doc = "0x362 - USB LPM Control"]
    pub lpmcntrl: LPMCNTRL,
    #[doc = "0x363 - USB LPM Interrupt Mask"]
    pub lpmim: LPMIM,
    #[doc = "0x364 - USB LPM Raw Interrupt Status"]
    pub lpmris: LPMRIS,
    #[doc = "0x365 - USB LPM Function Address"]
    pub lpmfaddr: LPMFADDR,
    _reserved58: [u8; 154usize],
    #[doc = "0x400 - USB External Power Control"]
    pub epc: EPC,
    #[doc = "0x404 - USB External Power Control Raw Interrupt Status"]
    pub epcris: EPCRIS,
    #[doc = "0x408 - USB External Power Control Interrupt Mask"]
    pub epcim: EPCIM,
    #[doc = "0x40c - USB External Power Control Interrupt Status and Clear"]
    pub epcisc: EPCISC,
    #[doc = "0x410 - USB Device RESUME Raw Interrupt Status"]
    pub drris: DRRIS,
    #[doc = "0x414 - USB Device RESUME Interrupt Mask"]
    pub drim: DRIM,
    #[doc = "0x418 - USB Device RESUME Interrupt Status and Clear"]
    pub drisc: DRISC,
    #[doc = "0x41c - USB General-Purpose Control and Status"]
    pub gpcs: GPCS,
    _reserved59: [u8; 16usize],
    #[doc = "0x430 - USB VBUS Droop Control"]
    pub vdc: VDC,
    #[doc = "0x434 - USB VBUS Droop Control Raw Interrupt Status"]
    pub vdcris: VDCRIS,
    #[doc = "0x438 - USB VBUS Droop Control Interrupt Mask"]
    pub vdcim: VDCIM,
    #[doc = "0x43c - USB VBUS Droop Control Interrupt Status and Clear"]
    pub vdcisc: VDCISC,
    _reserved60: [u8; 2944usize],
    #[doc = "0xfc0 - USB Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - USB Peripheral Configuration"]
    pub pc: PC,
    #[doc = "0xfc8 - USB Clock Configuration"]
    pub cc: CC,
}
#[doc = "USB Device Functional Address"]
pub struct FADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Functional Address"]
pub mod faddr;
#[doc = "USB Power"]
pub struct POWER {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Power"]
pub mod power;
#[doc = "USB Transmit Interrupt Status"]
pub struct TXIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Transmit Interrupt Status"]
pub mod txis;
#[doc = "USB Receive Interrupt Status"]
pub struct RXIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Interrupt Status"]
pub mod rxis;
#[doc = "USB Transmit Interrupt Enable"]
pub struct TXIE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Transmit Interrupt Enable"]
pub mod txie;
#[doc = "USB Receive Interrupt Enable"]
pub struct RXIE {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Interrupt Enable"]
pub mod rxie;
#[doc = "USB General Interrupt Status"]
pub struct IS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB General Interrupt Status"]
pub mod is;
#[doc = "USB Interrupt Enable"]
pub struct IE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Interrupt Enable"]
pub mod ie;
#[doc = "USB Frame Value"]
pub struct FRAME {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Frame Value"]
pub mod frame;
#[doc = "USB Endpoint Index"]
pub struct EPIDX {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Index"]
pub mod epidx;
#[doc = "USB Test Mode"]
pub struct TEST {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Test Mode"]
pub mod test;
#[doc = "USB FIFO Endpoint 0"]
pub struct FIFO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 0"]
pub mod fifo0;
#[doc = "USB FIFO Endpoint 1"]
pub struct FIFO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 1"]
pub mod fifo1;
#[doc = "USB FIFO Endpoint 2"]
pub struct FIFO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 2"]
pub mod fifo2;
#[doc = "USB FIFO Endpoint 3"]
pub struct FIFO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 3"]
pub mod fifo3;
#[doc = "USB FIFO Endpoint 4"]
pub struct FIFO4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 4"]
pub mod fifo4;
#[doc = "USB FIFO Endpoint 5"]
pub struct FIFO5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 5"]
pub mod fifo5;
#[doc = "USB FIFO Endpoint 6"]
pub struct FIFO6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 6"]
pub mod fifo6;
#[doc = "USB FIFO Endpoint 7"]
pub struct FIFO7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB FIFO Endpoint 7"]
pub mod fifo7;
#[doc = "USB Device Control"]
pub struct DEVCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Device Control"]
pub mod devctl;
#[doc = "USB Common Configuration"]
pub struct CCONF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Common Configuration"]
pub mod cconf;
#[doc = "USB Transmit Dynamic FIFO Sizing"]
pub struct TXFIFOSZ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Dynamic FIFO Sizing"]
pub mod txfifosz;
#[doc = "USB Receive Dynamic FIFO Sizing"]
pub struct RXFIFOSZ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Dynamic FIFO Sizing"]
pub mod rxfifosz;
#[doc = "USB Transmit FIFO Start Address"]
pub struct TXFIFOADD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Transmit FIFO Start Address"]
pub mod txfifoadd;
#[doc = "USB Receive FIFO Start Address"]
pub struct RXFIFOADD {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive FIFO Start Address"]
pub mod rxfifoadd;
#[doc = "USB ULPI VBUS Control"]
pub struct ULPIVBUSCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB ULPI VBUS Control"]
pub mod ulpivbusctl;
#[doc = "USB ULPI Register Data"]
pub struct ULPIREGDATA {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB ULPI Register Data"]
pub mod ulpiregdata;
#[doc = "USB ULPI Register Address"]
pub struct ULPIREGADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB ULPI Register Address"]
pub mod ulpiregaddr;
#[doc = "USB ULPI Register Control"]
pub struct ULPIREGCTL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB ULPI Register Control"]
pub mod ulpiregctl;
#[doc = "USB Endpoint Information"]
pub struct EPINFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Endpoint Information"]
pub mod epinfo;
#[doc = "USB RAM Information"]
pub struct RAMINFO {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB RAM Information"]
pub mod raminfo;
#[doc = "USB Connect Timing"]
pub struct CONTIM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Connect Timing"]
pub mod contim;
#[doc = "USB OTG VBUS Pulse Timing"]
pub struct VPLEN {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB OTG VBUS Pulse Timing"]
pub mod vplen;
#[doc = "USB High-Speed Last Transaction to End of Frame Timing"]
pub struct HSEOF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB High-Speed Last Transaction to End of Frame Timing"]
pub mod hseof;
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing"]
pub struct FSEOF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing"]
pub mod fseof;
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing"]
pub struct LSEOF {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing"]
pub mod lseof;
#[doc = "USB Transmit Functional Address Endpoint 0"]
pub struct TXFUNCADDR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 0"]
pub mod txfuncaddr0;
#[doc = "USB Transmit Hub Address Endpoint 0"]
pub struct TXHUBADDR0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 0"]
pub mod txhubaddr0;
#[doc = "USB Transmit Hub Port Endpoint 0"]
pub struct TXHUBPORT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 0"]
pub mod txhubport0;
#[doc = "USB Transmit Functional Address Endpoint 1"]
pub struct TXFUNCADDR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 1"]
pub mod txfuncaddr1;
#[doc = "USB Transmit Hub Address Endpoint 1"]
pub struct TXHUBADDR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 1"]
pub mod txhubaddr1;
#[doc = "USB Transmit Hub Port Endpoint 1"]
pub struct TXHUBPORT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 1"]
pub mod txhubport1;
#[doc = "USB Receive Functional Address Endpoint 1"]
pub struct RXFUNCADDR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 1"]
pub mod rxfuncaddr1;
#[doc = "USB Receive Hub Address Endpoint 1"]
pub struct RXHUBADDR1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 1"]
pub mod rxhubaddr1;
#[doc = "USB Receive Hub Port Endpoint 1"]
pub struct RXHUBPORT1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 1"]
pub mod rxhubport1;
#[doc = "USB Transmit Functional Address Endpoint 2"]
pub struct TXFUNCADDR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 2"]
pub mod txfuncaddr2;
#[doc = "USB Transmit Hub Address Endpoint 2"]
pub struct TXHUBADDR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 2"]
pub mod txhubaddr2;
#[doc = "USB Transmit Hub Port Endpoint 2"]
pub struct TXHUBPORT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 2"]
pub mod txhubport2;
#[doc = "USB Receive Functional Address Endpoint 2"]
pub struct RXFUNCADDR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 2"]
pub mod rxfuncaddr2;
#[doc = "USB Receive Hub Address Endpoint 2"]
pub struct RXHUBADDR2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 2"]
pub mod rxhubaddr2;
#[doc = "USB Receive Hub Port Endpoint 2"]
pub struct RXHUBPORT2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 2"]
pub mod rxhubport2;
#[doc = "USB Transmit Functional Address Endpoint 3"]
pub struct TXFUNCADDR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 3"]
pub mod txfuncaddr3;
#[doc = "USB Transmit Hub Address Endpoint 3"]
pub struct TXHUBADDR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 3"]
pub mod txhubaddr3;
#[doc = "USB Transmit Hub Port Endpoint 3"]
pub struct TXHUBPORT3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 3"]
pub mod txhubport3;
#[doc = "USB Receive Functional Address Endpoint 3"]
pub struct RXFUNCADDR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 3"]
pub mod rxfuncaddr3;
#[doc = "USB Receive Hub Address Endpoint 3"]
pub struct RXHUBADDR3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 3"]
pub mod rxhubaddr3;
#[doc = "USB Receive Hub Port Endpoint 3"]
pub struct RXHUBPORT3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 3"]
pub mod rxhubport3;
#[doc = "USB Transmit Functional Address Endpoint 4"]
pub struct TXFUNCADDR4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 4"]
pub mod txfuncaddr4;
#[doc = "USB Transmit Hub Address Endpoint 4"]
pub struct TXHUBADDR4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 4"]
pub mod txhubaddr4;
#[doc = "USB Transmit Hub Port Endpoint 4"]
pub struct TXHUBPORT4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 4"]
pub mod txhubport4;
#[doc = "USB Receive Functional Address Endpoint 4"]
pub struct RXFUNCADDR4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 4"]
pub mod rxfuncaddr4;
#[doc = "USB Receive Hub Address Endpoint 4"]
pub struct RXHUBADDR4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 4"]
pub mod rxhubaddr4;
#[doc = "USB Receive Hub Port Endpoint 4"]
pub struct RXHUBPORT4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 4"]
pub mod rxhubport4;
#[doc = "USB Transmit Functional Address Endpoint 5"]
pub struct TXFUNCADDR5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 5"]
pub mod txfuncaddr5;
#[doc = "USB Transmit Hub Address Endpoint 5"]
pub struct TXHUBADDR5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 5"]
pub mod txhubaddr5;
#[doc = "USB Transmit Hub Port Endpoint 5"]
pub struct TXHUBPORT5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 5"]
pub mod txhubport5;
#[doc = "USB Receive Functional Address Endpoint 5"]
pub struct RXFUNCADDR5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 5"]
pub mod rxfuncaddr5;
#[doc = "USB Receive Hub Address Endpoint 5"]
pub struct RXHUBADDR5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 5"]
pub mod rxhubaddr5;
#[doc = "USB Receive Hub Port Endpoint 5"]
pub struct RXHUBPORT5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 5"]
pub mod rxhubport5;
#[doc = "USB Transmit Functional Address Endpoint 6"]
pub struct TXFUNCADDR6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 6"]
pub mod txfuncaddr6;
#[doc = "USB Transmit Hub Address Endpoint 6"]
pub struct TXHUBADDR6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 6"]
pub mod txhubaddr6;
#[doc = "USB Transmit Hub Port Endpoint 6"]
pub struct TXHUBPORT6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 6"]
pub mod txhubport6;
#[doc = "USB Receive Functional Address Endpoint 6"]
pub struct RXFUNCADDR6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 6"]
pub mod rxfuncaddr6;
#[doc = "USB Receive Hub Address Endpoint 6"]
pub struct RXHUBADDR6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 6"]
pub mod rxhubaddr6;
#[doc = "USB Receive Hub Port Endpoint 6"]
pub struct RXHUBPORT6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 6"]
pub mod rxhubport6;
#[doc = "USB Transmit Functional Address Endpoint 7"]
pub struct TXFUNCADDR7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Functional Address Endpoint 7"]
pub mod txfuncaddr7;
#[doc = "USB Transmit Hub Address Endpoint 7"]
pub struct TXHUBADDR7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Address Endpoint 7"]
pub mod txhubaddr7;
#[doc = "USB Transmit Hub Port Endpoint 7"]
pub struct TXHUBPORT7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Hub Port Endpoint 7"]
pub mod txhubport7;
#[doc = "USB Receive Functional Address Endpoint 7"]
pub struct RXFUNCADDR7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Functional Address Endpoint 7"]
pub mod rxfuncaddr7;
#[doc = "USB Receive Hub Address Endpoint 7"]
pub struct RXHUBADDR7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Address Endpoint 7"]
pub mod rxhubaddr7;
#[doc = "USB Receive Hub Port Endpoint 7"]
pub struct RXHUBPORT7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Hub Port Endpoint 7"]
pub mod rxhubport7;
#[doc = "USB Control and Status Endpoint 0 Low"]
pub struct CSRL0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod csrl0;
#[doc = "USB Control and Status Endpoint 0 High"]
pub struct CSRH0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Control and Status Endpoint 0 High"]
pub mod csrh0;
#[doc = "USB Receive Byte Count Endpoint 0"]
pub struct COUNT0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Byte Count Endpoint 0"]
pub mod count0;
#[doc = "USB Type Endpoint 0"]
pub struct TYPE0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Type Endpoint 0"]
pub mod type0;
#[doc = "USB NAK Limit"]
pub struct NAKLMT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB NAK Limit"]
pub mod naklmt;
#[doc = "USB Maximum Transmit Data Endpoint 1"]
pub struct TXMAXP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 1"]
pub mod txmaxp1;
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub struct TXCSRL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod txcsrl1;
#[doc = "USB Transmit Control and Status Endpoint 1 High"]
pub struct TXCSRH1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 1 High"]
pub mod txcsrh1;
#[doc = "USB Maximum Receive Data Endpoint 1"]
pub struct RXMAXP1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 1"]
pub mod rxmaxp1;
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub struct RXCSRL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod rxcsrl1;
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub struct RXCSRH1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod rxcsrh1;
#[doc = "USB Receive Byte Count Endpoint 1"]
pub struct RXCOUNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 1"]
pub mod rxcount1;
#[doc = "USB Host Transmit Configure Type Endpoint 1"]
pub struct TXTYPE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 1"]
pub mod txtype1;
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub struct TXINTERVAL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod txinterval1;
#[doc = "USB Host Configure Receive Type Endpoint 1"]
pub struct RXTYPE1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 1"]
pub mod rxtype1;
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub struct RXINTERVAL1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod rxinterval1;
#[doc = "USB Maximum Transmit Data Endpoint 2"]
pub struct TXMAXP2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 2"]
pub mod txmaxp2;
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub struct TXCSRL2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod txcsrl2;
#[doc = "USB Transmit Control and Status Endpoint 2 High"]
pub struct TXCSRH2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 2 High"]
pub mod txcsrh2;
#[doc = "USB Maximum Receive Data Endpoint 2"]
pub struct RXMAXP2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 2"]
pub mod rxmaxp2;
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub struct RXCSRL2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod rxcsrl2;
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub struct RXCSRH2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod rxcsrh2;
#[doc = "USB Receive Byte Count Endpoint 2"]
pub struct RXCOUNT2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 2"]
pub mod rxcount2;
#[doc = "USB Host Transmit Configure Type Endpoint 2"]
pub struct TXTYPE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 2"]
pub mod txtype2;
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub struct TXINTERVAL2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod txinterval2;
#[doc = "USB Host Configure Receive Type Endpoint 2"]
pub struct RXTYPE2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 2"]
pub mod rxtype2;
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub struct RXINTERVAL2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod rxinterval2;
#[doc = "USB Maximum Transmit Data Endpoint 3"]
pub struct TXMAXP3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 3"]
pub mod txmaxp3;
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub struct TXCSRL3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod txcsrl3;
#[doc = "USB Transmit Control and Status Endpoint 3 High"]
pub struct TXCSRH3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 3 High"]
pub mod txcsrh3;
#[doc = "USB Maximum Receive Data Endpoint 3"]
pub struct RXMAXP3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 3"]
pub mod rxmaxp3;
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub struct RXCSRL3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod rxcsrl3;
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub struct RXCSRH3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod rxcsrh3;
#[doc = "USB Receive Byte Count Endpoint 3"]
pub struct RXCOUNT3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 3"]
pub mod rxcount3;
#[doc = "USB Host Transmit Configure Type Endpoint 3"]
pub struct TXTYPE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 3"]
pub mod txtype3;
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub struct TXINTERVAL3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod txinterval3;
#[doc = "USB Host Configure Receive Type Endpoint 3"]
pub struct RXTYPE3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 3"]
pub mod rxtype3;
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub struct RXINTERVAL3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod rxinterval3;
#[doc = "USB Maximum Transmit Data Endpoint 4"]
pub struct TXMAXP4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 4"]
pub mod txmaxp4;
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub struct TXCSRL4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod txcsrl4;
#[doc = "USB Transmit Control and Status Endpoint 4 High"]
pub struct TXCSRH4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 4 High"]
pub mod txcsrh4;
#[doc = "USB Maximum Receive Data Endpoint 4"]
pub struct RXMAXP4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 4"]
pub mod rxmaxp4;
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub struct RXCSRL4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod rxcsrl4;
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub struct RXCSRH4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod rxcsrh4;
#[doc = "USB Receive Byte Count Endpoint 4"]
pub struct RXCOUNT4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 4"]
pub mod rxcount4;
#[doc = "USB Host Transmit Configure Type Endpoint 4"]
pub struct TXTYPE4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 4"]
pub mod txtype4;
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub struct TXINTERVAL4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod txinterval4;
#[doc = "USB Host Configure Receive Type Endpoint 4"]
pub struct RXTYPE4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 4"]
pub mod rxtype4;
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub struct RXINTERVAL4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod rxinterval4;
#[doc = "USB Maximum Transmit Data Endpoint 5"]
pub struct TXMAXP5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 5"]
pub mod txmaxp5;
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub struct TXCSRL5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod txcsrl5;
#[doc = "USB Transmit Control and Status Endpoint 5 High"]
pub struct TXCSRH5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 5 High"]
pub mod txcsrh5;
#[doc = "USB Maximum Receive Data Endpoint 5"]
pub struct RXMAXP5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 5"]
pub mod rxmaxp5;
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub struct RXCSRL5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod rxcsrl5;
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub struct RXCSRH5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod rxcsrh5;
#[doc = "USB Receive Byte Count Endpoint 5"]
pub struct RXCOUNT5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 5"]
pub mod rxcount5;
#[doc = "USB Host Transmit Configure Type Endpoint 5"]
pub struct TXTYPE5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 5"]
pub mod txtype5;
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub struct TXINTERVAL5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod txinterval5;
#[doc = "USB Host Configure Receive Type Endpoint 5"]
pub struct RXTYPE5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 5"]
pub mod rxtype5;
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub struct RXINTERVAL5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod rxinterval5;
#[doc = "USB Maximum Transmit Data Endpoint 6"]
pub struct TXMAXP6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 6"]
pub mod txmaxp6;
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub struct TXCSRL6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod txcsrl6;
#[doc = "USB Transmit Control and Status Endpoint 6 High"]
pub struct TXCSRH6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 6 High"]
pub mod txcsrh6;
#[doc = "USB Maximum Receive Data Endpoint 6"]
pub struct RXMAXP6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 6"]
pub mod rxmaxp6;
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub struct RXCSRL6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod rxcsrl6;
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub struct RXCSRH6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod rxcsrh6;
#[doc = "USB Receive Byte Count Endpoint 6"]
pub struct RXCOUNT6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 6"]
pub mod rxcount6;
#[doc = "USB Host Transmit Configure Type Endpoint 6"]
pub struct TXTYPE6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 6"]
pub mod txtype6;
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub struct TXINTERVAL6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod txinterval6;
#[doc = "USB Host Configure Receive Type Endpoint 6"]
pub struct RXTYPE6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 6"]
pub mod rxtype6;
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub struct RXINTERVAL6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod rxinterval6;
#[doc = "USB Maximum Transmit Data Endpoint 7"]
pub struct TXMAXP7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Transmit Data Endpoint 7"]
pub mod txmaxp7;
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub struct TXCSRL7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod txcsrl7;
#[doc = "USB Transmit Control and Status Endpoint 7 High"]
pub struct TXCSRH7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Transmit Control and Status Endpoint 7 High"]
pub mod txcsrh7;
#[doc = "USB Maximum Receive Data Endpoint 7"]
pub struct RXMAXP7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Maximum Receive Data Endpoint 7"]
pub mod rxmaxp7;
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub struct RXCSRL7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod rxcsrl7;
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub struct RXCSRH7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod rxcsrh7;
#[doc = "USB Receive Byte Count Endpoint 7"]
pub struct RXCOUNT7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Byte Count Endpoint 7"]
pub mod rxcount7;
#[doc = "USB Host Transmit Configure Type Endpoint 7"]
pub struct TXTYPE7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Configure Type Endpoint 7"]
pub mod txtype7;
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub struct TXINTERVAL7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod txinterval7;
#[doc = "USB Host Configure Receive Type Endpoint 7"]
pub struct RXTYPE7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Configure Receive Type Endpoint 7"]
pub mod rxtype7;
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub struct RXINTERVAL7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod rxinterval7;
#[doc = "USB DMA Interrupt"]
pub struct DMAINTR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB DMA Interrupt"]
pub mod dmaintr;
#[doc = "USB DMA Control 0"]
pub struct DMACTL0 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 0"]
pub mod dmactl0;
#[doc = "USB DMA Address 0"]
pub struct DMAADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 0"]
pub mod dmaaddr0;
#[doc = "USB DMA Count 0"]
pub struct DMACOUNT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 0"]
pub mod dmacount0;
#[doc = "USB DMA Control 1"]
pub struct DMACTL1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 1"]
pub mod dmactl1;
#[doc = "USB DMA Address 1"]
pub struct DMAADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 1"]
pub mod dmaaddr1;
#[doc = "USB DMA Count 1"]
pub struct DMACOUNT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 1"]
pub mod dmacount1;
#[doc = "USB DMA Control 2"]
pub struct DMACTL2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 2"]
pub mod dmactl2;
#[doc = "USB DMA Address 2"]
pub struct DMAADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 2"]
pub mod dmaaddr2;
#[doc = "USB DMA Count 2"]
pub struct DMACOUNT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 2"]
pub mod dmacount2;
#[doc = "USB DMA Control 3"]
pub struct DMACTL3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 3"]
pub mod dmactl3;
#[doc = "USB DMA Address 3"]
pub struct DMAADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 3"]
pub mod dmaaddr3;
#[doc = "USB DMA Count 3"]
pub struct DMACOUNT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 3"]
pub mod dmacount3;
#[doc = "USB DMA Control 4"]
pub struct DMACTL4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 4"]
pub mod dmactl4;
#[doc = "USB DMA Address 4"]
pub struct DMAADDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 4"]
pub mod dmaaddr4;
#[doc = "USB DMA Count 4"]
pub struct DMACOUNT4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 4"]
pub mod dmacount4;
#[doc = "USB DMA Control 5"]
pub struct DMACTL5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 5"]
pub mod dmactl5;
#[doc = "USB DMA Address 5"]
pub struct DMAADDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 5"]
pub mod dmaaddr5;
#[doc = "USB DMA Count 5"]
pub struct DMACOUNT5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 5"]
pub mod dmacount5;
#[doc = "USB DMA Control 6"]
pub struct DMACTL6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 6"]
pub mod dmactl6;
#[doc = "USB DMA Address 6"]
pub struct DMAADDR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 6"]
pub mod dmaaddr6;
#[doc = "USB DMA Count 6"]
pub struct DMACOUNT6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 6"]
pub mod dmacount6;
#[doc = "USB DMA Control 7"]
pub struct DMACTL7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB DMA Control 7"]
pub mod dmactl7;
#[doc = "USB DMA Address 7"]
pub struct DMAADDR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Address 7"]
pub mod dmaaddr7;
#[doc = "USB DMA Count 7"]
pub struct DMACOUNT7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Count 7"]
pub mod dmacount7;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1"]
pub struct RQPKTCOUNT1 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1"]
pub mod rqpktcount1;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2"]
pub struct RQPKTCOUNT2 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2"]
pub mod rqpktcount2;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3"]
pub struct RQPKTCOUNT3 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3"]
pub mod rqpktcount3;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4"]
pub struct RQPKTCOUNT4 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4"]
pub mod rqpktcount4;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5"]
pub struct RQPKTCOUNT5 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5"]
pub mod rqpktcount5;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6"]
pub struct RQPKTCOUNT6 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6"]
pub mod rqpktcount6;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7"]
pub struct RQPKTCOUNT7 {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7"]
pub mod rqpktcount7;
#[doc = "USB Receive Double Packet Buffer Disable"]
pub struct RXDPKTBUFDIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Receive Double Packet Buffer Disable"]
pub mod rxdpktbufdis;
#[doc = "USB Transmit Double Packet Buffer Disable"]
pub struct TXDPKTBUFDIS {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Transmit Double Packet Buffer Disable"]
pub mod txdpktbufdis;
#[doc = "USB Chirp Timeout"]
pub struct CTO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB Chirp Timeout"]
pub mod cto;
#[doc = "USB High Speed to UTM Operating Delay"]
pub struct HHSRTN {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB High Speed to UTM Operating Delay"]
pub mod hhsrtn;
#[doc = "USB High Speed Time-out Adder"]
pub struct HSBT {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB High Speed Time-out Adder"]
pub mod hsbt;
#[doc = "USB LPM Attributes"]
pub struct LPMATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "USB LPM Attributes"]
pub mod lpmattr;
#[doc = "USB LPM Control"]
pub struct LPMCNTRL {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB LPM Control"]
pub mod lpmcntrl;
#[doc = "USB LPM Interrupt Mask"]
pub struct LPMIM {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB LPM Interrupt Mask"]
pub mod lpmim;
#[doc = "USB LPM Raw Interrupt Status"]
pub struct LPMRIS {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB LPM Raw Interrupt Status"]
pub mod lpmris;
#[doc = "USB LPM Function Address"]
pub struct LPMFADDR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "USB LPM Function Address"]
pub mod lpmfaddr;
#[doc = "USB External Power Control"]
pub struct EPC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB External Power Control"]
pub mod epc;
#[doc = "USB External Power Control Raw Interrupt Status"]
pub struct EPCRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB External Power Control Raw Interrupt Status"]
pub mod epcris;
#[doc = "USB External Power Control Interrupt Mask"]
pub struct EPCIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB External Power Control Interrupt Mask"]
pub mod epcim;
#[doc = "USB External Power Control Interrupt Status and Clear"]
pub struct EPCISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB External Power Control Interrupt Status and Clear"]
pub mod epcisc;
#[doc = "USB Device RESUME Raw Interrupt Status"]
pub struct DRRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device RESUME Raw Interrupt Status"]
pub mod drris;
#[doc = "USB Device RESUME Interrupt Mask"]
pub struct DRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device RESUME Interrupt Mask"]
pub mod drim;
#[doc = "USB Device RESUME Interrupt Status and Clear"]
pub struct DRISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device RESUME Interrupt Status and Clear"]
pub mod drisc;
#[doc = "USB General-Purpose Control and Status"]
pub struct GPCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB General-Purpose Control and Status"]
pub mod gpcs;
#[doc = "USB VBUS Droop Control"]
pub struct VDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Droop Control"]
pub mod vdc;
#[doc = "USB VBUS Droop Control Raw Interrupt Status"]
pub struct VDCRIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Droop Control Raw Interrupt Status"]
pub mod vdcris;
#[doc = "USB VBUS Droop Control Interrupt Mask"]
pub struct VDCIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Droop Control Interrupt Mask"]
pub mod vdcim;
#[doc = "USB VBUS Droop Control Interrupt Status and Clear"]
pub struct VDCISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB VBUS Droop Control Interrupt Status and Clear"]
pub mod vdcisc;
#[doc = "USB Peripheral Properties"]
pub struct PP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Peripheral Properties"]
pub mod pp;
#[doc = "USB Peripheral Configuration"]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Peripheral Configuration"]
pub mod pc;
#[doc = "USB Clock Configuration"]
pub struct CC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Configuration"]
pub mod cc;
