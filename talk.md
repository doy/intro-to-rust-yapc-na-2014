# Motivation

Rust is a new systems language being developed by Mozilla. It is still
in development, but the first stable release is planned for later this
year. As a heads up, the details in this talk will be based on the state
of the current master branch, *not* the latest development release.

Rust's goal is to provide an alternative for projects which would
otherwise be written in C or C++. The problem with C-based languages is
that they are extremely difficult to use correctly, as a project gets
big enough. It is very easy to accidentally write C++ code that causes
segmentation faults (unrecoverable errors caused by accessing memory
that doesn't exist), silent memory corruption, and all kinds of other
issues that can result in security issues and data loss.

Now, the reason that people still use C++ is because of the high level
of control that it gives you. This control allows you to write code
which wouldn't even be possible in other languages (how would you write
an interrupt handler in Perl?), and also allows you to write extremely
efficient code (computation-heavy code can run several orders of
magnitude faster in C++ compared to Perl). We often talk about premature
optimization, and how getting that last 10 or 15% of performance out of
a piece of code isn't actually worth it, but that is largely a factor of
the field that most of us work in. We can ignore those optimizations
because of how insignificant they are compared to the time it takes for
the OS to read some data in from disk or from a database, but that does
imply that a 10% difference in speed in the disk controller or database
code can actually matter.

Rust's goal, therefore, is to provide the same level of control you get
when writing in C++ while removing as many of the dangerous sharp edges
as possible. Its philosophy is based strongly on the idea of zero cost
abstractions. One of the main benefits of writing in C++ is that it is
fairly straightforward to see how a given piece of C++ code translates
down into machine instructions. To succeed, Rust needs to retain that.
This means no mandatory boxing of variables, no mandatory garbage
collection or reference counting, and really no mandatory runtime at
all. Instead, Rust has things like the ability to optionally box
variables explicitly, and have the compiler verify that they are used
and cleaned up properly, using the same sort of memory management you
would write by hand in C++ using new and delete. When it introduces an
entirely new abstraction like closures, it makes sure that those
closures are inlinable, so code written using them can end up just as
efficient as code written without the abstraction layer. These new
abstractions can be used by Rust's compiler to completely eliminate
things like null pointers, memory corruption, data races in concurrent
code, and use of uninitialized data while adding no overhead at all.

Sometimes avoiding those kinds of things isn't possible, though - for
instance, Rust is self-hosting, and so it needs to be able to talk to
the operating system somehow. Also, there are situations where a safe
implementation of an algorithm would be possible, but being able to
"cheat" internally can make the code much faster while still providing
an entirely safe public API. For this case, Rust also provides a way to
disable most of its safety checking within specific scopes. In effect,
the code within these unsafe blocks becomes an alternative syntax for C,
so anything you would be able to express in C should be possible within
that limited scope.

Now, a common question at this point is "Why a new language? Couldn't
you just write a better C++ compiler instead?" There are a couple
answers here. First, given the level of safety that Rust is targeting,
effectively no existing C++ programs would even compile. So much of the
reasoning behind why existing programs are safe is implicit that there
is no hope of writing a compiler which can figure it all out. So at this
point, you need to start adding additional annotations and such in order
to make it all explicit, and then you already basically have another
language. Also, Rust is still built on top of LLVM (the backend for
clang), so it's not like it's starting entirely from scratch - Rust
isn't throwing out the years of work that has gone into optimizing C++
code because most of that optimization only happens once it gets to the
compiler backend, and that is still the same.

Another common question is "Why Mozilla?" Well, as mentioned earlier,
there are a few places where every bit of speed counts, and these days,
web browsers are definitely one of those places. Really, if you squint a
bit, web browsers are basically on the level of operating systems at
this point. They run all kinds of untrusted code, all of that untrusted
code has to go through them to access the hardware, and their job is to
keep it all safe, sandboxed, and secure. Firefox, though, is around 8
million lines of C++ code at this point, and it's effectively impossible
to write 8 million lines of C++ code without a memory or concurrency bug
showing up somewhere. The issue with those kinds of bugs though is that
they are completely invisible until the exact right circumstances
occur, and so the normal strategies of testing and things like that
don't really help all that much. Mozilla and the other browser makers
are doing an excellent job at keeping things running the way they are,
but it's not clear at all if that's going to be sustainable in the long
term. With that in mind, Mozilla is using Rust to write a new browser
rendering engine called Servo, which is built from the ground up to be
both secure, leveraging Rust's stronger safety guarantees, and fast,
being built from the ground up to support pervasive (and safe)
parallelism, among other things. It already has parallel layout and
rendering, and passes the Acid2 test, and while it's not likely to
replace Firefox for quite some time yet, the goal is to have a usable
browser based on Servo implemented by the end of the year.

# Overview

## Language structure

