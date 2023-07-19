use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn cache_policy(mut cx: FunctionContext) -> JsResult<JsArray> {
    let list_cache: Handle<JsArray> = cx.empty_array();
    let str_hello_01 = cx.string("hello1");
    let str_hello_02 = cx.string("hello2");
    let str_hello_03 = cx.string("hello3");
    list_cache.set(&mut cx, 0, str_hello_01)?;
    list_cache.set(&mut cx, 1, str_hello_02)?;
    list_cache.set(&mut cx, 2, str_hello_03)?;
    Ok(list_cache)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("cache_policy", cache_policy)?;
    Ok(())
}
