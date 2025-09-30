% demonstration of MATLAB & Rust interop
clear; clc; close all;

matrix = [1 10 100 1000];
disp("input matrix:");
disp(matrix);

% call the foreign Rust function via C mex
ffi_modified = processmatrix(matrix);
disp("Rust processed:");
disp(ffi_modified);

function output = processmatrix(input)
    lib_name = 'srmgrain';
    lib_path = '../core/target/release/srmgrain';
    header_path = '../core/bindings.h';
    
    % load .dll library 
    if ~libisloaded(lib_name)
        loadlibrary(lib_path, header_path);
    end
    
    ptr = calllib(lib_name, 'process_matrix_ffi', input, numel(input));
    struct_val = ptr.Value;
    data_ptr = struct_val.data;
    len = struct_val.len;
    setdatatype(data_ptr, 'doublePtr', 1, len);

    % assign function output to returned value
    output = data_ptr.Value(1:len);

    % free cloned matrix
    calllib(lib_name, 'free_matrix_ffi', ptr);
end
