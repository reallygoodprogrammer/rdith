# rdith

Tool for layer images through dithering operations.

# usage

The ordering for layering the images is determined by the
order given through the `INPUT_FILES` arguments. The first
image argument is used to determine the size of the final
output image.

```
Usage: rdith [OPTIONS] [INPUT_FILES]...

Arguments:
  [INPUT_FILES]...  input files for dithering

Options:
      --output-file <OUTPUT_FILE>  name of output file [default: output.png]
      --predith                    predither the background layer
  -d, --dims-matrix <DIMS_MATRIX>  dimensions of dithering matrix [default: 5]
  -r, --res-matrix <RES_MATRIX>    resolution of dithering matrix [default: 1]
  -h, --help                       Print help
```
