use std::collections::HashMap;
use std::error::Error;

use json_comments::StripComments;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};

pub fn parse_str(json: &str) -> Result<TsConfig, Box<dyn Error>> {
    // Remove trailing commas from objects.
    let re = Regex::new(r",(?P<valid>\s*})").unwrap();
    let json = re.replace_all(json, "$valid");
    let stripped = StripComments::new(json.as_bytes());
    let r: TsConfig = serde_json::from_reader(stripped)?;
    Ok(r)
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum References {
    Bool(bool),
    References(Vec<Reference>),
}

#[derive(Deserialize, Debug)]
pub struct Reference {
    path: String,
    prepend: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub enum TypeAcquisition {
    Bool(bool),
    Object {
        enable: bool,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        disable_filename_based_type_acquisition: Option<bool>,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TsConfig {
    /// Specifies an array of filenames or patterns that should be skipped when resolving include.
    exclude: Option<Vec<String>>,
    /// The value of extends is a string which contains a path to another configuration file to inherit from. The path may use Node.js style resolution.
    extends: Option<String>,
    /// Specifies an allowlist of files to include in the program. An error occurs if any of the files can’t be found.
    files: Option<Vec<String>>,
    /// Specifies an array of filenames or patterns to include in the program. These filenames are resolved relative to the directory containing the tsconfig.json file.
    include: Option<Vec<String>>,
    /// Project references are a way to structure your TypeScript programs into smaller pieces.
    /// Using Project References can greatly improve build and editor interaction times,
    /// enforce logical separation between components, and organize your code in new and improved ways.
    references: Option<References>,
    /// When you have a JavaScript project in your editor, TypeScript will provide types for your node_modules automatically
    /// using the DefinitelyTyped set of @types definitions.
    /// This is called automatic type acquisition, and you can customize it using the typeAcquisition object in your configuration.
    type_acquisition: Option<TypeAcquisition>,
    compiler_options: Option<CompilerOptions>,
}

/// These options make up the bulk of TypeScript’s configuration and it covers how the language should work.
#[derive(Deserialize, Debug)]
pub struct CompilerOptions {
    /// Allow JavaScript files to be imported inside your project, instead of just .ts and .tsx files.
    allow_js: Option<bool>,
    /// Works in tandem with allowJs. When checkJs is enabled then errors are reported in JavaScript files.
    /// This is the equivalent of including // @ts-check at the top of all JavaScript files which are included in your project.
    check_js: Option<bool>,
    /// The composite option enforces certain constraints which make it possible for build tools
    /// (including TypeScript itself, under --build mode) to quickly determine if a project has been built yet.
    composite: Option<bool>,
    /// Generate .d.ts files for every TypeScript or JavaScript file inside your project.
    /// These .d.ts files are type definition files which describe the external API of your module.
    /// With .d.ts files, tools like TypeScript can provide intellisense and accurate types for un-typed code.
    declaration: Option<bool>,
    /// Generates a source map for .d.ts files which map back to the original .ts source file.
    /// This will allow editors such as VS Code to go to the original .ts file when using features like Go to Definition.
    declaration_map: Option<bool>,
    /// Downleveling is TypeScript’s term for transpiling to an older version of JavaScript.
    /// This flag is to enable support for a more accurate implementation of how modern JavaScript
    /// iterates through new concepts in older JavaScript runtimes.
    downlevel_iteration: Option<bool>,
    /// For certain downleveling operations, TypeScript uses some helper code for operations like extending class,
    /// spreading arrays or objects, and async operations. By default, these helpers are inserted into files
    /// which use them. This can result in code duplication if the same helper is used in many different modules.
    ///
    /// If the importHelpers flag is on, these helper functions are instead imported from the tslib module.
    /// ou will need to ensure that the tslib module is able to be imported at runtime.
    /// This only affects modules; global script files will not attempt to import modules.
    import_helpers: Option<bool>,
    /// Tells TypeScript to save information about the project graph from the last compilation to files stored
    /// on disk. This creates a series of .tsbuildinfo files in the same folder as your compilation output.
    /// They are not used by your JavaScript at runtime and can be safely deleted.
    incremental: Option<bool>,
    /// While you can use TypeScript to produce JavaScript code from TypeScript code, it’s also common to use other
    /// transpilers such as Babel to do this. However, other transpilers only operate on a single file at a time,
    /// which means they can’t apply code transforms that depend on understanding the full type system. This restriction
    /// also applies to TypeScript’s ts.transpileModule API which is used by some build tools.
    ///
    /// These limitations can cause runtime problems with some TypeScript features like const enums and namespaces.
    /// Setting the isolatedModules flag tells TypeScript to warn you if you write certain code that can’t be
    /// correctly interpreted by a single-file transpilation process.
    isolated_modules: Option<bool>,
    jsx: Option<Jsx>,
    /// TypeScript includes a default set of type definitions for built-in JS APIs (like Math), as well as
    /// type definitions for things found in browser environments (like document). TypeScript also includes APIs for
    /// newer JS features matching the target you specify; for example the definition for Map is available if target
    /// is ES6 or newer.
    ///
    /// You may want to change these for a few reasons:
    ///
    /// - Your program doesn’t run in a browser, so you don’t want the "dom" type definitions
    /// - Your runtime platform provides certain JavaScript API objects (maybe through polyfills), but doesn’t
    ///   yet support the full syntax of a given ECMAScript version
    /// - You have polyfills or native implementations for some, but not all, of a higher level ECMAScript version
    lib: Option<Vec<Lib>>,
    /// Sets the module system for the program. You very likely want "CommonJS" for node projects.
    module: Option<Module>,
    /// Do not emit compiler output files like JavaScript source code, source-maps or declarations.
    ///
    /// This makes room for another tool like Babel, or swc to handle converting the TypeScript file to a file which can run inside a JavaScript environment.
    ///
    /// You can then use TypeScript as a tool for providing editor integration, and as a source code type-checker.
    no_emit: Option<bool>,
    /// If specified, .js (as well as .d.ts, .js.map, etc.) files will be emitted into this directory.
    /// The directory structure of the original source files is preserved; see rootDir if the computed root
    /// is not what you intended.
    out_dir: Option<String>,
    /// If specified, all global (non-module) files will be concatenated into the single output file specified.
    out_file: Option<String>,
    /// List of language service plugins to run inside the editor.
    // plugins: Option<Vec<Value>>,
    /// Strips all comments from TypeScript files when converting into JavaScript.
    remove_comments: Option<bool>,
    /// Default: The longest common path of all non-declaration input files.
    /// If composite is set, the default is instead the directory containing the tsconfig.json file.
    root_dir: Option<String>,
    source_map: Option<bool>,
    /// The target setting changes which JS features are downleveled and which are left intact.
    /// For example, an arrow function `() => this` will be turned into an equivalent `function` expression if `target` is ES5 or lower.
    target: Option<Target>,
    /// This option offers a way to configure the place where TypeScript keeps track of the files it stores
    /// on the disk to indicate a project’s build state — by default, they are in the same folder as your
    /// emitted JavaScript.
    ts_build_info_file: Option<String>,

    // Strict checks
    //
    /// Ensures that your files are parsed in the ECMAScript strict mode, and emit “use strict” for each source file.
    always_strict: Option<bool>,
    /// TypeScript will issue an error whenever it would have inferred `any`.
    no_implicit_any: Option<bool>,
    /// Raise error on ‘this’ expressions with an implied ‘any’ type.
    no_implicit_this: Option<bool>,
    /// The strict flag enables a wide range of type checking behavior that results in stronger guarantees of program correctness.
    /// Turning this on is equivalent to enabling all of the strict mode family options. You can then turn off individual strict
    /// mode family checks as needed.
    strict: Option<bool>,
    /// When set, TypeScript will check that the built-in methods of functions call, bind,
    /// and apply are invoked with correct argument for the underlying function.
    strict_bind_call_apply: Option<bool>,
    /// Causes functions parameters to be checked more correctly.
    strict_function_types: Option<bool>,
    /// When strictNullChecks is `true`, `null` and `undefined` have their own distinct types and you’ll
    /// get a type error if you try to use them where a concrete value is expected.
    strict_null_checks: Option<bool>,
    /// When set to true, TypeScript will raise an error when a class property was declared but not set in the constructor.
    strict_property_initialization: Option<bool>,
    /// When set to true, allowSyntheticDefaultImports allows you to write an import like:
    ///
    /// ```ts
    /// import React from "react";
    /// ```
    ///
    /// instead of:
    /// ```ts
    /// import * as React from "react";
    /// ```
    allow_synthetic_default_imports: Option<bool>,
    /// When set to true, allowUmdGlobalAccess lets you access UMD exports as globals from inside module files.
    /// A module file is a file that has imports and/or exports. Without this flag, using an export from a UMD
    /// module requires an import declaration.
    ///
    /// An example use case for this flag would be a web project where you know the particular library (like
    /// jQuery or Lodash) will always be available at runtime, but you can’t access it with an import.
    allow_umd_global_access: Option<bool>,
    /// Lets you set a base directory to resolve non-absolute module names.
    base_url: Option<bool>,
    es_module_interop: Option<bool>,
    /// Specify the module resolution strategy: `'node'` (Node.js) or `'classic'` (used in TypeScript before
    /// the release of 1.6). You probably won’t need to use classic in modern code.
    module_resolution: Option<ModuleResolutionMode>,
    /// A series of entries which re-map imports to lookup locations relative to the baseUrl, there is a
    /// larger coverage of paths in the handbook.
    paths: Option<HashMap<String, Vec<String>>>,
    preserve_symlinks: Option<bool>,
    /// Using rootDirs, you can inform the compiler that there are many “virtual” directories acting as a single root.
    /// This allows the compiler to resolve relative module imports within these “virtual” directories, as if they
    /// were merged in to one directory.
    root_dirs: Option<Vec<String>>,
    /// By default all visible ”@types” packages are included in your compilation. Packages in `node_modules/@types`
    /// of any enclosing folder are considered visible. For example, that means packages within
    /// `./node_modules/@types/`, `../node_modules/@types/`, `../../node_modules/@types/`, and so on.
    ///
    /// If `typeRoots` is specified, only packages under `typeRoots` will be included.
    type_roots: Option<Vec<String>>,
    /// By default all visible ”@types” packages are included in your compilation. Packages in `node_modules/@types`
    /// of any enclosing folder are considered visible. For example, that means packages within
    /// `./node_modules/@types/`, `../node_modules/@types/`, `../../node_modules/@types/`, and so on.
    ///
    /// If `types` is specified, only the packages listed will be included in the global scope.
    types: Option<Vec<String>>,
    /// When set, instead of writing out a .js.map file to provide source maps, TypeScript will embed the
    /// source map content in the .js files. Although this results in larger JS files, it can be convenient
    /// in some scenarios. For example, you might want to debug JS files on a webserver that doesn’t allow
    /// `.map` files to be served.
    ///
    /// Mutually exclusive with `source_map`.
    inline_source_map: Option<bool>,
    /// When set, TypeScript will include the original content of the .ts file as an embedded string in
    /// the source map. This is often useful in the same cases as inlineSourceMap.
    ///
    /// Requires either sourceMap or inlineSourceMap to be set.
    inline_sources: Option<bool>,
    /// Specify the location where debugger should locate map files instead of generated locations.
    map_root: Option<String>,
    /// Specify the location where a debugger should locate TypeScript files instead of relative source locations.
    source_root: Option<String>,
    /// Report errors for fallthrough cases in switch statements. Ensures that any non-empty case inside
    /// a switch statement includes either break or return. This means you won’t accidentally ship a case
    /// fallthrough bug.
    no_fallthrough_cases_in_switch: Option<bool>,
    /// When enabled, TypeScript will check all code paths in a function to ensure they return a value.
    no_implicit_returns: Option<bool>,
    /// This setting ensures consistency between accessing a field via the “dot” (obj.key) syntax, and “indexed” (obj["key"]) and the way which the property is declared in the type.
    ///
    /// Without this flag, TypeScript will allow you to use the dot syntax to access fields which are not defined
    no_property_access_from_index_signature: Option<bool>,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ModuleResolutionMode {
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "classic")]
    Classic,
}

#[derive(Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Jsx {
    React,
    ReactJsx,
    ReactJsxdev,
    ReactNative,
    Preserve,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Target {
    Es3,
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
}
impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Target::Es5,
            "ES2015" => Target::Es2015,
            "ES6" => Target::Es6,
            "ES2016" => Target::Es2016,
            "ES7" => Target::Es7,
            "ES2017" => Target::Es2017,
            "ES2018" => Target::Es2018,
            "ES2019" => Target::Es2019,
            "ES2020" => Target::Es2020,
            "ESNEXT" => Target::EsNext,
            other => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Other(other),
                    &"valid target type",
                ))
            }
        };

        Ok(d)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Lib {
    Es5,
    Es2015,
    Es6,
    Es2016,
    Es7,
    Es2017,
    Es2018,
    Es2019,
    Es2020,
    EsNext,
    Dom,
    WebWorker,
    ScriptHost,
    DomIterable,
    Es2015Core,
    Es2015Generator,
    Es2015Iterable,
    Es2015Promise,
    Es2015Proxy,
    Es2015Reflect,
    Es2015Symbol,
    Es2015SymbolWellKnown,
    Es2016ArrayInclude,
    Es2017Object,
    Es2017Intl,
    Es2017SharedMemory,
    Es2017String,
    Es2017TypedArrays,
    Es2018Intl,
    Es2018Promise,
    Es2018RegExp,
    Es2019Array,
    Es2019Object,
    Es2019String,
    Es2019Symbol,
    Es2020String,
    Es2020SymbolWellknown,
    EsNextAsyncIterable,
    EsNextArray,
    EsNextIntl,
    EsNextSymbol,
}

