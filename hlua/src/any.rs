use AsLua;
use AsMutLua;

use Push;
use PushGuard;
use LuaRead;

/// Represents any value that can be stored by Lua
#[derive(Clone, Debug, PartialEq)]
pub enum AnyLuaValue {
    LuaString(String),
    LuaNumber(f64),
    LuaBoolean(bool),
    LuaArray(Vec<(AnyLuaValue, AnyLuaValue)>),

    /// The "Other" element is (hopefully) temporary and will be replaced by "Function" and "Userdata".
    /// A panic! will trigger if you try to push a Other.
    LuaOther
}

impl<L> Push<L> for AnyLuaValue where L: AsMutLua {
    fn push_to_lua(self, lua: L) -> PushGuard<L> {
        match self {
            AnyLuaValue::LuaString(val) => val.push_to_lua(lua),
            AnyLuaValue::LuaNumber(val) => val.push_to_lua(lua),
            AnyLuaValue::LuaBoolean(val) => val.push_to_lua(lua),
            AnyLuaValue::LuaArray(_val) => unimplemented!(),//val.push_to_lua(lua),   // FIXME: reached recursion limit during monomorphization
            AnyLuaValue::LuaOther => panic!("can't push a AnyLuaValue of type Other")
        }
    }
}

impl<L> LuaRead<L> for AnyLuaValue where L: AsLua {
    fn lua_read_at_position(lua: L, index: i32) -> Option<AnyLuaValue> {
        None
            .or_else(|| LuaRead::lua_read_at_position(&lua, index).map(|v| AnyLuaValue::LuaNumber(v)))
            .or_else(|| LuaRead::lua_read_at_position(&lua, index).map(|v| AnyLuaValue::LuaBoolean(v)))
            .or_else(|| LuaRead::lua_read_at_position(&lua, index).map(|v| AnyLuaValue::LuaString(v)))
            //.or_else(|| LuaRead::lua_read_at_position(&lua, index).map(|v| LuaArray(v)))
            .or_else(|| Some(AnyLuaValue::LuaOther))
    }
}
