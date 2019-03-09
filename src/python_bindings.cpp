#include "Python.h"
#include "flategy.hpp"

namespace {

    // Core data

    template<class T>
    const T* to_cpp(PyObject* bytes) {
        return flatbuffers::GetRoot<T>(PyBytes_AsString(bytes));
    }

    template<class T>
    PyObject* to_python(const T*

    // Interface management

    flategy::Game* get_game(PyObject* obj) {
        return reinterpret_cast<flategy::Game*>(PyCapsule_GetPointer(obj, nullptr));
    }

    void delete_game(PyObject* obj) {
        delete get_game(obj);
    }

    PyObject* create_game(PyObject* options_buffer) {
        return PyCapsule_New(flategy::create_game(...), nullptr, delete_game);
    }

    // Module

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
        "Core native logic for the RTS game Flategy",
        -1,
        methods,
        nullptr,
        nullptr,
        nullptr,
        nullptr
    };

} // namespace (anonymous)

PyMODINIT_FUNC PyInit_libflategy() {
    return PyModule_Create(&module);
}
