use std::io::Read;
use proc_macro::TokenStream;
use static_init::dynamic;
use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use prost_reflect::{DescriptorPool, MessageDescriptor, DynamicMessage};

const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cel.bin"));
#[dynamic] 
static MESSAGE_DESCRIPTOR: MessageDescriptor = {
    let pool = DescriptorPool::decode(BYTES.as_ref()).unwrap();
    pool
        .get_message_by_name("google.api.expr.test.v1.SimpleTestFile")
        .unwrap()
};

fn get_expected_value(value: &DynamicMessage) -> String {
    let field = value.fields().next().expect("get expected value");
    let (m, v, c) = match field.0.name() {
        "string_value" => (
            "String(std::rc::Rc::new(String::from_utf8(",
            format!("{:?}", field.1.as_str().unwrap().as_bytes().to_vec()),
            ".to_vec()).unwrap()))",
        ),
        "bool_value" => ("Bool(", format!("{}", field.1.as_bool().unwrap()), ")"),
        "int64_value" => ("Int(", format!("{}", field.1.as_i64().unwrap()), ")"),
        "uint64_value" => ("UInt(", format!("{}", field.1.as_u64().unwrap()), ")"),
        "double_value" => (
            "Float(",
            format!("({}).into()", field.1.as_f64().unwrap()),
            ")",
        ),
        "map_value" => (
            "Map(",
            String::from("ordered_hash_map::OrderedHashMap::new().into()"),
            ")",
        ),
        "list_value" => ("List(", String::from("Vec::new().into()"), ")"),
        "bytes_value" => (
            "Bytes(std::rc::Rc::new(Vec::from(",
            format!("{:?}", field.1.as_bytes().unwrap().to_vec()),
            ")))",
        ),
        _ => ("Null", String::new(), ""),
    };
    format!("cel_rs::value::Value::{}{}{}", m, v, c)
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    name: String,
    #[darling(multiple, rename = "include")]
    includes: Vec<String>,
}


#[proc_macro]
pub fn suite(rargs: TokenStream) -> TokenStream {

    let attr_args = match NestedMeta::parse_meta_list(rargs.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    let mut file = std::fs::File::open(format!(
        "{}/cel-spec/tests/simple/testdata/{}.textproto",
        env!("CARGO_MANIFEST_DIR"),
        args.name
    ))
    .expect("could not find the suite");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("can not read the suite file");
    let suite = DynamicMessage::parse_text_format(MESSAGE_DESCRIPTOR.to_owned(), &content).unwrap();

    let mut ast = String::new();
    for section in suite
        .get_field_by_name("section")
        .unwrap()
        .as_list()
        .unwrap()
    {
        let section = section.as_message().unwrap();
        let sname = section.get_field_by_name("name").unwrap();
        let sname = sname.as_str().unwrap();
        let includes = &args.includes;
        if includes.into_iter().find(|p| p.as_str() == sname).is_none(){
            println!("skip {}", &sname);
            continue;
        }

        ast.push_str(format!("pub mod {}{{", sname).as_str());

        for case in section
            .get_field_by_name("test")
            .expect("test")
            .as_list()
            .expect("test as list")
        {
            let case = case.as_message().expect("message as case");
            let name = case.get_field_by_name("name").expect("expected name");
            let expr = case.get_field_by_name("expr").expect("expected expr");
            let value = case.get_field_by_name("value").expect("expected value");

            let name = name.as_str().expect("name as str");
            let expr = expr.as_str().expect("expr as str");
            let value = value.as_message().expect("value as message");
            let expected_value = get_expected_value(value);

            ast.push_str(&format!(r##"
                #[test]
                fn {name}() {{
                    let expr = r#"{expr}"#;
                    let program = cel_rs::Program::new(expr);
                    assert!(program.is_ok(), "failed to parse '{{}}'", expr);
                    let program = program.unwrap();
                    let mut ctx = cel_rs::context::Context::default();
                    let value = program.eval(&mut ctx);
                    let expected_value = {expected_value};
                    assert_eq!(value, expected_value, r#""{{}}" did not match "{{}}""#, value, expected_value);
                }}
            "##, name = name, expr = expr, expected_value = expected_value ).to_string());
        }

        ast.push_str("}");
    }
    println!("{}", ast);
    ast.parse().unwrap()
}
