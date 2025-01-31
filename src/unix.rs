use crate::RootStoreBuilder;

use std::io::{Error, ErrorKind};
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

fn load_file(builder: &mut impl RootStoreBuilder, path: &Path) -> Result<(), Error> {
    let f = File::open(&path)?;
    let mut f = BufReader::new(f);
    if builder.load_pem_file(&mut f).is_err() {
        Err(Error::new(ErrorKind::InvalidData,
                       format!("Could not load PEM file {:?}", path)))
    } else {
        Ok(())
    }
}

pub fn build_native_certs<B: RootStoreBuilder>(builder: &mut B) -> Result<(), Error> {
    let likely_locations = openssl_probe::probe();
    let mut first_error = None;

    if let Some(file) = likely_locations.cert_file {
        if let Err(err) = load_file(builder, &file) {
            first_error = first_error.or(Some(err));
        }
    }

    if let Some(err) = first_error {
        Err(err)
    } else {
        Ok(())
    }
}
