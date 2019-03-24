cmake_minimum_required(VERSION 2.8.7)
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

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed
set(CMAKE_AUTOMOC ON)
# Create code from a list of Qt designer ui files
set(CMAKE_AUTOUIC ON)
		

find_package(Qt5Widgets CONFIG REQUIRED)

add_subdirectory(cpr)

set(SOURCES
	src/Main.cpp
	src/Managers/FileManager.cpp
	src/Syncers/Download.cpp
	src/Syncers/Upload.cpp
	src/UI/MainWindow.cpp	
)
set(HEADERS
	src/Managers/FileManager.h
	src/Models/Song.h
	src/Models/UploadForm.h
	src/Syncers/Download.h
	src/Syncers/Upload.h
	src/UI/CommonWindow.h
	src/UI/MainWindow.h
)

add_executable(icd ${SOURCES} ${HEADERS})
			   
target_link_libraries(icd ${CPR_LIBRARIES} Qt5::Widgets)
include_directories(${CPR_INCLUDE_DIRS} ${JSON_INCLUDE_DIRS} src/)