impl<'de> Deserialize<'de> for Lib {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let d = match s.as_str() {
            "ES5" => Lib::Es5,
            "ES2015" => Lib::Es2015,
            "ES6" => Lib::Es6,
            "ES2016" => Lib::Es2016,
            "ES7" => Lib::Es7,
            "ES2017" => Lib::Es2017,
            "ES2018" => Lib::Es2018,
            "ES2019" => Lib::Es2019,
            "ES2020" => Lib::Es2020,
            "ESNext" => Lib::EsNext,
            "DOM" => Lib::Dom,
            "WEBWORKER" => Lib::WebWorker,
            "SCRIPTHOST" => Lib::ScriptHost,
            "DOM.ITERABLE" => Lib::DomIterable,
            "ES2015.CORE" => Lib::Es2015Core,
            "ES2015.GENERATOR" => Lib::Es2015Generator,
            "ES2015.ITERABLE" => Lib::Es2015Iterable,
            "ES2015.PROMISE" => Lib::Es2015Promise,
            "ES2015.PROXY" => Lib::Es2015Proxy,
            "ES2015.REFLECT" => Lib::Es2015Reflect,
            "ES2015.SYMBOL" => Lib::Es2015Symbol,
            "ES2015.SYMBOL.WELLKNOWN" => Lib::Es2015SymbolWellKnown,
            "ES2015.ARRAY.INCLUDE" => Lib::Es2016ArrayInclude,
            "ES2015.OBJECT" => Lib::Es2017Object,
            "ES2017INTL" => Lib::Es2017Intl,
            "ES2015.SHAREDMEMORY" => Lib::Es2017SharedMemory,
            "ES2017.STRING" => Lib::Es2017String,
            "ES2017.TYPEDARRAYS" => Lib::Es2017TypedArrays,
            "ES2018.INTL" => Lib::Es2018Intl,
            "ES2018.PROMISE" => Lib::Es2018Promise,
            "ES2018.REGEXP" => Lib::Es2018RegExp,
            "ES2019.ARRAY" => Lib::Es2019Array,
            "ES2019.OBJECT" => Lib::Es2019Object,
            "ES2019.STRING" => Lib::Es2019String,
            "ES2019.SYMBOL" => Lib::Es2019Symbol,
            "ES2020.STRING" => Lib::Es2020String,
            "ES2020.SYMBOL.WELLKNOWN" => Lib::Es2020SymbolWellknown,
            "ESNEXT.ASYNCITERABLE" => Lib::EsNextAsyncIterable,
            "ESNEXT.ARRAY" => Lib::EsNextArray,
            "ESNEXT.INTL" => Lib::EsNextIntl,
            "ESNEXT.SYMBOL" => Lib::EsNextSymbol,
            other => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Other(other),
                    &"valid library type",
                ))
            }
        };

        Ok(d)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Module {
    CommonJs,
    Es6,
    Es2015,
    Es2020,
    None,
    Umd,
    Amd,
    System,
    EsNext,
}

