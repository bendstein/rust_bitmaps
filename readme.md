An application that attempts to read a bitmap and render it to a terminal using ANSI colors.
    - On a truecolor/24-bit-compatible terminal, the bitmap will be rendered using the exact RGB of each pixel.
    - Otherwise, each RGB value will be approximated to the nearest terminal color.

## todo:
- ~~Look into using the ANSI escape code for invert to possibly add support for more output colors when approximating to terminal colors.~~ (Not feasible)
- Terminal colors are implemented with different color codes in different terminals. (e.g. Dark Red is <span style="font-weight: bold; color: #800000">#800000</span> in Powershell, but <span style="font-weight: bold; color: #CC3131">#CC3131</span> in VS Code). Is there a way to tell what terminal is being used, and change approximations from RGB to terminal color based on the terminal?
- Apply transformations to bitmap?
  - Convert to greyscale, rotate hue, change saturation, invert colors, change palette, shaders, etc
- L\*a\*b\* color space has three reference values which are supposed to help approximate color distances in a way similar to human vision, but right now I have all 3 references set to 1_f32. The approximation might improve with different reference values.
- Reading 1/4/8/24/32-bit bitmaps is supported, but 16-bit is not.
- 32-bit bitmaps have an alpha channel, but it is currently ignored during rendering.
## Known Bugs:
- ~~Application currently only works in some consoles.~~ (Fixed)
  - ~~VS Code's integrated terminal, and bash/unix terminals seem to work.~~
  - ~~Windows CMD/Powershell do not seem to directly support ANSI colors, and print the escape sequence literally.~~
    - ~~This only occurs when running the application directly; running the application using ```cargo run``` works as intended in both CMD and Powershell, suggesting that possibly there's an environment variable or argument I need set?~~
- Output breaks when the image doesn't fit in the bounds of the console.

## Arguments:

- Application arguments must be of one of the following forms:
  - For a key/value: /{KEY}:{VALUE}
  - For a flag: /{FLAG}

- The following are recognized arguments:

    - help
        - Description: Display application help.
        - Usage: /help
        - Restrictions: If used as a key-value argument, rather than a flag argument, must be either true or false.

    - path
        - Description: The path to the bitmap.
        - Usage: /path:{VALUE}
        - Restrictions: Must be a valid filepath (either relative or absolute) to a bitmap.

    - transparency
        - Description: A 32-bit, RGBA color representing transparency. Can be in decimal, binary (prefixed with 0b), or hex (prefixed with 0x).
        - Usage: /transparency:{VALUE}
        - Restrictions: Must be a non-negative, 32-bit integer.
        - Example:
          - Red: 0xFF000000, 4278190080, 0b11111111000000000000000000000000

    - no_truecolor
        - Description: When set, will display bitmap using 4-bit terminal colors even if the terminal supports truecolor/24-bit color.
        - Usage: /no_truecolor
        - Restrictions: If used as a key-value argument, rather than a flag argument, must be either true or false.
        - Default Value: false

    - pixel_string
        - Description: The string to use to represent a pixel when displaying the bitmap in the terminal.
        - Usage: /pixel_string:{VALUE}
        - Default Value: █

    - pixel_width
        - Description: The number of times {/pixel_string} should be repeated to display a pixel.
        - Usage: /pixel_width:{VALUE}
        - Restrictions: Must be a non-negative, 32-bit integer.
        - Default Value: 1

    - algorithm
    	- Description: The algorithm to use to calculate the distance between 2 colors when determining the best match between the actual color of a pixel and the 4-bit terminal color to use. Ignored if displaying bitmap in truecolor.
    	- Usage: /algorithm:{VALUE}
    	- Restrictions: [euclidean, manhattan, lab_euclidean, lab_manhattan]
    	- Default Value: lab_euclidean
