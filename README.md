# RTIC 2 USB-Ethernet Gadgets

This is somewhere between a library and a software stack for creating USB gadgets that
look like ethernet devices.

The gadget exposes TCP services that send and receive COBS encoded, zero delimted
protocol buffer messages.

Protocol buffers, because they are low overhead for a micro controller, and have a good
Rust implementation in [micropb]()

## Networking

The gadget uses DHCP to get it's IP address. The idea was that the host would bridge
it onto the network, so it would be available network wide, without any routing.
That can't work for Wifi!

In my case, the host is a Raspberry Pi OS host. Once you know how, it's easiest to set up network routing and DHCP relay agents to deal with this.

### Predictable network interface names

The code uses the device ID to generate a _locally administed_ MAC. If you enable [Predictable Network Interface Names](https://systemd.io/PREDICTABLE_INTERFACE_NAMES/)
each gadget will have a predictable network interface name: `enx` followed by the mac address.

_Locally administered_ MACs have the second least significant bit in the first
byte set. The least significant bit is always zero (one is reserved for multicast addresses).
That means the second character of the MAC will be `[26ae]` which we will use later to
identify the created interfaces.

### Dynamic routing

Somehow, there must be routes to the subnet for the gadget interface. Static routes are
a possibility, but that often involves a lot of repetition.

babeld is fairly simple:

```shell
sudo apt install isc-dhcp-relay
sudo systemctl enable babeld
```

Then `/etc/babeld.conf` for the host:

```conf
# For more information about this configuration file, refer to
# babeld(8)
redistribute ip 192.168.0.0/21 allow
interface enxce567d849b70
interface wlan0
```

On other hosts (like your main router) you don't need the `redistribute`: just the list of interfaces.

Where `192.168.0.0/21` is the network assigned to gadgets, `enxce567d849b70` is
gadgets network interface, and wlan0 is the wifi network we couldn't bridge to.

```shell
sudo /etc/init.d/babeld start
```

### DHCP Relay

Now we just have to arrange for the gadgets DHCP request to be responded to.

```shell
sudo apt install isc-dhcp-relay
```

Don't list interfaces, instead use the options to specify upstream interfaces with `-iu` - the interface or interfaces that have a DHCP server, and downstream interfices with `-id` - the
interfaces you want to provide DHCP relay to - I.e. the gadget interface.

isc-dhcp-relay will not start if you list any relays that don't exist, so the list
of downstream interfaces needs to be dynamically generated.

So in `/etc/default/isc-dhcp-relay` you need something like this:

```shell
# What servers should the DHCP relay forward requests to?
SERVERS="dhcp"

# We we want to designate interfaces as upstream or downstream so leave this empty
INTERFACES=""

# Additional options that are passed to the DHCP relay daemon
OPTIONS="-iu wlan0"
# Find all the ethernet devices with locally administered addresses
LOCALLY_ADMINISTERED_DEVICES=$(ls /sys/class/net/ | grep -e "enx[0-9a-f][26ae][0-9a-f]*")
for iface in $LOCALLY_ADMINISTERED_DEVICES; do
    OPTIONS="$OPTIONS -id $iface"
done
```

isc-dhcp-relay needs to be restarted when an interface goes up or down, so create 
`/etc/NetworkManager/dispatcher.d/99-isc-dhcp-relay-restart`:

```shell
#!/bin/bash
case $2 in
    up|down)
        systemctl restart isc-dhcp-relay
        ;;
    *)
        ;;
esac
```

### DHCP Server

The DHCP server needs to be configured for each sub-net.
