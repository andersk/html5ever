use html5ever::driver;
use html5ever::serialize;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{RcDom, SerializableHandle};

#[test]
fn form_element_pointer_ignored_in_template_contents() {
    fn serialize_children(node: markup5ever_rcdom::Handle) -> String {
        let mut serialized = Vec::new();
        let node: SerializableHandle = node.into();
        serialize::serialize(&mut serialized, &node, Default::default()).unwrap();
        String::from_utf8(serialized).unwrap()
    }

    // A form start tag in a table in template contents inserts a form
    let dom = driver::parse_document(RcDom::default(), Default::default())
        .one("<template><table><form></table></template>");
    let html = dom.document.children.borrow()[0].clone();
    let head = html.children.borrow()[0].clone();
    let template = head.children.borrow()[0].clone();
    let markup5ever_rcdom::NodeData::Element {
        ref template_contents,
        ..
    } = template.data
    else {
        panic!("expected a template element");
    };
    let contents = template_contents.borrow().clone().unwrap();
    assert_eq!(serialize_children(contents), "<table><form></form></table>");

    // When fragment parsing with a template context, nested forms are not dropped
    let dom = driver::parse_fragment(
        RcDom::default(),
        Default::default(),
        html5ever::QualName::new(
            None,
            html5ever::ns!(html),
            html5ever::local_name!("template"),
        ),
        vec![],
        true,
    )
    .one("<form><form>");
    let html = dom.document.children.borrow()[0].clone();
    assert_eq!(serialize_children(html), "<form><form></form></form>");
}

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
fn many_templates() {
    let mut body = String::new();
    for _ in 1..10000 {
        body.push_str("<template>");
    }
    let _ = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(body.as_bytes());
}
