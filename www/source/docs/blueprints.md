# Blueprints
Blueprints are default implementations of the build and runtime phases for your application. They also provide some default health check hooks where appropriate to ensure your application functioning reliably. Customized Blueprints can be created to facilitate reusability of common patterns in your organization for developing, building, and running your applications.

## Getting Started
### New Projects
Simply run `hab plan init` to leverage blueprints. Habitat will look in your application code for known project types and the appropriate `pkg_blueprint`.

If you would like to specify the desired blueprint to use, you can run `hab plan init java8-maven-tomcat`

### Existing Plans

To begin using Blueprints, you will need to add the appropriate `pkg_blueprints` 

```bash
pkg_name="my_java_app"
....
pkg_blueprints=("core/blueprints/java8-maven-tomcat")
               ^     ^^^^^^^^^^^                    ^
               |______|_______|_____________________|
                 Arrays and additional namespace TBD -- just random thoughts at the moment.
```
* add `auto` package to `build_deps`
* set toolchain variable

#### Optional Variables
* `pkg_src` - now accepts URI notation. Each blueprint contains a default URI which is relevant to the type of blueprint being used.
*  `pkg_blueprint_opts` - Options which override some default runtime parameters. Eg: `--env DEV`. This operates similar to `pkg_svc_run` as the result of using this option will intelligently generate a hook file for you. However, you also can create the [hook file](#Run-Hooks) directly if you need to have more control over the resulting output.

### Available Blueprints
_The following list is a mock-up, and is does not currently reflect reality_:

* `core/blueprint/ruby-mri1.9-rails-unicorn`
* `core/blueprint/java8-maven-tomcat`
* `core/blueprint/java8-maven-jboss`
* `core/blueprint/java8-gradle-tomcat`
* `core/blueprint/java7-ant`
* `core/blueprint/python-unicorn`

## Callbacks
Each blueprint defines a set of callbacks which are unique to the blueprint type. This allows you to complex projects which may need to leverage different blueprints as needed within your project. Each of these callbacks define overrides of base level callbacks.

### ruby _(This will likely live in the blueprint README.md)_
#### do_bundler
#### do_rake
#### do_rack
#### do_lotsmorestuff 

## Run Hooks
#### Health Checks
Each blueprint comes with a default health check for your application.
#### blueprint_opts
Follow these directions if you wish to create the hook which would otherwise be created by the optional  `pkg_blueprint_opts` variable.

*TODO: Write the directions*

## Build Artifacts
### Auto-Ingestion
Each blueprint defines a default value for `pkg_src` in [URI syntax](https://tools.ietf.org/html/rfc3986). You can override this value within your plan should you application source live in a non-standard location.
