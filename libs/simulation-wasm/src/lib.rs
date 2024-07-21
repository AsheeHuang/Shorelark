use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn whos_that_dog() -> String {
	"Mister Peanutbutter!".into()
}

pub extern "C" fn __wasm_bindgen_generated_whos_that_dog() ->
	<String as wasm_bindgen::convert::ReturnWasmAbi>::Abi
{
	let _ret = { whos_that_dog() };
	<String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
}
