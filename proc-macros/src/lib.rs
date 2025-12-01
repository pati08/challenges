// use proc_macro::TokenStream;
// use syn::parse_macro_input;
//
// #[proc_macro]
// pub fn mk_test_input(item: TokenStream) -> TokenStream {
//     mk_test_input_impl(item)
// }
//
// fn mk_test_input_impl(item: TokenStream) -> TokenStream {
//     let str_lit = parse_macro_input!(item as syn::LitStr);
//     let str_val = str_lit
//         .value()
//         .lines()
//         .map(|i| syn::LitStr::new(i, str_lit.span()))
//         .collect::<Vec<_>>();
//     quote::quote! {{
//
//         challenges_input::Input::new(
//             vec![
//                 #( #str_val ),*
//             ]
//         )
//     }}
//     .into()
// }
