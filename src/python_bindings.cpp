#include "Python.h"
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include "numpy/arrayobject.h"
#include "flategy.hpp"

namespace {

    PyObject* hello_world(PyObject*, PyObject*) {
        return PyUnicode_FromString("hello from C++");
    }

    PyMethodDef methods[] = {
        { "hello_world", hello_world, METH_VARARGS,
          "Say hello from native code" },
        { nullptr, nullptr, 0, nullptr }
    };

    struct PyModuleDef module = {
        PyModuleDef_HEAD_INIT,
        "libflategy",
        flategy::Docstring,
        -1,
        methods,
        nullptr,
        nullptr,
        nullptr,
        nullptr
    };


} // namespace (anonymous)

PyMODINIT_FUNC PyInit_libflategy() {
    import_array();
    return PyModule_Create(&module);
}
