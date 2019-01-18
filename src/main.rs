// Copyright ⓒ 2018 Adrian Challinor <adrian.challinor@osiris.co.uk>
//
// LICENSED under GNU General Public License v3.0
//
// This program comes with ABSOLUTELY NO WARRANTY. See LICENSE for
// full details.
//

//! # LANMON - A simple LAN monitor
//! LANMON was built for one single purpose. It's function is to monitor a list of nodes on your LAN
//! and send an email alert if a node drops off the network. Its a very simple solution to a bigger SNMP application.
//!
//! It requires zero install on the nodes being monitored. They only need to be able to respond to an ICMP ping.
//!
//! ## Requirements
//! LANMON needs to be running on a system that can send mail and implements a standard Linux mail command. The simplest way to test this is to execute the command:
//! '
//! echo "TEST" | mail -s "Test email" <emailaddress>
//! '
//! where the **emailaddress** is, of course, your mail address. If this results in your receiving an email then you are good to go.
//!
//! ## Configuration
//! LAN runs off a standard JSON or TOML configuration file. The default file is **./lanmon.json** and is of the form:
//! ```
//! {
//!     "alert_mins" : 1,
//!     "alert" : "<<EMAIL-ADDRESS>",
//!     "emailonrestore" : true,
//!     "nodes" : [
//!         "192.168.1.10" ,
//!         "192.168.1.74",
//!         "192.168.1.76",
//!         "10.56.75.171",
//!         "node1",
//!         "node2",
//!         "node3",
//!         "2001:0db8:0000:0042:0000:8a2e:0370:7334"
//!     ]
//! }
//! ```
//!
//! the equivalent in TOML is
//! '''
//! alert_mins = 1
//! alert = "myemail@domain.com"
//! emailonrestore = true
//! nodes = [
//!         "192.168.1.10" ,
//!         "192.168.1.74",
//!         "192.168.1.76",
//!         "10.56.75.171",
//!         "o1",
//!         "o2",
//!         "o3"
//!     ]
//! '''
//!
//! where:
//! * **alert_mins** is the number of minutes a node needs to be offline before an alert will be generated. This is an integer positive number.
//! * **alert** is the emain address to receive alerts
//! * **emailonrestore** is a boolean where valid entries are either *true* or *false*. If set to *true* an email will be generated when the mode is restored to the network.
//! * **nodes** is a list of character strings representing the nodes to be monitored. This will accept nodes in three formats: IPV4; IPV6 or node name. The node will be checked on program startup to determine if an IP Address can be determined. It does not have to be active on startup, but it must be resolvable.
//!
//! There should be no practical limit to the number of nodes available.
//!
//! The nodes DO NOT have to be on the same subnet. Any node name can be monitored provided it responds to a ping.
//!
//! ## Running LANMON
//! LanMon is started as a background task. It does have a few command line flags.
//!
//! ```
//! Lan Monitor
//! Very simple LAN Monitor
//!
//! USAGE:
//!     lanmon [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!     -v, --verbose    Sets the level of verbosity
//!
//! OPTIONS:
//!     -c, --config <FILE>    The JSON formatted config file [default: ./lanmon.json]
//! ```
//!
//! By default, only limited information messages are printed to the standard output file, stdout. This can be increased to include debug messages (-v) or trace messages (-vv).
//!
//! **NOTE** ICMP Ping is treated as a privileged command. The lanmon executable needs to be either installed with setcap pribilege, or used as a sudo command.
//!
//!
//! ## Installation
//! LANMON is made available as source code only. This is because it executes with SUDO privileges, a requirement of the ICMP Ping protocol on Linux. No right minded sysadmin who wishes to monitor their network would accept an unsigned executable running in privilege mode. Hence LANMON needs to be compiled before being used.
//!
//! LANMON is written in the [rust](https://www.rust-lang.org/) programming language, so Rust and it's package manager, Cargo, must be installed prior to use. To check if Rust is installed, issue the commands seen below. you see output such as this, your good to go:
//! ```
//! $ rustc -V
//! rustc 1.32.0-nightly (6acbb5b65 2018-11-25)
//! $ cargo -V
//! cargo 1.32.0-nightly (b3d0b2e54 2018-11-15)
//! $
//! ```
//!
//! To build LanMon issue the commands
//! '''
//! github clone https://github.com/AdrianChallinorOsiris/lanmon.git
//! cd lanmon
//! cargo install --path .
//! //! sudo setcap cap_net_raw+ep ~/.cargo/bin/lanmon
//! '''
//!
//! ## LICENSE
//! LanMon is licenses un the GNU General Public License  v3.0 [GPL 3](https://www.gnu.org/licenses/gpl-3.0.en.html)
//!
//! ## WARRANTY AND LIABILITY
//!
//! The Software is made available to you free of charge.
//!
//! You may be entitled to warranties, conditions and terms that may not be excluded or limited by Osiris under
//! law.
//!
//! EXCEPT FOR THOSE NON-EXCLUDABLE WARRANTIES, CONDITIONS AND TERMS, THE SOFTWARE, OSIRIS CONSULTAMTS LTD ARE MADE
//! AVAILABLE “AS IS.” EXCEPT FOR THOSE NON-EXCLUDABLE WARRANTIES, CONDITIONS AND TERMS, OSIRIS CONSULTAMTS LTD
//! MAKE NO WARRANTIES, CONDITIONS,
//! REPRESENTATIONS, GUARANTEES OR TERMS (EXPRESS OR IMPLIED, WHETHER BY STATUTE,
//! COMMON LAW, CUSTOM, USAGE OR OTHERWISE) AS TO ANY MATTER INCLUDING
//! PERFORMANCE, RESULTS, SECURITY, NONINFRINGEMENT, MERCHANTABILITY,
//! INTEGRATION, QUIET ENJOYMENT, SATISFACTORY QUALITY, AND FITNESS FOR ANY
//! PARTICULAR PURPOSE. THIS DISCLAIMER OF WARRANTY MAY NOT BE VALID IN SOME
//! STATES. YOU MAY HAVE WARRANTY RIGHTS UNDER LAW WHICH MAY NOT BE WAIVED OR
//! DISCLAIMED. OSIRIS DOES NOT SEEK TO LIMIT YOUR WARRANTY RIGHTS TO ANY EXTENT
//! NOT PERMITTED BY LAW.
//!
//! These exclusions and limitations will apply to the maximum extent permitted by applicable law, even if any
//! remedy fails its essential purpose. Osiris provides no support services for the Software.
//!
//! YOU ACKNOWLEDGE AND AGREE THAT: OSIRIS HAS NO EXPRESS OR IMPLIED OBLIGATION
//! TO CONTINUE TO MAKE THE SOFTWARE OR ANY FEATURE THEREOF AVAILABLE NOR
//! INTRODUCE ANY PRODUCTS OR SERVICES COMPATIBLE WITH THE SOFTWARE.
//!
//! ## LEGAL JURISTICTION
//!
//! This contract will be governed by and construed in accordance of English law and shall be subject to the non-exclusive jurisdiction of the High Court in London.
//!




