#[macro_use]
extern crate yaserde_derive;

use log::{info, debug, trace};
use xml::reader::XmlEvent;
use std::io::prelude::*;
use xml_schema_derive::XmlSchema;
use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, XmlSchema)]
#[xml_schema(source = "AUTOSAR_00044.xsd", target_prefix = "AR", log_level = "warn", store_generated_code = "./out.rs")]
struct ArXmlSchema;