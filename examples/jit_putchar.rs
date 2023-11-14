use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = Context::create();
    let module = context.create_module("めいん");
    let builder = context.create_builder();
    let i32_type = context.i32_type();

    // declare i32 @putchar(i32)
    let putchar_type = i32_type.fn_type(&[i32_type.into()], false);
    module.add_function("putchar", putchar_type, None);

    // define i32 @main() {
    let main_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", main_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block);

    // call i32 @putchar(i32 72)
    let fun_putchar = module.get_function("putchar").unwrap();
    let build_putchar_call = |c: char| {
        builder.build_call(
            fun_putchar,
            &[i32_type.const_int(c as u64, false).into()],
            "ぷ",
        )
    };
    build_putchar_call('H')?;
    build_putchar_call('i')?;
    build_putchar_call('!')?;
    build_putchar_call('\n')?;

    // ret i32 0
    builder.build_return(Some(&i32_type.const_int(0, false)))?;

    module.print_to_stderr();

    // JITコンパイル
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;

    unsafe {
        // JITコンパイルした関数を取得
        let main_function: unsafe extern "C" fn() =
            execution_engine.get_function("main").unwrap().as_raw();

        // 関数を実行
        main_function();
    }

    Ok(())
}
