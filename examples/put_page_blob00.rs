extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate md5;
extern crate tokio_core;

use azure_sdk_for_rust::core::ba512_range::BA512Range;
use azure_sdk_for_rust::prelude::*;
use futures::future::*;
use std::collections::HashMap;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init();
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args().nth(2).expect("please specify blob name as command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let data: [u8; 700] = [51; 700];

    let mut metadata = HashMap::new();

    metadata.insert("pollo", "arrosto");
    metadata.insert("milk", "shake");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(&data[0..512]);

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    let future = client
        .put_page_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_content_length(1024 * 3)?
        .with_content_type("text/plain")
        .with_metadata(&metadata)
        .finalize();

    trace!("before put_page_blob");

    core.run(future.map(|res| println!("{:?}", res)))?;

    let future = client
        .update_page()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_ba512_range(&BA512Range::new(0, 511)?)
        .with_content_md5(&digest[..])
        .with_body(&data[..])
        .finalize();

    trace!("before update_page");

    core.run(future.map(|res| println!("{:?}", res)))?;

    Ok(())
}
