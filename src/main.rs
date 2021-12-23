fn set_feature(child: &minidom::Element) -> Feature {
    let original = match child.get_child("original", namespace) {
        Some(e) => Some(e.text()),
        None => None,
    };
    let variation = match child.get_child("variation", namespace) {
        Some(e) => Some(e.text()),
        None => None,
    };

/*
    features.push(
        Feature {
            f_type: child.attr("type").unwrap(),
            description: child.attr("description").unwrap(),
            evidence: child.attr("evidence"),
            original,
            variation,
        }
    );
*/
}

fn is_seq_variant(child: &minidom::Element) -> bool {
    let f_type = child.attr("type").unwrap();
    f_type == "sequence variant"
}

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
            if is_seq_variant(child) {
                println!("{:#?}", child);
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
    original: Option<String>,
    variation: Option<String>,
    location: Location,
}
