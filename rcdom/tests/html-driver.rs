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
fn many_templates() {
    let mut body = String::new();
    for _ in 1..10000 {
        body.push_str("<template>");
    }
    let _ = driver::parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .one(body.as_bytes());
}

#[test]
fn frameset_after_head_template() {
    // A template in the head, even one whose contents set frameset-ok to "not ok", doesn't stop a
    // later frameset from replacing the implied body.  (RcDom doesn't serialize template contents.)
    let expected = "<html><head><template></template></head><frameset></frameset></html>";
    for input in [
        "<head><template></template></head><p><frameset>",
        "<template>x</template><p><frameset>",
    ] {
        let dom = driver::parse_document(RcDom::default(), Default::default()).one(input);
        let mut serialized = Vec::new();
        let document: SerializableHandle = dom.document.clone().into();
        serialize::serialize(&mut serialized, &document, Default::default()).unwrap();
        assert_eq!(String::from_utf8(serialized).unwrap(), expected);
    }
}