Rust's syntax is based on C and ML, among a few others. Like Perl, it's
a whitespace-insensitive, brace-based language, but unlike Perl, pretty
much everything is an expression, including things like if statements.
This is what "hello world" looks like in Rust. Functions are declared
with 'fn', the entry point to the program is the function 'main' (just
like in C), and 'println!' is Rust's equivalent to printf.

Here's a more complicated example (from the main page of the Rust
website). As you can see, variables are declared using 'let', must be
initialized at the point of declaration, and are immutable by default.
Mutable variables are declared using 'let mut'. Iteration is done
through 'for' and 'while' loops. In this example, the 'chars' method on
a string returns an iterator which returns each character in the string
in turn. Characters in Rust are four byte Unicode codepoints, and
strings are stored internally in utf8. Another minor point is that like
Perl 6, for loops (and while loops, and conditionals) don't require
parentheses around the condition.

Rust also has pattern matching, similar to ML. Matching can be done on
arbitrary data structures, and the compiler verifies that the match is
exhaustive, so not only is it more readable than a series of if
statements, it is also more safe.

Finally, you can see a more complicated example of 'println!' at the
end. The trailing '!' indicates that 'println!' is a macro, so it can do
things not normally possible in the language syntax. This is a general
rule in order to make the language more easily parsable by external
tools - macros are introduced with an identifier that ends with an
exclamation mark, and must be delimited by matching parentheses,
brackets, or braces. The pattern language that println! uses is actually
based on Python rather than printf. A bare set of braces means to
automatically choose the correct stringification based on the type of
the given parameter (for types that define one, which includes most
builtin types). You can also pass the specifier explicitly if you need
to pass arguments to it, and the special '{:?}' specifier uses
reflection mechanisms in order to print out complicated data structures
for debugging, even if they haven't implemented a stringification.

As mentioned earlier, for loops use iterators for iteration. This lets
them avoid using more memory than necessary, and also allows operations
to be easily composed. In this example, for instance, we take the chars
iterator and filter out the spaces, leaving only the characters we care
about. This is all done without ever building a new list - the character
values are calculated out of the string directly. The filter method (and
most of the other iterator methods) can (most likely) then be inlined,
and the resulting code is no different from what you would write
otherwise by manually moving pointers around.

Another thing to note is that the filter method takes a closure as an
argument. Closure syntax is based on Ruby's block syntax. In this case,
the closure takes a borrowed pointer to the character to be filtered,
which is why the parameter is declared as '&x'. We'll get into what
exactly that means later in the talk.

Notice also that the closure doesn't require a return statement. Rust
works the same way that Perl does, in that return statements are
optional at the end of a function body, whether it's a closure or a
named function. There is one minor difference in that just as in Perl,
semicolons are statement separators rather than terminators, but unlike
in Perl, empty statements aren't ignored, so if you want to implicitly
return a value, the final semicolon must be omitted, or else your
function will be returning nil.

## Type System

In addition to basic types like integers floating point values, and
arrays, Rust also has several different ways to build more complicated
data structures. The most basic way is using structs, like this. Structs
in Rust are pretty much the same as structs in C, but you can actually
initialize them anywhere you allocate them (in fact, you're required
to). These structs are also entirely compatible on the memory
representation level with C, and so passing structs back and forth
between Rust and C is guaranteed to work.

Rust also has enum types, just like C. One advantage to them over C
enums is that when they are used in a pattern match, the compiler checks
that your match statement covers all of the possible enum values (like
this), and that it doesn't include values that don't exist (like this).
A bigger advantage though is that Rust enums aren't just enums - they
are actually algebraic data types in disguise. For instance, the Color
enum could be extended to include a custom color, like this. Here, the
Custom enum value includes data attached to it, which we can extract
through destructuring bind in the match statement (note that
destructuring bind also works identically in 'let' statements). The Rust
standard library includes some useful examples of enums, such as an
Option type, which looks like this.

The option type is also a good example of Rust's support for generics.
Structs, enums, and functions (as well as a few other things) can be
parameterized by types. This works pretty much identically to C++
templates, in that the compiler will see which types are actually being
used for the parameter, and generate separate copies of the type or
function for each type argument that was used.

As you can see from these examples, Rust is also capable of type
inference. You almost never have to explicitly specify types when
defining variables or calling functions, even when using things like
destructuring bind. One exception here is method signatures. One of
Rust's design principles is that public API should always be explicit to
avoid accidental incompatibilities, and so things like function
signatures require explicit types. Another exception is that you can't
infer on return values, but that's usually only relevant when using
generics.

