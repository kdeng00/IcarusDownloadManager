cmake_minimum_required(VERSION 3.23.3)

set(VCPKG_ROOT_PATH ${CMAKE_CURRENT_SOURCE_DIR}/icarus/3rdparty/vcpkg)

message("The vcpkg root path ${VCPKG_ROOT_PATH}")

if(NOT DEFINED CMAKE_TOOLCHAIN_FILE)
    set(TOOLCHAIN_PATH ${VCPKG_ROOT_PATH}/scripts/buildsystems/vcpkg.cmake)

    set(CMAKE_TOOLCHAIN_FILE
        "${TOOLCHAIN_PATH}"
        CACHE STRING "Vcpkg toolchain file")

    message("Using default toolchain file")
endif()


set(SOFTWARE_DESCRIPTION
    "A music streaming Web API that provides a way for users to stream their personal music collection")
set(SOFTWARE_VERSION
    "0.3.0")


project(IcarusDownloadManager VERSION ${SOFTWARE_VERSION} DESCRIPTION ${SOFTWARE_DESCRIPTION} LANGUAGES CXX)



message(STATUS "Checking compiler flags for C++20 support.")
# Set C++20 support flags for various compilers
include(CheckCXXCompilerFlag)

if(WIN32)
  message("Windows environment")

  if (MSVC_VERSION GREATER_EQUAL "1900")
    message("Visual Studio version is at least ${vs_ver}")

    include(CheckCXXCompilerFlag)
    CHECK_CXX_COMPILER_FLAG("/std:c++20" _cpp_latest_flag_supported)
    if (_cpp_latest_flag_supported)
        add_compile_options("/std:c++20")
    endif()

    set(CMAKE_CXX_STANDARD 20)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
  endif()
else()
    check_cxx_compiler_flag("-std=c++20" COMPILER_SUPPORTS_CXX20)
    check_cxx_compiler_flag("-std=c++0x" COMPILER_SUPPORTS_CXX0X)

    if(COMPILER_SUPPORTS_CXX20)
        message(STATUS "C++20 is supported.")

        set(CMAKE_CXX_STANDARD 20)
        set(CMAKE_CXX_STANDARD_REQUIRED ON)

        if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20 -stdlib=libc++")
        else()
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")
        endif()
    elseif(COMPILER_SUPPORTS_CXX0X)
        message(STATUS "C++0x is supported.")
        if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x -stdlib=libc++")
        else()
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x")
        endif()
    else()
        message(STATUS "The compiler ${CMAKE_CXX_COMPILER} has no C++20 support. Please use a different C++ compiler.")
    endif()
endif()


set(SOURCES
    src/Main.cpp
    src/Managers/ActionManager.cpp
    src/Managers/CommitManager.cpp
    src/Managers/FileManager.cpp
    src/Managers/TokenManager.cpp
    src/Managers/UserManager.cpp
    src/Models/Song.cpp
    src/Parsers/APIParser.cpp
    src/Syncers/Delete.cpp
    src/Syncers/Download.cpp
    src/Syncers/RetrieveRecords.cpp
    src/Syncers/Upload.cpp
    src/Utilities/Conversions.cpp
)

set(IDM_INCLUDE_DIR
    "${CMAKE_CURRENT_SOURCE_DIR}/include")

# conan
# set(CONAN_BUILDINFO
# ${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)

# message("conan build info ${CONAN_BUILDINFO}")
# include(${CONAN_BUILDINFO})
# conan_basic_setup()

# include_directories(${CPR_INCLUDE_DIRS} ${IDM_INCLUDE_DIR})

set(USE_SYSTEM_CURL OFF)
set(BUILD_CPR_TESTS OFF)


add_executable(icd ${SOURCES})
# target_link_libraries(icd PUBLIC ${CONAN_LIBS})
