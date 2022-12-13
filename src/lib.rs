use std::fmt;
use std::ptr::null_mut;

extern "system" {
    fn DetourTransactionBegin() -> u32;
    fn DetourAttachEx(interceptee: *mut*mut(),
                      interceptor: *mut(),
                      trampoline: *mut*mut(),
                      real_target: *mut*mut(),
                      real_detour: *mut*mut()) -> u32;
    fn DetourTransactionCommit() -> u32;
}

pub struct DetourTransaction {}

pub struct Detour {
    interceptor: *mut(),
    interceptee: *mut(),
    trampoline: *mut(),
}

#[macro_export]
macro_rules! make_detour {
    ($interceptor:ident, $interceptee: ident) => {{
        Detour::new($interceptor as *mut(), $interceptee as *mut())
    }}
}

#[macro_export]
macro_rules! get_trampoline {
    ($detour:ident) => (
        transmute($detour.trampoline())
    )
}

#[derive(Debug, Clone)]
pub struct NullError;

impl fmt::Display for NullError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Trampoline is null. The detour likely failed to attach.")
    }
}

impl Detour {
    pub fn new(interceptor: *mut(), interceptee: *mut()) -> Self {
        Self {
            interceptor,
            interceptee,
            trampoline: null_mut(),
        }
    }

    pub fn trampoline(&self) -> Result<*mut(), NullError> {
        if self.trampoline == null_mut() { Ok(self.trampoline) } else { Err(NullError{})}
    }
}

impl Detour {
    pub fn transaction<Func: FnMut(&DetourTransaction)>(mut f: Func) {
        unsafe {
            assert_eq!(DetourTransactionBegin(), 0);
        }
        f(&DetourTransaction{});
        unsafe {
            assert_eq!(DetourTransactionCommit(), 0);
        }
    }
}

impl DetourTransaction {
    pub fn attach(&self, detour: &mut Detour) -> u32 {
        unsafe {
            DetourAttachEx((&mut detour.interceptee) as *mut*mut(),
                           detour.interceptor,
                           (&mut detour.trampoline) as *mut*mut(),
                           null_mut(),
                           null_mut())

        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::transmute;
    use super::*;

    pub extern "C" fn detouree(a: u32, b: u32) -> u32 {
        a + b
    }

    pub extern "C" fn detourer(a: u32, b: u32) -> u32 {
        a - b
    }

    #[test]
    fn test_detour() {
        let mut det = make_detour!(detourer, detouree);
        Detour::transaction(|detour| {
            assert_eq!(
                detour.attach(&mut det),
                0);
        });
        assert_eq!(detouree(5, 2), 3);
        unsafe {
            let f: fn(u32, u32) -> u32 = transmute(det.trampoline);
            assert_eq!(f(4,4), 8);
        }
    }
}
