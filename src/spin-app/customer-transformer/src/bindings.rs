pub type Message = fermyon::spin_sqs::sqs_types::Message;
pub type MessageAction = fermyon::spin_sqs::sqs_types::MessageAction;
pub type Error = fermyon::spin_sqs::sqs_types::Error;
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_handle_queue_message_cabi<T: Guest>(
    arg0: i32,
    arg1: *mut u8,
    arg2: usize,
    arg3: *mut u8,
    arg4: usize,
    arg5: i32,
    arg6: *mut u8,
    arg7: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let base16 = arg3;
    let len16 = arg4;
    let mut result16 = _rt::Vec::with_capacity(len16);
    for i in 0..len16 {
        let base = base16.add(i * 32);
        let e16 = {
            let l1 = *base.add(0).cast::<*mut u8>();
            let l2 = *base.add(4).cast::<usize>();
            let len3 = l2;
            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
            let l4 = i32::from(*base.add(8).cast::<u8>());
            let l8 = i32::from(*base.add(20).cast::<u8>());
            use fermyon::spin_sqs::sqs_types::MessageAttributeValue as V15;
            let v15 = match l8 {
                0 => {
                    let e15 = {
                        let l9 = *base.add(24).cast::<*mut u8>();
                        let l10 = *base.add(28).cast::<usize>();
                        let len11 = l10;
                        let bytes11 = _rt::Vec::from_raw_parts(l9.cast(), len11, len11);
                        _rt::string_lift(bytes11)
                    };
                    V15::Str(e15)
                }
                n => {
                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                    let e15 = {
                        let l12 = *base.add(24).cast::<*mut u8>();
                        let l13 = *base.add(28).cast::<usize>();
                        let len14 = l13;
                        _rt::Vec::from_raw_parts(l12.cast(), len14, len14)
                    };
                    V15::Binary(e15)
                }
            };
            fermyon::spin_sqs::sqs_types::MessageAttribute {
                name: _rt::string_lift(bytes3),
                data_type: match l4 {
                    0 => None,
                    1 => {
                        let e = {
                            let l5 = *base.add(12).cast::<*mut u8>();
                            let l6 = *base.add(16).cast::<usize>();
                            let len7 = l6;
                            let bytes7 = _rt::Vec::from_raw_parts(l5.cast(), len7, len7);
                            _rt::string_lift(bytes7)
                        };
                        Some(e)
                    }
                    _ => _rt::invalid_enum_discriminant(),
                },
                value: v15,
            }
        };
        result16.push(e16);
    }
    _rt::cabi_dealloc(base16, len16 * 32, 4);
    let result18 = T::handle_queue_message(fermyon::spin_sqs::sqs_types::Message {
        id: match arg0 {
            0 => None,
            1 => {
                let e = {
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    _rt::string_lift(bytes0)
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        },
        message_attributes: result16,
        body: match arg5 {
            0 => None,
            1 => {
                let e = {
                    let len17 = arg7;
                    let bytes17 = _rt::Vec::from_raw_parts(arg6.cast(), len17, len17);
                    _rt::string_lift(bytes17)
                };
                Some(e)
            }
            _ => _rt::invalid_enum_discriminant(),
        },
    });
    let ptr19 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result18 {
        Ok(e) => {
            *ptr19.add(0).cast::<u8>() = (0i32) as u8;
            *ptr19.add(4).cast::<u8>() = (e.clone() as i32) as u8;
        }
        Err(e) => {
            *ptr19.add(0).cast::<u8>() = (1i32) as u8;
            use fermyon::spin_sqs::sqs_types::Error as V21;
            match e {
                V21::Other(e) => {
                    *ptr19.add(4).cast::<u8>() = (0i32) as u8;
                    let vec20 = (e.into_bytes()).into_boxed_slice();
                    let ptr20 = vec20.as_ptr().cast::<u8>();
                    let len20 = vec20.len();
                    ::core::mem::forget(vec20);
                    *ptr19.add(12).cast::<usize>() = len20;
                    *ptr19.add(8).cast::<*mut u8>() = ptr20.cast_mut();
                }
            }
        }
    };
    ptr19
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_handle_queue_message<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {}
        _ => {
            let l1 = i32::from(*arg0.add(4).cast::<u8>());
            match l1 {
                _ => {
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
            }
        }
    }
}
pub trait Guest {
    fn handle_queue_message(message: Message) -> Result<MessageAction, Error>;
}
#[doc(hidden)]
macro_rules! __export_world_spin_sqs_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "handle-queue-message"] unsafe extern "C" fn
        export_handle_queue_message(arg0 : i32, arg1 : * mut u8, arg2 : usize, arg3 : *
        mut u8, arg4 : usize, arg5 : i32, arg6 : * mut u8, arg7 : usize,) -> * mut u8 {
        $($path_to_types)*:: _export_handle_queue_message_cabi::<$ty > (arg0, arg1, arg2,
        arg3, arg4, arg5, arg6, arg7) } #[export_name = "cabi_post_handle-queue-message"]
        unsafe extern "C" fn _post_return_handle_queue_message(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_handle_queue_message::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_spin_sqs_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 16]);
