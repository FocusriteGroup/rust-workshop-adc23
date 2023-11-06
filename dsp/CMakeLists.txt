cmake_minimum_required(VERSION 3.21 FATAL_ERROR)

project(dsp
    VERSION     0.0.0
    LANGUAGES   CXX
)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR})
foreach(CFGNAME ${CMAKE_CONFIGURATION_TYPES})
    string(TOUPPER ${CFGNAME} CFGNAME)
    set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY_${CFGNAME} ${CMAKE_BINARY_DIR})
endforeach()

add_library(${PROJECT_NAME}
    STATIC
        source/filter.h
        source/maths.h
        source/oscillator.h
        source/dsp.cpp
        include/dsp.h
)

target_compile_features(${PROJECT_NAME}
    PRIVATE
        cxx_std_20
)

target_include_directories(${PROJECT_NAME}
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

if(MSVC)
    target_compile_options(${PROJECT_NAME}
        PRIVATE
            /W4
            /WX
    )

    target_compile_definitions(${PROJECT_NAME}
        PRIVATE
            _USE_MATH_DEFINES
    )
else()
    target_compile_options(${PROJECT_NAME}
        PRIVATE
            -Wall
            -Werror
    )
endif()
