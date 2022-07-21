####################################################################
# This project has three parts:
#   1. The Zephyr app target, which is the minimum additions of
#      any firmware build with Zephyr SO;
#   2. The Rust library made by the user;
#   3. The backend of RART built for Zephyr SO;
# The part 2 and 3 is merged into a library, with the project name,
# and further linked with the Zephyr app target.
####################################################################
cmake_minimum_required(VERSION 3.13.1)

# Find Zephyr package
find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

# Set project Name
project(rart_no_std_tests)
# The name of rs-lib project (package/name at Cargo.toml)
set(RS_LIB_NAME rart_no_std_tests_rs)

# Add main.c to sources
list(APPEND SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/src/main.c)
# Add ZBUS backend source file to soruces
list(APPEND SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/3rdparty/rart-c/zbus/zbus_backend.c)
# Add RART-C source file to sources
list(APPEND SOURCES ${CMAKE_CURRENT_SOURCE_DIR}/3rdparty/rart-c/zephyr/rart.c)
# Add src/zbus to include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src/zbus)
# Add to zephyr the zbus headers
zephyr_include_directories("${CMAKE_CURRENT_SOURCE_DIR}/src/zbus")
# Add generated to include directories
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src/generated)
# Add sources to app target (Zephyr required)
target_sources(app PRIVATE ${SOURCES})
# Creating a library, with the project name, to include rs-lib and rart-c
add_library(${PROJECT_NAME} STATIC IMPORTED GLOBAL)
# Adding rs-lib
set_target_properties(${PROJECT_NAME} PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_SOURCE_DIR}/rs_lib/target/thumbv7em-none-eabi/release/lib${RS_LIB_NAME}.a)
# Linking library to Zephyr app target
target_link_libraries(app PUBLIC ${PROJECT_NAME})
