# Examples

All of the examples in this folder use a
[Pico de Gallo] to communicate with the chip.

If you do not have a Pico de Gallo,
use can provide any SPI interface that implements
[embedded-hal] and/or [embedded-hal-async] traits.

Examples that have a filename starting with `ll`
interact with the low-level driver,
which is written using [device-driver].

## Connections

For most examples, it is useful to connect an encoder to your iC-MD.
This allows you to move it around and see what happens.
Depending on the type of encoder you are using,
you might want to set up the iC-MD in a specific counter configuration.
The following Pico de Gallo connections are required.

| Pico de Gallo v1.1 | iC-MD       |
| ------------------ | ----------- |
| SPI_MISO (p5)      | MISO (p13)  |
| SPI_MOSI (p6)      | MOSI (p14)  |
| SPI_SCK (p7)       | SCK (p15)   |
| GPIO0 (p11)        | NCS (p16)   |
| +3V3 (p23)         | VDD (p17)   |
| GND (p24)          | GND (p10)   |

*Note*: We are using `GPIO0` on the Pico de Gallo
for the chip select.

If you do not use an iC-MD dev board,
check the datasheet of the iC-MD on how to tie other logic pins.

## Example overview

The following shows and overview and a brief description of all the examples.
Documentation within the example gives you more information.
All examples are listed in order of potential interest/complexity.

### High-level driver examples

- `counter_readout`: Reads out a counter frequently and prints result to `STDOUT`.

[device-driver]: https://device-driver.com/
[embedded-hal]: https://docs.rs/embedded-hal/latest/embedded_hal/
[embedded-hal-async]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/
[Pico de Gallo]: https://balbi.sh/pico-de-gallo
