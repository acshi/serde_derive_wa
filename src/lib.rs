use proc_macro::TokenStream;
use watt::WasmMacro;

static WASM: &[u8] = include_bytes!("serde_derive.wasm");
static MACRO: WasmMacro = WasmMacro::new(WASM);

#[proc_macro_derive(Serialize, attributes(serde))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ts = MACRO.proc_macro_derive("derive_serialize", input);
    // eprintln!("{:#}", ts);
    ts
}

#[proc_macro_derive(Deserialize, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ts = MACRO.proc_macro_derive("derive_deserialize", input);
    // eprintln!("{:#}", ts);
    ts
}