#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate fern;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg};
use fern::colors::{Color, ColoredLevelConfig};
use fastping_rs::Pinger;
use fastping_rs::PingResult::{Idle, Receive};
use chrono::prelude::*;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::net::{ToSocketAddrs};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::{time};
use std::process::Command;

/// The `Config` struture is the JSON file read at
/// program initiation.  See README for details of the format
///
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    alert_mins: u64,
    alert: String,
    emailonrestore: bool,
    nodes: Vec<String>,
}

/// # Node
/// The `Node` structure is the internal status of a Node
/// being monitored. It is access by indexing the IP Address in to
/// Hashmap (nodes).
///
/// One extremely good question is why there is an Instant and
/// a Local DateTime. Simple answer is that, under time pressure, it was
/// easier to use the Instant to determin a duration when the node was unseen,
/// and a Local Date Time to output a formated string in the locale of the user.
///
/// TO-DO: review this decision.
///
#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    pub name:       String,
    pub lastseen:   Instant,
    pub dt:         DateTime<Local>,
    pub alerted:    bool,
}



impl Node {
    /// Issue an Alert because the node has not been seen in the
    /// alloted period. Note that we only issue one alert, so this
    /// function remembers that it has already been called for this node.
    ///
    fn alert(&mut self, alertto: &str) {
        warn!("ALERT - {} ", self.name);
        let dt = self.dt.format("%d-%b-%Y %H:%M").to_string();
        self.alerted = true;
        let subject = format!("LANMON - node {} - not seen since {}", self.name, dt);
        let body = format!("Message from LANMON\n\n Node {} not seen since {}", self.name, dt);
        let alertcmd = format!("echo \"{}\" | mail -s \"{}\" {} ", body, subject, alertto);
        let output = Command::new("/bin/bash")
            .arg("-c")
            .arg(alertcmd)
            .output()
            .expect("Failed to send email");
        let _ret = output.stdout;
    }

    /// Note that a node is idle, that is, nor responding to PINGs. This
    /// may issue an alert if the duration is expired and the node has not
    /// had an alert issued before.
    fn idle(&mut self, alertsecs: Duration,  alertto: &str) {
        if self.lastseen.elapsed() > alertsecs {
            if !self.alerted {
                self.alert(alertto);
            }
        }
    }

    /// The node is seen - i.e. responding to pings.
    ///
    /// If it was off line previously, and if so requested by the config file,
    /// we issue an email alert to note its reappearance.
    ///
    fn seen(&mut self,  alertto: &str, emailonrestore:bool ) {
        if self.alerted {
            info!("Node {} active again", self.name);
            self.alerted = false;
            if emailonrestore {
                let subject = format!("LANMON - node {} - recativated", self.name);
                let body = format!("Message from LANMON\n\n Node {} - recativated", self.name);
                let alertcmd = format!("echo \"{}\" | mail -s \"{}\" {} ", body, subject, alertto);
                let output = Command::new("/bin/bash")
                    .arg("-c")
                    .arg(alertcmd)
                    .output()
                    .expect("Failed to send email");
                let _ret = output.stdout;
            }
        }
        self.lastseen = Instant::now();
        self.dt = Local::now();
    }
}

