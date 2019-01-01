/// Allows to easily bind rust functions to your Lua code
///
/// **THIS USES UNWRAPS INTERNALLY, SO IT MIGHT PANIC**
///
/// # Arguments
///
/// * `lua_inst` - Your lua instance, create with rlua::Lua::new()
/// * `to_call` - The rust function you want to call, the lua function will have the same name
/// * `argname` - Optional parameter for functions with parameters: Specifies the name of the arg
/// * `argtype` - Optional parameter for functions with parameters: Specifies the type of the arg
///
/// # Example
///
/// Rust setup:
///
/// ```rust
/// let lua_inst = rlua::Lua::new();
/// make_lua_fn!(lua_inst, print_hello);
/// make_lua_fn!(lua_inst, print_arg, (arg, u32));
/// make_lua_fn!(lua_inst, print_multi_args, (arg, u32), (arg2, f32));
/// make_lua_fn!(lua_inst, print_multi_args_names, (argNameCanBeAnything, u32), (arg2, f32));
/// ```
///
/// Lua usage:
///
/// ```lua
/// print_hello()
/// print_arg(12)
/// print_multi_args(12, 4.0)
/// ```
#[macro_export]
macro_rules! make_lua_fn {
    ($lua_inst:ident, $to_call: ident, $(
        ($argname: ident, $argtype: ty))
        ,
        *
        ) => {
        let tmp = $lua_inst
            .create_function(|_, ($($argname,)*): ($($argtype,)*)| {
                Ok($to_call($($argname,)*))
                })
            .unwrap();
        $lua_inst.globals().set(stringify!($to_call), tmp).unwrap();
    };

    ($lua_inst:ident, $to_call: ident) => {
        let tmp = $lua_inst
            .create_function(move |_, ()| Ok($to_call()))
            .unwrap();
        $lua_inst.globals().set(stringify!($to_call), tmp).unwrap();
    };
}

#[cfg(test)]
mod tests {
    use rlua::Lua;
    #[test]
    fn test_functions() {
        let lua = setup_lua();
        let lua_str = r#"
        print_hello()
        print(get_random_number())
        print_arg(14)
        print_multi_args(14, 4.4)
        "#;
        lua.eval::<_, ()>(lua_str, None).unwrap();
    }

    fn setup_lua() -> crate::Lua {
        let lua_inst = crate::Lua::new();
        make_lua_fn!(lua_inst, print_hello);
        make_lua_fn!(lua_inst, get_random_number);
        make_lua_fn!(lua_inst, print_arg, (arg, u32));
        make_lua_fn!(lua_inst, print_multi_args, (arg, u32), (arg2, f32));
        lua_inst
    }

    fn print_hello() {
        println!("Hello from Rust");
    }

    fn get_random_number() -> u32 {
        println!("Returning 4");
        4
    }

    fn print_arg(data: u32) {
        println!("Got single arg {}", data);
    }

    fn print_multi_args(data: u32, data2: f32) {
        println!("Got multiple args {} and {}", data, data2);
    }

}
