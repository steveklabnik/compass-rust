
extern crate "rustc-serialize" as rustc_serialize;

use std::error::Error;
use rustc_serialize::Decodable;
use rustc_serialize::json::{self, Json};

fn decode<T: Decodable>(reader: &mut Reader) -> Result<T, Box<Error>> {
  let j = try!(Json::from_reader(reader).map_err(|e| Box::new(e) as Box<Error>));
  let mut decoder = json::Decoder::new(j);
  Decodable::decode(&mut decoder).map_err(|e| Box::new(e) as Box<Error>)
}


#[cfg(test)]
mod tests {

  #[test]
  fn test_dummy() {
    assert_eq!(1,1)
  }
}
