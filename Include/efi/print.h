#pragma once

#include "types.h"


EFI_STATUS efi_wprintln(str16 out_string);
EFI_STATUS efi_wprint(str16 out_string);
EFI_STATUS efi_flush_cin();
EFI_STATUS efi_flush_cout();

typedef struct {
	u32 MaxMode;
	u32 Mode;
	u32 Attribute;
	u32 CursorColumn;
	u32 CursorRow;
	u8  CursorVisible;
} SIMPLE_TEXT_OUTPUT_MODE;
 
typedef EFI_STATUS (*EFI_TEXT_CLEAR_SCREEN)(void *This);
typedef EFI_STATUS (*EFI_TEXT_ENABLE_CURSOR)(void *This, u8 Visible);
typedef EFI_STATUS (*EFI_TEXT_SET_ATTRIBUTE)(void *This, UINTN Attribute);
typedef EFI_STATUS (*EFI_TEXT_STRING)(void *This, str16 String);
 
typedef EFI_STATUS (*EFI_TEXT_QUERY_MODE)(
	void  *This,
	UINTN ModeNumber,
	UINTN *Columns,
	UINTN *Rows);
 
typedef EFI_STATUS (*EFI_TEXT_SET_CURSOR_POSITION)(
	void  *This,
	UINTN Column,
	UINTN Row);
 
typedef struct {
	EFI_PVOID                    Reset;
	EFI_TEXT_STRING              OutputString;
	EFI_PVOID                    TestString;
	EFI_TEXT_QUERY_MODE          QueryMode;
	EFI_PVOID                    SetMode;
	EFI_TEXT_SET_ATTRIBUTE       SetAttribute;
	EFI_TEXT_CLEAR_SCREEN        ClearScreen;
	EFI_TEXT_SET_CURSOR_POSITION SetCursorPosition;
	EFI_TEXT_ENABLE_CURSOR       EnableCursor;
	SIMPLE_TEXT_OUTPUT_MODE      *Mode;
} EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;
 
typedef struct {
	c16 ScanCode;
	c16 UnicodeChar;
} EFI_INPUT_KEY;
 
typedef EFI_STATUS (*EFI_INPUT_RESET)(void *This, bool ExtendedVerification);
typedef EFI_STATUS (*EFI_INPUT_READ_KEY)(void *This, EFI_INPUT_KEY *Key);
 
typedef struct {
	EFI_INPUT_RESET    Reset;
	EFI_INPUT_READ_KEY ReadKeyStroke;
	EFI_PVOID          WaitForKey;
} EFI_SIMPLE_TEXT_INPUT_PROTOCOL;