# [ doc = "DMA controller" ]
# [ repr ( C ) ]
pub struct Dma2 {
    # [ doc = "0x00 - low interrupt status register" ]
    pub lisr: Lisr,
    # [ doc = "0x04 - high interrupt status register" ]
    pub hisr: Hisr,
    # [ doc = "0x08 - low interrupt flag clear register" ]
    pub lifcr: Lifcr,
    # [ doc = "0x0c - high interrupt flag clear register" ]
    pub hifcr: Hifcr,
    # [ doc = "0x10 - stream x configuration register" ]
    pub s0cr: S0cr,
    # [ doc = "0x14 - stream x number of data register" ]
    pub s0ndtr: S0ndtr,
    # [ doc = "0x18 - stream x peripheral address register" ]
    pub s0par: S0par,
    # [ doc = "0x1c - stream x memory 0 address register" ]
    pub s0m0ar: S0m0ar,
    # [ doc = "0x20 - stream x memory 1 address register" ]
    pub s0m1ar: S0m1ar,
    # [ doc = "0x24 - stream x FIFO control register" ]
    pub s0fcr: S0fcr,
    # [ doc = "0x28 - stream x configuration register" ]
    pub s1cr: S1cr,
    # [ doc = "0x2c - stream x number of data register" ]
    pub s1ndtr: S1ndtr,
    # [ doc = "0x30 - stream x peripheral address register" ]
    pub s1par: S1par,
    # [ doc = "0x34 - stream x memory 0 address register" ]
    pub s1m0ar: S1m0ar,
    # [ doc = "0x38 - stream x memory 1 address register" ]
    pub s1m1ar: S1m1ar,
    # [ doc = "0x3c - stream x FIFO control register" ]
    pub s1fcr: S1fcr,
    # [ doc = "0x40 - stream x configuration register" ]
    pub s2cr: S2cr,
    # [ doc = "0x44 - stream x number of data register" ]
    pub s2ndtr: S2ndtr,
    # [ doc = "0x48 - stream x peripheral address register" ]
    pub s2par: S2par,
    # [ doc = "0x4c - stream x memory 0 address register" ]
    pub s2m0ar: S2m0ar,
    # [ doc = "0x50 - stream x memory 1 address register" ]
    pub s2m1ar: S2m1ar,
    # [ doc = "0x54 - stream x FIFO control register" ]
    pub s2fcr: S2fcr,
    # [ doc = "0x58 - stream x configuration register" ]
    pub s3cr: S3cr,
    # [ doc = "0x5c - stream x number of data register" ]
    pub s3ndtr: S3ndtr,
    # [ doc = "0x60 - stream x peripheral address register" ]
    pub s3par: S3par,
    # [ doc = "0x64 - stream x memory 0 address register" ]
    pub s3m0ar: S3m0ar,
    # [ doc = "0x68 - stream x memory 1 address register" ]
    pub s3m1ar: S3m1ar,
    # [ doc = "0x6c - stream x FIFO control register" ]
    pub s3fcr: S3fcr,
    # [ doc = "0x70 - stream x configuration register" ]
    pub s4cr: S4cr,
    # [ doc = "0x74 - stream x number of data register" ]
    pub s4ndtr: S4ndtr,
    # [ doc = "0x78 - stream x peripheral address register" ]
    pub s4par: S4par,
    # [ doc = "0x7c - stream x memory 0 address register" ]
    pub s4m0ar: S4m0ar,
    # [ doc = "0x80 - stream x memory 1 address register" ]
    pub s4m1ar: S4m1ar,
    # [ doc = "0x84 - stream x FIFO control register" ]
    pub s4fcr: S4fcr,
    # [ doc = "0x88 - stream x configuration register" ]
    pub s5cr: S5cr,
    # [ doc = "0x8c - stream x number of data register" ]
    pub s5ndtr: S5ndtr,
    # [ doc = "0x90 - stream x peripheral address register" ]
    pub s5par: S5par,
    # [ doc = "0x94 - stream x memory 0 address register" ]
    pub s5m0ar: S5m0ar,
    # [ doc = "0x98 - stream x memory 1 address register" ]
    pub s5m1ar: S5m1ar,
    # [ doc = "0x9c - stream x FIFO control register" ]
    pub s5fcr: S5fcr,
    # [ doc = "0xa0 - stream x configuration register" ]
    pub s6cr: S6cr,
    # [ doc = "0xa4 - stream x number of data register" ]
    pub s6ndtr: S6ndtr,
    # [ doc = "0xa8 - stream x peripheral address register" ]
    pub s6par: S6par,
    # [ doc = "0xac - stream x memory 0 address register" ]
    pub s6m0ar: S6m0ar,
    # [ doc = "0xb0 - stream x memory 1 address register" ]
    pub s6m1ar: S6m1ar,
    # [ doc = "0xb4 - stream x FIFO control register" ]
    pub s6fcr: S6fcr,
    # [ doc = "0xb8 - stream x configuration register" ]
    pub s7cr: S7cr,
    # [ doc = "0xbc - stream x number of data register" ]
    pub s7ndtr: S7ndtr,
    # [ doc = "0xc0 - stream x peripheral address register" ]
    pub s7par: S7par,
    # [ doc = "0xc4 - stream x memory 0 address register" ]
    pub s7m0ar: S7m0ar,
    # [ doc = "0xc8 - stream x memory 1 address register" ]
    pub s7m1ar: S7m1ar,
    # [ doc = "0xcc - stream x FIFO control register" ]
    pub s7fcr: S7fcr,
}

