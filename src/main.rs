use quick_xml::events::Event;
use quick_xml::Reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protain_id = "P0DTC2";
    let url = format!("https://www.uniprot.org/uniprot/{}.xml", protain_id);

    let xml = reqwest::get(url).await?.text().await?;

    println!("{:#?}", xml);

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"feature" => {
                        println!(
                            "attributes values: {:?}",
                            e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                        )
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                txt.push(e.unescape_and_decode(&reader).unwrap())
            },
            Ok(Event::Eof) => {
                break // exits the loop when reaching end of file
            },
            Err(e) => {
                panic!("Error at position {}: {:?}", reader.buffer_position(), e)
            },
            _ => (), // There are several other `Event`s we do not consider here
        }

        buf.clear();
    }

    println!("{:?}", txt);

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
