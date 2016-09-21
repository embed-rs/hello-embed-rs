/* Layout file for the STM32F429I-DISC1 */

MEMORY
{
  /* there's a small hole after sram0 ends, but sram1 and 2 are actually
     continuous */
  sram0 (rwx)  : ORIGIN = 0x20000000, LENGTH = 112K
  sram1 (rwx)  : ORIGIN = 0x2001C000, LENGTH = 16K
  sram2 (rwx)  : ORIGIN = 0x20020000, LENGTH = 64K

  /* 2 megs of flash memory */
  flash (rwx) : ORIGIN = 0x08000000, LENGTH = 2M
}

SECTIONS
{
    .text 0x08000000:
    {
        _VT_BEGIN = .;
        KEEP(*(vectors))    /* Vector table */
        _VT_END = .;
        /* FIXME: research how large the vector table is, instead of just
           skpping 4K of flash memory */
        . = 0x10000;
        _TEXT_BEGIN = .;
        *(.text)            /* Program code */
        _TEXT_END = .;
        _RODATA_BEGIN = .;
        *(.rodata)          /* Read only data */
        _RODATA_END = .;
    } > flash

    /* "The .data segment contains any global or static variables which
       have a pre-defined value and can be modified. That is any
       variables that are not defined within a function (and thus can be
       accessed from anywhere) or are defined in a function but are
       defined as static so they retain their value across subsequent
       calls." */

    .data :
    {
        *(.data)
    } > sram0

    /* "The BSS segment, also known as uninitialized data, is usually
       adjacent to the data segment. The BSS segment contains all global
       variables and static variables that are initialized to zero or do
       not have explicit initialization in source code." */

    .bss :
    {
      _BSS_BEGIN = .;
        *(.bss)
      _BSS_END = .;
    } > sram0

    /* Heap and Stack follow afterwards */

    /* our stack will use the 64K of sram2. _STACK_TOP must be at the end
       of this area. 0xFFFC = 64K - 4 */
    .stack 0x2002FFFC: {
      _STACK_TOP = .;
    }
}
