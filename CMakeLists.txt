cmake_minimum_required(VERSION 3.23)
project(Detours)

include(ExternalProject)

ExternalProject_Add(DetoursSrc
        GIT_REPOSITORY https://github.com/microsoft/Detours.git
        GIT_TAG master
        GIT_SHALLOW YES
        CONFIGURE_COMMAND ""
        BUILD_COMMAND ""
        INSTALL_COMMAND ""
        TEST_COMMAND ""
        BINARY_DIR ${CMAKE_BINARY_DIR}/Detours-Build
        )
ExternalProject_Get_Property(DetoursSrc SOURCE_DIR)
set(DETOURS_SOURCES
        ${CMAKE_BINARY_DIR}/Detours-Build/detours.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/modules.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disasm.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/image.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/creatwth.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disolx86.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disolx64.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disolia64.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disolarm.cpp
        ${CMAKE_BINARY_DIR}/Detours-Build/disolarm64.cpp)
set(DETOURS_HEADERS
        ${CMAKE_BINARY_DIR}/Detours-Build/detours.h
        ${CMAKE_BINARY_DIR}/Detours-Build/detver.h)
add_custom_command(OUTPUT ${DETOURS_SOURCES}
        COMMAND ${CMAKE_COMMAND} -E copy_directory ${SOURCE_DIR}/src/ ${CMAKE_BINARY_DIR}/Detours-Build
        BYPRODUCTS ${DETOURS_HEADERS}
        DEPENDS DetoursSrc)

add_library(Detours STATIC ${DETOURS_SOURCES})
add_dependencies(Detours DetoursSrc)
target_compile_definitions(Detours PRIVATE WIN32_LEAN_AND_MEAN)
target_sources(Detours PUBLIC ${DETOURS_HEADERS})
target_include_directories(Detours PUBLIC ${CMAKE_BINARY_DIR}/Detours-Build/)
install(TARGETS Detours)
