# ASCII Star Wars Server

I wanted to use the [Star Wars
ASCIImation](https://www.asciimation.co.nz/) for our _Introduction to
Networks_ class. Students write a basic client that receives a few bytes.
They get confused. Then I have them put it in a loop and they are
amazed! Wonderful!

I used to send them to
[towel.blinkenlights.nl](telnet://towel.blinkenlights.nl/), but that
site seems to have gone down. There's another one at
[starwarstel.net](telnet://starwarstel.net) that has awesome on-screen
controls and even works with the mouse, but it's more than I wanted and
the students' dumb clients had trouble with the telnet protocol.

Of course, I could have written this in any language, but I wrote it in
Rust to learn how it does networking and OS multithreading.

## The Animation Itself

I want to be clear that I did **not** create the original animation. I
just wrote a thing that sends it over the network. All credit for the
hard work goes to [Simon Jansen](https://www.asciimation.co.nz/).

## Usage

```
cargo run              # defaults to sw1.txt
cargo run sw1.txt      # specifying an ASCIImation file
```

## License

The code is released under the [Unlicense](https://unlicense.org/). But
the `sw1.txt` animation file is not. I did not include a `LICENSE.md` to
avoid ambiguity.

## Bugs

* Uses OS threads so it won't scale. Someone will have to make it
  async.

