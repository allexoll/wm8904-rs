#![no_std]


use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

pub const DEVICE_ADDRESS: u8 = 0x1A;

#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Register {
    SW_RESET_ID = 0x00,
    BIAS_CONTROL_0 = 0x04,
    VMID_CONTROL_0 = 0x05,
    MISC_BIAS_CONTROL_0 = 0x06,
    MISC_BIAS_CONTROL_1 = 0x07,
    ANALOG_ADC_0 = 0x0A,
    POWER_MGMT_0 = 0x0C,
    POWER_MGMT_2 = 0x0E,
    POWER_MGMT_3 = 0x0F,
    POWER_MGMT_6 = 0x12,

    CLK_RATES_0 = 0x14,
    CLK_RATES_1 = 0x15,
    CLK_RATES_2 = 0x16,

    AUDIO_IF_0 = 0x18,
    AUDIO_IF_1 = 0x19,
    AUDIO_IF_2 = 0x1A,
    AUDIO_IF_3 = 0x1B,

    DAC_VOL_LEFT = 0x1E,
    DAC_VOL_RIGHT = 0x1F,
    DAC_DIGITAL_0 = 0x20,
    DAC_DIGITAL_1 = 0x21,

    ADC_DIGITAL_VOL_LEFT = 0x24,
    ADC_DIGITAL_VOL_RIGHT = 0x25,
    ADC_DIGITAL_0 = 0x26,
    DIGITAL_MIC_0 = 0x27,
    DRC_0 = 0x28,
    DRC_1 = 0x29,
    DRC_2 = 0x2A,

    ANALOG_LEFT_IN_0 = 0x2C,
    ANALOG_RIGHT_IN_0 = 0x2D,
    ANALOG_LEFT_IN_1 = 0x2E,
    ANALOG_RIGHT_IN_1 = 0x2F,

    ANALOG_OUT1_LEFT = 0x39,
    ANALOG_OUT1_RIGHT = 0x3A,
    ANALOG_OUT2_LEFT = 0x3B,
    ANALOG_OUT2_RIGHT = 0x3C,
    ANALOG_OUT12_ZC = 0x3D,

    DC_SERVO_0 = 0x43,
    DC_SERVO_1 = 0x44,
    DC_SERVO_2 = 0x45,

    DC_SERVO_4 = 0x47,
    DC_SERVO_5 = 0x48,
    DC_SERVO_6 = 0x49,
    DC_SERVO_7 = 0x4A,
    DC_SERVO_8 = 0x4B,
    DC_SERVO_9 = 0x4C,

    DC_SERVO_READBACK_0 = 0x4D,

    ANALOG_HP_0 = 0x5A,
    ANALOG_LINEOUT_0 = 0x5E,

    CHRG_PUMP_0 = 0x62,
    CLASS_W = 0x68,

    WRT_SEQUENCER_0 = 0x6C,
    WRT_SEQUENCER_1 = 0x6D,
    WRT_SEQUENCER_2 = 0x6E,
    WRT_SEQUENCER_3 = 0x6F,
    WRT_SEQUENCER_4 = 0x70,

    FLL_CONTROL_1 = 0x74,
    FLL_CONTROL_2 = 0x75,
    FLL_CONTROL_3 = 0x76,
    FLL_CONTROL_4 = 0x77,
    FLL_CONTROL_5 = 0x78,

    GPIO_CONTROL_1 = 0x79,
    GPIO_CONTROL_2 = 0x7A,
    GPIO_CONTROL_3 = 0x7B,
    GPIO_CONTROL_4 = 0x7C,

    DIGITAL_PULLS = 0x7E,
    INT_STATUS = 0x7F,
    INT_STATUS_MASK = 0x80,
    INT_POLARITY = 0x81,
    INT_DEBOUNCE = 0x82,

    EQ1 = 0x86,
    EQ2 = 0x87,
    EQ3 = 0x88,
    EQ4 = 0x89,
    EQ5 = 0x8A,
    EQ6 = 0x8B,
    EQ7 = 0x8C,
    EQ8 = 0x8D,
    EQ9 = 0x8E,
    EQ10 = 0x8F,
    EQ11 = 0x90,
    EQ12 = 0x91,
    EQ13 = 0x92,
    EQ14 = 0x93,
    EQ15 = 0x94,
    EQ16 = 0x95,
    EQ17 = 0x96,
    EQ18 = 0x97,
    EQ19 = 0x98,
    EQ20 = 0x99,
    EQ21 = 0x9A,
    EQ22 = 0x9B,
    EQ23 = 0x9C,
    EQ24 = 0x9D,

    ADC_TEST_0 = 0xC6,

    FLL_NCO_TEST_0 = 0xF7,
    FLL_NCO_TEST_1 = 0xF8,
}

impl From<Register> for u8 {
    fn from(r: Register) -> Self {
        r as u8
    }
}

#[derive(Debug)]
pub struct WM8904<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> WM8904<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
    /// Create a new instance of the WM8904 driver
    pub fn new(i2c: I2C, address: u8) -> Self {
        WM8904 { i2c, address }
    }

    /// Write a WM8904 register
    pub fn write_register(&mut self, register: Register, data: u16) -> Result<(), E> {
        #[cfg(feature = "debug")]
        defmt::info!(
            "Writing register {}:{:02x} with data {:04x}",
            register,
            register as u8,
            data
        );
        let data = [(data >> 8) as u8, data as u8];
        self.i2c
            .write(self.address, &[register.into(), data[0], data[1]])
    }

    /// Read a WM8904 register
    pub fn read_register(&mut self, register: Register) -> Result<u16, E> {
        let mut data = [0u8; 2];
        self.i2c
            .write_read(self.address, &[register.into()], &mut data)?;
        let reg_data = ((data[0] as u16) << 8) | data[1] as u16;
        #[cfg(feature = "debug")]
        defmt::info!(
            "Reading register {}:{:02x} with data {:04x}",
            register,
            register as u8,
            reg_data
        );
        Ok(reg_data)
    }

    /// Modify register with mask and value
    pub fn modify_register(&mut self, register: Register, mask: u16, value: u16) -> Result<(), E> {
        let mut data = self.read_register(register)?;
        data &= !mask;
        data |= value;
        self.write_register(register, data)
    }

    /// Wait on write sequencer
    /// reads and waits until the write sequencer is not busy
    /// (bit 0 of register 0x70 is 0)
    pub fn wait_on_write_sequencer(&mut self) -> Result<(), E> {
        loop {
            if self.read_register(Register::WRT_SEQUENCER_4)? == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Return the I2C device
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}
