# Vibrant-rs

Extract vibrant colors from an image file. Can be used as a library, but also contains a simple executable.

[![Build Status](https://travis-ci.org/killercup/vibrant-rs.svg)](https://travis-ci.org/killercup/vibrant-rs)

## Getting Started

```bash
$ curl https://upload.wikimedia.org/wikipedia/commons/thumb/e/ec/Mona_Lisa%2C_by_Leonardo_da_Vinci%2C_from_C2RMF_retouched.jpg/687px-Mona_Lisa%2C_by_Leonardo_da_Vinci%2C_from_C2RMF_retouched.jpg > mona.jpg
$ cargo run --bin primary -- ./mona.jpg
Color Palette { #4D4D4D, #DFBB62, #A87B3A, #0F071C, #4E4141, #814F27, #512A22, #251629, #2C161E, #777B57 }
$ cargo run --bin vibrancy -- ./mona.jpg
Vibrant Colors {
	Primary Vibrant Color: #C3973F
	Dark Vibrant Color: #703C12
	Light Vibrant Color: #FEE087
	Muted Color: #A49F5C
	Dark Muted Color: #55362C
	Light Muted Color: #B4B57E
}
```

## License

MIT
