use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};

const BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cel.bin"));

#[proc_macro]
pub fn cel_test(attr: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    use prost_reflect::{DescriptorPool, DynamicMessage};

    let pool = DescriptorPool::decode(BYTES.as_ref()).unwrap();
    let message_descriptor = pool.get_message_by_name("google.api.expr.test.v1.SimpleTestFile").unwrap();
    let bytes = include_str!("../cel-spec/tests/simple/testdata/basic.textproto");
    let suite = DynamicMessage::parse_text_format(message_descriptor, &bytes).unwrap();

    let mut ast = String::new();
    for section in suite.get_field_by_name("section").unwrap().as_list().unwrap() {
        let section = section.as_message().unwrap();
        let sname = section.get_field_by_name("name").unwrap();

        ast.push_str(format!("pub mod {}{{", sname.as_str().unwrap()).as_str());

        for case in section.get_field_by_name("test").unwrap().as_list().unwrap() {
            let case = case.as_message().unwrap();
            let name = case.get_field_by_name("name").unwrap();
            let expr = case.get_field_by_name("expr").unwrap();
   
            let name = format_ident!("{}", name.as_str().unwrap());
            let expr = expr.as_str().unwrap();
            ast.push_str(
                &quote!{
                    #[test]
                    fn #name() {
                        assert!(program::Program::new(#expr).expect("failed to compile").execute(program::context::Context::default()));
                    }
                }.to_string()
            )
        }

        ast.push_str("}");
        break;
    }
    ast.parse().unwrap()
}