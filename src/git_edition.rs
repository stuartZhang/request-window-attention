#[cfg(any(feature = "nodejs", feature = "nw"))]
use ::node_bindgen::{core::{NjError, TryIntoJs, val::{JsEnv, JsObject}}, sys::napi_value};
use ::std::{ffi::{c_char, CString}, fmt::Display};

const GIT_BRANCH: &str = env!("GIT_BRANCH", r#"程序版本信息环境变量"GIT_BRANCH"没有被找到"#);
const GIT_TAG: &str = env!("GIT_TAG", r#"程序版本信息环境变量"GIT_TAG"没有被找到"#);
const GIT_LATEST_COMMIT_ID: &str = env!("GIT_LATEST_COMMIT_ID", r#"程序版本信息环境变量"GIT_LATEST_COMMIT_ID"没有被找到"#);
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION", r#"程序版本信息环境变量"CARGO_PKG_VERSION"没有被找到"#);
const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME", r#"程序版本信息环境变量"CARGO_PKG_NAME"没有被找到"#);

#[repr(C)]
pub struct GitEdition {
    branch: *const c_char,
    tag: *const c_char,
    latest_commit_id: *const c_char,
    pkg_name: *const c_char,
    pkg_version: *const c_char
}
#[cfg(any(feature = "nodejs", feature = "nw"))]
impl TryIntoJs for GitEdition {
    fn try_to_js(self, js_env: &JsEnv) -> Result<napi_value, NjError> {
        let mut json = JsObject::new(*js_env, js_env.create_object()?);
        macro_rules! set_str_prop {
            ($const: ident, $wField: literal) => {
                let js_val = js_env.create_string_utf8($const)?;
                json.set_property($wField, js_val)?;
            };
        }
        set_str_prop!(GIT_BRANCH, "branch");
        set_str_prop!(GIT_TAG, "tag");
        set_str_prop!(GIT_LATEST_COMMIT_ID, "latestCommitId");
        set_str_prop!(CARGO_PKG_NAME, "pkgName");
        set_str_prop!(CARGO_PKG_VERSION, "pkgVersion");
        json.try_to_js(js_env)
    }
}
impl Default for GitEdition {
    fn default() -> Self {
        GitEdition {
            branch: CString::new(GIT_BRANCH).unwrap().into_raw(),
            tag: CString::new(GIT_TAG).unwrap().into_raw(),
            latest_commit_id: CString::new(GIT_LATEST_COMMIT_ID).unwrap().into_raw(),
            pkg_name: CString::new(CARGO_PKG_NAME).unwrap().into_raw(),
            pkg_version: CString::new(CARGO_PKG_VERSION).unwrap().into_raw()
        }
    }
}
impl Display for GitEdition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"
            |{CARGO_PKG_NAME} 模块版本信息
            |----------------------------------
            |模块版本号       | {CARGO_PKG_VERSION}
            |发包标签         | {GIT_TAG}
            |代码分支名       | {GIT_BRANCH}
            |最后提交记录编号 | {GIT_LATEST_COMMIT_ID}
            |----------------------------------
        "#)
    }
}