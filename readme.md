This is the repository for the prototype [cohabitive game](https://makopool.com/peacewagers.html), P1.

It contains rust code that generates the cards. It can then invoke inkscape to generate `generated_card_pngs`, which can be dragged straight into thegamecrafter as mini card assets, and then printed and distributed from there.

### why generate cards programatically?

Although specifying cards in code takes a lot longer than just drawing a deck in an editor, it's only slower the first time. This approach allows us to change shared details across all cards all at once. We'll never have to do a full deck redraw.

It also gives cards consistent and accurate alignment. Parametric editing often makes it easy to tweak and explore parameters in a way that isn't possible with conventional editors.

### why rust?

Typescript would have been a better choice! But rust is pretty flexible, there aren't really any domains it's terrible at.

One hope I had was that I'd be able to call resvg and I'd be able to render pngs in pure rust. Resvg unfortunately seems to ignore line wrapping within text boxes, it also selects a fainter version of the font. For now we'll just call out to inkscape.

### usage

If you want to draw some cards entirely in inkscape instead of generating them, we can just put them in "handmade cards". Feel very free to just draw the part of the card that's unique, write a description, and ask me to do the rest.

[install Rust](https://www.rust-lang.org/tools/install) and the [Rubik](https://fonts.google.com/specimen/Rubik) font. Run `cargo run`. You'll also need inkscape to render the `generated_card_svgs` to pngs.

If you want to make a card, look at other card generation code that generates similar cards and adapt it to your needs. If you need help with understanding rust, we're here for you and you can get us in the [cohabitive games element chat](https://matrix.to/#/#peacewagers:matrix.org). If you need help with inkscape... I'm sorry about inkscape. But I'll try to help.