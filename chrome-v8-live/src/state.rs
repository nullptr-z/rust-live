use std::{cell::RefCell, rc::Rc};
use v8::{HandleScope, Isolate};

type GlobalContext = v8::Global<v8::Context>;
type JsRuntimeStateRef = Rc<RefCell<JsRuntimeState>>;

pub struct JsRuntimeState {
    context: Option<GlobalContext>,
}

// 用于在全局Global注册属性，而且同一线程下可以安全的共享并且修改
impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate) -> JsRuntimeStateRef {
        let context = {
            // create handel scope
            let handle_scope = &mut HandleScope::new(isolate);
            // create context
            let context = v8::Context::new(handle_scope);
            // enter context scope
            // let context_scope = &mut v8::ContextScope::new(handle_scope, context);
            // 在全局创建一个Context
            v8::Global::new(handle_scope, context)
        };

        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context),
        }))
    }

    pub fn get_context(isolate: &mut Isolate) -> GlobalContext {
        // 为什么这样子就能拿到context？因为在全局注册了一个属性，这个属性是一个JsRuntimeStateRef
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();

        let ctx = &state.borrow().context; // &
        ctx.as_ref().unwrap().clone()
    }

    pub fn drop_context(isolate: &mut Isolate) {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();
        state.borrow_mut().context.take();
    }
}
