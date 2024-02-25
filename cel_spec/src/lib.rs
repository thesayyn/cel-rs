use proc_macro::TokenStream;

const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cel.bin"));

fn get_expected_value(value: &prost_reflect::DynamicMessage) -> String {
    let field = value.fields().next().unwrap();
    let (m, v, c) = match field.0.name() {
        "string_value" => ("String(std::rc::Rc::new(String::from(", format!(r##"r#"{}"#"##, field.1.as_str().unwrap()), ")))"),
        "bool_value" => ("Bool(", format!("{}", field.1.as_bool().unwrap()), ")"),
        "int64_value" => ("Int(", format!("{}", field.1.as_i64().unwrap()), ")"),
        "uint64_value" => ("UInt(", format!("{}", field.1.as_u64().unwrap()), ")"),
        "double_value" => ("Float(", format!("{}.into()", field.1.as_f64().unwrap()), ")"),
        "map_value" => ("Map(", String::from("ordered_hash_map::OrderedHashMap::new().into()"), ")"),
        "bytes_value" => ("Bytes(std::rc::Rc::new(Vec::from(", format!("{:?}", field.1.as_bytes().unwrap().to_vec()), ")))"),
        _ => ("Null", String::new(), "")
    };
    format!("program::value::Value::{}{}{}", m, v, c)
}

#[proc_macro]
pub fn suite(attr: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    use prost_reflect::{DescriptorPool, DynamicMessage};

    let pool = DescriptorPool::decode(BYTES.as_ref()).unwrap();
    let message_descriptor = pool
        .get_message_by_name("google.api.expr.test.v1.SimpleTestFile")
        .unwrap();
    let bytes = include_str!("../cel-spec/tests/simple/testdata/basic.textproto");
    let suite = DynamicMessage::parse_text_format(message_descriptor, &bytes).unwrap();

    let mut ast = String::new();
    for section in suite
        .get_field_by_name("section")
        .unwrap()
        .as_list()
        .unwrap()
    {
        let section = section.as_message().unwrap();
        let sname = section.get_field_by_name("name").unwrap();

        ast.push_str(format!("pub mod {}{{", sname.as_str().unwrap()).as_str());

        for case in section
            .get_field_by_name("test")
            .unwrap()
            .as_list()
            .unwrap()
        {
            let case = case.as_message().unwrap();
            let name = case.get_field_by_name("name").unwrap();
            let expr = case.get_field_by_name("expr").unwrap();
            let value = case.get_field_by_name("value").unwrap();

            let name = name.as_str().unwrap();
            let expr = expr.as_str().unwrap();
            let value = value.as_message().unwrap();
            let expected_value = get_expected_value(value);

            ast.push_str(&format!(r##"
                #[test]
                fn {name}() {{
                    let program = program::Program::new(r#"{expr}"#);
                    assert!(program.is_ok(), "failed to parse '{{}}'", r#"{expr}"#);
                    let program = program.unwrap();
                    let ctx = program::context::Context::default();
                    let value = program.eval(ctx);
                    let expected_value = {expected_value};
                    assert_eq!(value, expected_value, r#""{{}}" did not match "{{}}""#, value, expected_value);
                }}
            "##, name = name, expr = expr, expected_value = expected_value ).to_string());
        }

        ast.push_str("}");
        break;
    }
    println!("{}", ast);
    ast.parse().unwrap()
}
