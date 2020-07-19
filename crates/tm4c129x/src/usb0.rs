#[doc = r"Register block"]
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
    _reserved11: [u8; 16usize],
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
    _reserved19: [u8; 32usize],
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
    _reserved25: [u8; 8usize],
    #[doc = "0x70 - USB ULPI VBUS Control"]
    pub ulpivbusctl: ULPIVBUSCTL,
    _reserved26: [u8; 3usize],
    #[doc = "0x74 - USB ULPI Register Data"]
    pub ulpiregdata: ULPIREGDATA,
    #[doc = "0x75 - USB ULPI Register Address"]
    pub ulpiregaddr: ULPIREGADDR,
    #[doc = "0x76 - USB ULPI Register Control"]
    pub ulpiregctl: ULPIREGCTL,
    _reserved29: [u8; 1usize],
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
    _reserved36: [u8; 1usize],
    #[doc = "0x80 - USB Transmit Functional Address Endpoint 0"]
    pub txfuncaddr0: TXFUNCADDR0,
    _reserved37: [u8; 1usize],
    #[doc = "0x82 - USB Transmit Hub Address Endpoint 0"]
    pub txhubaddr0: TXHUBADDR0,
    #[doc = "0x83 - USB Transmit Hub Port Endpoint 0"]
    pub txhubport0: TXHUBPORT0,
    _reserved39: [u8; 4usize],
    #[doc = "0x88 - USB Transmit Functional Address Endpoint 1"]
    pub txfuncaddr1: TXFUNCADDR1,
    _reserved40: [u8; 1usize],
    #[doc = "0x8a - USB Transmit Hub Address Endpoint 1"]
    pub txhubaddr1: TXHUBADDR1,
    #[doc = "0x8b - USB Transmit Hub Port Endpoint 1"]
    pub txhubport1: TXHUBPORT1,
    #[doc = "0x8c - USB Receive Functional Address Endpoint 1"]
    pub rxfuncaddr1: RXFUNCADDR1,
    _reserved43: [u8; 1usize],
    #[doc = "0x8e - USB Receive Hub Address Endpoint 1"]
    pub rxhubaddr1: RXHUBADDR1,
    #[doc = "0x8f - USB Receive Hub Port Endpoint 1"]
    pub rxhubport1: RXHUBPORT1,
    #[doc = "0x90 - USB Transmit Functional Address Endpoint 2"]
    pub txfuncaddr2: TXFUNCADDR2,
    _reserved46: [u8; 1usize],
    #[doc = "0x92 - USB Transmit Hub Address Endpoint 2"]
    pub txhubaddr2: TXHUBADDR2,
    #[doc = "0x93 - USB Transmit Hub Port Endpoint 2"]
    pub txhubport2: TXHUBPORT2,
    #[doc = "0x94 - USB Receive Functional Address Endpoint 2"]
    pub rxfuncaddr2: RXFUNCADDR2,
    _reserved49: [u8; 1usize],
    #[doc = "0x96 - USB Receive Hub Address Endpoint 2"]
    pub rxhubaddr2: RXHUBADDR2,
    #[doc = "0x97 - USB Receive Hub Port Endpoint 2"]
    pub rxhubport2: RXHUBPORT2,
    #[doc = "0x98 - USB Transmit Functional Address Endpoint 3"]
    pub txfuncaddr3: TXFUNCADDR3,
    _reserved52: [u8; 1usize],
    #[doc = "0x9a - USB Transmit Hub Address Endpoint 3"]
    pub txhubaddr3: TXHUBADDR3,
    #[doc = "0x9b - USB Transmit Hub Port Endpoint 3"]
    pub txhubport3: TXHUBPORT3,
    #[doc = "0x9c - USB Receive Functional Address Endpoint 3"]
    pub rxfuncaddr3: RXFUNCADDR3,
    _reserved55: [u8; 1usize],
    #[doc = "0x9e - USB Receive Hub Address Endpoint 3"]
    pub rxhubaddr3: RXHUBADDR3,
    #[doc = "0x9f - USB Receive Hub Port Endpoint 3"]
    pub rxhubport3: RXHUBPORT3,
    #[doc = "0xa0 - USB Transmit Functional Address Endpoint 4"]
    pub txfuncaddr4: TXFUNCADDR4,
    _reserved58: [u8; 1usize],
    #[doc = "0xa2 - USB Transmit Hub Address Endpoint 4"]
    pub txhubaddr4: TXHUBADDR4,
    #[doc = "0xa3 - USB Transmit Hub Port Endpoint 4"]
    pub txhubport4: TXHUBPORT4,
    #[doc = "0xa4 - USB Receive Functional Address Endpoint 4"]
    pub rxfuncaddr4: RXFUNCADDR4,
    _reserved61: [u8; 1usize],
    #[doc = "0xa6 - USB Receive Hub Address Endpoint 4"]
    pub rxhubaddr4: RXHUBADDR4,
    #[doc = "0xa7 - USB Receive Hub Port Endpoint 4"]
    pub rxhubport4: RXHUBPORT4,
    #[doc = "0xa8 - USB Transmit Functional Address Endpoint 5"]
    pub txfuncaddr5: TXFUNCADDR5,
    _reserved64: [u8; 1usize],
    #[doc = "0xaa - USB Transmit Hub Address Endpoint 5"]
    pub txhubaddr5: TXHUBADDR5,
    #[doc = "0xab - USB Transmit Hub Port Endpoint 5"]
    pub txhubport5: TXHUBPORT5,
    #[doc = "0xac - USB Receive Functional Address Endpoint 5"]
    pub rxfuncaddr5: RXFUNCADDR5,
    _reserved67: [u8; 1usize],
    #[doc = "0xae - USB Receive Hub Address Endpoint 5"]
    pub rxhubaddr5: RXHUBADDR5,
    #[doc = "0xaf - USB Receive Hub Port Endpoint 5"]
    pub rxhubport5: RXHUBPORT5,
    #[doc = "0xb0 - USB Transmit Functional Address Endpoint 6"]
    pub txfuncaddr6: TXFUNCADDR6,
    _reserved70: [u8; 1usize],
    #[doc = "0xb2 - USB Transmit Hub Address Endpoint 6"]
    pub txhubaddr6: TXHUBADDR6,
    #[doc = "0xb3 - USB Transmit Hub Port Endpoint 6"]
    pub txhubport6: TXHUBPORT6,
    #[doc = "0xb4 - USB Receive Functional Address Endpoint 6"]
    pub rxfuncaddr6: RXFUNCADDR6,
    _reserved73: [u8; 1usize],
    #[doc = "0xb6 - USB Receive Hub Address Endpoint 6"]
    pub rxhubaddr6: RXHUBADDR6,
    #[doc = "0xb7 - USB Receive Hub Port Endpoint 6"]
    pub rxhubport6: RXHUBPORT6,
    #[doc = "0xb8 - USB Transmit Functional Address Endpoint 7"]
    pub txfuncaddr7: TXFUNCADDR7,
    _reserved76: [u8; 1usize],
    #[doc = "0xba - USB Transmit Hub Address Endpoint 7"]
    pub txhubaddr7: TXHUBADDR7,
    #[doc = "0xbb - USB Transmit Hub Port Endpoint 7"]
    pub txhubport7: TXHUBPORT7,
    #[doc = "0xbc - USB Receive Functional Address Endpoint 7"]
    pub rxfuncaddr7: RXFUNCADDR7,
    _reserved79: [u8; 1usize],
    #[doc = "0xbe - USB Receive Hub Address Endpoint 7"]
    pub rxhubaddr7: RXHUBADDR7,
    #[doc = "0xbf - USB Receive Hub Port Endpoint 7"]
    pub rxhubport7: RXHUBPORT7,
    _reserved81: [u8; 66usize],
    #[doc = "0x102 - USB Control and Status Endpoint 0 Low"]
    pub csrl0: CSRL0,
    #[doc = "0x103 - USB Control and Status Endpoint 0 High"]
    pub csrh0: CSRH0,
    _reserved83: [u8; 4usize],
    #[doc = "0x108 - USB Receive Byte Count Endpoint 0"]
    pub count0: COUNT0,
    _reserved84: [u8; 1usize],
    #[doc = "0x10a - USB Type Endpoint 0"]
    pub type0: TYPE0,
    #[doc = "0x10b - USB NAK Limit"]
    pub naklmt: NAKLMT,
    _reserved86: [u8; 4usize],
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
    _reserved97: [u8; 2usize],
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
    _reserved108: [u8; 2usize],
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
    _reserved119: [u8; 2usize],
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
    _reserved130: [u8; 2usize],
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
    _reserved141: [u8; 2usize],
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
    _reserved152: [u8; 2usize],
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
    _reserved163: [u8; 130usize],
    #[doc = "0x200 - USB DMA Interrupt"]
    pub dmaintr: DMAINTR,
    _reserved164: [u8; 3usize],
    #[doc = "0x204 - USB DMA Control 0"]
    pub dmactl0: DMACTL0,
    _reserved165: [u8; 2usize],
    #[doc = "0x208 - USB DMA Address 0"]
    pub dmaaddr0: DMAADDR0,
    #[doc = "0x20c - USB DMA Count 0"]
    pub dmacount0: DMACOUNT0,
    _reserved167: [u8; 4usize],
    #[doc = "0x214 - USB DMA Control 1"]
    pub dmactl1: DMACTL1,
    _reserved168: [u8; 2usize],
    #[doc = "0x218 - USB DMA Address 1"]
    pub dmaaddr1: DMAADDR1,
    #[doc = "0x21c - USB DMA Count 1"]
    pub dmacount1: DMACOUNT1,
    _reserved170: [u8; 4usize],
    #[doc = "0x224 - USB DMA Control 2"]
    pub dmactl2: DMACTL2,
    _reserved171: [u8; 2usize],
    #[doc = "0x228 - USB DMA Address 2"]
    pub dmaaddr2: DMAADDR2,
    #[doc = "0x22c - USB DMA Count 2"]
    pub dmacount2: DMACOUNT2,
    _reserved173: [u8; 4usize],
    #[doc = "0x234 - USB DMA Control 3"]
    pub dmactl3: DMACTL3,
    _reserved174: [u8; 2usize],
    #[doc = "0x238 - USB DMA Address 3"]
    pub dmaaddr3: DMAADDR3,
    #[doc = "0x23c - USB DMA Count 3"]
    pub dmacount3: DMACOUNT3,
    _reserved176: [u8; 4usize],
    #[doc = "0x244 - USB DMA Control 4"]
    pub dmactl4: DMACTL4,
    _reserved177: [u8; 2usize],
    #[doc = "0x248 - USB DMA Address 4"]
    pub dmaaddr4: DMAADDR4,
    #[doc = "0x24c - USB DMA Count 4"]
    pub dmacount4: DMACOUNT4,
    _reserved179: [u8; 4usize],
    #[doc = "0x254 - USB DMA Control 5"]
    pub dmactl5: DMACTL5,
    _reserved180: [u8; 2usize],
    #[doc = "0x258 - USB DMA Address 5"]
    pub dmaaddr5: DMAADDR5,
    #[doc = "0x25c - USB DMA Count 5"]
    pub dmacount5: DMACOUNT5,
    _reserved182: [u8; 4usize],
    #[doc = "0x264 - USB DMA Control 6"]
    pub dmactl6: DMACTL6,
    _reserved183: [u8; 2usize],
    #[doc = "0x268 - USB DMA Address 6"]
    pub dmaaddr6: DMAADDR6,
    #[doc = "0x26c - USB DMA Count 6"]
    pub dmacount6: DMACOUNT6,
    _reserved185: [u8; 4usize],
    #[doc = "0x274 - USB DMA Control 7"]
    pub dmactl7: DMACTL7,
    _reserved186: [u8; 2usize],
    #[doc = "0x278 - USB DMA Address 7"]
    pub dmaaddr7: DMAADDR7,
    #[doc = "0x27c - USB DMA Count 7"]
    pub dmacount7: DMACOUNT7,
    _reserved188: [u8; 132usize],
    #[doc = "0x304 - USB Request Packet Count in Block Transfer Endpoint 1"]
    pub rqpktcount1: RQPKTCOUNT1,
    _reserved189: [u8; 2usize],
    #[doc = "0x308 - USB Request Packet Count in Block Transfer Endpoint 2"]
    pub rqpktcount2: RQPKTCOUNT2,
    _reserved190: [u8; 2usize],
    #[doc = "0x30c - USB Request Packet Count in Block Transfer Endpoint 3"]
    pub rqpktcount3: RQPKTCOUNT3,
    _reserved191: [u8; 2usize],
    #[doc = "0x310 - USB Request Packet Count in Block Transfer Endpoint 4"]
    pub rqpktcount4: RQPKTCOUNT4,
    _reserved192: [u8; 2usize],
    #[doc = "0x314 - USB Request Packet Count in Block Transfer Endpoint 5"]
    pub rqpktcount5: RQPKTCOUNT5,
    _reserved193: [u8; 2usize],
    #[doc = "0x318 - USB Request Packet Count in Block Transfer Endpoint 6"]
    pub rqpktcount6: RQPKTCOUNT6,
    _reserved194: [u8; 2usize],
    #[doc = "0x31c - USB Request Packet Count in Block Transfer Endpoint 7"]
    pub rqpktcount7: RQPKTCOUNT7,
    _reserved195: [u8; 34usize],
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
    _reserved200: [u8; 22usize],
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
    _reserved205: [u8; 154usize],
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
    _reserved213: [u8; 16usize],
    #[doc = "0x430 - USB VBUS Droop Control"]
    pub vdc: VDC,
    #[doc = "0x434 - USB VBUS Droop Control Raw Interrupt Status"]
    pub vdcris: VDCRIS,
    #[doc = "0x438 - USB VBUS Droop Control Interrupt Mask"]
    pub vdcim: VDCIM,
    #[doc = "0x43c - USB VBUS Droop Control Interrupt Status and Clear"]
    pub vdcisc: VDCISC,
    _reserved217: [u8; 2944usize],
    #[doc = "0xfc0 - USB Peripheral Properties"]
    pub pp: PP,
    #[doc = "0xfc4 - USB Peripheral Configuration"]
    pub pc: PC,
    #[doc = "0xfc8 - USB Clock Configuration"]
    pub cc: CC,
}
#[doc = "USB Device Functional Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faddr](faddr) module"]
pub type FADDR = crate::Reg<u8, _FADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FADDR;
#[doc = "`read()` method returns [faddr::R](faddr::R) reader structure"]
impl crate::Readable for FADDR {}
#[doc = "`write(|w| ..)` method takes [faddr::W](faddr::W) writer structure"]
impl crate::Writable for FADDR {}
#[doc = "USB Device Functional Address"]
pub mod faddr;
#[doc = "USB Power\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
pub type POWER = crate::Reg<u8, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "USB Power"]
pub mod power;
#[doc = "USB Transmit Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txis](txis) module"]
pub type TXIS = crate::Reg<u16, _TXIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXIS;
#[doc = "`read()` method returns [txis::R](txis::R) reader structure"]
impl crate::Readable for TXIS {}
#[doc = "USB Transmit Interrupt Status"]
pub mod txis;
#[doc = "USB Receive Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxis](rxis) module"]
pub type RXIS = crate::Reg<u16, _RXIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIS;
#[doc = "`read()` method returns [rxis::R](rxis::R) reader structure"]
impl crate::Readable for RXIS {}
#[doc = "USB Receive Interrupt Status"]
pub mod rxis;
#[doc = "USB Transmit Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txie](txie) module"]
pub type TXIE = crate::Reg<u16, _TXIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXIE;
#[doc = "`read()` method returns [txie::R](txie::R) reader structure"]
impl crate::Readable for TXIE {}
#[doc = "`write(|w| ..)` method takes [txie::W](txie::W) writer structure"]
impl crate::Writable for TXIE {}
#[doc = "USB Transmit Interrupt Enable"]
pub mod txie;
#[doc = "USB Receive Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxie](rxie) module"]
pub type RXIE = crate::Reg<u16, _RXIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIE;
#[doc = "`read()` method returns [rxie::R](rxie::R) reader structure"]
impl crate::Readable for RXIE {}
#[doc = "`write(|w| ..)` method takes [rxie::W](rxie::W) writer structure"]
impl crate::Writable for RXIE {}
#[doc = "USB Receive Interrupt Enable"]
pub mod rxie;
#[doc = "USB General Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](is) module"]
pub type IS = crate::Reg<u8, _IS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IS;
#[doc = "`read()` method returns [is::R](is::R) reader structure"]
impl crate::Readable for IS {}
#[doc = "USB General Interrupt Status"]
pub mod is;
#[doc = "USB Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](ie) module"]
pub type IE = crate::Reg<u8, _IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IE;
#[doc = "`read()` method returns [ie::R](ie::R) reader structure"]
impl crate::Readable for IE {}
#[doc = "`write(|w| ..)` method takes [ie::W](ie::W) writer structure"]
impl crate::Writable for IE {}
#[doc = "USB Interrupt Enable"]
pub mod ie;
#[doc = "USB Frame Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame](frame) module"]
pub type FRAME = crate::Reg<u16, _FRAME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAME;
#[doc = "`read()` method returns [frame::R](frame::R) reader structure"]
impl crate::Readable for FRAME {}
#[doc = "USB Frame Value"]
pub mod frame;
#[doc = "USB Endpoint Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epidx](epidx) module"]
pub type EPIDX = crate::Reg<u8, _EPIDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPIDX;
#[doc = "`read()` method returns [epidx::R](epidx::R) reader structure"]
impl crate::Readable for EPIDX {}
#[doc = "`write(|w| ..)` method takes [epidx::W](epidx::W) writer structure"]
impl crate::Writable for EPIDX {}
#[doc = "USB Endpoint Index"]
pub mod epidx;
#[doc = "USB Test Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u8, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "USB Test Mode"]
pub mod test;
#[doc = "USB FIFO Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo0](fifo0) module"]
pub type FIFO0 = crate::Reg<u32, _FIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO0;
#[doc = "`read()` method returns [fifo0::R](fifo0::R) reader structure"]
impl crate::Readable for FIFO0 {}
#[doc = "`write(|w| ..)` method takes [fifo0::W](fifo0::W) writer structure"]
impl crate::Writable for FIFO0 {}
#[doc = "USB FIFO Endpoint 0"]
pub mod fifo0;
#[doc = "USB FIFO Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo1](fifo1) module"]
pub type FIFO1 = crate::Reg<u32, _FIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO1;
#[doc = "`read()` method returns [fifo1::R](fifo1::R) reader structure"]
impl crate::Readable for FIFO1 {}
#[doc = "`write(|w| ..)` method takes [fifo1::W](fifo1::W) writer structure"]
impl crate::Writable for FIFO1 {}
#[doc = "USB FIFO Endpoint 1"]
pub mod fifo1;
#[doc = "USB FIFO Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo2](fifo2) module"]
pub type FIFO2 = crate::Reg<u32, _FIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO2;
#[doc = "`read()` method returns [fifo2::R](fifo2::R) reader structure"]
impl crate::Readable for FIFO2 {}
#[doc = "`write(|w| ..)` method takes [fifo2::W](fifo2::W) writer structure"]
impl crate::Writable for FIFO2 {}
#[doc = "USB FIFO Endpoint 2"]
pub mod fifo2;
#[doc = "USB FIFO Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo3](fifo3) module"]
pub type FIFO3 = crate::Reg<u32, _FIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO3;
#[doc = "`read()` method returns [fifo3::R](fifo3::R) reader structure"]
impl crate::Readable for FIFO3 {}
#[doc = "`write(|w| ..)` method takes [fifo3::W](fifo3::W) writer structure"]
impl crate::Writable for FIFO3 {}
#[doc = "USB FIFO Endpoint 3"]
pub mod fifo3;
#[doc = "USB FIFO Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo4](fifo4) module"]
pub type FIFO4 = crate::Reg<u32, _FIFO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO4;
#[doc = "`read()` method returns [fifo4::R](fifo4::R) reader structure"]
impl crate::Readable for FIFO4 {}
#[doc = "`write(|w| ..)` method takes [fifo4::W](fifo4::W) writer structure"]
impl crate::Writable for FIFO4 {}
#[doc = "USB FIFO Endpoint 4"]
pub mod fifo4;
#[doc = "USB FIFO Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo5](fifo5) module"]
pub type FIFO5 = crate::Reg<u32, _FIFO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO5;
#[doc = "`read()` method returns [fifo5::R](fifo5::R) reader structure"]
impl crate::Readable for FIFO5 {}
#[doc = "`write(|w| ..)` method takes [fifo5::W](fifo5::W) writer structure"]
impl crate::Writable for FIFO5 {}
#[doc = "USB FIFO Endpoint 5"]
pub mod fifo5;
#[doc = "USB FIFO Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo6](fifo6) module"]
pub type FIFO6 = crate::Reg<u32, _FIFO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO6;
#[doc = "`read()` method returns [fifo6::R](fifo6::R) reader structure"]
impl crate::Readable for FIFO6 {}
#[doc = "`write(|w| ..)` method takes [fifo6::W](fifo6::W) writer structure"]
impl crate::Writable for FIFO6 {}
#[doc = "USB FIFO Endpoint 6"]
pub mod fifo6;
#[doc = "USB FIFO Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo7](fifo7) module"]
pub type FIFO7 = crate::Reg<u32, _FIFO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO7;
#[doc = "`read()` method returns [fifo7::R](fifo7::R) reader structure"]
impl crate::Readable for FIFO7 {}
#[doc = "`write(|w| ..)` method takes [fifo7::W](fifo7::W) writer structure"]
impl crate::Writable for FIFO7 {}
#[doc = "USB FIFO Endpoint 7"]
pub mod fifo7;
#[doc = "USB Device Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devctl](devctl) module"]
pub type DEVCTL = crate::Reg<u8, _DEVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVCTL;
#[doc = "`read()` method returns [devctl::R](devctl::R) reader structure"]
impl crate::Readable for DEVCTL {}
#[doc = "`write(|w| ..)` method takes [devctl::W](devctl::W) writer structure"]
impl crate::Writable for DEVCTL {}
#[doc = "USB Device Control"]
pub mod devctl;
#[doc = "USB Common Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cconf](cconf) module"]
pub type CCONF = crate::Reg<u8, _CCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCONF;
#[doc = "`read()` method returns [cconf::R](cconf::R) reader structure"]
impl crate::Readable for CCONF {}
#[doc = "`write(|w| ..)` method takes [cconf::W](cconf::W) writer structure"]
impl crate::Writable for CCONF {}
#[doc = "USB Common Configuration"]
pub mod cconf;
#[doc = "USB Transmit Dynamic FIFO Sizing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifosz](txfifosz) module"]
pub type TXFIFOSZ = crate::Reg<u8, _TXFIFOSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIFOSZ;
#[doc = "`read()` method returns [txfifosz::R](txfifosz::R) reader structure"]
impl crate::Readable for TXFIFOSZ {}
#[doc = "`write(|w| ..)` method takes [txfifosz::W](txfifosz::W) writer structure"]
impl crate::Writable for TXFIFOSZ {}
#[doc = "USB Transmit Dynamic FIFO Sizing"]
pub mod txfifosz;
#[doc = "USB Receive Dynamic FIFO Sizing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifosz](rxfifosz) module"]
pub type RXFIFOSZ = crate::Reg<u8, _RXFIFOSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFOSZ;
#[doc = "`read()` method returns [rxfifosz::R](rxfifosz::R) reader structure"]
impl crate::Readable for RXFIFOSZ {}
#[doc = "`write(|w| ..)` method takes [rxfifosz::W](rxfifosz::W) writer structure"]
impl crate::Writable for RXFIFOSZ {}
#[doc = "USB Receive Dynamic FIFO Sizing"]
pub mod rxfifosz;
#[doc = "USB Transmit FIFO Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfifoadd](txfifoadd) module"]
pub type TXFIFOADD = crate::Reg<u16, _TXFIFOADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFIFOADD;
#[doc = "`read()` method returns [txfifoadd::R](txfifoadd::R) reader structure"]
impl crate::Readable for TXFIFOADD {}
#[doc = "`write(|w| ..)` method takes [txfifoadd::W](txfifoadd::W) writer structure"]
impl crate::Writable for TXFIFOADD {}
#[doc = "USB Transmit FIFO Start Address"]
pub mod txfifoadd;
#[doc = "USB Receive FIFO Start Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifoadd](rxfifoadd) module"]
pub type RXFIFOADD = crate::Reg<u16, _RXFIFOADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFIFOADD;
#[doc = "`read()` method returns [rxfifoadd::R](rxfifoadd::R) reader structure"]
impl crate::Readable for RXFIFOADD {}
#[doc = "`write(|w| ..)` method takes [rxfifoadd::W](rxfifoadd::W) writer structure"]
impl crate::Writable for RXFIFOADD {}
#[doc = "USB Receive FIFO Start Address"]
pub mod rxfifoadd;
#[doc = "USB ULPI VBUS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpivbusctl](ulpivbusctl) module"]
pub type ULPIVBUSCTL = crate::Reg<u8, _ULPIVBUSCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULPIVBUSCTL;
#[doc = "`read()` method returns [ulpivbusctl::R](ulpivbusctl::R) reader structure"]
impl crate::Readable for ULPIVBUSCTL {}
#[doc = "`write(|w| ..)` method takes [ulpivbusctl::W](ulpivbusctl::W) writer structure"]
impl crate::Writable for ULPIVBUSCTL {}
#[doc = "USB ULPI VBUS Control"]
pub mod ulpivbusctl;
#[doc = "USB ULPI Register Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregdata](ulpiregdata) module"]
pub type ULPIREGDATA = crate::Reg<u8, _ULPIREGDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULPIREGDATA;
#[doc = "`read()` method returns [ulpiregdata::R](ulpiregdata::R) reader structure"]
impl crate::Readable for ULPIREGDATA {}
#[doc = "`write(|w| ..)` method takes [ulpiregdata::W](ulpiregdata::W) writer structure"]
impl crate::Writable for ULPIREGDATA {}
#[doc = "USB ULPI Register Data"]
pub mod ulpiregdata;
#[doc = "USB ULPI Register Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregaddr](ulpiregaddr) module"]
pub type ULPIREGADDR = crate::Reg<u8, _ULPIREGADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULPIREGADDR;
#[doc = "`read()` method returns [ulpiregaddr::R](ulpiregaddr::R) reader structure"]
impl crate::Readable for ULPIREGADDR {}
#[doc = "`write(|w| ..)` method takes [ulpiregaddr::W](ulpiregaddr::W) writer structure"]
impl crate::Writable for ULPIREGADDR {}
#[doc = "USB ULPI Register Address"]
pub mod ulpiregaddr;
#[doc = "USB ULPI Register Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ulpiregctl](ulpiregctl) module"]
pub type ULPIREGCTL = crate::Reg<u8, _ULPIREGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ULPIREGCTL;
#[doc = "`read()` method returns [ulpiregctl::R](ulpiregctl::R) reader structure"]
impl crate::Readable for ULPIREGCTL {}
#[doc = "`write(|w| ..)` method takes [ulpiregctl::W](ulpiregctl::W) writer structure"]
impl crate::Writable for ULPIREGCTL {}
#[doc = "USB ULPI Register Control"]
pub mod ulpiregctl;
#[doc = "USB Endpoint Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinfo](epinfo) module"]
pub type EPINFO = crate::Reg<u8, _EPINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINFO;
#[doc = "`read()` method returns [epinfo::R](epinfo::R) reader structure"]
impl crate::Readable for EPINFO {}
#[doc = "USB Endpoint Information"]
pub mod epinfo;
#[doc = "USB RAM Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raminfo](raminfo) module"]
pub type RAMINFO = crate::Reg<u8, _RAMINFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMINFO;
#[doc = "`read()` method returns [raminfo::R](raminfo::R) reader structure"]
impl crate::Readable for RAMINFO {}
#[doc = "USB RAM Information"]
pub mod raminfo;
#[doc = "USB Connect Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [contim](contim) module"]
pub type CONTIM = crate::Reg<u8, _CONTIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTIM;
#[doc = "`read()` method returns [contim::R](contim::R) reader structure"]
impl crate::Readable for CONTIM {}
#[doc = "`write(|w| ..)` method takes [contim::W](contim::W) writer structure"]
impl crate::Writable for CONTIM {}
#[doc = "USB Connect Timing"]
pub mod contim;
#[doc = "USB OTG VBUS Pulse Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vplen](vplen) module"]
pub type VPLEN = crate::Reg<u8, _VPLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VPLEN;
#[doc = "`read()` method returns [vplen::R](vplen::R) reader structure"]
impl crate::Readable for VPLEN {}
#[doc = "`write(|w| ..)` method takes [vplen::W](vplen::W) writer structure"]
impl crate::Writable for VPLEN {}
#[doc = "USB OTG VBUS Pulse Timing"]
pub mod vplen;
#[doc = "USB High-Speed Last Transaction to End of Frame Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hseof](hseof) module"]
pub type HSEOF = crate::Reg<u8, _HSEOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSEOF;
#[doc = "`read()` method returns [hseof::R](hseof::R) reader structure"]
impl crate::Readable for HSEOF {}
#[doc = "`write(|w| ..)` method takes [hseof::W](hseof::W) writer structure"]
impl crate::Writable for HSEOF {}
#[doc = "USB High-Speed Last Transaction to End of Frame Timing"]
pub mod hseof;
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fseof](fseof) module"]
pub type FSEOF = crate::Reg<u8, _FSEOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSEOF;
#[doc = "`read()` method returns [fseof::R](fseof::R) reader structure"]
impl crate::Readable for FSEOF {}
#[doc = "`write(|w| ..)` method takes [fseof::W](fseof::W) writer structure"]
impl crate::Writable for FSEOF {}
#[doc = "USB Full-Speed Last Transaction to End of Frame Timing"]
pub mod fseof;
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lseof](lseof) module"]
pub type LSEOF = crate::Reg<u8, _LSEOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSEOF;
#[doc = "`read()` method returns [lseof::R](lseof::R) reader structure"]
impl crate::Readable for LSEOF {}
#[doc = "`write(|w| ..)` method takes [lseof::W](lseof::W) writer structure"]
impl crate::Writable for LSEOF {}
#[doc = "USB Low-Speed Last Transaction to End of Frame Timing"]
pub mod lseof;
#[doc = "USB Transmit Functional Address Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr0](txfuncaddr0) module"]
pub type TXFUNCADDR0 = crate::Reg<u8, _TXFUNCADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR0;
#[doc = "`read()` method returns [txfuncaddr0::R](txfuncaddr0::R) reader structure"]
impl crate::Readable for TXFUNCADDR0 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr0::W](txfuncaddr0::W) writer structure"]
impl crate::Writable for TXFUNCADDR0 {}
#[doc = "USB Transmit Functional Address Endpoint 0"]
pub mod txfuncaddr0;
#[doc = "USB Transmit Hub Address Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr0](txhubaddr0) module"]
pub type TXHUBADDR0 = crate::Reg<u8, _TXHUBADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR0;
#[doc = "`read()` method returns [txhubaddr0::R](txhubaddr0::R) reader structure"]
impl crate::Readable for TXHUBADDR0 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr0::W](txhubaddr0::W) writer structure"]
impl crate::Writable for TXHUBADDR0 {}
#[doc = "USB Transmit Hub Address Endpoint 0"]
pub mod txhubaddr0;
#[doc = "USB Transmit Hub Port Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport0](txhubport0) module"]
pub type TXHUBPORT0 = crate::Reg<u8, _TXHUBPORT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT0;
#[doc = "`read()` method returns [txhubport0::R](txhubport0::R) reader structure"]
impl crate::Readable for TXHUBPORT0 {}
#[doc = "`write(|w| ..)` method takes [txhubport0::W](txhubport0::W) writer structure"]
impl crate::Writable for TXHUBPORT0 {}
#[doc = "USB Transmit Hub Port Endpoint 0"]
pub mod txhubport0;
#[doc = "USB Transmit Functional Address Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr1](txfuncaddr1) module"]
pub type TXFUNCADDR1 = crate::Reg<u8, _TXFUNCADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR1;
#[doc = "`read()` method returns [txfuncaddr1::R](txfuncaddr1::R) reader structure"]
impl crate::Readable for TXFUNCADDR1 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr1::W](txfuncaddr1::W) writer structure"]
impl crate::Writable for TXFUNCADDR1 {}
#[doc = "USB Transmit Functional Address Endpoint 1"]
pub mod txfuncaddr1;
#[doc = "USB Transmit Hub Address Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr1](txhubaddr1) module"]
pub type TXHUBADDR1 = crate::Reg<u8, _TXHUBADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR1;
#[doc = "`read()` method returns [txhubaddr1::R](txhubaddr1::R) reader structure"]
impl crate::Readable for TXHUBADDR1 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr1::W](txhubaddr1::W) writer structure"]
impl crate::Writable for TXHUBADDR1 {}
#[doc = "USB Transmit Hub Address Endpoint 1"]
pub mod txhubaddr1;
#[doc = "USB Transmit Hub Port Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport1](txhubport1) module"]
pub type TXHUBPORT1 = crate::Reg<u8, _TXHUBPORT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT1;
#[doc = "`read()` method returns [txhubport1::R](txhubport1::R) reader structure"]
impl crate::Readable for TXHUBPORT1 {}
#[doc = "`write(|w| ..)` method takes [txhubport1::W](txhubport1::W) writer structure"]
impl crate::Writable for TXHUBPORT1 {}
#[doc = "USB Transmit Hub Port Endpoint 1"]
pub mod txhubport1;
#[doc = "USB Receive Functional Address Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr1](rxfuncaddr1) module"]
pub type RXFUNCADDR1 = crate::Reg<u8, _RXFUNCADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR1;
#[doc = "`read()` method returns [rxfuncaddr1::R](rxfuncaddr1::R) reader structure"]
impl crate::Readable for RXFUNCADDR1 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr1::W](rxfuncaddr1::W) writer structure"]
impl crate::Writable for RXFUNCADDR1 {}
#[doc = "USB Receive Functional Address Endpoint 1"]
pub mod rxfuncaddr1;
#[doc = "USB Receive Hub Address Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr1](rxhubaddr1) module"]
pub type RXHUBADDR1 = crate::Reg<u8, _RXHUBADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR1;
#[doc = "`read()` method returns [rxhubaddr1::R](rxhubaddr1::R) reader structure"]
impl crate::Readable for RXHUBADDR1 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr1::W](rxhubaddr1::W) writer structure"]
impl crate::Writable for RXHUBADDR1 {}
#[doc = "USB Receive Hub Address Endpoint 1"]
pub mod rxhubaddr1;
#[doc = "USB Receive Hub Port Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport1](rxhubport1) module"]
pub type RXHUBPORT1 = crate::Reg<u8, _RXHUBPORT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT1;
#[doc = "`read()` method returns [rxhubport1::R](rxhubport1::R) reader structure"]
impl crate::Readable for RXHUBPORT1 {}
#[doc = "`write(|w| ..)` method takes [rxhubport1::W](rxhubport1::W) writer structure"]
impl crate::Writable for RXHUBPORT1 {}
#[doc = "USB Receive Hub Port Endpoint 1"]
pub mod rxhubport1;
#[doc = "USB Transmit Functional Address Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr2](txfuncaddr2) module"]
pub type TXFUNCADDR2 = crate::Reg<u8, _TXFUNCADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR2;
#[doc = "`read()` method returns [txfuncaddr2::R](txfuncaddr2::R) reader structure"]
impl crate::Readable for TXFUNCADDR2 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr2::W](txfuncaddr2::W) writer structure"]
impl crate::Writable for TXFUNCADDR2 {}
#[doc = "USB Transmit Functional Address Endpoint 2"]
pub mod txfuncaddr2;
#[doc = "USB Transmit Hub Address Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr2](txhubaddr2) module"]
pub type TXHUBADDR2 = crate::Reg<u8, _TXHUBADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR2;
#[doc = "`read()` method returns [txhubaddr2::R](txhubaddr2::R) reader structure"]
impl crate::Readable for TXHUBADDR2 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr2::W](txhubaddr2::W) writer structure"]
impl crate::Writable for TXHUBADDR2 {}
#[doc = "USB Transmit Hub Address Endpoint 2"]
pub mod txhubaddr2;
#[doc = "USB Transmit Hub Port Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport2](txhubport2) module"]
pub type TXHUBPORT2 = crate::Reg<u8, _TXHUBPORT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT2;
#[doc = "`read()` method returns [txhubport2::R](txhubport2::R) reader structure"]
impl crate::Readable for TXHUBPORT2 {}
#[doc = "`write(|w| ..)` method takes [txhubport2::W](txhubport2::W) writer structure"]
impl crate::Writable for TXHUBPORT2 {}
#[doc = "USB Transmit Hub Port Endpoint 2"]
pub mod txhubport2;
#[doc = "USB Receive Functional Address Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr2](rxfuncaddr2) module"]
pub type RXFUNCADDR2 = crate::Reg<u8, _RXFUNCADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR2;
#[doc = "`read()` method returns [rxfuncaddr2::R](rxfuncaddr2::R) reader structure"]
impl crate::Readable for RXFUNCADDR2 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr2::W](rxfuncaddr2::W) writer structure"]
impl crate::Writable for RXFUNCADDR2 {}
#[doc = "USB Receive Functional Address Endpoint 2"]
pub mod rxfuncaddr2;
#[doc = "USB Receive Hub Address Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr2](rxhubaddr2) module"]
pub type RXHUBADDR2 = crate::Reg<u8, _RXHUBADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR2;
#[doc = "`read()` method returns [rxhubaddr2::R](rxhubaddr2::R) reader structure"]
impl crate::Readable for RXHUBADDR2 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr2::W](rxhubaddr2::W) writer structure"]
impl crate::Writable for RXHUBADDR2 {}
#[doc = "USB Receive Hub Address Endpoint 2"]
pub mod rxhubaddr2;
#[doc = "USB Receive Hub Port Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport2](rxhubport2) module"]
pub type RXHUBPORT2 = crate::Reg<u8, _RXHUBPORT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT2;
#[doc = "`read()` method returns [rxhubport2::R](rxhubport2::R) reader structure"]
impl crate::Readable for RXHUBPORT2 {}
#[doc = "`write(|w| ..)` method takes [rxhubport2::W](rxhubport2::W) writer structure"]
impl crate::Writable for RXHUBPORT2 {}
#[doc = "USB Receive Hub Port Endpoint 2"]
pub mod rxhubport2;
#[doc = "USB Transmit Functional Address Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr3](txfuncaddr3) module"]
pub type TXFUNCADDR3 = crate::Reg<u8, _TXFUNCADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR3;
#[doc = "`read()` method returns [txfuncaddr3::R](txfuncaddr3::R) reader structure"]
impl crate::Readable for TXFUNCADDR3 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr3::W](txfuncaddr3::W) writer structure"]
impl crate::Writable for TXFUNCADDR3 {}
#[doc = "USB Transmit Functional Address Endpoint 3"]
pub mod txfuncaddr3;
#[doc = "USB Transmit Hub Address Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr3](txhubaddr3) module"]
pub type TXHUBADDR3 = crate::Reg<u8, _TXHUBADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR3;
#[doc = "`read()` method returns [txhubaddr3::R](txhubaddr3::R) reader structure"]
impl crate::Readable for TXHUBADDR3 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr3::W](txhubaddr3::W) writer structure"]
impl crate::Writable for TXHUBADDR3 {}
#[doc = "USB Transmit Hub Address Endpoint 3"]
pub mod txhubaddr3;
#[doc = "USB Transmit Hub Port Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport3](txhubport3) module"]
pub type TXHUBPORT3 = crate::Reg<u8, _TXHUBPORT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT3;
#[doc = "`read()` method returns [txhubport3::R](txhubport3::R) reader structure"]
impl crate::Readable for TXHUBPORT3 {}
#[doc = "`write(|w| ..)` method takes [txhubport3::W](txhubport3::W) writer structure"]
impl crate::Writable for TXHUBPORT3 {}
#[doc = "USB Transmit Hub Port Endpoint 3"]
pub mod txhubport3;
#[doc = "USB Receive Functional Address Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr3](rxfuncaddr3) module"]
pub type RXFUNCADDR3 = crate::Reg<u8, _RXFUNCADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR3;
#[doc = "`read()` method returns [rxfuncaddr3::R](rxfuncaddr3::R) reader structure"]
impl crate::Readable for RXFUNCADDR3 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr3::W](rxfuncaddr3::W) writer structure"]
impl crate::Writable for RXFUNCADDR3 {}
#[doc = "USB Receive Functional Address Endpoint 3"]
pub mod rxfuncaddr3;
#[doc = "USB Receive Hub Address Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr3](rxhubaddr3) module"]
pub type RXHUBADDR3 = crate::Reg<u8, _RXHUBADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR3;
#[doc = "`read()` method returns [rxhubaddr3::R](rxhubaddr3::R) reader structure"]
impl crate::Readable for RXHUBADDR3 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr3::W](rxhubaddr3::W) writer structure"]
impl crate::Writable for RXHUBADDR3 {}
#[doc = "USB Receive Hub Address Endpoint 3"]
pub mod rxhubaddr3;
#[doc = "USB Receive Hub Port Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport3](rxhubport3) module"]
pub type RXHUBPORT3 = crate::Reg<u8, _RXHUBPORT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT3;
#[doc = "`read()` method returns [rxhubport3::R](rxhubport3::R) reader structure"]
impl crate::Readable for RXHUBPORT3 {}
#[doc = "`write(|w| ..)` method takes [rxhubport3::W](rxhubport3::W) writer structure"]
impl crate::Writable for RXHUBPORT3 {}
#[doc = "USB Receive Hub Port Endpoint 3"]
pub mod rxhubport3;
#[doc = "USB Transmit Functional Address Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr4](txfuncaddr4) module"]
pub type TXFUNCADDR4 = crate::Reg<u8, _TXFUNCADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR4;
#[doc = "`read()` method returns [txfuncaddr4::R](txfuncaddr4::R) reader structure"]
impl crate::Readable for TXFUNCADDR4 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr4::W](txfuncaddr4::W) writer structure"]
impl crate::Writable for TXFUNCADDR4 {}
#[doc = "USB Transmit Functional Address Endpoint 4"]
pub mod txfuncaddr4;
#[doc = "USB Transmit Hub Address Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr4](txhubaddr4) module"]
pub type TXHUBADDR4 = crate::Reg<u8, _TXHUBADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR4;
#[doc = "`read()` method returns [txhubaddr4::R](txhubaddr4::R) reader structure"]
impl crate::Readable for TXHUBADDR4 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr4::W](txhubaddr4::W) writer structure"]
impl crate::Writable for TXHUBADDR4 {}
#[doc = "USB Transmit Hub Address Endpoint 4"]
pub mod txhubaddr4;
#[doc = "USB Transmit Hub Port Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport4](txhubport4) module"]
pub type TXHUBPORT4 = crate::Reg<u8, _TXHUBPORT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT4;
#[doc = "`read()` method returns [txhubport4::R](txhubport4::R) reader structure"]
impl crate::Readable for TXHUBPORT4 {}
#[doc = "`write(|w| ..)` method takes [txhubport4::W](txhubport4::W) writer structure"]
impl crate::Writable for TXHUBPORT4 {}
#[doc = "USB Transmit Hub Port Endpoint 4"]
pub mod txhubport4;
#[doc = "USB Receive Functional Address Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr4](rxfuncaddr4) module"]
pub type RXFUNCADDR4 = crate::Reg<u8, _RXFUNCADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR4;
#[doc = "`read()` method returns [rxfuncaddr4::R](rxfuncaddr4::R) reader structure"]
impl crate::Readable for RXFUNCADDR4 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr4::W](rxfuncaddr4::W) writer structure"]
impl crate::Writable for RXFUNCADDR4 {}
#[doc = "USB Receive Functional Address Endpoint 4"]
pub mod rxfuncaddr4;
#[doc = "USB Receive Hub Address Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr4](rxhubaddr4) module"]
pub type RXHUBADDR4 = crate::Reg<u8, _RXHUBADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR4;
#[doc = "`read()` method returns [rxhubaddr4::R](rxhubaddr4::R) reader structure"]
impl crate::Readable for RXHUBADDR4 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr4::W](rxhubaddr4::W) writer structure"]
impl crate::Writable for RXHUBADDR4 {}
#[doc = "USB Receive Hub Address Endpoint 4"]
pub mod rxhubaddr4;
#[doc = "USB Receive Hub Port Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport4](rxhubport4) module"]
pub type RXHUBPORT4 = crate::Reg<u8, _RXHUBPORT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT4;
#[doc = "`read()` method returns [rxhubport4::R](rxhubport4::R) reader structure"]
impl crate::Readable for RXHUBPORT4 {}
#[doc = "`write(|w| ..)` method takes [rxhubport4::W](rxhubport4::W) writer structure"]
impl crate::Writable for RXHUBPORT4 {}
#[doc = "USB Receive Hub Port Endpoint 4"]
pub mod rxhubport4;
#[doc = "USB Transmit Functional Address Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr5](txfuncaddr5) module"]
pub type TXFUNCADDR5 = crate::Reg<u8, _TXFUNCADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR5;
#[doc = "`read()` method returns [txfuncaddr5::R](txfuncaddr5::R) reader structure"]
impl crate::Readable for TXFUNCADDR5 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr5::W](txfuncaddr5::W) writer structure"]
impl crate::Writable for TXFUNCADDR5 {}
#[doc = "USB Transmit Functional Address Endpoint 5"]
pub mod txfuncaddr5;
#[doc = "USB Transmit Hub Address Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr5](txhubaddr5) module"]
pub type TXHUBADDR5 = crate::Reg<u8, _TXHUBADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR5;
#[doc = "`read()` method returns [txhubaddr5::R](txhubaddr5::R) reader structure"]
impl crate::Readable for TXHUBADDR5 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr5::W](txhubaddr5::W) writer structure"]
impl crate::Writable for TXHUBADDR5 {}
#[doc = "USB Transmit Hub Address Endpoint 5"]
pub mod txhubaddr5;
#[doc = "USB Transmit Hub Port Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport5](txhubport5) module"]
pub type TXHUBPORT5 = crate::Reg<u8, _TXHUBPORT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT5;
#[doc = "`read()` method returns [txhubport5::R](txhubport5::R) reader structure"]
impl crate::Readable for TXHUBPORT5 {}
#[doc = "`write(|w| ..)` method takes [txhubport5::W](txhubport5::W) writer structure"]
impl crate::Writable for TXHUBPORT5 {}
#[doc = "USB Transmit Hub Port Endpoint 5"]
pub mod txhubport5;
#[doc = "USB Receive Functional Address Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr5](rxfuncaddr5) module"]
pub type RXFUNCADDR5 = crate::Reg<u8, _RXFUNCADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR5;
#[doc = "`read()` method returns [rxfuncaddr5::R](rxfuncaddr5::R) reader structure"]
impl crate::Readable for RXFUNCADDR5 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr5::W](rxfuncaddr5::W) writer structure"]
impl crate::Writable for RXFUNCADDR5 {}
#[doc = "USB Receive Functional Address Endpoint 5"]
pub mod rxfuncaddr5;
#[doc = "USB Receive Hub Address Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr5](rxhubaddr5) module"]
pub type RXHUBADDR5 = crate::Reg<u8, _RXHUBADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR5;
#[doc = "`read()` method returns [rxhubaddr5::R](rxhubaddr5::R) reader structure"]
impl crate::Readable for RXHUBADDR5 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr5::W](rxhubaddr5::W) writer structure"]
impl crate::Writable for RXHUBADDR5 {}
#[doc = "USB Receive Hub Address Endpoint 5"]
pub mod rxhubaddr5;
#[doc = "USB Receive Hub Port Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport5](rxhubport5) module"]
pub type RXHUBPORT5 = crate::Reg<u8, _RXHUBPORT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT5;
#[doc = "`read()` method returns [rxhubport5::R](rxhubport5::R) reader structure"]
impl crate::Readable for RXHUBPORT5 {}
#[doc = "`write(|w| ..)` method takes [rxhubport5::W](rxhubport5::W) writer structure"]
impl crate::Writable for RXHUBPORT5 {}
#[doc = "USB Receive Hub Port Endpoint 5"]
pub mod rxhubport5;
#[doc = "USB Transmit Functional Address Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr6](txfuncaddr6) module"]
pub type TXFUNCADDR6 = crate::Reg<u8, _TXFUNCADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR6;
#[doc = "`read()` method returns [txfuncaddr6::R](txfuncaddr6::R) reader structure"]
impl crate::Readable for TXFUNCADDR6 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr6::W](txfuncaddr6::W) writer structure"]
impl crate::Writable for TXFUNCADDR6 {}
#[doc = "USB Transmit Functional Address Endpoint 6"]
pub mod txfuncaddr6;
#[doc = "USB Transmit Hub Address Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr6](txhubaddr6) module"]
pub type TXHUBADDR6 = crate::Reg<u8, _TXHUBADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR6;
#[doc = "`read()` method returns [txhubaddr6::R](txhubaddr6::R) reader structure"]
impl crate::Readable for TXHUBADDR6 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr6::W](txhubaddr6::W) writer structure"]
impl crate::Writable for TXHUBADDR6 {}
#[doc = "USB Transmit Hub Address Endpoint 6"]
pub mod txhubaddr6;
#[doc = "USB Transmit Hub Port Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport6](txhubport6) module"]
pub type TXHUBPORT6 = crate::Reg<u8, _TXHUBPORT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT6;
#[doc = "`read()` method returns [txhubport6::R](txhubport6::R) reader structure"]
impl crate::Readable for TXHUBPORT6 {}
#[doc = "`write(|w| ..)` method takes [txhubport6::W](txhubport6::W) writer structure"]
impl crate::Writable for TXHUBPORT6 {}
#[doc = "USB Transmit Hub Port Endpoint 6"]
pub mod txhubport6;
#[doc = "USB Receive Functional Address Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr6](rxfuncaddr6) module"]
pub type RXFUNCADDR6 = crate::Reg<u8, _RXFUNCADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR6;
#[doc = "`read()` method returns [rxfuncaddr6::R](rxfuncaddr6::R) reader structure"]
impl crate::Readable for RXFUNCADDR6 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr6::W](rxfuncaddr6::W) writer structure"]
impl crate::Writable for RXFUNCADDR6 {}
#[doc = "USB Receive Functional Address Endpoint 6"]
pub mod rxfuncaddr6;
#[doc = "USB Receive Hub Address Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr6](rxhubaddr6) module"]
pub type RXHUBADDR6 = crate::Reg<u8, _RXHUBADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR6;
#[doc = "`read()` method returns [rxhubaddr6::R](rxhubaddr6::R) reader structure"]
impl crate::Readable for RXHUBADDR6 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr6::W](rxhubaddr6::W) writer structure"]
impl crate::Writable for RXHUBADDR6 {}
#[doc = "USB Receive Hub Address Endpoint 6"]
pub mod rxhubaddr6;
#[doc = "USB Receive Hub Port Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport6](rxhubport6) module"]
pub type RXHUBPORT6 = crate::Reg<u8, _RXHUBPORT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT6;
#[doc = "`read()` method returns [rxhubport6::R](rxhubport6::R) reader structure"]
impl crate::Readable for RXHUBPORT6 {}
#[doc = "`write(|w| ..)` method takes [rxhubport6::W](rxhubport6::W) writer structure"]
impl crate::Writable for RXHUBPORT6 {}
#[doc = "USB Receive Hub Port Endpoint 6"]
pub mod rxhubport6;
#[doc = "USB Transmit Functional Address Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfuncaddr7](txfuncaddr7) module"]
pub type TXFUNCADDR7 = crate::Reg<u8, _TXFUNCADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXFUNCADDR7;
#[doc = "`read()` method returns [txfuncaddr7::R](txfuncaddr7::R) reader structure"]
impl crate::Readable for TXFUNCADDR7 {}
#[doc = "`write(|w| ..)` method takes [txfuncaddr7::W](txfuncaddr7::W) writer structure"]
impl crate::Writable for TXFUNCADDR7 {}
#[doc = "USB Transmit Functional Address Endpoint 7"]
pub mod txfuncaddr7;
#[doc = "USB Transmit Hub Address Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubaddr7](txhubaddr7) module"]
pub type TXHUBADDR7 = crate::Reg<u8, _TXHUBADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBADDR7;
#[doc = "`read()` method returns [txhubaddr7::R](txhubaddr7::R) reader structure"]
impl crate::Readable for TXHUBADDR7 {}
#[doc = "`write(|w| ..)` method takes [txhubaddr7::W](txhubaddr7::W) writer structure"]
impl crate::Writable for TXHUBADDR7 {}
#[doc = "USB Transmit Hub Address Endpoint 7"]
pub mod txhubaddr7;
#[doc = "USB Transmit Hub Port Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txhubport7](txhubport7) module"]
pub type TXHUBPORT7 = crate::Reg<u8, _TXHUBPORT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXHUBPORT7;
#[doc = "`read()` method returns [txhubport7::R](txhubport7::R) reader structure"]
impl crate::Readable for TXHUBPORT7 {}
#[doc = "`write(|w| ..)` method takes [txhubport7::W](txhubport7::W) writer structure"]
impl crate::Writable for TXHUBPORT7 {}
#[doc = "USB Transmit Hub Port Endpoint 7"]
pub mod txhubport7;
#[doc = "USB Receive Functional Address Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfuncaddr7](rxfuncaddr7) module"]
pub type RXFUNCADDR7 = crate::Reg<u8, _RXFUNCADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFUNCADDR7;
#[doc = "`read()` method returns [rxfuncaddr7::R](rxfuncaddr7::R) reader structure"]
impl crate::Readable for RXFUNCADDR7 {}
#[doc = "`write(|w| ..)` method takes [rxfuncaddr7::W](rxfuncaddr7::W) writer structure"]
impl crate::Writable for RXFUNCADDR7 {}
#[doc = "USB Receive Functional Address Endpoint 7"]
pub mod rxfuncaddr7;
#[doc = "USB Receive Hub Address Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubaddr7](rxhubaddr7) module"]
pub type RXHUBADDR7 = crate::Reg<u8, _RXHUBADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBADDR7;
#[doc = "`read()` method returns [rxhubaddr7::R](rxhubaddr7::R) reader structure"]
impl crate::Readable for RXHUBADDR7 {}
#[doc = "`write(|w| ..)` method takes [rxhubaddr7::W](rxhubaddr7::W) writer structure"]
impl crate::Writable for RXHUBADDR7 {}
#[doc = "USB Receive Hub Address Endpoint 7"]
pub mod rxhubaddr7;
#[doc = "USB Receive Hub Port Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxhubport7](rxhubport7) module"]
pub type RXHUBPORT7 = crate::Reg<u8, _RXHUBPORT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXHUBPORT7;
#[doc = "`read()` method returns [rxhubport7::R](rxhubport7::R) reader structure"]
impl crate::Readable for RXHUBPORT7 {}
#[doc = "`write(|w| ..)` method takes [rxhubport7::W](rxhubport7::W) writer structure"]
impl crate::Writable for RXHUBPORT7 {}
#[doc = "USB Receive Hub Port Endpoint 7"]
pub mod rxhubport7;
#[doc = "USB Control and Status Endpoint 0 Low\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrl0](csrl0) module"]
pub type CSRL0 = crate::Reg<u8, _CSRL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSRL0;
#[doc = "`write(|w| ..)` method takes [csrl0::W](csrl0::W) writer structure"]
impl crate::Writable for CSRL0 {}
#[doc = "USB Control and Status Endpoint 0 Low"]
pub mod csrl0;
#[doc = "USB Control and Status Endpoint 0 High\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrh0](csrh0) module"]
pub type CSRH0 = crate::Reg<u8, _CSRH0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSRH0;
#[doc = "`write(|w| ..)` method takes [csrh0::W](csrh0::W) writer structure"]
impl crate::Writable for CSRH0 {}
#[doc = "USB Control and Status Endpoint 0 High"]
pub mod csrh0;
#[doc = "USB Receive Byte Count Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [count0](count0) module"]
pub type COUNT0 = crate::Reg<u8, _COUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNT0;
#[doc = "`read()` method returns [count0::R](count0::R) reader structure"]
impl crate::Readable for COUNT0 {}
#[doc = "USB Receive Byte Count Endpoint 0"]
pub mod count0;
#[doc = "USB Type Endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type0](type0) module"]
pub type TYPE0 = crate::Reg<u8, _TYPE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TYPE0;
#[doc = "`read()` method returns [type0::R](type0::R) reader structure"]
impl crate::Readable for TYPE0 {}
#[doc = "`write(|w| ..)` method takes [type0::W](type0::W) writer structure"]
impl crate::Writable for TYPE0 {}
#[doc = "USB Type Endpoint 0"]
pub mod type0;
#[doc = "USB NAK Limit\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [naklmt](naklmt) module"]
pub type NAKLMT = crate::Reg<u8, _NAKLMT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NAKLMT;
#[doc = "`read()` method returns [naklmt::R](naklmt::R) reader structure"]
impl crate::Readable for NAKLMT {}
#[doc = "`write(|w| ..)` method takes [naklmt::W](naklmt::W) writer structure"]
impl crate::Writable for NAKLMT {}
#[doc = "USB NAK Limit"]
pub mod naklmt;
#[doc = "USB Maximum Transmit Data Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp1](txmaxp1) module"]
pub type TXMAXP1 = crate::Reg<u16, _TXMAXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP1;
#[doc = "`read()` method returns [txmaxp1::R](txmaxp1::R) reader structure"]
impl crate::Readable for TXMAXP1 {}
#[doc = "`write(|w| ..)` method takes [txmaxp1::W](txmaxp1::W) writer structure"]
impl crate::Writable for TXMAXP1 {}
#[doc = "USB Maximum Transmit Data Endpoint 1"]
pub mod txmaxp1;
#[doc = "USB Transmit Control and Status Endpoint 1 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl1](txcsrl1) module"]
pub type TXCSRL1 = crate::Reg<u8, _TXCSRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL1;
#[doc = "`read()` method returns [txcsrl1::R](txcsrl1::R) reader structure"]
impl crate::Readable for TXCSRL1 {}
#[doc = "`write(|w| ..)` method takes [txcsrl1::W](txcsrl1::W) writer structure"]
impl crate::Writable for TXCSRL1 {}
#[doc = "USB Transmit Control and Status Endpoint 1 Low"]
pub mod txcsrl1;
#[doc = "USB Transmit Control and Status Endpoint 1 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh1](txcsrh1) module"]
pub type TXCSRH1 = crate::Reg<u8, _TXCSRH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH1;
#[doc = "`read()` method returns [txcsrh1::R](txcsrh1::R) reader structure"]
impl crate::Readable for TXCSRH1 {}
#[doc = "`write(|w| ..)` method takes [txcsrh1::W](txcsrh1::W) writer structure"]
impl crate::Writable for TXCSRH1 {}
#[doc = "USB Transmit Control and Status Endpoint 1 High"]
pub mod txcsrh1;
#[doc = "USB Maximum Receive Data Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp1](rxmaxp1) module"]
pub type RXMAXP1 = crate::Reg<u16, _RXMAXP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP1;
#[doc = "`read()` method returns [rxmaxp1::R](rxmaxp1::R) reader structure"]
impl crate::Readable for RXMAXP1 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp1::W](rxmaxp1::W) writer structure"]
impl crate::Writable for RXMAXP1 {}
#[doc = "USB Maximum Receive Data Endpoint 1"]
pub mod rxmaxp1;
#[doc = "USB Receive Control and Status Endpoint 1 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl1](rxcsrl1) module"]
pub type RXCSRL1 = crate::Reg<u8, _RXCSRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL1;
#[doc = "`read()` method returns [rxcsrl1::R](rxcsrl1::R) reader structure"]
impl crate::Readable for RXCSRL1 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl1::W](rxcsrl1::W) writer structure"]
impl crate::Writable for RXCSRL1 {}
#[doc = "USB Receive Control and Status Endpoint 1 Low"]
pub mod rxcsrl1;
#[doc = "USB Receive Control and Status Endpoint 1 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh1](rxcsrh1) module"]
pub type RXCSRH1 = crate::Reg<u8, _RXCSRH1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH1;
#[doc = "`read()` method returns [rxcsrh1::R](rxcsrh1::R) reader structure"]
impl crate::Readable for RXCSRH1 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh1::W](rxcsrh1::W) writer structure"]
impl crate::Writable for RXCSRH1 {}
#[doc = "USB Receive Control and Status Endpoint 1 High"]
pub mod rxcsrh1;
#[doc = "USB Receive Byte Count Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount1](rxcount1) module"]
pub type RXCOUNT1 = crate::Reg<u16, _RXCOUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT1;
#[doc = "`read()` method returns [rxcount1::R](rxcount1::R) reader structure"]
impl crate::Readable for RXCOUNT1 {}
#[doc = "USB Receive Byte Count Endpoint 1"]
pub mod rxcount1;
#[doc = "USB Host Transmit Configure Type Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype1](txtype1) module"]
pub type TXTYPE1 = crate::Reg<u8, _TXTYPE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE1;
#[doc = "`read()` method returns [txtype1::R](txtype1::R) reader structure"]
impl crate::Readable for TXTYPE1 {}
#[doc = "`write(|w| ..)` method takes [txtype1::W](txtype1::W) writer structure"]
impl crate::Writable for TXTYPE1 {}
#[doc = "USB Host Transmit Configure Type Endpoint 1"]
pub mod txtype1;
#[doc = "USB Host Transmit Interval Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval1](txinterval1) module"]
pub type TXINTERVAL1 = crate::Reg<u8, _TXINTERVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL1;
#[doc = "`read()` method returns [txinterval1::R](txinterval1::R) reader structure"]
impl crate::Readable for TXINTERVAL1 {}
#[doc = "`write(|w| ..)` method takes [txinterval1::W](txinterval1::W) writer structure"]
impl crate::Writable for TXINTERVAL1 {}
#[doc = "USB Host Transmit Interval Endpoint 1"]
pub mod txinterval1;
#[doc = "USB Host Configure Receive Type Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype1](rxtype1) module"]
pub type RXTYPE1 = crate::Reg<u8, _RXTYPE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE1;
#[doc = "`read()` method returns [rxtype1::R](rxtype1::R) reader structure"]
impl crate::Readable for RXTYPE1 {}
#[doc = "`write(|w| ..)` method takes [rxtype1::W](rxtype1::W) writer structure"]
impl crate::Writable for RXTYPE1 {}
#[doc = "USB Host Configure Receive Type Endpoint 1"]
pub mod rxtype1;
#[doc = "USB Host Receive Polling Interval Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval1](rxinterval1) module"]
pub type RXINTERVAL1 = crate::Reg<u8, _RXINTERVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL1;
#[doc = "`read()` method returns [rxinterval1::R](rxinterval1::R) reader structure"]
impl crate::Readable for RXINTERVAL1 {}
#[doc = "`write(|w| ..)` method takes [rxinterval1::W](rxinterval1::W) writer structure"]
impl crate::Writable for RXINTERVAL1 {}
#[doc = "USB Host Receive Polling Interval Endpoint 1"]
pub mod rxinterval1;
#[doc = "USB Maximum Transmit Data Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp2](txmaxp2) module"]
pub type TXMAXP2 = crate::Reg<u16, _TXMAXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP2;
#[doc = "`read()` method returns [txmaxp2::R](txmaxp2::R) reader structure"]
impl crate::Readable for TXMAXP2 {}
#[doc = "`write(|w| ..)` method takes [txmaxp2::W](txmaxp2::W) writer structure"]
impl crate::Writable for TXMAXP2 {}
#[doc = "USB Maximum Transmit Data Endpoint 2"]
pub mod txmaxp2;
#[doc = "USB Transmit Control and Status Endpoint 2 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl2](txcsrl2) module"]
pub type TXCSRL2 = crate::Reg<u8, _TXCSRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL2;
#[doc = "`read()` method returns [txcsrl2::R](txcsrl2::R) reader structure"]
impl crate::Readable for TXCSRL2 {}
#[doc = "`write(|w| ..)` method takes [txcsrl2::W](txcsrl2::W) writer structure"]
impl crate::Writable for TXCSRL2 {}
#[doc = "USB Transmit Control and Status Endpoint 2 Low"]
pub mod txcsrl2;
#[doc = "USB Transmit Control and Status Endpoint 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh2](txcsrh2) module"]
pub type TXCSRH2 = crate::Reg<u8, _TXCSRH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH2;
#[doc = "`read()` method returns [txcsrh2::R](txcsrh2::R) reader structure"]
impl crate::Readable for TXCSRH2 {}
#[doc = "`write(|w| ..)` method takes [txcsrh2::W](txcsrh2::W) writer structure"]
impl crate::Writable for TXCSRH2 {}
#[doc = "USB Transmit Control and Status Endpoint 2 High"]
pub mod txcsrh2;
#[doc = "USB Maximum Receive Data Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp2](rxmaxp2) module"]
pub type RXMAXP2 = crate::Reg<u16, _RXMAXP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP2;
#[doc = "`read()` method returns [rxmaxp2::R](rxmaxp2::R) reader structure"]
impl crate::Readable for RXMAXP2 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp2::W](rxmaxp2::W) writer structure"]
impl crate::Writable for RXMAXP2 {}
#[doc = "USB Maximum Receive Data Endpoint 2"]
pub mod rxmaxp2;
#[doc = "USB Receive Control and Status Endpoint 2 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl2](rxcsrl2) module"]
pub type RXCSRL2 = crate::Reg<u8, _RXCSRL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL2;
#[doc = "`read()` method returns [rxcsrl2::R](rxcsrl2::R) reader structure"]
impl crate::Readable for RXCSRL2 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl2::W](rxcsrl2::W) writer structure"]
impl crate::Writable for RXCSRL2 {}
#[doc = "USB Receive Control and Status Endpoint 2 Low"]
pub mod rxcsrl2;
#[doc = "USB Receive Control and Status Endpoint 2 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh2](rxcsrh2) module"]
pub type RXCSRH2 = crate::Reg<u8, _RXCSRH2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH2;
#[doc = "`read()` method returns [rxcsrh2::R](rxcsrh2::R) reader structure"]
impl crate::Readable for RXCSRH2 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh2::W](rxcsrh2::W) writer structure"]
impl crate::Writable for RXCSRH2 {}
#[doc = "USB Receive Control and Status Endpoint 2 High"]
pub mod rxcsrh2;
#[doc = "USB Receive Byte Count Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount2](rxcount2) module"]
pub type RXCOUNT2 = crate::Reg<u16, _RXCOUNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT2;
#[doc = "`read()` method returns [rxcount2::R](rxcount2::R) reader structure"]
impl crate::Readable for RXCOUNT2 {}
#[doc = "USB Receive Byte Count Endpoint 2"]
pub mod rxcount2;
#[doc = "USB Host Transmit Configure Type Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype2](txtype2) module"]
pub type TXTYPE2 = crate::Reg<u8, _TXTYPE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE2;
#[doc = "`read()` method returns [txtype2::R](txtype2::R) reader structure"]
impl crate::Readable for TXTYPE2 {}
#[doc = "`write(|w| ..)` method takes [txtype2::W](txtype2::W) writer structure"]
impl crate::Writable for TXTYPE2 {}
#[doc = "USB Host Transmit Configure Type Endpoint 2"]
pub mod txtype2;
#[doc = "USB Host Transmit Interval Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval2](txinterval2) module"]
pub type TXINTERVAL2 = crate::Reg<u8, _TXINTERVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL2;
#[doc = "`read()` method returns [txinterval2::R](txinterval2::R) reader structure"]
impl crate::Readable for TXINTERVAL2 {}
#[doc = "`write(|w| ..)` method takes [txinterval2::W](txinterval2::W) writer structure"]
impl crate::Writable for TXINTERVAL2 {}
#[doc = "USB Host Transmit Interval Endpoint 2"]
pub mod txinterval2;
#[doc = "USB Host Configure Receive Type Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype2](rxtype2) module"]
pub type RXTYPE2 = crate::Reg<u8, _RXTYPE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE2;
#[doc = "`read()` method returns [rxtype2::R](rxtype2::R) reader structure"]
impl crate::Readable for RXTYPE2 {}
#[doc = "`write(|w| ..)` method takes [rxtype2::W](rxtype2::W) writer structure"]
impl crate::Writable for RXTYPE2 {}
#[doc = "USB Host Configure Receive Type Endpoint 2"]
pub mod rxtype2;
#[doc = "USB Host Receive Polling Interval Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval2](rxinterval2) module"]
pub type RXINTERVAL2 = crate::Reg<u8, _RXINTERVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL2;
#[doc = "`read()` method returns [rxinterval2::R](rxinterval2::R) reader structure"]
impl crate::Readable for RXINTERVAL2 {}
#[doc = "`write(|w| ..)` method takes [rxinterval2::W](rxinterval2::W) writer structure"]
impl crate::Writable for RXINTERVAL2 {}
#[doc = "USB Host Receive Polling Interval Endpoint 2"]
pub mod rxinterval2;
#[doc = "USB Maximum Transmit Data Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp3](txmaxp3) module"]
pub type TXMAXP3 = crate::Reg<u16, _TXMAXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP3;
#[doc = "`read()` method returns [txmaxp3::R](txmaxp3::R) reader structure"]
impl crate::Readable for TXMAXP3 {}
#[doc = "`write(|w| ..)` method takes [txmaxp3::W](txmaxp3::W) writer structure"]
impl crate::Writable for TXMAXP3 {}
#[doc = "USB Maximum Transmit Data Endpoint 3"]
pub mod txmaxp3;
#[doc = "USB Transmit Control and Status Endpoint 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl3](txcsrl3) module"]
pub type TXCSRL3 = crate::Reg<u8, _TXCSRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL3;
#[doc = "`read()` method returns [txcsrl3::R](txcsrl3::R) reader structure"]
impl crate::Readable for TXCSRL3 {}
#[doc = "`write(|w| ..)` method takes [txcsrl3::W](txcsrl3::W) writer structure"]
impl crate::Writable for TXCSRL3 {}
#[doc = "USB Transmit Control and Status Endpoint 3 Low"]
pub mod txcsrl3;
#[doc = "USB Transmit Control and Status Endpoint 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh3](txcsrh3) module"]
pub type TXCSRH3 = crate::Reg<u8, _TXCSRH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH3;
#[doc = "`read()` method returns [txcsrh3::R](txcsrh3::R) reader structure"]
impl crate::Readable for TXCSRH3 {}
#[doc = "`write(|w| ..)` method takes [txcsrh3::W](txcsrh3::W) writer structure"]
impl crate::Writable for TXCSRH3 {}
#[doc = "USB Transmit Control and Status Endpoint 3 High"]
pub mod txcsrh3;
#[doc = "USB Maximum Receive Data Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp3](rxmaxp3) module"]
pub type RXMAXP3 = crate::Reg<u16, _RXMAXP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP3;
#[doc = "`read()` method returns [rxmaxp3::R](rxmaxp3::R) reader structure"]
impl crate::Readable for RXMAXP3 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp3::W](rxmaxp3::W) writer structure"]
impl crate::Writable for RXMAXP3 {}
#[doc = "USB Maximum Receive Data Endpoint 3"]
pub mod rxmaxp3;
#[doc = "USB Receive Control and Status Endpoint 3 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl3](rxcsrl3) module"]
pub type RXCSRL3 = crate::Reg<u8, _RXCSRL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL3;
#[doc = "`read()` method returns [rxcsrl3::R](rxcsrl3::R) reader structure"]
impl crate::Readable for RXCSRL3 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl3::W](rxcsrl3::W) writer structure"]
impl crate::Writable for RXCSRL3 {}
#[doc = "USB Receive Control and Status Endpoint 3 Low"]
pub mod rxcsrl3;
#[doc = "USB Receive Control and Status Endpoint 3 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh3](rxcsrh3) module"]
pub type RXCSRH3 = crate::Reg<u8, _RXCSRH3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH3;
#[doc = "`read()` method returns [rxcsrh3::R](rxcsrh3::R) reader structure"]
impl crate::Readable for RXCSRH3 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh3::W](rxcsrh3::W) writer structure"]
impl crate::Writable for RXCSRH3 {}
#[doc = "USB Receive Control and Status Endpoint 3 High"]
pub mod rxcsrh3;
#[doc = "USB Receive Byte Count Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount3](rxcount3) module"]
pub type RXCOUNT3 = crate::Reg<u16, _RXCOUNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT3;
#[doc = "`read()` method returns [rxcount3::R](rxcount3::R) reader structure"]
impl crate::Readable for RXCOUNT3 {}
#[doc = "USB Receive Byte Count Endpoint 3"]
pub mod rxcount3;
#[doc = "USB Host Transmit Configure Type Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype3](txtype3) module"]
pub type TXTYPE3 = crate::Reg<u8, _TXTYPE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE3;
#[doc = "`read()` method returns [txtype3::R](txtype3::R) reader structure"]
impl crate::Readable for TXTYPE3 {}
#[doc = "`write(|w| ..)` method takes [txtype3::W](txtype3::W) writer structure"]
impl crate::Writable for TXTYPE3 {}
#[doc = "USB Host Transmit Configure Type Endpoint 3"]
pub mod txtype3;
#[doc = "USB Host Transmit Interval Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval3](txinterval3) module"]
pub type TXINTERVAL3 = crate::Reg<u8, _TXINTERVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL3;
#[doc = "`read()` method returns [txinterval3::R](txinterval3::R) reader structure"]
impl crate::Readable for TXINTERVAL3 {}
#[doc = "`write(|w| ..)` method takes [txinterval3::W](txinterval3::W) writer structure"]
impl crate::Writable for TXINTERVAL3 {}
#[doc = "USB Host Transmit Interval Endpoint 3"]
pub mod txinterval3;
#[doc = "USB Host Configure Receive Type Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype3](rxtype3) module"]
pub type RXTYPE3 = crate::Reg<u8, _RXTYPE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE3;
#[doc = "`read()` method returns [rxtype3::R](rxtype3::R) reader structure"]
impl crate::Readable for RXTYPE3 {}
#[doc = "`write(|w| ..)` method takes [rxtype3::W](rxtype3::W) writer structure"]
impl crate::Writable for RXTYPE3 {}
#[doc = "USB Host Configure Receive Type Endpoint 3"]
pub mod rxtype3;
#[doc = "USB Host Receive Polling Interval Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval3](rxinterval3) module"]
pub type RXINTERVAL3 = crate::Reg<u8, _RXINTERVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL3;
#[doc = "`read()` method returns [rxinterval3::R](rxinterval3::R) reader structure"]
impl crate::Readable for RXINTERVAL3 {}
#[doc = "`write(|w| ..)` method takes [rxinterval3::W](rxinterval3::W) writer structure"]
impl crate::Writable for RXINTERVAL3 {}
#[doc = "USB Host Receive Polling Interval Endpoint 3"]
pub mod rxinterval3;
#[doc = "USB Maximum Transmit Data Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp4](txmaxp4) module"]
pub type TXMAXP4 = crate::Reg<u16, _TXMAXP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP4;
#[doc = "`read()` method returns [txmaxp4::R](txmaxp4::R) reader structure"]
impl crate::Readable for TXMAXP4 {}
#[doc = "`write(|w| ..)` method takes [txmaxp4::W](txmaxp4::W) writer structure"]
impl crate::Writable for TXMAXP4 {}
#[doc = "USB Maximum Transmit Data Endpoint 4"]
pub mod txmaxp4;
#[doc = "USB Transmit Control and Status Endpoint 4 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl4](txcsrl4) module"]
pub type TXCSRL4 = crate::Reg<u8, _TXCSRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL4;
#[doc = "`read()` method returns [txcsrl4::R](txcsrl4::R) reader structure"]
impl crate::Readable for TXCSRL4 {}
#[doc = "`write(|w| ..)` method takes [txcsrl4::W](txcsrl4::W) writer structure"]
impl crate::Writable for TXCSRL4 {}
#[doc = "USB Transmit Control and Status Endpoint 4 Low"]
pub mod txcsrl4;
#[doc = "USB Transmit Control and Status Endpoint 4 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh4](txcsrh4) module"]
pub type TXCSRH4 = crate::Reg<u8, _TXCSRH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH4;
#[doc = "`read()` method returns [txcsrh4::R](txcsrh4::R) reader structure"]
impl crate::Readable for TXCSRH4 {}
#[doc = "`write(|w| ..)` method takes [txcsrh4::W](txcsrh4::W) writer structure"]
impl crate::Writable for TXCSRH4 {}
#[doc = "USB Transmit Control and Status Endpoint 4 High"]
pub mod txcsrh4;
#[doc = "USB Maximum Receive Data Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp4](rxmaxp4) module"]
pub type RXMAXP4 = crate::Reg<u16, _RXMAXP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP4;
#[doc = "`read()` method returns [rxmaxp4::R](rxmaxp4::R) reader structure"]
impl crate::Readable for RXMAXP4 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp4::W](rxmaxp4::W) writer structure"]
impl crate::Writable for RXMAXP4 {}
#[doc = "USB Maximum Receive Data Endpoint 4"]
pub mod rxmaxp4;
#[doc = "USB Receive Control and Status Endpoint 4 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl4](rxcsrl4) module"]
pub type RXCSRL4 = crate::Reg<u8, _RXCSRL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL4;
#[doc = "`read()` method returns [rxcsrl4::R](rxcsrl4::R) reader structure"]
impl crate::Readable for RXCSRL4 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl4::W](rxcsrl4::W) writer structure"]
impl crate::Writable for RXCSRL4 {}
#[doc = "USB Receive Control and Status Endpoint 4 Low"]
pub mod rxcsrl4;
#[doc = "USB Receive Control and Status Endpoint 4 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh4](rxcsrh4) module"]
pub type RXCSRH4 = crate::Reg<u8, _RXCSRH4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH4;
#[doc = "`read()` method returns [rxcsrh4::R](rxcsrh4::R) reader structure"]
impl crate::Readable for RXCSRH4 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh4::W](rxcsrh4::W) writer structure"]
impl crate::Writable for RXCSRH4 {}
#[doc = "USB Receive Control and Status Endpoint 4 High"]
pub mod rxcsrh4;
#[doc = "USB Receive Byte Count Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount4](rxcount4) module"]
pub type RXCOUNT4 = crate::Reg<u16, _RXCOUNT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT4;
#[doc = "`read()` method returns [rxcount4::R](rxcount4::R) reader structure"]
impl crate::Readable for RXCOUNT4 {}
#[doc = "USB Receive Byte Count Endpoint 4"]
pub mod rxcount4;
#[doc = "USB Host Transmit Configure Type Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype4](txtype4) module"]
pub type TXTYPE4 = crate::Reg<u8, _TXTYPE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE4;
#[doc = "`read()` method returns [txtype4::R](txtype4::R) reader structure"]
impl crate::Readable for TXTYPE4 {}
#[doc = "`write(|w| ..)` method takes [txtype4::W](txtype4::W) writer structure"]
impl crate::Writable for TXTYPE4 {}
#[doc = "USB Host Transmit Configure Type Endpoint 4"]
pub mod txtype4;
#[doc = "USB Host Transmit Interval Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval4](txinterval4) module"]
pub type TXINTERVAL4 = crate::Reg<u8, _TXINTERVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL4;
#[doc = "`read()` method returns [txinterval4::R](txinterval4::R) reader structure"]
impl crate::Readable for TXINTERVAL4 {}
#[doc = "`write(|w| ..)` method takes [txinterval4::W](txinterval4::W) writer structure"]
impl crate::Writable for TXINTERVAL4 {}
#[doc = "USB Host Transmit Interval Endpoint 4"]
pub mod txinterval4;
#[doc = "USB Host Configure Receive Type Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype4](rxtype4) module"]
pub type RXTYPE4 = crate::Reg<u8, _RXTYPE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE4;
#[doc = "`read()` method returns [rxtype4::R](rxtype4::R) reader structure"]
impl crate::Readable for RXTYPE4 {}
#[doc = "`write(|w| ..)` method takes [rxtype4::W](rxtype4::W) writer structure"]
impl crate::Writable for RXTYPE4 {}
#[doc = "USB Host Configure Receive Type Endpoint 4"]
pub mod rxtype4;
#[doc = "USB Host Receive Polling Interval Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval4](rxinterval4) module"]
pub type RXINTERVAL4 = crate::Reg<u8, _RXINTERVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL4;
#[doc = "`read()` method returns [rxinterval4::R](rxinterval4::R) reader structure"]
impl crate::Readable for RXINTERVAL4 {}
#[doc = "`write(|w| ..)` method takes [rxinterval4::W](rxinterval4::W) writer structure"]
impl crate::Writable for RXINTERVAL4 {}
#[doc = "USB Host Receive Polling Interval Endpoint 4"]
pub mod rxinterval4;
#[doc = "USB Maximum Transmit Data Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp5](txmaxp5) module"]
pub type TXMAXP5 = crate::Reg<u16, _TXMAXP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP5;
#[doc = "`read()` method returns [txmaxp5::R](txmaxp5::R) reader structure"]
impl crate::Readable for TXMAXP5 {}
#[doc = "`write(|w| ..)` method takes [txmaxp5::W](txmaxp5::W) writer structure"]
impl crate::Writable for TXMAXP5 {}
#[doc = "USB Maximum Transmit Data Endpoint 5"]
pub mod txmaxp5;
#[doc = "USB Transmit Control and Status Endpoint 5 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl5](txcsrl5) module"]
pub type TXCSRL5 = crate::Reg<u8, _TXCSRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL5;
#[doc = "`read()` method returns [txcsrl5::R](txcsrl5::R) reader structure"]
impl crate::Readable for TXCSRL5 {}
#[doc = "`write(|w| ..)` method takes [txcsrl5::W](txcsrl5::W) writer structure"]
impl crate::Writable for TXCSRL5 {}
#[doc = "USB Transmit Control and Status Endpoint 5 Low"]
pub mod txcsrl5;
#[doc = "USB Transmit Control and Status Endpoint 5 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh5](txcsrh5) module"]
pub type TXCSRH5 = crate::Reg<u8, _TXCSRH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH5;
#[doc = "`read()` method returns [txcsrh5::R](txcsrh5::R) reader structure"]
impl crate::Readable for TXCSRH5 {}
#[doc = "`write(|w| ..)` method takes [txcsrh5::W](txcsrh5::W) writer structure"]
impl crate::Writable for TXCSRH5 {}
#[doc = "USB Transmit Control and Status Endpoint 5 High"]
pub mod txcsrh5;
#[doc = "USB Maximum Receive Data Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp5](rxmaxp5) module"]
pub type RXMAXP5 = crate::Reg<u16, _RXMAXP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP5;
#[doc = "`read()` method returns [rxmaxp5::R](rxmaxp5::R) reader structure"]
impl crate::Readable for RXMAXP5 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp5::W](rxmaxp5::W) writer structure"]
impl crate::Writable for RXMAXP5 {}
#[doc = "USB Maximum Receive Data Endpoint 5"]
pub mod rxmaxp5;
#[doc = "USB Receive Control and Status Endpoint 5 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl5](rxcsrl5) module"]
pub type RXCSRL5 = crate::Reg<u8, _RXCSRL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL5;
#[doc = "`read()` method returns [rxcsrl5::R](rxcsrl5::R) reader structure"]
impl crate::Readable for RXCSRL5 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl5::W](rxcsrl5::W) writer structure"]
impl crate::Writable for RXCSRL5 {}
#[doc = "USB Receive Control and Status Endpoint 5 Low"]
pub mod rxcsrl5;
#[doc = "USB Receive Control and Status Endpoint 5 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh5](rxcsrh5) module"]
pub type RXCSRH5 = crate::Reg<u8, _RXCSRH5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH5;
#[doc = "`read()` method returns [rxcsrh5::R](rxcsrh5::R) reader structure"]
impl crate::Readable for RXCSRH5 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh5::W](rxcsrh5::W) writer structure"]
impl crate::Writable for RXCSRH5 {}
#[doc = "USB Receive Control and Status Endpoint 5 High"]
pub mod rxcsrh5;
#[doc = "USB Receive Byte Count Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount5](rxcount5) module"]
pub type RXCOUNT5 = crate::Reg<u16, _RXCOUNT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT5;
#[doc = "`read()` method returns [rxcount5::R](rxcount5::R) reader structure"]
impl crate::Readable for RXCOUNT5 {}
#[doc = "USB Receive Byte Count Endpoint 5"]
pub mod rxcount5;
#[doc = "USB Host Transmit Configure Type Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype5](txtype5) module"]
pub type TXTYPE5 = crate::Reg<u8, _TXTYPE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE5;
#[doc = "`read()` method returns [txtype5::R](txtype5::R) reader structure"]
impl crate::Readable for TXTYPE5 {}
#[doc = "`write(|w| ..)` method takes [txtype5::W](txtype5::W) writer structure"]
impl crate::Writable for TXTYPE5 {}
#[doc = "USB Host Transmit Configure Type Endpoint 5"]
pub mod txtype5;
#[doc = "USB Host Transmit Interval Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval5](txinterval5) module"]
pub type TXINTERVAL5 = crate::Reg<u8, _TXINTERVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL5;
#[doc = "`read()` method returns [txinterval5::R](txinterval5::R) reader structure"]
impl crate::Readable for TXINTERVAL5 {}
#[doc = "`write(|w| ..)` method takes [txinterval5::W](txinterval5::W) writer structure"]
impl crate::Writable for TXINTERVAL5 {}
#[doc = "USB Host Transmit Interval Endpoint 5"]
pub mod txinterval5;
#[doc = "USB Host Configure Receive Type Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype5](rxtype5) module"]
pub type RXTYPE5 = crate::Reg<u8, _RXTYPE5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE5;
#[doc = "`read()` method returns [rxtype5::R](rxtype5::R) reader structure"]
impl crate::Readable for RXTYPE5 {}
#[doc = "`write(|w| ..)` method takes [rxtype5::W](rxtype5::W) writer structure"]
impl crate::Writable for RXTYPE5 {}
#[doc = "USB Host Configure Receive Type Endpoint 5"]
pub mod rxtype5;
#[doc = "USB Host Receive Polling Interval Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval5](rxinterval5) module"]
pub type RXINTERVAL5 = crate::Reg<u8, _RXINTERVAL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL5;
#[doc = "`read()` method returns [rxinterval5::R](rxinterval5::R) reader structure"]
impl crate::Readable for RXINTERVAL5 {}
#[doc = "`write(|w| ..)` method takes [rxinterval5::W](rxinterval5::W) writer structure"]
impl crate::Writable for RXINTERVAL5 {}
#[doc = "USB Host Receive Polling Interval Endpoint 5"]
pub mod rxinterval5;
#[doc = "USB Maximum Transmit Data Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp6](txmaxp6) module"]
pub type TXMAXP6 = crate::Reg<u16, _TXMAXP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP6;
#[doc = "`read()` method returns [txmaxp6::R](txmaxp6::R) reader structure"]
impl crate::Readable for TXMAXP6 {}
#[doc = "`write(|w| ..)` method takes [txmaxp6::W](txmaxp6::W) writer structure"]
impl crate::Writable for TXMAXP6 {}
#[doc = "USB Maximum Transmit Data Endpoint 6"]
pub mod txmaxp6;
#[doc = "USB Transmit Control and Status Endpoint 6 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl6](txcsrl6) module"]
pub type TXCSRL6 = crate::Reg<u8, _TXCSRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL6;
#[doc = "`read()` method returns [txcsrl6::R](txcsrl6::R) reader structure"]
impl crate::Readable for TXCSRL6 {}
#[doc = "`write(|w| ..)` method takes [txcsrl6::W](txcsrl6::W) writer structure"]
impl crate::Writable for TXCSRL6 {}
#[doc = "USB Transmit Control and Status Endpoint 6 Low"]
pub mod txcsrl6;
#[doc = "USB Transmit Control and Status Endpoint 6 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh6](txcsrh6) module"]
pub type TXCSRH6 = crate::Reg<u8, _TXCSRH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH6;
#[doc = "`read()` method returns [txcsrh6::R](txcsrh6::R) reader structure"]
impl crate::Readable for TXCSRH6 {}
#[doc = "`write(|w| ..)` method takes [txcsrh6::W](txcsrh6::W) writer structure"]
impl crate::Writable for TXCSRH6 {}
#[doc = "USB Transmit Control and Status Endpoint 6 High"]
pub mod txcsrh6;
#[doc = "USB Maximum Receive Data Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp6](rxmaxp6) module"]
pub type RXMAXP6 = crate::Reg<u16, _RXMAXP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP6;
#[doc = "`read()` method returns [rxmaxp6::R](rxmaxp6::R) reader structure"]
impl crate::Readable for RXMAXP6 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp6::W](rxmaxp6::W) writer structure"]
impl crate::Writable for RXMAXP6 {}
#[doc = "USB Maximum Receive Data Endpoint 6"]
pub mod rxmaxp6;
#[doc = "USB Receive Control and Status Endpoint 6 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl6](rxcsrl6) module"]
pub type RXCSRL6 = crate::Reg<u8, _RXCSRL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL6;
#[doc = "`read()` method returns [rxcsrl6::R](rxcsrl6::R) reader structure"]
impl crate::Readable for RXCSRL6 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl6::W](rxcsrl6::W) writer structure"]
impl crate::Writable for RXCSRL6 {}
#[doc = "USB Receive Control and Status Endpoint 6 Low"]
pub mod rxcsrl6;
#[doc = "USB Receive Control and Status Endpoint 6 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh6](rxcsrh6) module"]
pub type RXCSRH6 = crate::Reg<u8, _RXCSRH6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH6;
#[doc = "`read()` method returns [rxcsrh6::R](rxcsrh6::R) reader structure"]
impl crate::Readable for RXCSRH6 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh6::W](rxcsrh6::W) writer structure"]
impl crate::Writable for RXCSRH6 {}
#[doc = "USB Receive Control and Status Endpoint 6 High"]
pub mod rxcsrh6;
#[doc = "USB Receive Byte Count Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount6](rxcount6) module"]
pub type RXCOUNT6 = crate::Reg<u16, _RXCOUNT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT6;
#[doc = "`read()` method returns [rxcount6::R](rxcount6::R) reader structure"]
impl crate::Readable for RXCOUNT6 {}
#[doc = "USB Receive Byte Count Endpoint 6"]
pub mod rxcount6;
#[doc = "USB Host Transmit Configure Type Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype6](txtype6) module"]
pub type TXTYPE6 = crate::Reg<u8, _TXTYPE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE6;
#[doc = "`read()` method returns [txtype6::R](txtype6::R) reader structure"]
impl crate::Readable for TXTYPE6 {}
#[doc = "`write(|w| ..)` method takes [txtype6::W](txtype6::W) writer structure"]
impl crate::Writable for TXTYPE6 {}
#[doc = "USB Host Transmit Configure Type Endpoint 6"]
pub mod txtype6;
#[doc = "USB Host Transmit Interval Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval6](txinterval6) module"]
pub type TXINTERVAL6 = crate::Reg<u8, _TXINTERVAL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL6;
#[doc = "`read()` method returns [txinterval6::R](txinterval6::R) reader structure"]
impl crate::Readable for TXINTERVAL6 {}
#[doc = "`write(|w| ..)` method takes [txinterval6::W](txinterval6::W) writer structure"]
impl crate::Writable for TXINTERVAL6 {}
#[doc = "USB Host Transmit Interval Endpoint 6"]
pub mod txinterval6;
#[doc = "USB Host Configure Receive Type Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype6](rxtype6) module"]
pub type RXTYPE6 = crate::Reg<u8, _RXTYPE6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE6;
#[doc = "`read()` method returns [rxtype6::R](rxtype6::R) reader structure"]
impl crate::Readable for RXTYPE6 {}
#[doc = "`write(|w| ..)` method takes [rxtype6::W](rxtype6::W) writer structure"]
impl crate::Writable for RXTYPE6 {}
#[doc = "USB Host Configure Receive Type Endpoint 6"]
pub mod rxtype6;
#[doc = "USB Host Receive Polling Interval Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval6](rxinterval6) module"]
pub type RXINTERVAL6 = crate::Reg<u8, _RXINTERVAL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL6;
#[doc = "`read()` method returns [rxinterval6::R](rxinterval6::R) reader structure"]
impl crate::Readable for RXINTERVAL6 {}
#[doc = "`write(|w| ..)` method takes [rxinterval6::W](rxinterval6::W) writer structure"]
impl crate::Writable for RXINTERVAL6 {}
#[doc = "USB Host Receive Polling Interval Endpoint 6"]
pub mod rxinterval6;
#[doc = "USB Maximum Transmit Data Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmaxp7](txmaxp7) module"]
pub type TXMAXP7 = crate::Reg<u16, _TXMAXP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXMAXP7;
#[doc = "`read()` method returns [txmaxp7::R](txmaxp7::R) reader structure"]
impl crate::Readable for TXMAXP7 {}
#[doc = "`write(|w| ..)` method takes [txmaxp7::W](txmaxp7::W) writer structure"]
impl crate::Writable for TXMAXP7 {}
#[doc = "USB Maximum Transmit Data Endpoint 7"]
pub mod txmaxp7;
#[doc = "USB Transmit Control and Status Endpoint 7 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrl7](txcsrl7) module"]
pub type TXCSRL7 = crate::Reg<u8, _TXCSRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRL7;
#[doc = "`read()` method returns [txcsrl7::R](txcsrl7::R) reader structure"]
impl crate::Readable for TXCSRL7 {}
#[doc = "`write(|w| ..)` method takes [txcsrl7::W](txcsrl7::W) writer structure"]
impl crate::Writable for TXCSRL7 {}
#[doc = "USB Transmit Control and Status Endpoint 7 Low"]
pub mod txcsrl7;
#[doc = "USB Transmit Control and Status Endpoint 7 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txcsrh7](txcsrh7) module"]
pub type TXCSRH7 = crate::Reg<u8, _TXCSRH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCSRH7;
#[doc = "`read()` method returns [txcsrh7::R](txcsrh7::R) reader structure"]
impl crate::Readable for TXCSRH7 {}
#[doc = "`write(|w| ..)` method takes [txcsrh7::W](txcsrh7::W) writer structure"]
impl crate::Writable for TXCSRH7 {}
#[doc = "USB Transmit Control and Status Endpoint 7 High"]
pub mod txcsrh7;
#[doc = "USB Maximum Receive Data Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaxp7](rxmaxp7) module"]
pub type RXMAXP7 = crate::Reg<u16, _RXMAXP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXMAXP7;
#[doc = "`read()` method returns [rxmaxp7::R](rxmaxp7::R) reader structure"]
impl crate::Readable for RXMAXP7 {}
#[doc = "`write(|w| ..)` method takes [rxmaxp7::W](rxmaxp7::W) writer structure"]
impl crate::Writable for RXMAXP7 {}
#[doc = "USB Maximum Receive Data Endpoint 7"]
pub mod rxmaxp7;
#[doc = "USB Receive Control and Status Endpoint 7 Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrl7](rxcsrl7) module"]
pub type RXCSRL7 = crate::Reg<u8, _RXCSRL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRL7;
#[doc = "`read()` method returns [rxcsrl7::R](rxcsrl7::R) reader structure"]
impl crate::Readable for RXCSRL7 {}
#[doc = "`write(|w| ..)` method takes [rxcsrl7::W](rxcsrl7::W) writer structure"]
impl crate::Writable for RXCSRL7 {}
#[doc = "USB Receive Control and Status Endpoint 7 Low"]
pub mod rxcsrl7;
#[doc = "USB Receive Control and Status Endpoint 7 High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcsrh7](rxcsrh7) module"]
pub type RXCSRH7 = crate::Reg<u8, _RXCSRH7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCSRH7;
#[doc = "`read()` method returns [rxcsrh7::R](rxcsrh7::R) reader structure"]
impl crate::Readable for RXCSRH7 {}
#[doc = "`write(|w| ..)` method takes [rxcsrh7::W](rxcsrh7::W) writer structure"]
impl crate::Writable for RXCSRH7 {}
#[doc = "USB Receive Control and Status Endpoint 7 High"]
pub mod rxcsrh7;
#[doc = "USB Receive Byte Count Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcount7](rxcount7) module"]
pub type RXCOUNT7 = crate::Reg<u16, _RXCOUNT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCOUNT7;
#[doc = "`read()` method returns [rxcount7::R](rxcount7::R) reader structure"]
impl crate::Readable for RXCOUNT7 {}
#[doc = "USB Receive Byte Count Endpoint 7"]
pub mod rxcount7;
#[doc = "USB Host Transmit Configure Type Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txtype7](txtype7) module"]
pub type TXTYPE7 = crate::Reg<u8, _TXTYPE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXTYPE7;
#[doc = "`read()` method returns [txtype7::R](txtype7::R) reader structure"]
impl crate::Readable for TXTYPE7 {}
#[doc = "`write(|w| ..)` method takes [txtype7::W](txtype7::W) writer structure"]
impl crate::Writable for TXTYPE7 {}
#[doc = "USB Host Transmit Configure Type Endpoint 7"]
pub mod txtype7;
#[doc = "USB Host Transmit Interval Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txinterval7](txinterval7) module"]
pub type TXINTERVAL7 = crate::Reg<u8, _TXINTERVAL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXINTERVAL7;
#[doc = "`read()` method returns [txinterval7::R](txinterval7::R) reader structure"]
impl crate::Readable for TXINTERVAL7 {}
#[doc = "`write(|w| ..)` method takes [txinterval7::W](txinterval7::W) writer structure"]
impl crate::Writable for TXINTERVAL7 {}
#[doc = "USB Host Transmit Interval Endpoint 7"]
pub mod txinterval7;
#[doc = "USB Host Configure Receive Type Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtype7](rxtype7) module"]
pub type RXTYPE7 = crate::Reg<u8, _RXTYPE7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTYPE7;
#[doc = "`read()` method returns [rxtype7::R](rxtype7::R) reader structure"]
impl crate::Readable for RXTYPE7 {}
#[doc = "`write(|w| ..)` method takes [rxtype7::W](rxtype7::W) writer structure"]
impl crate::Writable for RXTYPE7 {}
#[doc = "USB Host Configure Receive Type Endpoint 7"]
pub mod rxtype7;
#[doc = "USB Host Receive Polling Interval Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxinterval7](rxinterval7) module"]
pub type RXINTERVAL7 = crate::Reg<u8, _RXINTERVAL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXINTERVAL7;
#[doc = "`read()` method returns [rxinterval7::R](rxinterval7::R) reader structure"]
impl crate::Readable for RXINTERVAL7 {}
#[doc = "`write(|w| ..)` method takes [rxinterval7::W](rxinterval7::W) writer structure"]
impl crate::Writable for RXINTERVAL7 {}
#[doc = "USB Host Receive Polling Interval Endpoint 7"]
pub mod rxinterval7;
#[doc = "USB DMA Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaintr](dmaintr) module"]
pub type DMAINTR = crate::Reg<u8, _DMAINTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTR;
#[doc = "`read()` method returns [dmaintr::R](dmaintr::R) reader structure"]
impl crate::Readable for DMAINTR {}
#[doc = "`write(|w| ..)` method takes [dmaintr::W](dmaintr::W) writer structure"]
impl crate::Writable for DMAINTR {}
#[doc = "USB DMA Interrupt"]
pub mod dmaintr;
#[doc = "USB DMA Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl0](dmactl0) module"]
pub type DMACTL0 = crate::Reg<u16, _DMACTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL0;
#[doc = "`read()` method returns [dmactl0::R](dmactl0::R) reader structure"]
impl crate::Readable for DMACTL0 {}
#[doc = "`write(|w| ..)` method takes [dmactl0::W](dmactl0::W) writer structure"]
impl crate::Writable for DMACTL0 {}
#[doc = "USB DMA Control 0"]
pub mod dmactl0;
#[doc = "USB DMA Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr0](dmaaddr0) module"]
pub type DMAADDR0 = crate::Reg<u32, _DMAADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR0;
#[doc = "`read()` method returns [dmaaddr0::R](dmaaddr0::R) reader structure"]
impl crate::Readable for DMAADDR0 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr0::W](dmaaddr0::W) writer structure"]
impl crate::Writable for DMAADDR0 {}
#[doc = "USB DMA Address 0"]
pub mod dmaaddr0;
#[doc = "USB DMA Count 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount0](dmacount0) module"]
pub type DMACOUNT0 = crate::Reg<u32, _DMACOUNT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT0;
#[doc = "`read()` method returns [dmacount0::R](dmacount0::R) reader structure"]
impl crate::Readable for DMACOUNT0 {}
#[doc = "`write(|w| ..)` method takes [dmacount0::W](dmacount0::W) writer structure"]
impl crate::Writable for DMACOUNT0 {}
#[doc = "USB DMA Count 0"]
pub mod dmacount0;
#[doc = "USB DMA Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl1](dmactl1) module"]
pub type DMACTL1 = crate::Reg<u16, _DMACTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL1;
#[doc = "`read()` method returns [dmactl1::R](dmactl1::R) reader structure"]
impl crate::Readable for DMACTL1 {}
#[doc = "`write(|w| ..)` method takes [dmactl1::W](dmactl1::W) writer structure"]
impl crate::Writable for DMACTL1 {}
#[doc = "USB DMA Control 1"]
pub mod dmactl1;
#[doc = "USB DMA Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr1](dmaaddr1) module"]
pub type DMAADDR1 = crate::Reg<u32, _DMAADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR1;
#[doc = "`read()` method returns [dmaaddr1::R](dmaaddr1::R) reader structure"]
impl crate::Readable for DMAADDR1 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr1::W](dmaaddr1::W) writer structure"]
impl crate::Writable for DMAADDR1 {}
#[doc = "USB DMA Address 1"]
pub mod dmaaddr1;
#[doc = "USB DMA Count 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount1](dmacount1) module"]
pub type DMACOUNT1 = crate::Reg<u32, _DMACOUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT1;
#[doc = "`read()` method returns [dmacount1::R](dmacount1::R) reader structure"]
impl crate::Readable for DMACOUNT1 {}
#[doc = "`write(|w| ..)` method takes [dmacount1::W](dmacount1::W) writer structure"]
impl crate::Writable for DMACOUNT1 {}
#[doc = "USB DMA Count 1"]
pub mod dmacount1;
#[doc = "USB DMA Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl2](dmactl2) module"]
pub type DMACTL2 = crate::Reg<u16, _DMACTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL2;
#[doc = "`read()` method returns [dmactl2::R](dmactl2::R) reader structure"]
impl crate::Readable for DMACTL2 {}
#[doc = "`write(|w| ..)` method takes [dmactl2::W](dmactl2::W) writer structure"]
impl crate::Writable for DMACTL2 {}
#[doc = "USB DMA Control 2"]
pub mod dmactl2;
#[doc = "USB DMA Address 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr2](dmaaddr2) module"]
pub type DMAADDR2 = crate::Reg<u32, _DMAADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR2;
#[doc = "`read()` method returns [dmaaddr2::R](dmaaddr2::R) reader structure"]
impl crate::Readable for DMAADDR2 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr2::W](dmaaddr2::W) writer structure"]
impl crate::Writable for DMAADDR2 {}
#[doc = "USB DMA Address 2"]
pub mod dmaaddr2;
#[doc = "USB DMA Count 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount2](dmacount2) module"]
pub type DMACOUNT2 = crate::Reg<u32, _DMACOUNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT2;
#[doc = "`read()` method returns [dmacount2::R](dmacount2::R) reader structure"]
impl crate::Readable for DMACOUNT2 {}
#[doc = "`write(|w| ..)` method takes [dmacount2::W](dmacount2::W) writer structure"]
impl crate::Writable for DMACOUNT2 {}
#[doc = "USB DMA Count 2"]
pub mod dmacount2;
#[doc = "USB DMA Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl3](dmactl3) module"]
pub type DMACTL3 = crate::Reg<u16, _DMACTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL3;
#[doc = "`read()` method returns [dmactl3::R](dmactl3::R) reader structure"]
impl crate::Readable for DMACTL3 {}
#[doc = "`write(|w| ..)` method takes [dmactl3::W](dmactl3::W) writer structure"]
impl crate::Writable for DMACTL3 {}
#[doc = "USB DMA Control 3"]
pub mod dmactl3;
#[doc = "USB DMA Address 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr3](dmaaddr3) module"]
pub type DMAADDR3 = crate::Reg<u32, _DMAADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR3;
#[doc = "`read()` method returns [dmaaddr3::R](dmaaddr3::R) reader structure"]
impl crate::Readable for DMAADDR3 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr3::W](dmaaddr3::W) writer structure"]
impl crate::Writable for DMAADDR3 {}
#[doc = "USB DMA Address 3"]
pub mod dmaaddr3;
#[doc = "USB DMA Count 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount3](dmacount3) module"]
pub type DMACOUNT3 = crate::Reg<u32, _DMACOUNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT3;
#[doc = "`read()` method returns [dmacount3::R](dmacount3::R) reader structure"]
impl crate::Readable for DMACOUNT3 {}
#[doc = "`write(|w| ..)` method takes [dmacount3::W](dmacount3::W) writer structure"]
impl crate::Writable for DMACOUNT3 {}
#[doc = "USB DMA Count 3"]
pub mod dmacount3;
#[doc = "USB DMA Control 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl4](dmactl4) module"]
pub type DMACTL4 = crate::Reg<u16, _DMACTL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL4;
#[doc = "`read()` method returns [dmactl4::R](dmactl4::R) reader structure"]
impl crate::Readable for DMACTL4 {}
#[doc = "`write(|w| ..)` method takes [dmactl4::W](dmactl4::W) writer structure"]
impl crate::Writable for DMACTL4 {}
#[doc = "USB DMA Control 4"]
pub mod dmactl4;
#[doc = "USB DMA Address 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr4](dmaaddr4) module"]
pub type DMAADDR4 = crate::Reg<u32, _DMAADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR4;
#[doc = "`read()` method returns [dmaaddr4::R](dmaaddr4::R) reader structure"]
impl crate::Readable for DMAADDR4 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr4::W](dmaaddr4::W) writer structure"]
impl crate::Writable for DMAADDR4 {}
#[doc = "USB DMA Address 4"]
pub mod dmaaddr4;
#[doc = "USB DMA Count 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount4](dmacount4) module"]
pub type DMACOUNT4 = crate::Reg<u32, _DMACOUNT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT4;
#[doc = "`read()` method returns [dmacount4::R](dmacount4::R) reader structure"]
impl crate::Readable for DMACOUNT4 {}
#[doc = "`write(|w| ..)` method takes [dmacount4::W](dmacount4::W) writer structure"]
impl crate::Writable for DMACOUNT4 {}
#[doc = "USB DMA Count 4"]
pub mod dmacount4;
#[doc = "USB DMA Control 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl5](dmactl5) module"]
pub type DMACTL5 = crate::Reg<u16, _DMACTL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL5;
#[doc = "`read()` method returns [dmactl5::R](dmactl5::R) reader structure"]
impl crate::Readable for DMACTL5 {}
#[doc = "`write(|w| ..)` method takes [dmactl5::W](dmactl5::W) writer structure"]
impl crate::Writable for DMACTL5 {}
#[doc = "USB DMA Control 5"]
pub mod dmactl5;
#[doc = "USB DMA Address 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr5](dmaaddr5) module"]
pub type DMAADDR5 = crate::Reg<u32, _DMAADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR5;
#[doc = "`read()` method returns [dmaaddr5::R](dmaaddr5::R) reader structure"]
impl crate::Readable for DMAADDR5 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr5::W](dmaaddr5::W) writer structure"]
impl crate::Writable for DMAADDR5 {}
#[doc = "USB DMA Address 5"]
pub mod dmaaddr5;
#[doc = "USB DMA Count 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount5](dmacount5) module"]
pub type DMACOUNT5 = crate::Reg<u32, _DMACOUNT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT5;
#[doc = "`read()` method returns [dmacount5::R](dmacount5::R) reader structure"]
impl crate::Readable for DMACOUNT5 {}
#[doc = "`write(|w| ..)` method takes [dmacount5::W](dmacount5::W) writer structure"]
impl crate::Writable for DMACOUNT5 {}
#[doc = "USB DMA Count 5"]
pub mod dmacount5;
#[doc = "USB DMA Control 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl6](dmactl6) module"]
pub type DMACTL6 = crate::Reg<u16, _DMACTL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL6;
#[doc = "`read()` method returns [dmactl6::R](dmactl6::R) reader structure"]
impl crate::Readable for DMACTL6 {}
#[doc = "`write(|w| ..)` method takes [dmactl6::W](dmactl6::W) writer structure"]
impl crate::Writable for DMACTL6 {}
#[doc = "USB DMA Control 6"]
pub mod dmactl6;
#[doc = "USB DMA Address 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr6](dmaaddr6) module"]
pub type DMAADDR6 = crate::Reg<u32, _DMAADDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR6;
#[doc = "`read()` method returns [dmaaddr6::R](dmaaddr6::R) reader structure"]
impl crate::Readable for DMAADDR6 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr6::W](dmaaddr6::W) writer structure"]
impl crate::Writable for DMAADDR6 {}
#[doc = "USB DMA Address 6"]
pub mod dmaaddr6;
#[doc = "USB DMA Count 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount6](dmacount6) module"]
pub type DMACOUNT6 = crate::Reg<u32, _DMACOUNT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT6;
#[doc = "`read()` method returns [dmacount6::R](dmacount6::R) reader structure"]
impl crate::Readable for DMACOUNT6 {}
#[doc = "`write(|w| ..)` method takes [dmacount6::W](dmacount6::W) writer structure"]
impl crate::Writable for DMACOUNT6 {}
#[doc = "USB DMA Count 6"]
pub mod dmacount6;
#[doc = "USB DMA Control 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmactl7](dmactl7) module"]
pub type DMACTL7 = crate::Reg<u16, _DMACTL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACTL7;
#[doc = "`read()` method returns [dmactl7::R](dmactl7::R) reader structure"]
impl crate::Readable for DMACTL7 {}
#[doc = "`write(|w| ..)` method takes [dmactl7::W](dmactl7::W) writer structure"]
impl crate::Writable for DMACTL7 {}
#[doc = "USB DMA Control 7"]
pub mod dmactl7;
#[doc = "USB DMA Address 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr7](dmaaddr7) module"]
pub type DMAADDR7 = crate::Reg<u32, _DMAADDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR7;
#[doc = "`read()` method returns [dmaaddr7::R](dmaaddr7::R) reader structure"]
impl crate::Readable for DMAADDR7 {}
#[doc = "`write(|w| ..)` method takes [dmaaddr7::W](dmaaddr7::W) writer structure"]
impl crate::Writable for DMAADDR7 {}
#[doc = "USB DMA Address 7"]
pub mod dmaaddr7;
#[doc = "USB DMA Count 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacount7](dmacount7) module"]
pub type DMACOUNT7 = crate::Reg<u32, _DMACOUNT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACOUNT7;
#[doc = "`read()` method returns [dmacount7::R](dmacount7::R) reader structure"]
impl crate::Readable for DMACOUNT7 {}
#[doc = "`write(|w| ..)` method takes [dmacount7::W](dmacount7::W) writer structure"]
impl crate::Writable for DMACOUNT7 {}
#[doc = "USB DMA Count 7"]
pub mod dmacount7;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount1](rqpktcount1) module"]
pub type RQPKTCOUNT1 = crate::Reg<u16, _RQPKTCOUNT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT1;
#[doc = "`read()` method returns [rqpktcount1::R](rqpktcount1::R) reader structure"]
impl crate::Readable for RQPKTCOUNT1 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount1::W](rqpktcount1::W) writer structure"]
impl crate::Writable for RQPKTCOUNT1 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 1"]
pub mod rqpktcount1;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount2](rqpktcount2) module"]
pub type RQPKTCOUNT2 = crate::Reg<u16, _RQPKTCOUNT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT2;
#[doc = "`read()` method returns [rqpktcount2::R](rqpktcount2::R) reader structure"]
impl crate::Readable for RQPKTCOUNT2 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount2::W](rqpktcount2::W) writer structure"]
impl crate::Writable for RQPKTCOUNT2 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 2"]
pub mod rqpktcount2;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount3](rqpktcount3) module"]
pub type RQPKTCOUNT3 = crate::Reg<u16, _RQPKTCOUNT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT3;
#[doc = "`read()` method returns [rqpktcount3::R](rqpktcount3::R) reader structure"]
impl crate::Readable for RQPKTCOUNT3 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount3::W](rqpktcount3::W) writer structure"]
impl crate::Writable for RQPKTCOUNT3 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 3"]
pub mod rqpktcount3;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount4](rqpktcount4) module"]
pub type RQPKTCOUNT4 = crate::Reg<u16, _RQPKTCOUNT4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT4;
#[doc = "`read()` method returns [rqpktcount4::R](rqpktcount4::R) reader structure"]
impl crate::Readable for RQPKTCOUNT4 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount4::W](rqpktcount4::W) writer structure"]
impl crate::Writable for RQPKTCOUNT4 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 4"]
pub mod rqpktcount4;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount5](rqpktcount5) module"]
pub type RQPKTCOUNT5 = crate::Reg<u16, _RQPKTCOUNT5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT5;
#[doc = "`read()` method returns [rqpktcount5::R](rqpktcount5::R) reader structure"]
impl crate::Readable for RQPKTCOUNT5 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount5::W](rqpktcount5::W) writer structure"]
impl crate::Writable for RQPKTCOUNT5 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 5"]
pub mod rqpktcount5;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount6](rqpktcount6) module"]
pub type RQPKTCOUNT6 = crate::Reg<u16, _RQPKTCOUNT6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT6;
#[doc = "`read()` method returns [rqpktcount6::R](rqpktcount6::R) reader structure"]
impl crate::Readable for RQPKTCOUNT6 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount6::W](rqpktcount6::W) writer structure"]
impl crate::Writable for RQPKTCOUNT6 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 6"]
pub mod rqpktcount6;
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rqpktcount7](rqpktcount7) module"]
pub type RQPKTCOUNT7 = crate::Reg<u16, _RQPKTCOUNT7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RQPKTCOUNT7;
#[doc = "`read()` method returns [rqpktcount7::R](rqpktcount7::R) reader structure"]
impl crate::Readable for RQPKTCOUNT7 {}
#[doc = "`write(|w| ..)` method takes [rqpktcount7::W](rqpktcount7::W) writer structure"]
impl crate::Writable for RQPKTCOUNT7 {}
#[doc = "USB Request Packet Count in Block Transfer Endpoint 7"]
pub mod rqpktcount7;
#[doc = "USB Receive Double Packet Buffer Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdpktbufdis](rxdpktbufdis) module"]
pub type RXDPKTBUFDIS = crate::Reg<u16, _RXDPKTBUFDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDPKTBUFDIS;
#[doc = "`read()` method returns [rxdpktbufdis::R](rxdpktbufdis::R) reader structure"]
impl crate::Readable for RXDPKTBUFDIS {}
#[doc = "`write(|w| ..)` method takes [rxdpktbufdis::W](rxdpktbufdis::W) writer structure"]
impl crate::Writable for RXDPKTBUFDIS {}
#[doc = "USB Receive Double Packet Buffer Disable"]
pub mod rxdpktbufdis;
#[doc = "USB Transmit Double Packet Buffer Disable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdpktbufdis](txdpktbufdis) module"]
pub type TXDPKTBUFDIS = crate::Reg<u16, _TXDPKTBUFDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDPKTBUFDIS;
#[doc = "`read()` method returns [txdpktbufdis::R](txdpktbufdis::R) reader structure"]
impl crate::Readable for TXDPKTBUFDIS {}
#[doc = "`write(|w| ..)` method takes [txdpktbufdis::W](txdpktbufdis::W) writer structure"]
impl crate::Writable for TXDPKTBUFDIS {}
#[doc = "USB Transmit Double Packet Buffer Disable"]
pub mod txdpktbufdis;
#[doc = "USB Chirp Timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cto](cto) module"]
pub type CTO = crate::Reg<u16, _CTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTO;
#[doc = "`read()` method returns [cto::R](cto::R) reader structure"]
impl crate::Readable for CTO {}
#[doc = "`write(|w| ..)` method takes [cto::W](cto::W) writer structure"]
impl crate::Writable for CTO {}
#[doc = "USB Chirp Timeout"]
pub mod cto;
#[doc = "USB High Speed to UTM Operating Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hhsrtn](hhsrtn) module"]
pub type HHSRTN = crate::Reg<u16, _HHSRTN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HHSRTN;
#[doc = "`read()` method returns [hhsrtn::R](hhsrtn::R) reader structure"]
impl crate::Readable for HHSRTN {}
#[doc = "`write(|w| ..)` method takes [hhsrtn::W](hhsrtn::W) writer structure"]
impl crate::Writable for HHSRTN {}
#[doc = "USB High Speed to UTM Operating Delay"]
pub mod hhsrtn;
#[doc = "USB High Speed Time-out Adder\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbt](hsbt) module"]
pub type HSBT = crate::Reg<u16, _HSBT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSBT;
#[doc = "`read()` method returns [hsbt::R](hsbt::R) reader structure"]
impl crate::Readable for HSBT {}
#[doc = "`write(|w| ..)` method takes [hsbt::W](hsbt::W) writer structure"]
impl crate::Writable for HSBT {}
#[doc = "USB High Speed Time-out Adder"]
pub mod hsbt;
#[doc = "USB LPM Attributes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmattr](lpmattr) module"]
pub type LPMATTR = crate::Reg<u16, _LPMATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMATTR;
#[doc = "`read()` method returns [lpmattr::R](lpmattr::R) reader structure"]
impl crate::Readable for LPMATTR {}
#[doc = "USB LPM Attributes"]
pub mod lpmattr;
#[doc = "USB LPM Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmcntrl](lpmcntrl) module"]
pub type LPMCNTRL = crate::Reg<u8, _LPMCNTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMCNTRL;
#[doc = "`read()` method returns [lpmcntrl::R](lpmcntrl::R) reader structure"]
impl crate::Readable for LPMCNTRL {}
#[doc = "USB LPM Control"]
pub mod lpmcntrl;
#[doc = "USB LPM Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmim](lpmim) module"]
pub type LPMIM = crate::Reg<u8, _LPMIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMIM;
#[doc = "`read()` method returns [lpmim::R](lpmim::R) reader structure"]
impl crate::Readable for LPMIM {}
#[doc = "USB LPM Interrupt Mask"]
pub mod lpmim;
#[doc = "USB LPM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmris](lpmris) module"]
pub type LPMRIS = crate::Reg<u8, _LPMRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMRIS;
#[doc = "`read()` method returns [lpmris::R](lpmris::R) reader structure"]
impl crate::Readable for LPMRIS {}
#[doc = "USB LPM Raw Interrupt Status"]
pub mod lpmris;
#[doc = "USB LPM Function Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmfaddr](lpmfaddr) module"]
pub type LPMFADDR = crate::Reg<u8, _LPMFADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LPMFADDR;
#[doc = "`read()` method returns [lpmfaddr::R](lpmfaddr::R) reader structure"]
impl crate::Readable for LPMFADDR {}
#[doc = "`write(|w| ..)` method takes [lpmfaddr::W](lpmfaddr::W) writer structure"]
impl crate::Writable for LPMFADDR {}
#[doc = "USB LPM Function Address"]
pub mod lpmfaddr;
#[doc = "USB External Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epc](epc) module"]
pub type EPC = crate::Reg<u32, _EPC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPC;
#[doc = "`read()` method returns [epc::R](epc::R) reader structure"]
impl crate::Readable for EPC {}
#[doc = "`write(|w| ..)` method takes [epc::W](epc::W) writer structure"]
impl crate::Writable for EPC {}
#[doc = "USB External Power Control"]
pub mod epc;
#[doc = "USB External Power Control Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcris](epcris) module"]
pub type EPCRIS = crate::Reg<u32, _EPCRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCRIS;
#[doc = "`read()` method returns [epcris::R](epcris::R) reader structure"]
impl crate::Readable for EPCRIS {}
#[doc = "USB External Power Control Raw Interrupt Status"]
pub mod epcris;
#[doc = "USB External Power Control Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcim](epcim) module"]
pub type EPCIM = crate::Reg<u32, _EPCIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCIM;
#[doc = "`read()` method returns [epcim::R](epcim::R) reader structure"]
impl crate::Readable for EPCIM {}
#[doc = "`write(|w| ..)` method takes [epcim::W](epcim::W) writer structure"]
impl crate::Writable for EPCIM {}
#[doc = "USB External Power Control Interrupt Mask"]
pub mod epcim;
#[doc = "USB External Power Control Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epcisc](epcisc) module"]
pub type EPCISC = crate::Reg<u32, _EPCISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPCISC;
#[doc = "`read()` method returns [epcisc::R](epcisc::R) reader structure"]
impl crate::Readable for EPCISC {}
#[doc = "`write(|w| ..)` method takes [epcisc::W](epcisc::W) writer structure"]
impl crate::Writable for EPCISC {}
#[doc = "USB External Power Control Interrupt Status and Clear"]
pub mod epcisc;
#[doc = "USB Device RESUME Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drris](drris) module"]
pub type DRRIS = crate::Reg<u32, _DRRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRRIS;
#[doc = "`read()` method returns [drris::R](drris::R) reader structure"]
impl crate::Readable for DRRIS {}
#[doc = "USB Device RESUME Raw Interrupt Status"]
pub mod drris;
#[doc = "USB Device RESUME Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drim](drim) module"]
pub type DRIM = crate::Reg<u32, _DRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRIM;
#[doc = "`read()` method returns [drim::R](drim::R) reader structure"]
impl crate::Readable for DRIM {}
#[doc = "`write(|w| ..)` method takes [drim::W](drim::W) writer structure"]
impl crate::Writable for DRIM {}
#[doc = "USB Device RESUME Interrupt Mask"]
pub mod drim;
#[doc = "USB Device RESUME Interrupt Status and Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drisc](drisc) module"]
pub type DRISC = crate::Reg<u32, _DRISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DRISC;
#[doc = "`write(|w| ..)` method takes [drisc::W](drisc::W) writer structure"]
impl crate::Writable for DRISC {}
#[doc = "USB Device RESUME Interrupt Status and Clear"]
pub mod drisc;
#[doc = "USB General-Purpose Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpcs](gpcs) module"]
pub type GPCS = crate::Reg<u32, _GPCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPCS;
#[doc = "`read()` method returns [gpcs::R](gpcs::R) reader structure"]
impl crate::Readable for GPCS {}
#[doc = "`write(|w| ..)` method takes [gpcs::W](gpcs::W) writer structure"]
impl crate::Writable for GPCS {}
#[doc = "USB General-Purpose Control and Status"]
pub mod gpcs;
#[doc = "USB VBUS Droop Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdc](vdc) module"]
pub type VDC = crate::Reg<u32, _VDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDC;
#[doc = "`read()` method returns [vdc::R](vdc::R) reader structure"]
impl crate::Readable for VDC {}
#[doc = "`write(|w| ..)` method takes [vdc::W](vdc::W) writer structure"]
impl crate::Writable for VDC {}
#[doc = "USB VBUS Droop Control"]
pub mod vdc;
#[doc = "USB VBUS Droop Control Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcris](vdcris) module"]
pub type VDCRIS = crate::Reg<u32, _VDCRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCRIS;
#[doc = "`read()` method returns [vdcris::R](vdcris::R) reader structure"]
impl crate::Readable for VDCRIS {}
#[doc = "USB VBUS Droop Control Raw Interrupt Status"]
pub mod vdcris;
#[doc = "USB VBUS Droop Control Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcim](vdcim) module"]
pub type VDCIM = crate::Reg<u32, _VDCIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCIM;
#[doc = "`read()` method returns [vdcim::R](vdcim::R) reader structure"]
impl crate::Readable for VDCIM {}
#[doc = "`write(|w| ..)` method takes [vdcim::W](vdcim::W) writer structure"]
impl crate::Writable for VDCIM {}
#[doc = "USB VBUS Droop Control Interrupt Mask"]
pub mod vdcim;
#[doc = "USB VBUS Droop Control Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdcisc](vdcisc) module"]
pub type VDCISC = crate::Reg<u32, _VDCISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDCISC;
#[doc = "`read()` method returns [vdcisc::R](vdcisc::R) reader structure"]
impl crate::Readable for VDCISC {}
#[doc = "`write(|w| ..)` method takes [vdcisc::W](vdcisc::W) writer structure"]
impl crate::Writable for VDCISC {}
#[doc = "USB VBUS Droop Control Interrupt Status and Clear"]
pub mod vdcisc;
#[doc = "USB Peripheral Properties\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](pp) module"]
pub type PP = crate::Reg<u32, _PP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PP;
#[doc = "`read()` method returns [pp::R](pp::R) reader structure"]
impl crate::Readable for PP {}
#[doc = "USB Peripheral Properties"]
pub mod pp;
#[doc = "USB Peripheral Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc](pc) module"]
pub type PC = crate::Reg<u32, _PC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PC;
#[doc = "`read()` method returns [pc::R](pc::R) reader structure"]
impl crate::Readable for PC {}
#[doc = "`write(|w| ..)` method takes [pc::W](pc::W) writer structure"]
impl crate::Writable for PC {}
#[doc = "USB Peripheral Configuration"]
pub mod pc;
#[doc = "USB Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "USB Clock Configuration"]
pub mod cc;
