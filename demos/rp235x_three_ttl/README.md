# Demo of three TTL counters for RP235x Pico boards.

This repository contains example code to hook up an iC-MD three-channel TTL counter
to a Raspberry Pi Pico RP235x board. 

## Connections

The following tables shows the connections on the iC-MD (left) and the RP235x Pico board (right):

| iC-MD TTL Counter | RP235x Pico Board |
|-------------------|-------------------|
| VDD               | 3V3(OUT) (pin 36) |
| GND               | GND               |
| COPI              | GP19 (SPI0 TX)    |
| CIPO              | GP16 (SPI0 RX)    |
| SCK               | GP18 (SPI0 SCK)   |
| CS                | GP17 (SPI0 CSn)   |
