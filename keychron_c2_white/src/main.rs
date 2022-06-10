#![no_std]
#![no_main]

use panic_abort as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use vcell::VolatileCell;

#[repr(C)]
struct SysCtrl0 {
    _pad0: [u32; 8],
    swd_ctrl: VolatileCell<u32>,
}
impl SysCtrl0 {
    fn disable_swd(&mut self) {
        self.swd_ctrl.set(1);
    }
}

#[repr(C)]
struct GPIOPorts {
    a: GPIOPort,
    b: GPIOPort,
    c: GPIOPort,
    d: GPIOPort,
}

#[repr(C, align(8192))]
struct GPIOPort {
    data: VolatileCell<u32>,
    mode: VolatileCell<u32>,
    config: VolatileCell<u32>,
    intr_sense: VolatileCell<u32>,
    intr_both_sense: VolatileCell<u32>,
    intr_event: VolatileCell<u32>,
    intr_enable: VolatileCell<u32>,
    intr_status: VolatileCell<u32>,
    intr_clr: VolatileCell<u32>,
    bset: VolatileCell<u32>,
    bclr: VolatileCell<u32>,
}

impl GPIOPort {
    fn read_pins(&self) -> PinSet {
        PinSet(self.data.get())
    }
    fn set_pins(&self, pins: PinSet) {
        self.bset.set(pins.0)
    }
    fn clear_pins(&self, pins: PinSet) {
        self.bclr.set(pins.0)
    }
    fn set_mode(&self, mode: GPIOMode, pins: PinSet) {
        let old = self.mode.get();
        let new_value = match mode {
            GPIOMode::In => old & !pins.0,
            GPIOMode::Out => old | pins.0,
        };
        self.mode.set(new_value);
    }
    fn set_config(&self, config: GPIOConfig, pins: PinSet) {
        let old = self.config.get();
        let config_mask = match config {
            GPIOConfig::PullUp => 0,
            GPIOConfig::Inactive => 0xaaaa_aaaa,
            GPIOConfig::InactiveSchmitt => 0xffff_ffff,
        };
        let mut pin_mask = 0;
        for pin in pins.pins() {
            pin_mask |= 0b11 << (pin * 2);
        }
        let new_value = (old & !pin_mask) | (pin_mask & config_mask);
        self.config.set(new_value);
    }
}

enum GPIOMode {
    In,
    Out,
}
enum GPIOConfig {
    PullUp,
    Inactive,
    InactiveSchmitt,
}

#[derive(Clone, Copy)]
struct PinSet(u32);
impl PinSet {
    const fn one_pin(pin_idx: u8) -> Self {
        PinSet(1 << pin_idx)
    }

    const fn from_indexes<const N: usize>(pin_idxs: [u8; N]) -> Self {
        let mut pins = 0u32;
        let mut pin_idx = 0;
        while pin_idx < pin_idxs.len() {
            pins |= 1 << pin_idxs[pin_idx];
            pin_idx += 1;
        }
        PinSet(pins)
    }

    fn len(self) -> u8 {
        self.0.count_ones() as u8
    }

    fn contains_pin(self, idx: u8) -> bool {
        self.0 & (1 << idx) != 0
    }

    fn intersect(self, other: Self) -> Self {
        PinSet(self.0 & other.0)
    }

    fn pins(self) -> PinsIter {
        PinsIter {
            set: self,
            next_pin: 0,
        }
    }
}
struct PinsIter {
    set: PinSet,
    next_pin: u8,
}
impl Iterator for PinsIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        for pin in self.next_pin..31 {
            if self.set.contains_pin(pin) {
                self.next_pin = pin + 1;
                return Some(pin);
            }
        }
        None
    }
}

fn delay() {
    for _ in 0..400 {
        asm::nop();
    }
}

#[entry]
fn _start() -> ! {
    unsafe {
        let sys_ctrl0 = &mut *(0x4006_0000 as *mut SysCtrl0);
        let gpio = &mut *(0x4004_4000 as *mut GPIOPorts);

        let cols = [
            (
                &gpio.a,
                PinSet::from_indexes([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            ),
            (&gpio.d, PinSet::from_indexes([0, 1, 2, 3, 8])),
        ];
        let rows = [(&gpio.c, PinSet::from_indexes([3, 4, 5, 6, 7, 8]))];

        sys_ctrl0.disable_swd();
        let win_mac_keys_led_pins = PinSet::from_indexes([4, 5]);
        let lock_keys_led_pins = PinSet::from_indexes([6, 7]);
        gpio.b.set_mode(GPIOMode::Out, win_mac_keys_led_pins);
        gpio.b.clear_pins(win_mac_keys_led_pins);
        gpio.d.set_mode(GPIOMode::Out, lock_keys_led_pins);
        gpio.d.clear_pins(lock_keys_led_pins);
        for &(gpio_port, pins) in cols.iter() {
            gpio_port.set_mode(GPIOMode::Out, pins);
            gpio_port.set_pins(pins);
        }
        for &(gpio_port, pins) in rows.iter() {
            gpio_port.set_mode(GPIOMode::In, pins);
            gpio_port.set_config(GPIOConfig::PullUp, pins);
        }
        delay();

        loop {
            let mut num_keys_pressed = 0;
            for (col_port, col_pin) in cols
                .into_iter()
                .flat_map(|(port, pins)| pins.pins().map(move |pin| (port, pin)))
            {
                col_port.clear_pins(PinSet::one_pin(col_pin));
                delay();
                for (row_port, row_pins) in rows {
                    let active_pins = row_port.read_pins().intersect(row_pins);
                    num_keys_pressed += row_pins.len() - active_pins.len();
                }
                col_port.set_pins(PinSet::one_pin(col_pin));
                delay();
            }

            for (idx, (led_port, led_pin)) in
                [(&gpio.b, 4), (&gpio.b, 5), (&gpio.d, 6), (&gpio.d, 7)]
                    .into_iter()
                    .enumerate()
            {
                if num_keys_pressed & (1 << idx) == 0 {
                    led_port.clear_pins(PinSet::one_pin(led_pin));
                } else {
                    led_port.set_pins(PinSet::one_pin(led_pin));
                }
            }
        }
    }
}
