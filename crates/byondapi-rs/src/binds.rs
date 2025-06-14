use std::io::Write;

pub struct Bind {
    pub proc_path: &'static str,
    pub func_name: &'static str,
    pub func_arguments: &'static str,
    pub docs: &'static str,
    pub function_type: FunctionType,
}
pub enum FunctionType {
    Macro,
    Variadic,
    Default,
}

inventory::collect!(Bind);

pub fn generate_bindings(libname: &str) {
    _ = std::fs::remove_file("./bindings.dm");
    let mut file = std::fs::File::create("./bindings.dm").unwrap();
    let libname_upper = libname.to_uppercase();

    let byond_compat_crash = {
        if cfg!(feature = "byond-515-1621") {
            "
/proc/byondapi_stack_trace(msg)
	CRASH(msg)"
        } else {
            ""
        }
    };
    file.write_fmt(format_args!(
        "//THIS FILE IS AUTOMATICALLY GENERATED BY {libname_upper}, PLEASE DO NOT TOUCH IT
//PROC DEFINITIONS MAY MOVE AROUND, THIS IS NORMAL

/* This comment bypasses grep checks */ /var/__{libname}

/proc/__detect_{libname}()
	if (world.system_type == UNIX)
		return __{libname} = \"lib{libname}\"
	else
		return __{libname} = \"{libname}\"

#define {libname_upper} (__{libname} || __detect_{libname}())
{byond_compat_crash}
"
    ))
    .unwrap();
    for thing in inventory::iter::<Bind> {
        let path = thing.proc_path;
        let docs = thing.docs;
        let func_name = thing.func_name;
        let func_arguments = thing.func_arguments;
        let func_arguments_srcless = func_arguments
            .to_owned()
            .replace("src, ", "")
            .replace("src", "");
        match thing.function_type {
            FunctionType::Macro => {
                let func_name_libname = func_name.replace("_ffi", &format!("_{libname}"));
                if cfg!(feature = "byond-515-1621") {
                    file.write_fmt(format_args!(
                r#"{docs}#define {func_name_libname}({func_arguments}) call_ext({libname_upper}, "byond:{func_name}")({func_arguments})

"#
            ))
            .unwrap();
                } else {
                    file.write_fmt(format_args!(
                r#"var/static/__loaded_{libname}_{func_name} = load_ext({libname_upper}, "byond:{func_name}")
{docs}#define {func_name_libname}({func_arguments}) call_ext(__loaded_{libname}_{func_name})({func_arguments})

"#
            ))
            .unwrap();
                }
            }
            FunctionType::Variadic => {
                //can't directly modify args, fuck you byond
                if cfg!(feature = "byond-515-1621") {
                    file.write_fmt(format_args!(
                        r#"{docs}{path}(...)
	var/list/args_copy = args.Copy()
	args_copy.Insert(1, src)
	return call_ext({libname_upper}, "byond:{func_name}")(arglist(args_copy))

"#
                    ))
                    .unwrap()
                } else {
                    file.write_fmt(format_args!(
                        r#"{docs}{path}(...)
	var/list/args_copy = args.Copy()
	args_copy.Insert(1, src)
	var/static/loaded = load_ext({libname_upper}, "byond:{func_name}")
	return call_ext(loaded)(arglist(args_copy))

"#
                    ))
                    .unwrap()
                }
            }
            FunctionType::Default => {
                if cfg!(feature = "byond-515-1621") {
                    file.write_fmt(format_args!(
                        r#"{docs}{path}({func_arguments_srcless})
	return call_ext({libname_upper}, "byond:{func_name}")({func_arguments})

"#
                    ))
                    .unwrap()
                } else {
                    file.write_fmt(format_args!(
                        r#"{docs}{path}({func_arguments_srcless})
	var/static/loaded = load_ext({libname_upper}, "byond:{func_name}")
	return call_ext(loaded)({func_arguments})

"#
                    ))
                    .unwrap()
                }
            }
        }
    }
}
