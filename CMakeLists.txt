cmake_minimum_required(VERSION 3.12)


project(IcarusDownloadManager)



message(STATUS "Checking compiler flags for C++17 support.")
# Set C++17 support flags for various compilers
include(CheckCXXCompilerFlag)

check_cxx_compiler_flag("-std=c++17" COMPILER_SUPPORTS_CXX17)
check_cxx_compiler_flag("-std=c++0x" COMPILER_SUPPORTS_CXX0X)

if(COMPILER_SUPPORTS_CXX17)
    message(STATUS "C++17 is supported.")

    set(CMAKE_CXX_STANDARD 17)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)

    if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17 -stdlib=libc++")
    else()
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++17")
    endif()
elseif(COMPILER_SUPPORTS_CXX0X)
    message(STATUS "C++0x is supported.")
    if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x -stdlib=libc++")
    else()
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x")
    endif()
else()
    message(STATUS "The compiler ${CMAKE_CXX_COMPILER} has no C++17 support. Please use a different C++ compiler.")
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
include(${CMAKE_BINARY_DIR}/conanbuildinfo.cmake)
conan_basic_setup()

include_directories(${CPR_INCLUDE_DIRS} ${IDM_INCLUDE_DIR})

set(USE_SYSTEM_CURL OFF)
set(BUILD_CPR_TESTS OFF)


add_executable(icd ${SOURCES})
target_link_libraries(icd PUBLIC ${CONAN_LIBS})