# [ repr ( C ) ]
pub struct Lisr {
    register: ::volatile_register::RO<u32>,
}

impl Lisr {
    pub fn read(&self) -> LisrR {
        LisrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LisrR {
    bits: u32,
}

impl LisrR {
    # [ doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif3(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif3(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif3(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif3(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif3(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif2(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif2(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif2(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif2(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif2(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif1(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif1(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif1(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif1(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif1(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif0(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif0(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif0(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif0(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LisrW {
    bits: u32,
}

impl LisrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        LisrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)" ]
    pub fn tcif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)" ]
    pub fn htif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)" ]
    pub fn teif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)" ]
    pub fn dmeif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)" ]
    pub fn feif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Hisr {
    register: ::volatile_register::RO<u32>,
}

impl Hisr {
    pub fn read(&self) -> HisrR {
        HisrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HisrR {
    bits: u32,
}

impl HisrR {
    # [ doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif7(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif7(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif7(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif7(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif7(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif6(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif6(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif6(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif6(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif6(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif5(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif5(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif5(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif5(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif5(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif4(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif4(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif4(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HisrW {
    bits: u32,
}

impl HisrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HisrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Stream x transfer complete interrupt flag (x=7..4)" ]
    pub fn tcif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Stream x half transfer interrupt flag (x=7..4)" ]
    pub fn htif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Stream x transfer error interrupt flag (x=7..4)" ]
    pub fn teif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Stream x direct mode error interrupt flag (x=7..4)" ]
    pub fn dmeif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream x FIFO error interrupt flag (x=7..4)" ]
    pub fn feif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Lifcr {
    register: ::volatile_register::RW<u32>,
}

impl Lifcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&LifcrR, &'w mut LifcrW) -> &'w mut LifcrW
    {
        let bits = self.register.read();
        let r = LifcrR { bits: bits };
        let mut w = LifcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> LifcrR {
        LifcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut LifcrW) -> &mut LifcrW
    {
        let mut w = LifcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LifcrR {
    bits: u32,
}

impl LifcrR {
    # [ doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif3(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif3(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif3(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif3(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif3(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif2(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif2(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif2(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif2(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif2(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif1(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif1(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif1(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif1(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif1(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif0(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif0(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif0(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif0(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LifcrW {
    bits: u32,
}

impl LifcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        LifcrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)" ]
    pub fn ctcif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)" ]
    pub fn chtif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)" ]
    pub fn cteif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)" ]
    pub fn cdmeif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)" ]
    pub fn cfeif0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Hifcr {
    register: ::volatile_register::RW<u32>,
}

