cmake_minimum_required(VERSION 3.2)


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
    message(STATUS "Checking compiler flags for C++11 support.")
    # Set C++11 support flags for various compilers
    include(CheckCXXCompilerFlag)
    check_cxx_compiler_flag("-std=c++11" COMPILER_SUPPORTS_CXX11)
    check_cxx_compiler_flag("-std=c++0x" COMPILER_SUPPORTS_CXX0X)
    if(COMPILER_SUPPORTS_CXX11)
        message(STATUS "C++11 is supported.")
        if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11 -stdlib=libc++")
        else()
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
        endif()
    elseif(COMPILER_SUPPORTS_CXX0X)
        message(STATUS "C++0x is supported.")
        if(${CMAKE_SYSTEM_NAME} MATCHES "Darwin")
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x -stdlib=libc++")
        else()
            set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++0x")
        endif()
    else()
        message(STATUS "The compiler ${CMAKE_CXX_COMPILER} has no C++11 support. Please use a different C++ compiler.")
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
	src/Managers/ActionManager.h
	src/Managers/CommitManager.h
	src/Managers/FileManager.h
	src/Managers/TokenManager.h
	src/Managers/UserManager.h
	src/Models/API.h
	src/Models/Flags.h
	src/Models/IcarusAction.h
	src/Models/Song.h
	src/Models/Token.h
	src/Models/UploadForm.h
	src/Models/User.h
	src/Parsers/APIParser.h
	src/Syncers/Delete.h
	src/Syncers/Download.h
	src/Syncers/SyncerBase.h
	src/Syncers/Upload.h
	src/Utilities/Conversions.h
)


hunter_add_package(nlohmann_json)
find_package(nlohmann_json CONFIG REQUIRED)

hunter_add_package(cpr)
find_package(cpr CONFIG REQUIRED)


add_executable(icd ${SOURCES} ${HEADERS})
target_link_libraries(icd PUBLIC nlohmann_json::nlohmann_json cpr::cpr)
include_directories(src/)
