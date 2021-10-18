use rscript::{FFiVec, Hook, ScriptInfo, ScriptType, VersionReq};
mod script;

struct Vim {
    state: State,
    mode: Mode,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum State {
    Empty,
    c,
    ci,
    d,
    di,
    g,
    f,
    F,
    r,
}

#[derive(PartialEq)]
enum Mode {
    Normal,
    Insert,
}

static mut VIM: Vim = Vim::new();

#[no_mangle]
pub fn script_info() -> FFiVec {
    let info = ScriptInfo::new(
        "VimDylib",
        ScriptType::DynamicLib,
        &[
            irust_api::InputEvent::NAME,
            irust_api::Shutdown::NAME,
            irust_api::Startup::NAME,
        ],
        VersionReq::parse(">=1.19.0").expect("correct version requirement"),
    );
    FFiVec::serialize_from(&info).unwrap()
}

/// # Safety
/// No stable ABI => Not safe
#[no_mangle]
pub extern "C" fn script(hook: FFiVec, data: FFiVec) -> FFiVec {
    let hook: String = hook.deserialize().unwrap();
    unsafe {
        match hook.as_str() {
            irust_api::InputEvent::NAME => {
                let hook: irust_api::InputEvent = data.deserialize().unwrap();
                let output = VIM.handle_input_event(hook);
                FFiVec::serialize_from(&output).unwrap()
            }
            irust_api::Shutdown::NAME => {
                let hook: irust_api::Shutdown = data.deserialize().unwrap();
                let output = VIM.clean_up(hook);
                FFiVec::serialize_from(&output).unwrap()
            }
            irust_api::Startup::NAME => {
                let hook: irust_api::Startup = data.deserialize().unwrap();
                let output = VIM.start_up(hook);
                FFiVec::serialize_from(&output).unwrap()
            }
            _ => unreachable!(),
        }
    }
}
