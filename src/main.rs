fn set_location(feature: &minidom::Element, namespace: &str) -> Location {
    let location = feature.get_child("location", namespace).unwrap();
    dbg!(location);
    let position = match location.get_child("position", namespace) {
        Some(e) => e.attr("position").map(|p| p.parse::<u32>().unwrap()),
        None => None,
    };
    let begin = match location.get_child("begin", namespace) {
        Some(e) => e.attr("begin").map(|p| p.parse::<u32>().unwrap()),
        None => None,
    };
    let end = match location.get_child("end", namespace) {
        Some(e) => e.attr("end").map(|p| p.parse::<u32>().unwrap()),
        None => None,
    };

    Location {
        position,
        begin,
        end,
    }
}

fn set_feature(feature: &minidom::Element, namespace: &str) -> Feature {
    Feature {
        f_type: feature.attr("type").unwrap().to_string(),
        description: feature.attr("description").unwrap().to_string(),
        evidence: feature.attr("evidence").map(|s| s.to_string()),
        original: feature.get_child("original", namespace).map(|e| e.text()),
        variation: feature.get_child("variation", namespace).map(|e| e.text()),
        location: set_location(feature, namespace),
    }
}

fn is_seq_variant(feature: &minidom::Element) -> bool {
    let f_type = feature.attr("type").unwrap();
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
                features.push(set_feature(child, namespace));
            }
        }
    }

    Ok(())
}

struct Location {
    position: Option<u32>,
    begin: Option<u32>,
    end: Option<u32>,
}

struct Feature {
    f_type: String,
    description: String,
    evidence: Option<String>,
    original: Option<String>,
    variation: Option<String>,
    location: Location,
}
