# image-to-ascii

This is a toy project started with the intention of learning about WebAssembly
and the Rust programming language.

## Usage

### Browser

```js
import('image-to-ascii').then(({ fromData }) => {
  const ctx = canvas.getContext('2d');
  const { data } = ctx.getImageData(0, 0, width, height);
  const ascii = fromData(canvas.width, canvas.height, data);
});
```

```js
import('image-to-ascii').then(({ fromCanvas }) => {
  const ascii = fromCanvas(canvas);
});
```

### Rust

```rust
extern crate image_to_ascii;

use image_to_ascii::from_data;

pub fn main() {
  from_data(width, height, &data);
}
```

## API

### JavaScript

```ts
function fromData(width: number, height: number, data: Uint8Array): string;
```

### Rust

```rust
fn from_data(width: i32, height: i32, data: &[u8]) -> String
```

## CLI

```sh
$ image-to-ascii ./some/image.png
```

```text
                                               .;;
                              1@@@@@@@@@@@@@888000C
                             18GGLttttttttttttLGGGG8:
                            L8GGGttttttttttttttLGGGG8G.
                          .00GG8Lttttttttttttttf8GGGG0@C.
                         i@GGG8GtttttLtttttttttt8@88808@@8t
                        G@@@@0CtttttLGftttttttttttttfG0088@;
                      ;@GftttttttttfGGCttttttttttttttCGGGG@1   .,:;i111ii:.
                      i@CttttttttttCGGGftttttttttttttLGGGG@@800GGGGGGGGGGGGGG8f
                      ;@CtttttttttLGGGGCtttttttttttttfGGGG@8GGGGGGGGGGGGGGGGGLf8
                      ;@GtttttttttttttttttttttttttttttG08@@8GGGGGGGGGGGGGGGCftf@
                  .iC88@@@@@0GLftttttttttttttffCG08@@@@@@@@8GGGGGGGGGGCCfttttf8t
             ,t88Gfttt0@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@8GGGGCCLfftttttttttC8,
        .108Cfttttttttf0@@@@@@@@@@@@@@@@@@@@@@@@@@@@@GLttttttttttttttttttf00,
     i00LttttttttttttttttttffLCGG0000888000GGCLffttttttttttttttttttttfC88i
  18CttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttLG@@C:
18fttttttttttttttttttttttttttttttttttttttttttttttttttttttttLG8@@@C:
80ttttttttttttttttttttttttttttttttttttttttttttttttfLCG8@@@@@01.
 f@8GLftttttttttttttttttttttttttttttttfLLCG08@@@@@@80CLt1080
    ,1C8@@@@@@@@@@@@@@@@@@@@@@@@8800GCLLft11ttffft1iiiii1@0@;
                      t@iitG@@@@0ftiiiiiiii8@@@LLLCCiiiiC808L
                      f8iiiiitL1iiiiiiiiiiii11iiiii;;;;;@0008
                      L0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;L@000@;
                      C0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;88000@f
                      G@8@@@@@@@@@@@@@@@@@@@@80GCLft1if@000080
                      00GGGGGGGGGGGGGGGGGGGGG0000000888@00008@,
                    ;G@@@@@88888@GCCGGGGG0GG00000000000000000@i
                   8f;;C0GGGGGGG@i;;;;;;;;;;;;000000000000000@f
                   8f;;L0GGGGGGG@i;;;;;;;;;;;;800000000000000@C
                   8f;;LGGGGGGGG@1;;;;;;;;;;;t800000000000000@0
                  .@0GGGGGGGGGGG8Ct1i;;;;;;;t800000000000000088
                  .@0GGGGGGGGGGGGGGGGGGGGG00000000000000000008@,
                  .@0GGGGGGGGGGGGGGGGGGGGG00000000000000000008@;
                  .@0GGGGGGGGGGGGGGGGGGGGG00000000000000000000@i
                  .@0GGGGGGGGGGGGGGGGGGGGG0000000000000008@@@@8i
                  .@0GGGGGGGGGGGGGGGGGGGGG0000000008@@@@@Ci.
                   .iL0@@@888000GGGGGGG0088@@@@@@8C1:
                             .,:;ii111ii;:,.
```