#[allow(dead_code)]
pub mod fermyon {
    #[allow(dead_code)]
    pub mod spin_sqs {
        #[allow(dead_code, clippy::all)]
        pub mod sqs_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Clone)]
            pub enum MessageAttributeValue {
                Str(_rt::String),
                /// TODO: parse for the number case?
                Binary(_rt::Vec<u8>),
            }
            impl ::core::fmt::Debug for MessageAttributeValue {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        MessageAttributeValue::Str(e) => {
                            f.debug_tuple("MessageAttributeValue::Str").field(e).finish()
                        }
                        MessageAttributeValue::Binary(e) => {
                            f.debug_tuple("MessageAttributeValue::Binary")
                                .field(e)
                                .finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub struct MessageAttribute {
                pub name: _rt::String,
                pub data_type: Option<_rt::String>,
                pub value: MessageAttributeValue,
            }
            impl ::core::fmt::Debug for MessageAttribute {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("MessageAttribute")
                        .field("name", &self.name)
                        .field("data-type", &self.data_type)
                        .field("value", &self.value)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct Message {
                pub id: Option<_rt::String>,
                /// TODO: built-in attributes?  E.g. timestamps
                pub message_attributes: _rt::Vec<MessageAttribute>,
                pub body: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for Message {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Message")
                        .field("id", &self.id)
                        .field("message-attributes", &self.message_attributes)
                        .field("body", &self.body)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum Error {
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        Error::Other(e) => {
                            f.debug_tuple("Error::Other").field(e).finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for Error {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for Error {}
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum MessageAction {
                Delete,
                Leave,
            }
            impl ::core::fmt::Debug for MessageAction {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        MessageAction::Delete => {
                            f.debug_tuple("MessageAction::Delete").finish()
                        }
                        MessageAction::Leave => {
                            f.debug_tuple("MessageAction::Leave").finish()
                        }
                    }
                }
            }
            impl MessageAction {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> MessageAction {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => MessageAction::Delete,
                        1 => MessageAction::Leave,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_spin_sqs_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_spin_sqs_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_spin_sqs_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:fermyon:spin-sqs@2.0.0:spin-sqs:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 552] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa9\x03\x01A\x02\x01\
A\x0b\x01B\x0d\x01p}\x01q\x02\x03str\x01s\0\x06binary\x01\0\0\x04\0\x17message-a\
ttribute-value\x03\0\x01\x01ks\x01r\x03\x04names\x09data-type\x03\x05value\x02\x04\
\0\x11message-attribute\x03\0\x04\x01p\x05\x01r\x03\x02id\x03\x12message-attribu\
tes\x06\x04body\x03\x04\0\x07message\x03\0\x07\x01q\x01\x05other\x01s\0\x04\0\x05\
error\x03\0\x09\x01m\x02\x06delete\x05leave\x04\0\x0emessage-action\x03\0\x0b\x03\
\0\x20fermyon:spin-sqs/sqs-types@2.0.0\x05\0\x02\x03\0\0\x07message\x03\0\x07mes\
sage\x03\0\x01\x02\x03\0\0\x0emessage-action\x03\0\x0emessage-action\x03\0\x03\x02\
\x03\0\0\x05error\x03\0\x05error\x03\0\x05\x01j\x01\x04\x01\x06\x01@\x01\x07mess\
age\x02\0\x07\x04\0\x14handle-queue-message\x01\x08\x04\0\x1ffermyon:spin-sqs/sp\
in-sqs@2.0.0\x04\0\x0b\x0e\x01\0\x08spin-sqs\x03\0\0\0G\x09producers\x01\x0cproc\
essed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
