NRG - general purpose energy management system

# Abstract

I was fed up with...

- ...the terrible control software of my Daikin Altherma 3 H HT
  ECH<sub>2</sub>O (ETSXB16P50D) heat pump.
- ...the barely useable internet gateway [Gateway RoCon G1
  EHS157056](https://www.daikin.de/de_de/produkte/product.html/EHS157056.html)
  for that heat pump.
- ...the terrible EHS157068 mixer module.
- ...the lack of interoperability between inverters, charging stations and the
  heat pump.

This project aims to provide...

- ...a local gateway for the Daikin Altherma 3 H HT connected via the CAN BUS
  interface.
- ...a new and open-source mixer control (this obsoletes my mixer unit) using
  multiple relais and various temperature sensors.
- ...a simple yet complete power management solution for delivering the excess
  PV electricity to my heat pump and charging stations.

All that bundled in a set of applications written in the Rust programming
language, running on a Raspberry PI with a CAN HAT and some Relais.

All communication is done via MQTT and the services are written following
the [Unix philosophy](https://en.wikipedia.org/wiki/Unix_philosophy).

# Installation instructions

There are none yet. This project is still very much work in progress.

# My Hardware

- Raspberry PI 4 8 GB
- [Waveshare Isolated RS485 CAN HAT (B)](https://www.amazon.de/dp/B0BKKZG6C4)
- [Waveshare Modbus RTU 16-Ch Relay
  Module](https://www.amazon.de/dp/B0C9M7YL93)
- [Keba KeConnect P30
  x-series](https://www.keba.com/de/emobility/products/x-series/x-serie)
- [SolarEdge
SE25K Inverter](https://www.solaredge.com/en/products/residential/pv-inverters/solaredge-home-wave-inverters)
