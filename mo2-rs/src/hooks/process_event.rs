use retour::static_detour;
use sdk::core::classes::UObject;
use sdk::core::constants::PROCESS_EVENTS_INDEX;

type FnProcessEvent =
    unsafe extern "fastcall" fn(a1: *const UObject, a2: *const UObject, params: *mut usize);

static_detour! {
    static ProcessEvent: unsafe extern "fastcall" fn(
        *const UObject,
        *const UObject,
        *mut usize);
}
fn hk_process_event(a1: *const UObject, a2: *const UObject, params: *mut usize) {
    unsafe {
        let name = (*a2).get_full_name();
        if name != "Function Engine.HUD.ReceiveDrawHUD" {
            ProcessEvent.call(a1, a2, params);
            return;
        }

        ProcessEvent.call(a1, a2, params);
    }
}

pub fn hook_process_event(object: &UObject) -> bool {
    let vf_table = object.vf_table;
    unsafe {
        let address = *vf_table.add(PROCESS_EVENTS_INDEX);
        let fn_process_event: FnProcessEvent = std::mem::transmute(address as *const usize);

        ProcessEvent.initialize(fn_process_event, hk_process_event).unwrap().enable().unwrap();

        fn_process_event as u64 > 0
    }
}

pub fn unhook_process_event() {
    if ProcessEvent.is_enabled() {
        unsafe {
            ProcessEvent.disable().unwrap();
        }
    }
}
