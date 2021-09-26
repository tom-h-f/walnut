#include "libc/string.h"

c16* strcat(c16* dest, const c16* src)
{
    // create temp pointer to the dest buffer
    c16* p = dest;
    
    // iterate through the dest buffer, so we point to the terminating null byte
    while(*p != '\0') ++p;

    // Now we are pointing at the end of 'dest'
    // copy every byte of 'src' up to and including
    // the terminating zero.
    while ((*p++ = *src++));

    // return the original dest pointer, but now it will have
    // 'src' concatenated on the end along with it's terminating null byte.
    return dest;
}

static unsigned int strlen(const c16* s)
{
	unsigned int len = 0;
	while (s[len] != '\0') len++;
	return len;
}


static unsigned int
itoa(int value, unsigned int radix, unsigned int uppercase, unsigned int unsig,
	 c16 *buffer, unsigned int zero_pad)
{
	c16	*pbuffer = buffer;
	int	negative = 0;
	unsigned int	i, len;

	/* No support for unusual radixes. */
	if (radix > 16)
		return 0;

	if (value < 0 && !unsig) {
		negative = 1;
		value = -value;
	}

	/* This builds the string back to front ... */
	do {
		int digit = value % radix;
		*(pbuffer++) = (digit < 10 ? '0' + digit : (uppercase ? 'A' : 'a') + digit - 10);
		value /= radix;
	} while (value > 0);

	for (i = (pbuffer - buffer); i < zero_pad; i++)
		*(pbuffer++) = '0';

	if (negative)
		*(pbuffer++) = '-';

	*(pbuffer) = '\0';

	/* ... now we reverse it (could do it recursively but will
	 * conserve the stack space) */
	len = (pbuffer - buffer);
	for (i = 0; i < len / 2; i++) {
		c16 j = buffer[i];
		buffer[i] = buffer[len-i-1];
		buffer[len-i-1] = j;
	}

	return len;
}

struct buff {
	c16 *buffer, *pbuffer;
	unsigned int buffer_len;
};

struct guid {
	unsigned long data1;
	unsigned short data2;
	unsigned short data3;
	c16 data4[8];
};

const c16 *lguid_fmt = L"%08x-%04x-%04x-%02x%02x-%02x%02x%02x%02x%02x%02x";
const c16 *uguid_fmt = L"%08X-%04X-%04X-%02X%02X-%02X%02X%02X%02X%02X%02X";

struct efi_status_type {
	unsigned long long status;
	c16 *string;
};

/* FIXME: support 32-bit */
const struct efi_status_type efi_status[] = {
	{ 0x0ULL               , L"EFI_SUCCESS"               },
	{ 0x8000000000000001ULL, L"EFI_LOAD_ERROR"            },
	{ 0x8000000000000002ULL, L"EFI_INVALID_PARAMETER"     },
	{ 0x8000000000000003ULL, L"EFI_UNSUPPORTED"           },
	{ 0x8000000000000004ULL, L"EFI_BAD_BUFFER_SIZE"       },
	{ 0x8000000000000005ULL, L"EFI_BUFFER_TOO_SMALL"      },
	{ 0x8000000000000006ULL, L"EFI_NOT_READY"             },
	{ 0x8000000000000007ULL, L"EFI_DEVICE_ERROR"          },
	{ 0x8000000000000008ULL, L"EFI_WRITE_PROTECTED"       },
	{ 0x8000000000000009ULL, L"EFI_OUT_OF_RESOURCES"      },
	{ 0x800000000000000aULL, L"EFI_VOLUME_CORRUPTED"      },
	{ 0x800000000000000bULL, L"EFI_VOLUME_FULL"           },
	{ 0x800000000000000cULL, L"EFI_NO_MEDIA"              },
	{ 0x800000000000000dULL, L"EFI_MEDIA_CHANGED"         },
	{ 0x800000000000000eULL, L"EFI_NOT_FOUND"             },
	{ 0x800000000000000fULL, L"EFI_ACCESS_DENIED"         },
	{ 0x8000000000000010ULL, L"EFI_NO_RESPONSE"           },
	{ 0x8000000000000011ULL, L"EFI_NO_MAPPING"            },
	{ 0x8000000000000012ULL, L"EFI_TIMEOUT"               },
	{ 0x8000000000000013ULL, L"EFI_NOT_STARTED"           },
	{ 0x8000000000000014ULL, L"EFI_ALREADY_STARTED"       },
	{ 0x8000000000000015ULL, L"EFI_ABORTED"               },
	{ 0x8000000000000016ULL, L"EFI_ICMP_ERROR"            },
	{ 0x8000000000000017ULL, L"EFI_TFTP_ERROR"            },
	{ 0x8000000000000018ULL, L"EFI_PROTOCOL_ERROR"        },
	{ 0x8000000000000019ULL, L"EFI_INCOMPATIBLE_VERSION"  },
	{ 0x800000000000001aULL, L"EFI_SECURITY_VIOLATION"    },
	{ 0x800000000000001bULL, L"EFI_CRC_ERROR"             },
	{ 0x800000000000001cULL, L"EFI_END_OF_MEDIA"          },
	{ 0x800000000000001fULL, L"EFI_END_OF_FILE"           },
	{ 0x8000000000000020ULL, L"EFI_INVALID_LANGUAGE"      },
	{ 0x8000000000000021ULL, L"EFI_COMPROMISED_DATA"      },
	{ 0x8000000000000023ULL, L"EFI_HTTP_ERROR"            },
	{ 0x8000000000000064ULL, L"EFI_NETWORK_UNREACHABLE"   },
	{ 0x8000000000000065ULL, L"EFI_HOST_UNREACHABLE"      },
	{ 0x8000000000000066ULL, L"EFI_PROTOCOL_UNREACHABLE"  },
	{ 0x8000000000000067ULL, L"EFI_PORT_UNREACHABLE"      },
	{ 0x8000000000000068ULL, L"EFI_CONNECTION_FIN"        },
	{ 0x8000000000000069ULL, L"EFI_CONNECTION_RESET"      },
	{ 0x800000000000006aULL, L"EFI_CONNECTION_REFUSED"    },
	{ 0x1ULL               , L"EFI_WARN_UNKNOWN_GLYPH"    },
	{ 0x2ULL               , L"EFI_WARN_DELETE_FAILURE"   },
	{ 0x3ULL               , L"EFI_WARN_WRITE_FAILURE"    },
	{ 0x4ULL               , L"EFI_WARN_BUFFER_TOO_SMALL" },
	{ 0x5ULL               , L"EFI_WARN_STALE_DATA"       },
	{ 0x6ULL               , L"EFI_WARN_FILE_SYSTEM"      },
};

