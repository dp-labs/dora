cmake_minimum_required(VERSION 3.10...3.27)

if(NOT EXISTS ${CMAKE_CURRENT_SOURCE_DIR}/evmc/.git)
    message(FATAL_ERROR "EVMC dependency not installed, execute:\n  git clone https://github.com/ethereum/evmc")
endif()

project(dora-c-example LANGUAGES CXX C)
set(PROJECT_VERSION 0.13.0)

string(REGEX MATCH "([0-9]+)\\.([0-9]+)" _ ${PROJECT_VERSION})
set(PROJECT_VERSION_MAJOR ${CMAKE_MATCH_1})
set(PROJECT_VERSION_MINOR ${CMAKE_MATCH_2})

set(PROJECT_SOVERSION ${PROJECT_VERSION_MAJOR})
if(PROJECT_VERSION_MAJOR EQUAL 0)
    set(PROJECT_SOVERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR})
endif()

if(COMMAND block)
    block()
        set(CMAKE_C_CLANG_TIDY "")
        set(CMAKE_CXX_CLANG_TIDY "")
        add_subdirectory(evmc)
    endblock()
else()
    add_subdirectory(evmc)
endif()

set(CMAKE_C_VISIBILITY_PRESET hidden)
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_VISIBILITY_INLINES_HIDDEN YES)

include(GNUInstallDirs)

set(include_dir ${CMAKE_CURRENT_SOURCE_DIR}/include)

set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++17")

# INSTALL
# Get cargo target dir using cargo locate-project
# We should do this because the target dir is different for development and release
execute_process(COMMAND cargo locate-project --workspace --message-format plain
    OUTPUT_VARIABLE CARGO_TARGET_DIR
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR})
string(REGEX REPLACE "/Cargo.toml\n$" "/target" CARGO_TARGET_DIR "${CARGO_TARGET_DIR}")
set(CARGO_MANIFEST ${PROJECT_SOURCE_DIR}/Cargo.toml)
set(DORA_C_HEADER_FILE ${PROJECT_SOURCE_DIR}/bindings/c/include/dora.h)
set(DORA_C_LIB ${CARGO_TARGET_DIR}/release/${CMAKE_STATIC_LIBRARY_PREFIX}dora_c${CMAKE_STATIC_LIBRARY_SUFFIX})
set(DORA_C_INCLUDE_DIR ${PROJECT_SOURCE_DIR} ../include)

execute_process(COMMAND llvm-config --libdir
    OUTPUT_VARIABLE LLVM_LIBRARY_DIRS OUTPUT_STRIP_TRAILING_WHITESPACE)

add_custom_command(
    OUTPUT ${DORA_C_LIB}
    COMMAND cargo build --manifest-path ${CARGO_MANIFEST} --release
    USES_TERMINAL
    COMMENT "Running cargo build --release..."
)

add_executable(dora-c-example example.c example_host.cpp)
link_directories(${LLVM_LIBRARY_DIRS})
target_include_directories(dora-c-example PUBLIC ${DORA_C_INCLUDE_DIR})
target_link_libraries(dora-c-example PUBLIC ${DORA_C_LIB})
target_link_libraries(dora-c-example PUBLIC evmc)
target_link_libraries(dora-c-example PRIVATE ${LLVM_LIBS})
target_link_libraries(dora-c-example PRIVATE LLVM-19)
if(WIN32)
    target_link_libraries(dora-c-example PRIVATE userenv ws2_32 bcrypt)
endif()
if(APPLE)
    find_library(CORESERVICES_LIB CoreServices)
    target_link_libraries(dora-c-example PRIVATE ${CORESERVICES_LIB})
    target_link_libraries(dora-c-example PRIVATE "-framework CoreFoundation -framework Security")
endif()

# Platform-specific test configuration
if(WIN32)
    target_link_libraries(dora-c userenv ws2_32 bcrypt)
    set_target_properties(
        dora-c
        PROPERTIES
        MSVC_RUNTIME_LIBRARY "MultiThreadedDLL"
        RUNTIME_OUTPUT_DIRECTORY_DEBUG ${CMAKE_BINARY_DIR}
        RUNTIME_OUTPUT_DIRECTORY_RELEASE ${CMAKE_BINARY_DIR}
    )
endif()
