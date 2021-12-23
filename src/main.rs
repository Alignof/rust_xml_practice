#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protain_id = "P0DTC2";
    let url = format!("https://www.uniprot.org/uniprot/{}.xml", protain_id);
    let xml = reqwest::get(url).await?.text().await?;

    const namespace: &str = "http://uniprot.org/uniprot";
    let root: minidom::Element = xml.parse().unwrap();
    let entry: &minidom::Element = root.get_child("entry", namespace).unwrap();
    let mut features: Vec<Feature> = Vec::new(); 

    for child in entry.children() {
        if child.is("feature", namespace) {
            let f_type = child.attr("type").unwrap();
            if f_type == "sequence variant" {

                println!("{:#?}", child);

            /*
                features.push(
                    Feature {
                        f_type,
                        description: child.attr("description").unwrap(),
                        id: ,
                        evidence: child.attr("evidence"),
                    }
                );
            */
            }
        }
    }

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
    evidence: Option<u8>,
    location: Location,
}
