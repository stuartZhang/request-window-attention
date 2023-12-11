use ::node_bindgen::{core::{napi_call_result, JSValue, NjError, TryIntoJs, val::JsEnv}, sys::{napi_call_function, napi_ref, napi_value, napi_valuetype_napi_function}};
use ::std::ptr;

#[derive(Clone)]
pub struct JsGlobalCallbackFunction {
    js_func: napi_ref,
    env: JsEnv,
}
unsafe impl Send for JsGlobalCallbackFunction {}
unsafe impl Sync for JsGlobalCallbackFunction {}
impl JSValue<'_> for JsGlobalCallbackFunction {
    fn label() -> &'static str {
        "ref_callback"
    }
    fn convert_to_rust(env: &JsEnv, js_value: napi_value) -> Result<Self, NjError> {
        env.assert_type(js_value, napi_valuetype_napi_function)?;
        let js_ref = env.create_reference(js_value, 1)?;
        Ok(Self {
            js_func: js_ref,
            env: *env,
        })
    }
}
impl Drop for JsGlobalCallbackFunction {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        let _ = self.call(vec!["[JsGlobalCallbackFunction] 释放 logging 回调函数的全局引用".to_string()]);
        self.env.delete_reference(self.js_func).unwrap();
    }
}
impl JsGlobalCallbackFunction {
    pub fn call<T>(&self, rust_argv: Vec<T>) -> Result<napi_value, NjError>
    where T: TryIntoJs {
        let env = &self.env;
        let mut argv: Vec<napi_value> = vec![];
        for rust_arg in rust_argv {
            match rust_arg.try_to_js(env) {
                Ok(js_value) => argv.push(js_value),
                Err(err) => return Err(err),
            }
        }
        let js_value = self.env.get_reference_value(self.js_func)?;
        let mut result = ptr::null_mut();
        let ctx = self.env.get_global()?;
        napi_call_result!(napi_call_function(
            self.env.inner(),
            ctx,
            js_value,
            argv.len(),
            argv.as_mut_ptr(),
            &mut result
        ))?;
        Ok(result)
    }
}
