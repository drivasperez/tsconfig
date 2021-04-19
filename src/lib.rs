use std::collections::HashMap;
use std::{collections::BTreeMap as Map, error::Error};

use json_comments::StripComments;
use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;

pub fn parse_str(json: &str) -> Result<TsConfig, Box<dyn Error>> {
    let stripped = StripComments::new(json.as_bytes());
    let r: TsConfig = serde_json::from_reader(stripped)?;
    Ok(r)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum References {
    Bool(bool),
    References(Vec<Reference>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reference {
    path: String,
    prepend: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TypeAcquisition {
    Bool(bool),
    Object {
        enable: bool,
        include: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
        disable_filename_based_type_acquisition: Option<bool>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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
    module: Option<Vec<Module>>,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum ModuleResolutionMode {
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "classic")]
    Classic,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Jsx {
    React,
    ReactJsx,
    ReactJsxdev,
    ReactNative,
    Preserve,
}

fn case_insensitive<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    let map = Map::<String, Value>::deserialize(deserializer)?;
    let lower = map
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();
    T::deserialize(Value::Object(lower)).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Target {
    #[serde(rename = "ES53", deserialize_with = "case_insensitive")]
    Es3,
    #[serde(rename = "ES5", deserialize_with = "case_insensitive")]
    Es5,
    #[serde(rename = "ES2015", deserialize_with = "case_insensitive")]
    Es2015,
    #[serde(rename = "ES6", deserialize_with = "case_insensitive")]
    Es6,
    #[serde(rename = "ES2016", deserialize_with = "case_insensitive")]
    Es2016,
    #[serde(rename = "ES7", deserialize_with = "case_insensitive")]
    Es7,
    #[serde(rename = "ES2017", deserialize_with = "case_insensitive")]
    Es2017,
    #[serde(rename = "ES2018", deserialize_with = "case_insensitive")]
    Es2018,
    #[serde(rename = "ES2019", deserialize_with = "case_insensitive")]
    Es2019,
    #[serde(rename = "ES2020", deserialize_with = "case_insensitive")]
    Es2020,
    #[serde(rename = "ESNext", deserialize_with = "case_insensitive")]
    EsNext,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum Lib {
    #[serde(rename = "ES5")]
    Es5,
    #[serde(rename = "ES2015")]
    Es2015,
    #[serde(rename = "ES6")]
    Es6,
    #[serde(rename = "ES2016")]
    Es2016,
    #[serde(rename = "ES7")]
    Es7,
    #[serde(rename = "ES2017")]
    Es2017,
    #[serde(rename = "ES2018")]
    Es2018,
    #[serde(rename = "ES2019")]
    Es2019,
    #[serde(rename = "ES2020")]
    Es2020,
    #[serde(rename = "ESNext")]
    EsNext,
    #[serde(rename = "DOM")]
    Dom,
    WebWorker,
    ScriptHost,
    #[serde(rename = "DOM.Iterable")]
    DomIterable,
    #[serde(rename = "ES2015.Core")]
    Es2015Core,
    #[serde(rename = "ES2015.Generator")]
    Es2015Generator,
    #[serde(rename = "ES2015.Iterable")]
    Es2015Iterable,
    #[serde(rename = "ES2015.Promise")]
    Es2015Promise,
    #[serde(rename = "ES2015.Proxy")]
    Es2015Proxy,
    #[serde(rename = "ES2015.Reflect")]
    Es2015Reflect,
    #[serde(rename = "ES2015.Symbol")]
    Es2015Symbol,
    #[serde(rename = "ES2015.Symbol.WellKnown")]
    Es2015SymbolWellKnown,
    #[serde(rename = "ES2015.Array.Include")]
    Es2016ArrayInclude,
    #[serde(rename = "ES2015.object")]
    Es2017Object,
    #[serde(rename = "ES2017Intl")]
    Es2017Intl,
    #[serde(rename = "ES2015.SharedMemory")]
    Es2017SharedMemory,
    #[serde(rename = "ES2017.String")]
    Es2017String,
    #[serde(rename = "ES2017.TypedArrays")]
    Es2017TypedArrays,
    #[serde(rename = "ES2018.Intl")]
    Es2018Intl,
    #[serde(rename = "ES2018.Promise")]
    Es2018Promise,
    #[serde(rename = "ES2018.RegExp")]
    Es2018RegExp,
    #[serde(rename = "ES2019.Array")]
    Es2019Array,
    #[serde(rename = "ES2019.Object")]
    Es2019Object,
    #[serde(rename = "ES2019.String")]
    Es2019String,
    #[serde(rename = "ES2019.Symbol")]
    Es2019Symbol,
    #[serde(rename = "ES2020.String")]
    Es2020String,
    #[serde(rename = "ES2020.Symbol.wellknown")]
    Es2020SymbolWellknown,
    #[serde(rename = "ESNext.AsyncIterable")]
    EsNextAsyncIterable,
    #[serde(rename = "ESNext.Array")]
    EsNextArray,
    #[serde(rename = "ESNext.Intl")]
    EsNextIntl,
    #[serde(rename = "ESNext.Symbol")]
    EsNextSymbol,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Module {
    #[serde(rename = "CommonJS")]
    CommonJs,
    #[serde(rename = "ES6")]
    Es6,
    #[serde(rename = "ES2015")]
    Es2015,
    #[serde(rename = "ES2020")]
    Es2020,
    None,
    #[serde(rename = "UMD")]
    Umd,
    #[serde(rename = "AMD")]
    Amd,
    System,
    #[serde(rename = "ESNext")]
    EsNext,
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
