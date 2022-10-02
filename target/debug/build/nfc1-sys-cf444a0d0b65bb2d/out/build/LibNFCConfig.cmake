#                                               -*- cmake -*-


# Use the following variables to compile and link against LibNFC:
#  LIBNFC_FOUND              - True if LibNFC was found on your system
#  LIBNFC_USE_FILE           - The file making LibNFC usable
#  LIBNFC_DEFINITIONS        - Definitions needed to build with LibNFC
#  LIBNFC_INCLUDE_DIR        - Directory where nfc/nfc.h can be found
#  LIBNFC_INCLUDE_DIRS       - List of directories of LibNFC and it's dependencies
#  LIBNFC_LIBRARY            - LibNFC library location
#  LIBNFC_LIBRARIES          - List of libraries to link against LibNFC library
#  LIBNFC_LIBRARY_DIRS       - List of directories containing LibNFC' libraries
#  LIBNFC_ROOT_DIR           - The base directory of LibNFC
#  LIBNFC_VERSION_STRING     - A human-readable string containing the version
#  LIBNFC_VERSION_MAJOR      - The major version of LibNFC
#  LIBNFC_VERSION_MINOR      - The minor version of LibNFC
#  LIBNFC_VERSION_PATCH      - The patch version of LibNFC

set ( LIBNFC_FOUND 1 )
set ( LIBNFC_USE_FILE     "/home/random/Projects/charlie-rs/target/debug/build/nfc1-sys-cf444a0d0b65bb2d/out/lib/cmake/libnfc/UseLibNFC.cmake" )

set ( LIBNFC_DEFINITIONS  "" )
set ( LIBNFC_INCLUDE_DIR  "/home/random/Projects/charlie-rs/target/debug/build/nfc1-sys-cf444a0d0b65bb2d/out/include" )
set ( LIBNFC_INCLUDE_DIRS "/home/random/Projects/charlie-rs/target/debug/build/nfc1-sys-cf444a0d0b65bb2d/out/include;/home/random/Projects/charlie-rs/target/debug/build/usb-compat-01-sys-dde48ed253cdd80a/out/include" )
set ( LIBNFC_LIBRARY      "nfc" )
set ( LIBNFC_LIBRARIES    "nfc;/home/random/Projects/charlie-rs/target/debug/build/usb-compat-01-sys-dde48ed253cdd80a/out/libusb.a;/home/random/Projects/charlie-rs/target/debug/build/libusb1-sys-a9d1c6ec1780e002/out/libusb.a" )
set ( LIBNFC_LIBRARY_DIRS "/home/random/Projects/charlie-rs/target/debug/build/nfc1-sys-cf444a0d0b65bb2d/out/lib" )
set ( LIBNFC_ROOT_DIR     "/home/random/Projects/charlie-rs/target/debug/build/nfc1-sys-cf444a0d0b65bb2d/out" )

set ( LIBNFC_VERSION_STRING "1.8.0" )
set ( LIBNFC_VERSION_MAJOR  "1" )
set ( LIBNFC_VERSION_MINOR  "8" )
set ( LIBNFC_VERSION_PATCH  "0" )

