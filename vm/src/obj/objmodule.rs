use crate::obj::objtype::PyClassRef;
use crate::pyobject::{PyContext, PyRef, PyResult, PyValue};
use crate::vm::VirtualMachine;

#[derive(Debug)]
pub struct PyModule {
    pub name: String,
}
pub type PyModuleRef = PyRef<PyModule>;

impl PyValue for PyModule {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.module_type()
    }
}

impl PyModuleRef {
    fn dir(self: PyModuleRef, vm: &VirtualMachine) -> PyResult {
        if let Some(dict) = &self.into_object().dict {
            let keys = dict.into_iter().map(|(k, _v)| k.clone()).collect();
            Ok(vm.ctx.new_list(keys))
        } else {
            panic!("Modules should definitely have a dict.");
        }
    }

    fn name(self: PyModuleRef, _vm: &VirtualMachine) -> String {
        self.name.clone()
    }
}

pub fn init(context: &PyContext) {
    extend_class!(&context, &context.module_type, {
        "__dir__" => context.new_rustfunc(PyModuleRef::dir),
        "__name__" => context.new_rustfunc(PyModuleRef::name)
    });
}
