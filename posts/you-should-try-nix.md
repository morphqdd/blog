You may have never heard of [Nix](https://nixos.org/) — unless you know me personally.  
If you do, chances are I’ve already bugged you about it enough! And now, well… you’re officially one of the chosen few. This post is all about Nix.



Nix is primarily a package manager, but surprisingly, it’s also a programming language. And just the way I like it — functional. This language plays a key role in how the package manager works.

<br>

We’re all used to this: you want to install a package, type some command in the terminal, and then pray that it works — and that you don’t have to clean up after your brilliant idea of installing a third browser (seriously, why do you need it?).  

<br>

Nix does things differently: to install, say, your “last-ever” browser, you actually **add it to the package manager’s configuration**. Yes, it’s as wild as it sounds.

<br>

Here’s what makes Nix special from a functional perspective:
  - **Immutability** – every installed package is immutable and isolated from others. You can have two versions of the same library without worrying that an old app will refuse to move forward.  
  - **Packages as functions** – every package has a well-defined output for given inputs and no side effects. This ensures that the same configuration produces identical programs on your machine.  
  - **Declarative approach** – you don’t describe the exact steps to install a package. You simply declare what you want, and Nix gives it to you.

<br>

The result? **Reproducibility**. Every change can be rolled back before “garbage collection,” letting you use previous versions of your package manager setup.

<br>

If any of this resonates — or even if it doesn’t — you **have to try Nix**. It’s an experience no other package manager on the market can give you.

<br>

P.S. After using it, you might develop a mild aversion to the imperative approach
