use std::io::Write;
use std::fs::File;

fn set_location(feature: &minidom::Element, namespace: &str) -> Location {
    let location = feature.get_child("location", namespace).unwrap();
    let position = match location.get_child("position", namespace) {
        Some(e) => e.attr("position").map(|p| p.parse::<u32>().unwrap()),
        None => None,
    };
    let begin = match location.get_child("begin", namespace) {
        Some(e) => e.attr("position").map(|p| p.parse::<u32>().unwrap()),
        None => None,
    };
    let end = match location.get_child("end", namespace) {
        Some(e) => e.attr("position").map(|p| p.parse::<u32>().unwrap()),
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
        f_type: feature
            .attr("type")
            .unwrap()
            .to_string(),
        description: feature
            .attr("description")
            .unwrap()
            .to_string()
            .replace(",", "")
            .replace("Frequently in strain ", "")
            .replace("In strain: ", ""),
        evidence: feature
            .attr("evidence")
            .map(|s| s.to_string()),
        original: feature
            .get_child("original", namespace)
            .map(|e| e.text()),
        variation: feature
            .get_child("variation", namespace)
            .map(|e| e.text()),
        location: set_location(feature, namespace),
    }
}

fn is_seq_variant(feature: &minidom::Element) -> bool {
    let f_type = feature.attr("type").unwrap();
    f_type == "sequence variant"
}

fn write_to_file(path: &str, features: Vec<Feature>) -> std::io::Result<()> {
    let mut file = File::create(path).unwrap();

    for feature in features {
        dbg!(&feature.location);

        writeln!(
            &mut file,
            "{}, {}, {}, {}, {}",
            feature.description,
            feature.original
                .unwrap_or("".to_string()),
            feature.variation
                .unwrap_or("".to_string()),
            feature.location.begin
                .map(|x| format!("{}", x))
                .unwrap_or_else(
                    || feature.location.position
                        .map(|x| format!("{}", x))
                        .unwrap()
                ),
            feature.location.end
                .map(|x| format!("{}", x))
                .unwrap_or("".to_string()),
        )?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protain_id = "P0DTC2";
    let url = format!("https://www.uniprot.org/uniprot/{}.xml", protain_id);
    let xml = reqwest::get(url).await?.text().await?;

    const NAMESPACE: &str = "http://uniprot.org/uniprot";
    let root: minidom::Element = xml.parse().unwrap();
    let entry: &minidom::Element = root.get_child("entry", NAMESPACE).unwrap();
    let mut features: Vec<Feature> = Vec::new(); 

    for child in entry.children() {
        if child.is("feature", NAMESPACE) {
            if is_seq_variant(child) {
                features.push(set_feature(child, NAMESPACE));
            }
        }
    }

    let path: &str = "./variations.csv";
    write_to_file(path, features)
        .expect("writing error.");

    Ok(())
}

#[derive(Debug)]
struct Location {
    position: Option<u32>,
    begin: Option<u32>,
    end: Option<u32>,
}

#[derive(Debug)]
struct Feature {
    f_type: String,
    description: String,
    evidence: Option<String>,
    original: Option<String>,
    variation: Option<String>,
    location: Location,
}
