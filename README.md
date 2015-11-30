# Components

> Systems are built from **Components**, not from **Dependencies**

### About

Components is a concept aimed to solve a problem of dependency management for long-running projects.

Work in progress, here are two articles where you can find more information:

[Components: taking a step back from Dependency Management](http://lowlevelbits.org/components-management)

[How to build a static iOS framework and distribute it as a Component using Make](http://stanislaw.github.io/2015/11/23/how-to-build-static-framework-using-make.html)

### Contribution

If you liked the concept and want to contribute a Component be sure to read introductory articles and review existing Components in `Components.make` folder. The following are work-in-progress guidelines we adhere to when we create Components.

#### Testing a Component

Be aware that the file structure inside `Components.make` folder in this repository is different from such in end-user projects as here we also maintain multiple versions of Components. So to be able to test your newly created `.make` file inside this repository, you can pass `COMPONENTS_MAKE_PATH` variable pointing to your new component's folder. So for example if you're creating a Component from `utf8proc 1.3.1` library, you should put `utf8proc.make` to the `Components.make/utf8proc/1.3.1` folder and point `COMPONENTS_MAKE_PATH` variable to it:

```
COMPONENTS_MAKE_PATH=./Components.make/utf8proc/1.3.1 ./components.sh install
```

This will make `components.sh` driver script to source `.make` files from `Components.make/utf8proc/1.3.1` directory.

#### Precompiled binary package distribution is recommended

If you are a maintainer of your own library we recommend you to distribute it as precompiled package i.e. static library or framework. While it is possible to write `.make` file which will copy your project's source files to an installation directory or which will build static framework from your library on consumer's side, frameworks are much faster to download and install since consumer of your component does not need to build them on his own. Also `.make` rules for libraries with binary distribution are shorter and therefore easier to deal with. Most likely you'll end up with your zipped project's binaries published in your Github Releases ([example](https://github.com/stanislaw/CompositeOperations/releases)), having that in place it is very easy to create .make file which downloads zip/tar, unpacks it and then copies to your project's `Components` directory.

#### We are here to help

If you have any questions about how to create a proper Component, how Make works or how to create static framework etc feel free to open issue and ask.

### Authors

The concept is brought to you by [AlexDenisov](https://github.com/AlexDenisov) and [Stanislaw Pankevich](https://github.com/stanislaw).

### License

Released under MIT license, see `LICENSE` for more details.

