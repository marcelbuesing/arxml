#[macro_use]
extern crate yaserde_derive;

use log::{debug, info, trace};
use std::io::prelude::*;
use xml::reader::XmlEvent;
use xml_schema_derive::XmlSchema;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(
    source = "AUTOSAR_00046.xsd",
    target_prefix = "AR",
    log_level = "warn",
    store_generated_code = "./out.rs"
)]
struct ArXmlSchema;