/// Decode the IP address of a node. We don;t care if it's IPV4 or OPV6, but
/// we must have an IP Address. Names are converted to IP address. IP addresses read
/// from the config will be validated.
fn to_ip(n: &str)  -> Option<String> {
    let node = format!("{}:80", n);
    match node.to_socket_addrs() {
        Ok(addresses) => {
            for addr in addresses {
                return Some(addr.ip().to_string());
            }
         },
        Err(e) => {
            error!("Parsing node {} - error: {}", n, e);
            return None;
        }
    }
    error!("In 'to_ip'. Don't know how I got here, parsing {}", n);
    return None;
}


/// Main program.
///
/// In principal, this does the following:
/// 1) Parse the command line arguments
/// 2) Read the config file
/// 3) For each node in the config file:
///     3.1) Convert the name in to IPV4 format
///     3.2) Start a thread to monitor the node
///
/// In the thread, we:
/// Ping the node
///   - If we find it, update the last seen time
///   - If we dont, then
///         - Check if we are outside our alert limit duration
///         - If we are, send an email
///         - And be careful that we don't bombard the receiever with emails !
///

fn main() {
    // Parse the command line
    let cmd_arguments = App::new("Lan Monitor")
        .about("Very simple LAN Monitor")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .help("The JSON formatted config file")
                .default_value("./lanmon.json"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Sets the level of verbosity")
                .multiple(true),
        )
        .get_matches();

    // Set up logging
    let verbosity: u64 = cmd_arguments.occurrences_of("verbose");
    setup_logging(verbosity);

    // Initializing
    info!("Starting up - Version {}", crate_version!());
    debug!("Verbosity level: {}", verbosity);

    // Read the config file
    let cfile = cmd_arguments.value_of("config").unwrap();
    debug!("Reading config from: {}", cfile);
    let config = read_config(cfile).unwrap();

    // Debug sugar
    debug!("Alert when down for: {} minutes", config.alert_mins );
    debug!("Email to: {} ", config.alert);
    debug!("Monitor nodes: {:?}", config.nodes);

    // Get native versions of the values
    let alertsecs = time::Duration::from_secs(config.alert_mins * 60);

    debug!("Creating pinger");
    let (pinger, results) = match Pinger::new(None, None) {
        Ok((pinger, results)) => (pinger, results),
        Err(e) => panic!("Error creating pinger: {}", e)
    };

    // Getting nodes
    debug!("Setting up notes");
    let mut nodes = HashMap::new();
    for name in config.nodes {
        match to_ip(&name) {
            Some(ipaddr) => {
                trace!("Found node: {} - {}", name, ipaddr);
                let node = Node{
                    name:   name,
                    lastseen:   Instant::now(),
                    alerted: false,
                    dt: Local::now(),
                };
                nodes.insert(ipaddr.clone(), node);
                pinger.add_ipaddr(&ipaddr);
             },
            None => {
                error!("Node {} - no IP address", name);
                return;
            }
        };
    }

    // Main loop
    debug!("Starting pinger");
    pinger.run_pinger();
    loop {
            match results.recv() {
                Ok(result) => {
                    match result {
                        Idle{addr} => {
                            match nodes.get_mut(&addr.to_string()) {
                                Some(n) => {
                                    trace!("Idle address {}", addr);
                                    n.idle(alertsecs, &config.alert);
                                },
                                None => { } // Do nothing - not for us.
                            };
                        },
                        Receive{addr, rtt} => {
                           match nodes.get_mut(&addr.to_string()) {
                                Some(n) => {
                                    trace!("Reset last seen: {} - RTT: {:?}", n.name, rtt);
                                    n.seen(&config.alert, config.emailonrestore );
                                },
                                None => { } // Do nothing - not for us.
                            };
                       }
                    }
                },
                Err(_) => panic!("Worker threads disconnected before the solution was found!"),
            }
        }
}


/// Read the JSON config file
///
fn read_config(filename: &str) -> Result<Config, Box<Error>> {
    let path = Path::new(filename);
    let ext = path.extension();
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");


    // Read the JSON contents of the file as an instance of `Config`.
    let c: Config =
    match ext {
        Some(ftype) => {
            match ftype.to_str().unwrap() {
                "toml" => toml::from_str(&contents)?,
                "json" => serde_json::from_str(&contents)?,
                _ => {
                    let e1: Box<Error + Send + Sync> = From::from("File extension not supported");
                    return Err(e1);
                }
            }
        },
        None => {
            let e1: Box<Error + Send + Sync> = From::from("No file extension on config");
            return Err(e1);
        }
    };

    // Return the `User`.
    Ok(c)
}

/// Set up logging with nice colour scheme
///
fn setup_logging(verbosity: u64) -> u64 {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta)
        .info(Color::Green)
        .trace(Color::BrightBlue);
    let level = match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _2_or_more => log::LevelFilter::Trace,
    };

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .level(level)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}\t [{}] {}",
                // This will color the log level only, not the whole line. Just a touch.
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();
    verbosity
}