static c16 *
_efi_status_str(unsigned long long status)
{
	unsigned int i;
	unsigned int len;
	len = sizeof(efi_status) / sizeof(efi_status[0]);
	for (i = 0; i < len; i++) {
		if (efi_status[i].status == status)
			return efi_status[i].string;
	}
	return L"";
}

static int
_putc(int ch, struct buff *b)
{
	if ((unsigned int)((b->pbuffer - b->buffer) + 1) >= b->buffer_len)
		return 0;
	*(b->pbuffer++) = ch;
	*(b->pbuffer) = '\0';
	return 1;
}

static int
_puts(c16 *s, unsigned int len, struct buff *b)
{
	unsigned int i;

	if (b->buffer_len - (b->pbuffer - b->buffer) - 1 < len)
		len = b->buffer_len - (b->pbuffer - b->buffer) - 1;

	/* Copy to buffer */
	for (i = 0; i < len; i++)
		*(b->pbuffer++) = s[i];
	*(b->pbuffer) = '\0';

	return len;
}

int
vsnprintf(c16 *buffer, unsigned int buffer_len, const c16 *fmt, __builtin_va_list va)
{
	struct buff b;
	c16 bf[48];
	c16 ch;

	b.buffer = buffer;
	b.pbuffer = buffer;
	b.buffer_len = buffer_len;

	while ((ch=*(fmt++))) {
		if ((unsigned int)((b.pbuffer - b.buffer) + 1) >= b.buffer_len)
			break;
		if (ch!='%')
			_putc(ch, &b);
		else {
			c16 zero_pad = 0;
			c16 *ptr;
			unsigned short *wptr;
			struct guid *g;
			unsigned int len;

			ch=*(fmt++);

			/* Zero padding requested */
			if (ch=='0') {
				ch=*(fmt++);
				if (ch == '\0')
					goto end;
				if (ch >= '0' && ch <= '9')
					zero_pad = ch - '0';
				ch=*(fmt++);
			}

			switch (ch) {
				case 0:
					goto end;

				case 'u':
				case 'd':
					len = itoa(__builtin_va_arg(va, unsigned int), 10, 0, (ch=='u'), bf, zero_pad);
					_puts(bf, len, &b);
					break;

				case 'x':
				case 'X':
					len = itoa(__builtin_va_arg(va, unsigned int), 16, (ch=='X'), 1, bf, zero_pad);
					_puts(bf, len, &b);
					break;

				case 'c' :
					_putc((c16)(__builtin_va_arg(va, int)), &b);
					break;

				case 's' :
					ptr = __builtin_va_arg(va, c16*);
					_puts(ptr, strlen(ptr), &b);
					break;

                case 'S' :
					wptr = __builtin_va_arg(va, unsigned short *);
                    while (*wptr != L'\0')
						_putc((c16)*(wptr++), &b);
					break;

				case 'g' :
				case 'G' :
					g = __builtin_va_arg(va, struct guid*);
					len = snprintf(bf, 48,
						(ch=='G') ? uguid_fmt : lguid_fmt,
						g->data1, g->data2, g->data3,
						g->data4[0], g->data4[1], g->data4[2], g->data4[3],
						g->data4[4], g->data4[5], g->data4[6], g->data4[7]);
					_puts(bf, len, &b);
					break;

				case 'r' :
					ptr = _efi_status_str(__builtin_va_arg(va, unsigned long long));
					_puts(ptr, strlen(ptr), &b);
					break;

				default:
					_putc(ch, &b);
					break;
			}
		}
	}
end:
	return b.pbuffer - b.buffer;
}


int
snprintf(c16* buffer, unsigned int buffer_len, const c16 *fmt, ...)
{
	int ret;
	__builtin_va_list va;
    __builtin_va_start(va, fmt);
	ret = vsnprintf(buffer, buffer_len, fmt, va);
	__builtin_va_end(va);
	return ret;
}

