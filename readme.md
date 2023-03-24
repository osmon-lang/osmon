<header>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/osmon-lang/.github/raw/main/ASSETS/Osmon%20White.png">
  <img alt="Osmon's Logo" height="100" align="left" src="https://github.com/osmon-lang/.github/raw/main/ASSETS/Osmon%20Black.png">
</picture>
<h1 style="display: inline">Havo</h1>

Statically compilation component for Osmon. 

</header>

[![GitHub top language](https://img.shields.io/github/languages/top/osmon-lang/havo?color=232323&logo=github&labelColor=232323)](https://github.com/osmon-lang/havo)
[![Channel](https://img.shields.io/badge/telegram-grey?color=232323&label=chat&logo=telegram&labelColor=232323)](https://t.me/osmonlang)
[![Tests CI](https://img.shields.io/github/actions/workflow/status/osmon-lang/havo/test.yml?color=232323&label=test&logo=github-actions&labelColor=232323)](https://github.com/osmon-lang/havo/actions/workflows/test.yml)
 
## About

Compiler written on Rust using libgccjit as backend. Statically compiled programming language for advanced developers.

## Example

```
extern func printf(fmt: *char,...) void;
extern func calloc(c: i32,size: i32) *u8;

pub struct Point {
	x: i32,
	y: i32
}

pub func point_print(p: *Point) void {
	printf("(%i;%i)\n",p.x,p.y);
	return;
}

pub func main() i32 {
    var p: *Point;
    p = calloc(1,8) as *Point;
    p.x = 3;
    p.y = 4;
	point_print(p);
	printf("0x%lx\n",17179869187L);
	return 0;
}
```

## Installation

For *NIX based operating systems, you can install Osmon by running the following command:

```shell
cargo install havo
```

However, it's not possible to install Osmon on Windows. You can use [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to install Osmon on Windows.
If you want to use havo's libraries, then you may want to run:

```shell
cargo add havo
```

## License

This project is licensed under dual licence MIT and Apache-2.0 Licenses - see the [MIT](../license-mit) and [Apache](../license-apache) files for details.