use html5ever::driver;
use html5ever::serialize;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{RcDom, SerializableHandle};

#[test]
fn from_utf8() {
    let dom = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one("<title>Test".as_bytes());
    let mut serialized = Vec::new();
    let document: SerializableHandle = dom.document.clone().into();
    serialize::serialize(&mut serialized, &document, Default::default()).unwrap();
    assert_eq!(
        String::from_utf8(serialized).unwrap().replace(' ', ""),
        "<html><head><title>Test</title></head><body></body></html>"
    );
}

#[test]
fn input_ignored_in_select_fragment() {
    let dom = driver::parse_fragment(
        RcDom::default(),
        Default::default(),
        html5ever::QualName::new(None, html5ever::ns!(html), html5ever::local_name!("select")),
        vec![],
        true,
    )
    .one("<input><option>");
    let mut serialized = Vec::new();
    let html: SerializableHandle = dom.document.children.borrow()[0].clone().into();
    serialize::serialize(&mut serialized, &html, Default::default()).unwrap();
    assert_eq!(String::from_utf8(serialized).unwrap(), "<option></option>");
}

#[test]
fn many_templates() {
    let mut body = String::new();
    for _ in 1..10000 {
        body.push_str("<template>");
    }
    let _ = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(body.as_bytes());
}
