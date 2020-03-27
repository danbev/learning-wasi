cmake_minimum_required(VERSION 3.5 FATAL_ERROR)

project(learning-wasi 
	VERSION 0.1 
	DESCRIPTION "Project for learning about WASI ")

set(LLVM_HOME $ENV{HOME}/work/wasm/wasi-sdk/download/wasi-sdk-8.0 CACHE STRING "llvm home")
set(LLVM_BIN ${LLVM_HOME}/bin CACHE STRING "llvm bin")
set(WASI_SYSROOT ${LLVM_HOME}/share/wasi-sysroot CACHE STRING "wasi-sysroot")
set(TRIPLE wasm32-wasi CACHE STRING "triple")

set(CMAKE_SYSTEM_NAME Wasm)
set(CMAKE_SYSTEM_VERSION 1)
set(CMAKE_SYSTEM_PROCESSOR wasm32)

set(CMAKE_C_COMPILER ${LLVM_BIN}/clang)
set(CMAKE_CXX_COMPILER ${LLVM_BIN}/clang++)

set(CMAKE_FIND_ROOT_PATH_MODE_LIBRARY ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_INCLUDE ONLY)
set(CMAKE_FIND_ROOT_PATH_MODE_PACKAGE ONLY)

set(CMAKE_SYSROOT ${WASI_SYSROOT} CACHE STRING "wasi-sysroot")
set(CMAKE_C_COMPILER_TARGET ${TRIPLE} CACHE STRING "c target")
set(CMAKE_CXX_COMPILER_TARGET ${TRIPLE} CACHE STRING "c++ target")
set(CMAKE_C_FLAGS "-v" CACHE STRING "cflags")
set(CMAKE_CXX_FLAGS "-v -std=c++11 -I${WASI_SYSROOT}/include/c++/v1" CACHE STRING "c++flags")
set(CMAKE_EXE_LINKER_FLAGS "-Wl,--no-threads" CACHE STRING "ld flags")

add_executable(first.wasm src/first.c)
add_executable(firstcxx.wasm src/first.cc)

message(STATUS "summary of build options:

    Package version: ${CMAKE_PROJECT_VERSION}
    Description:     ${CMAKE_PROJECT_DESCRIPTION}
    Install prefix:  ${CMAKE_INSTALL_PREFIX}
    LLVM home:       ${LLVM_HOME}
    Target tripple:  ${TRIPLE}
    Compiler:
      C compiler:     ${CMAKE_C_COMPILER}
      CXX compiler:   ${CMAKE_CXX_COMPILER}
      CFLAGS:         ${CMAKE_C_FLAGS_${_build_type}} ${CMAKE_C_FLAGS}
      sysroot:        ${WASI_SYSROOT}
")