In addition to the basic builtin types, Rust's standard library also
includes a lot of helpful data structures. The two that you'll probably
be using most often are Vec and str (roughly corresponding to vector and
string in C++). Here's an example of using vectors - you can see the
vector being initialized and modified, and printing the length and the
individual values. Here's a similar example using strings. Something to
notice is how both vectors and strings have special initialization
syntax (the vec! macro and the String::from_str function). This is
because the builtin vectors and string that you can use with bare
brackets or a bare quoted string are fixed size, which allows them to be
allocated in place, which is much more efficient in general. If you want
to be able to modify the string or vector, you need to create a
modifiable version, which requires special initialization. You can
easily get fixed size slices out of the data stored in a growable vector
or string, though, and this is useful because the majority of functions
in the Rust standard library operate on fixed size slices.

One other thing you may have noticed in the previous examples is that I
was calling methods on the vectors and strings. Rust allows you to
define implementations of types using the impl keyword. You can define
class methods, which are called just like normal functions, as well as
instance methods, which are distinguished from class methods by taking
an initial 'self' parameter (we'll talk about what that '&' means
later). Methods use static dispatch - dynamic dispatch does exist, but
it's more complicated and not really in the scope of this talk.

One final aspect to the type system I'd like to cover is traits. Traits
work pretty similar to implementations elsewhere - they represent a
common bundle of behavior that can be implemented by any given type.
Traits can have default implementations for their methods, and can be
implemented on a type either by the author of the trait or by the author
of the type, for maximum flexibility. Traits can also be used as bounds
on type parameters, in order to write functions that only operate on
types that implement a given trait. Traits are also used to implement
various builtin features like operator overloading, as well as things in
the standard library - for instance, the Show trait implements the
default formatting behavior for println! as seen here. The details of
this implementation aren't important, just the fact that this is all
handled through traits.

## Pointers and ownership

You may have heard that Rust has all of these different kinds of
pointers and it's all confusing. This is no longer really the case. As
the language is moving towards a stable release, the development team
has been putting a lot of effort into simplifying the language and
removing features that don't really pull their weight.

In general, most data you will deal with will be values allocated
statically on the stack. If you need an integer, you can just declare an
integer variable and use it. The same thing holds true for more
complicated data structures - for instance, the Point example earlier.
Allocating as much as possible on the stack is a good thing because
stack allocation is extremely fast.

Stack variables have limitations though, in that they are only valid in
the function in which they are declared. They can only be passed into
functions and returned from functions by copying. This is fine for small
types like integers, but can have a significant impact for larger types.
In order to pass data around without requiring copying it everywhere,
you'll need to use pointers. The most common type of pointer you'll
encounter is the borrowed pointer. When you take a borrowed pointer to a
piece of data, the compiler verifies that the data it's pointing to
lives as least as long as the pointer - if it doesn't, then it throws a
compile-time error. Once it has verified this, you can use it however
you want, and you'll know that it will never end up pointing to invalid
data. This means that borrowed pointers have no runtime impact at all -
they don't require any cleanup because the compiler already verified
that the data will be cleaned up elsewhere.

Take this C++ example, for instance. This program will happily compile,
and result in undefined behavior since the variable being pointed to no
longer exists once the function returns. This is called a "dangling
pointer", and can also happen when you dynamically allocate memory, but
free it too early. In contrast, if we translate the same example into
Rust, a compile time error is issued, telling us that we're trying to
make a borrowed pointer live longer than the thing it points to.

Borrowed pointers allow you to take references to existing data easily
enough, but sometimes you need to create data that will outlive the
current function's scope. In other words, you need to allocate a new
chunk of memory that you own, and ensure that it is cleaned up. For this
case, Rust allows you to "box" values, which just means to allocate a
chunk of memory and give you a pointer to it instead. For instance, we
can fix our earlier example like this. Here we create a new boxed value
with the integer 2 inside it, and then we return that boxed value. Since
this memory was dynamically allocated rather than allocated on the
stack, it still exists when the function in which it was allocated
returns, and so we can then use it by dereferencing it.

One thing you'll notice here is that there is no deallocation code
anywhere. We're not actually leaking memory here - Rust can determine
at compile time where the allocated memory is done being used, and it
automatically inserts the call to free the memory at that point. The way
it determines this is by using a concept called "ownership" (boxed
values are sometimes called "owned pointers"). See this example: if I
create a boxed value and then try to store it in two different
variables, I get a compiler error. This is because boxed values aren't
copied, they are "moved". Assigning a boxed value to a different
variable doesn't copy anything at all, it just changes the name of the
variable that can be used to access the same data. Only a single
variable can own a boxed value at any given point, and given that
constraint, it is trivial to just trace through the code to see where
the value is no longer used.

