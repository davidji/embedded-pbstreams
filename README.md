# RTIC 2 USB-Ethernet Gadgets

This is somewhere between a library and a software stack for creating USB gadgets that
look like ethernet devices.

The gadget exposes TCP services that send and receive COBS encoded, zero delimted
protocol buffer messages.

Protocol buffers, because they are low overhead for a micro controller, have a good
Rust implementation in [micropb]()
