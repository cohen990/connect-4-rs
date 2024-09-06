# Connect 4

## To set up rust

Follow the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## To run

`cargo run`

## To test

`cargo test`

## Acceptance Tests

I will be developing this following acceptance test driven development, otherwise known as double loop TDD.
You will notice there is a separate test file. This is sometimes thought of as non-idiomatic rust, but acceptance tests are high level and not tied to an individual file or even module. So I store them independently.

## The three versions

I got a wee bit carried away - which tends to happen when I write rust.

`submission` is the formal submission. I spent a couple of hours and it works and I'm happy with it.
`overengineered` was the start of the problem. I got very curious about how I could improve the system and enable it to support custom rulesets by using some kind of strategy / pipeline pattern for the win conditions.
`remove_consts` was the final thing I wanted to do. I wanted to be able to customise board size. But because of a choice I made very early, using const generics to size the board, this became impossible to assign dynamically. I wanted to have a sized array rather than a vector because I believe it's easier for the compiler to optimise the memory of. Vecs allocate to the heap whereas arrays can allocate to the stack. I thought the choice would give me the freedom to assign different board sizes in the future, but it became clear that it was impossible. I ended up using vecs. I think that the game itself is still assigned to the stack because the Vec is essentially a pointer to heap allocated data. (as far as I know).

Sorry, I got carried away! I'm happy if we ignore the `overengineered` and the `remove_consts` versions because I think the `submission` version is good enough. I did have fun though :)
