use neon::prelude::*;
use crate::api::process_one_str;

fn translate(mut cx: FunctionContext) -> JsResult<JsString> {
    let mapping = cx.argument::<JsString>(0)?.value(&mut cx);
    let translated = process_one_str(mapping.as_str());

    Ok(cx.string(translated))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("translate", translate)?;
    Ok(())
}
