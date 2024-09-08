use axum::Json;

use serde::Serialize;

#[derive(Serialize)]
pub struct Evaluation {
    result: String,
    logs: Vec<String>,
}

pub async fn post_run_js(body: String) -> Json<Evaluation> {
    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context: v8::Local<'_, v8::Context> = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);

    let log_callback = {
        move |scope: &mut v8::HandleScope,
              args: v8::FunctionCallbackArguments,
              mut rv: v8::ReturnValue| {
            let arg = args.get(0);

            // get global scope

            let global = scope.get_current_context().global(scope);

            // get logs array
            let logs_name = v8::String::new(scope, "logs").unwrap();
            let logs = global
                .get(scope, logs_name.into())
                .unwrap()
                .to_string(scope)
                .unwrap();

            // push log to logs array
            let log = arg.to_string(scope).unwrap();

            let logs = logs.to_rust_string_lossy(scope).to_owned();
            let log = log.to_rust_string_lossy(scope).to_owned();

            let logs = format!("{}\n{}", logs, log);

            let logs = v8::String::new(scope, &logs).unwrap();
            global.set(scope, logs_name.into(), logs.into());

            rv.set(v8::undefined(scope).into());
        }
    };

    // https://github.com/matejkoncal/v8-experiments/blob/master/src/main.rs
    let log_fn = v8::Function::new(scope, log_callback).unwrap();
    let name = v8::String::new(scope, "log").unwrap();

    let logs_name = v8::String::new(scope, "logs").unwrap();
    let logs = v8::String::new(scope, "").unwrap();

    let global = context.global(scope);
    global.set(scope, name.into(), log_fn.into());
    global.set(scope, logs_name.into(), logs.into());

    let code = v8::String::new(scope, &body).unwrap();

    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap();
    let result: v8::Local<'_, v8::String> = result.to_string(scope).unwrap();

    let logs: v8::Local<'_, v8::String> = global
        .get(scope, logs_name.into())
        .unwrap()
        .to_string(scope)
        .unwrap();

    let logs = logs.to_rust_string_lossy(scope).to_owned();

    Json(Evaluation {
        result: result.to_rust_string_lossy(scope).to_owned(),
        logs: logs
            .lines()
            .map(|s| s.to_owned())
            .filter(|s| s.is_empty() == false)
            .collect(),
    })
}
