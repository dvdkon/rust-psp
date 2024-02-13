/// A version of the macros module that exports macros that follow the same
/// interface, but only create function declarations.
///
/// This is useful since by default, psp creates function stubs with
/// #[no_mangle], so only one instance of this crate is able to be linked into
/// one binary. The "solution" to that is to build just once instance with
/// feature "sys-stubs" (with macros.rs) and others without (with
/// macros_declonly.rs).

/// A complex macro used to define and link a PSP system library.
macro_rules! psp_extern {
    // Generate body with default ABI.
    (__BODY $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {{
        paste! {
            extern "C" {
                pub fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }
            let func = [< __ $name _stub >];
            func($($arg),*)
        }
    }};

    // Generate body with an ABI mapper
    (__BODY $abi:ident $name:ident ($($arg:ident : $arg_ty:ty),*) $(-> $ret:ty)?) => {{
        type Func = unsafe extern "C" fn($($arg : $arg_ty),*) $(-> $ret)?;

        paste! {
            extern "C" {
                pub fn [< __ $name _stub >]($($arg : $arg_ty),*) $(-> $ret)?;
            }
            let func = [< __ $name _stub >] as Func;

            // The transmutes here are for newtypes that fit into a single
            // register.
            core::mem::transmute($abi(
                $(core::mem::transmute($arg)),*,
                core::mem::transmute(func),
            ))
        }
    }};

    (
        #![name = $lib_name:expr]
        #![flags = $lib_flags:expr]
        #![version = ($lib_major_version:expr, $lib_minor_version:expr)]

        $(
            #[psp($nid:expr $(, $abi:ident)?)]
            $(#[$attr:meta])*
            pub fn $name:ident($($arg:ident : $arg_ty:ty),* $(,)?)
            $(-> $ret:ty)?;
        )*
    ) => {
        paste! {
            #[allow(non_snake_case)]
            mod [< __ $lib_name _mod >] {
                #[allow(unused)]
                use super::*;

				extern "C" {
					$(
						$(#[$attr])*
						#[allow(non_snake_case, clippy::missing_safety_doc)]
						pub fn $name($($arg : $arg_ty),*) $(-> $ret)?;
					)*
				}
            }
        }

        paste! {
            $(
                pub use self :: [< __ $lib_name _mod >] :: $name;
            )*
        }
    }
}