impl<'de> Deserialize<'de> for Module {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.to_uppercase();

        let r = match s.as_str() {
            "COMMONJS" => Module::CommonJs,
            "ESNEXT" => Module::EsNext,
            "ES6" => Module::Es6,
            "ES2015" => Module::Es2015,
            "ES2020" => Module::Es2020,
            "NONE" => Module::None,
            "UMD" => Module::Umd,
            "AMD" => Module::Amd,
            "SYSTEM" => Module::System,
            other => {
                return Err(de::Error::invalid_value(
                    de::Unexpected::Other(other),
                    &"valid module type",
                ))
            }
        };

        Ok(r)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_jsx() {
        let json = r#"{"compilerOptions": {"jsx": "react-jsx"}}"#;

        let config: TsConfig = parse_str(json).unwrap();
        assert_eq!(config.compiler_options.unwrap().jsx, Some(Jsx::ReactJsx));
    }

    #[test]
    fn parse_paths() {
        let json = r#"{
        "compilerOptions": {
            "baseUrl": "src",
            "paths": {
                "tests/*": ["tests/*"],
                "blah": ["bloop"]
            }
        }
    }
        
        "#;

        let config: TsConfig = parse_str(json).unwrap();
        assert_eq!(
            config
                .compiler_options
                .unwrap()
                .paths
                .unwrap()
                .get("tests/*"),
            Some(&vec!["tests/*".to_string()])
        );
    }

    #[test]
    fn parse_empty() {
        let _: TsConfig = parse_str("{}").unwrap();
        let _: TsConfig = parse_str(r#"{"compilerOptions": {}}"#).unwrap();
    }

    #[test]
    fn parse_default() {
        let json = include_str!("../test/default_tsconfig.json");
        let _: TsConfig = parse_str(json).unwrap();
    }
}
