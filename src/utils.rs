use inkwell::{
    context::Context,
    types::{BasicMetadataTypeEnum, BasicTypeEnum, FunctionType},
};

use crate::corelang::CoreType;

pub fn coretype_to_llvm<'ctx>(context: &'ctx Context, core_type: CoreType) -> BasicTypeEnum<'ctx> {
    match core_type {
        CoreType::I32 => context.i32_type().into(),
        CoreType::Tuple(elems) => {
            let types: Vec<_> = elems
                .into_iter()
                .map(|elem| coretype_to_llvm(context, elem))
                .collect();
            context.struct_type(&types, false).into()
        }
        _ => panic!("CoreTypeからBasicTypeEnumへ変換できません"),
    }
}

pub fn make_fn_type<'ctx>(
    ret_type: BasicTypeEnum<'ctx>,
    param_types: &[BasicMetadataTypeEnum<'ctx>],
) -> FunctionType<'ctx> {
    match ret_type {
        BasicTypeEnum::IntType(it) => it.fn_type(param_types, false),
        BasicTypeEnum::StructType(st) => st.fn_type(param_types, false),
        _ => panic!("(´・ω・｀)"),
    }
}
