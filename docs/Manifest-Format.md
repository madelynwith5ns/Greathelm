# Project Manifest (Project.ghm) Documentation

A Greathelm Manifest (Project.ghm) contains two main types of data:

- Properties:
```ghm
Key=Value
```
- Directives
```ghm
@Dependency libexample
@Directive no-link-libc
```

### Properties
Properties are in the form of Key=Value pairs.

#### The following properties exist on any Greathelm project builder:

- **Project-Name** This specifies the project name.
- **Project-Author** This sets the author. Currently unused.
- **Project-Version** This sets the version. Currently unused.
- **Project-Type** This denotes the type of project. It is very important as without it your project cannot be built. Currently the only valid project type is `C`.

#### C Properties: These properties exist on C projects:
- **Compiler-Opt-Level** Sets the compiler optimization level. Translates to the -O argument.
- **Executable-Name** Sets the name of the compiled executable. If the Emit flag is set to `dylib` or `shared` the builder will add `lib-` and `-.so` to the executable name.
- **Emit** Decides what type of compiled binary should be produced. Valid options are `executable`, `binary` (both mean normal executable binaries), `dylib`, and `shared` (both mean .so shared objects).
- **Override-C-Compiler** Specifies the compiler binary to be used. Defaults to `cc` if unset.
- **Override-C-Linker** Specifies the linker binary to be used. Defaults to `cc` if unset.
- **Additional-CC-Flags** Specifies additional C compiler flags to be used. Separated by a comma (,). Defaults to none.
- **Additional-LD-Flags** Specifies additional C linker flags to be used. Separated by a comma (,). Defaults to none.
- **C-Linker-Script** Specifies a custom linker script to be used.

#### C++ Properties: These properties exist on C++ projects:
- **Compiler-Opt-Level** Sets the compiler optimization level. Translates to the -O argument.
- **Executable-Name** Sets the name of the compiled executable. If the Emit flag is set to `dylib` or `shared` the builder will add `lib-` and `-.so` to the executable name.
- **Emit** Decides what type of compiled binary should be produced. Valid options are `executable`, `binary` (both mean normal executable binaries), `dylib`, and `shared` (both mean .so shared objects).
- **Override-C++-Compiler** Specifies the compiler binary to be used. Defaults to `c++` if unset.
- **Override-C++-Linker** Specifies the linker binary to be used. Defaults to `c++` if unset.
- **Additional-CC-Flags** Specifies additional C compiler flags to be used. Separated by a comma (,). Defaults to none.
- **Additional-LD-Flags** Specifies additional C linker flags to be used. Separated by a comma (,). Defaults to none.
- **C-Linker-Script** Specifies a custom linker script to be used.
- **C++-Stdlib-Flavor** Specifies which C++ standard library should be linked. Defaults to `stdc++`.

### @Dependency Directives

#### @Dependency Directives in C and C++ Projects
Dependency directives come in three forms:

- `@Dependency raw/<dependency>` The `raw/` prefix denotes this as a raw object dependency. These are located in the `lib/obj/` directory. Greathelm automatically appends `.o`. For example if you use `@Dependency !test` it will link `lib/obj/test.o`.

- `@Dependency sys/<dependency>` The `sys/` prefix denotes that the dependency should come from your system instead of the `lib/` directory. This uses `pkg-config`.

- `@Dependency provided/<dependency>` The `provided/` prefix denotes that this dependency is stored locally in the project. Its headers are in `lib/include/` and its binaries are in `lib/shared/`.

- `@Dependency <identifier>` - A normal dependency coming from your local store. If you have previously `greathelm import`-ed a project with the namespaced identifier `com.example.libs:ExampleLib`, you can include it in any new project with `@Dependency com.example.libs:ExampleLib`. You can also append the version like so: `@Dependency com.example.libs:ExampleLib@3.0.0-rc-4`

### @Directive Directives

@Directives are special instructions given to the builder.

#### @Directives in C Projects
The C builder currently recognizes two @Directives:

- **@Directive no-link-libc** This directive specifies to not link libc into the project. Translates to `-nostdlib`.

- **@Directive freestanding** This directive translates to the `-ffreestanding` cc flag.

### @Module Directives

@Module directives specify a sub-project used as a component in a larger project. They are in the following format:

```ghm
@Module <name> <components...>
```

An individual module component uses the following format:

```ghm
<path in parent>:<path in module>
```

For example, to include the artifact `build/libexample.so` from the `example` module as `lib/shared/libexample.so` in your project and the header `src/example.h` from the same module as `lib/include/example.h` you would use the following syntax:

```ghm
@Module example lib/shared/libexample.so:build/libexample.so lib/include/example.h:src/example.h
```

Module components can be directories if needed.

### @Alias Directives
Alias directives are intended to be set in local manifests (`Project.local.ghm`, `$XDG_CONFIG_HOME/greathelm/UserManifest.md`). They specify ways to resolve ambiguous names or simply just changing how you refer to something.

For example:

```ghm
# Use specifically the Greathelm-builtin C builder and generator instead of one provided by a plugin.
@Alias C=io.github.madelynwith5ns.greathelm:C
```

### @Import Directives
Import directives simply include another manifest file at the location of the directive.

For example:

```ghm
@Import OtherManifest.ghm
```