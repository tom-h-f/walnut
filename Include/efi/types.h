typedef void* EFI_PVOID;
typedef void* EFI_HANDLE;
 
#ifdef __i386__
#define MESSAGE L"i386\r\n"
typedef u32 UINTN;
#endif
#ifdef __amd64__
#define MESSAGE L"x86_64\r\n"
typedef u64 UINTN;
#endif
 
typedef UINTN EFI_STATUS;