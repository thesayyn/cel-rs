mod google {
    mod rpc {
        include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));
    }
    pub mod api {
        pub mod expr {
            pub mod v1alpha1 {
                include!(concat!(env!("OUT_DIR"), "/google.api.expr.v1alpha1.rs"));
            }
            pub mod test {
                pub mod v1 {
                    include!(concat!(env!("OUT_DIR"), "/google.api.expr.test.v1.rs"));
                }
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

use darling::ast::NestedMeta;
use darling::{Error, FromMeta};
use google::api::expr::test::v1::{simple_test::ResultMatcher, SimpleTestFile};
use google::api::expr::v1alpha1::value::Kind;
use google::api::expr::v1alpha1::Value;
use proc_macro::TokenStream;
use prost::Message;

fn expand_value(val: Value) -> String {
    match val.kind.unwrap() {
        Kind::NullValue(_) => "cel_rs::Val::new_null()".to_string(),
        Kind::BoolValue(b) => format!("cel_rs::Val::new_bool({})", b),
        Kind::Int64Value(i) => format!("cel_rs::Val::new_int({})", i),
        Kind::Uint64Value(ui) => format!("cel_rs::Val::new_uint({})", ui),
        Kind::DoubleValue(db) => format!("cel_rs::Val::new_double({}f64)", db),
        Kind::StringValue(str) => format!(
            "cel_rs::Val::new_string(std::rc::Rc::new(String::from_utf8({:?}.to_vec()).unwrap()))",
            str.as_bytes()
        ),
        Kind::BytesValue(bytes) => {
            format!("cel_rs::Val::new_bytes(Vec::from({:?}).into())", bytes)
        }
        Kind::MapValue(map) => format!(
            "cel_rs::Val::new_map(std::collections::HashMap::<cel_rs::Val, cel_rs::Val>::from([{}]).into())",
            map.entries.iter().map(|entry| {
               let key =  entry.key.clone().unwrap();
               let value = entry.value.clone().unwrap();

               format!("({}, {}),", expand_value(key), expand_value(value))
            }).collect::<Vec<String>>().join("\n")
        ),
        Kind::ListValue(list) => format!("cel_rs::Val::new_list({})", "Vec::new().into()"),
        Kind::EnumValue(en) => "TODO".to_string(),
        Kind::ObjectValue(obj) => "TODO".to_string(),
        Kind::TypeValue(ty) => "TODO".to_string(),
    }
}

fn expand_result_matcher(rm: Option<ResultMatcher>) -> String {
    if rm.is_none() {
        panic!("result matcher is none.");
    }
    if let ResultMatcher::Value(val) = rm.unwrap() {
        expand_value(val)
    } else {
        String::from("TODO")
    }
}

#[derive(Debug, FromMeta)]
struct MacroArgs {
    name: String,
    #[darling(multiple, rename = "skip_section")]
    skip_sections: Vec<String>,
    #[darling(multiple, rename = "skip_test")]
    skip_tests: Vec<String>,
}

#[proc_macro]
pub fn suite(rargs: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(rargs.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let mut lock = TESTS.lock().unwrap();
    let mut buf = lock.get_mut(args.name.as_str()).unwrap();
    let testfile = SimpleTestFile::decode(&mut buf).expect("msg");

    let mut ast = String::new();
    for section in testfile.section {

        if args.skip_sections.contains(&section.name){
            continue;
        }

        ast.push_str("pub mod ");
        ast.push_str(section.name.as_str());
        ast.push_str("{");

        for test in section.test {
            if args.skip_tests.contains(&test.name){
                continue;
            }

            let expected_value = expand_result_matcher(test.result_matcher);

            ast.push_str(&format!(r##"
                #[test]
                fn r#{name}() {{
                    let expr = r#"{expr}"#;
                    let program = cel_rs::Program::new(expr);
                    assert!(program.is_ok(), "failed to parse '{{}}'", expr);
                    let program = program.unwrap();
                    let mut ctx = cel_rs::Context::default();
                    let value = program.eval(&mut ctx);
                    let expected_value = {expected_value};
                    assert_eq!(value, expected_value);
                }}
            "##, name = test.name, expr = test.expr, expected_value = expected_value ).to_string());
        }

        ast.push_str("}");
    }
    println!("{}", ast);
    ast.parse().unwrap()
}
