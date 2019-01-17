# LANMON - A simple LAN monitor
LANMON was built for one single purpose. It's function is to monitor a list of nodes on your LAN
and send an email alert if a node drops off the network. Its a very simple solution to a bigger SNMP application.

It requires zero install on the nodes being monitored. They only need to be able to respond to an ICMP ping.

## Requirements
LANMON needs to be running on a system that can send mail and implements a standard Linux mail command. The simplest way to test this is to execute the command:
'
echo "TEST" | mail -s "Test email" <emailaddress>
'
where the **emailaddress** is, of course, your mail address. If this results in your receiving an email then you are good to go.

## Configuration
LAN runs off a standard JSON configuration file. The default file is **./lanmon.json** and is of the form:
```
{
    "alert_mins" : 1,
    "alert" : "<<EMAIL-ADDRESS>",
    "emailonrestore" : true,
    "nodes" : [
        "192.168.1.10" ,
        "192.168.1.74",
        "192.168.1.76",
        "10.56.75.171",
        "node1",
        "node2",
        "node3",
        "2001:0db8:0000:0042:0000:8a2e:0370:7334"
    ]
}
```
where:
* **alert_mins** is the number of minutes a node needs to be offline before an alert will be generated. This is an integer positive number.
* **alert** is the emain address to receive alerts
* **emailonrestore** is a boolean where valid entries are either *true* or *false*. If set to *true* an email will be generated when the mode is restored to the network.
* **nodes** is a list of character strings representing the nodes to be monitored. This will accept nodes in three formats: IPV4; IPV6 or node name. The node will be checked on program startup to determine if an IP Address can be determined. It does not have to be active on startup, but it must be resolvable.

There should be no practical limit to the number of nodes available.

The nodes DO NOT have to be on the same subnet. Any node name can be monitored provided it responds to a ping.

## Running LANMON
LanMon is started as a background task. It does have a few command line flags.

```
Lan Monitor
Very simple LAN Monitor

USAGE:
    lanmon [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Sets the level of verbosity

OPTIONS:
    -c, --config <FILE>    The JSON formatted config file [default: ./lanmon.json]
```

By default, only limited information messages are printed to the standard output file, stdout. This can be increased to include debug messages (-v) or trace messages (-vv).

**NOTE** ICMP Ping is treated as a privileged command. The lanmon executable needs to be either installed with setcap pribilege, or used as a sudo command.


## Installation
LANMON is made available as source code only. This is because it executes with SUDO privileges, a requirement of the ICMP Ping protocol on Linux. No right minded sysadmin who wishes to monitor their network would accept an unsigned executable running in privilege mode. Hence LANMON needs to be compiled before being used.

LANMON is written in the [rust](https://www.rust-lang.org/) programming language, so Rust and it's package manager, Cargo, must be installed prior to use. To check if Rust is installed, issue the commands seen below. you see output such as this, your good to go:
```
$ rustc -V
rustc 1.32.0-nightly (6acbb5b65 2018-11-25)
$ cargo -V
cargo 1.32.0-nightly (b3d0b2e54 2018-11-15)
$
```

To build LanMon issue the commands
'''
github clone https://github.com/AdrianChallinorOsiris/lanmon.git
cd lanmon
cargo install --path .
sudo setcap cap_net_raw+ep ~/.cargo/bin/lanmon
'''

## LICENSE
LanMon is licenses un the GNU General Public License  v3.0 [GPL 3](https://www.gnu.org/licenses/gpl-3.0.en.html)

## WARRANTY AND LIABILITY

The Software is made available to you free of charge.

You may be entitled to warranties, conditions and terms that may not be excluded or limited by Osiris under
law.

EXCEPT FOR THOSE NON-EXCLUDABLE WARRANTIES, CONDITIONS AND TERMS, THE SOFTWARE, OSIRIS CONSULTAMTS LTD ARE MADE
AVAILABLE “AS IS.” EXCEPT FOR THOSE NON-EXCLUDABLE WARRANTIES, CONDITIONS AND TERMS, OSIRIS CONSULTAMTS LTD
MAKE NO WARRANTIES, CONDITIONS,
REPRESENTATIONS, GUARANTEES OR TERMS (EXPRESS OR IMPLIED, WHETHER BY STATUTE,
COMMON LAW, CUSTOM, USAGE OR OTHERWISE) AS TO ANY MATTER INCLUDING
PERFORMANCE, RESULTS, SECURITY, NONINFRINGEMENT, MERCHANTABILITY,
INTEGRATION, QUIET ENJOYMENT, SATISFACTORY QUALITY, AND FITNESS FOR ANY
PARTICULAR PURPOSE. THIS DISCLAIMER OF WARRANTY MAY NOT BE VALID IN SOME
STATES. YOU MAY HAVE WARRANTY RIGHTS UNDER LAW WHICH MAY NOT BE WAIVED OR
DISCLAIMED. OSIRIS DOES NOT SEEK TO LIMIT YOUR WARRANTY RIGHTS TO ANY EXTENT
NOT PERMITTED BY LAW.

These exclusions and limitations will apply to the maximum extent permitted by applicable law, even if any
remedy fails its essential purpose. Osiris provides no support services for the Software.

YOU ACKNOWLEDGE AND AGREE THAT: OSIRIS HAS NO EXPRESS OR IMPLIED OBLIGATION
TO CONTINUE TO MAKE THE SOFTWARE OR ANY FEATURE THEREOF AVAILABLE NOR
INTRODUCE ANY PRODUCTS OR SERVICES COMPATIBLE WITH THE SOFTWARE.

## LEGAL JURISTICTION

This contract will be governed by and construed in accordance of English law and shall be subject to the non-exclusive jurisdiction of the High Court in London.



