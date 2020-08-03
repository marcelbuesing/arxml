use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::path::PathBuf;

/// Name of file in archive
const XSD_FILE_NAME: &str = "AUTOSAR_00046.xsd";

/// Url of archive containing the XSD
const URL_ARXML_XSD: &str = "https://www.autosar.org/fileadmin/Releases_TEMP/Classic_Platform_4.4.0/MethodologyAndTemplates.zip";

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut output_path = PathBuf::new();
    output_path.push(manifest_dir);
    output_path.push(XSD_FILE_NAME);

    if output_path.exists() {
        println!("AUTOSAR file already existing");
        return;
    }

    let mut resp = reqwest::blocking::get(URL_ARXML_XSD).unwrap();

    println!("Finished downloading ARXML");

    let mut buf: Vec<u8> = vec![];
    resp.copy_to(&mut buf).unwrap();

    let reader = std::io::Cursor::new(buf);
    let mut zip = zip::ZipArchive::new(reader).unwrap();

    let mut xml_schema_zip = zip
        .by_name("MethodologyAndTemplates/AUTOSAR_MMOD_XMLSchema.zip")
        .unwrap();

    let mut inner_buf: Vec<u8> = vec![];
    xml_schema_zip.read_to_end(&mut inner_buf).unwrap();
    let inner_reader = std::io::Cursor::new(inner_buf);

    let mut inner_zip = zip::ZipArchive::new(inner_reader).unwrap();
    let mut xsd_file = inner_zip.by_name("AUTOSAR_00046.xsd").unwrap();

    let temp_output_path = output_path.with_extension("tmp");

    {
        let mut temp_outfile = fs::File::create(&temp_output_path).unwrap();
        io::copy(&mut xsd_file, &mut temp_outfile).unwrap();
    }

    remove_element_by_lines(&temp_output_path, &output_path);

    fs::remove_file(temp_output_path).unwrap();
}

fn remove_element_by_lines(input_path: &PathBuf, output_path: &PathBuf) {
    let f = fs::File::open(input_path).unwrap();
    let f = io::BufReader::new(f);

    let output_update = fs::File::create(output_path).unwrap();

    let mut writer = io::BufWriter::new(output_update);

    for (i, line) in f.lines().enumerate() {
        if i < 1245 || i > 1252 {
            writer.write(line.unwrap().as_bytes()).unwrap();
            writer.write(b"\n").unwrap();
        } else {
            println!("Skipping line {}", i);
        }
    }

    writer.flush().unwrap();
}
