#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protain_id = "P0DTC2";
    let url = format!("https://www.uniprot.org/uniprot/{}.xml", protain_id);

    let xml = reqwest::get(url).await?.text().await?;

    let root: minidom::Element = xml.parse().unwrap();

    println!("{:#?}", root);

    Ok(())
}

struct Location {
    position: Option<u8>,
    begin: Option<u8>,
    end: Option<u8>,
}

struct Feature {
    f_type: String,
    description: String,
    id: String,
    evidence: u8,
    location: Location,
}
