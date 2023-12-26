    /// Register a named transformer.
    #[napi(ts_args_type = "name: string, callback: (input: any, object?: any, ctx?: any) => any | Promise<any>")]
    pub fn transform(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value, js_object.into_unknown(), js_ctx.into_unknown()])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.transform(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
            result.to_teo_value().await
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Register a named validator.
    #[napi(ts_args_type = "name: string, callback: (input: any, object?: any, ctx?: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn validate(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_object = js_value.coerce_to_object()?;
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value_object, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.validate(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
            let teo_value = result.to_teo_value().await;
            match teo_value {
                TeoValue::String(s) => {
                    ValidateResult::Validity(Validity::Invalid(s.to_owned()))
                },
                TeoValue::Bool(b) => if b {
                    ValidateResult::Validity(Validity::Valid)
                } else {
                    ValidateResult::Validity(Validity::Invalid("value is invalid".to_owned()))
                },
                _ => ValidateResult::Validity(Validity::Valid)
            }
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Register a named callback.
    #[napi(ts_args_type = "name: string, callback: (input: any, object?: any, ctx?: any) => void | Promise<void>")]
    pub fn callback(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoObject, UserCtx)>| {
            let js_value = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_object = js_value.coerce_to_object()?;
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.1.clone())?;
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.2.clone())?;
            Ok(vec![js_value_object, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.callback(Box::leak(Box::new(name)).as_str(), |value: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((value, object, ctx)).await.unwrap();
            let _teo_value = result.to_teo_value().await;
        }).into_nodejs_result()?;
        Ok(())
    }

    #[napi(js_name = "compare<T>", ts_args_type = "name: string, callback: (oldValue: T, newValue: T, object?: any, ctx?: any) => boolean | string | undefined | null | Promise<boolean | string | undefined | null>")]
    pub fn compare(&self, name: String, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<(TeoValue, TeoValue, TeoObject, UserCtx), ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx: ThreadSafeCallContext<(TeoValue, TeoValue, TeoObject, UserCtx)>| {
            let js_value_old = teo_value_to_js_unknown(&ctx.value.0, &ctx.env);
            let js_value_new = teo_value_to_js_unknown(&ctx.value.1, &ctx.env);
            let js_object = js_object_from_teo_object(ctx.env, ctx.value.2.clone())?.into_unknown();
            let js_ctx = js_user_ctx_from_user_ctx(ctx.env, ctx.value.3.clone())?.into_unknown();
            Ok(vec![js_value_old, js_value_new, js_object, js_ctx])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.compare(Box::leak(Box::new(name)).as_str(), |old: TeoValue, new: TeoValue, object: TeoObject, ctx: UserCtx| async {
            let result: WrappedTeoValue = tsfn_cloned.call_async((old, new, object, ctx)).await.unwrap();
            let teo_value = result.to_teo_value().await;
            match teo_value {
                TeoValue::String(s) => {
                    ValidateResult::Validity(Validity::Invalid(s.to_owned()))
                },
                TeoValue::Bool(b) => if b {
                    ValidateResult::Validity(Validity::Valid)
                } else {
                    ValidateResult::Validity(Validity::Invalid("value is invalid".to_owned()))
                },
                _ => ValidateResult::Validity(Validity::Valid)
            }
        }).into_nodejs_result()?;
        Ok(())
    }

    /// Run before server is started.
    #[napi(ts_args_type = "callback: () => void | Promise<void>")]
    pub fn setup(&self, callback: JsFunction) -> Result<()> {
        let tsfn: ThreadsafeFunction<i32, ErrorStrategy::Fatal> = callback.create_threadsafe_function(0, |ctx| {
            let undefined = ctx.env.get_undefined()?;
            Ok(vec![undefined])
        })?;
        let tsfn_cloned = Box::leak(Box::new(tsfn));
        self.teo_app.setup(|| async {
            let _: Result<TeoUnused> = tsfn_cloned.call_async(0).await;
            Ok(())
        }).into_nodejs_result()?;
        Ok(())
    }