impl Hifcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&HifcrR, &'w mut HifcrW) -> &'w mut HifcrW
    {
        let bits = self.register.read();
        let r = HifcrR { bits: bits };
        let mut w = HifcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> HifcrR {
        HifcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut HifcrW) -> &mut HifcrW
    {
        let mut w = HifcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HifcrR {
    bits: u32,
}

impl HifcrR {
    # [ doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif7(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif7(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif7(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif7(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif7(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif6(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif6(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif6(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif6(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif6(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif5(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif5(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif5(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif5(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif5(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif4(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif4(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif4(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HifcrW {
    bits: u32,
}

impl HifcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HifcrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)" ]
    pub fn ctcif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)" ]
    pub fn chtif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)" ]
    pub fn cteif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)" ]
    pub fn cdmeif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)" ]
    pub fn cfeif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S0cr {
    register: ::volatile_register::RW<u32>,
}

impl S0cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0crR, &'w mut S0crW) -> &'w mut S0crW
    {
        let bits = self.register.read();
        let r = S0crR { bits: bits };
        let mut w = S0crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0crR {
        S0crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0crW) -> &mut S0crW
    {
        let mut w = S0crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0crR {
    bits: u32,
}

impl S0crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0crW {
    bits: u32,
}

impl S0crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S0ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S0ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0ndtrR, &'w mut S0ndtrW) -> &'w mut S0ndtrW
    {
        let bits = self.register.read();
        let r = S0ndtrR { bits: bits };
        let mut w = S0ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0ndtrR {
        S0ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0ndtrW) -> &mut S0ndtrW
    {
        let mut w = S0ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0ndtrR {
    bits: u32,
}

impl S0ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0ndtrW {
    bits: u32,
}

impl S0ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S0par {
    register: ::volatile_register::RW<u32>,
}

impl S0par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0parR, &'w mut S0parW) -> &'w mut S0parW
    {
        let bits = self.register.read();
        let r = S0parR { bits: bits };
        let mut w = S0parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0parR {
        S0parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0parW) -> &mut S0parW
    {
        let mut w = S0parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0parR {
    bits: u32,
}

impl S0parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0parW {
    bits: u32,
}

impl S0parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S0m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S0m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0m0arR, &'w mut S0m0arW) -> &'w mut S0m0arW
    {
        let bits = self.register.read();
        let r = S0m0arR { bits: bits };
        let mut w = S0m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0m0arR {
        S0m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0m0arW) -> &mut S0m0arW
    {
        let mut w = S0m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0m0arR {
    bits: u32,
}

impl S0m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0m0arW {
    bits: u32,
}

impl S0m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S0m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S0m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0m1arR, &'w mut S0m1arW) -> &'w mut S0m1arW
    {
        let bits = self.register.read();
        let r = S0m1arR { bits: bits };
        let mut w = S0m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0m1arR {
        S0m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0m1arW) -> &mut S0m1arW
    {
        let mut w = S0m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0m1arR {
    bits: u32,
}

impl S0m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0m1arW {
    bits: u32,
}

impl S0m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S0fcr {
    register: ::volatile_register::RW<u32>,
}

impl S0fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S0fcrR, &'w mut S0fcrW) -> &'w mut S0fcrW
    {
        let bits = self.register.read();
        let r = S0fcrR { bits: bits };
        let mut w = S0fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S0fcrR {
        S0fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S0fcrW) -> &mut S0fcrW
    {
        let mut w = S0fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0fcrR {
    bits: u32,
}

impl S0fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S0fcrW {
    bits: u32,
}

impl S0fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S0fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S1cr {
    register: ::volatile_register::RW<u32>,
}

impl S1cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1crR, &'w mut S1crW) -> &'w mut S1crW
    {
        let bits = self.register.read();
        let r = S1crR { bits: bits };
        let mut w = S1crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1crR {
        S1crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1crW) -> &mut S1crW
    {
        let mut w = S1crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1crR {
    bits: u32,
}

impl S1crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1crW {
    bits: u32,
}

impl S1crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S1ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S1ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1ndtrR, &'w mut S1ndtrW) -> &'w mut S1ndtrW
    {
        let bits = self.register.read();
        let r = S1ndtrR { bits: bits };
        let mut w = S1ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1ndtrR {
        S1ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1ndtrW) -> &mut S1ndtrW
    {
        let mut w = S1ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1ndtrR {
    bits: u32,
}

impl S1ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1ndtrW {
    bits: u32,
}

impl S1ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S1par {
    register: ::volatile_register::RW<u32>,
}

impl S1par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1parR, &'w mut S1parW) -> &'w mut S1parW
    {
        let bits = self.register.read();
        let r = S1parR { bits: bits };
        let mut w = S1parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1parR {
        S1parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1parW) -> &mut S1parW
    {
        let mut w = S1parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1parR {
    bits: u32,
}

impl S1parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1parW {
    bits: u32,
}

impl S1parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S1m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S1m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1m0arR, &'w mut S1m0arW) -> &'w mut S1m0arW
    {
        let bits = self.register.read();
        let r = S1m0arR { bits: bits };
        let mut w = S1m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1m0arR {
        S1m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1m0arW) -> &mut S1m0arW
    {
        let mut w = S1m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1m0arR {
    bits: u32,
}

impl S1m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1m0arW {
    bits: u32,
}

impl S1m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S1m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S1m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1m1arR, &'w mut S1m1arW) -> &'w mut S1m1arW
    {
        let bits = self.register.read();
        let r = S1m1arR { bits: bits };
        let mut w = S1m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1m1arR {
        S1m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1m1arW) -> &mut S1m1arW
    {
        let mut w = S1m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1m1arR {
    bits: u32,
}

impl S1m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1m1arW {
    bits: u32,
}

impl S1m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S1fcr {
    register: ::volatile_register::RW<u32>,
}

impl S1fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S1fcrR, &'w mut S1fcrW) -> &'w mut S1fcrW
    {
        let bits = self.register.read();
        let r = S1fcrR { bits: bits };
        let mut w = S1fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S1fcrR {
        S1fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S1fcrW) -> &mut S1fcrW
    {
        let mut w = S1fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1fcrR {
    bits: u32,
}

impl S1fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S1fcrW {
    bits: u32,
}

impl S1fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S1fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S2cr {
    register: ::volatile_register::RW<u32>,
}

impl S2cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2crR, &'w mut S2crW) -> &'w mut S2crW
    {
        let bits = self.register.read();
        let r = S2crR { bits: bits };
        let mut w = S2crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2crR {
        S2crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2crW) -> &mut S2crW
    {
        let mut w = S2crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2crR {
    bits: u32,
}

impl S2crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2crW {
    bits: u32,
}

impl S2crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S2ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S2ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2ndtrR, &'w mut S2ndtrW) -> &'w mut S2ndtrW
    {
        let bits = self.register.read();
        let r = S2ndtrR { bits: bits };
        let mut w = S2ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2ndtrR {
        S2ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2ndtrW) -> &mut S2ndtrW
    {
        let mut w = S2ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2ndtrR {
    bits: u32,
}

impl S2ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2ndtrW {
    bits: u32,
}

impl S2ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S2par {
    register: ::volatile_register::RW<u32>,
}

impl S2par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2parR, &'w mut S2parW) -> &'w mut S2parW
    {
        let bits = self.register.read();
        let r = S2parR { bits: bits };
        let mut w = S2parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2parR {
        S2parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2parW) -> &mut S2parW
    {
        let mut w = S2parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2parR {
    bits: u32,
}

impl S2parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2parW {
    bits: u32,
}

impl S2parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S2m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S2m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2m0arR, &'w mut S2m0arW) -> &'w mut S2m0arW
    {
        let bits = self.register.read();
        let r = S2m0arR { bits: bits };
        let mut w = S2m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2m0arR {
        S2m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2m0arW) -> &mut S2m0arW
    {
        let mut w = S2m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2m0arR {
    bits: u32,
}

impl S2m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2m0arW {
    bits: u32,
}

impl S2m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S2m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S2m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2m1arR, &'w mut S2m1arW) -> &'w mut S2m1arW
    {
        let bits = self.register.read();
        let r = S2m1arR { bits: bits };
        let mut w = S2m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2m1arR {
        S2m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2m1arW) -> &mut S2m1arW
    {
        let mut w = S2m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2m1arR {
    bits: u32,
}

impl S2m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2m1arW {
    bits: u32,
}

impl S2m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S2fcr {
    register: ::volatile_register::RW<u32>,
}

impl S2fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S2fcrR, &'w mut S2fcrW) -> &'w mut S2fcrW
    {
        let bits = self.register.read();
        let r = S2fcrR { bits: bits };
        let mut w = S2fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S2fcrR {
        S2fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S2fcrW) -> &mut S2fcrW
    {
        let mut w = S2fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2fcrR {
    bits: u32,
}

impl S2fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S2fcrW {
    bits: u32,
}

impl S2fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S2fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3cr {
    register: ::volatile_register::RW<u32>,
}

impl S3cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3crR, &'w mut S3crW) -> &'w mut S3crW
    {
        let bits = self.register.read();
        let r = S3crR { bits: bits };
        let mut w = S3crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3crR {
        S3crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3crW) -> &mut S3crW
    {
        let mut w = S3crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3crR {
    bits: u32,
}

impl S3crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3crW {
    bits: u32,
}

impl S3crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S3ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S3ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3ndtrR, &'w mut S3ndtrW) -> &'w mut S3ndtrW
    {
        let bits = self.register.read();
        let r = S3ndtrR { bits: bits };
        let mut w = S3ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3ndtrR {
        S3ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3ndtrW) -> &mut S3ndtrW
    {
        let mut w = S3ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3ndtrR {
    bits: u32,
}

impl S3ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3ndtrW {
    bits: u32,
}

impl S3ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3par {
    register: ::volatile_register::RW<u32>,
}

impl S3par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3parR, &'w mut S3parW) -> &'w mut S3parW
    {
        let bits = self.register.read();
        let r = S3parR { bits: bits };
        let mut w = S3parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3parR {
        S3parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3parW) -> &mut S3parW
    {
        let mut w = S3parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3parR {
    bits: u32,
}

impl S3parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3parW {
    bits: u32,
}

impl S3parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S3m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3m0arR, &'w mut S3m0arW) -> &'w mut S3m0arW
    {
        let bits = self.register.read();
        let r = S3m0arR { bits: bits };
        let mut w = S3m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3m0arR {
        S3m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3m0arW) -> &mut S3m0arW
    {
        let mut w = S3m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3m0arR {
    bits: u32,
}

impl S3m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3m0arW {
    bits: u32,
}

impl S3m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S3m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3m1arR, &'w mut S3m1arW) -> &'w mut S3m1arW
    {
        let bits = self.register.read();
        let r = S3m1arR { bits: bits };
        let mut w = S3m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3m1arR {
        S3m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3m1arW) -> &mut S3m1arW
    {
        let mut w = S3m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3m1arR {
    bits: u32,
}

impl S3m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3m1arW {
    bits: u32,
}

impl S3m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S3fcr {
    register: ::volatile_register::RW<u32>,
}

impl S3fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S3fcrR, &'w mut S3fcrW) -> &'w mut S3fcrW
    {
        let bits = self.register.read();
        let r = S3fcrR { bits: bits };
        let mut w = S3fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S3fcrR {
        S3fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S3fcrW) -> &mut S3fcrW
    {
        let mut w = S3fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3fcrR {
    bits: u32,
}

impl S3fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S3fcrW {
    bits: u32,
}

impl S3fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S3fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S4cr {
    register: ::volatile_register::RW<u32>,
}

impl S4cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4crR, &'w mut S4crW) -> &'w mut S4crW
    {
        let bits = self.register.read();
        let r = S4crR { bits: bits };
        let mut w = S4crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4crR {
        S4crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4crW) -> &mut S4crW
    {
        let mut w = S4crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4crR {
    bits: u32,
}

impl S4crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4crW {
    bits: u32,
}

impl S4crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S4ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S4ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4ndtrR, &'w mut S4ndtrW) -> &'w mut S4ndtrW
    {
        let bits = self.register.read();
        let r = S4ndtrR { bits: bits };
        let mut w = S4ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4ndtrR {
        S4ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4ndtrW) -> &mut S4ndtrW
    {
        let mut w = S4ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4ndtrR {
    bits: u32,
}

impl S4ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4ndtrW {
    bits: u32,
}

impl S4ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S4par {
    register: ::volatile_register::RW<u32>,
}

impl S4par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4parR, &'w mut S4parW) -> &'w mut S4parW
    {
        let bits = self.register.read();
        let r = S4parR { bits: bits };
        let mut w = S4parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4parR {
        S4parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4parW) -> &mut S4parW
    {
        let mut w = S4parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4parR {
    bits: u32,
}

impl S4parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4parW {
    bits: u32,
}

impl S4parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S4m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S4m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4m0arR, &'w mut S4m0arW) -> &'w mut S4m0arW
    {
        let bits = self.register.read();
        let r = S4m0arR { bits: bits };
        let mut w = S4m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4m0arR {
        S4m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4m0arW) -> &mut S4m0arW
    {
        let mut w = S4m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4m0arR {
    bits: u32,
}

impl S4m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4m0arW {
    bits: u32,
}

impl S4m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S4m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S4m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4m1arR, &'w mut S4m1arW) -> &'w mut S4m1arW
    {
        let bits = self.register.read();
        let r = S4m1arR { bits: bits };
        let mut w = S4m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4m1arR {
        S4m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4m1arW) -> &mut S4m1arW
    {
        let mut w = S4m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4m1arR {
    bits: u32,
}

impl S4m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4m1arW {
    bits: u32,
}

impl S4m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S4fcr {
    register: ::volatile_register::RW<u32>,
}

impl S4fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S4fcrR, &'w mut S4fcrW) -> &'w mut S4fcrW
    {
        let bits = self.register.read();
        let r = S4fcrR { bits: bits };
        let mut w = S4fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S4fcrR {
        S4fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S4fcrW) -> &mut S4fcrW
    {
        let mut w = S4fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4fcrR {
    bits: u32,
}

impl S4fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S4fcrW {
    bits: u32,
}

impl S4fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S4fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S5cr {
    register: ::volatile_register::RW<u32>,
}

impl S5cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5crR, &'w mut S5crW) -> &'w mut S5crW
    {
        let bits = self.register.read();
        let r = S5crR { bits: bits };
        let mut w = S5crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5crR {
        S5crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5crW) -> &mut S5crW
    {
        let mut w = S5crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5crR {
    bits: u32,
}

impl S5crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5crW {
    bits: u32,
}

impl S5crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S5ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S5ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5ndtrR, &'w mut S5ndtrW) -> &'w mut S5ndtrW
    {
        let bits = self.register.read();
        let r = S5ndtrR { bits: bits };
        let mut w = S5ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5ndtrR {
        S5ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5ndtrW) -> &mut S5ndtrW
    {
        let mut w = S5ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5ndtrR {
    bits: u32,
}

impl S5ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5ndtrW {
    bits: u32,
}

impl S5ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S5par {
    register: ::volatile_register::RW<u32>,
}

impl S5par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5parR, &'w mut S5parW) -> &'w mut S5parW
    {
        let bits = self.register.read();
        let r = S5parR { bits: bits };
        let mut w = S5parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5parR {
        S5parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5parW) -> &mut S5parW
    {
        let mut w = S5parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5parR {
    bits: u32,
}

impl S5parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5parW {
    bits: u32,
}

impl S5parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S5m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S5m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5m0arR, &'w mut S5m0arW) -> &'w mut S5m0arW
    {
        let bits = self.register.read();
        let r = S5m0arR { bits: bits };
        let mut w = S5m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5m0arR {
        S5m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5m0arW) -> &mut S5m0arW
    {
        let mut w = S5m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5m0arR {
    bits: u32,
}

impl S5m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5m0arW {
    bits: u32,
}

impl S5m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S5m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S5m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5m1arR, &'w mut S5m1arW) -> &'w mut S5m1arW
    {
        let bits = self.register.read();
        let r = S5m1arR { bits: bits };
        let mut w = S5m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5m1arR {
        S5m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5m1arW) -> &mut S5m1arW
    {
        let mut w = S5m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5m1arR {
    bits: u32,
}

impl S5m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5m1arW {
    bits: u32,
}

impl S5m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S5fcr {
    register: ::volatile_register::RW<u32>,
}

impl S5fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S5fcrR, &'w mut S5fcrW) -> &'w mut S5fcrW
    {
        let bits = self.register.read();
        let r = S5fcrR { bits: bits };
        let mut w = S5fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S5fcrR {
        S5fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S5fcrW) -> &mut S5fcrW
    {
        let mut w = S5fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5fcrR {
    bits: u32,
}

impl S5fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S5fcrW {
    bits: u32,
}

impl S5fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S5fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S6cr {
    register: ::volatile_register::RW<u32>,
}

impl S6cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6crR, &'w mut S6crW) -> &'w mut S6crW
    {
        let bits = self.register.read();
        let r = S6crR { bits: bits };
        let mut w = S6crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6crR {
        S6crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6crW) -> &mut S6crW
    {
        let mut w = S6crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6crR {
    bits: u32,
}

impl S6crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6crW {
    bits: u32,
}

impl S6crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S6ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S6ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6ndtrR, &'w mut S6ndtrW) -> &'w mut S6ndtrW
    {
        let bits = self.register.read();
        let r = S6ndtrR { bits: bits };
        let mut w = S6ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6ndtrR {
        S6ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6ndtrW) -> &mut S6ndtrW
    {
        let mut w = S6ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6ndtrR {
    bits: u32,
}

impl S6ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6ndtrW {
    bits: u32,
}

impl S6ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S6par {
    register: ::volatile_register::RW<u32>,
}

impl S6par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6parR, &'w mut S6parW) -> &'w mut S6parW
    {
        let bits = self.register.read();
        let r = S6parR { bits: bits };
        let mut w = S6parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6parR {
        S6parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6parW) -> &mut S6parW
    {
        let mut w = S6parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6parR {
    bits: u32,
}

impl S6parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6parW {
    bits: u32,
}

impl S6parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S6m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S6m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6m0arR, &'w mut S6m0arW) -> &'w mut S6m0arW
    {
        let bits = self.register.read();
        let r = S6m0arR { bits: bits };
        let mut w = S6m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6m0arR {
        S6m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6m0arW) -> &mut S6m0arW
    {
        let mut w = S6m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6m0arR {
    bits: u32,
}

impl S6m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6m0arW {
    bits: u32,
}

impl S6m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S6m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S6m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6m1arR, &'w mut S6m1arW) -> &'w mut S6m1arW
    {
        let bits = self.register.read();
        let r = S6m1arR { bits: bits };
        let mut w = S6m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6m1arR {
        S6m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6m1arW) -> &mut S6m1arW
    {
        let mut w = S6m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6m1arR {
    bits: u32,
}

impl S6m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6m1arW {
    bits: u32,
}

impl S6m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S6fcr {
    register: ::volatile_register::RW<u32>,
}

impl S6fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S6fcrR, &'w mut S6fcrW) -> &'w mut S6fcrW
    {
        let bits = self.register.read();
        let r = S6fcrR { bits: bits };
        let mut w = S6fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S6fcrR {
        S6fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S6fcrW) -> &mut S6fcrW
    {
        let mut w = S6fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6fcrR {
    bits: u32,
}

impl S6fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S6fcrW {
    bits: u32,
}

impl S6fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S6fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S7cr {
    register: ::volatile_register::RW<u32>,
}

impl S7cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7crR, &'w mut S7crW) -> &'w mut S7crW
    {
        let bits = self.register.read();
        let r = S7crR { bits: bits };
        let mut w = S7crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7crR {
        S7crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7crW) -> &mut S7crW
    {
        let mut w = S7crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7crR {
    bits: u32,
}

impl S7crR {
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 23u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7crW {
    bits: u32,
}

impl S7crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7crW { bits: 0u32 }
    }
    # [ doc = "Bits 25:27 - Channel selection" ]
    pub fn chsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 23:24 - Memory burst transfer configuration" ]
    pub fn mburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 23u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:22 - Peripheral burst transfer configuration" ]
    pub fn pburst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - ACK" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Current target (only in double buffer mode)" ]
    pub fn ct(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Double buffer mode" ]
    pub fn dbm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Peripheral increment offset size" ]
    pub fn pincos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Memory data size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Peripheral data size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Data transfer direction" ]
    pub fn dir(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Peripheral flow controller" ]
    pub fn pfctrl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Direct mode error interrupt enable" ]
    pub fn dmeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Stream enable / flag stream ready when read low" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct S7ndtr {
    register: ::volatile_register::RW<u32>,
}

impl S7ndtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7ndtrR, &'w mut S7ndtrW) -> &'w mut S7ndtrW
    {
        let bits = self.register.read();
        let r = S7ndtrR { bits: bits };
        let mut w = S7ndtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7ndtrR {
        S7ndtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7ndtrW) -> &mut S7ndtrW
    {
        let mut w = S7ndtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7ndtrR {
    bits: u32,
}

impl S7ndtrR {
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7ndtrW {
    bits: u32,
}

impl S7ndtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7ndtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data items to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S7par {
    register: ::volatile_register::RW<u32>,
}

impl S7par {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7parR, &'w mut S7parW) -> &'w mut S7parW
    {
        let bits = self.register.read();
        let r = S7parR { bits: bits };
        let mut w = S7parW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7parR {
        S7parR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7parW) -> &mut S7parW
    {
        let mut w = S7parW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7parR {
    bits: u32,
}

impl S7parR {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7parW {
    bits: u32,
}

impl S7parW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7parW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S7m0ar {
    register: ::volatile_register::RW<u32>,
}

impl S7m0ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7m0arR, &'w mut S7m0arW) -> &'w mut S7m0arW
    {
        let bits = self.register.read();
        let r = S7m0arR { bits: bits };
        let mut w = S7m0arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7m0arR {
        S7m0arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7m0arW) -> &mut S7m0arW
    {
        let mut w = S7m0arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7m0arR {
    bits: u32,
}

impl S7m0arR {
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7m0arW {
    bits: u32,
}

impl S7m0arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7m0arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 0 address" ]
    pub fn m0a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S7m1ar {
    register: ::volatile_register::RW<u32>,
}

impl S7m1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7m1arR, &'w mut S7m1arW) -> &'w mut S7m1arW
    {
        let bits = self.register.read();
        let r = S7m1arR { bits: bits };
        let mut w = S7m1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7m1arR {
        S7m1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7m1arW) -> &mut S7m1arW
    {
        let mut w = S7m1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7m1arR {
    bits: u32,
}

impl S7m1arR {
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7m1arW {
    bits: u32,
}

impl S7m1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7m1arW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)" ]
    pub fn m1a(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct S7fcr {
    register: ::volatile_register::RW<u32>,
}

impl S7fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&S7fcrR, &'w mut S7fcrW) -> &'w mut S7fcrW
    {
        let bits = self.register.read();
        let r = S7fcrR { bits: bits };
        let mut w = S7fcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> S7fcrR {
        S7fcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut S7fcrW) -> &mut S7fcrW
    {
        let mut w = S7fcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7fcrR {
    bits: u32,
}

impl S7fcrR {
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - FIFO status" ]
    pub fn fs(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct S7fcrW {
    bits: u32,
}

impl S7fcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        S7fcrW { bits: 33u32 }
    }
    # [ doc = "Bit 7 - FIFO error interrupt enable" ]
    pub fn feie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Direct mode disable" ]
    pub fn dmdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - FIFO threshold selection" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
