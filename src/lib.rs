use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

/// 为函数添加 tracing 功能的过程宏
///
/// # 参数
/// - `level`: 日志等级 (trace, debug, info, warn, error)，默认为 trace
/// - `skip`: 跳过的参数列表
/// - `force`: 是否强制在release模式下启用tracing，默认为false
///
/// # 示例
/// ```rust
/// #[tracing_fn]
/// fn example_fn(a: i32, b: String) -> i32 {
///     a + b.len() as i32
/// }
///
/// #[tracing_fn(level = "info")]
/// fn example_fn2(a: i32, b: String) -> i32 {
///     a + b.len() as i32
/// }
///
/// #[tracing_fn(skip = "b")]
/// fn example_fn3(a: i32, b: String) -> i32 {
///     a + b.len() as i32
/// }
///
/// // 强制在release模式下也启用tracing
/// #[tracing_fn(force = true)]
/// fn example_fn4(a: i32) -> i32 {
///     a * 2
/// }
/// ```
#[proc_macro_attribute]
pub fn tracing_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut level = "trace".to_string();
    let mut skip_args = Vec::new();
    let mut force = false;

    // 解析参数
    if !args.is_empty() {
        let args_string = args.to_string();
        // 解析参数字符串
        for arg in args_string.split(',') {
            let arg = arg.trim();
            if let Some(eq_pos) = arg.find('=') {
                let key = arg[..eq_pos].trim();
                let value = arg[eq_pos + 1..].trim();
                // 去掉引号
                let value = value.trim_matches(|c| c == '"' || c == '\'');

                match key {
                    "level" => level = value.to_string(),
                    "skip" => {
                        skip_args = value.split(',').map(|s| s.trim().to_string()).collect();
                    }
                    "force" => {
                        force = value == "true";
                    }
                    _ => {} // 忽略未知参数
                }
            }
        }
    }

    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_block = &input_fn.block;
    let fn_sig = &input_fn.sig;
    let fn_attrs = &input_fn.attrs;

    // 获取所有参数名
    let mut arg_names = Vec::new();
    let mut arg_values = Vec::new();
    for arg in &fn_sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                let arg_name = ident.ident.to_string();
                arg_names.push(arg_name.clone());
                if !skip_args.contains(&arg_name) {
                    let ident = &ident.ident;
                    arg_values.push(quote! {
                        format!("{}={:?}", #arg_name, #ident)
                    });
                } else {
                    arg_values.push(quote! {
                        format!("{}={}", #arg_name, "***")
                    });
                }
            }
        }
    }

    let level_ident = syn::Ident::new(&level.to_uppercase(), proc_macro2::Span::call_site());
    let fn_name_str = fn_name.to_string();

    // 根据force参数决定是否在release模式下强制启用
    let expanded = if force {
        // 如果force=true，则无论什么模式都启用tracing
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig {
                {
                    let __tracing_fn_args: Vec<String> = vec![#(#arg_values),*];
                    let __tracing_fn_args_str = if __tracing_fn_args.is_empty() {
                        "()".to_string()
                    } else {
                        __tracing_fn_args.join(", ")
                    };
                    tracing::event!(
                        tracing::Level::#level_ident,
                        ">>> [{}] #Args: {} --- {}:{}",
                        #fn_name_str,
                        __tracing_fn_args_str,
                        file!(),
                        line!()
                    );
                }

                let __tracing_fn_start = std::time::Instant::now();
                let __tracing_fn_result = (move || #fn_block )();
                let __tracing_fn_duration = __tracing_fn_start.elapsed();

                tracing::event!(
                    tracing::Level::#level_ident,
                    "<<< [{}] #Ret: {:?}, duration: {:?}",
                    #fn_name_str,
                    __tracing_fn_result,
                    __tracing_fn_duration
                );

                __tracing_fn_result
            }
        }
    } else {
        // 否则仅在debug模式下启用tracing
        quote! {
            #(#fn_attrs)*
            #fn_vis #fn_sig {
                #[cfg(debug_assertions)]
                {
                    let __tracing_fn_args: Vec<String> = vec![#(#arg_values),*];
                    let __tracing_fn_args_str = if __tracing_fn_args.is_empty() {
                        "()".to_string()
                    } else {
                        __tracing_fn_args.join(", ")
                    };
                    tracing::event!(
                        tracing::Level::#level_ident,
                        ">>> [{}] #Args: {} --- {}:{}",
                        #fn_name_str,
                        __tracing_fn_args_str,
                        file!(),
                        line!()
                    );
                }

                #[cfg(debug_assertions)]
                {
                    let __tracing_fn_start = std::time::Instant::now();
                    let __tracing_fn_result = (move || #fn_block )();
                    let __tracing_fn_duration = __tracing_fn_start.elapsed();

                    tracing::event!(
                        tracing::Level::#level_ident,
                        "<<< [{}] #Ret:  {:?}, duration: {:?}",
                        #fn_name_str,
                        __tracing_fn_result,
                        __tracing_fn_duration
                    );

                    __tracing_fn_result
                }

                // 在 Release 模式下直接执行原函数
                #[cfg(not(debug_assertions))]
                (move || #fn_block )()
            }
        }
    };

    TokenStream::from(expanded)
}
