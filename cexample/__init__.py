from .cexample import lib, ffi

def _handle_rust_str(result) -> str:
    """
    This method decodes bytes to UTF-8 and makes a new python string object. 
    It then frees the bytes that Rust allocated correctly.
    """
    try:
        txt = ffi.cast("char*", result)
        txt = ffi.string(txt).decode("utf-8")
        return txt
    finally:
        lib.free_str(result)

def _handle_c_result(c_result):
    """
    This handles the logical-OR struct of the CDataset { error_message, success } 
    where both the wrapper and the error_message will be freed by the end of this function.
    The success pointer is returned or an error is raised!
    """
    if c_result == ffi.NULL:
        raise ValueError("CResult should not be NULL")
    error = None
    success = None
    if c_result.error_message != ffi.NULL:
        error = _handle_rust_str(c_result.error_message)
    if c_result.success != ffi.NULL:
        success = c_result.success
    lib.free_c_result(c_result)
    if error is not None:
        raise Exception(error)
    return success

def operate(op: str, x: int, y: int) -> int:
    if len(op) != 1:
        raise ValueError("{} should be a character.".format(op))
    int_ptr = ffi.cast("int32_t*", _handle_c_result(lib.c_operate(op, x, y)))
    num = int_ptr[0]
    lib.free_i32(int_ptr)
    return num


