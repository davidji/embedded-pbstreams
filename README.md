# RTIC 2 USB-Ethernet Gadgets

This is somewhere between a library and a software stack for creating USB gadgets that
look like ethernet devices.

The gadget exposes TCP services that send and receive COBS encoded, zero delimted
protocol buffer messages.

Protocol buffers, because they are low overhead for a micro controller, and have a good
Rust implementation in [micropb]()

## Networking

The device uses DHCP to get it's IP address. The idea was that the host would bridge
it onto the network, so it would be available network wide, without any routing.
That can't work for Wifi!

In my case, the host is a Raspberry Pi OS host. Once you know how, it's easiest to set up network routing and DHCP relay agents to deal with this.

### Predictable network interface names

The code uses the device ID to generate a MAC. If you enable [Predictable Network Interface Names](https://systemd.io/PREDICTABLE_INTERFACE_NAMES/)
each gadget will have a predictable network interface name: `enx` followed by the mac address.

### Dynamic routing

Somehow, there must be routes to the subnet for the black pill interface. Static routes are
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
interfaces you want to provide DHCP relay to - I.e. the black pill interface.

So in `/etc/default/isc-dhcp-relay` you need something like this:

```shell
# What servers should the DHCP relay forward requests to?
SERVERS="dhcp"

# On what interfaces should the DHCP relay (dhrelay) serve DHCP requests?
INTERFACES=""

# Additional options that are passed to the DHCP relay daemon?
OPTIONS="-iu enxb827eb63585a"
# These devices mostly won't be connected, and the relay demon won't start if
# you give it an interface that doesn't exist
for iface in enxce567d849b70 -id enx4ee2bfdb202c; do
    if ifconfig $iface > /dev/null 2> /dev/null; then
        OPTIONS="$OPTIONS -id $iface"
    fi
done
```

### DHCP Server

The DHCP server needs to be configured for each sub-net.

