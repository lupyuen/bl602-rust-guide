MEMORY
{
    ROM       (rx)  : ORIGIN = 0x21000000, LENGTH = 128K
    ITCM      (rx)  : ORIGIN = 0x22008000, LENGTH = 48K   /* Instruction Tightly Coupled Memory */
    DTCM      (rwa) : ORIGIN = 0x22014000, LENGTH = 48K   /* Data Tightly Coupled Memory */
    XIP_FLASH (rwx) : ORIGIN = 0x23000000, LENGTH = 16M   /* eXecute In Place Flash */
    /* On chip memory. Also aliased as 0x4202_0000. See BL602 Reference manual page 18 for details */
    RAM       (rwa) : ORIGIN = 0x42020000, LENGTH = 64K   
    WIFI_RAM  (wxa) : ORIGIN = 0x42030000, LENGTH = 112K
}

REGION_ALIAS("REGION_TEXT", XIP_FLASH);
REGION_ALIAS("REGION_RODATA", ITCM);
REGION_ALIAS("REGION_DATA", DTCM);
REGION_ALIAS("REGION_BSS", DTCM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", DTCM);
