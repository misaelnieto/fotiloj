use clap::Command;
use searchlight::{
	discovery::{DiscoveryBuilder, DiscoveryEvent},
	dns::{op::DnsResponse, rr::RData},
	net::IpVersion,
};


/// Taken from Searchlight's documentation
fn get_chromecast_name(dns_packet: &DnsResponse) -> String {
	dns_packet
		.additionals()
		.iter()
		.find_map(|record| {
			if let Some(RData::SRV(_)) = record.data() {
				let name = record.name().to_utf8();
				let name = name.strip_suffix('.').unwrap_or(&name);
				let name = name.strip_suffix("_googlecast._tcp.local").unwrap_or(&name);
				let name = name.strip_suffix('.').unwrap_or(&name);
				Some(name.to_string())
			} else {
				None
			}
		})
		.unwrap_or_else(|| "Unknown".into())
}

fn discover_devices() {
    DiscoveryBuilder::new()
    .service("_googlecast._tcp.local.")
    .unwrap()
    .build(IpVersion::Both)
    .unwrap()
    .run(|event| match event {
        DiscoveryEvent::ResponderFound(responder) => {
            println!(
                "Found Chromecast {} at {}",
                get_chromecast_name(&responder.last_response),
                responder.addr.ip()
            );
        }
        DiscoveryEvent::ResponderLost(responder) => {
            println!(
                "Chromecast {} at {} has gone away",
                get_chromecast_name(&responder.last_response),
                responder.addr.ip()
            );
        }
        DiscoveryEvent::ResponseUpdate { .. } => {}
    })
    .unwrap();    
}



fn main() {
    let cli = Command::new("fotiloj")
        .about("A toolkit to stream your IP cameras to Chromecast devices")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // Discover
        .subcommand(
            Command::new("discover")
                .about("Discover available devices on the network.")
        )
        .get_matches();

    match cli.subcommand() {
        Some(("discover", subm)) => {
            discover_devices();
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}