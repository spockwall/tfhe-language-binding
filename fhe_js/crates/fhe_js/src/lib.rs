pub mod apis {
    pub mod base64;
    pub mod fhe;
    pub mod fhe_ops;
    pub mod http;
    pub mod key_generator;
    pub mod proven_fhe_ops;
    pub mod serializer;
    pub mod server_key_setter;
}
pub mod utils {
    pub mod conversion;
}
pub mod configs {
    pub mod error;
    pub mod typing;
    pub mod zk_params;
}

//#[neon::main]
//fn main(mut cx: ModuleContext) -> NeonResult<()> {
//    Ok(())
//}
