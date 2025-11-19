use std::env;
use std::fs;
use std::path::Path;

const PUBLIC_SERVER_LIST_URL: &str = "https://publist.mumble.info/v1/list";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("public_server_list.rs");
    let cached_xml_path = Path::new(&out_dir).join("public_servers.xml");

    println!("cargo:rerun-if-changed=build.rs");

    // Attempt to fetch the server list
    println!("--- Mumble Build Script: Attempting to fetch public server list...");
    let server_xml = match reqwest::blocking::get(PUBLIC_SERVER_LIST_URL) {
        Ok(resp) => {
            if resp.status().is_success() {
                let xml = resp.text().unwrap_or_default();
                println!("--- Mumble Build Script: Successfully fetched and cached server list.");
                fs::write(&cached_xml_path, &xml).expect("Unable to write cached XML");
                xml
            } else {
                println!("--- Mumble Build Script: Fetch failed with status: {}", resp.status());
                // Fetch failed, try to use cache
                fs::read_to_string(&cached_xml_path).expect("Failed to fetch public server list and no cached version was available. Please check your internet connection.")
            }
        }
        Err(e) => {
            println!("--- Mumble Build Script: Fetch failed with error: {}", e);
            // If fetching fails, try to use the cached version
            fs::read_to_string(&cached_xml_path).expect("Failed to fetch public server list and no cached version was available. Please check your internet connection.")
        }
    };

    let rust_code = format!(
        "const PUBLIC_SERVER_LIST_XML: &str = r#\"{}\"#;",
        server_xml
    );

    fs::write(&dest_path, rust_code).unwrap();
}