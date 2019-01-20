use log::debug;

use ssdp::FieldMap;
use ssdp::header::{HeaderMut, HeaderRef, Location, Man, MX, ST};
use ssdp::message::{SearchRequest, Multicast};

use url::Url;

const DOXIE_URN : &'static str = "schemas-getdoxie-com:device:Scanner:1";

pub fn discover_doxie() -> Option<String> {
    let mut request = SearchRequest::new();
    request.set(Man);
    request.set(MX(1));
    request.set(ST::Target(FieldMap::URN(DOXIE_URN.to_string())));

    // Iterate Over Streaming Responses
    // FIXME: Would rather short-circuit this loop when we find the target URL, but I get panic
    //        from the library if I subsequently do network stuff. Instead, iterate through everything.
    let mut base_url = None;
    for (msg, src) in request.multicast().unwrap() {
        debug!("Received The Following Message From {}:\n{:?}\n", src, msg);
        if let ST::Target(FieldMap::URN(urn_string)) = msg.get::<ST>().expect("Expected ST header") {
            if urn_string == DOXIE_URN {
                debug!("Found the Doxie!");
                let location = msg.get::<Location>().expect("Expected Location header");
                let url = Url::parse(location).expect("Expected a valid URL in the location header");
                let origin = url.origin();
                base_url = Some(origin.ascii_serialization());
            }
        }
    }
    base_url
}
