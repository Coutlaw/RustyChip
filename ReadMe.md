**What is a chip-8?**

It essentially a virtual machine to make the development of games easier.

## Memory

![https://s3-us-west-2.amazonaws.com/secure.notion-static.com/dcc7d926-0b79-47be-be6f-6341742bb0b3/Untitled.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/dcc7d926-0b79-47be-be6f-6341742bb0b3/Untitled.png)

The bottom reserved memory is for things liek character sets

## Stack

The stack has 16, 16 bit values used to store the address that the Chip8 should return to when returning a subroutine. Chip8 allows for up to 16 levels of nested subroutines

The stack is not a part of the memory lay out, it is separate.

## Registers

the chip8 has 16 8 bit data registers, each register can hold 1 byte of information

![https://s3-us-west-2.amazonaws.com/secure.notion-static.com/45e4c782-9c36-4822-9545-c017727637b6/Untitled.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/45e4c782-9c36-4822-9545-c017727637b6/Untitled.png)

The data registers should be implemented with an array with a size of 16

### The I register

there is a 16 bit register I to store memory addresses

### The Program Counter

The chip8 has a 16 bit (2 bytes) Program Counter register that points to the address of the current instruction being executed

### Stack Pointer

Chip8 has an 8 bit Stack Pointer register that points to a location in the stack.

### Sound Timer + Delay Timer

these are each 8 bit registers and when they are below 0 they decrement at a rate of 60Hz

When sound is 0 the sound no longer plays, decrements by 60Hz

The delay timer keeps the program from executing instructions when the delay timer is above 0. it also decrements at 60Hz, and the program will continue executing instructions when the delay timer is back to 0.

## Instructions

Instructions in the Chip8 are 2 bytes in size, this is why we always increment the Program Counter by 2

there are 36 different instructions for implementation in the Chip8

## Display

the display is a 64x32 monochrome

Drawing is done with sprites, not pixels. sprites wrap the screen

Sprites are many pixels groupsed together that can be drawn to the screen as a whole

![https://s3-us-west-2.amazonaws.com/secure.notion-static.com/3dc7dae1-7b9b-4f66-a538-45e6aa842cc2/Untitled.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/3dc7dae1-7b9b-4f66-a538-45e6aa842cc2/Untitled.png)

Sprites can be a max of 8 bits in width, and 15 bits in height.

There is a draw instruction in the Chip8 so it knows what to draw

![https://s3-us-west-2.amazonaws.com/secure.notion-static.com/0d4b8d50-3cd5-477e-b0bf-621b1a6ddbd2/Untitled.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/0d4b8d50-3cd5-477e-b0bf-621b1a6ddbd2/Untitled.png)

Sprites get XOR'd onto the dispaly, if this causes any pixels to be erased then the VF register is set to 1, else 0

## Keyboard

16 keys from 0F, you could represent this with a 16 byte array. We would have to map our physical keyboard to this old keyboard.

![https://s3-us-west-2.amazonaws.com/secure.notion-static.com/d2d15b37-7b94-4120-afc7-4b0512a88d92/Untitled.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/d2d15b37-7b94-4120-afc7-4b0512a88d92/Untitled.png)

## Credits
- https://www.youtube.com/watch?v=ml44mHKd0ss

- http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0