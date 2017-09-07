bluepill-momo
==============
A minimal example using USB CDC.

Try out
=======

Build by:

::
    xargo build --release``
Extract the image:

::
    arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/bluepill-momo bp

Flash the image and test run (suppose the device is discovered as ``/dev/ttyACM0``):

::
    sudo screen /dev/ttyACM0 9600
