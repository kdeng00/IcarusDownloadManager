cmake_minimum_required(VERSION 3.10)


include("cmake/HunterGate.cmake")
HunterGate(
    URL "https://github.com/ruslo/hunter/archive/v0.23.184.tar.gz"
    SHA1 "fe3fb05d51c21499c7eebbe7d0e102742a54a9cd"
)

project(IcarusDownloadManager)



if(NOT ${CMAKE_VERSION} LESS 3.2)
    set(CMAKE_CXX_STANDARD 11)
    set(CMAKE_CXX_STANDARD_REQUIRED ON)
else()
    message(STATUS "Checking compiler flags for C++117support.")
    # Set C++17 support flags for various compilers
    include(CheckCXXCompilerFlag)
    check_cxx_compiler_flag("-std=c++17" COMPILER_SUPPORTS_CXX17)
    check_cxx_compiler_flag("-std=c++0x" COMPILER_SUPPORTS_CXX0X)
    if(COMPILER_SUPPORTS_CXX17)
        message(STATUS "C++17 is supported.")
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
endif()


set(SOURCES
	src/Main.cpp
	src/Managers/ActionManager.cpp
	src/Managers/CommitManager.cpp
	src/Managers/FileManager.cpp
	src/Managers/TokenManager.cpp
	src/Managers/UserManager.cpp
	src/Parsers/APIParser.cpp
	src/Syncers/Delete.cpp
	src/Syncers/Download.cpp
	src/Syncers/Upload.cpp
	src/Utilities/Conversions.cpp
)
set(HEADERS
	include/Managers/ActionManager.h
	include/Managers/CommitManager.h
	include/Managers/FileManager.h
	include/Managers/TokenManager.h
	include/Managers/UserManager.h
	include/Models/API.h
	include/Models/Flags.h
	include/Models/IcarusAction.h
	include/Models/Song.h
	include/Models/Token.h
	include/Models/UploadForm.h
	include/Models/User.h
	include/Parsers/APIParser.h
	include/Syncers/Delete.h
	include/Syncers/Download.h
	include/Syncers/SyncerBase.h
	include/Syncers/Upload.h
	include/Utilities/Conversions.h
)


hunter_add_package(nlohmann_json)
find_package(nlohmann_json CONFIG REQUIRED)

hunter_add_package(cpr)
find_package(cpr CONFIG REQUIRED)


add_executable(icd ${SOURCES} ${HEADERS})
target_link_libraries(icd PUBLIC nlohmann_json::nlohmann_json cpr::cpr)
include_directories(include/)
