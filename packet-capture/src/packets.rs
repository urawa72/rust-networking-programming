use pnet::packet::ipv4::Ipv4;

pub trait GettableEndPoints {
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &Vec<u8>;
}

impl GettableEndPoints for Ipv4 {
    fn get_source(&self) -> String {
        self.source.to_string()
    }

    fn get_destination(&self) -> String {
        self.destination.to_string()
    }

    fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }
}
