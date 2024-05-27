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
use google::api::expr::v1alpha1::{Value, value};
use google::api::expr::v1alpha1::{ExprValue, expr_value};
use proc_macro::TokenStream;
use prost::Message;

fn expand_expr_value(val: ExprValue) -> String {
    match val.kind.unwrap() {
        expr_value::Kind::Value(val) => expand_value(val),
        expr_value::Kind::Error(_) => String::from("TODO: ExprValue::Error"),
        expr_value::Kind::Unknown(_) =>  String::from("TODO: ExprValue::Unknown"),
    }
}

fn expand_value(val: Value) -> String {
    match val.kind.unwrap() {
        value::Kind::NullValue(_) => "cel_rs::Val::new_null()".to_string(),
        value::Kind::BoolValue(b) => format!("cel_rs::Val::new_bool({})", b),
        value::Kind::Int64Value(i) => format!("cel_rs::Val::new_int({})", i),
        value::Kind::Uint64Value(ui) => format!("cel_rs::Val::new_uint({})", ui),
        value::Kind::DoubleValue(db) => format!("cel_rs::Val::new_double({}f64)", db),
        value::Kind::StringValue(str) => format!(
            "cel_rs::Val::new_string(std::rc::Rc::new(String::from_utf8({:?}.to_vec()).unwrap()))",
            str.as_bytes()
        ),
        value::Kind::BytesValue(bytes) => {
            format!("cel_rs::Val::new_bytes(Vec::from({:?}).into())", bytes)
        }
        value::Kind::MapValue(map) => format!(
            "cel_rs::Val::new_map(std::collections::HashMap::<cel_rs::Val, cel_rs::Val>::from([{}]).into())",
            map.entries.iter().map(|entry| {
               let key =  entry.key.clone().unwrap();
               let value = entry.value.clone().unwrap();

               format!("({}, {}),", expand_value(key), expand_value(value))
            }).collect::<Vec<String>>().join("\n")
        ),
        value::Kind::ListValue(list) => format!("cel_rs::Val::new_list({})", "Vec::new().into()"),
        value::Kind::EnumValue(en) => "TODO: EnumValue".to_string(),
        value::Kind::ObjectValue(obj) => "TODO: ObjectValue".to_string(),
        value::Kind::TypeValue(ty) => "TODO: TypeValue".to_string(),
    }
}

fn expand_result_matcher(rm: Option<ResultMatcher>) -> String {
    if rm.is_none() {
        panic!("result matcher is none.");
    }

    match rm.unwrap() {
        ResultMatcher::Value(val) => expand_value(val),
        ResultMatcher::EvalError(err) => format!("cel_rs::Val::new_error({:?}.into())", err.errors[0].message),
        ResultMatcher::AnyEvalErrors(eval) => format!("TODO: AnyEvalErrors: {:#?}", eval),
        ResultMatcher::Unknown(unk) => format!("TODO: Unknown: {:#?}", unk),
        ResultMatcher::AnyUnknowns(anyunk) => format!("TODO: AnyUnknowns: {:#?}", anyunk),
    }
}

fn expand_bindings(bindings: HashMap<String, ExprValue>) -> String {
    let mut exp = String::new();

    for (k, v) in bindings.iter() {
        exp.push_str(
            format!(
                r#"ctx.add_variable("{k}", {v});"#,
                k = k,
                v = expand_expr_value(v.clone())
            )
            .as_str(),
        )
    }

    exp
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
        if args.skip_sections.contains(&section.name) {
            continue;
        }

        ast.push_str("pub mod ");
        ast.push_str(section.name.as_str());
        ast.push_str("{");

        for test in section.test {
            if args.skip_tests.contains(&test.name) {
                continue;
            }

            let expected_value = expand_result_matcher(test.result_matcher);

            let bindings = expand_bindings(test.bindings);

            ast.push_str(
                &format!(
                    r##"
                #[test]
                fn r#{name}() {{
                    let expr = r#"{expr}"#;
                    let program = cel_rs::Program::new(expr);
                    assert!(program.is_ok(), "failed to parse '{{}}'", expr);
                    let program = program.unwrap();
                    let mut ctx = cel_rs::Context::default();
                    {bindings}
                    let value = program.eval(&mut ctx);
                    let expected_value = {expected_value};
                    assert_eq!(value, expected_value);
                }}
            "##,
                    name = test.name,
                    expr = test.expr,
                    expected_value = expected_value,
                    bindings = bindings
                )
                .to_string(),
            );
        }

        ast.push_str("}");
    }
    println!("{}", ast);
    ast.parse().unwrap()
}