Boxed values are not usually used on their own like this, however. In
almost all cases, for simple values, stack allocated values with
borrowed pointers are sufficient, and where they aren't, copying values
doesn't have a large enough performance impact to worry about. Where
boxed values are useful is in building data structures. Take this linked
list example, for instance. If you try to compile this code, you'll get
an error, because the compiler has no way of knowing how big the List
data structure is, since it contains a copy of itself. The solution here
is to instead make it contain a pointer to a copy of itself, which works
because pointers have a fixed size. Boxed values are also used in the
implementation of things like strings and vectors, since the data they
contain may need to be reallocated as they grow, and so storing the data
externally makes that possible.

Finally, we also have unsafe pointers (also called raw pointers), but
these are only intended for use when interoperating with C (these
pointers work exactly like C pointers). You can ignore their existence
entirely when writing normal Rust code.

Something you may have noticed in how we are using borrowed pointers and
boxed values is that they must always be initialized. Null pointers do
not exist in Rust (except when using unsafe pointers). Instead, you can
use the Option type mentioned earlier to wrap any pointers you want. The
compiler has an optimization for this which allows it to use a single
normal pointer as the representation, since it knows that null is an
invalid value for these pointers and the Option type has a single
"extra" value outside of the normal pointer range, and so using Option
with pointers actually has no overhead at all. This eliminates a huge
range of potential errors, since it's no longer possible to forget to
check a value for null - if you do, your program will fail to compile.

## Concurrency

Rust has also put a lot of effort into concurrency. In the interest of
time, I'm just going to give a brief overview, but the most interesting
point is that not only can the Rust type system ensure that your code
uses memory safely, it can also ensure that your code has no data races
when accessing the same memory from different threads. This allows you
to use parallelism quite a bit more effectively than you would be able
to without those guarantees, because figuring out where data races might
be in your code is incredibly hard to do on your own, and so usually
languages just fall back on copying a lot more than is necessary. Rust
just expands the ownership semantics I mentioned earlier with regards to
boxed values to also be applied to shared memory.

Rust's concurrency model is based around tasks. Tasks default to mapping
directly to threads (1:1 model), but they also have an optional M:N
scheduler if OS-level threads are too heavy. The basic idea is that all
data races are caused by data that is both mutable and aliasable, and so
any memory that is shared between tasks must be either entirely
immutable, or it must be owned by the task. Here's a basic example which
calculates the value of the Ackermann function at a given point in a
background task, and the main task waits for the result and then prints
it out. The channel function here is similar to the 'pipe' operator in
Perl - it just creates a one-way communication channel that the tasks
can communicate with. Now, clearly the channel can't be entirely
immutable, since you have to be able to send data across it, so the
thing that makes this example work is the 'proc' keyword here. A 'proc'
is a special type of closure which takes ownership of anything it closes
over (normal closures just take borrowed pointers to things they close
over). In this case, it closes over the writing end of the channel, and
so the main task can no longer access that end of the pipe, and neither
can any other tasks you might try to spawn in the same scope (if you
tried to, you would get a compilation error). This ensures that at any
given point in your program's execution, there is only a single task
trying to write to the pipe at any given time, and only a single task
trying to read from the pipe at any given time, so your program remains
deterministic. On the other hand, 'm' and 'n' are entirely immutable,
and so there are no issues with them being accessed from both the main
task and the calculation task.

## Misc

Rust also has quite a few other useful features that I didn't touch on.
It has namespacing and a module system with privacy controls. It has
integrated testing and benchmarks. It has quite a few compiler lint
checks, from warnings about things like unused variables and dead code
to optional errors about entire language features like "allocation" or
"unsafe blocks", and they can all be adjusted to be ignored, to warn, or
to error independently. It can interoperate with C directly, via extern
"C" blocks. The entire runtime and standard library can even be left out
or replaced in order to write things like kernels or embedded code -
there are already existing projects for writing a simple kernel in Rust
and running Rust code on Arduinos. There is a powerful macro system
available which is still constrained enough to not make writing external
parsing tools impossible. And the language is very flexible - most
language features are implemented via normal Rust functions which can be
overriden - either via traits for operations on new data types, or via
special "language items" for low level operations like memory
allocation.

# Contributing

So you've heard all of this and you're interested in learning more? A
good start to getting into the language is the tutorial on the Rust
website, as well as play.rust-lang.org and rustbyexample.com. If you're
interested in getting into Rust development, Rust is developed entirely
openly, and is always welcoming of new contributors. Discussion happens
both on IRC (on irc.mozilla.org) and on the rust-dev mailing list, and
decisions are made during open meetings between Mozilla's Rust team. For
keeping up with the language changes until 1.0 is released, This Week In
Rust is an excellent resource - it documents the major changes to the
language and libraries on a weekly basis, in case you don't have the
time to keep up with everything going on. Finally, Rust has a community
Standards of Conduct that is regularly enforced by the core team, and
this has helped to make the Rust community to be, in my experience, one
of the friendliest and most pleasant programming communities I've seen.
If this talk seemed interesting to you at all, I highly recommend
getting involved.

Any questions?
