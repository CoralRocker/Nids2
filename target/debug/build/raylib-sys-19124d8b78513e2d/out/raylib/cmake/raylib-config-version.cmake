set(PACKAGE_VERSION "@PROJECT_VERSION@")

if(PACKAGE_FIND_VERSION VERSION_EQUAL PACKAGE_VERSION)
  set(PACKAGE_VERSION_EXACT TRUE)
endif()
if(NOT PACKAGE_FIND_VERSION VERSION_GREATER PACKAGE_VERSION)
  set(PACKAGE_VERSION_COMPATIBLE TRUE)
else(NOT PACKAGE_FIND_VERSION VERSION_GREATER PACKAGE_VERSION)
  set(PACKAGE_VERSION_UNSUITABLE TRUE)
endif(NOT PACKAGE_FIND_VERSION VERSION_GREATER PACKAGE_VERSION)

# if the installed or the using project don't have CMAKE_SIZEOF_VOID_P set, ignore it:
if("${CMAKE_SIZEOF_VOID_P}" STREQUAL "" OR "@CMAKE_SIZEOF_VOID_P@" STREQUAL "")
   return()
endif()

if(NOT CMAKE_SIZEOF_VOID_P STREQUAL "@CMAKE_SIZEOF_VOID_P@")
  math(EXPR installedBits "8 * 8")
  set(PACKAGE_VERSION "${PACKAGE_VERSION} (${installedBits}bit)")
  set(PACKAGE_VERSION_UNSUITABLE TRUE)
endif()
