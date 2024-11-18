<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->

<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">AlgeMapLoom: Weaving Mapping Languages with Algebraic Operators</h3>
</div>

<!-- ABOUT THE PROJECT -->

## About The Project

Mapping algebra provides operational semantics to the mapping process, opening
the door to study of complexity and expressiveness of existing mapping languages.
This project provides the CLI translator from RML and ShExML to mapping algebra.

<!-- GETTING STARTED -->

## Prerequisites

To compile the project on your own, you'll need to have
[Rust toolchain](https://www.rust-lang.org/tools/install) installed.

For Linux-based users:

- Rust
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

For the visualization of the generated mapping plans, you'll need
python version >= 3.10 and the following packages:

1. graphviz

## Running

1. Download this repo
2. Run cargo build at the root this repo
   ```sh
   cd {repo dir}
   cargo build --release
   ```
3. Run the CLI translator app from the compiled translator binary
   ```sh
   cd ./target/release/
   ./translator  file  <RML_DOCUMENT>
   ```
   For more information/options of CLI app:
   ```sh
   ./translator  -h
   ```
4. Visualize the created mapping plan
   ```sh
   dot -Tpng {generated dot file} > output.png
   ```
5. Simple plain text format of the mapping plan for parsing
   ```sh
   dot -Tplain {generated dot file} > output.txt
   ```
   <p align="right">(<a href="#readme-top">back to top</a>)</p>

## Test cases


### RML 
Currently, the translator generates valid mapping plans for the official
[RML test cases](https://github.com/kg-construct/rml-test-casesk) with mapping plans
for CSV data sources (all test cases ending in **CSV**).

The generated mapping plans for the test cases are inside the
[/resources/csv-testcases](/resources/csv-testcases).

### ShExML
The translator can *partially* translate ShExML documents. 
The translator will make a **best-effort** translation if the ShExML document
uses the following *unsupported* features. 
It will still generate a mapping plan which could be executed but the 
results won't be complete. 

The following features are not supported in translation yet: 

1) Autoincrements
2) Query statements
3) Joins 
4) Functions 
5) Conditionals

## Bindings
AlgeMapLoom provides bindings for Java, Python and Node.js.
These can be enabled with the features `jni`, `pyo3` and `neon` respectively.

If you build from source, you can run the `build_java.sh`, `build_python.sh` and `build_nodejs.sh`
scripts respectively.

For cross-compiling Windows binaries from Linux you need to install the MinGW cross-compiler.

For usage, check out the [src/java](src/java), [src/python](src/python), and [src/nodejs](src/nodejs) folders.

## Acknowledgement

This software makes use of [sophia_rs](https://github.com/pchampin/sophia_rs) crate!

<p align="right">(<a href="#readme-top">back to top</a>)</p>
