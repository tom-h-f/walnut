section .text

global __chkstk

__chkstk:
    pop     cx      ; grab the return address
    pop     dx      ; (and its segment)

    sub     sp, ax

    push    dx      ; push the return address
    push    cx
    ret             ; and go back to that address