MEMORY {
    BOOT2   : ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH   : ORIGIN = 0x10000100, LENGTH = 1048K - 0x100
    FLASH_X : ORIGIN = 0x10100000, LENGTH = 1048K
    RAM     : ORIGIN = 0x20000000, LENGTH = 264K
}



SECTIONS
{
  .modem_firmware (NOLOAD) : ALIGN(4) {
    *(.modem_firmware .modem_firmware.*);
    . = ALIGN(4);
    } > FLASH_X

}