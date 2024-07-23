target extended-remote :1234
layout split
break kmain
break qemu_cortex_r5_app::panic
